# PC monitoring demo

A Rust producer that samples this machine and publishes its live state to a Haulout product dashboard: CPU, memory, swap, disks, network, temperatures, GPUs, battery, and top processes. Metrics are upserted with `put_metrics_by_key` from the Haulout Rust SDK.

## What it samples

| Platform | Temperatures | GPU | Battery |
| --- | --- | --- | --- |
| Linux | hwmon | NVIDIA via NVML | starship-battery |
| Windows | WMI where exposed (often none without admin) | NVIDIA via NVML | starship-battery |
| macOS | SMC | Apple GPU via `ioreg` | starship-battery |

AMD and Intel discrete GPU utilisation is not sampled.

Apple GPU temperatures are not exposed by `ioreg` or sysinfo, so Apple GPUs publish no temperature. The sensor table shows SoC PMU sensors only.

## Setup

```sh
cp .env.example .env
# set HAULOUT_PRODUCT_KEY in .env to a real product key
cargo run --release
```

`HAULOUT_PRODUCT_KEY` may also come from the environment. It is sent in the request body as `p_product_key`, never in an Authorization header.

## Flags

- `--interval <seconds>`: time between samples, default 60, minimum 5.
- `--once`: publish one sample and exit.
- `--dry-run`: print the metrics as JSON and exit. No key is needed, nothing is published, and no history is written.
- `--prefix <prefix>`: namespace for metric keys. Defaults to the hostname, lowercased, with invalid characters replaced by `-` and truncated to 20 characters. When several machines share one product, give each a distinct prefix.
- `--state-file <path>`: history file location.

## History and windows

Samples are kept in a history file so the dashboard can show trends. The default location is `<local data dir>/haulout-pc-monitoring/<prefix>.json`, or `./.haulout-pc-monitoring-<prefix>.json` when no data directory is available. History is kept for 12 hours.

Timeseries metrics are published for three windows:

- `1h`: raw samples from the last hour.
- `6h`: 5-minute bucket averages over the last 6 hours.
- `12h`: 10-minute bucket averages over the last 12 hours.

## Metric keys

All keys are prefixed with `<prefix>.`.

Snapshot metrics:

- `status`: online state. Shown as neutral when offline.
- `host`: hostname, OS, kernel, CPU brand, core counts, boot time.
- `uptime`
- `cpu.usage`, `cpu.cores`, `cpu.frequency`, `cpu.load`
- `memory.usage`, `memory.used`, `swap.used`
- `disks`, `disks.usage`
- `network`
- `temperature.max`, `temperatures`, `temperatures.bar`, and `temperature.<slug>` per sensor (see below)
- `gpu{i}.utilization`, `gpu{i}.memory.used`, and the other GPU fields the device reports (`temperature`, `power`, `fan`, `clock`)
- `battery.charge`, `battery.health`, `battery.cycles`
- `processes`

Timeseries metrics, each with `.1h`, `.6h`, and `.12h` suffixes:

- `cpu`, `memory`, `swap`, `temperature`, `battery`, `gpu{i}`
- `disk.read`, `disk.write`
- `network.rx`, `network.tx`

Metrics whose source is absent are omitted rather than sent as zero. Stale keys (a GPU or battery that disappears) are deleted on the next publish.

### Per-sensor temperature series

Every sysinfo temperature sensor gets its own trended series with the same 1h, 6h, and 12h windows as the others: `<prefix>.temperature.<slug>.1h|6h|12h`. The label is the sensor label and the unit is °C. Any sensor, including one whose label mentions a GPU, is handled the same way. NVIDIA GPU temperatures come separately from NVML as `gpu{i}.temperature`.

- Slug: the label is lowercased, characters outside `[a-z0-9]` become `-`, repeated and edge dashes are removed, and the result is truncated to 27 characters. An empty result becomes `sensor`.
- Collisions get `-2`, `-3`, and so on. Sensors are ordered by label, so ids stay stable across ticks. Sensors with identical labels keep sysinfo's order.
- The longest key, `<prefix>.temperature.<27 chars>.12h`, stays within the 64-character limit.

### Metric budget

A product holds at most 256 metrics. The producer always publishes every non-sensor metric and fills the remaining room with per-sensor series in label order. A sensor's three windows are kept or dropped together. Once one sensor does not fit, that sensor and every later one in label order are dropped, so only as many as needed are dropped. Dropped keys are not published, so the stale-key delete removes any that were published on an earlier tick. A warning goes to stderr only when the dropped set changes.

## Errors

- A 400 or 403 response is permanent. The process prints `permanent error <code>: <message>` and exits 1.
- Other errors print `publish failed, retrying next tick` and the loop continues. With `--once` they exit 1.
- Ctrl-C publishes `<prefix>.status` as offline with the time, leaves other metrics in place, and exits 0.

## Tests

```sh
cargo test
```
