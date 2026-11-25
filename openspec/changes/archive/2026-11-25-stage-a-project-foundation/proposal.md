# Proposal

## Why

Paket has a production plan but only a partial foundation: a Cargo package and identifier
types exist, but naming/licensing, toolchain pinning, the GUI target, lint policy, error
families, scopes, catalog/app records and transaction models do not. Stage A establishes
these tested contracts (G0) so later stages can add package-manager behavior without
inventing identity, metadata, capability or transaction shapes ad hoc.

## What Changes

- Record the repository/toolchain/host baseline (S001).
- Lock naming and licensing, including the Slint Kit GPL compatibility decision (S002).
- Pin edition 2024, `rust-version = "1.95"` and the CI toolchain channel (S003).
- Complete the `paket` library, GUI placeholder and `kangpaket` targets (S004).
- Adopt rustfmt and warnings-denied clippy as the local baseline (S005).
- Add typed catalog, plan and operation error families (S006).
- Add backend/package identity with installation scope and `SourceId` (S007).
- Model installed/system/user/named/local-store scopes (S008).
- Add catalog package, update and search-scope records (S009).
- Add independent application grouping over package candidates (S010).
- Add the expanded capability set and backend descriptor (S011).
- Add transaction intents, batch plans, outcomes and preview quality (S012).
- No package-manager commands, Tokio, provider registry or UI dependencies in this stage.

## Capabilities

### New Capabilities

- `package-model`: backend- and scope-qualified identity, source identity, catalog package
  and update records, search scope, and application grouping over package candidates.
- `backend-capabilities`: backend descriptor with availability, version, unsupported reason
  and the declared capability set.
- `error-model`: typed catalog, plan and operation failures with safe user messages and
  separate diagnostics.
- `transaction-model`: intents, resolved changes, per-backend and batch plans, preview
  quality, outcomes and partial-success summaries.

### Modified Capabilities

None.

## Impact

- `Cargo.toml`, `rust-toolchain.toml`, `src/main.rs`, `src/lib.rs`, `src/bin/kangpaket.rs`.
- New `src/core/` modules for errors, IDs, packages, capabilities and transactions.
- Existing `src/core/model.rs` is partial and will be completed/renamed by S007–S011.
- Dependencies: `serde` (derive), `serde_json`, `thiserror`. No Tokio, Slint or backend
  dependencies.
- `docs/baseline.md` (S001) and `docs/decisions.md` (S002) are documentation deliverables.
- No existing provider or executable behavior changes.
