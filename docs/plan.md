# Paket — Incremental Implementation Plan

> **Execution plan for a small-context coding agent.**
>
> Backend first. GUI only after the first backend contract is stable.
>
> **This is not a “build the whole application” prompt.** Every `Sxx` item is one
> independently executable task. The agent must implement exactly one item per run,
> validate it, report the result, and stop. Do not ask the agent to execute an entire
> stage or milestone in one go.

---

## 0. How to execute this plan

### 0.1 The most important rule

**One agent run = one `Sxx` task.**

An OpenSpec change covers a whole stage; an agent run still implements only one unchecked
`Sxx` task from it. Do not loop through the change's remaining tasks.

For example, when assigning `S14`, give the agent only `S14` plus the architecture
sections it needs. Do not ask it to “continue with the next steps” after it finishes.
The human reviews the result, then starts a new run for `S15`.

A task should normally touch no more than:

- one production module or a small pair of directly related modules;
- the matching unit/integration test file;
- the fixture files required by those tests.

If the agent discovers that a task needs a wider refactor, it must stop and report the
blocker. It must not silently expand the task.

### 0.2 Required agent workflow

For every `Sxx` task, the agent must:

1. Read the task's goal, allowed files, implementation notes, tests and DoD from this
   plan, plus the active OpenSpec change for the stage.
2. Inspect the existing code before editing. Do not assume the repository is empty or
   that the previous task was implemented exactly as expected.
3. Implement only the requested scope. Do not implement the next step “while already
   here”.
4. Add or update tests for the behavior changed in this step.
5. Run the exact validation commands listed in the task. Also run `cargo fmt --check`
   and the narrowest relevant `cargo test` target.
6. Fix failures introduced by this task. Do not suppress, skip, weaken or delete tests
   to obtain a green result.
7. Mark only this `Sxx` complete in the change's `tasks.md`. Stop. Report: files changed,
   behavior added, validation output, known limitations, and whether the DoD passed. Do not
   begin another step and do not archive the change.

### 0.3 Standard agent prompt

Copy this prompt and replace `Sxx` with the task being assigned:

```text
Implement exactly one unchecked Sxx task from the active OpenSpec change for <stage>.

Context:
- docs/plan.md is the plan; the change's tasks.md mirrors this stage's Sxx tasks.
- Read the task and its prerequisites before coding.
- Inspect the existing repository and adapt to its actual state.

Rules:
- Implement only Sxx. Do not implement later tasks or opportunistic refactors.
- Keep the diff small. Prefer the simplest implementation that satisfies the DoD.
- Do not add dependencies unless Sxx explicitly requires one. Explain any necessary
  dependency in the final report.
- Never use shell command construction (`sh -c`, `bash -c`, etc.). Use argv arrays.
- Do not invent behavior for a backend. Preserve unsupported/unknown states explicitly.
- Add tests for the behavior in Sxx. Do not modify fixtures to hide parser failures.
- Run the task's validation commands, `cargo fmt --check`, and relevant tests.
- If prerequisites are missing or the task cannot fit its stated scope, stop and report
  the blocker instead of broadening the change.
- Mark only this Sxx complete in the change's tasks.md when the DoD passes.
- Stop after Sxx and provide a concise implementation report. Do not continue to the next
  task and do not archive the change.
```

### 0.4 Task sizing rule

Target **30–120 minutes of focused agent work** per task. This is a scope limit, not a
promise about elapsed time. If a task is likely to exceed that limit, split it before
execution into smaller tasks. Do not merge adjacent tasks just because they are related.

A task is not complete merely because the code compiles. It is complete only when its
DoD passes and the evidence is recorded.

### 0.5 Progress tracking

Progress is tracked only in OpenSpec. Do not create or maintain a hand-written progress
file.

- One OpenSpec change covers one stage or task pack. `AGENTS.md` lists the change names.
- The change's `tasks.md` mirrors that stage's `Sxx` tasks from this plan.
- Check off an `Sxx` task only when its DoD passes.
- When every task in the change is done, sync specs and archive the change with the
  OpenSpec workflow. Gates still control when the next stage may start.
- If `tasks.md` and this plan disagree, this plan wins.

Record factual mismatches between this plan and the actual host/tool output in
`docs/deviations.md`. That file is not a progress tracker.

### 0.6 Commit policy

Do not commit automatically. I will do it myself

### 0.7 Gates

| Gate | Required completed tasks | What becomes possible |
|---|---|---|
| G0 — Project builds | S01–S06 | Shared basic domain and error types |
| G1 — Command layer works | S07–S12 | Safe, testable process execution and host probing |
| G2 — DNF catalog works | S13–S21 | Read installed packages, search and updates on Fedora |
| G3 — DNF mutations work | S22–S34 | Reviewable DNF plans and privileged install/remove |
| G4 — Initial backend set stable | S35–S45 | DNF + Flatpak usable through a stable backend API |
| G5 — UI MVP | S46–S54 | Browse, search, review, install/remove and see progress in Slint |
| G6 — Additional providers | S55 onward | Add APT, Pacman, AUR, Snap and AppImage incrementally |

**Do not start a task whose listed prerequisites are incomplete.**

---

## 1. Product decisions (locked for the initial implementation)

These decisions prevent the agent from inventing architecture while implementing small
tasks. Revisit a decision only through an explicit design change, not as part of an
ordinary implementation step.

| ID | Decision | Reason |
|---|---|---|
| D1 | Rust, one Cargo package, one library and separate binaries | Avoid workspace complexity in the first version. |
| D2 | Fedora Workstation with DNF5 is the first host target | One known host makes backend behavior testable. |
| D3 | Flatpak is the second backend | It adds a distinct package ecosystem without adding another root helper. |
| D4 | Other backends are separate later task groups | APT, Pacman, AUR, Snap and AppImage must not delay the first working vertical slice. |
| D5 | Use official native package-manager CLIs | Do not reimplement dependency resolution or transaction semantics. |
| D6 | No async runtime initially | Blocking process I/O runs on a worker thread; UI remains responsive through channels. |
| D7 | Never invoke a shell | Spawn programs with a fixed executable and argument vector. |
| D8 | Core uses opaque `PkgId`; backend owns native ID parsing | Prevents the UI/core from accidentally targeting a different package. |
| D9 | Capabilities are explicit | Unsupported operations are disabled/reported, never silently treated as successful. |
| D10 | DNF/APT/Pacman privileged writes use a narrow helper | The helper accepts typed operations, never arbitrary commands or argv. |
| D11 | Flatpak and Snap use their own polkit flow | Do not wrap their normal client commands in Paket's root helper. |
| D12 | AUR builds run as the logged-in user, never root | AUR build scripts are untrusted code. |
| D13 | AppImage is a local-file manager, not a repository backend | It has no native package solver or central repository. |
| D14 | Slint + Slint Kit UI comes after G4 | UI must consume a stable backend contract. |
| D15 | Pin Slint `1.19.0` and Slint Kit `0.9.0` when UI work starts | The prior research identified the Kit as pre-1.0 with possible breaking changes. |

### 1.1 Initial supported scope

**First usable slice:** Fedora Workstation (non-Atomic) + DNF5 + Flatpak.

The application must be useful before any optional backend exists. On a Fedora host:

- DNF5 can list installed packages, search available packages, show updates, plan an
  install/remove and apply approved operations.
- Flatpak can list installed applications/runtimes, search, and later install/remove
  using Flatpak's own authorization flow.
- A missing Flatpak installation does not prevent DNF from working.
- A missing DNF5 installation does not prevent the application from starting; the
  provider reports why it is unavailable.

### 1.2 Explicit non-goals for the first usable slice

Do not implement these until their dedicated tasks are reached:

- APT/DEB, Pacman/AUR, Snap or AppImage.
- Application grouping/deduplication with AppStream.
- Repository/source editing, key import or trust management.
- Automatic updates, rollback, snapshots or history restoration.
- A daemon, remote management, multi-user management or plugin ABI.
- A custom dependency solver or cross-ecosystem atomic transaction.
- Settings screens, themes, telemetry, auto-update service or advanced activity UI.

---

## 2. Architecture (small initial shape)

```text
paket (unprivileged desktop app)
  ├── core/       domain types and use cases
  ├── backends/   one adapter per package ecosystem
  ├── exec/       process runner; only place allowed to spawn ordinary commands
  ├── helper/     client-side protocol for privileged helper
  └── ui/         Slint bridge, added after backend gate G4

paket-helper (narrow privileged binary)
  └── accepts typed JSON requests; validates them; constructs fixed native argv
```

### 2.1 Initial process model

- One background worker thread owns the core/backend registry.
- UI thread sends requests over a channel and receives events over another channel.
- Backend commands are serialized. This is intentionally conservative; package managers
  lock shared databases and concurrent transactions are undesirable.
- Read-only commands have a bounded timeout (normally 30 seconds).
- Install/remove commands have no short fixed timeout; the user can cancel them.
- The UI never calls a backend directly and never runs package-manager commands.

### 2.2 Initial crate layout

Do not create every file in this tree at S01. Add files only when their task is reached.

