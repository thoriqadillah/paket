# Design

## Context

The repository is greenfield. S01 produced `docs/baseline.md`; S02 already created the
`paket` package (edition 2024, `rust-version = 1.85`), `src/lib.rs`, `src/bin/helper.rs`
and the `serde`/`serde_json`/`thiserror` dependencies. Remaining work is S03–S06. See
proposal.md for motivation.

## Goals / Non-Goals

**Goals:**
- One small `src/core/` module tree owning identity, catalog rows, backend descriptors and
  errors, reused by every later backend.
- Keep native identifiers opaque in core.
- Compile and test with `cargo check --all-targets`, `cargo test`, `cargo fmt --check`.

**Non-Goals:**
- Probing hosts, running package commands, provider traits or registries (Stages B+).
- Version comparison, icons, categories, AppStream or UI types.

## Decisions

- `BackendId` as a plain enum with a `as_str()` returning stable lowercase names; `PkgId`
  stores `backend` plus an opaque `native: String`. A derived `Hash`/`Eq` gives identity;
  `key()` returns `"<backend>:<native>"` for list models. Alternative: stringly backend
  names — rejected, typos would compile.
- `Pkg` mirrors §3.2 exactly, with `Option` for all possibly-missing metadata and a
  required `PkgId`; derives `Serialize`/`Deserialize` and skips nothing, so absent values
  round-trip as `null`/missing. Alternative: default empty strings — rejected, invents data.
- `BackendInfo` holds `available: bool` plus `reason_unavailable: Option<String>`, and
  `capabilities: Vec<Capability>`; `Capability` is a small enum with only the six planned
  operations. Alternative: boolean flags per feature — rejected as unbounded.
- `CatalogError` and `OperationError` are `thiserror` enums; command-style variants carry
  optional diagnostic text but keep a distinct meaning. No `anyhow`.
- Module layout: `src/core/mod.rs` re-exports `model` and `error`; tests live in the same
  files under `#[cfg(test)]`, matching `src/bin/helper.rs`.

## Risks / Trade-offs

- `rust-version = 1.85` while the host toolchain is 1.97 → edition 2024 is available on both;
  keep code free of newer-edition-only syntax.
- Adding capability variants later requires touching the enum → acceptable; plan says add
  capabilities only when a task needs them.
- `Vec<Capability>` allows duplicates → no dedup now; callers construct descriptors once.
