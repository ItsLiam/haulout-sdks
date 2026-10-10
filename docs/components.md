# Components

Validated against definition v2.0.

## How to read this page

Valid means the hosted API accepts the definition write. These pages describe the accepted shapes; the API's validator decides. On conflict between docs and a rejected write, the rejection wins.

Every object here is a closed set: only the listed props are accepted, anything else is rejected. All ids, keys, and names use the pattern `^[a-z][a-z0-9_.-]{0,63}$` (lowercase start, up to 64 chars).

Every snippet is a schema-shaped JSON fragment, verified against the canonical definition schema before publishing. Snippets need a live product key; without one they do not run. For placement (grid, spans, groups), see `layouts.md`. For the write contract (CAS revisions, limits on the whole document), see `definitions-state.md`.

## Component index (machine-readable)

Parse this block, not the prose. Keys are stable; prose may change.

```json
[
  {"component": "Surface", "required": ["type", "id", "title", "metric"], "optional": ["span", "requires", "fallback", "isVisible"]},
  {"component": "SurfaceGroup", "required": ["type", "id", "title", "metrics"], "optional": ["span", "columns", "requires", "fallback", "isVisible"]},
  {"component": "Surface", "metric": "number", "required": ["kind", "value"], "optional": ["requires", "format"], "binds": ["literal", "state", "stream", "view", "input"]},
  {"component": "Surface", "metric": "text", "required": ["kind", "value"], "optional": ["requires", "format"], "binds": ["literal", "state", "stream", "view", "input"]},
  {"component": "Surface", "metric": "status", "required": ["kind", "value", "mapping"], "optional": ["requires"], "binds": ["literal", "state", "stream", "view", "input"]},
  {"component": "Surface", "metric": "progress", "required": ["kind", "value", "min", "max"], "optional": ["requires", "format"], "binds": ["literal", "state", "stream", "view", "input"]},
  {"component": "Surface", "metric": "gauge", "required": ["kind", "value", "min", "max"], "optional": ["requires", "format"], "binds": ["literal", "state", "stream", "view", "input"]},
  {"component": "Surface", "metric": "graph.timeseries", "required": ["kind", "series"], "optional": ["requires", "xAxis", "yAxis"], "binds": ["stream"]},
  {"component": "Surface", "metric": "graph.bar", "required": ["kind", "subscription", "categoryField", "valueField"], "optional": ["requires", "maxCategories", "format"], "binds": ["subscription"]},
  {"component": "Surface", "metric": "input.toggle", "required": ["kind", "value"], "optional": ["onChange", "requires"], "binds": ["view"]},
  {"component": "Surface", "metric": "input.choice", "required": ["kind", "value", "choices"], "optional": ["onChange", "requires"], "binds": ["view"]},
  {"component": "Surface", "metric": "input.text", "required": ["kind"], "optional": ["value", "onChange", "requires", "placeholder", "secure", "maxLength", "keyboard"], "binds": ["view"]},
  {"component": "Surface", "metric": "input.number", "required": ["kind"], "optional": ["value", "onChange", "requires", "min", "max", "step"], "binds": ["view"]},
  {"component": "Surface", "metric": "input.date", "required": ["kind"], "optional": ["value", "onChange", "requires"], "binds": ["view"]},
  {"component": "Surface", "metric": "input.time", "required": ["kind"], "optional": ["value", "onChange", "requires"], "binds": ["view"]}
]
```

## Surface (single metric cell)

### When to use

Use Surface for one metric: one value, one chart, one control. Do not use it for a cluster of related metrics; that is a SurfaceGroup.

### JSON shape

```json
{"type": "Surface", "id": "temperature", "title": "Temperature", "span": 1, "metric": {"kind": "number", "value": {"kind": "state", "path": "facility.temperature"}}}
```

### Props

