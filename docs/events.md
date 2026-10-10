# Events

Publish 1–100 events per call against the active definition revision. The batch receipt is valid for 24 hours.

## Request shape

- Required: `p_product_key`, `p_idempotency_key` (UUID), `p_definition_revision` (integer >= 0, must equal the active definition revision), `p_events` (1–100 items).
- Each event: `id` (string), `source` (string), `time` (timestamp string), `fields` (object validated against the selected stream declaration; values preserve arbitrary JSON).
- `source` must exactly equal a declared stream name (case-sensitive key lookup, no prefix or pattern matching); anything else rejects the whole batch (`22023 'undeclared event source'`). Stream names satisfy `[a-z][a-z0-9_.-]{0,63}` — `/` is not allowed, use `.` (e.g. `"sensor.thermostat"`).
- Limits: the encoded batch and each event `fields` object are limited to 131072 bytes.

Go:

```go
event := *haulout.NewPublishEventBatchByKeyRequestPEventsInner(
	"event-1", "sensor", "2026-10-03T12:00:00Z",
	map[string]interface{}{
		"reading": 21.5,
		"details": map[string]interface{}{"unit": "celsius", "raw": nil},
	},
)
request := haulout.NewPublishEventBatchByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), "batch-uuid", definitionRevision,
	[]haulout.PublishEventBatchByKeyRequestPEventsInner{event},
)
receipt, _, err := client.DefaultAPI.PublishEventBatchByKey(ctx).
	PublishEventBatchByKeyRequest(*request).
	Execute()
```

Python:

```python
from uuid import UUID
from haulout import PublishEventBatchByKeyRequest
from haulout.models import PublishEventBatchByKeyRequestPEventsInner

request = PublishEventBatchByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_idempotency_key=UUID("44444444-4444-4444-8444-444444444444"),
    p_definition_revision=definition_revision,
    p_events=[
        PublishEventBatchByKeyRequestPEventsInner.from_dict(
            {
                "id": "evt-1",
                "source": "sensor.thermostat",
                "time": "2025-04-05T00:37:08.123456Z",
                "fields": {"temperature": 21.5},
            }
        )
    ],
)
receipt = api.publish_event_batch_by_key(request)
```

Note the UUID difference (verified in tests): Go constructors take UUID-formatted strings; Python models take `UUID` objects.

Snippets need a live product key; without one they do not run.

## Receipt

- `accepted`: count of newly accepted events.
- `duplicates`: count of events already seen and counted as duplicates.
- `cursor`: opaque position encoded as a decimal string; keep it exact (values can exceed 64-bit range).
- `committedAt` / `dedupeUntil`: timestamps.

Go accessors: `receipt.GetAccepted()`, `receipt.GetDuplicates()`, `receipt.GetCursor()`, `receipt.GetCommittedAt()`, `receipt.GetDedupeUntil()`.
Python attributes: `receipt.accepted`, `receipt.duplicates`, `receipt.cursor`, `receipt.committed_at`, `receipt.dedupe_until`.

## Idempotency and dedup rules

- Identical retries with the same product-scoped idempotency UUID return the saved receipt for 24 hours. Preserve event IDs, timestamps, definition revision, and fields exactly.
- Reusing an event ID with identical `source`, revision, timestamp, and `fields` counts as a duplicate; changed content conflicts (code `23505`). A changed payload under an existing idempotency UUID also conflicts (`23505`).
- A `p_definition_revision` that does not match the active revision conflicts (code `PT409`).
- On ambiguous network outcome, retry with the same idempotency UUID and byte-identical events; never mint a fresh UUID for the same logical batch.

Event-query/history reads are omitted: no such operation exists in the six-RPC contract. The sixth RPC is provider-driven `purge_events_by_key(p_product_key, p_idempotency_key, p_definition_revision, p_sources, p_before)`, where `p_sources` is a list of 1–128 declared stream names and `p_before` is a nullable timestamp bound. Warning: purging a source S deletes all of the product's events with source S (bounded by `p_before` when given); there is no per-event or whole-product form.
