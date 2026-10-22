use std::collections::{BTreeMap, HashSet, VecDeque};
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::sample::{Sample, TempSample};

const FORMAT_VERSION: u32 = 1;
/// Samples older than this are pruned; the longest published window is 12 h.
const MAX_AGE_SECS: i64 = 12 * 3600;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StoredSample {
    pub t: i64,
    pub v: BTreeMap<String, f64>,
}

#[derive(Default)]
pub struct History {
    samples: VecDeque<StoredSample>,
}

#[derive(Serialize)]
struct FileRef<'a> {
    version: u32,
    samples: &'a VecDeque<StoredSample>,
}

#[derive(Deserialize)]
struct FileOwned {
    version: u32,
    samples: Vec<StoredSample>,
}

/// Flattens a sample into the series values that history tracks. Keys are the
/// series ids used in metric keys (for example `cpu`, `disk.read`, `gpu0`).
pub fn series_values(s: &Sample) -> BTreeMap<String, f64> {
    let mut v = BTreeMap::new();
    let mut put = |k: &str, x: f64| {
        if x.is_finite() {
            v.insert(k.to_string(), x);
        }
    };
    put("cpu", f64::from(s.cpu_total));
    if s.mem_total > 0 {
        put("memory", pct(s.mem_used, s.mem_total));
    }
    if s.swap_total > 0 {
        put("swap", pct(s.swap_used, s.swap_total));
    }
    if let Some(t) = s.temps.first() {
        put("temperature", f64::from(t.celsius));
    }
    for (slug, t) in trended_sensors(&s.temps) {
        put(&format!("sensor.{slug}"), f64::from(t.celsius));
    }
    put("disk.read", s.disks.iter().map(|d| d.read_rate).sum());
    put("disk.write", s.disks.iter().map(|d| d.write_rate).sum());
    put("network.rx", s.nets.iter().map(|n| n.rx_rate).sum());
    put("network.tx", s.nets.iter().map(|n| n.tx_rate).sum());
    for (i, g) in s.gpus.iter().enumerate() {
        if let Some(u) = g.util {
            put(&format!("gpu{i}"), f64::from(u));
        }
        if let Some(t) = g.temp_c {
            put(&format!("gpu{i}.temperature"), f64::from(t));
        }
    }
    if let Some(b) = &s.battery {
        put("battery", b.percent);
    }
    v
}

/// Longest slug that keeps `<prefix>.temperature.<slug>.12h` within 64 characters.
const MAX_SLUG: usize = 27;

/// Returns `(slug, sensor)` for every sensor, sorted by label so ids stay stable across
/// ticks. Slugs are deduplicated with `-2`, `-3`, and so on.
pub fn trended_sensors(temps: &[TempSample]) -> Vec<(String, &TempSample)> {
    let mut sorted: Vec<&TempSample> = temps.iter().collect();
    sorted.sort_by(|a, b| a.label.cmp(&b.label));
    let mut used = HashSet::new();
    sorted
        .into_iter()
        .map(|t| {
            let base = slug(&t.label);
            let mut candidate = base.clone();
            let mut n = 2;
            while !used.insert(candidate.clone()) {
                let suffix = format!("-{n}");
                candidate = format!("{}{suffix}", cut(&base, MAX_SLUG - suffix.len()));
                n += 1;
            }
            (candidate, t)
        })
        .collect()
}

fn slug(label: &str) -> String {
    let mut out = String::new();
    for c in label.chars().flat_map(char::to_lowercase) {
        let c = if c.is_ascii_lowercase() || c.is_ascii_digit() { c } else { '-' };
        if !(c == '-' && out.ends_with('-')) {
            out.push(c);
        }
    }
    let cut_slug = cut(out.trim_start_matches('-'), MAX_SLUG).to_string();
    if cut_slug.is_empty() { "sensor".to_string() } else { cut_slug }
}

/// Truncates an ASCII slug to `max` bytes and drops trailing dashes.
fn cut(s: &str, max: usize) -> &str {
    s[..max.min(s.len())].trim_end_matches('-')
}

fn pct(used: u64, total: u64) -> f64 {
    used as f64 / total as f64 * 100.0
}

