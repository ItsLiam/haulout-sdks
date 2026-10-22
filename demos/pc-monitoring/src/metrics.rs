use chrono::{SecondsFormat, TimeZone, Utc};
use haulout::models::metric::Kind;
use haulout::models::metric_format::{MetricFormat, Style};
use haulout::models::Metric;
use serde_json::{Value, json};

use crate::history::{History, series_values, trended_sensors};
use crate::sample::Sample;

const MAX_LABEL: usize = 100;
const MAX_TEXT: usize = 256;
const MAX_ROWS: usize = 100;
const MAX_TABLE_ROWS: usize = 500;
/// Product metric limit; per-sensor series are dropped to stay under it.
const MAX_METRICS: usize = 256;

/// Series windows published for every series: (key suffix, span secs, bucket secs, description).
const WINDOWS: [(&str, i64, Option<i64>, &str); 3] = [
    ("1h", 3600, None, "last hour"),
    ("6h", 21600, Some(300), "6h (5-min avg)"),
    ("12h", 43200, Some(600), "12h (10-min avg)"),
];

/// Accepts `^[a-z][a-z0-9_-]{0,19}$`.
pub fn valid_prefix(p: &str) -> bool {
    let mut chars = p.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_lowercase())
        && p.len() <= 20
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

/// Derives a key prefix from a hostname: lowercased, unsafe characters become
/// `-`, at most 20 characters, and always starting with a letter.
pub fn derive_prefix(host: &str) -> String {
    let cleaned: String = host
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect();
    let trimmed: String = cleaned.trim_matches('-').chars().take(20).collect();
    if trimmed.starts_with(|c: char| c.is_ascii_lowercase()) {
        trimmed
    } else {
        format!("pc-{trimmed}").chars().take(20).collect()
    }
}

/// Result of one build: the metrics to publish, plus sensor labels whose per-sensor series
/// were dropped to stay under `MAX_METRICS` (in label order).
pub struct Built {
    pub metrics: Vec<Metric>,
    pub dropped: Vec<String>,
}

/// Builds the full metric catalogue for one sample plus stored history.
pub fn build(prefix: &str, s: &Sample, history: &History) -> Built {
    let p = |suffix: &str| format!("{prefix}.{suffix}");
    let mut out = Vec::new();

    status(&p, s, &mut out);
    host(&p, s, &mut out);

    out.push(metric(
        p("uptime"),
        Kind::Number,
        json!(s.host.uptime_secs),
        "Uptime",
        Some(fmt(Style::Duration, "s", None)),
    ));

    cpu(&p, s, &mut out);
    memory(&p, s, &mut out);
    disks(&p, s, &mut out);
    network(&p, s, &mut out);
    temperatures(&p, s, &mut out);
    gpus(&p, s, &mut out);
    battery(&p, s, &mut out);
    processes(&p, s, &mut out);
    series(prefix, s, history, &mut out);
    let (sensors, dropped) = split_by_budget(out.len(), sensor_groups(prefix, s, history));
    out.extend(sensors);

    Built { metrics: out, dropped }
}

fn status(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    let critical = s
        .temps
        .iter()
        .find(|t| t.critical.is_some_and(|c| t.celsius >= c));
    let mut warnings = Vec::new();
    if s.cpu_total > 90.0 {
        warnings.push(format!("CPU {:.0}%", s.cpu_total));
    }
    if s.mem_total > 0 && percent(s.mem_used, s.mem_total) > 90.0 {
        warnings.push("memory above 90%".to_string());
    }
    for d in &s.disks {
        if percent(d.total - d.available, d.total) > 90.0 {
            warnings.push(format!("disk {} above 90%", d.mount));
        }
    }
    let (state, reason) = match critical {
        Some(t) => (
            "critical",
            format!("critical temperature: {} {:.1} °C", t.label, t.celsius),
        ),
        None if !warnings.is_empty() => ("warning", warnings.join(", ")),
        None => ("positive", "all clear".to_string()),
    };
    let at = s.at.format("%H:%M UTC");
    let text = trunc(format!("{} · sampled {at} · {reason}", s.host.hostname), MAX_TEXT);
    out.push(metric(
        p("status"),
        Kind::Status,
        json!({"state": state, "text": text}),
        "Status",
        None,
    ));
}

