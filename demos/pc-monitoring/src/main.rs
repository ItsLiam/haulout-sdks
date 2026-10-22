mod gpu;
mod history;
mod metrics;
mod sample;

use std::collections::HashSet;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use clap::Parser;
use haulout::apis::configuration::Configuration;
use haulout::apis::default_api::{
    DeleteMetricsByKeyError, PutMetricsByKeyError, delete_metrics_by_key, put_metrics_by_key,
};
use haulout::apis::Error as ApiError;
use haulout::models::metric::Kind;
use haulout::models::{DeleteMetricsByKeyRequest, Metric, PutMetricsByKeyRequest};
use serde_json::json;
use tokio::time::MissedTickBehavior;

use crate::history::History;
use crate::sample::Sampler;

const PLACEHOLDER_KEY: &str = "replace-with-a-real-product-key";
const PUT_CHUNK: usize = 100;
const DELETE_CHUNK: usize = 256;

#[derive(Parser)]
#[command(about = "Publish this PC's live metrics to a Haulout product dashboard")]
struct Args {
    /// Seconds between samples (minimum 5).
    #[arg(long, default_value_t = 60, value_parser = clap::value_parser!(u64).range(5..))]
    interval: u64,
    /// Take one sample, publish it, and exit.
    #[arg(long)]
    once: bool,
    /// Take one sample, print the metrics as JSON, and exit. Needs no key and writes nothing.
    #[arg(long)]
    dry_run: bool,
    /// Metric key namespace. Defaults to a prefix derived from the hostname.
    #[arg(long)]
    prefix: Option<String>,
    /// History file. Defaults to the local data directory.
    #[arg(long)]
    state_file: Option<PathBuf>,
}

enum PublishError {
    /// 400/403: retrying cannot help, so the process exits.
    Permanent(String),
    Transient(String),
}

#[tokio::main]
async fn main() -> ExitCode {
    dotenvy::dotenv().ok();
    let args = Args::parse();

    if let Some(p) = &args.prefix {
        if !metrics::valid_prefix(p) {
            eprintln!("invalid --prefix");
            return ExitCode::from(1);
        }
    }

    let key = if args.dry_run {
        None
    } else {
        match read_key() {
            Some(k) => Some(k),
            None => {
                eprintln!("set HAULOUT_PRODUCT_KEY (environment or .env) to a real product key");
                return ExitCode::from(1);
            }
        }
    };

    let prefix = args.prefix.clone().unwrap_or_else(|| {
        metrics::derive_prefix(
            &sysinfo::System::host_name().unwrap_or_else(|| "unknown-host".to_string()),
        )
    });
    let state_path = args
        .state_file
        .clone()
        .unwrap_or_else(|| default_state_path(&prefix));
    let mut history = History::load(&state_path);
    let config = Configuration::default();

    let mut sampler = Sampler::new();
    sampler.prime();

    if args.dry_run {
        let s = sampler.sample();
        history.push(&s);
        let built = metrics::build(&prefix, &s, &history);
        report_dropped(&built.dropped, &mut Vec::new());
        println!(
            "{}",
            serde_json::to_string_pretty(&built.metrics).unwrap_or_else(|_| "[]".to_string())
        );
        return ExitCode::SUCCESS;
    }

    let key = key.unwrap_or_default();
    let mut published: HashSet<String> = HashSet::new();
    let mut last_dropped: Vec<String> = Vec::new();
    let mut ticker = tokio::time::interval(Duration::from_secs(args.interval));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = ticker.tick() => {}
            _ = tokio::signal::ctrl_c() => {
                let now = chrono::Utc::now().format("%H:%M UTC");
                let mut status = Metric::new(
                    format!("{prefix}.status"),
                    Kind::Status,
                    Some(json!({"state": "neutral", "text": format!("offline since {now}")})),
                );
                status.label = Some("Status".to_string());
                let _ = put_metrics_by_key(
                    &config,
                    PutMetricsByKeyRequest::new(key.clone(), vec![status]),
                )
                .await;
                return ExitCode::SUCCESS;
            }
        }

        let s = sampler.sample();
        history.push(&s);
        history.prune(s.at.timestamp());
        if let Err(e) = history.save(&state_path) {
            eprintln!("warning: could not save history: {e}");
        }

        let built = metrics::build(&prefix, &s, &history);
        report_dropped(&built.dropped, &mut last_dropped);
        match publish(&config, &key, built.metrics, &mut published).await {
            Ok((count, max_rev)) => {
                println!("published {count} metrics, max revision {max_rev}");
                if args.once {
                    return ExitCode::SUCCESS;
                }
            }
            Err(PublishError::Permanent(msg)) => {
                eprintln!("permanent error {msg}");
                return ExitCode::from(1);
            }
            Err(PublishError::Transient(msg)) => {
                eprintln!("publish failed, retrying next tick: {msg}");
                if args.once {
                    return ExitCode::from(1);
                }
            }
        }
    }
}