```text
Cargo.toml
build.rs                         # added when Slint work starts
src/
  lib.rs
  main.rs                        # GUI binary, added with UI
  bin/
    helper.rs                    # privileged helper binary
  core/
    mod.rs
    model.rs
    error.rs
    registry.rs
    service.rs
    planner.rs
    executor.rs
  exec/
    mod.rs
    runner.rs
  platform/
    mod.rs
    os_release.rs
  backends/
    mod.rs
    dnf/
      mod.rs
      probe.rs
      parse.rs
      catalog.rs
      plan.rs
      progress.rs
      error.rs
    flatpak/
      mod.rs
      probe.rs
      parse.rs
      catalog.rs
      transaction.rs
  helper/
    mod.rs
    protocol.rs
    client.rs
  ui/
    mod.rs
    bridge.rs
    state.rs
ui/
  app.slint
  views/
  components/
tests/
  fixtures/
  backend_contract.rs
scripts/
  check.sh
  test-fedora.sh
```

### 2.3 Keep interfaces small

Start with ordinary structs and functions. Add a trait only when there are multiple
implementations or when a fake implementation is required for tests.

The intended provider split is:

- `CatalogSource`: read-only operations such as installed/search/details/updates.
- `Installer`: planning and applying package changes.

The process runner may have a small trait because tests must substitute a fake process
runner. Do not add a generic plugin framework, async abstraction, dependency injection
container or event bus.

---

## 3. Domain and process contracts

This section defines the minimum behavior later tasks must preserve. Implement the types
incrementally; do not create all of them in one initial task.

### 3.1 Package identity

A package is identified by `(BackendId, native_id)`, not by display name.

Examples of backend-owned native identity:

- DNF: package name plus architecture and the needed NEVRA discriminator.
- Flatpak: installation scope plus full application/runtime ref.
- APT: package name, plus architecture where multiarch matters.
- Pacman: repository/name for available candidates; package name for installed removal.
- Snap: snap name and instance key where applicable.
- AUR: AUR package name.
- AppImage: canonical path inside Paket's managed directory.

The core treats `native_id` as opaque. A backend must validate and decode it before using
it in a native command. Never use a display name as a mutation target.

### 3.2 Package model: start small

Minimum fields for the DNF catalog:

```rust
pub struct Pkg {
    pub id: PkgId,
    pub name: String,
    pub version: Option<String>,
    pub arch: Option<String>,
    pub summary: Option<String>,
    pub source: Option<String>,
    pub installed: bool,
}
```

Add fields such as icon, AppStream ID, license, homepage, installed size and package kind
only in the task that first needs them. Unknown values stay `None`; never invent metadata.

### 3.3 Backend availability

A provider probe returns a descriptor, not a panic:

```rust
pub struct BackendInfo {
    pub id: BackendId,
    pub label: String,
    pub available: bool,
    pub version: Option<String>,
    pub reason_unavailable: Option<String>,
    pub capabilities: Vec<Capability>,
}
```

A provider may be unavailable because its command is missing, its daemon is absent or
the host is unsupported. Preserve a human-readable reason.

### 3.4 Plans

A plan is not just the user's selected package list. It must represent what the native
solver intends to change, including dependencies and removals when available.

Each plan step records:

- backend and action;
- exact selected target IDs;
- native-resolved changes where parseable;
- preview quality: `Native`, `Estimated` or `Unavailable`;
- warning/notes where the native tool cannot provide a trustworthy preview.

Never claim cross-backend atomicity. Apply backend steps sequentially and report partial
success/failure accurately.

### 3.5 Execution safety

- Only `exec/runner.rs` may use `std::process::Command` for ordinary application commands.
- `paket-helper` has its own narrowly-scoped process-spawn code.
- Always set `LC_ALL=C` for output parsers.
- Cap captured stdout/stderr to avoid unbounded memory use.
- Never use `sh -c`, `bash -c`, interpolated shell strings or arbitrary UI-provided argv.
- Validate native package IDs in the backend and validate again in the privileged helper.
- Never disable signature verification or auto-import a repository key.

---

# 4. Implementation tasks

## Stage A — Project foundation

**Gate G0.** Complete S01–S06. The project builds and has the smallest useful
domain model. No package-manager commands yet.

### S01 — Inspect repository and record baseline

**Goal:** establish the actual starting state without changing implementation code.

**Allowed changes:** `docs/baseline.md` only.

**Implementation:**

1. Inspect the repository tree, Cargo manifest, Rust toolchain, existing tests and CI.
2. Identify whether the repository already contains code that overlaps this plan.
3. Record current `cargo test`, `cargo check` and `cargo fmt --check` results where
   applicable. Do not “fix everything” during reconnaissance.
4. Record OS/distribution details from `/etc/os-release` and whether `dnf5`, `flatpak`
   and `pkexec` are present. Do not perform package operations.

**Validation:** run baseline commands without modifying source.

**DoD:** baseline file contains repository layout, toolchain version, host distro, current
build/test status and blockers. No production files changed.

**Do not:** scaffold a new project if the repository already has one; change dependencies;
start S02 in the same run.

### S02 — Create the minimal Cargo package

**Prerequisite:** S01.

**Goal:** create a compiling Rust library package and helper binary target.

**Allowed files:** `Cargo.toml`, `src/lib.rs`, `src/bin/helper.rs` and `.gitignore` if needed.

**Implementation:**

1. Use one Cargo package named `paket`, Rust edition 2024 and the declared minimum
   Rust version from the current toolchain decision.
2. Add a library target at `src/lib.rs`.
3. Add a `paket-helper` binary target with a placeholder `main` that exits successfully.
4. Do not add Slint dependencies yet.
5. Add only dependencies required by the next foundational tasks: `serde` with derive,
   `serde_json` and `thiserror`. If the repository already has valid equivalents, reuse them.

**Validation:** `cargo check --all-targets`; `cargo test`.

**DoD:** both targets compile; no UI dependency is pulled in; clean diff limited to the
allowed files.

**Do not:** create all planned module directories or add unused dependencies.

### S03 — Add backend and package identifiers

**Prerequisite:** S02.

**Goal:** define stable identities without implementing package behavior.

**Allowed files:** `src/core/mod.rs`, `src/core/model.rs`, `src/lib.rs`, matching unit tests.

**Implementation:**

1. Add `BackendId` variants for `Dnf5`, `Flatpak`, `Apt`, `Pacman`, `Aur`, `Snap` and
   `AppImage`.
2. Add `PkgId { backend: BackendId, native: String }`.
3. Implement `BackendId::as_str()` and `PkgId::key()` for display/list-model keys only.
4. Derive the traits needed for equality, hashing, cloning and serialization.
5. Do not parse or normalize `native` in core.

**Validation:** unit tests for stable string forms, equality and distinct IDs with the
same display name but different backend/native values.

**DoD:** identifiers compile and tests prove that `dnf5:foo` and `flatpak:foo` remain
separate identities.

**Do not:** add package fields, provider traits, registry or native command logic.

### S04 — Add the minimal package model

**Prerequisite:** S03.

**Goal:** represent a package row returned by a catalog provider.

**Allowed files:** `src/core/model.rs`, model unit tests.

**Implementation:**

1. Add the minimum `Pkg` fields listed in §3.2.
2. Use `Option` for metadata that may be missing.
3. Do not add cross-backend version comparison or sorting logic.
4. Ensure a `Pkg` contains a `PkgId`; do not use package name as identity.

**Validation:** tests for serialization/deserialization and a package with missing optional
metadata.

**DoD:** package model compiles and round-trips through JSON without losing its ID or
optional fields.

**Do not:** add `PkgDetail`, `Update`, icons, categories or AppStream enrichment yet.

### S05 — Add backend availability and capability types

**Prerequisite:** S03.

**Goal:** make “backend unavailable” and “operation unsupported” explicit.

**Allowed files:** `src/core/model.rs`, model unit tests.

**Implementation:**

1. Add a minimal `Capability` enum for `Search`, `Installed`, `Details`, `Updates`,
   `Install` and `Remove`.
2. Add `BackendInfo` with ID, label, availability, optional version, optional unavailable
   reason and capabilities.
3. Do not make assumptions that every backend implements every capability.
4. Avoid boolean flags for every possible future feature; add capabilities only as needed.

**Validation:** tests for available and unavailable descriptors.

**DoD:** a missing CLI can be represented as an unavailable backend with a reason and an
empty/limited capability list.

**Do not:** implement probing or registry discovery yet.

### S06 — Add a small typed error model

**Prerequisite:** S02.

**Goal:** distinguish expected failures without leaking raw process errors everywhere.

**Allowed files:** `src/core/error.rs`, `src/core/mod.rs`, matching tests.

**Implementation:**

1. Add `CatalogError` variants for `Unavailable`, `Command`, `Parse` and `NotFound`.
2. Add `OperationError` variants for `Unsupported`, `Busy`, `AuthDenied`, `Network`,
   `Dependency`, `Signature`, `NoSpace`, `Cancelled` and a generic native failure.
3. Keep raw stderr as diagnostic context, not as the enum's only meaning.
4. Implement user-readable `Display` text with `thiserror`.

**Validation:** unit tests for representative messages and variant matching.

**DoD:** core modules can return typed errors; no `anyhow` dependency is introduced.

**Do not:** implement backend-specific string classification in this step.

---

## Stage B — Safe command execution and host detection

**Gate G1.** Complete S07–S12. The app can run commands safely and test them
without invoking real package managers.

### S07 — Define command input/output types

**Prerequisite:** S02, S06.

