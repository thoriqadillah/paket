# Design

## Context

The repository is no longer empty: S001 produced `docs/baseline.md`, and the package already
has `Cargo.toml` (edition 2024, `rust-version = 1.85`), `src/lib.rs`,
`src/bin/helper.rs`, `src/core/model.rs` with `BackendId`/`PkgId`, and the
`serde`/`serde_json`/`thiserror` dependencies. The production plan adds S002–S012, so
existing work is partial and must be completed rather than re-scaffolded. See
`proposal.md` for motivation.

## Goals / Non-Goals

**Goals:**
- A reproducible toolchain, GUI/helper targets and a repeatable lint baseline.
- One small `src/core/` module tree owning identity, scopes, catalog records, application
  grouping, backend capabilities and transaction shapes, reused by every later stage.
- Native identifiers opaque in core; display names never used as mutation identity.
- Compile and test with `cargo check --all-targets`, `cargo test`, `cargo fmt --check` and
  `cargo clippy --all-targets -- -D warnings`.

**Non-Goals:**
- Tokio, process execution, host probing, provider registry or provider traits (Stages B+).
- Version comparison, AppStream enrichment, icons, cache, persistence or UI types.
- Any package-manager operation.

## Decisions

- **Toolchain:** edition 2024, `rust-version = "1.95"` while Slint Kit 0.9.0 is selected, and
  `rust-toolchain.toml` pinning the CI channel. Host toolchain is 1.97; keep code free of
  newer-only syntax.
- **Targets:** one library, a GUI placeholder at `src/main.rs`, and the `paket-helper`
  placeholder. No Slint dependency yet.
- **Policy:** standard rustfmt, no broad allowlists, clippy warnings denied in CI.
- **Errors:** `CatalogError`, `PlanError` and `OperationError` as `thiserror` enums. Native
  stderr/diagnostics stay in context fields, separate from `Display`. No `anyhow`.
- **Identity:** `BackendId` enum (seven providers), `InstallationScope` enum (system, user,
  named Flatpak installation, Paket-managed local store), `PackageId { backend, scope,
  native }` and `SourceId`. A derived `Hash`/`Eq` gives identity; `key()` includes all
  discriminators.
- **Catalog:** `PackageRecord` with optional version/architecture/summary/description/
  source/size, installed state, package kind and metadata origin; `UpdateRecord`;
  `SearchScope` filters by optional backend, scope and source. Uncertain metadata is
  optional or explicitly unknown, never invented.
- **Applications:** `AppId` independent of `PackageId`; `AppRecord` references several
  package candidates without collapsing them. Grouping never changes install identity.
- **Capabilities:** one enum covering installed list, search, details, updates, plan,
  install, remove, sources, local import, launch and history. `BackendInfo` carries
  availability, optional version, unsupported reason and the capability set.
- **Transactions:** `ChangeIntent` (selected by the user), `ResolvedChange` (solver effects),
  `BackendPlan`, `BatchPlan`, `BackendOutcome`, preview quality and job/step statuses.
  Selected intents stay separate from resolved dependency effects; batches are non-atomic.
- **Module layout:** `src/core/mod.rs` re-exports `error`, `ids`, `package`, `capability`
  and `transaction`; tests live beside the code under `#[cfg(test)]`, matching the existing
  helper.

## Risks / Trade-offs

- Rewriting `src/core/model.rs` into focused modules touches existing tests; keep identity
  behavior and add coverage rather than deleting assertions.
- Raising `rust-version` from 1.85 to 1.95 requires the CI image to offer 1.95+; the host
  already has 1.97.
- The capability enum grows with the plan; add only variants a task needs and keep unknown
  operations disabled.
- S004 and S007 are partially implemented; check their DoD against the actual code before
  marking them complete.
