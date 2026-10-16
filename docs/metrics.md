# Metrics

A product publishes metrics: named values, each keyed by `key` within the product. The SDK has two operations.

- `put_metrics_by_key` (`PutMetricsByKey` in Go) writes 1–100 metrics per call.
- `delete_metrics_by_key` (`DeleteMetricsByKey` in Go) removes 1–256 keys per call.

## Semantics

- Each write replaces the metric's whole value, label, and format. Omitted `label` and `format` become null.
- Writes are last-write-wins. Rewriting an identical value is a no-op and returns the existing revision.
- A product holds at most 256 metrics. A put batch is at most 1 MiB.
- Unknown keys in a delete are ignored, so retries are safe.
- Each put returns one row per input metric, in input order: `key`, `revision`, `updated_at`.
- Each delete returns one row per key actually deleted.

## Kinds

`kind` is one of nine values. Each `value` must match its kind:

| kind | `value` |
| --- | --- |
| `number` | a JSON number |
| `text` | a string up to 4096 characters |
| `status` | `{"state": "neutral" \| "positive" \| "warning" \| "critical", "text": "..."}` |
| `progress` | `{"current": n >= 0, "total": n > 0}` |
| `gauge` | `{"value": n, "min": n, "max": n}` with `min < max` |
| `list` | `{"items": [{"id", "label", "detail"?, "done"?, "progress"?, "state"?}]}`, up to 500 items |
| `timeseries` | `{"series": [{"label", "color"?, "points": [{"t", "v"}]}]}`, 1–8 series of up to 2000 points each |
| `bar` | `{"bars": [{"label", "value", "color"?}]}`, 1–100 bars |
| `table` | `{"columns": ["..."], "rows": [[scalar, ...]]}`, 1–16 columns and up to 500 rows |

Timeseries details: `t` is an RFC 3339 timestamp, and `v` is a number or `null` for a gap.

`format` is optional and describes display: `{"style", "unit"?, "precision"?}`.

## Keys

Keys match `^[a-z][a-z0-9_.-]{0,63}$`, for example `facility.temperature`.

## Limits

- Put: 1–100 metrics and at most 1 MiB per batch.
- Product: at most 256 metrics.
- Delete: 1–256 keys.

## Example

Go:

```go
metric := haulout.NewMetric("facility.temperature", "number", 21.4)
request := haulout.NewPutMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), []haulout.Metric{*metric})
rows, _, err := client.DefaultAPI.PutMetricsByKey(ctx).PutMetricsByKeyRequest(*request).Execute()
```

Python:

```python
from haulout import Metric, PutMetricsByKeyRequest

request = PutMetricsByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_metrics=[Metric(key="facility.temperature", kind="number", value=21.4)],
)
rows = api.put_metrics_by_key(put_metrics_by_key_request=request)
```

Delete, Python:

```python
from haulout import DeleteMetricsByKeyRequest

api.delete_metrics_by_key(
    delete_metrics_by_key_request=DeleteMetricsByKeyRequest(
        p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
        p_keys=["facility.temperature"],
    )
)
```