| Prop | Required | Type | Notes |
|---|---|---|---|
| `type` | Yes | const `"Surface"` | Discriminator. |
| `id` | Yes | id string | Unique across the dashboard's components and group cells. |
| `title` | Yes | string ≤1024 | Display title and accessibility name. |
| `metric` | Yes | Metric | One of the 13 kinds below, or a forward-compatible unknown kind. |
| `span` | No | integer 1–24, default 1 | Width on the outer grid; clamped at render, never mutated. |
| `requires` | No | object ≤32 props | Capability gates; values are `major.minor` strings. |
| `fallback` | No | const `"drop"` | Only value allowed; unsupported Surface is dropped. |
| `isVisible` | No | boolean, default true | Hidden cells keep their ids reserved. |

### Binding

Surface itself binds nothing; its metric's `value` (a Ref, never a bare string) does. Every `state`/`stream`/`view`/`input` target must resolve to a declared field, subscription, view-state key, or input id, or the write is rejected.

### Worked snippet

```json
{"type": "Surface", "id": "facility_name", "title": "Facility", "metric": {"kind": "text", "value": {"kind": "state", "path": "facility.name"}}}
```

## SurfaceGroup (metric cell cluster)

### When to use

Use SurfaceGroup for 2 or more related metrics that share one heading (e.g. "Live facility": temperature, humidity, status). Do not use it for a single metric, and do not nest groups: cells must be Surfaces.

### JSON shape

```json
{"type": "SurfaceGroup", "id": "live_facility", "title": "Live facility", "span": 2, "columns": 4, "metrics": [{"type": "Surface", "id": "temperature", "title": "Temperature", "metric": {"kind": "number", "value": {"kind": "state", "path": "facility.temperature"}}}]}
```

### Props

| Prop | Required | Type | Notes |
|---|---|---|---|
| `type` | Yes | const `"SurfaceGroup"` | Discriminator. |
| `id` | Yes | id string | Unique across the dashboard's components and group cells. |
| `title` | Yes | string ≤1024 | Group heading. |
| `metrics` | Yes | Surface array, 1–100 items | Cells. Must be Surfaces; a nested group fails validation. |
| `span` | No | integer 1–12, default 1 | Width on the outer grid. Independent range from Surface span. |
| `columns` | No | integer 1–24 | Inner grid width. Omitted means `span * 2` at render time; never written back. |
| `requires` | No | object ≤32 props | Capability gates; values are `major.minor` strings. |
| `fallback` | No | const `"drop"` | Only value allowed. |
| `isVisible` | No | boolean, default true | Hides the whole group including cells. |

### Binding

Same rule as Surface: each cell metric's Refs must resolve. Cell ids share the dashboard-wide uniqueness scope with top-level component ids.

### Worked snippet

```json
{"type": "SurfaceGroup", "id": "alerts_group", "title": "Alerts", "span": 1, "columns": 3, "metrics": [{"type": "Surface", "id": "door_open", "title": "Door currently open", "metric": {"kind": "status", "value": {"kind": "literal", "value": "closed"}, "mapping": {"open": "warning", "closed": "positive"}}}]}
```

## Metric: number

Use for a single numeric readout (temperature, count, ratio). Do not use it for bar-style fill or thresholds; those are progress/gauge. Do not use it for percentage-point values with a bare unit guess: percentage points are `number` with `unit: "%"`; fractions (0.57 renders as 57%) are `percent` style.

```json
{"kind": "number", "value": {"kind": "state", "path": "facility.temperature"}, "format": {"style": "temperature", "unit": "°C", "precision": 1}}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"number"` | — |
| `value` | Yes | Ref | Must resolve to a numeric value; series bindings rejected. |
| `requires` | No | object ≤32 props | Capability gates. |
| `format` | No | Format | `style` required inside; `unit` ≤32 chars, `precision` 0–6. |

Binding: `value` accepts any Ref kind except a series selector. A `stream` Ref must use `selector: "latest"` and resolve through a declared subscription to a numeric field.

