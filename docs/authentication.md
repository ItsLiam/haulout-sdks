# Authentication

## Two keys, two places

| Key | Where it goes | Who owns it |
| --- | --- | --- |
| Product key | `p_product_key` in the request body | Your product. Shown once by Haulout; stored only as a hash. |
| Service key | `apikey` header | Haulout's public key, built into the SDK defaults (`x-haulout-service-key` in [`../openapi.yaml`](../openapi.yaml)). |

The client already carries the hosted endpoint and the service key. Never configure URLs or extra keys. Put only the product key in your code, from the environment.

Go (key is the first constructor argument):

```go
request := haulout.NewPutMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), metrics)
```

Python (key is the `p_product_key` keyword; `import os` first):

```python
from haulout import PutMetricsByKeyRequest

request = PutMetricsByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_metrics=metrics,
)
```

## Handling

- Load the product key from the environment (`HAULOUT_PRODUCT_KEY`); never commit it to source control.
- Never place it in an `Authorization` header or log request bodies; bodies contain the key by design.
- Do not enable HTTP debug logging that records request bodies.
- Rotation immediately revokes the previous key. Update every producer at the time you rotate; until then their writes fail with `403` / `28000`.

## Failure modes

- Wrong product key: `403`, code `28000`, message `invalid product key`.
- Wrong service key in `apikey`: `401` from the API gateway, body `{"message": "Invalid API key", ...}` with no `code`.

See [`errors.md`](errors.md).