**Goal:** describe a process invocation as structured data.

**Allowed files:** `src/exec/mod.rs`, `src/exec/runner.rs`.

**Implementation:**

1. Add `RunSpec` with executable, argv, environment overrides, optional working directory,
   timeout and output-capture mode.
2. Keep executable separate from arguments; do not accept a shell command string.
3. Add `RunOutput` with exit code, stdout, bounded stderr tail and duration.
4. Add `RunError` for spawn failure, timeout, cancellation and output-limit failure.
5. Do not implement process spawning yet.

**Validation:** unit tests for construction defaults and output-limit behavior where
possible.

**DoD:** types compile and no field represents arbitrary shell text.

**Do not:** add `tokio`, `async` methods or event-bus abstractions.

### S08 — Define the testable runner port

**Prerequisite:** S07.

**Goal:** let backends run commands without depending on `std::process` directly.

**Allowed files:** `src/exec/runner.rs` and `src/exec/mod.rs`.

**Implementation:**

1. Define a small blocking `Runner` trait with one method that accepts `RunSpec` and
   returns `RunOutput`/`RunError`.
2. Keep the method synchronous.
3. Do not expose `std::process::Child` to backend modules.
4. Document that only the production runner is allowed to spawn a process.

**Validation:** a tiny compile-time test or test stub implementing `Runner`.

**DoD:** backend code can depend on `dyn Runner` without knowing how processes are run.

**Do not:** add streaming progress callbacks yet; that is a later task.

### S09 — Implement the real process runner

**Prerequisite:** S08.

**Goal:** execute one process safely and capture bounded output.

**Allowed files:** `src/exec/runner.rs`, runner unit tests.

**Implementation:**

1. Use `Command::new(executable).args(argv)`; never invoke a shell.
2. Add the requested environment overrides and always set `LC_ALL=C` unless a task
   explicitly documents why a backend needs a different locale.
3. Drain stdout and stderr concurrently so neither pipe can block the child.
4. Bound captured stdout (for example, 32 MiB) and retain only a bounded stderr tail
   (for example, the last 200 lines); if a configured hard output limit is exceeded, stop
   the child and return `OutputLimit`.
5. Convert output with `String::from_utf8_lossy`.
6. Return non-zero exit codes as output, not as a spawn error; callers classify them.
7. Implement timeout with a bounded terminate/kill sequence. Do not fake successful
   cancellation if the child could not be stopped.

**Validation:** tests using a known harmless executable (`printf`/`echo` where available)
plus a test for non-zero exit status and timeout.

**DoD:** runner returns correct stdout, stderr, exit code and duration; output remains
bounded; static scan confirms no shell invocation.

**Do not:** run real package-manager operations.

### S10 — Add a fake runner for backend tests

**Prerequisite:** S08.

**Goal:** test backend command construction and parsing without touching the host.

**Allowed files:** `src/exec/fake.rs` or `tests/support/fake_runner.rs`, test support only.

**Implementation:**

1. Fake runner stores expected executable/argv patterns and queued outputs.
2. A mismatch should fail loudly with a useful assertion message.
3. Allow tests to assert the order and number of invocations.
4. Keep the fake out of production builds unless it is harmless and feature-gated.

**Validation:** a test proves the fake returns a queued output and detects an unexpected
command.

**DoD:** a backend unit test can run with no DNF/Flatpak installed and no root privileges.

**Do not:** create a generic mocking framework.

### S11 — Parse `/etc/os-release`

**Prerequisite:** S02.

**Goal:** detect the host distribution using a simple, testable parser.

**Allowed files:** `src/platform/mod.rs`, `src/platform/os_release.rs`, fixtures/tests.

**Implementation:**

1. Parse key/value lines; ignore comments and blank lines.
2. Support quoted and unquoted values according to common os-release syntax; do not use
   shell `source` or execute the file.
3. Extract `ID`, `ID_LIKE`, `VERSION_ID` and `PRETTY_NAME`.
4. Unknown or missing values remain optional.
5. Add fixtures for Fedora, Ubuntu/Debian, Arch and an unknown distribution.

**Validation:** parser unit tests for all fixtures and malformed lines.

**DoD:** parser never panics on malformed input and never executes `/etc/os-release`.

**Do not:** decide backend availability in this task.

### S12 — Add backend registry skeleton

**Prerequisite:** S05, S10, S11.

**Goal:** hold backend descriptors and expose a stable list to the rest of the app.

**Allowed files:** `src/core/registry.rs`, `src/core/mod.rs`, tests.

**Implementation:**

1. Add a registry that stores backend descriptors in a deterministic order.
2. Provide lookup by `BackendId` and iteration over descriptors.
3. Allow unavailable descriptors to remain in the registry with a reason.
4. Do not create a dynamic plugin loader.
5. Do not instantiate DNF/Flatpak providers until their specific tasks.

**Validation:** tests for deterministic ordering, lookup and unavailable entries.

**DoD:** registry behavior is independent of any installed package manager.

**Do not:** add all backend modules or feature flags now.

---

## Stage C — DNF5 read-only catalog

**Gate G2.** Complete S13–S21. On a Fedora Workstation host, installed
packages, search results and updates are available through typed Rust models. No writes.

### S13 — Probe DNF5 availability

**Prerequisite:** S09, S11, S12.

**Goal:** determine whether the current host can use the DNF5 provider.

**Allowed files:** `src/backends/mod.rs`, `src/backends/dnf/mod.rs`,
`src/backends/dnf/probe.rs`, tests.

**Implementation:**

1. Confirm the host is a supported traditional Fedora/RHEL-family system using parsed
   `os-release` data. Do not enable DNF5 on Fedora Atomic desktops.
2. Probe the `dnf5` executable with a bounded version command.
3. Return `BackendInfo` whether available or unavailable.
4. Include a clear reason for unsupported distro, missing executable or failed probe.
5. Do not infer support solely from the existence of `rpm`.

**Validation:** fake-runner tests for supported Fedora, unsupported distro, missing DNF5
and failed version command. Run a read-only probe on a Fedora host if available.

**DoD:** probing never panics and does not run a package transaction.

**Do not:** implement catalog commands yet.

### S14 — Parse DNF5 installed-package JSON

**Prerequisite:** S04, S13.

**Goal:** convert one captured installed-package JSON response into `Vec<Pkg>`.

**Allowed files:** `src/backends/dnf/parse.rs`, `tests/fixtures/dnf/installed.json`, tests.

**Implementation:**

1. Use a real sample captured from the supported DNF5 version as the fixture.
2. Map only fields needed by the current `Pkg` model.
3. Construct backend-owned `PkgId` values that distinguish package architecture/version
   where required for safe targeting.
4. Treat absent or unknown fields as `None`.
5. Return `CatalogError::Parse` with useful context for invalid JSON or missing required
   identity fields.

**Validation:** fixture test, malformed JSON test and missing-optional-field test.

**DoD:** parser is a pure function with no process execution and no filesystem access.

**Do not:** invent a JSON schema from memory; capture actual output first.

### S15 — Implement DNF installed-package listing

**Prerequisite:** S14.

**Goal:** list installed RPM packages using DNF5's machine-readable output.

**Allowed files:** `src/backends/dnf/catalog.rs`, `src/backends/dnf/mod.rs`, tests.

**Implementation:**

1. Build the exact installed-list argv using DNF5's JSON mode supported by the researched
   DNF version (`dnf5 list --installed --json`).
2. Execute through `Runner` with `LC_ALL=C` and a bounded timeout/output cap.
3. Reject non-zero command exit as a catalog command error.
4. Pass stdout to the pure parser from S14.
5. Do not run as root for read-only listing.

**Validation:** fake-runner test asserts exact argv and parsed result. Manual Fedora check
compares a sample of results with `rpm -qa`.

**DoD:** installed package list returns typed `Pkg` values and does not require privilege.

**Do not:** add search/details/updates in this task.

### S16 — Parse DNF5 repoquery output

**Prerequisite:** S14.

**Goal:** parse DNF search results from the DNF5 `repoquery --queryformat` output into the same `Pkg` model.

**Allowed files:** `src/backends/dnf/parse.rs`, search fixtures and tests.

**Implementation:**

1. Do not assume `repoquery` supports `--json`; the researched interface exposes `--queryformat`.
2. Verify the exact DNF5 `repoquery` queryformat syntax against the installed DNF5 help/man page and a real command before capturing the fixture.
3. Start with the smallest stable field set: name, EVR/version, architecture and repository ID. Add summary only if the chosen output format can delimit it safely.
4. Parse the chosen delimiter format with strict field-count validation.
5. Mark results as not installed unless the source explicitly says otherwise.
6. Preserve multiple package candidates when their native IDs differ.
7. Add empty-result and malformed-record fixtures.

**Validation:** parser-only fixture tests.

**DoD:** search parser is deterministic and does not deduplicate by display name.

**Do not:** run the DNF search command in this step.

### S17 — Implement DNF search

**Prerequisite:** S16.

**Goal:** run DNF search from a user query and parse its results.

**Allowed files:** `src/backends/dnf/catalog.rs`, tests.

**Implementation:**

1. Construct argv as separate arguments; pass the query as one argument.
2. Use the `dnf5 repoquery` command and queryformat verified in S16; do not append unverified flags.
3. Execute without root, with timeout and output bounds.
4. Return an empty vector for a valid no-result response.
5. Return a typed error for command failure or malformed output.

