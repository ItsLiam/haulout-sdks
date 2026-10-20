# Getting started

## Concepts

- **Product**: created by a signed-in user in Haulout. Each product has one integration key. Haulout shows the key once, when it is created or rotated, and stores only a hash.
- **Metric**: one current value, keyed by `key` within the product. A put replaces the value. Haulout keeps no history; send a `timeseries` metric with its points each time.
- **Dashboards**: users compose dashboards from a product's metrics in Haulout apps. Producers only publish values; they do not define layouts.
- **Rotation**: rotating the key immediately revokes the old one. Update the producer's `HAULOUT_PRODUCT_KEY` at the same time, or its writes start failing with `28000`.

The flow:

1. Sign in to Haulout and create a product.
2. Put its product key in your producer's environment as `HAULOUT_PRODUCT_KEY`.
3. Publish metrics from the producer with the SDK, as below.
4. Compose dashboards from those metrics in Haulout.

## Install

- Go 1.23+: `go get github.com/ItsLiam/haulout-sdks/go`
- Python 3.10+: `pip install "git+https://github.com/ItsLiam/haulout-sdks.git#subdirectory=python"`
- Rust (edition 2021, needs an async runtime such as tokio): `cargo add haulout --git https://github.com/ItsLiam/haulout-sdks`

The packages are not published to a registry yet; install from Git as shown.

The clients are preconfigured with the hosted endpoint and Haulout's public service key. Do not set a server URL or add another credential. Supply only `HAULOUT_PRODUCT_KEY`.

## First write

Go:

```go
import (
	"context"
	"fmt"
	"os"

	haulout "github.com/ItsLiam/haulout-sdks/go"
)

func main() {
	ctx := context.Background()
	client := haulout.NewAPIClient(haulout.NewConfiguration())

	label := "Online"
	metric := haulout.NewMetric("facility.status", "status", map[string]interface{}{
		"state": "positive", "text": "Online",
	})
	metric.Label = &label
	request := haulout.NewPutMetricsByKeyRequest(os.Getenv("HAULOUT_PRODUCT_KEY"), []haulout.Metric{*metric})
	rows, _, err := client.DefaultAPI.PutMetricsByKey(ctx).PutMetricsByKeyRequest(*request).Execute()
	if err != nil {
		panic(err)
	}
	fmt.Println(rows[0].Key, rows[0].Revision)
}
```

Python:

```python
import os

from haulout import DefaultApi, Metric, PutMetricsByKeyRequest

api = DefaultApi()
rows = api.put_metrics_by_key(
    put_metrics_by_key_request=PutMetricsByKeyRequest(
        p_product_key=os.environ["HAULOUT_PRODUCT_KEY"],
        p_metrics=[Metric(key="facility.status", kind="status", label="Online",
                          value={"state": "positive", "text": "Online"})],
    )
)
print(rows[0].key, rows[0].revision)
```

Rust (async; `serde_json` and `tokio` are your dependencies):

```rust
use haulout::apis::{configuration::Configuration, default_api};
use haulout::models::{self, metric::Kind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new();
    let mut metric = models::Metric::new(
        "facility.status".into(),
        Kind::Status,
        Some(serde_json::json!({"state": "positive", "text": "Online"})),
    );
    metric.label = Some("Online".into());
    let request = models::PutMetricsByKeyRequest::new(
        std::env::var("HAULOUT_PRODUCT_KEY")?,
        vec![metric],
    );
    let rows = default_api::put_metrics_by_key(&config, request).await?;
    println!("{} {}", rows[0].key, rows[0].revision);
    Ok(())
}
```

Success returns one row per metric: `key`, `revision`, and `updated_at`. See [`metrics.md`](metrics.md) for every kind and its value shape.

## Provider loop

A complete producer: publish every 30 seconds, stop on a permanent error, and delete its metric on shutdown. Replace `readTemperature` / `read_temperature` with your sensor.

Go:

```go
package main

import (
	"context"
	"errors"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	haulout "github.com/ItsLiam/haulout-sdks/go"
)

const key = "facility.temperature"

func main() {
	productKey := os.Getenv("HAULOUT_PRODUCT_KEY")
	if productKey == "" {
		log.Fatal("HAULOUT_PRODUCT_KEY is not set")
	}
	client := haulout.NewAPIClient(haulout.NewConfiguration())

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()
	for running := true; running; {
		resp, err := publish(ctx, client, productKey)
		if err != nil {
			if resp != nil && (resp.StatusCode == http.StatusBadRequest || resp.StatusCode == http.StatusForbidden) {
				log.Fatalf("permanent error, fix configuration: %s", describe(err))
			}
			log.Printf("put failed, retrying next tick: %s", describe(err))
		}
		select {
		case <-ctx.Done():
			running = false
		case <-ticker.C:
		}
	}

	shutdown, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if err := remove(shutdown, client, productKey); err != nil {
		log.Printf("delete failed: %s", describe(err))
	}
}

func publish(ctx context.Context, client *haulout.APIClient, productKey string) (*http.Response, error) {
	label := "Temperature"
	metric := haulout.NewMetric(key, "number", readTemperature())
	metric.Label = &label
	request := haulout.NewPutMetricsByKeyRequest(productKey, []haulout.Metric{*metric})
	_, resp, err := client.DefaultAPI.PutMetricsByKey(ctx).PutMetricsByKeyRequest(*request).Execute()
	return resp, err
}

func remove(ctx context.Context, client *haulout.APIClient, productKey string) error {
	request := haulout.NewDeleteMetricsByKeyRequest(productKey, []string{key})
	_, _, err := client.DefaultAPI.DeleteMetricsByKey(ctx).DeleteMetricsByKeyRequest(*request).Execute()
	return err
}

func describe(err error) string {
	var apiErr *haulout.GenericOpenAPIError
	if errors.As(err, &apiErr) {
		return string(apiErr.Body())
	}
	return err.Error()
}

func readTemperature() float64 {
	return 21.4 // replace with your sensor read
}
```