fn host(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    let h = &s.host;
    let phys = h
        .physical_cores
        .map_or_else(|| "unknown".to_string(), |n| n.to_string());
    let gpu_names = if s.gpus.is_empty() {
        "none detected".to_string()
    } else {
        s.gpus.iter().map(|g| g.name.as_str()).collect::<Vec<_>>().join(", ")
    };
    let battery = s
        .battery
        .as_ref()
        .map_or("none".to_string(), |b| b.state.clone());
    let rows = vec![
        row(["Hostname".into(), trunc(h.hostname.clone(), MAX_LABEL).into()]),
        row(["OS".into(), trunc(h.os.clone(), MAX_LABEL).into()]),
        row(["Kernel".into(), trunc(h.kernel.clone(), MAX_LABEL).into()]),
        row(["CPU".into(), trunc(h.cpu_brand.clone(), MAX_LABEL).into()]),
        row(["Cores".into(), format!("{phys} physical, {} logical", h.logical_cores).into()]),
        row(["GPUs".into(), trunc(gpu_names, MAX_LABEL).into()]),
        row(["Temperature sensors".into(), json!(s.temps.len())]),
        row(["Battery".into(), battery.into()]),
        row(["Boot time".into(), ts(h.boot_time).into()]),
    ];
    out.push(table(
        p("host"),
        "Host",
        &["Property", "Value"],
        rows,
        &["left", "left"],
    ));
}

fn cpu(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    out.push(metric(
        p("cpu.usage"),
        Kind::Gauge,
        json!({"value": r1(f64::from(s.cpu_total)), "min": 0, "max": 100}),
        "CPU usage",
        Some(pct()),
    ));
    if !s.cpu_cores.is_empty() {
        let bars = s
            .cpu_cores
            .iter()
            .enumerate()
            .take(MAX_ROWS)
            .map(|(i, c)| (format!("Core {}", i + 1), f64::from(*c)))
            .collect();
        out.push(bars_metric(p("cpu.cores"), "CPU cores", bars, Some(pct())));
    }
    if s.cpu_freq_mhz > 0 {
        out.push(metric(
            p("cpu.frequency"),
            Kind::Number,
            json!(s.cpu_freq_mhz),
            "CPU frequency",
            Some(fmt(Style::Number, "MHz", Some(0))),
        ));
    }
    if let Some(l) = s.load {
        out.push(table(
            p("cpu.load"),
            "Load average",
            &["1 min", "5 min", "15 min"],
            vec![vec![json!(r1(l[0])), json!(r1(l[1])), json!(r1(l[2]))]],
            &["right", "right", "right"],
        ));
    }
}

fn memory(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    if s.mem_total > 0 {
        out.push(metric(
            p("memory.usage"),
            Kind::Gauge,
            json!({"value": r1(percent(s.mem_used, s.mem_total)), "min": 0, "max": 100}),
            "Memory usage",
            Some(pct()),
        ));
        out.push(progress(p("memory.used"), "Memory used", s.mem_used, s.mem_total));
    }
    if s.swap_total > 0 {
        out.push(progress(p("swap.used"), "Swap used", s.swap_used, s.swap_total));
    }
}

fn disks(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    if s.disks.is_empty() {
        return;
    }
    let mut rows = Vec::new();
    let mut bars = Vec::new();
    for d in s.disks.iter().take(MAX_TABLE_ROWS) {
        let used_pct = percent(d.total - d.available, d.total);
        let mount = trunc(d.mount.clone(), MAX_LABEL);
        rows.push(vec![
            json!(trunc(d.name.clone(), MAX_LABEL)),
            json!(mount),
            json!(trunc(d.fs.clone(), MAX_LABEL)),
            json!(trunc(d.kind.clone(), MAX_LABEL)),
            json!(gib(d.total - d.available)),
            json!(gib(d.total)),
            json!(r1(used_pct)),
        ]);
        if bars.len() < MAX_ROWS {
            bars.push((mount, used_pct));
        }
    }
    out.push(table(
        p("disks"),
        "Disks",
        &["Disk", "Mount", "Filesystem", "Kind", "Used GiB", "Total GiB", "Used %"],
        rows,
        &["left", "left", "left", "left", "right", "right", "right"],
    ));
    out.push(bars_metric(p("disks.usage"), "Disk usage", bars, Some(pct())));
}