**Validation:** fake-runner checks executable/argv and no-result behavior. Manual query on
Fedora for a common package such as `htop`.

**DoD:** query text never becomes shell syntax and results preserve DNF native identity.

**Do not:** add network catalogue indexing or global cross-backend deduplication.

### S18 — Add DNF package details

**Prerequisite:** S15, S17.

**Goal:** retrieve basic details for one selected DNF package.

**Allowed files:** `src/core/model.rs`, `src/backends/dnf/catalog.rs`, DNF parser/tests.

**Implementation:**

1. Add `PkgDetail` with only fields actually returned by the chosen DNF5 info/repoquery
   command: package row, description and optional homepage/license/size if present.
2. Validate `PkgId.backend == BackendId::Dnf5` before decoding its native ID.
3. Build the native query from validated decoded fields, not from the display name.
4. Return `NotFound` for a package that no longer exists.

**Validation:** fake-runner test for valid ID, wrong backend ID and not-found output.

**DoD:** details cannot accidentally query a Flatpak ID or interpolate arbitrary UI text.

**Do not:** fetch screenshots, ratings, user reviews or AppStream data.

### S19 — Parse DNF5 update JSON

**Prerequisite:** S04.

**Goal:** parse the native update list into a small `Update` model.

**Allowed files:** `src/core/model.rs`, `src/backends/dnf/parse.rs`, update fixture/tests.

**Implementation:**

1. Add the minimal `Update` model: package ID, name, current version and candidate version.
2. Capture real `dnf5 list --upgrades --json` output from the supported version.
3. Map missing version/source fields to `None`.
4. Do not compare DNF versions with other ecosystems.

**Validation:** parser tests for normal, empty and malformed responses.

**DoD:** updates are parsed without invoking a command and preserve package identity.

**Do not:** calculate security update status unless the source explicitly supplies it.

### S20 — Implement DNF update listing

**Prerequisite:** S19.

**Goal:** expose available DNF upgrades through the backend catalog.

**Allowed files:** `src/backends/dnf/catalog.rs`, tests.

**Implementation:**

1. Run DNF5's JSON upgrade-list command unprivileged.
2. Use a bounded timeout and output cap.
3. Parse with the function from S19.
4. Treat unavailable metadata as empty/unknown, not as “up to date” unless the command
   successfully reports an empty result.

**Validation:** fake-runner tests for command failure, empty output and one update.

**DoD:** provider can distinguish successful zero updates from a failed update query.

**Do not:** install updates yet.

### S21 — Add DNF read-contract tests

**Prerequisite:** S15, S17, S18, S20.

**Goal:** verify the read-only contract without requiring Fedora or root.

**Allowed files:** `tests/dnf_catalog.rs`, test fixtures and support only.

**Implementation:**

1. Test installed, search, details and updates through the public DNF adapter API.
2. Use fake runner responses; do not mock the parser directly in every test.
3. Assert every command is non-privileged and has `LC_ALL=C`.
4. Assert a backend command/parse failure does not terminate the test process.

**Validation:** `cargo test dnf_catalog` on a machine without DNF installed.

**DoD:** DNF read operations have an end-to-end fake-runner test suite.

**Do not:** start DNF mutations until G2 passes.

---

## Stage D — DNF planning and privileged mutations

**Gate G3.** Complete S22–S34. This is the first privileged feature. Do not
combine it with Flatpak or the GUI.

### S22 — Add the minimal transaction model

**Prerequisite:** G2.

**Goal:** represent a selected action separately from the native solver's expected changes.

**Allowed files:** `src/core/model.rs`, model tests.

**Implementation:**

1. Add `Action` (`Install`, `Remove`, `Upgrade`), `ChangeKind`, `PlanPreview`,
   `PlannedChange`, `PlanStep` and `Plan`.
2. A `PlanStep` records backend, action, selected targets, resolved changes, preview quality
   and user-visible notes.
3. Keep the model backend-neutral, but do not pretend every backend can provide the same
   preview completeness.
4. Use a simple unique plan ID generated by core; do not add a UUID dependency yet.

**Validation:** serialization and equality tests for a plan with a dependency change and
one with `PlanPreview::Unavailable`.

**DoD:** a plan can represent both user-selected targets and additional native-resolved
changes.

**Do not:** implement planning or execution logic.

### S23 — Build DNF dry-run commands

**Prerequisite:** S22.

**Goal:** safely construct DNF5 simulation argv for install and remove.

**Allowed files:** `src/backends/dnf/plan.rs`, tests.

**Implementation:**

1. Accept only DNF `PkgId`s; reject IDs from other backends.
2. Decode and validate each native ID using DNF's identifier rules.
3. Build a DNF5 dry-run/assume-no command for install or remove using the syntax verified
   against the supported DNF version.
4. Do not add `-y` behavior that causes a real transaction; simulation must remain a dry run.
5. Keep install and remove command construction in separate small functions.

**Validation:** unit tests assert exact argv and reject malformed IDs, empty targets and
wrong-backend IDs.

**DoD:** the builder cannot turn an arbitrary package display string into command options.

**Do not:** execute the command or parse output yet.

### S24 — Parse DNF dry-run transaction output

**Prerequisite:** S23.

**Goal:** turn the native simulation result into `PlanStep` changes.

**Allowed files:** `src/backends/dnf/plan.rs`, parser fixtures/tests.

**Implementation:**

1. Capture actual dry-run output for install and remove from the supported DNF5 version.
2. Parse the package changes DNF says it would perform, including dependency installs,
   upgrades and removals where the output exposes them.
3. Mark the preview `Native` only when the complete transaction set is parsed.
4. If the output format cannot be parsed completely, fail planning or mark preview
   `Unavailable`; never fabricate a partial list as complete.
5. Include download/install sizes only when native output provides reliable values.

**Validation:** fixtures for install, remove, no-op, dependency changes and malformed
output.

**DoD:** the parser is pure, fixture-backed and honest about preview completeness.

**Do not:** run the real mutation command.

### S25 — Define the helper request/response protocol

**Prerequisite:** S22.

**Goal:** define the smallest typed protocol for requesting DNF install/remove.

**Allowed files:** `src/helper/mod.rs`, `src/helper/protocol.rs`, tests.

**Implementation:**

1. Define request variants for DNF install and DNF remove only.
2. Request contains an operation enum and a list of validated package identifiers; it does
   not contain an executable path, arbitrary argv or environment overrides.
3. Define JSONL response events for started, output/progress text and completed/failed.
4. Add a protocol version field so incompatible client/helper binaries fail closed.
5. Keep the types serde-compatible and small.

**Validation:** JSON round-trip tests and tests that unknown operation/version is rejected.

**DoD:** protocol cannot express arbitrary process execution.

**Do not:** implement helper process spawning or support APT/Pacman yet.

### S26 — Make the helper binary read one request

**Prerequisite:** S02, S25.

**Goal:** read and parse one JSON request from stdin in the helper binary.

**Allowed files:** `src/bin/helper.rs`, helper protocol tests.

**Implementation:**

1. Read one bounded line from stdin.
2. Deserialize into the typed request.
3. Reject oversized, malformed or unsupported protocol input with a non-zero exit.
4. Ensure stdout is reserved for protocol JSONL; diagnostic messages go to stderr.
5. Do not execute any command in this task.

**Validation:** tests or a small subprocess test for valid JSON, malformed JSON and oversized
input.

**DoD:** helper accepts exactly one typed request and does nothing beyond validation/parsing.

**Do not:** add `pkexec` invocation or root checks yet.

### S27 — Validate DNF helper requests and construct fixed argv

**Prerequisite:** S26.

**Goal:** ensure helper builds only approved DNF commands.

**Allowed files:** `src/bin/helper.rs` or `src/helper/validate.rs`, tests.

**Implementation:**

1. Check effective UID is root before performing privileged work.
2. Accept only the DNF install/remove operation variants from S25.
3. Validate each package identifier against a conservative DNF package-ID grammar.
4. Reject empty lists, excessive list sizes, leading option-like values and malformed IDs.
5. Construct the executable and argv from a static match on the operation. Never accept
   argv from stdin.
6. Set `LC_ALL=C` and safe stdin behavior.

**Validation:** unit tests for valid IDs, option injection attempts, path-like strings,
unknown operation and non-root invocation.

**DoD:** helper can produce only the two explicitly supported DNF command forms.

**Do not:** support arbitrary backend commands or repository/key management.

### S28 — Run the DNF child process in the helper

**Prerequisite:** S27.

**Goal:** execute the validated command and return a bounded JSONL result stream.

**Allowed files:** `src/bin/helper.rs` or a small helper execution module; tests.

**Implementation:**

1. Spawn the fixed DNF executable with the argv constructed in S27.
2. Drain stdout and stderr so the child cannot block on a full pipe.
3. Emit protocol events only on helper stdout; retain the last bounded stderr lines for
   diagnostics.
4. Return the real child exit code in the completion response.
5. For this first version, it is acceptable to emit only `Started` and final `Completed`/
   `Failed` events. Do not parse progress text yet. Never output raw unframed logs on stdout.

**Validation:** test with a harmless fake executable if the helper execution is structured
for injection; otherwise isolate process spawning behind a tiny testable function.

