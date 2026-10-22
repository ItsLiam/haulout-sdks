use std::collections::HashSet;
use std::time::Instant;

use chrono::{DateTime, Utc};
use starship_battery::Manager;
use sysinfo::{Components, Disks, MINIMUM_CPU_UPDATE_INTERVAL, Networks, System};

use crate::gpu::{self, GpuSample};

/// Owns every sysinfo handle plus the optional NVML and battery managers, so
/// rate deltas and CPU usage are computed across consecutive refreshes.
pub struct Sampler {
    system: System,
    disks: Disks,
    networks: Networks,
    components: Components,
    nvml: Option<nvml_wrapper::Nvml>,
    battery: Option<Manager>,
    last: Instant,
}

pub struct Sample {
    pub at: DateTime<Utc>,
    pub host: HostInfo,
    pub cpu_total: f32,
    pub cpu_cores: Vec<f32>,
    pub cpu_freq_mhz: u64,
    pub load: Option<[f64; 3]>,
    pub mem_used: u64,
    pub mem_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub disks: Vec<DiskSample>,
    pub nets: Vec<NetSample>,
    pub temps: Vec<TempSample>,
    pub gpus: Vec<GpuSample>,
    pub battery: Option<BatterySample>,
    pub processes: Vec<ProcSample>,
}

pub struct HostInfo {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub cpu_brand: String,
    pub physical_cores: Option<usize>,
    pub logical_cores: usize,
    pub uptime_secs: u64,
    pub boot_time: i64,
}

pub struct DiskSample {
    pub name: String,
    pub mount: String,
    pub fs: String,
    pub kind: String,
    pub total: u64,
    pub available: u64,
    pub read_rate: f64,
    pub write_rate: f64,
}

pub struct NetSample {
    pub name: String,
    pub rx_rate: f64,
    pub tx_rate: f64,
    pub rx_total: u64,
    pub tx_total: u64,
}

pub struct TempSample {
    pub label: String,
    pub celsius: f32,
    pub max: Option<f32>,
    pub critical: Option<f32>,
}

pub struct BatterySample {
    pub percent: f64,
    pub state: String,
    pub health_percent: f64,
    pub cycles: Option<u32>,
}

pub struct ProcSample {
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
}

impl Sampler {
    pub fn new() -> Self {
        Self {
            system: System::new(),
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            components: Components::new_with_refreshed_list(),
            nvml: nvml_wrapper::Nvml::init().ok(),
            battery: Manager::new().ok(),
            last: Instant::now(),
        }
    }

    /// Performs an initial refresh, then waits so the first CPU reading and the
    /// rate deltas taken by `sample` are valid.
    pub fn prime(&mut self) {
        self.refresh();
        self.last = Instant::now();
        std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    }

    fn refresh(&mut self) {
        self.system.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);
        self.components.refresh(true);
    }

    pub fn sample(&mut self) -> Sample {
        self.refresh();
        let elapsed = self.last.elapsed().as_secs_f64().max(1e-3);
        self.last = Instant::now();

        let cpus = self.system.cpus();
        let host = HostInfo {
            hostname: System::host_name().unwrap_or_else(|| "unknown-host".to_string()),
            os: System::long_os_version().unwrap_or_else(|| "unknown".to_string()),
            kernel: System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
            cpu_brand: cpus.first().map(|c| c.brand().to_string()).unwrap_or_default(),
            physical_cores: System::physical_core_count(),
            logical_cores: cpus.len(),
            uptime_secs: System::uptime(),
            boot_time: System::boot_time() as i64,
        };
        let cpu_freq_mhz = if cpus.is_empty() {
            0
        } else {
            cpus.iter().map(|c| c.frequency()).sum::<u64>() / cpus.len() as u64
        };
        let cpu_cores = cpus.iter().map(|c| c.cpu_usage()).collect();

        let load = if cfg!(windows) {
            None
        } else {
            let l = System::load_average();
            Some([l.one, l.five, l.fifteen])
        };

        let mut seen = HashSet::new();
        let mut disks = Vec::new();
        for d in self.disks.list() {
            let total = d.total_space();
            if total == 0 {
                continue;
            }
            let name = d.name().to_string_lossy().into_owned();
            if !seen.insert(name.clone()) {
                continue;
            }
            let usage = d.usage();
            disks.push(DiskSample {
                name,
                mount: d.mount_point().display().to_string(),
                fs: d.file_system().to_string_lossy().into_owned(),
                kind: format!("{:?}", d.kind()),
                total,
                available: d.available_space(),
                read_rate: usage.read_bytes as f64 / elapsed,
                write_rate: usage.written_bytes as f64 / elapsed,
            });
        }

        let mut nets: Vec<NetSample> = self
            .networks
            .list()
            .iter()
            .filter(|(name, d)| {
                !name.starts_with("lo") && d.total_received() + d.total_transmitted() > 0
            })
            .map(|(name, d)| NetSample {
                name: name.clone(),
                rx_rate: d.received() as f64 / elapsed,
                tx_rate: d.transmitted() as f64 / elapsed,
                rx_total: d.total_received(),
                tx_total: d.total_transmitted(),
            })
            .collect();
        nets.sort_by(|a, b| a.name.cmp(&b.name));

        let mut temps: Vec<TempSample> = self
            .components
            .list()
            .iter()
            .filter_map(|c| {
                let t = c.temperature()?;
                (t.is_finite() && t > 0.0).then(|| TempSample {
                    label: c.label().to_string(),
                    celsius: t,
                    max: c.max(),
                    critical: c.critical(),
                })
            })
            .collect();
        temps.sort_by(|a, b| b.celsius.total_cmp(&a.celsius));

        let mut processes: Vec<ProcSample> = self
            .system
            .processes()
            .values()
            .map(|p| ProcSample {
                name: p.name().to_string_lossy().into_owned(),
                cpu: p.cpu_usage(),
                memory: p.memory(),
            })
            .collect();
        processes.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
        processes.truncate(10);

        Sample {
            at: Utc::now(),
            host,
            cpu_total: self.system.global_cpu_usage(),
            cpu_cores,
            cpu_freq_mhz,
            load,
            mem_used: self.system.used_memory(),
            mem_total: self.system.total_memory(),
            swap_used: self.system.used_swap(),
            swap_total: self.system.total_swap(),
            disks,
            nets,
            temps,
            gpus: gpu::sample(self.nvml.as_ref()),
            battery: battery(self.battery.as_ref()),
            processes,
        }
    }
}

fn battery(manager: Option<&Manager>) -> Option<BatterySample> {
    let mut batteries = manager?.batteries().ok()?;
    let b = batteries.next()?.ok()?;
    Some(BatterySample {
        percent: f64::from(b.state_of_charge().value) * 100.0,
        state: b.state().to_string(),
        health_percent: f64::from(b.state_of_health().value) * 100.0,
        cycles: b.cycle_count(),
    })
}
