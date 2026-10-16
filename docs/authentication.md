# Authentication

## One credential: the product key

Put your product key in the `p_product_key` field of every request. The client already carries the hosted endpoint and its public service key as defaults; never configure URLs or extra keys. The service key is sent in the `apikey` header and is not the product key.

Go (key is the first constructor argument):

```go
request := haulout.NewPutMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), metrics)
```

Python (key is the `p_product_key` keyword):

```python
from haulout import PutMetricsByKeyRequest

request = PutMetricsByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_metrics=metrics,
)
```

## Handling

- Load the key from the environment (`HAULOUT_PRODUCT_KEY`); never commit it to source control.
- Never place it in an `Authorization` header or log request bodies; bodies contain the key by design.
- Do not enable HTTP debug logging that records request bodies.
- Key rotation immediately invalidates the previous key. Treat rotation as instant: all calls must use the new key.

## Failure mode

An invalid product key fails with error code `28000`. See [`errors.md`](errors.md).
