# Errors

## Shape

Failures return a JSON body with `code`, `message`, and optional `details` and `hint`, plus the HTTP status. Gateway failures (wrong `apikey`) return a body with only `message` and `hint`, and status `401`.

- Go: non-2xx responses return `*haulout.GenericOpenAPIError` (a pointer). Use `errors.As` with a `*haulout.GenericOpenAPIError` target, then read `Body()` (JSON text). The HTTP status is the `*http.Response` returned by `Execute()`.
- Python: `ApiException` and its subclasses expose `.status` and `.body` (JSON text).

| HTTP status | Go | Python |
| --- | --- | --- |
| 400 | `resp.StatusCode == 400` | `BadRequestException` |
| 401 | `resp.StatusCode == 401` | `UnauthorizedException` |
| 403 | `resp.StatusCode == 403` | `ForbiddenException` |
| 404 | `resp.StatusCode == 404` | `NotFoundException` |
| 409 | `resp.StatusCode == 409` | `ConflictException` |
| 422 | `resp.StatusCode == 422` | `UnprocessableEntityException` |
| 5xx | `resp.StatusCode >= 500` | `ServiceException` |

All of these subclass `ApiException` (`haulout.exceptions`).

```python
from haulout.exceptions import ApiException, ForbiddenException, BadRequestException

try:
    api.put_metrics_by_key(put_metrics_by_key_request=request)
except ForbiddenException:
    print("invalid product key")
except BadRequestException as exc:
    print("invalid input:", exc.body)
except ApiException as exc:
    print("failed:", exc.status, exc.body)
```

## Codes

Database errors come back as a `code` in the body. Only these are returned by the metric operations.

| Code | HTTP | Operations | `message` |
| --- | --- | --- | --- |
| `22023` | 400 | put | `invalid metric`, `metric does not match the metric schema`, `gauge min must be less than max`, `duplicate list item id`, `table row length must match columns`, `invalid timeseries timestamp`, `metric batch must contain 1 to 100 metrics within 1 MiB`, `duplicate metric key`, `product metric limit exceeded` |
| `22023` | 400 | delete | `metric keys must contain 1 to 256 valid keys` |
| `28000` | 403 | put, delete | `invalid product key` |

Notes:

- `product metric limit exceeded` means the product would hold more than 256 metrics. Nothing in that batch is written. Delete unused keys, then retry.
- The baseline has no rate-limit error. Haulout does not return a rate-limit code for these operations. Treat any `429` you see from a proxy or network path as transient.

## Retry rules

- `400` / `22023`: permanent for the same input. Fix the metric or batch.
- `403` / `28000`: permanent. Check `HAULOUT_PRODUCT_KEY`. Rotation revokes the old key immediately.
- `401` (gateway, invalid `apikey`): permanent. Check the public service key in the SDK configuration.
- `5xx`, timeouts, and network errors: safe to retry. Puts are last-write-wins, identical rewrites are no-ops, and deletes ignore unknown keys.