**DoD:** helper returns a parseable completion event for success and failure and never
accepts a client-supplied executable.

**Do not:** add cancellation or progress parsing in this step.

### S29 — Implement the unprivileged helper client

**Prerequisite:** S25, S28.

**Goal:** let the main application invoke the installed helper through `pkexec`.

**Allowed files:** `src/helper/client.rs`, `src/helper/mod.rs`, tests.

**Implementation:**

1. Serialize the typed request as JSONL on stdin.
2. Invoke the fixed helper path through `pkexec`; do not allow the UI to choose a helper
   path or pass arbitrary helper arguments.
3. Parse only valid protocol events from stdout.
4. Map pkexec authentication cancellation/refusal to `OperationError::AuthDenied`.
5. Map missing helper and malformed protocol to explicit errors.

**Validation:** unit tests for JSON encoding/decoding and fake process outputs. Do not
require a real polkit prompt in CI.

**DoD:** caller can request one typed DNF operation and receive a typed result.

**Do not:** call DNF directly as root from the main GUI process.

### S30 — Implement DNF install planning end to end

**Prerequisite:** S23, S24.

**Goal:** expose `plan_install(PkgId)` using the dry-run builder and parser.

**Allowed files:** `src/backends/dnf/plan.rs`, DNF tests.

**Implementation:**

1. Validate selected IDs.
2. Run the DNF simulation command using the ordinary unprivileged runner.
3. Parse the full transaction result.
4. Return a `PlanStep` containing selected targets and every known native change.
5. If simulation fails, do not return an apparently valid empty plan.

**Validation:** fake-runner end-to-end tests for successful plan, empty/no-op plan and
native command failure.

**DoD:** install planning is read-only and returns a truthful preview.

**Do not:** call the helper or execute an install.

### S31 — Implement DNF remove planning end to end

**Prerequisite:** S30.

**Goal:** expose `plan_remove(PkgId)` with dependency-removal visibility.

**Allowed files:** `src/backends/dnf/plan.rs`, tests.

**Implementation:**

1. Validate selected installed package IDs.
2. Run DNF's remove simulation.
3. Parse packages DNF proposes to remove, including dependent packages if the native
   output includes them.
4. A plan with additional removals must list them; do not collapse them to one summary.
5. If parsing is incomplete, mark the preview unavailable or fail closed.

**Validation:** fixture tests for a simple removal, dependent removals and package-not-found.

**DoD:** remove review can show the exact native removal set whenever DNF exposes it.

**Do not:** add a force-remove option.

### S32 — Apply an approved DNF install

**Prerequisite:** S27, S28, S29, S30.

**Goal:** execute install only after the user confirms a previously generated plan.

**Allowed files:** `src/backends/dnf/transaction.rs` (or existing DNF module), helper client
and targeted tests.

**Implementation:**

1. Keep the approved plan in the unprivileged process.
2. On confirmation, extract the selected target IDs and send only those IDs to the helper.
3. Helper revalidates the IDs and constructs `dnf5 install -y ...` (exact argv confirmed
   against the supported DNF version).
4. Do not pass a previously generated arbitrary argv from the plan to the helper.
5. After success, refresh installed packages through the read-only catalog path.

**Validation:** fake helper-client test; manual install in a disposable Fedora VM using a
small known package; verify installed package appears in the catalog.

**DoD:** install requires an explicit approved plan and helper validation; the GUI process
never performs a privileged DNF write.

**Do not:** implement remove apply or batch/cross-backend transactions.

### S33 — Apply an approved DNF removal

**Prerequisite:** S31, S32.

**Goal:** execute removal only after reviewing all native-proposed removals.

**Allowed files:** DNF transaction module and targeted tests.

**Implementation:**

1. Require the confirmed plan to match the pending plan ID and selected target IDs.
2. Send the primary selected targets to the helper; do not allow the UI to add extra targets
   that were not in the plan.
3. The helper revalidates the identifiers and constructs the fixed DNF remove argv.
4. DNF remains authoritative for dependency protection; do not add `--nodeps` or other
   force flags.
5. Refresh installed package state after success.

**Validation:** fake tests for stale plan ID and mismatched target; manual remove of a
reinstallable test package in a disposable VM.

**DoD:** removal cannot bypass DNF dependency checks and cannot execute a stale/unreviewed
plan.

**Do not:** implement update-all or rollback.

### S34 — Add DNF transaction errors and integration smoke test

**Prerequisite:** S32, S33.

**Goal:** make common native failures understandable and verify the real DNF vertical slice.

**Allowed files:** `src/backends/dnf/error.rs`, targeted tests, `scripts/test-fedora.sh`,
`docs/deviations.md` if needed.

**Implementation:**

1. Classify only well-supported signatures: lock/busy, dependency conflict, signature/key
   issue, insufficient space, authentication cancelled and generic native failure.
2. Keep classification in one function; do not scatter stderr matching across the backend.
3. Add an integration script that checks DNF probe, installed list, search, plan install and
   plan remove. A real install/remove test must require an explicit opt-in environment flag
   and run only in a disposable VM.
4. Record tested Fedora/DNF versions.

**Validation:** `cargo test`; `cargo clippy --all-targets -- -D warnings`; read-only script
on Fedora; opt-in transaction test only in disposable VM.

**DoD:** G3 passes; expected failures produce typed errors; no test unexpectedly mutates the
host package database.

**Do not:** add Flatpak until G3 is reviewed.

---

## Stage E — Flatpak catalog and operations

**Goal:** add Flatpak as the second provider while keeping its distinct installation scope
and authorization behavior visible.

### S35 — Probe Flatpak availability and installation scope

**Prerequisite:** G3.

**Goal:** detect the Flatpak CLI and whether user/system installations exist.

**Allowed files:** `src/backends/flatpak/mod.rs`, `probe.rs`, tests.

**Implementation:**

1. Probe `flatpak --version` with the normal runner.
2. Determine available user/system installation scopes using supported read-only CLI output.
3. Treat missing CLI/daemon as unavailable, not fatal to application startup.
4. Return `BackendInfo` with only currently supported capabilities.

**Validation:** fake-runner tests for missing CLI, successful CLI and failed probe.

**DoD:** Flatpak detection works independently of the host distro and does not require root.

**Do not:** implement list/search or assume Flathub is configured.

### S36 — Parse installed Flatpak references

**Prerequisite:** S35, S03.

**Goal:** parse installed applications and runtimes while preserving scope.

**Allowed files:** `src/backends/flatpak/parse.rs`, fixtures/tests.

**Implementation:**

1. Capture actual `flatpak list` output using the selected machine-readable format.
2. Parse app/runtime kind, application ID, branch, architecture, origin and installation
   scope where output exposes them.
3. Encode installation scope plus full ref into `PkgId.native`.
4. Do not assume two refs with the same application ID but different installations are the
   same package.

**Validation:** parser fixtures containing user and system installations, app and runtime,
empty list and malformed output.

**DoD:** parser is pure and preserves scope identity.

**Do not:** deduplicate Flatpak refs with DNF packages.

### S37 — Implement Flatpak installed listing

**Prerequisite:** S36.

**Goal:** expose installed Flatpak packages through the catalog adapter.

**Allowed files:** `src/backends/flatpak/catalog.rs`, tests.

**Implementation:**

1. Run the selected `flatpak list` machine-readable format.
2. Parse output through S36.
3. Return both user and system packages if both scopes are present.
4. Do not run the command through Paket's root helper.

**Validation:** fake-runner test asserts exact argv and scope preservation.

**DoD:** installed Flatpaks appear as individual provider rows.

**Do not:** add search/details/update operations yet.

### S38 — Implement Flatpak search

**Prerequisite:** S37.

**Goal:** search configured Flatpak remotes without making assumptions about Flathub.

**Allowed files:** `src/backends/flatpak/catalog.rs`, parser/tests.

**Implementation:**

1. Capture output from the chosen `flatpak search` machine-readable form.
2. Map app ID, branch, remote and available metadata.
3. Preserve remote/ref identity in `PkgId`.
4. Empty results are valid; remote/network errors are not empty results.

**Validation:** fixtures for results from two remotes, no result and failed remote query.

**DoD:** search returns provider-specific candidates and does not claim Flatpak is configured
when no remotes exist.

**Do not:** add a remote automatically or change trust settings.

### S39 — Add Flatpak update listing and details

**Prerequisite:** S37, S38.

**Goal:** expose basic details and available updates.

**Allowed files:** `src/core/model.rs`, Flatpak catalog/parser/tests.

**Implementation:**

1. Add only detail/update fields required by current UI/API needs.
2. Use Flatpak's supported CLI output for installed refs and updates.
3. Preserve user/system scope on update IDs.
4. If the native output cannot classify a security update or download size, leave it unknown.

**Validation:** fixtures for no updates, one update and missing optional metadata.

**DoD:** updates can be listed without comparing versions across ecosystems.

**Do not:** implement automatic background refresh.

### S40 — Implement Flatpak install/remove

**Prerequisite:** S38, S39.

**Goal:** create an explicitly confirmable Flatpak action and install/remove a selected ref
using Flatpak's own authorization mechanism.

**Allowed files:** Flatpak transaction module, targeted tests.

**Implementation:**

