# Tasks

Stage B tasks mirror `docs/plan.md` S013–S026. The repository currently contains none of the
Stage B work — no `src/runtime/`, no `src/app/`, no `tests/` tree, no `tokio` or `tracing`
dependencies — so every task below is unchecked. One task is implemented per agent run
(AGENTS.md), each verified against its DoD before its box is marked.

## 1. Runtime foundation

- [ ] S013 Add Tokio 1.54.1 with only the needed features (`rt-multi-thread`, `macros`,
  `process`, `time`, `sync`, `io-util`, `fs`, `signal` where supported) to `Cargo.toml`;
  verify `cargo tree -e features` shows exactly the enabled features and
  `cargo check --all-targets` passes.
- [ ] S014 Construct the dedicated multi-thread Tokio runtime owned by a named `std::thread`
  that runs the application coordinator, with bounded `mpsc` command submission,
  `try_send` from UI callbacks, results through bounded events, and explicit startup,
  shutdown and join ownership; verify a test starts the owner, round-trips one command,
  requests shutdown and joins the thread. Do not use `#[tokio::main]`.
- [ ] S024 Add typed bounded runtime event channels — bounded `mpsc` for commands and
  terminal events, latest-value `watch`/coalescing for progress; verify tests cover queue
  capacity and progress coalescing with terminal events never silently lost.

## 2. Command specification and process execution

- [ ] S015 Define the structured `CommandSpec` — fixed executable, `Vec<OsString>` argv,
  explicit environment overrides, timeout, output limits, capture mode — accepting no shell
  string and no working directory from UI input; verify construction tests and a test proving
  a query containing spaces stays one argv element.
- [ ] S016 Define the async process-runner port returning a handle or structured completion
  and events, independent of concrete Tokio child types; verify with a compile-time fake
  implementation.
- [ ] S017 Spawn one process safely with fixed executable and argv, sanitized environment,
  forced `LC_ALL=C` where parsers need stable English, and explicit stdio, never invoking a
  shell; verify harmless-executable tests for success, non-zero exit and missing executable.
- [ ] S018 Stream stdout and stderr concurrently as typed chunk events with source stream,
  preserving final partial lines without assuming UTF-8; verify with a child writing heavily
  to both streams.
- [ ] S019 Enforce per-command stdout/stderr limits with a bounded diagnostic tail and
  streaming mode for long transactions, reporting a typed output-limit error on the hard
  limit and terminating per cancellation policy; verify output just below and above the
  limit.
- [ ] S020 Add command timeouts, separating short read-command defaults from long
  transaction policy where a transaction timeout requests graceful cancellation and
  reconciles before reporting; verify timeout tests with a sleeping child.
- [ ] S021 Add explicit cancellation tokens so a caller can request cancellation while
  retaining child ownership — queued work cancels without spawn, running work enters
  `CancelRequested` until exit and reap; verify tests for cancellation before spawn and
  while running (add `tokio-util` only if required).
- [ ] S022 Add Unix process-group cancellation for providers that may spawn descendants —
  dedicated process group, staged termination (graceful interrupt, bounded wait,
  provider-approved termination), child kept alive and reaped; verify parent+child
  termination with a disposable helper process.

## 3. Test harness

- [ ] S023 Add the deterministic fake process runner under `tests/support/` queueing
  predetermined results/events, asserting exact executable/argv/env and failing on unexpected
  commands; verify the fake-runner self-test for matching and mismatch.

## 4. Logging and shutdown

- [ ] S025 Add `tracing`-based structured logging with configurable levels and bounded
  rotation, redacting credential-bearing URLs, authorization data, environment variables and
  sensitive kangpaket payload fields; verify redaction tests and a sample log record.
- [ ] S026 Define application shutdown in the coordinator — stop accepting new work, cancel
  read-only work, never silently exit or drop the child handle while a system transaction
  runs, persist enough state for next-launch reconciliation; verify unit tests for shutdown
  with idle, queued and active task states.

## Workflow follow-up

- Implement exactly one unchecked task per agent run; do not continue to the next task
  (AGENTS.md §Per-run workflow).
- Run `cargo fmt --check` and the narrowest relevant `cargo test` after every task;
  `cargo clippy --all-targets -- -D warnings` at gate G1.
- Confirm gate G1 (S013–S026) before archiving; archive the change when every task's DoD
  passes. Do not start Stage C in the same run.
