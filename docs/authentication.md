# Authentication

## One credential: the product key

Put your product key in the `p_product_key` field of every request. The client already carries the hosted endpoint and its routing credential as defaults; never configure URLs or extra keys.

Go (key is the first constructor argument):

```go
request := haulout.NewWriteStateByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), expectedRevision, values,
)
```

Python (key is the `p_product_key` keyword):

```python
from haulout import WriteStateByKeyRequest

request = WriteStateByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_expected_revision=expected_revision,
    p_values=values,
)
```

Snippets need a live product key; without one they do not run.

## Handling

- Load the key from the environment (`HAULOUT_PRODUCT_KEY`); never commit it to source control.
- Never place it in an `Authorization` header or log request bodies; bodies contain the key by design.
- Do not enable HTTP debug logging that records request bodies.
- Key rotation immediately invalidates the previous key. After rotation, all calls must use the new key; there is no grace period documented, so treat rotation as instant.

## Failure mode

An invalid product key fails with error code `28000`. See `errors.md`. No key-discovery or key-creation operation exists in these SDKs; that flow is out of scope and omitted.
