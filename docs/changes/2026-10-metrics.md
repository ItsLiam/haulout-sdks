# 2026-10: metrics replace definitions, state, and events

This is a breaking v2 contract. The clients now expose two operations, `put_metrics_by_key` and `delete_metrics_by_key`. These replace the previous six operations: definition write, state write, event publish, event purge, command claim, and command completion.

## Removed

- `write_definition_by_key`, `write_state_by_key`
- `publish_event_batch_by_key`, `purge_events_by_key`
- `claim_command_by_key`, `complete_command_by_key`
- All of their request and response models, including `ProductDefinition`, `Command`, `CommandClaim`, and `Stream`.

## Added

- `put_metrics_by_key`: write up to 100 metrics per call, each with one of nine kinds. See [`metrics.md`](../metrics.md).
- `delete_metrics_by_key`: remove up to 256 keys per call.

## Migrating

- Replace each dashboard, state field, and event stream with one metric per value you show. Timeseries metrics carry their own points.
- Replace command handling with your own transport. The SDK no longer carries commands.
- Error codes are now `22023` (invalid input) and `28000` (invalid product key) only. Remove handling for `PT409`, `23505`, `42501`, and `P0002`.

Clients built on the previous contract cannot call the new operations; regenerate or upgrade.
