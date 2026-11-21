# Paket — Repository Baseline

Recorded during S01. This is a factual snapshot, not a progress log.

## Repository layout (as found)

```text
.agents/skills/          OpenSpec and ponytail skill definitions
docs/plan.md             Incremental implementation plan
openspec/config.yaml     OpenSpec project config (schema: spec-driven)
openspec/changes/archive/.gitkeep
openspec/specs/.gitkeep
.git/
```

- No `Cargo.toml`, `src/`, `tests/`, or `scripts/` directory exists yet.
- No CI configuration (`.github/`, `.gitlab-ci.yml`, etc.).
- No existing Rust code, so there is no overlap with the plan to reconcile.
- The repository is effectively greenfield for S02.

### Git state at baseline

- Branch: `master`
- Recent commits: `2e21300 skills`, `634943f openspec`
- Pre-existing changes (unrelated to S01, left untouched):
  - staged: `.agents/.skill-lock.json` modified
  - staged-then-deleted: `openspec/research.md` (`AD`)
  - untracked: `docs/`

### Toolchain

- `cargo` 1.97.1 (c980f4866 2026-06-30)
- `rustc` 1.97.1 (8bab26f4f 2026-07-14)
- `rustup` 1.29.0 (28d1352db 2026-03-05)
- Active default toolchain: `stable-x86_64-unknown-linux-gnu`
- Installed toolchains: `1.97-x86_64-unknown-linux-gnu`, `1.99.1-x86_64-unknown-linux-gnu`
- Rust 1.97 supports edition 2024, so the edition chosen in S02 is available.

### Host

- `NAME="Fedora Linux"`
- `VERSION="44 (KDE Plasma Desktop Edition)"`
- `ID=fedora`, `VERSION_ID=44`, `VARIANT_ID=kde`
- `PRETTY_NAME="Fedora Linux 44 (KDE Plasma Desktop Edition)"`

### Required host tools

| Tool | Path | Version |
|---|---|---|
| `dnf5` | `/usr/bin/dnf5` | 5.4.7.0 (libdnf5 5.4.7.0) |
| `flatpak` | `/usr/bin/flatpak` | 1.18.4 |
| `pkexec` | `/usr/bin/pkexec` | 127 |

All three are present. No package operations were performed.

### Baseline validation

No `Cargo.toml` exists yet, so Rust validation is not applicable at S01:

- `cargo check --all-targets` → `error: could not find Cargo.toml in ... or any parent directory`
- `cargo fmt --check` → `cargo metadata` failed: could not find `Cargo.toml`
- `cargo test` → not run (same reason: no package)

### Blockers

None for S01. The next task, S02, can create the Cargo package on this host.
