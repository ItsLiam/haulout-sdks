# Haulout Provider SDKs

Clients for product integrations. Two operations, keyed by your product key:

- Put metrics: write 1–100 named metrics per call, each with one of nine kinds (last-write-wins).
- Delete metrics: remove 1–256 keys per call.

The clients arrive preconfigured (hosted API endpoint and public service key are baked in as defaults). You supply only your product key, ideally via an environment variable. Do not configure URLs or additional keys.

See [`docs/metrics.md`](docs/metrics.md) for kinds and limits, and [`docs/changes/2026-10-metrics.md`](docs/changes/2026-10-metrics.md) for the breaking change from the previous contract.

## Install

Go:

```sh
go get github.com/ItsLiam/haulout-sdks/go
```

Python:

```sh
pip install "git+https://github.com/ItsLiam/haulout-sdks.git#subdirectory=python"
uv pip install "haulout @ git+https://github.com/ItsLiam/haulout-sdks#subdirectory=python"
```

## Quickstart

Go:

```go
client := haulout.NewAPIClient(haulout.NewConfiguration())
metric := haulout.NewMetric("facility.temperature", "number", 21.4)
request := haulout.NewPutMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), []haulout.Metric{*metric})
rows, _, err := client.DefaultAPI.PutMetricsByKey(ctx).PutMetricsByKeyRequest(*request).Execute()
```

Python:

```python
from haulout import DefaultApi, Metric, PutMetricsByKeyRequest

api = DefaultApi()
rows = api.put_metrics_by_key(
    put_metrics_by_key_request=PutMetricsByKeyRequest(
        p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
        p_metrics=[Metric(key="facility.temperature", kind="number", value=21.4)],
    )
)
```

## Documentation

- [Getting started](docs/getting-started.md)
- [Metrics](docs/metrics.md)
- [Authentication](docs/authentication.md)
- [Errors](docs/errors.md)
- [Changes](docs/changes/)
