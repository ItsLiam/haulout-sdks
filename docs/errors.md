# Errors

## Shape

Failures surface as an API error body with `code`, `message`, and optional `details`/`hint`. HTTP status accompanies it (e.g. 409 on conflicts). Other backend failures remain possible; there is no separate custom SDK error envelope.

Go: non-2xx responses return `*haulout.GenericOpenAPIError` (inspect `Body()` / `Error()`; the 409 conflict path carries the code above). Python: HTTP 409 raises `ConflictException` (a subclass of `ApiException`) with `.status` and `.data` exposing `.code`, `.message`, `.details`, `.hint`.

Python example (verified shape):

```python
from haulout.exceptions import ConflictException

try:
    api.complete_command_by_key(request)
except ConflictException as exc:
    print(exc.status, exc.data.code, exc.data.message)
```

This snippet needs a live product key to reach the failure path.

## Per-operation codes

| Operation | Code | Meaning |
| --- | --- | --- |
| All | `28000` | Invalid product key. |
| Definition write | `22023` | Invalid definition document. |
| Definition write | `PT409` | Definition revision conflict (stale `p_expected_revision`). |
| State write | `22023` | Missing/invalid revision or invalid/undeclared state values. |
| State write | `PT409` | State revision conflict; refresh the revision before retrying. |
| Event publish | `22023` | Invalid batch, event, source, or event fields. |
| Event publish | `PT409` | Definition revision conflict. |
| Event publish | `23505` | Idempotency-key conflict or event ID reused with different content. |
| Claim | `22023` | Invalid worker/claim arguments. |
| Claim | `23505` | Claim ID reused for a different worker. |
| Complete | `22023` | Invalid status/result. |
| Complete | `42501` | Inactive product. |
| Complete | `23505` | Result conflict (same lease, different payload). |
| Complete | `P0002` | Command lease not found. |

## Retry rules

- `PT409`: refresh the authoritative revision (definition/state) or definition revision (events) first; do not blindly resend.
- `23505`: do not retry with changed content; resend the identical payload under the identical key/IDs, or mint a new idempotency UUID only for genuinely new work.
- Ambiguous network outcome: retry writes with the original CAS revision, event batches with the same idempotency UUID and identical events, claims with the same worker + claim UUID, and completions with the same lease + status + result.
- HTTP 200 with JSON `null` from claim is not an error; back off and poll again.
- A `complete` returning `unknown` means the lease expired; do not treat it as success of your work.
