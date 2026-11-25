# Tasks

Stage A tasks mirror `docs/plan.md` S001–S012. Only S001 is complete. The existing Cargo
package and identity types partially cover S004 and S007 and must be finished there.

## 1. Baseline and decisions

- [x] S001 Record repository, toolchain and host baseline in `docs/baseline.md`; verify it
  names the layout, `cargo`/`rustc` versions, distro, installed package tools, baseline
  command results and blockers, with no source files changed.
- [x] S002 Record naming, distribution, project license and third-party license policy in
  `docs/decisions.md`; verify Slint Kit GPL-3.0-or-later compatibility is an explicit
  yes/no with evidence.
- [x] S003 Pin edition 2024, `rust-version = "1.95"` and the CI toolchain channel; verify
  `rustc --version` and `cargo check` pass with the pinned toolchain.

## 2. Package scaffolding and policy

- [x] S004 Create the `paket` library, GUI binary placeholder and `paket-helper` placeholder;
  verify `cargo check --all-targets` and `cargo test` pass with no UI dependency. Existing
  `src/lib.rs` and `src/bin/helper.rs` are partial; `src/main.rs` is missing.
- [x] S005 Adopt standard rustfmt and a warnings-denied clippy baseline; verify
  `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` pass.

## 3. Core domain

- [x] S006 Add distinct catalog, plan and operation error families with safe user messages;
  verify unit tests match every variant without parsing stderr.
- [x] S007 Add `BackendId`, `PackageId { backend, scope, native }` and `SourceId` with stable
  keys; verify equality/hash/serialization tests keep same-name packages distinct. Existing
  `BackendId`/`PkgId` are partial; scope and `SourceId` are missing.
- [x] S008 Add typed installation scopes (system, user, named Flatpak installation, managed
  local store); verify user and system refs stay distinct through serde.
- [x] S009 Add catalog package, update and search-scope records; verify serialization tests
  cover missing fields and same-name records remain distinct.
- [x] S010 Add independent application grouping with many-to-many package candidates; verify
  one app maps to RPM and Flatpak candidates without collapsing install identity.
- [x] S011 Add the expanded capability set and backend descriptor; verify absent, read-only
  and fully available descriptors are distinguishable.
- [x] S012 Add transaction domain models for intents, resolved changes, plans, outcomes and
  preview quality; verify native, partial, unavailable and multi-backend partial-failure
  cases.

## Workflow follow-up

- Run `cargo fmt --check` and confirm gate G0 (S001–S012) before archiving.
- Archive the change when all tasks pass; do not start Stage B in the same run.
