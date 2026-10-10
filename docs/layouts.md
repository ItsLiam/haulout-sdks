# Layouts

Validated against definition v2.0.

## How to read this page

Valid means the hosted API accepts the definition write. These pages describe the accepted shapes; the API's validator decides. On conflict between docs and a rejected write, the rejection wins.

A dashboard is one object inside the definition's `dashboards` array (0–32 entries). Metric shapes live in `components.md`; this page owns the dashboard shell, the grid, and composition. Every snippet is schema-shaped and was verified against the canonical definition schema before publishing. Snippets need a live product key; without one they do not run.

## Dashboard shell

```json
{"type": "Dashboard", "version": "2.0", "id": "overview", "title": "Facility overview", "subscriptions": [], "components": [], "actions": []}
```

| Prop | Required | Type | Notes |
|---|---|---|---|
| `type` | Yes | const `"Dashboard"` | Discriminator. |
| `version` | Yes | const `"2.0"` | Must match the definition version. |
| `id` | Yes | id string | Unique across dashboards; navigation targets this. |
| `title` | Yes | string | Dashboard title. |
| `components` | Yes | array 0–500 | Surfaces and SurfaceGroups, including group cells in the count. |
| `fallbackText` | No | string ≤4096 | Shown when the dashboard cannot render. |
| `requires` | No | object ≤32 props | Capability gates; values are `major.minor` strings. |
| `viewState` | No | object ≤64 keys | Local key/value declarations; see below. |
| `subscriptions` | No | array 0–128 | Stream bindings; see below. |
| `actions` | No | array 0–20 | Dashboard-level buttons; see below. |

This object is written as part of the whole definition with CAS (`p_expected_revision` = current revision); see `definitions-state.md`. There is no per-dashboard write.

## Subscriptions, viewState, actions: where they live

All three live on the dashboard object, and all Refs on that dashboard resolve against them.

Subscriptions bind streams (required: `id`, `source`, `mode`):

```json
{"id": "telemetry_series", "source": "telemetry", "mode": "timeseries"}
```

`mode` is one of `latest`, `timeseries`, `events`. Subscription ids must be unique within the dashboard. Optional `parameters` (up to 8 keys) take Refs, but never series bindings. Optional `window` (`"1h"`, `"24h"`, `"7d"`) keeps only events from the last hour / day / week, evaluated when the dashboard renders; without it, charts see all loaded events. A window narrows what is already loaded, it does not fetch older history. Every chart reading a windowed subscription must declare `"requires": {"subscription.window": "1.0"}` on its own metric, or the definition is rejected:

```json
{"id": "alert_events_1h", "source": "alerts", "mode": "events", "window": "1h"}
```

View state declares local keys (required: `type`, `default`):

```json
{"local_reminder": {"type": "boolean", "default": false}, "reporting_zone": {"type": "string", "default": "north", "allowed": ["north", "south", "west"]}}
```

`type` is one of `string`, `boolean`, `integer`, `number`. The default must match the type and fall inside `allowed` when present (1–100 entries). Optional `persistence` is one of `none`, `session`, `device`. Inputs bound to these keys edit only the local view, never shared state.

Actions are dashboard-level buttons (all require `type` + `title` ≤1024; optional `style` of `default/primary/secondary/destructive`):

```json
{"type": "Action.Navigate", "title": "View charts", "dashboard": "charts"}
{"type": "Action.SetViewState", "title": "Use south zone", "key": "reporting_zone", "value": "south"}
{"type": "Action.ToggleVisibility", "title": "Toggle local note", "targets": ["input_note"]}
{"type": "Action.Execute", "title": "Vent", "style": "destructive", "command": "vent", "arguments": {}}
{"type": "Action.OpenUrl", "title": "Docs", "url": "https://example.com/x"}
```