fn read_key() -> Option<String> {
    std::env::var("HAULOUT_PRODUCT_KEY")
        .ok()
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty() && k != PLACEHOLDER_KEY)
}

fn default_state_path(prefix: &str) -> PathBuf {
    match dirs::data_local_dir() {
        Some(dir) => dir
            .join("haulout-pc-monitoring")
            .join(format!("{prefix}.json")),
        None => PathBuf::from(format!(".haulout-pc-monitoring-{prefix}.json")),
    }
}

/// Warns on stderr when the set of sensors whose series were dropped by the metric limit
/// changes between ticks. Stale keys for dropped sensors are removed by `publish`.
fn report_dropped(dropped: &[String], last: &mut Vec<String>) {
    if dropped == last.as_slice() {
        return;
    }
    if dropped.is_empty() {
        eprintln!("note: per-sensor series back within the 256-metric limit");
    } else {
        eprintln!(
            "warning: 256-metric limit reached; per-sensor series dropped for {} sensor(s): {}",
            dropped.len(),
            dropped.join(", ")
        );
    }
    *last = dropped.to_vec();
}

/// Upserts the current metrics in chunks, then deletes keys published on the
/// previous tick that are no longer reported. Returns (metrics written, max revision).
async fn publish(
    config: &Configuration,
    key: &str,
    list: Vec<Metric>,
    published: &mut HashSet<String>,
) -> Result<(usize, i64), PublishError> {
    let current: HashSet<String> = list.iter().map(|m| m.key.clone()).collect();
    let mut count = 0;
    let mut max_rev = 0;

    for chunk in list.chunks(PUT_CHUNK) {
        let results = put_metrics_by_key(
            config,
            PutMetricsByKeyRequest::new(key.to_string(), chunk.to_vec()),
        )
        .await
        .map_err(put_error)?;
        for r in results {
            count += 1;
            max_rev = max_rev.max(r.revision);
        }
    }

    let stale: Vec<String> = published.difference(&current).cloned().collect();
    for chunk in stale.chunks(DELETE_CHUNK) {
        delete_metrics_by_key(
            config,
            DeleteMetricsByKeyRequest::new(key.to_string(), chunk.to_vec()),
        )
        .await
        .map_err(delete_error)?;
    }

    *published = current;
    Ok((count, max_rev))
}

fn put_error(e: ApiError<PutMetricsByKeyError>) -> PublishError {
    match e {
        ApiError::ResponseError(r) => {
            if matches!(r.status.as_u16(), 400 | 403) {
                let detail = match r.entity {
                    Some(PutMetricsByKeyError::DefaultResponse(pg)) => {
                        format!("{}: {}", pg.code, pg.message)
                    }
                    _ => format!("HTTP {}", r.status),
                };
                PublishError::Permanent(detail)
            } else {
                PublishError::Transient(format!("HTTP {}", r.status))
            }
        }
        ApiError::Reqwest(_) => PublishError::Transient("transport error".to_string()),
        ApiError::Serde(_) | ApiError::Io(_) => {
            PublishError::Transient("request error".to_string())
        }
    }
}

fn delete_error(e: ApiError<DeleteMetricsByKeyError>) -> PublishError {
    match e {
        ApiError::ResponseError(r) => {
            if matches!(r.status.as_u16(), 400 | 403) {
                let detail = match r.entity {
                    Some(DeleteMetricsByKeyError::DefaultResponse(pg)) => {
                        format!("{}: {}", pg.code, pg.message)
                    }
                    _ => format!("HTTP {}", r.status),
                };
                PublishError::Permanent(detail)
            } else {
                PublishError::Transient(format!("HTTP {}", r.status))
            }
        }
        ApiError::Reqwest(_) => PublishError::Transient("transport error".to_string()),
        ApiError::Serde(_) | ApiError::Io(_) => {
            PublishError::Transient("request error".to_string())
        }
    }
}
