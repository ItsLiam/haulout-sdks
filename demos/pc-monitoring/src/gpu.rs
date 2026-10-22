use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};

/// Upper bound on GPUs published, keeping the metric catalogue bounded.
const MAX_GPUS: usize = 8;

pub struct GpuSample {
    pub name: String,
    pub util: Option<f32>,
    pub temp_c: Option<f32>,
    pub mem_used: Option<u64>,
    pub mem_total: Option<u64>,
    pub power_w: Option<f32>,
    pub fan_percent: Option<u32>,
    pub clock_mhz: Option<u32>,
}

/// NVIDIA devices via NVML when available, otherwise Apple GPUs via ioreg.
pub fn sample(nvml: Option<&Nvml>) -> Vec<GpuSample> {
    let mut gpus = match nvml {
        Some(n) => nvidia(n),
        None => Vec::new(),
    };
    if gpus.is_empty() {
        gpus = apple();
    }
    gpus.truncate(MAX_GPUS);
    gpus
}

fn nvidia(nvml: &Nvml) -> Vec<GpuSample> {
    let Ok(count) = nvml.device_count() else {
        return Vec::new();
    };
    (0..count)
        .filter_map(|i| nvml.device_by_index(i).ok().map(|d| (i, d)))
        .map(|(i, d)| GpuSample {
            name: d.name().unwrap_or_else(|_| format!("NVIDIA GPU {i}")),
            util: d.utilization_rates().ok().map(|u| u.gpu as f32),
            temp_c: d.temperature(TemperatureSensor::Gpu).ok().map(|t| t as f32),
            mem_used: d.memory_info().ok().map(|m| m.used),
            mem_total: d.memory_info().ok().map(|m| m.total),
            power_w: d.power_usage().ok().map(|mw| mw as f32 / 1000.0),
            fan_percent: d.fan_speed(0).ok(),
            clock_mhz: d.clock_info(Clock::Graphics).ok(),
        })
        .collect()
}

#[cfg(target_os = "macos")]
fn apple() -> Vec<GpuSample> {
    use plist::Value;

    let Ok(output) = std::process::Command::new("ioreg")
        .args(["-r", "-c", "IOAccelerator", "-d", "1", "-a"])
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let Ok(Value::Array(devices)) = Value::from_reader_xml(output.stdout.as_slice()) else {
        return Vec::new();
    };

    devices
        .iter()
        .filter_map(Value::as_dictionary)
        .map(|device| {
            let perf = device
                .get("PerformanceStatistics")
                .and_then(Value::as_dictionary);
            let stat = |key: &str| perf.and_then(|p| p.get(key)).and_then(num);
            GpuSample {
                name: device
                    .get("model")
                    .and_then(Value::as_string)
                    .unwrap_or("Apple GPU")
                    .to_string(),
                util: stat("Device Utilization %").map(|v| v as f32),
                temp_c: None,
                mem_used: stat("In use system memory").map(|v| v as u64),
                mem_total: None,
                power_w: None,
                fan_percent: None,
                clock_mhz: None,
            }
        })
        .collect()
}

#[cfg(target_os = "macos")]
fn num(v: &plist::Value) -> Option<f64> {
    match v {
        plist::Value::Integer(i) => i
            .as_signed()
            .map(|x| x as f64)
            .or_else(|| i.as_unsigned().map(|x| x as f64)),
        plist::Value::Real(r) => Some(*r),
        _ => None,
    }
}

#[cfg(not(target_os = "macos"))]
fn apple() -> Vec<GpuSample> {
    Vec::new()
}