1. Validate that the selected `PkgId` belongs to Flatpak and decode its full ref/scope.
2. Construct a fixed Flatpak command from the validated ID.
3. Respect user/system scope; do not silently switch scope.
4. Invoke `flatpak install`/`uninstall` directly as the user. Allow Flatpak/polkit to request
   authorization; do not run Flatpak inside `pkexec`.
5. If plan preview uses a command that downloads objects into cache (for example a
   no-deploy style preview), disclose that side effect and do not run it in the background.
6. Build a `PlanStep` for the selected Flatpak ref. If no reliable native preview is
   available, set `PlanPreview::Unavailable`, disclose that fact and still require the user
   to review/confirm the exact selected ref and scope.

**Validation:** fake-runner tests for scope, argv and invalid IDs; manual operation in a
Fedora VM with Flatpak configured.

**DoD:** Flatpak operations use native authorization and cannot target a different scope
than the one selected.

**Do not:** add/remove remotes or auto-accept signing keys.

### S41 — Add Flatpak provider contract tests

**Prerequisite:** S35–S40.

**Goal:** verify the provider contract without requiring Flatpak to be installed.

**Allowed files:** `tests/flatpak_catalog.rs`, fixtures/support.

**Implementation:**

1. Test probe, installed, search, details/update behavior and install/remove command
   construction using the fake runner.
2. Assert that Flatpak commands never use the Paket privileged helper.
3. Assert scope is preserved in every relevant `PkgId` round-trip.

**Validation:** `cargo test flatpak_catalog` on any Linux host.

**DoD:** G4 provider tests pass; missing Flatpak does not break DNF.

**Do not:** start the UI before S42–S45 stabilize the public core API.

---

## Stage F — Core service and backend contract stabilization

**Gate G4.** Complete S42–S45. This stage joins the two providers behind a
small core interface. Keep it smaller than a framework.

### S42 — Define the provider traits

**Prerequisite:** S21, S41.

**Goal:** define only the common provider operations used by the first UI.

**Allowed files:** `src/core/traits.rs`, `src/core/mod.rs`, implementations/tests as needed.

**Implementation:**

1. Add `CatalogSource` for backend ID/info, installed list, search and details.
2. Add `Installer` for plan and apply operations. Flatpak may report a less-complete plan
   than DNF, but it must never fake a native preview.
3. Keep capability differences in `BackendInfo`; do not create default no-op success
   methods.
4. Use the fake runner in tests; do not introduce a dependency-injection crate.

**Validation:** compile both provider implementations through the traits.

**DoD:** DNF and Flatpak can be called through the same catalog interface while preserving
provider-specific IDs and errors.

**Do not:** add methods for future features that no current backend needs.

### S43 — Add a small core catalog service

**Prerequisite:** S42.

**Goal:** combine read-only results from available providers.

**Allowed files:** `src/core/service.rs`, tests.

**Implementation:**

1. Add `installed_all()` that queries providers in deterministic order.
2. Add `search_all(query)` that queries each provider and returns results grouped by
   backend/provider without merging by display name.
3. Preserve per-provider errors so one broken backend does not hide successful results
   from another provider.
4. Do not add background refresh, cache or worker threads in this task.

**Validation:** tests where DNF succeeds and Flatpak fails, and the reverse; both result
sets remain available with a provider-specific error.

**DoD:** one provider failure does not fail the entire aggregate catalog.

**Do not:** deduplicate app identities or create a cross-provider version comparator.

### S44 — Add install/remove plan aggregation

**Prerequisite:** S22, S30, S31, S40, S42.

**Goal:** create a plan from one or more selected package IDs.

**Allowed files:** `src/core/planner.rs`, tests.

**Implementation:**

1. Group intents by `BackendId`.
2. Ask each selected provider for its native plan.
3. Keep plan steps ordered deterministically; do not run multiple native simulations in
   parallel initially.
4. Preserve each backend's preview quality and notes.
5. Fail the requested plan clearly when a selected provider cannot produce a safe plan;
   do not silently omit the selected package.

**Validation:** tests for DNF-only, Flatpak-only and mixed-provider plan; verify mixed plan
contains separate steps and no atomicity claim.

**DoD:** a plan accurately represents every selected backend action.

**Do not:** implement transaction execution in this task.

### S45 — Add sequential executor and contract tests

**Prerequisite:** S43, S44.

**Goal:** apply approved plan steps sequentially and report partial outcomes honestly.

**Allowed files:** `src/core/executor.rs`, `tests/backend_contract.rs`, tests.

**Implementation:**

1. Execute one backend step at a time in plan order.
2. Stop on first failure by default; retain the results of completed steps.
3. Do not attempt rollback of already-completed steps.
4. Report success/failure per step; never label a partially-applied plan as fully successful.
5. Add common contract assertions for package identity, capability truthfulness and error
   handling.

**Validation:** fake providers prove ordering, stop-on-failure and partial outcome behavior.

**DoD:** G4 passes. Backend API is stable enough to build a UI without depending on DNF or
Flatpak concrete types.

**Do not:** add a database journal or persistent cache yet.

---

## Stage G — Minimal Slint UI

**Gate G5.** Only start after G4. Build one working workflow at a time. Do not
try to create a polished app shell and all screens in one task.

### S46 — Add Slint build integration

**Prerequisite:** G4.

**Goal:** compile a minimal Slint window without connecting backend state.

**Allowed files:** `Cargo.toml`, `build.rs`, `src/main.rs`, `ui/app.slint`.

**Implementation:**

1. Pin Slint `=1.19.0` and Slint Kit `=0.9.0` as researched in the original plan.
2. Add the Slint Kit build integration using the official setup matching those versions.
3. Isolate Kit-specific imports under `ui/components/`; do not spread `@kit` imports into
   view code.
4. Add a window containing only a title and a placeholder body.
5. Document the Slint Kit license implications in the project README/license notes before
   distributing a binary.

**Validation:** `cargo check --features gui`; run the app and verify the window opens.

**DoD:** minimal GUI builds and launches on the target Fedora desktop.

**Do not:** add models, navigation, package lists or backend calls.

### S47 — Create the application shell

**Prerequisite:** S46.

**Goal:** add static navigation and an empty content area.

**Allowed files:** `ui/app.slint`, one small navigation component, related theme file.

**Implementation:**

1. Add navigation entries for Browse, Installed and Updates.
2. Add a content panel that changes its placeholder title when navigation changes.
3. Keep navigation state local to Slint for this task.
4. Use simple layout primitives; no animation or complex component framework.

**Validation:** manual click-through of all navigation entries.

**DoD:** all three views can be selected and render the correct placeholder.

**Do not:** connect the backend or create detailed design polish.

### S48 — Add the Rust/UI message bridge

**Prerequisite:** S47.

**Goal:** move requests/events between the Slint thread and the backend worker.

**Allowed files:** `Cargo.toml`, `src/ui/mod.rs`, `src/ui/bridge.rs`, `src/main.rs`.

**Implementation:**

1. Add `crossbeam-channel` in this task if the standard library channels are insufficient;
   justify the dependency in the progress note. Create request and event channels using
   the selected channel implementation.
2. Start exactly one worker thread that owns the core service.
3. UI sends requests; worker sends typed events. Worker never touches Slint objects.
4. Drain events on the UI thread using a Slint timer (50–100 ms is sufficient initially).
5. Do not block the UI thread on a backend command.

**Validation:** a test or manual debug action sends a ping request and receives a response;
window remains responsive.

**DoD:** bridge works without any package-manager call from the UI thread.

**Do not:** add progress streaming or multiple worker pools.

### S49 — Show installed packages

**Prerequisite:** S48.

**Goal:** render installed packages in a simple list.

**Allowed files:** `ui/views/installed.slint`, UI model bindings, bridge event handling.

**Implementation:**

1. Add a Rust request for installed packages.
2. Worker calls `installed_all()` and returns rows plus provider errors.
3. Render package name, version, backend/source and installed state.
4. Use stable `PkgId`-derived keys, never row indexes as identity.
5. Show empty, loading and provider-error states.

**Validation:** manual Fedora test with both DNF and Flatpak installed; test with one provider
unavailable.

**DoD:** installed list loads asynchronously and one provider's failure does not blank the
other provider's results.

**Do not:** add icons, app grouping or infinite scrolling.

### S50 — Add search UI

**Prerequisite:** S49.

**Goal:** search both current providers and display results.

**Allowed files:** Browse view, bridge request/event types, focused tests.

**Implementation:**

1. Add a search input and submit/debounce behavior.
2. Send search requests to the worker; do not call a backend from a Slint callback.
3. Keep each result's backend visible so identical names remain separate choices.
4. Add a simple request sequence number; ignore responses from an older query after a newer
   query has been submitted.
5. Preserve the last results while a new query is loading.

**Validation:** search common packages; rapidly change query and verify stale results do
not replace newer ones.

**DoD:** search remains responsive and source identity is visible.

**Do not:** implement global app deduplication or remote search suggestions.

### S51 — Add package details view

**Prerequisite:** S50.

**Goal:** show details for a selected package without performing mutations.

**Allowed files:** details component/view, UI state and bridge event mapping.

**Implementation:**

1. Selection stores the full `PkgId`, not the package row index.
2. Request details from the corresponding provider.
3. Show available details and explicitly omit unavailable values.
4. Handle package disappearing between list and detail query.

**Validation:** select DNF and Flatpak results with the same display name; verify correct
provider details are shown.