impl History {
    /// Loads history from disk. A missing file yields an empty history; an
    /// unreadable or unparsable file is reported and ignored.
    pub fn load(path: &Path) -> History {
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return History::default(),
            Err(_) => {
                eprintln!("warning: ignoring unreadable history at {}", path.display());
                return History::default();
            }
        };
        let file: FileOwned = match serde_json::from_slice::<FileOwned>(&bytes) {
            Ok(f) if f.version == FORMAT_VERSION => f,
            _ => {
                eprintln!("warning: ignoring unreadable history at {}", path.display());
                return History::default();
            }
        };
        let mut history = History::default();
        for s in file.samples {
            if history.samples.back().is_none_or(|last| s.t > last.t) {
                history.samples.push_back(s);
            }
        }
        history
    }

    pub fn push(&mut self, s: &Sample) {
        self.push_values(s.at.timestamp(), series_values(s));
    }

    /// Appends a sample, ignoring any whose timestamp does not advance.
    pub fn push_values(&mut self, t: i64, v: BTreeMap<String, f64>) {
        if self.samples.back().is_none_or(|last| t > last.t) {
            self.samples.push_back(StoredSample { t, v });
        }
    }

    pub fn prune(&mut self, now: i64) {
        while self.samples.front().is_some_and(|s| s.t < now - MAX_AGE_SECS) {
            self.samples.pop_front();
        }
    }

    /// Writes history atomically via a temporary file followed by a rename.
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        let mut tmp = path.as_os_str().to_owned();
        tmp.push(".tmp");
        let bytes = serde_json::to_vec(&FileRef {
            version: FORMAT_VERSION,
            samples: &self.samples,
        })
        .map_err(io::Error::other)?;
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(tmp, path)
    }

    /// Returns `(bucket_start, value)` points for one series id. With no
    /// bucket size, every stored sample in the span is returned raw. With a
    /// bucket size, epoch-aligned buckets are averaged over present values and
    /// empty buckets yield `None`.
    pub fn window(
        &self,
        id: &str,
        now: i64,
        span: i64,
        bucket: Option<i64>,
    ) -> Vec<(i64, Option<f64>)> {
        match bucket {
            None => self
                .samples
                .iter()
                .filter(|s| s.t >= now - span)
                .map(|s| (s.t, s.v.get(id).copied()))
                .collect(),
            Some(b) => {
                let first = (now - span).div_euclid(b) * b;
                let last = now.div_euclid(b) * b;
                (first..=last)
                    .step_by(b as usize)
                    .map(|start| {
                        let vals: Vec<f64> = self
                            .samples
                            .iter()
                            .filter(|s| s.t >= start && s.t < start + b)
                            .filter_map(|s| s.v.get(id).copied())
                            .collect();
                        let avg = (!vals.is_empty())
                            .then(|| vals.iter().sum::<f64>() / vals.len() as f64);
                        (start, avg)
                    })
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i64 = 1_800_000_000;

    fn one(id: &str, x: f64) -> BTreeMap<String, f64> {
        BTreeMap::from([(id.to_string(), x)])
    }

    #[test]
    fn bucket_averages_values_at_bucket_start() {
        let mut h = History::default();
        h.push_values(NOW - 200, one("cpu", 10.0));
        h.push_values(NOW - 100, one("cpu", 20.0));
        let pts = h.window("cpu", NOW, 21600, Some(300));
        assert!(pts.contains(&(NOW - 300, Some(15.0))));
    }

    #[test]
    fn empty_bucket_is_none() {
        let mut h = History::default();
        h.push_values(NOW, one("cpu", 10.0));
        let pts = h.window("cpu", NOW, 21600, Some(300));
        assert_eq!(pts.first(), Some(&(NOW - 21600, None)));
        assert_eq!(pts.len(), 73);
    }

    #[test]
    fn old_sample_excluded_from_raw_hour_but_in_six_hours() {
        let mut h = History::default();
        h.push_values(NOW - 7200, one("cpu", 5.0));
        assert!(h.window("cpu", NOW, 3600, None).is_empty());
        let pts = h.window("cpu", NOW, 21600, Some(300));
        assert!(pts.contains(&(NOW - 7200, Some(5.0))));
    }

    #[test]
    fn prune_drops_samples_past_twelve_hours() {
        let mut h = History::default();
        h.push_values(NOW - 43201, one("cpu", 1.0));
        h.push_values(NOW - 43199, one("cpu", 2.0));
        h.prune(NOW);
        assert_eq!(h.samples.len(), 1);
        assert_eq!(h.samples[0].t, NOW - 43199);
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = std::env::temp_dir().join(format!("haulout-pc-test-{}", std::process::id()));
        let path = dir.join("state.json");
        let mut h = History::default();
        h.push_values(NOW - 10, one("cpu", 42.5));
        h.save(&path).expect("save");
        let loaded = History::load(&path);
        assert_eq!(loaded.samples, h.samples);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unparsable_file_loads_empty() {
        let dir = std::env::temp_dir().join(format!("haulout-pc-bad-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("state.json");
        std::fs::write(&path, "not json").expect("write");
        assert!(History::load(&path).samples.is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn trended_sensor_slugs_are_sanitised_and_deduped() {
        let t = |label: &str| TempSample { label: label.into(), celsius: 50.0, max: None, critical: None };
        let temps = vec![t("PMU tdie1"), t("PMU tdie-1"), t("PMU  tdie1"), t("  ")];
        let slugs: Vec<String> = trended_sensors(&temps).into_iter().map(|(s, _)| s).collect();
        assert_eq!(slugs, ["sensor", "pmu-tdie1", "pmu-tdie-1", "pmu-tdie1-2"]);
    }

    #[test]
    fn trended_sensors_keeps_every_sensor_in_label_order() {
        let temps: Vec<TempSample> = (0..45)
            .map(|i| TempSample { label: format!("s{i:02}"), celsius: 1.0, max: None, critical: None })
            .collect();
        let out = trended_sensors(&temps);
        assert_eq!(out.len(), 45);
        assert_eq!(out.last().map(|(s, _)| s.as_str()), Some("s44"));
    }

    #[test]
    fn long_colliding_slugs_stay_within_key_limit() {
        let long = "x".repeat(60);
        let temps = vec![
            TempSample { label: long.clone(), celsius: 1.0, max: None, critical: None },
            TempSample { label: long, celsius: 1.0, max: None, critical: None },
        ];
        let slugs: Vec<String> = trended_sensors(&temps).into_iter().map(|(s, _)| s).collect();
        assert_eq!(slugs[0], "x".repeat(27));
        assert_eq!(slugs[1], format!("{}-2", "x".repeat(25)));
    }
}