```json
{"type": "Surface", "id": "device_count", "title": "Devices", "metric": {"kind": "number", "value": {"kind": "state", "path": "facility.device_count"}}}
```

## Metric: text

Use for a single line of text (name, zone, status line). Do not use it for numbers that need formatting; that is `number`. `value` is a Ref, never a bare string.

```json
{"kind": "text", "value": {"kind": "literal", "value": "Ready"}}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"text"` | — |
| `value` | Yes | Ref | Never a bare string. Series bindings rejected. |
| `requires` | No | object ≤32 props | Capability gates. |
| `format` | No | Format | Rarely needed on text. |

Binding: any scalar Ref. `stream` Refs use `selector: "latest"`.

```json
{"type": "Surface", "id": "latest_zone", "title": "Latest zone", "metric": {"kind": "text", "value": {"kind": "stream", "subscription": "telemetry_latest", "selector": "latest", "field": "zone"}}}
```

## Metric: status

Use for a value mapped to a severity (healthy/attention/offline, open/closed). Do not use it for free text; that is `text`. Mapping keys are unconstrained strings; mapping values must be one of `neutral`, `positive`, `warning`, `critical`.

```json
{"kind": "status", "value": {"kind": "literal", "value": "healthy"}, "mapping": {"healthy": "positive", "attention": "warning", "offline": "critical"}}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"status"` | — |
| `value` | Yes | Ref | Series bindings rejected. |
| `mapping` | Yes | object ≤32 props | Keys free-form; values enum `neutral/positive/warning/critical`. |
| `requires` | No | object ≤32 props | Capability gates. |

Binding: any scalar Ref. Boolean-valued sources map with `"true"`/`"false"` keys.

```json
{"type": "Surface", "id": "door_open", "title": "Door currently open", "metric": {"kind": "status", "value": {"kind": "stream", "subscription": "telemetry_latest", "selector": "latest", "field": "door_open"}, "mapping": {"true": "warning", "false": "positive"}}}
```

## Metric: progress

Use for completion toward a fixed range (humidity 0–100, quota used). Do not use it for gauges with danger bands or live thresholds; that is `gauge`. Requires `min` strictly less than `max`.

```json
{"kind": "progress", "value": {"kind": "state", "path": "facility.humidity"}, "min": 0, "max": 100, "format": {"style": "number", "unit": "%", "precision": 0}}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"progress"` | — |
| `value` | Yes | Ref | Must resolve to numeric; series rejected. |
| `min` | Yes | number | Must be strictly less than `max`. |
| `max` | Yes | number | Must be strictly greater than `min`. |
| `requires` | No | object ≤32 props | Capability gates. |
| `format` | No | Format | Percentage points: `number` style with `unit: "%"`. |

Binding: numeric Refs only (`literal`/`state`/`stream` latest/`view`/`input`).

```json
{"type": "Surface", "id": "humidity", "title": "Humidity", "metric": {"kind": "progress", "value": {"kind": "state", "path": "facility.humidity"}, "min": 0, "max": 100, "format": {"style": "number", "unit": "%", "precision": 0}}}
```

## Metric: gauge

Use for a live reading against a scale (utilization, pressure). Do not use it for simple completion; that is `progress`. Shape is identical to progress; requires `min` strictly less than `max`.

```json
{"kind": "gauge", "value": {"kind": "stream", "subscription": "devices_latest", "selector": "latest", "field": "utilization"}, "min": 0, "max": 100, "format": {"style": "number", "unit": "%", "precision": 0}}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"gauge"` | — |
| `value` | Yes | Ref | Must resolve to numeric; series rejected. |
| `min` | Yes | number | Must be strictly less than `max`. |
| `max` | Yes | number | Must be strictly greater than `min`. |
| `requires` | No | object ≤32 props | Capability gates. |
| `format` | No | Format | Percentage points: `number` style with `unit: "%"`. |