Python:

```python
import os
import signal
import sys
import time

from haulout import DefaultApi, DeleteMetricsByKeyRequest, Metric, PutMetricsByKeyRequest
from haulout.exceptions import ApiException

KEY = "facility.temperature"
PRODUCT_KEY = os.environ.get("HAULOUT_PRODUCT_KEY")
if not PRODUCT_KEY:
    sys.exit("HAULOUT_PRODUCT_KEY is not set")

api = DefaultApi()
running = True


def stop(signum, frame):
    global running
    running = False


def read_temperature():
    return 21.4  # replace with your sensor read


def publish():
    api.put_metrics_by_key(
        put_metrics_by_key_request=PutMetricsByKeyRequest(
            p_product_key=PRODUCT_KEY,
            p_metrics=[Metric(key=KEY, kind="number", label="Temperature",
                              format={"style": "temperature", "unit": "°C", "precision": 1},
                              value=read_temperature())],
        )
    )


signal.signal(signal.SIGINT, stop)
signal.signal(signal.SIGTERM, stop)

while running:
    try:
        publish()
    except ApiException as exc:
        if exc.status in (400, 403):
            sys.exit(f"permanent error, fix configuration: {exc.body}")
        print("put failed, retrying next tick:", exc.status, exc.body, file=sys.stderr)
    for _ in range(30):
        if not running:
            break
        time.sleep(1)

try:
    api.delete_metrics_by_key(
        delete_metrics_by_key_request=DeleteMetricsByKeyRequest(
            p_product_key=PRODUCT_KEY, p_keys=[KEY]))
except ApiException as exc:
    print("delete failed:", exc.status, exc.body, file=sys.stderr)
```

Rust (`tokio` with the `full` feature, plus `serde_json`):

```rust
use std::time::Duration;

use haulout::apis::{configuration::Configuration, default_api, Error};
use haulout::models::{self, metric::Kind, metric_format::Style};

const KEY: &str = "facility.temperature";

fn read_temperature() -> f64 {
    21.4 // replace with your sensor read
}

#[tokio::main]
async fn main() {
    let product_key = std::env::var("HAULOUT_PRODUCT_KEY").expect("HAULOUT_PRODUCT_KEY is not set");
    let config = Configuration::new();
    let mut ticker = tokio::time::interval(Duration::from_secs(30));
    let shutdown = tokio::signal::ctrl_c();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            _ = &mut shutdown => break,
            _ = ticker.tick() => {
                let mut metric = models::Metric::new(
                    KEY.into(), Kind::Number, Some(serde_json::json!(read_temperature())),
                );
                metric.label = Some("Temperature".into());
                let mut format = models::MetricFormat::new(Style::Temperature);
                format.unit = Some("°C".into());
                format.precision = Some(1);
                metric.format = Some(Box::new(format));
                let request = models::PutMetricsByKeyRequest::new(product_key.clone(), vec![metric]);
                match default_api::put_metrics_by_key(&config, request).await {
                    Ok(_) => {}
                    Err(Error::ResponseError(r)) if matches!(r.status.as_u16(), 400 | 403) => {
                        eprintln!("permanent error, fix configuration: {}", r.content);
                        std::process::exit(1);
                    }
                    Err(e) => eprintln!("put failed, retrying next tick: {e}"),
                }
            }
        }
    }

    let request = models::DeleteMetricsByKeyRequest::new(product_key, vec![KEY.into()]);
    if let Err(e) = default_api::delete_metrics_by_key(&config, request).await {
        eprintln!("delete failed: {e}");
    }
}
```

Status codes: `400` (invalid input) and `403` (invalid product key) are permanent; fix the input or key rather than retrying. Other failures are safe to retry. See [`errors.md`](errors.md).

## Raw HTTP

The SDKs call PostgREST RPCs directly. Any HTTP client can do the same:

```http
POST https://qszqptdtmkppelqligvb.supabase.co/rest/v1/rpc/put_metrics_by_key
apikey: <Haulout public service key>
Content-Type: application/json

{"p_product_key": "<product key>", "p_metrics": [{"key": "facility.temperature", "kind": "number", "value": 21.4}]}
```

- `apikey` is Haulout's public service key, not the product key. Its value is `x-haulout-service-key` in [`../openapi.yaml`](../openapi.yaml), and it is also the default in the SDK configuration.
- The product key goes only in the body, as `p_product_key`.
- Delete is `POST /rpc/delete_metrics_by_key` with `p_product_key` and `p_keys`.

## The methods

- `PutMetricsByKey` / `put_metrics_by_key`: write 1–100 metrics per call.
- `DeleteMetricsByKey` / `delete_metrics_by_key`: remove 1–256 keys per call.
- `PutNotificationCategoriesByKey` / `put_notification_categories_by_key`: replace the product's notification categories (0–64); omitted categories and their device subscriptions are deleted.
- `SendNotificationByKey` / `send_notification_by_key`: send a title and body to one category; `recipients` is the number of subscribed devices targeted.
