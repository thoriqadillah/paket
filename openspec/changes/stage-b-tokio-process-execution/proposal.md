# Proposal

## Why

Stage A (G0) delivered the domain contracts, but the codebase can still not execute anything:
there is no asynchronous runtime, no subprocess layer, no bounded event delivery, and no
logging. Every later stage — provider probing, catalog reads, native transactions, progress
in the UI — depends on a safe, bounded, explicitly cancellable execution foundation, so
Stage B (plan §2.1, §3.1, gate G1) builds it before any backend is enabled.

## What Changes

- Add Tokio 1.54.1 with a minimal explicit feature set and no `full` (S013).
- Construct a dedicated multi-thread runtime owned by one named thread that runs the
  application coordinator; UI callbacks submit commands through a bounded channel with
  non-blocking sends and results return through bounded events (S014).
- Add structured command specifications: fixed executable, argv array, explicit environment
  overrides, timeout, output limits and capture mode; no shell strings and no arbitrary
  working directory from UI input (S015).
- Define an async process-runner port providers depend on, substitutable in tests (S016).
- Spawn processes safely with sanitized environment and explicit stdio (S017); stream stdout
  and stderr concurrently as typed chunk events (S018); enforce per-command output limits
  with a typed output-limit error (S019); separate read-command and transaction timeout
  policies (S020); add explicit cancellation with retained child ownership (S021); add Unix
  process-group staged termination (S022).
- Add a deterministic fake process runner in the test harness (S023).
- Add bounded runtime event channels with latest-value progress coalescing and reliable
  terminal delivery (S024).
- Add structured logging with configurable levels and redaction of credential-bearing data
  (S025).
- Define application shutdown: stop accepting work, cancel reads, and never exit silently
  while a system transaction is running (S026).
- No package-manager commands, provider code, host probing, scheduler or UI work in this
  stage.

## Capabilities

### New Capabilities

- `process-execution`: structured command specification and bounded, cancellable child-process
  execution with typed output, timeout, cancellation and process-group termination semantics.
- `runtime-lifecycle`: dedicated runtime ownership, bounded command/result flow between the
  UI thread and the application coordinator, and deterministic startup and shutdown.
- `event-delivery`: bounded backend event delivery with progress coalescing and a reliable
  terminal-result path.
- `structured-logging`: bounded, redacted structured logs that make failures diagnosable
  without recording secrets.

### Modified Capabilities

None. Stage B consumes the existing `error-model`, `package-model`, `backend-capabilities` and
`transaction-model` contracts without changing their requirements.

## Impact

- `Cargo.toml` / `Cargo.lock`: add `tokio` 1.54.1 (minimal explicit features), `tokio-util`
  only if cancellation needs a cancellation token, and `tracing` with a subscriber for S025.
- New `src/runtime/` modules: `mod.rs`, `runtime.rs`, `command.rs`, `process.rs`, `events.rs`,
  `logging.rs`; new `src/app/coordinator.rs`.
- New `tests/support/fake_runner.rs` harness; new tests beside the runtime modules.
- No changes to `src/core/` domain models, no provider registration, no UI dependencies.