fn network(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    if s.nets.is_empty() {
        return;
    }
    let rows = s
        .nets
        .iter()
        .take(MAX_TABLE_ROWS)
        .map(|n| {
            vec![
                json!(trunc(n.name.clone(), MAX_LABEL)),
                json!(human(n.rx_rate)),
                json!(human(n.tx_rate)),
                json!(human(n.rx_total as f64)),
                json!(human(n.tx_total as f64)),
            ]
        })
        .collect();
    out.push(table(
        p("network"),
        "Network",
        &["Interface", "Received/s", "Sent/s", "Total received", "Total sent"],
        rows,
        &["left", "right", "right", "right", "right"],
    ));
}

fn temperatures(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    let Some(hottest) = s.temps.first() else {
        return;
    };
    out.push(metric(
        p("temperature.max"),
        Kind::Number,
        json!(r1(f64::from(hottest.celsius))),
        "Hottest sensor",
        Some(fmt(Style::Temperature, "°C", Some(1))),
    ));
    let rows = s
        .temps
        .iter()
        .take(MAX_TABLE_ROWS)
        .map(|t| {
            vec![
                json!(trunc(t.label.clone(), MAX_LABEL)),
                json!(r1(f64::from(t.celsius))),
                t.max.map_or(Value::Null, |m| json!(r1(f64::from(m)))),
                t.critical.map_or(Value::Null, |c| json!(r1(f64::from(c)))),
            ]
        })
        .collect();
    out.push(table(
        p("temperatures"),
        "Temperatures",
        &["Sensor", "°C", "Max °C", "Critical °C"],
        rows,
        &["left", "right", "right", "right"],
    ));
    let bars = s
        .temps
        .iter()
        .take(MAX_ROWS)
        .map(|t| (trunc(t.label.clone(), MAX_LABEL), f64::from(t.celsius)))
        .collect();
    out.push(bars_metric(
        p("temperatures.bar"),
        "Sensor temperatures",
        bars,
        Some(fmt(Style::Temperature, "°C", Some(1))),
    ));
}

fn gpus(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    for (i, g) in s.gpus.iter().enumerate() {
        let name = trunc(g.name.clone(), MAX_LABEL);
        if let Some(u) = g.util {
            out.push(metric(
                p(&format!("gpu{i}.utilization")),
                Kind::Gauge,
                json!({"value": r1(f64::from(u)), "min": 0, "max": 100}),
                format!("{name} utilization"),
                Some(pct()),
            ));
        }
        if let Some(t) = g.temp_c {
            out.push(metric(
                p(&format!("gpu{i}.temperature")),
                Kind::Number,
                json!(r1(f64::from(t))),
                format!("{name} temperature"),
                Some(fmt(Style::Temperature, "°C", Some(1))),
            ));
        }
        match (g.mem_used, g.mem_total) {
            (Some(used), Some(total)) => out.push(progress(
                p(&format!("gpu{i}.memory")),
                &format!("{name} memory"),
                used,
                total,
            )),
            (Some(used), None) => out.push(metric(
                p(&format!("gpu{i}.memory.used")),
                Kind::Number,
                json!(used),
                format!("{name} memory used"),
                Some(fmt(Style::Bytes, "", None)),
            )),
            _ => {}
        }
        if let Some(w) = g.power_w {
            out.push(metric(
                p(&format!("gpu{i}.power")),
                Kind::Number,
                json!(r1(f64::from(w))),
                format!("{name} power"),
                Some(fmt(Style::Number, "W", Some(1))),
            ));
        }
        if let Some(f) = g.fan_percent {
            out.push(metric(
                p(&format!("gpu{i}.fan")),
                Kind::Gauge,
                json!({"value": f, "min": 0, "max": 100}),
                format!("{name} fan"),
                Some(pct()),
            ));
        }
        if let Some(c) = g.clock_mhz {
            out.push(metric(
                p(&format!("gpu{i}.clock")),
                Kind::Number,
                json!(c),
                format!("{name} graphics clock"),
                Some(fmt(Style::Number, "MHz", Some(0))),
            ));
        }
    }
}

