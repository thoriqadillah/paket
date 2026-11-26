# Design

## Context

Stage A left the crate with only domain models under `src/core/` (see proposal.md — Why).
`src/main.rs` and `src/bin/kangpaket.rs` are `ExitCode::SUCCESS` placeholders, there is no
`src/runtime/`, no `src/app/`, no `tests/` tree, and `Cargo.toml` carries only
serde/serde_json/thiserror. Plan §2.1 fixes the runtime model (D6–D8: Tokio 1.54.1 on a
dedicated multi-thread runtime, never `#[tokio::main]` around the Slint loop) and §3.1 gives
the module layout this stage creates. The error model already provides `OperationError`
Timeout/Cancelled variants that Stage B reports from the process layer.

## Goals / Non-Goals

**Goals:**
- A dedicated multi-thread Tokio runtime owned by one named `std::thread` that block_on-runs
  the coordinator, joinable on shutdown.
- Structured `CommandSpec` → runner port → process implementation chain, with bounded output,
  explicit timeout policy, explicit cancellation, and Unix process-group termination.
- Bounded command/result/event flow with latest-value progress coalescing and reliable
  terminal delivery; deterministic fake runner for provider tests; `tracing`-based logging
  with redaction.
- Every task validated by `cargo fmt --check` plus the narrowest relevant `cargo test`.

**Non-Goals:**
- Any package-manager command, provider code, host probing, registry, scheduler or read/write
  coordination (Stage C+ per plan §0.2, S033–S036).
- Slint integration; the channel contract is exercised from tests without a UI (Stage M).
- The final shutdown UI dialog (S026 leaves that to Stage M).

## Decisions

- **Tokio features (S013):** `rt-multi-thread`, `macros`, `process`, `time`, `sync`,
  `io-util`, `fs`, `signal` — the plan's exact list; never `full`. `tokio-util` is added only
  if S021's cancellation token needs it, otherwise a small `AtomicBool`/`watch` signal is
  enough. Rationale: minimal feature surface, matching D6.
- **Runtime shape (S014):** `build_runtime()` returns a `Runtime`; a dedicated named thread
  owns it and `block_on`s the coordinator loop. UI side gets a bounded `mpsc::Sender<AppCommand>`
  used with `try_send`. Alternative rejected: `#[tokio::main]` — plan D8 forbids wrapping the
  Slint loop in a Tokio runtime.
- **CommandSpec (S015):** executable as an absolute/validated path plus `Vec<OsString>` argv;
  environment as an explicit override map (whitelist-set, with `LC_ALL=C` for stable parsers);
  timeout, output limits and capture mode as explicit fields. No working-directory field from
  UI input; no shell-string constructor exists at all.
- **Runner port (S016):** one `#[async_trait]`-free small async interface (providers are added
  in Stage D via `async-trait`; here a plain generic/trait object suffices — final naming left
  to implementation). Returns a completion with exit status, captured output and events; the
  concrete Tokio implementation stays behind the port so providers and tests never depend on
  `tokio::process::Child` types.
- **Process implementation (S017–S022):** `tokio::process::Command` with explicit stdio;
  concurrent drain of both pipes preserving final partial lines as bytes; per-stream output
  limits with bounded tail retention, typed output-limit error (maps to `CatalogError`/
  `OperationError` context, not a new core variant); read-vs-transaction timeout split;
  cancellation keeps the `Child` owned until reaped (drop ≠ proof of stop, per §2.1); Unix
  `setsid`-style process group with staged termination — interrupt, bounded wait, then kill —
  documented per backend that must not be force-killed mid-transaction.
- **Events (S024):** typed `BackendEvent`; bounded `mpsc` for commands and terminal events;
  `watch` (or equivalent latest-value cell) for progress. No broadcast bus (plan S024 forbids
  it).
- **Logging (S025):** `tracing` + a subscriber writer with size-bounded file rotation and a
  redaction layer for credential-bearing URLs, auth data, env values and kangpaket payload
  fields. No telemetry. `tracing-substrate` alternatives were not considered — the Rust
  ecosystem standard is `tracing-subscriber`; license check per decisions.md (MIT/Apache-2.0
  compatible) before the dependency lands in S025's own run.
- **Shutdown (S026):** coordinator state machine (accepting → draining → stopped) in
  `src/app/coordinator.rs`; cancel read-only work, refuse new commands with a typed
  rejection, and for an active system transaction keep the child owned and surface
  wait-for-completion; persist just enough state for the Stage C reconciliation design
  (S039) without implementing persistence now.

## Risks / Trade-offs

- Tokio version drift (plan pins 1.54.1; cargo may resolve a newer 1.x) → pin
  `=1.54.1`-compatible range in S013 and record any resolver mismatch in
  `docs/deviations.md`.
- Process-group tests need a disposable helper process → keep helpers in `tests/` fixtures;
  never test against host package managers.
- Blocking sends on full event channels could stall a child drain → drain tasks write with
  timeouts/failure propagation, and terminal results get their own reliable path.
- Logging file rotation policy is host-dependent → keep it configurable and bounded; exact
  rotation scheme documented in S025.
- `signal` feature availability across platforms → feature is added "where supported";
  Unix is the only production target in the support matrix.

## Open Questions

None that change specs, approach or task breakdown. The concrete runner-port trait shape
(named trait vs generic parameter) is decided in S016's own run against what compiles
cleanest.
