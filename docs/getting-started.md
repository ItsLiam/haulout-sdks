# Getting started

## What you need

- Go package from `go get github.com/ItsLiam/haulout-sdks/go`, or Python package from `pip install haulout`.
- One product key, available at runtime as `HAULOUT_PRODUCT_KEY`.

The clients are preconfigured. Do not set a server URL or add another credential.

Go:

```go
import haulout "github.com/ItsLiam/haulout-sdks/go"

client := haulout.NewAPIClient(haulout.NewConfiguration())
```

Python:

```python
from haulout import DefaultApi

api = DefaultApi()
```

## First write

Go:

```go
metric := haulout.NewMetric("facility.status", "status", map[string]interface{}{
	"state": "positive", "text": "Online",
})
request := haulout.NewPutMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), []haulout.Metric{*metric})
rows, _, err := client.DefaultAPI.PutMetricsByKey(ctx).PutMetricsByKeyRequest(*request).Execute()
```

Python:

```python
from haulout import Metric, PutMetricsByKeyRequest

request = PutMetricsByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_metrics=[Metric(key="facility.status", kind="status",
                      value={"state": "positive", "text": "Online"})],
)
rows = api.put_metrics_by_key(put_metrics_by_key_request=request)
```

Success returns one row per metric: `key`, `revision`, and `updated_at`. See [`metrics.md`](metrics.md) for every kind and its value shape.

These snippets need a live product key; without one they do not run.

## The two methods

- `PutMetricsByKey` / `put_metrics_by_key`: write 1–100 metrics per call.
- `DeleteMetricsByKey` / `delete_metrics_by_key`: remove 1–256 keys per call.