fn battery(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    let Some(b) = &s.battery else {
        return;
    };
    out.push(metric(
        p("battery.charge"),
        Kind::Gauge,
        json!({"value": r1(b.percent), "min": 0, "max": 100}),
        format!("Battery charge ({})", b.state),
        Some(pct()),
    ));
    out.push(metric(
        p("battery.health"),
        Kind::Number,
        json!(r1(b.health_percent)),
        "Battery health",
        Some(pct()),
    ));
    if let Some(c) = b.cycles {
        out.push(metric(
            p("battery.cycles"),
            Kind::Number,
            json!(c),
            "Battery cycles",
            None,
        ));
    }
}

fn processes(p: &impl Fn(&str) -> String, s: &Sample, out: &mut Vec<Metric>) {
    if s.processes.is_empty() {
        return;
    }
    let rows = s
        .processes
        .iter()
        .map(|pr| {
            vec![
                json!(trunc(pr.name.clone(), MAX_LABEL)),
                json!(r1(f64::from(pr.cpu))),
                json!(human(pr.memory as f64)),
            ]
        })
        .collect();
    out.push(table(
        p("processes"),
        "Top processes by CPU",
        &["Process", "CPU %", "Memory"],
        rows,
        &["left", "right", "right"],
    ));
}

fn series(prefix: &str, s: &Sample, history: &History, out: &mut Vec<Metric>) {
    for id in series_values(s).keys().filter(|id| !id.starts_with("sensor.")) {
        let base = format!("{prefix}.{id}");
        out.extend(series_metrics(&base, id, &series_label(id, s), s, history));
    }
}

/// One trended temperature series per sensor, in label order. Keys are
/// `<prefix>.temperature.<slug>.<window>`.
fn sensor_groups(prefix: &str, s: &Sample, history: &History) -> Vec<(String, Vec<Metric>)> {
    trended_sensors(&s.temps)
        .into_iter()
        .map(|(slug, t)| {
            let id = format!("sensor.{slug}");
            let base = format!("{prefix}.temperature.{slug}");
            let label = trunc(t.label.clone(), MAX_LABEL);
            (t.label.clone(), series_metrics(&base, &id, &label, s, history))
        })
        .collect()
}

/// Keeps the longest leading run of groups that fits under `MAX_METRICS` after `fixed`
/// metrics. Every group after the first one that does not fit is dropped whole; their labels
/// are returned in order.
fn split_by_budget<T>(fixed: usize, groups: Vec<(String, Vec<T>)>) -> (Vec<T>, Vec<String>) {
    let mut used = fixed;
    let mut kept = Vec::new();
    let mut dropped = Vec::new();
    for (label, items) in groups {
        if dropped.is_empty() && used + items.len() <= MAX_METRICS {
            used += items.len();
            kept.extend(items);
        } else {
            dropped.push(label);
        }
    }
    (kept, dropped)
}

fn series_metrics(base: &str, id: &str, label: &str, s: &Sample, history: &History) -> Vec<Metric> {
    let now = s.at.timestamp();
    let mut out = Vec::new();
    for (suffix, span, bucket, desc) in WINDOWS {
        let pts = history.window(id, now, span, bucket);
        if pts.iter().all(|(_, v)| v.is_none()) {
            continue;
        }
        let points: Vec<Value> = pts
            .iter()
            .map(|(t, v)| json!({"t": ts(*t), "v": v.map(r1)}))
            .collect();
        out.push(metric(
            format!("{base}.{suffix}"),
            Kind::Timeseries,
            json!({"series": [{"label": label, "points": points}]}),
            format!("{label} · {desc}"),
            Some(series_format(id)),
        ));
    }
    out
}

fn series_label(id: &str, s: &Sample) -> String {
    let named = match id {
        "cpu" => "CPU",
        "memory" => "Memory",
        "swap" => "Swap",
        "temperature" => "Hottest sensor",
        "disk.read" => "Disk read",
        "disk.write" => "Disk write",
        "network.rx" => "Network received",
        "network.tx" => "Network sent",
        "battery" => "Battery",
        _ => "",
    };
    if !named.is_empty() {
        return named.to_string();
    }
    let Some(rest) = id.strip_prefix("gpu") else {
        return id.to_string();
    };
    let (index, is_temp) = match rest.split_once('.') {
        Some((i, "temperature")) => (i, true),
        _ => (rest, false),
    };
    let name = index
        .parse::<usize>()
        .ok()
        .and_then(|i| s.gpus.get(i))
        .map_or("GPU", |g| g.name.as_str());
    if is_temp {
        trunc(format!("{name} temperature"), MAX_LABEL)
    } else {
        trunc(name.to_string(), MAX_LABEL)
    }
}