Execute needs a declared `command` plus matching `arguments` (up to 32 Ref values); destructive commands require `style: "destructive"`. Navigate targets a dashboard id, not a Surface. ToggleVisibility targets component or cell ids. SetViewState needs a declared key with a type-matching value inside `allowed`. Inputs also accept one action as `onChange`.

## Grid model

The dashboard is an outer grid. Each top-level component takes `span` columns of it:

- Surface `span`: integer 1–24, default 1.
- SurfaceGroup `span`: integer 1–12, default 1.
- Spans are integers only: no fractions, percentages, or strings.
- Omitted span means 1. Defaults apply at render time and are never written back into submitted JSON.

A group with `span: 2` is twice as wide as a default Surface on the outer grid; what happens inside the group is governed by `columns` (next section).

## Surface vs SurfaceGroup

One metric: Surface. Two or more related metrics under one heading: SurfaceGroup. That is the whole rule.

Nonrecursive nesting rule: a SurfaceGroup's `metrics` array holds Surfaces only (1–100). A group inside a group fails validation, and clients reject it. Groups never nest; pages are exactly two levels (dashboard, then components, then group cells).

## SurfaceGroup columns and inner spans

`columns` (integer 1–24) sets the group's inner grid width. Omitted `columns` means `span * 2` at render time: `span: 1` renders 2 inner columns, `span: 2` renders 4. Like span defaults, this is render-time behavior, never written back.

Cell spans are relative to the inner grid, not the page: a cell with `span: 2` inside a 4-column group takes half the group. Cell span range is 1–24 with default 1, same as a top-level Surface.

## Responsive clamping

Spans that exceed the available columns clamp at render time; the configured values never change. On narrow screens the outer grid shrinks to fewer columns and wide components wrap to full width; inside groups, the rendered inner columns shrink below the declared `columns` and oversize cells clamp to fit. Compose with that in mind: a `span: 2` group stays readable at one column, and cells with span 1 always fit. No per-client breakpoint tables are documented here; clients agree on clamp-don't-mutate.

## Composition patterns

### Pattern A: single-column status page {#pattern-a}

One group or a short stack of Surfaces, every span default. Intent: glanceable health with zero layout thought. Skeleton:

```json
{"type": "Dashboard", "version": "2.0", "id": "status", "title": "Status", "components": [{"type": "Surface", "id": "health", "title": "Health", "metric": {"kind": "status", "...": "..."}}]}
```

Metric bodies: `components.md#metric-status`, `#metric-text`.

### Pattern B: overview + detail {#pattern-b}

A wide group (`span: 2`) of headline metrics first, then full-width detail Surfaces below. Intent: the common home page. Skeleton:

```json
{"type": "Dashboard", "version": "2.0", "id": "overview", "title": "Overview", "components": [{"type": "SurfaceGroup", "id": "live", "title": "Live", "span": 2, "metrics": []}, {"type": "Surface", "id": "detail", "title": "Detail", "metric": {"kind": "number", "...": "..."}}]}
```

Metric bodies: `components.md#metric-number`, `#metric-progress`, `#metric-gauge`.

### Pattern C: charts page {#pattern-c}

One single-series `graph.timeseries` Surface per field, plus at most one `graph.bar` over an events subscription. Intent: trends without multi-series rendering surprises. Skeleton:

```json
{"type": "Dashboard", "version": "2.0", "id": "charts", "title": "Trends", "subscriptions": [{"id": "telemetry_series", "source": "telemetry", "mode": "timeseries"}], "components": [{"type": "Surface", "id": "t_chart", "title": "Temperature trend", "metric": {"kind": "graph.timeseries", "...": "..."}}]}
```

Metric bodies: `components.md#metric-graph-timeseries`, `#metric-graph-bar`. Every chart needs its subscription declared above.

### Pattern D: local-controls page {#pattern-d}

View-state declarations plus input Surfaces bound to them. Intent: per-client preferences that never touch shared state. Skeleton:

```json
{"type": "Dashboard", "version": "2.0", "id": "controls", "title": "Controls", "viewState": {"local_reminder": {"type": "boolean", "default": false}}, "components": [{"type": "Surface", "id": "reminder", "title": "Reminder", "metric": {"kind": "input.toggle", "...": "..."}}]}
```

Metric bodies: `components.md#metric-input-toggle` through `#metric-input-time`. State the limitation on the page if users will ask: these controls update only the local view.

## Full-page example

Self-contained dashboard: one subscription, one group, two Surfaces, one navigation action.

```json
{"type": "Dashboard", "version": "2.0", "id": "overview", "title": "Facility overview",
"subscriptions": [{"id": "telemetry_latest", "source": "telemetry", "mode": "latest"}],
"components": [
{"type": "SurfaceGroup", "id": "live_facility", "title": "Live facility", "span": 2, "metrics": [
{"type": "Surface", "id": "temperature", "title": "Temperature", "metric": {"kind": "number", "value": {"kind": "state", "path": "facility.temperature"}, "format": {"style": "temperature", "unit": "°C", "precision": 1}}},
{"type": "Surface", "id": "humidity", "title": "Humidity", "metric": {"kind": "progress", "value": {"kind": "state", "path": "facility.humidity"}, "min": 0, "max": 100, "format": {"style": "number", "unit": "%", "precision": 0}}}]},
{"type": "Surface", "id": "facility_name", "title": "Facility", "metric": {"kind": "text", "value": {"kind": "state", "path": "facility.name"}}},
{"type": "Surface", "id": "latest_zone", "title": "Latest zone", "metric": {"kind": "text", "value": {"kind": "stream", "subscription": "telemetry_latest", "selector": "latest", "field": "zone"}}}],
"actions": [{"type": "Action.Navigate", "title": "View charts", "dashboard": "charts"}]}
```

## Invalid layouts

1. Chart without its subscription. `{"kind": "graph.timeseries", "series": [{"label": "T", "data": {"kind": "stream", "subscription": "missing", "selector": "series", "field": "temperature"}}]}` fails on the unresolved subscription. Fix: declare `{"id": "missing", "source": "telemetry", "mode": "timeseries"}` in `subscriptions`.
2. Span as a string. `{"type": "Surface", "id": "a", "title": "A", "span": "2", "metric": {...}}` fails; span must be an integer. Fix: `"span": 2`.
3. Nested group. `{"type": "SurfaceGroup", "id": "g", "title": "G", "metrics": [{"type": "SurfaceGroup", ...}]}` fails; cells must be Surfaces. Fix: flatten to one group of Surfaces.
4. Ref to an undeclared target. `{"kind": "state", "path": "facility.ghost"}` fails when `facility.ghost` is not a declared state field. Fix: declare the field or point at one that exists.
5. Input pretending to write shared state. `{"kind": "input.toggle", "value": {"kind": "state", "path": "facility.online"}}` mismatches the control: inputs bind `view` keys only. Fix: declare a boolean in `viewState` and bind `{"kind": "view", "key": "..."}`.

## Compose loop

1. Pick a layout pattern (#pattern-a to #pattern-d above).
2. Place components: one metric per Surface, related metrics in a SurfaceGroup, never nested.
3. Fill metric bodies from the `components.md` anchors.
4. Bind: every `state`/`stream`/`view`/`input` Ref resolves to a declared field, subscription, key, or input id on the same dashboard.
5. Validate each snippet against the canonical definition schema before assembling.
6. Write the whole definition with CAS (`p_expected_revision` = current revision); see `definitions-state.md`.

Follow the loop in order. Do not invent props, do not copy example ids verbatim (they will collide), do not emit spans as strings.

These pages are hand-written against the private schema. On any change to dashboards, components, metrics, or refs: update the affected subsection plus the component index, re-validate every snippet on both pages, then bump the version stamp. Merging a schema change without the stamp bump is a docs regression.
