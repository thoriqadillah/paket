# Tasks

## 1. Baseline

- [x] S01 Record repository, toolchain and host baseline in `docs/baseline.md`; verify the
  file names the layout, `cargo`/`rustc` versions, distro, `dnf5`/`flatpak`/`pkexec`
  presence, baseline command results and blockers, with no production files changed.

## 2. Package scaffolding

- [x] S02 Create the `paket` library and `paket-helper` binary with `serde`, `serde_json` and
  `thiserror`; verify `cargo check --all-targets` and `cargo test` pass and no UI
  dependency is present.

## 3. Identifiers

- [x] S03 Add `BackendId` (all seven backends), `PkgId` and stable string forms; verify unit
  tests show `dnf5:foo` and `flatpak:foo` remain distinct identities.

## 4. Catalog model

- [ ] S04 Add the minimal `Pkg` model from plan §3.2; verify JSON round-trip tests keep the
  ID and optional fields, including a package with no optional metadata.

## 5. Backend descriptors

- [ ] S05 Add `Capability` and `BackendInfo`; verify tests cover available and unavailable
  descriptors, with an unavailable backend carrying a reason and limited capabilities.

## 6. Errors

- [ ] S06 Add `CatalogError` and `OperationError` with `thiserror` display text; verify unit
  tests cover representative messages and variant matching, and no `anyhow` is added.

## Workflow follow-up

- Run `cargo fmt --check` and confirm gate G0 (S01–S06) before archiving.
- Archive the change when all tasks pass; do not start Stage B in the same run.
