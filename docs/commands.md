# Commands

Poll `claim` for work, execute it, then report with `complete`. Claims carry a 45-second lease.

## Claim

- Required: `p_product_key`, `p_worker_id` (1–128 chars), `p_claim_id` (UUID).
- Success is either a claim object (`commandId`, `command`, `arguments`, `definitionRevision`, `leaseId`, `leaseExpiresAt`) or HTTP 200 with JSON `null`, which means no eligible work. `null` is normal, not an error.
- Lease behavior: claims hold a 45-second lease. Expired accepted commands become expired; expired running commands become `unknown` and are not silently rerun.
- Retry-safety: repeating the same worker ID + claim UUID returns the same claim (use this after an ambiguous failure). Reusing one claim UUID for a different worker conflicts (code `23505`).

Go (`claim == nil` means no work):

```go
claimReq := haulout.NewClaimCommandByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), "worker-1", "claim-uuid",
)
claim, _, err := client.DefaultAPI.ClaimCommandByKey(ctx).
	ClaimCommandByKeyRequest(*claimReq).
	Execute()
```

Python (`None` means no work):

```python
from uuid import UUID
from haulout import ClaimCommandByKeyRequest

request = ClaimCommandByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_worker_id="worker-a",
    p_claim_id=UUID("11111111-1111-4111-8111-111111111111"),
)
claim = api.claim_command_by_key(request)
```

## Complete (result reporting)

- Required: `p_product_key`, `p_command_id` (UUID), `p_lease_id` (UUID from the claim), `p_status` (one of `succeeded`, `failed`, `rejected`), `p_result` (arbitrary JSON including scalar, array, object, and explicit `null`; limited to 131072 bytes).
- Returns the completed status directly as a JSON string: `succeeded`, `failed`, `rejected`, or `unknown`.
- Late completion (at or after lease expiry) records/returns `unknown` with a null result.
- Retry-safety: repeating the same command + lease + status + result is idempotent; changed content conflicts (code `23505`). A missing/invalid lease reports code `P0002`.

Go (`p_result` accepts any JSON value, including `nil` for explicit null; status constants are `haulout.SUCCEEDED`, `haulout.FAILED`, `haulout.REJECTED`, `haulout.UNKNOWN`):

```go
request := haulout.NewCompleteCommandByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), "command-uuid", "lease-uuid", "succeeded",
	map[string]interface{}{"ok": true},
)
status, _, err := client.DefaultAPI.CompleteCommandByKey(ctx).
	CompleteCommandByKeyRequest(*request).
	Execute()
```

Python (`p_result` accepts any JSON value, including `None`; the call returns the status string):

```python
from uuid import UUID
from haulout import CompleteCommandByKeyRequest

request = CompleteCommandByKeyRequest(
    p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
    p_command_id=UUID("22222222-2222-4222-8222-222222222222"),
    p_lease_id=UUID("33333333-3333-4333-8333-333333333333"),
    p_status="succeeded",
    p_result={"ok": True},
)
status = api.complete_command_by_key(request)
```

Snippets need a live product key; without one they do not run.

Do not promise exactly-once side effects for the work you do between claim and complete; the idempotency documented here covers the report, not your side effects. Command-queue administration (creation/cancellation/listing) is omitted: no such operation exists in the contract.
