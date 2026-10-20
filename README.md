# Haulout Provider SDKs

Clients for product integrations. A product publishes named metrics using its product key:

- Put metrics: write 1–100 named metrics per call, each with one of nine kinds (last-write-wins).
- Delete metrics: remove 1–256 keys per call.
- Notifications: replace the product's notification categories, and send a push to the devices subscribed to a category.

The clients arrive preconfigured with the hosted endpoint and public service key. You supply only your product key, ideally via the `HAULOUT_PRODUCT_KEY` environment variable.

## Install

Go 1.23+:

```sh
go get github.com/ItsLiam/haulout-sdks/go
```

Python 3.10+:

```sh
pip install "git+https://github.com/ItsLiam/haulout-sdks.git#subdirectory=python"
```

Rust (edition 2021, async with [tokio](https://tokio.rs)):

```sh
cargo add haulout --git https://github.com/ItsLiam/haulout-sdks
```

The packages are not published to a registry yet.

## Documentation

- [Getting started](docs/getting-started.md): concepts, first write, and a complete provider loop in Go, Python, and Rust
- [Metrics](docs/metrics.md): metric object, kinds, format, limits, and examples
- [Authentication](docs/authentication.md): product key and service key
- [Notifications](docs/notifications.md): categories, sending, limits, and examples
- [Errors](docs/errors.md): error shape, codes, and retry rules
- [Changes](docs/changes/README.md): contract history; start with the current metrics contract