**DoD:** selection and details are identity-safe.

**Do not:** add an install button in this task.

### S52 — Add plan review UI

**Prerequisite:** S51, S44.

**Goal:** let the user request a plan and review changes before confirmation.

**Allowed files:** plan review view/component, bridge request/event mapping.

**Implementation:**

1. Add an action to request an install/remove plan for the selected `PkgId`.
2. Show targets separately from solver-resolved dependency changes.
3. Show preview quality (`Native`, `Estimated`, `Unavailable`) and notes.
4. Require an explicit confirmation action; do not apply from the package details view.
5. Disable confirmation when planning failed or the plan is stale.

**Validation:** test DNF native preview and Flatpak unavailable/limited preview cases.

**DoD:** user can inspect the plan before any mutation begins.

**Do not:** execute operations in this task.

### S53 — Add apply progress and result view

**Prerequisite:** S52, S45.

**Goal:** apply only the confirmed plan and show the outcome.

**Allowed files:** progress/result views, bridge event mapping, focused tests.

**Implementation:**

1. Confirmation sends `Apply(plan_id)`; backend re-checks that the plan exists and is current.
2. Show running state, current backend/action and textual status.
3. Show success/failure for each plan step; represent partial success clearly.
4. Disable duplicate apply while a transaction is active.
5. Refresh installed/update data after completion.

**Validation:** fake-provider success, first-step failure and later-step failure; manual
Fedora install/remove workflow in disposable VM.

**DoD:** G5 passes; no package operation can be triggered without plan review and confirmation.

**Do not:** add activity history, settings or auto-updates.

### S54 — Run the initial end-to-end acceptance checklist

**Prerequisite:** S46–S53.

**Goal:** validate the first usable DNF + Flatpak application as one product.

**Allowed files:** `scripts/test-fedora.sh`, `docs/acceptance.md`, bug fixes limited to
failures found by the checklist.

**Implementation:**

1. Verify clean start when DNF or Flatpak is missing.
2. Verify installed list, search, details, plan review, install/remove and refresh.
3. Verify DNF helper requires authorization and rejects malformed requests.
4. Verify Flatpak's own polkit behavior is used and it is not invoked via Paket helper.
5. Verify identical package names remain distinct by provider.
6. Record Fedora version, DNF5 version and Flatpak version used for acceptance.

**Validation:** run the checklist in a disposable Fedora VM; run `cargo test`, `cargo fmt
--check` and `cargo clippy --all-targets -- -D warnings`.

**DoD:** initial MVP is accepted or every failed acceptance item is recorded as a separate
follow-up task. Do not conceal failures by relaxing acceptance criteria.

**Do not:** start APT/Pacman/Snap/AppImage in the same task.

---

# 5. Additional backend task packs

These are intentionally separate from the first usable slice. Add one provider at a time.
Reuse the common contracts but do not assume command syntax, transaction preview quality,
privilege flow or package identity is identical between ecosystems.

## Pack A — APT / DEB (Debian and Ubuntu)

### S55 — Detect APT-capable hosts

Probe `/etc/os-release`, `apt-get`, `apt-cache` and `dpkg-query`. Return an unavailable
reason on unsupported hosts. Do not run an update or install. Add fake-runner tests.

**DoD:** Debian/Ubuntu compatibility detection is isolated and does not affect Fedora.

### S56 — Parse installed dpkg records

Capture `dpkg-query -W` with an explicit tab-separated format. Parse package name,
architecture, version and status. Only status `install ok installed` means installed.
Add fixtures for installed, removed, half-configured and multiarch records.

**DoD:** parser is pure and does not mislabel dpkg's residual/config-only records as installed.

### S57 — Implement APT installed listing and search

Use `dpkg-query` for installed state and `apt-cache search` for candidates. Parse output in
separate pure functions. Keep installed package identity distinct from candidate identity.

**DoD:** installed/search work through the common catalog API with fake-runner tests.

### S58 — Implement APT package details and update listing

Use `apt-cache show` for details and the researched simulation/list path for candidate
updates. Do not compare Debian versions with RPM/Flatpak versions. Missing security data
remains unknown.

**DoD:** details/updates have fixtures and a real read-only Debian VM smoke test.

### S59 — Implement APT install/remove simulations

Use `apt-get -s` for install/remove/upgrade previews. Parse `Inst`, `Remv` and relevant
summary lines. Mark preview incomplete/unavailable if the complete effect list cannot be
parsed. A simulation is not a guarantee the later transaction is unchanged.

**DoD:** plan includes native dependency effects or explicitly reports preview limitations.

### S60 — Extend the helper with APT operations

Add only typed APT install/remove/update operations to the helper. Validate package IDs,
construct fixed argv and set the documented noninteractive environment/conffile policy.
Never accept arbitrary argv. Preserve apt/dpkg lock errors. Do not add source mutation.

**DoD:** helper tests reject malformed requests; real mutation tests pass only in a disposable
Debian/Ubuntu VM; modified-conffile policy is documented.

### S61 — Add APT provider contract and VM tests

Run the shared catalog/installer contract tests with APT fixtures and a Debian-family VM.
Verify missing/unsupported hosts do not break other providers.

**DoD:** APT is independently enabled/disabled by probe and its tested capability matrix is
recorded.

## Pack B — Pacman (Arch-compatible systems)

### S62 — Detect Pacman hosts and list installed packages

Probe Arch-compatible distro identity and `pacman`. Parse installed package data from
`pacman -Qi` key/value blocks. Preserve package name/version/architecture and handle
multi-line descriptions safely.

**DoD:** installed listing works on Arch VM and parser fixtures cover multiple package blocks.

### S63 — Implement Pacman search and package details

Use `pacman -Ss` and `pacman -Si`; parse repository/name identity without conflating
same-name packages from different repositories.

**DoD:** search/details use fake-runner fixtures and preserve repository identity.

### S64 — Implement Pacman update listing

Use `checkupdates` when available or the researched non-mutating update-list command.
Never run `pacman -Sy` as a standalone refresh/install preparation step.

**DoD:** updates list is read-only and clearly reports missing optional tooling.

### S65 — Implement Pacman plan preview

Use `pacman --print` / `--print-format` for install/remove previews where supported. All
install/update paths must use a full system upgrade (`-Syu`) to avoid partial-upgrade
hazards. Parse the full removal set before allowing removal.

**DoD:** preview shows all native changes or is explicitly marked unavailable.

### S66 — Extend helper for Pacman transactions

Add fixed typed operations for install/remove/full-upgrade. The helper validates native IDs
and never offers a partial-upgrade operation. No `--nodeps` or force bypass.

**DoD:** commands are covered by exact argv tests and VM transaction tests.

### S67 — Add Pacman contract and Arch VM tests

Run shared provider tests on an Arch-compatible VM. Record behavior differences for
Manjaro/other derivatives rather than assuming all Arch derivatives are identical.

**DoD:** Pacman availability and supported distro set are evidence-based.

## Pack C — AUR (separate from Pacman)

### S68 — Implement AUR RPC search/details

Use the official AUR RPC v5 API through blocking HTTPS. Parse search/info responses into
AUR candidates. Cache only if needed; initially a small in-memory cache is enough.

**DoD:** search/details work without an AUR helper installed and without root.

### S69 — Detect supported AUR helper

Probe `paru` and `yay` independently and report which one is available. Do not assume one
is installed because the host is Arch.

**DoD:** AUR install capability is exposed only when a supported helper exists.

### S70 — Add explicit AUR build/install flow

AUR builds execute as the logged-in user, never through `pkexec` and never as root. Require
an explicit warning/review step for PKGBUILD/build scripts. If a safe interactive review
flow cannot be implemented, keep install disabled and provide a clear reason. Do not build
a sandbox in this task pack.

**DoD:** no AUR build command is executed with effective UID 0; test with a disposable Arch
user account.

## Pack D — Snap

### S71 — Probe Snap and list installed snaps

Probe the CLI and snapd availability separately. Parse installed snaps and channels using
machine-readable output where available. Missing snapd is an unavailable backend, not a
startup error.

**DoD:** list/probe works on a supported Snap host and does not assume Fedora/Ubuntu has
Snap installed.

### S72 — Implement Snap search and update listing

Use Snap's supported CLI/API output; preserve channel, publisher and confinement metadata
where available. Leave unavailable fields unknown.

**DoD:** search/update parser has captured fixtures and no network/tool failure is reported
as an empty result.

### S73 — Implement Snap install/remove using snapd authorization

Use the Snap client normally so snapd/polkit owns authorization. Do not invoke it through
Paket's root helper. Model snapd change IDs/status and cancellation according to supported
APIs; do not kill a socket request and claim that the Snap operation was cancelled.

**DoD:** install/remove and cancellation behavior are verified on a Snap-enabled VM.

## Pack E — AppImage local applications

### S74 — Define the managed AppImage directory and record model

Choose one per-user managed directory using the platform directories API. Add a small local
record containing canonical managed path, display name and optional version/icon metadata.
No repository or package-solver abstraction is implied.

**DoD:** path resolution is deterministic and cannot escape the managed directory.

### S75 — List and import AppImages safely

Allow the user to explicitly select a file and copy it into the managed directory. Do not
execute an untrusted AppImage to inspect metadata. Do not auto-mount or run embedded
`--appimage-extract` code as part of a background scan. Extract metadata only through a
reviewed, non-executing and bounded parser; otherwise leave it unknown.