Binding: numeric Refs only. Stream-sourced gauges use `selector: "latest"`.

```json
{"type": "Surface", "id": "utilization", "title": "Fleet utilization", "metric": {"kind": "gauge", "value": {"kind": "stream", "subscription": "devices_latest", "selector": "latest", "field": "utilization"}, "min": 0, "max": 100, "format": {"style": "number", "unit": "%", "precision": 0}}}
```

## Metric: graph.timeseries

Use for a time-series chart. Do not put two series in one chart: use one single-series chart per field (multi-series charts render inconsistently across clients). Do not bind it to latest/event data; every series needs a series binding.

```json
{"kind": "graph.timeseries", "series": [{"label": "Temperature (°C)", "data": {"kind": "stream", "subscription": "telemetry_series", "selector": "series", "field": "temperature"}, "format": {"style": "temperature", "unit": "°C", "precision": 1}}], "xAxis": {"kind": "time"}}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"graph.timeseries"` | — |
| `series` | Yes | Series array, 1–12 items | Each needs `label` ≤1024 and `data` Ref. Keep to 1 entry. |
| `requires` | No | object ≤32 props | Hosts support this kind at `1.0`; gate newer needs here. |
| `xAxis` | No | object | If present, must be exactly `{"kind": "time"}`. |
| `yAxis` | No | object | Optional `{min, max}` numbers; ordering not checked. |

Binding: every `series[].data` must be a `stream` Ref with `selector: "series"`, resolving through a subscription in `timeseries` mode to a numeric field of a timeseries stream.

```json
{"type": "Surface", "id": "temperature_chart", "title": "Temperature trend", "metric": {"kind": "graph.timeseries", "series": [{"label": "Temperature (°C)", "data": {"kind": "stream", "subscription": "telemetry_series", "selector": "series", "field": "temperature"}, "format": {"style": "temperature", "unit": "°C", "precision": 1}}], "xAxis": {"kind": "time"}}}
```

## Metric: graph.bar

Use for per-event rows over an events subscription (alert counts by zone). Do not expect category aggregation: bars render per-event rows, not rolled-up totals. This metric binds by subscription id plus field names, not by Refs.

```json
{"kind": "graph.bar", "subscription": "alert_events", "categoryField": "zone", "valueField": "count", "maxCategories": 12}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"graph.bar"` | — |
| `subscription` | Yes | id string | Must resolve to a subscription in `events` mode over an events stream. |
| `categoryField` | Yes | string | Must be a declared string field of the stream. |
| `valueField` | Yes | string | Must be a declared numeric field of the stream. |
| `maxCategories` | No | integer 1–100 | Caps rendered rows. |
| `requires` | No | object ≤32 props | Hosts support this kind at `1.0`. |
| `format` | No | Format | Formats the values. |

Binding: no Refs. The named subscription must exist on the same dashboard with `mode: "events"`.

```json
{"type": "Surface", "id": "alert_counts", "title": "Count per alert (zone)", "metric": {"kind": "graph.bar", "subscription": "alert_events", "categoryField": "zone", "valueField": "count", "maxCategories": 12}}
```

## Metric: input.toggle

Use for a local on/off switch (reminder preference, flag). Do not use it to write shared product state: view-bound edits update only the local view, never shared state.

```json
{"kind": "input.toggle", "value": {"kind": "view", "key": "local_reminder"}}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"input.toggle"` | — |
| `value` | Yes | Ref | Must be a `view` Ref to a declared boolean key. |
| `onChange` | No | Action | Fired on toggle; often `Action.SetViewState`. |
| `requires` | No | object ≤32 props | Capability gates. |

Binding: `view` Refs only, and the bound value must be boolean or the write is rejected.

```json
{"type": "Surface", "id": "input_preference", "title": "Local reminder preference", "metric": {"kind": "input.toggle", "value": {"kind": "view", "key": "local_reminder"}}}
```

