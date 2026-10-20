# Metrics

A product publishes metrics: named values, each keyed by `key` within the product. The SDK exposes four RPCs; the two metric operations are:

- `put_metrics_by_key` (`PutMetricsByKey` in Go) writes 1–100 metrics per call.
- `delete_metrics_by_key` (`DeleteMetricsByKey` in Go) removes 1–256 keys per call.

## Semantics

- A put replaces the metric's whole value, label, and format. Omitted `label` and `format` become null.
- Writes are last-write-wins. Rewriting an identical metric is a no-op and returns the existing revision.
- Haulout keeps no history. Each metric is only its current value; a `timeseries` value carries its own points.
- Each put returns one row per input metric, in input order: `key`, `revision`, `updated_at`.
- Unknown keys in a delete are ignored, so retries are safe. Each delete returns one row per key actually deleted.

## Metric object

| Field | Required | Rule |
| --- | --- | --- |
| `key` | yes | `^[a-z][a-z0-9_.-]{0,63}$`, for example `facility.temperature`. Unique within the product. |
| `kind` | yes | One of the nine kinds below. |
| `value` | yes | Must match `kind`. |
| `label` | no | 1–100 characters. |
| `format` | no | `{"style", "precision"?, "unit"?}`; see below. |

Unknown top-level fields are rejected.

## Format

- `style` (required): `number`, `percent`, `bytes`, `duration`, `rate`, `temperature`, `date`, `time`, or `dateTime`.
- `precision` (optional): integer from 0 to 6.
- `unit` (optional): string up to 32 characters.

Unknown format fields are rejected. Use `"percent"` only for fractions (0.42 shows as 42%). For values already in percentage points, use `"style": "number"` with `"unit": "%"`.

## Kinds

| kind | `value` |
| --- | --- |
| `number` | a JSON number |
| `text` | a string up to 4096 characters |
| `status` | `{"state": "neutral" \| "positive" \| "warning" \| "critical", "text"}`; `text` up to 256 characters |
| `progress` | `{"current": n >= 0, "total": n > 0}` |
| `gauge` | `{"value": n, "min": n, "max": n}` with `min < max` |
| `list` | `{"items": [{"id", "label", "detail"?, "done"?, "progress"?, "state"?}]}`, up to 500 items; `id` 1–128 characters, unique in the list; `label` 1–256; `detail` up to 1024 |
| `timeseries` | `{"series": [{"label", "color"?, "points": [{"t", "v"}]}]}`, 1–8 series of up to 2000 points each |
| `bar` | `{"bars": [{"label", "value", "color"?}]}`, 1–100 bars |
| `table` | `{"columns": ["..."], "align"?: ["left"\|"center"\|"right", ...], "rows": [[scalar, ...]]}`, 1–16 columns and up to 500 rows; every row has one cell per column; `align`, when present, has one entry per column (default left) |

Shared rules:

- Labels (metric, series, bar, and column labels) are 1–100 characters. List item labels are 1–256.
- `color` is optional and must be `#RRGGBB` (for example `"#005C55"`).
- Timeseries `t` is an RFC 3339 timestamp with `Z` or a `±HH:MM` offset (for example `"2026-10-16T09:00:00Z"`). `v` is a number, or `null` for a gap in the line.
- Table cells are strings, numbers, booleans, or `null`.

Each kind as a metric object (valid input, one array):

```json
[
  {"key": "line.throughput", "kind": "number", "label": "Throughput",
   "format": {"style": "rate", "unit": "units/h", "precision": 0}, "value": 1280},
  {"key": "line.status_text", "kind": "text", "label": "Shift note", "value": "Changeover at 14:00"},
  {"key": "line.state", "kind": "status", "label": "Line state",
   "value": {"state": "positive", "text": "Running"}},
  {"key": "batch.progress", "kind": "progress", "label": "Batch",
   "value": {"current": 72, "total": 120}},
  {"key": "tank.level", "kind": "gauge", "label": "Tank level",
   "format": {"style": "percent", "precision": 0}, "value": {"value": 0.64, "min": 0, "max": 1}},
  {"key": "maintenance.tasks", "kind": "list", "label": "Maintenance",
   "value": {"items": [
     {"id": "belt", "label": "Replace belt", "done": true},
     {"id": "filter", "label": "Clean filter", "detail": "Due Friday", "state": "warning"}
   ]}},
  {"key": "facility.temperature.history", "kind": "timeseries", "label": "Temperature",
   "format": {"style": "temperature", "unit": "°C", "precision": 1},
   "value": {"series": [{"label": "Hall 2", "color": "#005C55", "points": [
     {"t": "2026-10-16T09:00:00Z", "v": 21.4},
     {"t": "2026-10-16T09:05:00Z", "v": null},
     {"t": "2026-10-16T09:10:00Z", "v": 21.9}
   ]}]}},
  {"key": "dock.arrivals", "kind": "bar", "label": "Arrivals by dock",
   "value": {"bars": [{"label": "Dock A", "value": 12}, {"label": "Dock B", "value": 7, "color": "#BDC9C6"}]}},
  {"key": "dock.schedule", "kind": "table", "label": "Schedule",
   "value": {"columns": ["Dock", "Next"], "rows": [["Dock A", "09:30"], ["Dock B", null]]}}
]
```