fn series_format(id: &str) -> MetricFormat {
    if id == "temperature" || id.ends_with(".temperature") || id.starts_with("sensor.") {
        fmt(Style::Temperature, "°C", None)
    } else if id.starts_with("disk.") || id.starts_with("network.") {
        fmt(Style::Rate, "B/s", None)
    } else {
        pct()
    }
}

fn metric(
    key: String,
    kind: Kind,
    value: Value,
    label: impl Into<String>,
    format: Option<MetricFormat>,
) -> Metric {
    let mut m = Metric::new(key, kind, Some(value));
    m.label = Some(trunc(label.into(), MAX_LABEL));
    m.format = format.map(Box::new);
    m
}

fn progress(key: String, label: &str, current: u64, total: u64) -> Metric {
    metric(
        key,
        Kind::Progress,
        json!({"current": current, "total": total}),
        label,
        Some(fmt(Style::Bytes, "", None)),
    )
}

fn bars_metric(key: String, label: &str, bars: Vec<(String, f64)>, format: Option<MetricFormat>) -> Metric {
    let bars: Vec<Value> = bars
        .into_iter()
        .map(|(l, v)| json!({"label": trunc(l, MAX_LABEL), "value": r1(v)}))
        .collect();
    metric(key, Kind::Bar, json!({"bars": bars}), label, format)
}

fn table(key: String, label: &str, columns: &[&str], rows: Vec<Vec<Value>>, align: &[&str]) -> Metric {
    metric(
        key,
        Kind::Table,
        json!({"columns": columns, "rows": rows, "align": align}),
        label,
        None,
    )
}

fn row<const N: usize>(cells: [Value; N]) -> Vec<Value> {
    cells.into()
}

fn pct() -> MetricFormat {
    fmt(Style::Number, "%", Some(1))
}

fn fmt(style: Style, unit: &str, precision: Option<i32>) -> MetricFormat {
    let mut f = MetricFormat::new(style);
    f.unit = (!unit.is_empty()).then(|| unit.to_string());
    f.precision = precision;
    f
}

fn percent(used: u64, total: u64) -> f64 {
    used as f64 / total as f64 * 100.0
}

fn r1(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}

fn gib(bytes: u64) -> f64 {
    r1(bytes as f64 / (1024.0 * 1024.0 * 1024.0))
}

fn human(bytes: f64) -> String {
    const KIB: f64 = 1024.0;
    if bytes < KIB {
        format!("{bytes:.0} B")
    } else if bytes < KIB * KIB {
        format!("{:.1} KiB", bytes / KIB)
    } else if bytes < KIB * KIB * KIB {
        format!("{:.1} MiB", bytes / (KIB * KIB))
    } else {
        format!("{:.1} GiB", bytes / (KIB * KIB * KIB))
    }
}

fn ts(secs: i64) -> String {
    Utc.timestamp_opt(secs, 0)
        .single()
        .map(|d| d.to_rfc3339_opts(SecondsFormat::Secs, true))
        .unwrap_or_default()
}

fn trunc(s: String, max: usize) -> String {
    if s.chars().count() <= max {
        s
    } else {
        s.chars().take(max).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_drops_whole_trailing_sensors_deterministically() {
        let groups: Vec<(String, Vec<u8>)> = (0..100)
            .map(|i| (format!("sensor {i:03}"), vec![0u8; 3]))
            .collect();
        let (kept, dropped) = split_by_budget(52, groups.clone());
        assert_eq!(kept.len(), 204);
        assert!(52 + kept.len() <= MAX_METRICS);
        assert_eq!(dropped.len(), 32);
        assert_eq!(dropped.first().map(String::as_str), Some("sensor 068"));
        assert_eq!(dropped.last().map(String::as_str), Some("sensor 099"));
        let (_, again) = split_by_budget(52, groups);
        assert_eq!(again, dropped);
    }
}
