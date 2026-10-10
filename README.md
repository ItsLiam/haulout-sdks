# Haulout Provider SDKs

Clients for product integrations. Six operations, keyed by your product key:

- Replace the product definition (compare-and-swap on revision).
- Replace product state values (compare-and-swap on revision).
- Publish idempotent event batches.
- Purge stored events by source (provider-driven retention).
- Claim queued commands with a short lease.
- Complete leased commands with a JSON result.

The clients arrive preconfigured (hosted API endpoint and routing credential are baked in as defaults). You supply only your product key, ideally via an environment variable. Do not configure URLs or additional keys.

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

All snippets need a live product key in `HAULOUT_PRODUCT_KEY`. They are symbol-exact but do not run without one.

Go (definition write):

```go
import (
	"context"
	"fmt"
	"os"

	haulout "github.com/ItsLiam/haulout-sdks/go"
)

ctx := context.Background()
client := haulout.NewAPIClient(haulout.NewConfiguration())

definition := *haulout.NewProductDefinition(
	"ProductDefinition", "2.0",
	map[string]haulout.Stream{},
	map[string]haulout.Field{},
	map[string]haulout.Command{},
	[]map[string]interface{}{},
)
request := haulout.NewWriteDefinitionByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), 0, definition,
)
rows, _, err := client.DefaultAPI.WriteDefinitionByKey(ctx).
	WriteDefinitionByKeyRequest(*request).
	Execute()
if err != nil {
	return err
}
fmt.Println(rows[0].GetRevision(), rows[0].GetHash())
```

Python (definition write):

```python
import os
from haulout import DefaultApi, ProductDefinition, WriteDefinitionByKeyRequest

api = DefaultApi()
request = WriteDefinitionByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_expected_revision=0,
    p_definition=ProductDefinition(
        type="ProductDefinition",
        version="2.0",
        streams={},
        state={},
        commands={},
        dashboards=[],
    ),
)
rows = api.write_definition_by_key(request)
print(rows[0].revision, rows[0].hash)
```

## Docs

Start here:

- `docs/getting-started.md` — setup and first write.
- `docs/authentication.md` — product-key handling.

Run the integration:

- `docs/definitions-state.md` — definition/state writes, revisions, conflicts.
- `docs/events.md` — event batches, idempotency, receipts.
- `docs/commands.md` — claim/complete loop, leases, results.
- `docs/errors.md` — error shape and per-operation codes.

Compose dashboards:

- `docs/components.md` — every metric kind with JSON shapes for building dashboard cells.
- `docs/layouts.md` — dashboard shell, grid/spans, and the compose loop for full pages.