## Metric: input.choice

Use for a local single choice (reporting zone, display mode). Do not use free text for bounded sets; enumerate `choices` (1–100 items of `{title, value}`).

```json
{"kind": "input.choice", "value": {"kind": "view", "key": "reporting_zone"}, "choices": [{"title": "North", "value": "north"}, {"title": "South", "value": "south"}]}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"input.choice"` | — |
| `value` | Yes | Ref | Must be a `view` Ref type-matching the choices. |
| `choices` | Yes | array 1–100 | Items: `title` ≤1024 required, `value` scalar-or-null required. |
| `onChange` | No | Action | Fired on selection. |
| `requires` | No | object ≤32 props | Capability gates. |

Binding: `view` Refs only. Heterogeneous choice values are allowed but keep them one scalar type.

```json
{"type": "Surface", "id": "input_zone", "title": "Reporting zone", "metric": {"kind": "input.choice", "value": {"kind": "view", "key": "reporting_zone"}, "choices": [{"title": "North", "value": "north"}, {"title": "South", "value": "south"}, {"title": "West", "value": "west"}]}}
```

## Metric: input.text

Use for a local free-text note. Do not use it for bounded sets (that is `input.choice`) or secrets without setting `secure: true`. Unbound text (no `value`) keeps view-local scratch text.

```json
{"kind": "input.text", "placeholder": "Add a note for this view", "maxLength": 120, "keyboard": "default"}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"input.text"` | — |
| `value` | No | Ref | When present, must be a `view` Ref to a string key. |
| `onChange` | No | Action | Fired on edit. |
| `placeholder` | No | string ≤1024 | Hint text. |
| `secure` | No | boolean | Masks entry when true. |
| `maxLength` | No | integer 1–4096 | Caps entry length. |
| `keyboard` | No | enum | One of `default/email/url/number/decimal`. |
| `requires` | No | object ≤32 props | Capability gates. |

Binding: `view` Refs only; bound values must be strings.

```json
{"type": "Surface", "id": "input_note", "title": "Local note", "metric": {"kind": "input.text", "placeholder": "Add a note for this view", "maxLength": 120, "keyboard": "default"}}
```

## Metric: input.number

Use for a local numeric entry (target temperature, threshold). Do not use it for shared-state writes; like all inputs it edits only the local view. `min`/`max`/`step` are not cross-checked against each other.

```json
{"kind": "input.number", "value": {"kind": "view", "key": "target_temperature"}, "min": 10, "max": 35, "step": 0.5}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"input.number"` | — |
| `value` | No | Ref | When present, must be a `view` Ref to a numeric key. |
| `onChange` | No | Action | Fired on edit. |
| `min` | No | number | Floor; ordering against `max` not checked. |
| `max` | No | number | Ceiling; ordering against `min` not checked. |
| `step` | No | number > 0 | Increment; must be positive. |
| `requires` | No | object ≤32 props | Capability gates. |

Binding: `view` Refs only; bound values must be numeric (integer accepted for number).

```json
{"type": "Surface", "id": "input_target", "title": "Target temperature", "metric": {"kind": "input.number", "value": {"kind": "view", "key": "target_temperature"}, "min": 10, "max": 35, "step": 0.5}}
```

## Metric: input.date

Use for a local calendar date (review date). Do not use it for timestamps with time-of-day; pair with `input.time` or use two cells.

```json
{"kind": "input.date"}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"input.date"` | — |
| `value` | No | Ref | When present, must be a `view` Ref to a string key. |
| `onChange` | No | Action | Fired on edit. |
| `requires` | No | object ≤32 props | Capability gates. |

Binding: `view` Refs only; resolved values are strings.

```json
{"type": "Surface", "id": "input_date", "title": "Review date", "metric": {"kind": "input.date"}}
```

## Metric: input.time

Use for a local time-of-day (review time). Do not use it for full timestamps; pair with `input.date`.

