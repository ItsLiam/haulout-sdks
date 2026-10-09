# Getting started

## What you need

- Go package from `go get github.com/ItsLiam/haulout-sdks/go`, or Python package from `pip install haulout`.
- One product key, available at runtime as `HAULOUT_PRODUCT_KEY`.

The clients are preconfigured. Do not set a server URL or add another credential. Construction only:

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

## First write (definition, revision 0)

A new product starts at definition revision 0. Replace the whole definition document with `p_expected_revision` set to the current revision.

Go:

```go
request := haulout.NewWriteDefinitionByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), 0, definition,
)
rows, _, err := client.DefaultAPI.WriteDefinitionByKey(ctx).
	WriteDefinitionByKeyRequest(*request).
	Execute()
```

Python:

```python
from haulout import WriteDefinitionByKeyRequest

request = WriteDefinitionByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_expected_revision=0,
    p_definition=definition,
)
rows = api.write_definition_by_key(request)
```

Success returns a one-row array: `revision` (old revision + 1) and `hash` (`sha256:<lowercase hex>`). In Go read them with `rows[0].GetRevision()` and `rows[0].GetHash()`; in Python with `rows[0].revision` and `rows[0].hash`.

These snippets need a live product key; without one they do not run.

## The five methods

Go (all hang off `client.DefaultAPI`, take `ctx`, and end in `.Execute()`):

- `WriteDefinitionByKey(ctx).WriteDefinitionByKeyRequest(req).Execute()` returns `([]WriteDefinitionByKey200ResponseInner, *http.Response, error)`.
- `WriteStateByKey(ctx).WriteStateByKeyRequest(req).Execute()` returns `([]WriteStateByKey200ResponseInner, *http.Response, error)`.
- `PublishEventBatchByKey(ctx).PublishEventBatchByKeyRequest(req).Execute()` returns `(*PublishEventBatchByKey200Response, *http.Response, error)`.
- `ClaimCommandByKey(ctx).ClaimCommandByKeyRequest(req).Execute()` returns `(*CommandClaim, *http.Response, error)`; `nil` claim with a 200 response means no eligible work.
- `CompleteCommandByKey(ctx).CompleteCommandByKeyRequest(req).Execute()` returns `(*CompletionStatus, *http.Response, error)`.

Python (all hang off `DefaultApi()`):

- `write_definition_by_key(request)` returns a one-row list.
- `write_state_by_key(request)` returns a one-row list.
- `publish_event_batch_by_key(request)` returns a receipt object.
- `claim_command_by_key(request)` returns a claim object or `None` when no work is eligible.
- `complete_command_by_key(request)` returns a status string.

Next: `authentication.md`, then per-operation guides. Dashboard component reference is intentionally omitted here: dashboards travel as opaque JSON in the SDK data transfer objects and are validated by the hosted API; only the envelope (`type`/`version`/`streams`/`state`/`commands`/`dashboards`) is documented.
