# Definitions and state

Both writes replace their whole document and use revision compare-and-swap (CAS). Every listed `p_*` field is required; none has a server-side default.

## Definition write

Contract (verified in `openapi.yaml`):

- Required: `p_product_key`, `p_expected_revision` (integer >= 0), `p_definition`.
- `p_definition` envelope: `type: "ProductDefinition"`, `version: "2.0"`, plus `streams`, `state`, `commands`, `dashboards`.
- Limit: definition payload limited to 1 MiB.
- Success: replaces the whole definition only when the stored revision equals `p_expected_revision`; revision increments by one and the single returned row carries `revision` and `hash` (`sha256:<hex>`).
- Stale revision: conflict with code `PT409`.

Go:

```go
definition := *haulout.NewProductDefinition(
	"ProductDefinition", "2.0",
	map[string]haulout.Stream{},
	map[string]haulout.Field{},
	map[string]haulout.Command{},
	[]map[string]interface{}{},
)
request := haulout.NewWriteDefinitionByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), expectedRevision, definition,
)
rows, _, err := client.DefaultAPI.WriteDefinitionByKey(ctx).
	WriteDefinitionByKeyRequest(*request).
	Execute()
```

Python:

```python
from haulout import ProductDefinition, WriteDefinitionByKeyRequest

definition = ProductDefinition(
    type="ProductDefinition",
    version="2.0",
    streams={},
    state={},
    commands={},
    dashboards=[],
)
request = WriteDefinitionByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_expected_revision=expected_revision,
    p_definition=definition,
)
rows = api.write_definition_by_key(request)
```

## State write

Contract:

- Required: `p_product_key`, `p_expected_revision` (integer >= 0), `p_values` (object; properties and values are validated against declared state fields; nested objects where the declaration uses dotted paths).
- Limit: payload limited to 1 MiB.
- Empty initial state has revision 0; its first successful write produces revision 1.
- Success returns one row: `revision` plus `values`.
- Retry-safety quirk (verified): an identical existing value is returned without changing its revision, even if the supplied expected revision is stale. Any other write uses CAS; a stale revision conflicts with code `PT409`.

Go (nested values, explicit null, and numbers survive the round trip):

```go
values := map[string]interface{}{
	"profile": map[string]interface{}{
		"score":    12.5,
		"enabled":  true,
		"labels":   []interface{}{"alpha", "beta"},
		"nickname": nil,
	},
}
request := haulout.NewWriteStateByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), expectedRevision, values,
)
rows, _, err := client.DefaultAPI.WriteStateByKey(ctx).
	WriteStateByKeyRequest(*request).
	Execute()
```

Python:

```python
from haulout import WriteStateByKeyRequest

request = WriteStateByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_expected_revision=expected_revision,
    p_values={"profile": {"display": "Ada", "optional": None}},
)
rows = api.write_state_by_key(request)
print(rows[0].revision, rows[0].values)
```

Snippets need a live product key; without one they do not run.

## Conflict handling

1. On `PT409`, refresh the authoritative revision before deciding whether to retry.
2. Do not retry a conflict with changed input as though it were transient.
3. On an ambiguous network outcome, retry with the original CAS revision and identical document; for state, an identical-values retry is safe even with a stale revision per the quirk above.

Stream/field/command declaration details beyond the envelope are omitted: the hosted API validates them authoritatively, and no declaration reference was verified beyond the constructor shapes (`NewField(type)`, `NewStream(kind, fields)`, `NewCommand(title, danger, arguments)` in Go).
