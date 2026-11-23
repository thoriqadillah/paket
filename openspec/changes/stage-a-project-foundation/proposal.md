# Proposal

## Why

Paket has a plan but no compiling package or shared domain model. Stage A establishes the
smallest foundation (G0) so later stages can add package-manager behavior without inventing
identity, metadata, capability or error shapes ad hoc.

## What Changes

- Record the repository/toolchain/host baseline (S01).
- Create the `paket` library and `paket-helper` binary targets with only the dependencies the
  next tasks need (S02).
- Add stable backend and package identifiers (S03).
- Add the minimal catalog package model (S04).
- Add backend availability and capability descriptors (S05).
- Add a small typed error model for catalog and operation failures (S06).
- No package-manager commands, UI dependencies or provider registry in this stage.

## Capabilities

### New Capabilities

- `package-model`: stable `BackendId`/`PkgId` identity and the minimal `Pkg` catalog row,
  including JSON round-tripping with optional metadata.
- `backend-capabilities`: backend descriptor with availability, optional reason/version and
  the declared capability set.
- `error-model`: typed catalog and operation errors with diagnostic context and readable
  display text.

### Modified Capabilities

None.

## Impact

- New `Cargo.toml`, `src/lib.rs`, `src/bin/helper.rs`; new `src/core/` modules for model and
  errors.
- Dependencies: `serde` (derive), `serde_json`, `thiserror`. No `anyhow`, no Slint.
- `docs/baseline.md` added by S01.
- No change to existing code; the repository is greenfield.