**DoD:** imports are explicit, paths are canonicalized, and listing never executes files.

### S76 — Remove and launch managed AppImages

Remove only records/files whose canonical paths remain inside the managed directory.
Launching is a separate explicit user action; pass a selected path directly to process
execution without a shell. Do not claim dependency management or package updates.

**DoD:** traversal/symlink tests prevent deleting outside the managed root; launching uses
argv and never shell interpolation.

---

# 6. Shared improvements after providers are stable

These are optional enhancements. They must not block the first usable application.

### S77 — Add a small persistent catalog cache

Cache successful installed/catalog results in a simple versioned JSON file under the
per-user application data directory. Write to a temporary file and atomically rename.
Treat malformed/stale cache as a miss. Do not cache privileged credentials or native
transaction plans. Add this only after the live API is stable.

**DoD:** app can display last known installed list immediately and refreshes it in background
without blocking the UI.

### S78 — Add AppStream display enrichment

Parse only needed AppStream fields (desktop ID, localized name, summary, categories and icon
reference). Use it only for presentation/grouping. Never use AppStream ID as an install ID
or silently choose between DNF/Flatpak/AUR sources. Bound file size, decompressed size and
record count; atomically replace the cache after a successful build.

**DoD:** AppStream failure does not prevent package management; same app from multiple
providers still requires an explicit source selection.

### S79 — Add read-only sources view

Show configured DNF/APT/Pacman repositories, Flatpak remotes and Snap sources where the
native CLI supports reliable read-only listing. Do not add/edit/remove sources in this
release. Never import signing keys automatically.

**DoD:** view reports observed native configuration and exposes no mutation controls.

### S80 — Add transaction activity log

Write a small append-only JSONL log containing timestamp, provider, action, selected IDs,
result and a bounded diagnostic tail. Do not store secrets or environment variables. Avoid
building a database for the first version.

**DoD:** activity survives app restarts and malformed trailing log entries do not crash load.

### S81 — Add basic settings

Only add settings after a real setting exists. Use a small TOML file for theme preference,
cache retention and default provider ordering. Validate values and fall back to defaults on
invalid configuration.

**DoD:** settings load/save tests pass; no generic settings framework is introduced.

---

# 7. Packaging and release tasks

### S82 — Package the first Fedora release

Create an RPM spec for the GUI and helper. Install the helper under `/usr/libexec/paket/`
with root-owned permissions supplied by the package manager. Do not manually set setuid bits.
Include the desktop file, icon and AppStream metainfo when available. Verify the helper
path/ownership/SELinux label after installation.

**DoD:** clean Fedora VM install/uninstall works; helper is not writable by an unprivileged
user; DNF + Flatpak workflows pass after package installation.

### S83 — Add CI quality checks

Add CI for format, tests, clippy and a static scan that rejects shell execution in
application/backend code. Allow process spawning only in the approved runner and helper
modules. Do not require package managers or root in unit-test CI.

**DoD:** CI catches formatting, lint, test and forbidden process-spawn regressions.

### S84 — Document support and limitations

Document tested distro versions, backend versions, capabilities, privilege behavior,
preview limitations, cancellation behavior, license notices and known risks. Clearly
label untested backends as experimental rather than supported.

**DoD:** release documentation matches actual contract/VM test evidence.

---

# 8. Test strategy

Use the lightest test level that validates the change.

| Test level | When to use it | Examples |
|---|---|---|
| Pure unit test | Parsing, ID validation, model behavior | JSON parsers, os-release parser, helper request validation |
| Fake-runner test | Command construction and adapter flow | Expected executable/argv and supplied stdout/stderr |
| Contract test | Provider behavior shared across backends | Identity preservation, capabilities, error isolation |
| VM smoke test | Real package-manager behavior | DNF install/remove, Flatpak polkit, APT/Pacman transaction |
| UI manual test | Slint interaction and responsiveness | Search, plan review, cancellation/result rendering |

Rules:

1. Most tests must run without root and without any package manager installed.
2. Real install/remove tests run only in disposable virtual machines with snapshots.
3. Fixtures must be captured from real supported tool versions. Do not invent a convenient
   output schema and then treat it as authoritative.
4. When native output changes, add a new fixture and a parser case; do not edit old fixture
   bytes merely to force the old parser to pass.
5. Keep parser functions pure. Keep process execution outside parsers.
6. Never run host-mutating tests as a default `cargo test` side effect.

## 8.1 Minimum validation after every task

Run:

```bash
cargo fmt --check
cargo test
```

For tasks that touch process execution, backend adapters, helper or core orchestration, also
run:

```bash
cargo clippy --all-targets -- -D warnings
```

The task may specify a narrower test command as well. The narrow test is additional, not a
replacement for the minimum checks unless the repository is not yet in a state where the
full command can pass; such a limitation must be recorded.

---

# 9. Risk register and mandatory safeguards

| Risk | Safeguard |
|---|---|
| Agent expands a task into a large refactor | One `Sxx` per run; strict allowed-files list; stop on blocker. |
| Package name used as mutation identity | `PkgId` is required; decode/validate in backend and helper. |
| Shell/argument injection | Fixed executable + argv arrays; no shell; helper never accepts arbitrary argv. |
| DNF/APT/Pacman package database lock | Serialize operations; map lock errors to a readable `Busy` state. |
| Partial cross-backend transaction | Execute sequentially; report each completed/failed step; no fake rollback. |
| Dependency solver differences hidden by common model | Native plan details and preview quality are explicit. |
| Fedora Atomic treated as normal DNF | Disable DNF5 backend; add a separate rpm-ostree backend only with a dedicated plan. |
| AUR build script executes as root | AUR helpers run as the user; never use Paket root helper. |
| Flatpak/Snap double privilege prompt | Use their native polkit authorization flow. |
| Untrusted AppImage executed during scan | Never execute files for metadata; explicit user launch only. |
| Repository key silently trusted | Never auto-import keys or disable signature verification. |
| Slint Kit breaking changes/license | Pin researched versions; isolate Kit imports; document license obligations. |

---

# 10. Suggested first execution sequence

Do not ask the agent to do this whole list in one run. This is only the order for separate
agent invocations and human reviews.

| Order | Task | Expected result |
|---:|---|---|
| 1 | S01 | Repository baseline recorded |
| 2 | S02 | Minimal Cargo package compiles |
| 3 | S03–S06 | Basic domain types and errors |
| 4 | S07–S10 | Safe process runner and fake runner |
| 5 | S11–S12 | Host parser and backend registry skeleton |
| 6 | S13–S21 | DNF read-only catalog |
| 7 | S22–S34 | DNF plan + privileged install/remove |
| 8 | S35–S41 | Flatpak provider |
| 9 | S42–S45 | Stable two-provider core API |
| 10 | S46–S54 | Initial Slint UI and acceptance test |
| 11 | S55 onward | Add optional providers one task pack at a time |

**Recommended first coding task after this document is reviewed: S01 only.**

---

# Appendix A — What “done” means for a provider

A provider is not considered implemented because it can list packages. Before it is
advertised as supported, verify each applicable item:

- [ ] Probe reports available/unavailable correctly, with a reason.
- [ ] Package IDs preserve all identity discriminators needed by that backend.
- [ ] Installed listing is correct for the supported host/tool version.
- [ ] Search preserves distinct sources/candidates.
- [ ] Details never use display name as mutation identity.
- [ ] Updates are listed without cross-backend version comparison.
- [ ] Plans distinguish selected targets from solver-resolved effects.
- [ ] Plan preview quality is honest (`Native`, `Estimated`, `Unavailable`).
- [ ] Install/remove use the native authorization model.
- [ ] Helper validation is repeated for privileged system package managers.
- [ ] Common errors are classified without hiding diagnostic context.
- [ ] Missing tools/unsupported hosts do not crash other providers.
- [ ] Unit, fake-runner, contract and required VM tests pass.
- [ ] Tested distro and package-manager versions are documented.

# Appendix B — Capability matrix template

Fill this table only with tested facts. “Planned” is not “supported”.

| Backend | Host tested | Installed | Search | Details | Updates | Plan | Install | Remove | Notes |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|
| DNF5 | Fedora Workstation | — | — | — | — | — | — | — | |
| Flatpak | Fedora Workstation | — | — | — | — | — | — | — | |
| APT | Debian/Ubuntu | — | — | — | — | — | — | — | |
| Pacman | Arch-compatible | — | — | — | — | — | — | — | |
| AUR | Arch-compatible | — | — | — | — | — | — | — | Unprivileged build only |
| Snap | Snap-enabled host | — | — | — | — | — | — | — | |
| AppImage | Linux desktop | — | — | — | N/A | N/A | — | — | Local file manager |

# Appendix C — Change-control rule

If implementation reality conflicts with this document:

1. Stop the current task if the conflict changes its scope or safety assumptions.
2. Record the observed command output, tool version and host in `docs/deviations.md`.
3. Propose the smallest correction to the relevant task or contract.
4. Do not use the discovery as permission to implement future tasks.
5. Resume only the current `Sxx` after the scope is clear.

This keeps the plan usable for a smaller coding model: small prompts, small diffs,
fast validation, explicit stopping points and no hidden multi-stage work.