```json
{"kind": "input.time"}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `kind` | Yes | const `"input.time"` | — |
| `value` | No | Ref | When present, must be a `view` Ref to a string key. |
| `onChange` | No | Action | Fired on edit. |
| `requires` | No | object ≤32 props | Capability gates. |

Binding: `view` Refs only; resolved values are strings.

```json
{"type": "Surface", "id": "input_time", "title": "Review time", "metric": {"kind": "input.time"}}
```

## Forward compatibility: unknown metric kinds

A metric whose `kind` is not one of the 13 above is accepted only with a declared fallback: `"drop"`, or a text substitute `{kind: "text", value, format?}` rendered where the unknown metric would appear. Anything else makes the whole Surface unparsable.

```json
{"kind": "future.metric", "fallback": "drop"}
{"kind": "future.metric", "fallback": {"kind": "text", "value": {"kind": "literal", "value": "Not supported"}}}
```

`kind` must be 1–128 chars and must not collide with a known kind. `requires` gates apply at metric scope too: when a client reports a capability version older than required, the metric-level fallback runs. Component-level `fallback: "drop"` covers the Surface itself.

## Ref bindings (value sources)

Five Ref kinds, all closed sets. Unresolved targets reject the write.

### literal

Inline scalar. `{"kind": "literal", "value": "Ready"}`. Value may be string, number, integer, boolean, or null. Integral numbers within ±2^53−1 resolve as integer, else number.

### state

Shared product state. `{"kind": "state", "path": "facility.temperature"}`. The dotted path must be a declared state field. Reads shared state; inputs never write it.

### stream

Live event data. `{"kind": "stream", "subscription": "telemetry_series", "selector": "series", "field": "temperature"}`. The subscription id must be declared on the same dashboard, and must resolve through the stream source to a declared field. `selector` is `latest` (scalar metrics, gauges) or `series` (timeseries charts; requires a `timeseries`-mode subscription over a timeseries stream and a numeric field).

### view

Dashboard-local view state. `{"kind": "view", "key": "reporting_zone"}`. The key must be declared in the dashboard's `viewState`. Edits update only the local view, never shared product state.

### input

Another input's value. `{"kind": "input", "id": "target_temperature"}`. The id must be a component or cell whose metric is one of the six input kinds. Resolved type follows the control (toggle to boolean, text to string, number to number, date/time to string, choice to its choices' unified type).

## Format and Series helpers

Format (required: `style`):

```json
{"style": "temperature", "unit": "°C", "precision": 1}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `style` | Yes | enum | One of `number/percent/bytes/duration/rate/temperature/date/time/dateTime`. |
| `precision` | No | integer 0–6 | Decimal places. |
| `unit` | No | string ≤32 | Display unit. |

`percent` means fraction: 0.57 renders as 57%. Percentage-point data (57 for 57%) uses `number` style with `unit: "%"`.

Series (required: `label`, `data`):

```json
{"label": "Temperature (°C)", "data": {"kind": "stream", "subscription": "telemetry_series", "selector": "series", "field": "temperature"}}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `label` | Yes | string ≤1024 | Legend label. |
| `data` | Yes | Ref | Must be a series binding (stream, `selector: "series"`). |
| `format` | No | Format | Per-series formatting. |

## What components cannot do

- Inputs never write shared product state or publish events. View-bound edits update only the local view; shared writes need a declared command used through an action.
- No cross-dashboard references. Refs resolve within one dashboard (its subscriptions, view state, component ids) plus definition-level state and streams.
- No styling or layout props on Surface. Width is `span` only; there are no colors, fonts, or card options.
- No nested SurfaceGroups. Groups contain Surfaces only.

These pages are hand-written against the private schema. On any change to dashboards, components, metrics, or refs: update the affected subsection plus the index block, re-validate every snippet on both pages, then bump the version stamp. Merging a schema change without the stamp bump is a docs regression.
