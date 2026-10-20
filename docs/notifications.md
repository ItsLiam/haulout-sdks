# Notifications

A product can push notifications to the iOS devices of its owner. The SDK exposes four RPCs; the two notification operations (the other two are in [`metrics.md`](metrics.md)) both authenticate with `p_product_key`:

- `put_notification_categories_by_key` (`PutNotificationCategoriesByKey` in Go) publishes the product's notification categories.
- `send_notification_by_key` (`SendNotificationByKey` in Go) sends a title and body to one category.

Users subscribe their devices to categories in the Haulout iOS app. Only the product owner can subscribe.

## Semantics

- Publishing replaces the whole category set (0–64 categories). Categories omitted from the call are deleted along with their device subscriptions. An empty list clears every category.
- A category has a `key` matching `^[a-z][a-z0-9_.-]{0,63}$`, a `label` of 1–100 characters, and an optional `description` of 1–280 characters. Keys must be unique within a call.
- A send takes `category`, a `title` of 1–100 characters, and a `body` of 0–500 characters. An empty body is valid, but the argument is required.
- Nothing is stored. The payload exists only for dispatch; there is no history and no web push.
- A send returns exactly one row, `recipients`: the number of subscribed devices whose sessions are live. `0` means nobody is subscribed. Delivery itself is not confirmed.

## Example: publish categories

Go (inside a function with `ctx context.Context`):

```go
client := haulout.NewAPIClient(haulout.NewConfiguration())

category := haulout.NewNotificationCategory("deploys", "Deploys")
description := "Deployment results"
category.Description = &description
request := haulout.NewPutNotificationCategoriesByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), []haulout.NotificationCategory{*category})
_, _, err := client.DefaultAPI.PutNotificationCategoriesByKey(ctx).
	PutNotificationCategoriesByKeyRequest(*request).Execute()
```

Python:

```python
import os

from haulout import DefaultApi, NotificationCategory, PutNotificationCategoriesByKeyRequest

api = DefaultApi()
api.put_notification_categories_by_key(
    put_notification_categories_by_key_request=PutNotificationCategoriesByKeyRequest(
        p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
        p_categories=[NotificationCategory(key="deploys", label="Deploys",
                                           description="Deployment results")],
    )
)
```

Rust (async):

```rust
use haulout::apis::{configuration::Configuration, default_api};
use haulout::models;

let config = Configuration::new();
let mut category = models::NotificationCategory::new("deploys".into(), "Deploys".into());
category.description = Some("Deployment results".into());
let request = models::PutNotificationCategoriesByKeyRequest::new(
    std::env::var("HAULOUT_PRODUCT_KEY")?,
    vec![category],
);
default_api::put_notification_categories_by_key(&config, request).await?;
```

## Example: send

Go:

```go
request := haulout.NewSendNotificationByKeyRequest(
	os.Getenv("HAULOUT_PRODUCT_KEY"), "deploys", "Deploy finished", "")
rows, _, err := client.DefaultAPI.SendNotificationByKey(ctx).
	SendNotificationByKeyRequest(*request).Execute()
if err != nil {
	return err
}
fmt.Println("recipients:", rows[0].Recipients)
```

Python:

```python
from haulout import SendNotificationByKeyRequest

rows = api.send_notification_by_key(
    send_notification_by_key_request=SendNotificationByKeyRequest(
        p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
        p_category="deploys",
        p_title="Deploy finished",
        p_body="",
    )
)
print("recipients:", rows[0].recipients)
```

Rust:

```rust
let request = models::SendNotificationByKeyRequest::new(
    std::env::var("HAULOUT_PRODUCT_KEY")?,
    "deploys".into(),
    "Deploy finished".into(),
    String::new(),
);
let rows = default_api::send_notification_by_key(&config, request).await?;
println!("recipients: {}", rows[0].recipients);
```

## Errors

| Code | HTTP | Operations | Meaning |
| --- | --- | --- | --- |
| `22023` | 400 | put categories | invalid, duplicate, or over-limit category batch |
| `22023` | 400 | send | invalid or unknown category, title, or body |
| `28000` | 403 | both | invalid product key |

See [`errors.md`](errors.md) for the error shape and retry rules. A send is not idempotent: retrying after a timeout can notify devices twice.