## Example: put

Go (inside a function with `ctx context.Context`; imports `context`, `os`, `fmt`, `time`, and `haulout "github.com/ItsLiam/haulout-sdks/go"`):

```go
client := haulout.NewAPIClient(haulout.NewConfiguration())

label := "Temperature"
metric := haulout.NewMetric("facility.temperature", "number", 21.4)
metric.Label = &label
request := haulout.NewPutMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), []haulout.Metric{*metric})
rows, _, err := client.DefaultAPI.PutMetricsByKey(ctx).PutMetricsByKeyRequest(*request).Execute()
if err != nil {
	return err
}
fmt.Println(rows[0].Key, rows[0].Revision)
```

Python (`import os`):

```python
import os

from haulout import DefaultApi, Metric, PutMetricsByKeyRequest

api = DefaultApi()
request = PutMetricsByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_metrics=[
        Metric(key="facility.temperature", kind="number", label="Temperature",
               format={"style": "temperature", "unit": "°C", "precision": 1},
               value=21.4),
    ],
)
rows = api.put_metrics_by_key(put_metrics_by_key_request=request)
print(rows[0].key, rows[0].revision)
```

Rust (async; `serde_json` and `tokio` are your dependencies):

```rust
use haulout::apis::{configuration::Configuration, default_api};
use haulout::models::{self, metric::Kind, metric_format::Style};

let config = Configuration::new();
let mut format = models::MetricFormat::new(Style::Temperature);
format.unit = Some("°C".into());
format.precision = Some(1);
let mut metric = models::Metric::new(
    "facility.temperature".into(),
    Kind::Number,
    Some(serde_json::json!(21.4)),
);
metric.label = Some("Temperature".into());
metric.format = Some(Box::new(format));
let request = models::PutMetricsByKeyRequest::new(
    std::env::var("HAULOUT_PRODUCT_KEY")?,
    vec![metric],
);
let rows = default_api::put_metrics_by_key(&config, request).await?;
println!("{} {}", rows[0].key, rows[0].revision);
```

## Example: timeseries with a gap

Send the whole series each time you publish. A `null` value draws a gap.

Python:

```python
points = [
    {"t": "2026-10-16T09:00:00Z", "v": 21.4},
    {"t": "2026-10-16T09:05:00Z", "v": None},  # gap
    {"t": "2026-10-16T09:10:00Z", "v": 21.9},
]
request = PutMetricsByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_metrics=[Metric(
        key="facility.temperature.history", kind="timeseries", label="Temperature",
        format={"style": "temperature", "unit": "°C", "precision": 1},
        value={"series": [{"label": "Hall 2", "points": points}]},
    )],
)
api.put_metrics_by_key(put_metrics_by_key_request=request)
```

Go:

```go
points := []map[string]interface{}{
	{"t": "2026-10-16T09:00:00Z", "v": 21.4},
	{"t": "2026-10-16T09:05:00Z", "v": nil}, // gap
	{"t": "2026-10-16T09:10:00Z", "v": 21.9},
}
value := map[string]interface{}{
	"series": []interface{}{
		map[string]interface{}{"label": "Hall 2", "points": points},
	},
}
label := "Temperature"
metric := haulout.NewMetric("facility.temperature.history", "timeseries", value)
metric.Label = &label
request := haulout.NewPutMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), []haulout.Metric{*metric})
_, _, err := client.DefaultAPI.PutMetricsByKey(ctx).PutMetricsByKeyRequest(*request).Execute()
if err != nil {
	return err
}
```

## Example: delete

Go:

```go
request := haulout.NewDeleteMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), []string{"facility.temperature"})
deleted, _, err := client.DefaultAPI.DeleteMetricsByKey(ctx).DeleteMetricsByKeyRequest(*request).Execute()
if err != nil {
	return err
}
fmt.Println(len(deleted), "deleted")
```

Python:

```python
from haulout import DeleteMetricsByKeyRequest

api.delete_metrics_by_key(
    delete_metrics_by_key_request=DeleteMetricsByKeyRequest(
        p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
        p_keys=["facility.temperature"],
    )
)
```

## Limits

- Put: 1–100 metrics and at most 1 MiB per batch.
- Product: at most 256 metrics. A put that would exceed this fails and writes nothing.
- Delete: 1–256 keys.

Violations return `22023`; see [`errors.md`](errors.md).
