# Errors

## Shape

Failures surface as an API error body with `code`, `message`, and optional `details`/`hint`. The HTTP status accompanies it.

Go: non-2xx responses return `*haulout.GenericOpenAPIError`; inspect `Body()` and `Error()`.
Python: HTTP 400 raises `BadRequestException` and HTTP 403 raises `ForbiddenException`, both subclasses of `ApiException`, with `.status` and `.body` (the JSON body text).

```python
from haulout.exceptions import BadRequestException, ForbiddenException

try:
    api.put_metrics_by_key(put_metrics_by_key_request=request)
except ForbiddenException as exc:
    print("invalid product key", exc.status)
except BadRequestException as exc:
    print("invalid input", exc.status)
```

## Codes

Only two SQLSTATEs are returned by these operations.

| Code | Operations | Meaning |
| --- | --- | --- |
| `22023` | put, delete | Invalid input: metric shape or kind, batch or product limit exceeded, duplicate key in a batch, invalid key, or no keys given. |
| `28000` | put, delete | Invalid product key. |

## Retry rules

- `22023`: do not retry with the same input. Fix the metric or batch first.
- `28000`: do not retry. Check `HAULOUT_PRODUCT_KEY`.
- Other failures (network, timeouts, 5xx): retrying is safe. Puts are last-write-wins, identical rewrites are no-ops, and deletes ignore unknown keys.
