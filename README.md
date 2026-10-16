# Haulout Provider SDKs

Clients for product integrations. A product publishes named metrics using its product key:

- Put metrics: write 1–100 named metrics per call, each with one of nine kinds (last-write-wins).
- Delete metrics: remove 1–256 keys per call.

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

The packages are not published to a registry yet.

## Documentation

- [Getting started](docs/getting-started.md): concepts, first write, and a complete provider loop in Go and Python
- [Metrics](docs/metrics.md): metric object, kinds, format, limits, and examples
- [Authentication](docs/authentication.md): product key and service key
- [Errors](docs/errors.md): error shape, codes, and retry rules
- [Changes](docs/changes/README.md): contract history; start with the current metrics contract
