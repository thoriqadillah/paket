# Paket — Production Implementation Plan

> **Execution plan for a small-context coding agent.**
>
> Product goal: one Linux desktop application for discovering and managing software from
> native distribution repositories, Flatpak, Snap, AUR, and locally managed AppImages.
>
> **This is not a one-shot implementation prompt. One agent run = exactly one `Sxxx`
> task.** Each task is independently reviewable, has a narrow file scope, and ends with
> explicit validation. Complete the tasks in order unless a prerequisite says otherwise.
>
> Research snapshot: 2026-11-23. Re-check pinned upstream documentation when a backend
> task starts; package-manager CLI and D-Bus interfaces can change between distro releases.

---

## 0. How to execute this plan

### 0.1 One task per agent run

Give the coding agent exactly one task, for example `S023`, plus the architecture sections
referenced by that task. Do not ask it to implement a whole stage, milestone, backend, or
“everything until the UI”. Review the result before assigning the next task.

An OpenSpec change covers one stage; a run still implements only one unchecked `Sxxx` task
from it. Do not loop through the change's remaining tasks.

Target 30–90 minutes of focused agent work per task. Most tasks should touch one small
production module, its corresponding tests, and at most one fixture file. If a task appears
larger in the actual repository, the agent must stop and propose a split rather than
silently widening scope.

### 0.2 Required workflow for every task

1. Read the task, its prerequisites, the relevant architecture sections, and the active
   OpenSpec change for the stage.
2. Inspect the current repository. Never assume the previous task was implemented exactly
   as planned.
3. Implement only the assigned task. Do not start the next task or perform opportunistic
   refactors.
4. Add tests for the behavior introduced in this task.
5. Run the exact task validation, then `cargo fmt --check` and the narrowest relevant
   `cargo test` command. Run `cargo clippy --all-targets -- -D warnings` at the gates.
6. Do not weaken, skip, delete, or silence tests to obtain a passing result.
7. Mark only this `Sxxx` complete in the change's `tasks.md`. Stop and report: summary,
   files changed, validation evidence, known limitations, deviations, and whether the DoD
   passed. Do not start the next task and do not archive the change.

### 0.3 Standard prompts for the local agent

Change names are listed in `AGENTS.md`. Use the propose prompt once per stage, then the
apply prompt once per task.

**Propose a stage change**

```text
Use the openspec-propose skill to create the OpenSpec change `<change-name>` for
<stage name> from docs/plan.md.

Rules:
- Read the matching plan section, its prerequisites and referenced architecture first.
- Derive scope only from docs/plan.md. Do not invent requirements or include later stages.
- Cover exactly this stage's tasks: `<Sxxx–Sxxx>`.
- In tasks.md, mirror each plan task with its ID and verification. Mark a task complete
  only if the current repository already satisfies its DoD.
- Do not edit implementation code. Stop after the planning artifacts are written.
```

**Apply one task**

```text
Use the openspec-apply-change skill on the OpenSpec change `<change-name>`.
Implement only task `<Sxxx>`, the next unchecked task in that change.

Context:
- docs/plan.md is the plan; the change's tasks.md mirrors this stage's Sxxx tasks.
- Read the task, prerequisites, and referenced architecture sections first.
- Inspect the actual repository before editing.

Rules:
- Verify the task's prerequisites and that it is the next unchecked task. If not, stop and
  report the ordering problem.
- Implement only `<Sxxx>`. Do not implement later tasks or broad refactors.
- Keep the diff small and follow the task's Allowed files list.
- Do not add dependencies unless the task explicitly allows them. Explain any addition.
- Never construct shell commands. Invoke a fixed executable with an argv array.
- Never turn a package display name into a mutation target. Use validated native IDs.
- Never disable signature checks, auto-trust a repository key, or run a package build as root.
- Add behavior-focused tests and preserve unknown/unsupported states explicitly.
- Run the task validations, cargo fmt --check, and the relevant tests.
- Mark only `<Sxxx>` complete in the change's tasks.md when the DoD passes.
- If the scope does not fit, stop and report the blocker instead of widening it.
- Do not commit. Stop after `<Sxxx>`; do not continue and do not archive the change.
```

### 0.4 Progress, commits, and deviations

Progress is tracked only in OpenSpec. Do not create or maintain a hand-written progress
file.

- One OpenSpec change covers one stage. `AGENTS.md` lists the change names.
- The change's `tasks.md` mirrors that stage's `Sxxx` tasks from this plan.
- Check off an `Sxxx` task only when its DoD passes.
- When every task in the change is done, sync specs and archive the change with the
  OpenSpec workflow. Gates still control when the next stage may start.
- If `tasks.md` and this plan disagree, this plan wins.

Record factual differences between this plan and the host/upstream tool behavior in
`docs/deviations.md`. A deviation is not permission to redesign unrelated architecture.

Prefer one commit per completed task, matching the human's repository workflow. Suggested
message: `S023: stream child process output`. Do not commit unless the user asks.

### 0.5 Gates

| Gate | Required tasks | Result |
|---|---|---|
| G0 | S001–S012 | Stable product decisions, toolchain, and domain contracts |
| G1 | S013–S026 | Tokio runtime, safe subprocess layer, bounded event delivery, shutdown |
| G2 | S027–S039 | Batch intent model, transaction queue, serialization, and recovery rules |
| G3 | S040–S048 | Host detection and provider capability registry |
| G4 | S049–S060 | DNF5 provider with supported read and mutation paths |
| G5 | S061–S071 | Flatpak provider |
| G6 | S072–S078 | Typed privileged-helper foundation |
| G7 | S079–S090 | APT provider |
| G8 | S091–S107 | Pacman and opt-in AUR provider |
| G9 | S108–S119 | Zypper and rpm-ostree providers |
| G10 | S120–S131 | Snap and managed AppImage providers |
| G11 | S132–S144 | Unified catalog, AppStream enrichment, persistence, cache, history |
| G12 | S145–S160 | Production UI workflows in Slint + Slint Kit |
| G13 | S161–S167 | Security, integration qualification, packaging, release readiness |

**A gate is not a prompt.** It is a review checkpoint. Do not ask the agent to implement a
gate in one run.

---

## 1. Product scope and production definition

### 1.1 Product objective

Paket is a **local package-management client**, not a new package manager. It discovers
native package-management systems on the host and presents their capabilities in one GUI.
It delegates dependency solving, package verification, file ownership, installation,
removal, and update semantics to the official tool/API for each ecosystem.

The common UX may unify:

- application discovery and search;
- installed software and available updates;
- package/application details;
- a reviewable pending-change basket;
- transaction progress, results, and history;
- a consistent view of which backend/source owns each candidate.

The common UX must **not** erase ecosystem-specific semantics such as installation scope,
repository identity, package architecture, dependency effects, full-upgrade requirements,
reboot-required deployments, runtime/app distinctions, or AUR build trust.

### 1.2 Production support policy

“Linux support” must never be inferred from `/etc/os-release` alone. A backend is enabled only
when the host family/version is in the tested support matrix **and** its native tool/service
is present at a supported version.

Use these provider families as the initial production target:

| Ecosystem | Provider | Important boundary |
|---|---|---|
| Fedora/RPM family | DNF5 | DNF5 is the provider; raw RPM queries alone are not an installer |
| RHEL-compatible systems with DNF4 | Future DNF4 adapter | Not enabled in the first production matrix; requires a separate task pack and VM qualification. Never reuse DNF5 parsers. |
| openSUSE/SUSE RPM | Zypper | Separate resolver and repository semantics |
| Fedora Atomic / rpm-ostree hosts | rpm-ostree | Separate deployment/layering model; operations may require reboot |
| Debian/Ubuntu | APT + dpkg-query | Use APT for solving; do not use raw `dpkg -i` as the normal install path |
| Arch Linux | pacman/libalpm-facing adapter | Package install must respect full-system-upgrade semantics |
| Arch User Repository | AUR (separate provider) | Untrusted build scripts; explicit review and user build process |
| Cross-distro application layer | Flatpak | Track user/system installation and full ref identity |
| Snap | snapd / Snap CLI | Optional; available only when snapd is present and healthy |
| Standalone binaries | Managed AppImage | Local file management, not a repository/package solver |

For the first supported production release, publish a precise matrix of tested distro
versions and architectures. A reasonable initial matrix to qualify is Fedora Workstation
(DNF5), Ubuntu LTS and Debian stable (APT), Arch Linux (pacman; AUR opt-in), openSUSE
(Zypper), Flatpak on those hosts, and Snap only where snapd is installed. rpm-ostree is a
separate supported provider, not a DNF mode. Expand the exact distro/version list only
when VM qualification passes.

Do not claim that every distro in an OS family is supported merely because it shares RPM,
DEB, or pacman packages. Include a visible “detected but not qualified” state.

### 1.3 Production-ready means

A release is production-ready only when all of these are true:

1. Every advertised backend has fixture-backed parsers, command/API contract tests, and
   successful/failed transaction tests on disposable VMs.
2. Package identities remain unique across backend, source, architecture, branch, and
   installation scope.
3. A transaction is reviewed before mutation; the actual native solver remains the source
   of truth; changed/stale plans require re-review.
4. One mutation coordinator prevents Paket from starting overlapping system writes.
5. External package-manager locks are handled without deleting lock files or bypassing
   native lock protection.
6. AUR and local file operations are treated as untrusted input.
7. Crashes/restarts never cause Paket to replay a write automatically or claim a result it
   cannot prove.
8. Logs, caches, and UI event streams are bounded and do not expose credentials.
9. Keyboard navigation, focus, screen-reader labels, scaling, translations, and common
   failure dialogs are tested.
10. Installation packages, helper ownership/permissions, signatures, upgrades, and
    uninstall paths are verified on clean VMs.

### 1.4 Explicit non-goals

Do not implement:

- a custom dependency solver, downloader, repository format, or package database;
- cross-ecosystem atomic transactions or an undo button that promises rollback;
- automatic system updates or background installs without user consent;
- arbitrary shell execution, arbitrary privileged argv, or user-defined root scripts;
- repository key import or “trust this key” automation;
- a dynamic third-party plugin ABI in the first production line;
- remote host administration, multi-user administration, or server mode;
- Nix/Guix support until the provider contract is stable;
- automatic removal of user data, Flatpak app data, orphan packages, or AUR build files.

---

## 2. Locked technical decisions

| ID | Decision | Reason / qualification |
|---|---|---|
| D1 | Project name and binaries are `paket` / `paket-helper` | All Cargo targets, desktop IDs, docs, logs, package names and paths use Paket |
| D2 | Rust 2024 edition | Stable language baseline |
| D3 | Pin Slint `1.19.0` and Slint Kit `0.9.0` when UI work begins | Current research snapshot; do not follow floating latest versions |
| D4 | Rust toolchain minimum is `1.95` while using Slint Kit `0.9.0` | Slint Kit currently declares Rust 1.95+ and Slint 1.19.0 |
| D5 | Slint Kit adoption is gated by license review | The base kit is GPL-3.0-or-later; choose a compatible Paket license or do not ship Kit components |
| D6 | Use Tokio `1.54.1` with minimal explicit features | It fits async subprocesses, queues, timeouts, and bounded concurrency |
| D7 | Tokio runs on a dedicated multi-thread runtime, separate from Slint's event loop | Slint documents that its event loop does not drive Tokio futures reliably |
| D8 | Do not wrap the Slint event loop in `#[tokio::main]` | Avoid runtime/event-loop coupling and accidental main-thread blocking |
| D9 | Native package tools/services own dependency resolution and transaction semantics | Paket only adapts, validates, displays, and schedules them |
| D10 | Never invoke a shell | Fixed executable + `OsString` argv; no `sh -c`, `bash -c`, or interpolated command string |
| D11 | Mutations run through one global FIFO coordinator initially | Multiple clicks enqueue intent; they do not start competing package transactions |
| D12 | Group same-backend/same-scope intents into one native transaction | One resolver pass is better than one process per clicked package |
| D13 | Cross-backend batches are sequential and non-atomic | Show partial success explicitly; never promise all-or-nothing across ecosystems |
| D14 | Read-only discovery can run concurrently with a strict bound | Keep UI responsive without unlimited subprocesses or network calls |
| D15 | System privilege uses native authorization or a narrow typed helper | DNF5 daemon/polkit, Flatpak's own system helper, snapd, or Paket helper as appropriate |
| D16 | AUR builds always run as the logged-in user | Never run PKGBUILD code as root |
| D17 | AppImage is a managed local-file provider | No universal repository or update source exists for every AppImage |
| D18 | SQLite stores Paket state, not package-manager truth | Installed state and dependency resolution must be refreshed from native providers |
| D19 | No plugin ABI | A static provider registry keeps security and release compatibility manageable |
| D20 | Slint UI consumes a stable Rust application-service contract | No package-manager logic in Slint callbacks |

### 2.1 Tokio and Slint integration decision

Use Tokio. Package management is primarily I/O-bound and Paket will coordinate multiple
long-running child processes, D-Bus calls, network catalog queries, timeouts, cancellation,
and progress updates. Tokio gives these operations a shared scheduler and async process
I/O; it does **not** make package transactions safe to run concurrently.

Runtime model:

```text
Main OS thread
  Slint event loop
  ├── UI callbacks send AppCommand through bounded channel
  └── UI updates only happen on this thread

Dedicated Tokio multi-thread runtime
  AppCoordinator
  ├── read scheduler: bounded concurrency (initially 4)
  ├── mutation coordinator: one active mutation batch globally
  ├── provider adapters
  ├── Tokio subprocesses / D-Bus / Snap requests
  └── bounded event delivery + latest-progress watch

Tokio -> Slint
  slint::invoke_from_event_loop(move || update_model(...))
```

Do not pass Tokio futures directly to `slint::spawn_local`. Slint's current documentation
warns that the Slint event loop cannot reliably drive Tokio scheduling. A dedicated Tokio
runtime plus `slint::invoke_from_event_loop` keeps responsibilities clear.

Use `tokio::process::Command` for async child I/O. Keep each `Child` handle alive until the
process is reaped. Never assume dropping a future or child handle cancels a native package
transaction. Cancellation is an explicit request: stop reading as appropriate, signal the
child/process group according to the backend policy, await exit/reap, then reconcile state.

### 2.2 Multi-click and batch behavior

The app must support repeated Install/Remove clicks without launching a new package-manager
process on every click.

Required behavior:

1. A click adds or updates a **draft intent** in the pending-changes basket.
2. Duplicate identical intents are de-duplicated. Contradictory intents for the same exact
   package/scope are shown as a conflict that the user must resolve.
3. On Review, intents are grouped by `(BackendId, InstallationScope)`.
4. Each group becomes one native transaction plan. The native solver reports dependency,
   upgrade, removal, and conflict effects where available.
5. A batch containing more than one backend is an ordered list of independent transaction
   steps. It is not atomic across backends.
6. Apply starts one global mutation batch. Additional clicks during execution belong to
   the next draft; they must not mutate the already-approved batch.
7. The coordinator runs one transaction step at a time. On failure, stop by default,
   refresh affected providers, mark later steps as skipped, and require a new review.
8. After each success, refresh installed state and invalidate the related caches.

This permits many clicks and many selected packages while preserving package-manager locks
and understandable failure semantics. Later optimization may run operations concurrently
only when resource locks and provider semantics prove they are independent. Do not add
parallel mutations in this plan's first production line.

---

## 3. Architecture

### 3.1 Runtime and module layout

Keep one Cargo package with a library and two binaries. Do not split into many crates until
build isolation or a clear ownership boundary actually requires it.

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
build.rs                         # Slint integration, added in UI stage
src/
  lib.rs
  main.rs                        # creates Tokio runtime, then runs Slint on main thread
  bin/
    helper.rs                    # typed privileged helper
  app/
    mod.rs
    coordinator.rs                # application commands and service orchestration
    events.rs                    # typed UI/application events
  core/
    mod.rs
    ids.rs
    package.rs                   # package/app/source identity models
    capability.rs
    transaction.rs               # plans, batches, progress and outcomes
    error.rs
    registry.rs
  runtime/
    mod.rs
    runtime.rs                   # runtime construction and shutdown
    command.rs                   # safe command spec and output types
    process.rs                   # async process runner
    events.rs                    # bounded event delivery
    scheduler.rs                 # read concurrency and mutation queue
  platform/
    mod.rs
    os_release.rs
    host.rs
    probe.rs
    xdg.rs
  backends/
    mod.rs
    dnf5/                        # CLI catalog + optional/required DNF5 daemon adapter
    apt/
    pacman/
    aur/
    zypper/
    rpm_ostree/
    flatpak/
    snap/
    appimage/
  helper/
    mod.rs
    protocol.rs
    client.rs
    validation.rs
  store/
    mod.rs
    db.rs
    migrations/
    package_cache.rs
    history.rs
    settings.rs
  catalog/
    mod.rs
    search.rs
    appstream.rs
    ranking.rs
  ui/
    mod.rs
    bridge.rs
    state.rs
ui/
  app.slint
  components/
  views/
tests/
  fixtures/
  support/
  providers/
  vm/
scripts/
docs/
```

Add each file only when its task is reached. Do not scaffold all directories in S001.

### 3.2 Provider interfaces (SOLID without framework overengineering)

Use interface segregation. Do not force every provider to implement every operation.

Suggested contracts (adapt names as code evolves):

```rust
#[async_trait::async_trait]
pub trait CatalogBackend: Send + Sync {
    fn info(&self) -> BackendInfo;
    async fn list_installed(&self, scope: &InstallationScope)
        -> Result<Vec<PackageRecord>, CatalogError>;
    async fn search(&self, query: &str, scope: &SearchScope)
        -> Result<Vec<PackageRecord>, CatalogError>;
    async fn details(&self, id: &PackageId)
        -> Result<PackageRecord, CatalogError>;
    async fn updates(&self, scope: &InstallationScope)
        -> Result<Vec<UpdateRecord>, CatalogError>;
}

#[async_trait::async_trait]
pub trait TransactionBackend: Send + Sync {
    async fn plan(&self, intents: &[ChangeIntent])
        -> Result<BackendPlan, PlanError>;
    async fn apply(&self, approved: &BackendPlan, events: EventSink)
        -> Result<BackendOutcome, OperationError>;
}
```

Optional narrow interfaces may be added for repository listing, app metadata, local files,
or update policies. Do not make a giant trait with 30 methods. Do not add a dynamic plugin
loader, custom async-trait framework, DI container, or custom event bus. The small
`async-trait` macro used to make the two provider ports object-safe is an implementation
detail, not a new runtime or provider abstraction.

The provider registry is assembled statically at startup. A provider can register read-only
capabilities even when its mutation service/helper is unavailable.

### 3.3 Identity model

Never use package display name as identity.

- `BackendId`: Dnf5, Apt, Pacman, Aur, Zypper, RpmOstree, Flatpak, Snap, AppImage. DNF4 is reserved for a later adapter task pack.
- `InstallationScope`: System, User, NamedInstallation(String), LocalManagedStore(PathKey).
- `PackageId`: backend + scope + opaque backend-native identifier.
- `SourceId`: backend-native repository/remote/channel identity.
- `AppId`: AppStream component ID when available; this is a display/application grouping
  identity, not an install target.

Examples:

- RPM package: native NEVRA (name, epoch, version, release, architecture) plus repo context
  where needed to distinguish candidate packages.
- DEB: package name and architecture; candidate version/source remain separate fields.
- Pacman: package name and repository for candidates; package name for installed query.
- Flatpak: installation identity + app/runtime + ID + architecture + branch + remote.
- Snap: snap name plus channel/revision where applicable.
- AUR: AUR package base/name; installed binary package identity remains Pacman-owned.
- AppImage: Paket-managed file record ID + canonical managed path.

Core keeps native IDs opaque. Each provider decodes and validates its own ID immediately
before command/API calls.

### 3.4 Package versus application

A package is a native installable unit. An application is a user-facing product that may map
to zero, one, or many package candidates across providers. Never collapse package records
just because their display names match.

Minimum production models:

```rust
struct PackageRecord {
    id: PackageId,
    name: String,
    version: Option<String>,          // opaque display string; not globally comparable
    architecture: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    source: Option<SourceId>,
    installed: InstalledState,
    kind: PackageKind,
    size: Option<u64>,
    metadata_origin: MetadataOrigin,
}

struct AppRecord {
    id: Option<AppId>,
    name: String,
    summary: Option<String>,
    description: Option<String>,
    icon: Option<TrustedIconRef>,
    categories: Vec<String>,
    package_candidates: Vec<PackageId>,
}
```

Unknown values remain `None`/Unknown. Metadata must carry provenance and last-refresh
information so the UI can indicate stale/incomplete information.

### 3.5 Transaction model

A `ChangeIntent` is what the user asked for. A `BackendPlan` is what a native provider
resolved. A `BatchPlan` is an ordered group of backend plans. These are different objects.

Each resolved change contains action (install/update/downgrade/remove/reinstall/obsolete),
exact native package identity, previous/candidate versions where known, download/installed
size where known, and whether the change is dependency-driven or explicitly selected when
the backend can tell us.

Each plan has `PreviewQuality::{NativeComplete, NativePartial, Estimated, Unavailable}`.
Never display an incomplete list as a complete solver preview. If a backend cannot provide
a safe preview, require an explicit warning and a confirmation policy approved for that
backend; otherwise keep its mutation capability disabled.

A plan is invalidated if provider state changed, repositories were refreshed, or a
conflicting external package-manager transaction ran after review. Re-plan and ask for
confirmation again if the transaction result differs materially from the approved plan.

### 3.6 Security boundary

- Only `runtime/process.rs` spawns normal unprivileged commands.
- Only `paket-helper` spawns privileged package-manager commands.
- The helper accepts typed operations, never an executable, arbitrary argv, working
  directory, environment, or shell string from the GUI.
- Helper binary must be root-owned, not writable by regular users, installed at a fixed
  absolute path, and invoked through the system's polkit mechanism.
- Use native polkit integrations when the backend provides them (DNF5 daemon, Flatpak
  system helper, snapd); do not wrap those operations in a second authentication layer.
- Do not set permissive polkit rules or retain passwordless authorization by default.
- Do not log secrets, credential-bearing URLs, repository auth config, or full environment.
- Do not disable GPG/signature checks or auto-import keys.

---

## 4. Backend behavior contracts

### 4.1 DNF5 and DNF4 are different providers

DNF5 has JSON output for supported commands and a documented D-Bus daemon API. The DNF5
`repoquery` command uses a query-format interface; do not assume every command supports JSON.
The `dnf5daemon-server` service exposes session, goal resolution, transaction methods,
polkit authorization, and transaction progress signals. Use its documented API for DNF5
system mutations when available and qualify the exact server/API version in the support
matrix. The service is a separate package on Fedora; detect it and report the missing
capability clearly. If implementing a CLI fallback, keep it a separate tested transport.

DNF4 is a separate future adapter and is not part of the first production support matrix.
If added later, command syntax, output, machine-readable support, and helper behavior must
be researched and fixture-tested independently. Do not run DNF4 through DNF5 parsers.

### 4.2 APT / dpkg

Use `dpkg-query` for installed package inventory and APT for repository search, dependency
resolution, simulations, updates, and installs/removals. APT exposes machine-readable
progress through `APT::Status-Fd`; use it rather than parsing a localized progress bar.
Treat dpkg lock contention as a normal Busy/Waiting state. Preserve local conffiles by a
reviewed policy; never silently force overwrite or discard local configuration.

Do not use raw `dpkg -i` as the ordinary installation path. Local `.deb` installs, if later
supported, go through APT with file validation and a typed helper flow.

### 4.3 Pacman and AUR

Arch does not support partial upgrades. Never implement `pacman -Sy package` or refresh
sync databases without completing the corresponding full upgrade. A normal repository
install/update plan must incorporate `pacman -Syu` semantics and include any full-system
upgrade effects in the review. If the provider cannot present a sufficiently honest plan,
mark its preview quality accordingly or disable that operation until a reliable resolver
path is implemented.

AUR is not a trusted binary repository. PKGBUILDs are user-produced scripts. Show the
PKGBUILD and changes since the previous version, require explicit user review, build as the
logged-in user (never root), preserve build logs, and install the resulting package through
Pacman. Never assume an AUR signature makes its build script safe.

### 4.4 Zypper and rpm-ostree

Zypper is its own RPM ecosystem adapter, not an alias for DNF. It has dry-run support,
non-interactive options, vendor/repository rules, patches, and orphaned-package semantics.
Do not auto-accept licenses or vendor switches. Record the exact policy shown to the user.

rpm-ostree is a separate deployment/layering provider. Its package changes often stage a
new deployment that becomes active after reboot. The UI must show pending deployment and
reboot-required states; it must not pretend that layering behaves like a live DNF install.

### 4.5 Flatpak

A Flatpak identity includes installation scope and full ref details. Distinguish app from
runtime, remote, architecture, branch, and system/user/other named installations. Do not
silently choose between same-ID refs from multiple remotes or installations. Deleting app
data is a separate destructive choice and is off by default. Prefer the supported Flatpak
CLI, which uses Flatpak's own system helper/polkit path; do not call the privileged
`org.freedesktop.Flatpak.SystemHelper` interface directly as a third-party client.

### 4.6 Snap

Snap availability depends on snapd. Snap auto-refresh is controlled by snapd and can happen
outside Paket. Represent snapd changes/tasks distinctly from Paket's own local queue. Never
claim Paket owns or prevents auto-refresh behavior. Use supported CLI or documented snapd
interfaces and reconcile the actual snapd change state after any interruption.

### 4.7 AppImage

An AppImage is a local executable file. Import must not execute it. Do not assume every
AppImage has update information; the update endpoint is embedded by the application author
and may be absent or use a tool-specific mechanism. Store managed files under the Paket
user data directory, avoid overwriting unrelated files, preserve checksums, and ask before
replacing/deleting. Launch only after explicit user action and use the normal desktop user,
never root.

---

# 5. Incremental implementation tasks

Every task below is intentionally narrow. Its “Allowed files” list is a scope boundary,
not an invitation to create the entire planned tree early.

## Stage A — Product and domain foundation

**Gate G0.** The codebase has locked decisions, a reproducible toolchain, and tested core
models. No package-manager operation is executed.

### S001 — Inspect the actual repository

**Goal:** record baseline without changing implementation code.

**Allowed files:** `docs/baseline.md` only.

**Implement:** inspect repository tree, Cargo files, toolchain, tests, CI, current host
`/etc/os-release`, installed package tools, and existing app code. Record baseline `cargo
check`, `cargo test`, and `cargo fmt --check` where applicable.

**Validate:** run baseline commands only.

**DoD:** baseline file records current state and blockers; no source files changed.

**Do not:** scaffold or rename files in this task.

### S002 — Lock project naming and licensing decision

**Prerequisite:** S001.

**Allowed files:** `docs/decisions.md`.

**Implement:** record Paket naming, expected distribution model, intended project license,
third-party license policy, and the Slint Kit GPL-3.0-or-later compatibility gate. If the
project license is undecided, mark Slint Kit adoption blocked rather than assuming a license.

**Validate:** manually review the decision record.

**DoD:** license compatibility is an explicit yes/no decision with evidence.

**Do not:** change source license headers or add UI dependencies.

### S003 — Pin Rust toolchain and edition

**Prerequisite:** S002.

**Allowed files:** `rust-toolchain.toml`, `Cargo.toml`.

**Implement:** use edition 2024 and set `rust-version = "1.95"` while Slint Kit 0.9.0 is
selected. Pin the toolchain channel used in CI. Document that any later MSRV reduction
requires proving compatibility with the selected Slint/Kit versions.

**Validate:** `rustc --version`, `cargo check`.

**DoD:** local and CI toolchain requirements are explicit and reproducible.

**Do not:** add Tokio/Slint or backend dependencies yet.

### S004 — Create the minimal Cargo package

**Prerequisite:** S003.

**Allowed files:** `Cargo.toml`, `src/lib.rs`, `src/main.rs`, `src/bin/helper.rs`.

**Implement:** create one Cargo package named `paket`, one library, GUI binary placeholder,
and `paket-helper` placeholder. Keep targets compiling. Add only serde, serde_json and
thiserror if the project lacks equivalents.

**Validate:** `cargo check --all-targets`; `cargo test`.

**DoD:** all targets build; no unused backend/UI dependency set.

**Do not:** create all subdirectories or implement command execution.

### S005 — Establish formatting and lint policy

**Prerequisite:** S004.

**Allowed files:** `rustfmt.toml`, `clippy.toml` only if required, `Cargo.toml`.

**Implement:** adopt standard rustfmt; configure no broad allowlist. Document the required
CI commands: fmt, test, clippy with warnings denied.

**Validate:** `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`.

**DoD:** project has a repeatable local lint baseline.

**Do not:** add unrelated lint suppression attributes.

### S006 — Add typed core errors

**Prerequisite:** S004.

**Allowed files:** `src/core/mod.rs`, `src/core/error.rs`.

**Implement:** add distinct `CatalogError`, `PlanError`, and `OperationError` families for
unsupported capability, unavailable backend, not-found, invalid identity, busy/locked,
authorization denied, network, dependency, signature/trust, no space, timeout, cancellation,
parse failure, stale plan, and native command failure. Keep native stderr/diagnostic detail
separate from user-facing message.

**Validate:** unit tests for matching variants and safe user messages.

**DoD:** callers can branch on error kinds without parsing arbitrary stderr.

**Do not:** implement distro-specific text parsing here.

### S007 — Add backend and package identity types

**Prerequisite:** S004.

**Allowed files:** `src/core/ids.rs`, `src/core/mod.rs`.

**Implement:** add `BackendId`, `PackageId { backend, scope, native }`, `SourceId`, and
stable display/key helpers. Include all planned providers but keep native IDs opaque.

**Validate:** equality/hash/serialization tests; same name from DNF and Flatpak must not
compare equal.

**DoD:** package identity includes backend and scope and is stable as a UI list key.

**Do not:** parse a native package identifier in core.

### S008 — Add installation scopes

**Prerequisite:** S007.

**Allowed files:** `src/core/ids.rs`, matching tests.

**Implement:** model System, User, named Flatpak installation, and Paket-managed local
store scopes. Use a typed enum, not free-form labels for mutation operations.

**Validate:** serde round-trip and distinct system/user identity tests.

**DoD:** the same Flatpak ref in user and system installations remains distinct.

**Do not:** probe the host yet.

### S009 — Add package record model

**Prerequisite:** S007–S008.

**Allowed files:** `src/core/package.rs`, tests.

**Implement:** add `PackageRecord` with ID, name, optional version/architecture/summary/
description/source/size, installed state, package kind, and metadata origin. Add
`UpdateRecord` (package ID, current version, candidate version, and optional update metadata)
and `SearchScope` (optional backend, installation scope, and source filters). All uncertain
metadata is optional or explicitly Unknown.

**Validate:** serialization tests including missing fields and distinct same-name records.

**DoD:** no package is identified by its display name.

**Do not:** add global version comparison or AppStream grouping.

### S010 — Add application grouping model

**Prerequisite:** S009.

**Allowed files:** `src/core/package.rs`, tests.

**Implement:** add `AppRecord` and optional `AppId` independent of `PackageId`. One app may
reference several candidates; package candidates remain individually addressable.

**Validate:** test one app mapping to RPM and Flatpak candidates without collapsing them.

**DoD:** application grouping is many-to-many and never changes install identity.

**Do not:** build search ranking.

### S011 — Add backend capability model

**Prerequisite:** S007.

**Allowed files:** `src/core/capability.rs`, tests.

**Implement:** capabilities for installed list, search, details, updates, plan, install,
remove, sources, local import, launch, and history. Add BackendInfo with availability,
version, unsupported reason, and capability set.

**Validate:** descriptors for absent, read-only, and fully available providers.

**DoD:** UI can distinguish unavailable, unsupported, and empty results.

**Do not:** infer capability from distro ID alone.

### S012 — Add transaction domain models

**Prerequisite:** S006–S011.

**Allowed files:** `src/core/transaction.rs`, tests.

**Implement:** `ChangeIntent`, `ResolvedChange`, `BackendPlan`, `BatchPlan`, `BackendOutcome`,
preview quality, job/transaction statuses, per-step outcomes, and partial-success summary.
Keep selected intent separate from solver-resolved dependency effects.

**Validate:** tests for complete preview, partial preview, unavailable preview, and
multi-backend partial failure.

**DoD:** domain model can describe dependency installs/removals and non-atomic batches.

**Do not:** implement planner/executor logic.

---

## Stage B — Tokio runtime and process execution

**Gate G1.** Paket can execute bounded asynchronous commands, stream output, cancel
explicitly, and keep Slint-independent runtime ownership.

### S013 — Add Tokio with minimal features

**Prerequisite:** S004.

**Allowed files:** `Cargo.toml`, `Cargo.lock`.

**Implement:** add Tokio 1.54.1 with only needed features (`rt-multi-thread`, `macros`,
`process`, `time`, `sync`, `io-util`, `fs`, and `signal` where supported). Add tokio-util
only when the cancellation task needs `CancellationToken`.

**Validate:** `cargo tree -e features`; `cargo check --all-targets`.

**DoD:** Tokio compiles and enabled features are documented.

**Do not:** use `full` by default or integrate Slint in this task.

### S014 — Construct a dedicated Tokio runtime

**Prerequisite:** S013.

**Allowed files:** `src/runtime/mod.rs`, `src/runtime/runtime.rs`.

**Implement:** create a runtime builder function returning a multi-thread runtime/handle.
The final startup shape is: the main OS thread constructs the Slint component and starts
the Slint event loop; a dedicated named `std::thread` owns the Tokio runtime and runs the
application coordinator with `Runtime::block_on`. UI callbacks submit commands through a
bounded Tokio `mpsc::Sender` using non-blocking `try_send`; the coordinator sends results
back through bounded events, and the UI uses `slint::invoke_from_event_loop` to update
models. Do not use `#[tokio::main]` around the Slint event loop. Add explicit startup,
shutdown, and runtime-thread join ownership.

**Validate:** a test starts the runtime owner, sends one command through the channel,
observes its result, requests shutdown, and confirms the owner thread joins.

**DoD:** the runtime lifecycle has one owner, Slint stays on the main thread, and UI callbacks
never block waiting for Tokio work.

**Do not:** start package-manager processes.

### S015 — Define structured command specifications

**Prerequisite:** S013.

**Allowed files:** `src/runtime/command.rs`.

**Implement:** `CommandSpec` with fixed executable path/name, `Vec<OsString>` argv, explicit
environment overrides, timeout, output limits, and capture mode. Never accept a shell
string or arbitrary working directory from UI input.

**Validate:** construction tests and a test proving one query containing spaces stays one
argument.

**DoD:** all invocation data is structured and typed.

**Do not:** spawn a process.

### S016 — Define async process runner port

**Prerequisite:** S015.

**Allowed files:** `src/runtime/process.rs`.

**Implement:** define one small async `ProcessRunner` interface returning a process handle
or structured completion/events. Keep backend-facing API independent of Tokio concrete
child types.

**Validate:** compile-time fake implementation.

**DoD:** providers can depend on a runner port and tests can substitute a fake.

**Do not:** add a general process framework.

### S017 — Spawn one process safely

**Prerequisite:** S016.

**Allowed files:** `src/runtime/process.rs`.

**Implement:** use `tokio::process::Command` with fixed executable and argv. Clear/sanitize
environment according to the spec, force `LC_ALL=C` for parsers that need stable English,
set stdin/stdout/stderr explicitly, and never invoke a shell.

**Validate:** harmless executable tests for success, non-zero exit, missing executable.

**DoD:** process output and exit status are returned without blocking the Slint thread.

**Do not:** implement provider-specific command builders.

### S018 — Stream stdout and stderr concurrently

**Prerequisite:** S017.

**Allowed files:** `src/runtime/process.rs`, tests.

**Implement:** drain both pipes concurrently to prevent deadlock. Emit typed line/chunk
events with source stream; preserve final partial line. Do not assume all output is UTF-8.

**Validate:** test a child that writes heavily to both streams.

**DoD:** large stdout+stderr cannot block one another.

**Do not:** parse package-manager progress here.

### S019 — Enforce output and memory bounds

**Prerequisite:** S018.

**Allowed files:** `src/runtime/process.rs`, tests.

**Implement:** define per-command stdout/stderr limits, bounded diagnostic tail, and a
streaming mode for long transactions. On hard limit, report a typed output-limit error and
terminate according to the cancellation policy.

**Validate:** test output just below and above the configured limit.

**DoD:** a noisy command cannot allocate unbounded memory.

**Do not:** truncate transaction output silently and report success.

### S020 — Add command timeouts

**Prerequisite:** S017.

**Allowed files:** `src/runtime/process.rs`, tests.

**Implement:** separate short read-command defaults from long-running transaction policy.
A timed-out read command can be terminated; a transaction timeout must request graceful
cancellation and reconcile before reporting its final state.

**Validate:** timeout tests using a sleeping child process.

**DoD:** timeout has an explicit result state and does not masquerade as success.

**Do not:** apply a 30-second timeout to package installations.

### S021 — Add explicit cancellation tokens

**Prerequisite:** S020.

**Allowed files:** `src/runtime/process.rs`, `Cargo.toml` if tokio-util is required.

**Implement:** allow a caller to request cancellation while retaining child ownership.
Queued work is cancellable without spawn; running work enters `CancelRequested` until the
child exits and is reaped.

**Validate:** tests for cancellation before spawn and while running.

**DoD:** dropping a Rust future is not treated as proof that a process stopped.

**Do not:** instantly SIGKILL a package manager.

### S022 — Add Unix process-group cancellation

**Prerequisite:** S021.

**Allowed files:** `src/runtime/process.rs`, platform-specific helper module, tests.

**Implement:** when a provider may spawn descendants, start a dedicated Unix process group
and define staged termination (graceful interrupt, bounded wait, then provider-approved
termination). Keep `Child` alive and await/reap it. Document any backend that must not be
force-killed mid-transaction.

**Validate:** test parent+child termination in a disposable helper process.

**DoD:** cancellation policy covers descendants and zombie/reaping behavior.

**Do not:** claim all native package managers are safely cancellable at every moment.

### S023 — Add deterministic fake process runner

**Prerequisite:** S016.

**Allowed files:** `tests/support/fake_runner.rs`, `tests/support/mod.rs`.

**Implement:** queue predetermined results/events, assert exact executable/argv/env, and
fail tests on unexpected commands. Keep fake support out of normal production behavior.

**Validate:** fake-runner self-test for matching and mismatch.

**DoD:** provider tests can run without installed package tools or root privileges.

**Do not:** create a general mocking library.

### S024 — Add bounded runtime event channels

**Prerequisite:** S018.

**Allowed files:** `src/runtime/events.rs`.

**Implement:** define typed `BackendEvent`/progress events and
`type EventSink = tokio::sync::mpsc::Sender<BackendEvent>`. Use bounded `mpsc` for commands
and terminal events; use a latest-value `watch` or coalescing mechanism for progress.
Never let a fast child process flood an unbounded UI queue. Terminal results must have a
reliable delivery path even when progress is coalesced.

**Validate:** tests for queue capacity and progress coalescing.

**DoD:** event delivery is bounded and terminal events cannot be silently lost.

**Do not:** add a broadcast event bus.

### S025 — Add structured logging

**Prerequisite:** S014.

**Allowed files:** `src/runtime/logging.rs`, `src/runtime/mod.rs`, `Cargo.toml`.

**Implement:** add `tracing` and a subscriber with configurable log levels and rotation or
bounded size. Redact credential-bearing URLs, authorization data, environment variables,
and sensitive helper payload fields.

**Validate:** tests for redaction and a sample log record.

**DoD:** failures can be diagnosed without recording secrets.

**Do not:** add telemetry or remote reporting.

### S026 — Define application shutdown behavior

**Prerequisite:** S014, S021, S024.

**Allowed files:** `src/app/coordinator.rs`, `src/runtime/runtime.rs`.

**Implement:** stop accepting new work, cancel read-only work, and prevent silent app exit
while a system transaction is running. Offer minimize/keep-open or wait-for-completion;
never drop the child handle and assume the package operation ended. Persist enough state
for next-launch reconciliation.

**Validate:** unit tests for shutdown with idle, queued, and active task states.

**DoD:** shutdown behavior is deterministic and documented.

**Do not:** implement the final UI dialog yet.

---

## Stage C — Batch intents, scheduler, and transaction lifecycle

**Gate G2.** Multiple clicks create a reviewable batch, and only one mutation batch can run
at a time. No backend-specific write is enabled before this gate.

### S027 — Define job and batch identifiers

**Prerequisite:** S012.

**Allowed files:** `src/core/transaction.rs`.

**Implement:** add stable `JobId`, `BatchId`, `BackendTransactionId`, and creation timestamps.
Use a small existing dependency only if already present; otherwise use a simple monotonic
or random ID strategy approved in decisions.

**Validate:** uniqueness and serialization tests.

**DoD:** every job/step has a stable traceable identity.

**Do not:** add persistence.

### S028 — Implement draft intent collection

**Prerequisite:** S027.

**Allowed files:** `src/core/planner.rs`, tests.

**Implement:** add/remove/update operations to an in-memory draft basket. A click changes
intent state only; it must not spawn a package-manager command.

**Validate:** tests for add, remove, clear, and update intent.

**DoD:** package row actions can safely create pending intent.

**Do not:** resolve dependencies.

### S029 — De-duplicate identical intents

**Prerequisite:** S028.

**Allowed files:** `src/core/planner.rs`, tests.

**Implement:** de-duplicate identical action+package+scope intents. Preserve separate IDs
when backend/source/architecture differs.

**Validate:** duplicate and distinct-source identity tests.

**DoD:** repeated install clicks cannot create duplicate native targets.

**Do not:** silently discard meaningful source differences.

### S030 — Detect contradictory intents

**Prerequisite:** S028.

**Allowed files:** `src/core/planner.rs`, tests.

**Implement:** detect install/remove or update/downgrade conflicts for the same exact
package/scope. Return a typed conflict requiring user resolution.

**Validate:** conflict matrix tests.

**DoD:** Paket never guesses which contradictory action wins.

**Do not:** invent precedence based on click order.

### S031 — Group intents by backend and scope

**Prerequisite:** S029–S030.

**Allowed files:** `src/core/planner.rs`, tests.

**Implement:** group compatible intents by `(BackendId, InstallationScope)` and preserve a
deterministic order. Keep each group's target list intact for one native solver invocation.

**Validate:** same-backend grouping and cross-backend separation tests.

**DoD:** multiple package clicks on the same backend become one transaction candidate.

**Do not:** execute groups concurrently.

### S032 — Build a cross-backend batch plan

**Prerequisite:** S031.

**Allowed files:** `src/core/planner.rs`, `src/core/transaction.rs`.

**Implement:** represent a batch as ordered backend plan steps, dependencies between steps
only when explicitly declared, and a non-atomic batch flag. Default behavior is fail-stop:
on one step's failure, mark later steps skipped pending review.

**Validate:** plan ordering and failure summary tests.

**DoD:** a multi-backend plan cannot be represented as one atomic transaction.

**Do not:** add rollback promises.

### S033 — Implement one-active-mutation coordinator

**Prerequisite:** S032.

**Allowed files:** `src/runtime/scheduler.rs`, tests.

**Implement:** FIFO queue for mutation batches with a single active writer globally.
Repeated clicks enqueue intent; they do not spawn parallel writers. Support queue position,
queued time, and explicit user cancellation while pending.

**Validate:** concurrent submit test proves max active mutations is exactly one.

**DoD:** no two Paket-owned system mutations run simultaneously.

**Do not:** add per-backend parallel mutation optimization.

### S034 — Add bounded read scheduler

**Prerequisite:** S024, S033.

**Allowed files:** `src/runtime/scheduler.rs`, tests.

**Implement:** limit discovery/search/read work to four concurrent tasks initially; make the
limit configurable internally. Prevent same-backend reads from returning an older cache
snapshot while its mutation is running.

**Validate:** concurrency counter tests.

**DoD:** read concurrency is bounded and UI requests do not create unbounded tasks.

**Do not:** run writes through this read scheduler.

### S035 — Add backend resource lock keys

**Prerequisite:** S033.

**Allowed files:** `src/core/transaction.rs`, `src/runtime/scheduler.rs`.

**Implement:** introduce internal resource keys for system package DB, per-user Flatpak
installation, system Flatpak, snapd, and Paket AppImage store. Use them for future scheduling
and state invalidation; the release default remains one global mutation writer.

**Validate:** key equality and lock ownership tests.

**DoD:** resource contention is explicit, not inferred from UI state.

**Do not:** enable parallel writes yet.

### S036 — Classify external package-manager contention

**Prerequisite:** S033, S035.

**Allowed files:** `src/runtime/scheduler.rs`, `src/core/error.rs`.

**Implement:** classify known native lock/busy responses as `Busy/WaitingForExternalManager`.
Allow cancellable wait with bounded backoff where native tool supports it. Never delete
lock files or attempt to bypass database locks.

**Validate:** fake responses for lock-held and ordinary command failure.

**DoD:** a running GNOME Software/apt/pacman/dnf transaction is shown as contention.

**Do not:** inspect and remove stale locks manually.

### S037 — Implement mutation state transitions

**Prerequisite:** S033.

**Allowed files:** `src/core/transaction.rs`, tests.

**Implement:** validate allowed transitions: Draft -> Planning -> AwaitingReview -> Queued
-> Running -> Succeeded/Failed/CancelRequested/Cancelled/NeedsReconciliation. Reject invalid
transitions.

**Validate:** transition table tests.

**DoD:** impossible states cannot be set silently.

**Do not:** add a UI-specific state machine.

### S038 — Implement batch stop-on-failure semantics

**Prerequisite:** S032, S037.

**Allowed files:** `src/runtime/scheduler.rs`, tests.

**Implement:** after a failed/unknown backend step, stop later steps, preserve successful
ones, mark the rest skipped, and request provider refresh before the user can re-plan.

**Validate:** test success -> failure -> skipped sequence.

**DoD:** partial success is represented accurately.

**Do not:** automatically retry writes.

### S039 — Reconcile interrupted transactions on next launch

**Prerequisite:** S026, S037.

**Allowed files:** `src/app/coordinator.rs`, `src/core/transaction.rs`, tests.

**Implement:** mark persisted Running/CancelRequested states as Interrupted/Unknown after
restart. Re-query provider state and show observed results. Never replay the transaction.

**Validate:** simulate persisted active task followed by app startup.

**DoD:** Paket never assumes a transaction succeeded or failed solely from stale local state.

**Do not:** run a repair or reinstall automatically.

---

## Stage D — Host detection and provider registry

**Gate G3.** Paket can explain which backends are present, absent, unsupported, or detected
but not qualified.

### S040 — Parse `/etc/os-release` safely

**Prerequisite:** S004.

**Allowed files:** `src/platform/os_release.rs`, fixtures/tests.

**Implement:** parse key/value lines and quoted values without sourcing or executing the file.
Extract ID, ID_LIKE, VERSION_ID, PRETTY_NAME, VARIANT_ID where present.

**Validate:** Fedora, Ubuntu, Debian, Arch, openSUSE and malformed fixtures.

**DoD:** parser is pure and never panics on malformed input.

**Do not:** enable providers directly from distro ID.

### S041 — Add HostProfile model

**Prerequisite:** S040.

**Allowed files:** `src/platform/host.rs`, tests.

**Implement:** represent distro family/version, architecture, immutable/atomic hints,
container/chroot hints where detectable, and qualification state.

**Validate:** serialization and unknown-host tests.

**DoD:** unsupported hosts remain representable.

**Do not:** claim distro aliases guarantee compatibility.

### S042 — Probe executable versions

**Prerequisite:** S017, S041.

**Allowed files:** `src/platform/probe.rs`, tests.

**Implement:** safe executable lookup and bounded version probes. Store absolute executable
path after validation; do not trust a UI-supplied PATH or executable.

**Validate:** fake runner for missing executable, valid version and malformed output.

**DoD:** provider can distinguish missing command from unsupported version.

**Do not:** run package operations.

### S043 — Detect D-Bus and system service availability

**Prerequisite:** S042.

**Allowed files:** `src/platform/probe.rs`, tests.

**Implement:** probe only documented service names/objects needed by DNF5, Flatpak and
snapd. A missing optional service removes only its capability.

**Validate:** fixtures/mocks for absent and present service.

**DoD:** service absence is a normal capability state.

**Do not:** add raw D-Bus calls to UI code.

### S044 — Define tested host support matrix

**Prerequisite:** S041–S043.

**Allowed files:** `docs/support-matrix.md`.

**Implement:** list distro/version/architecture/provider/minimum native-tool versions and
VM qualification status. Include unsupported/untested states and last-tested date.

**Validate:** manual review against CI VM images.

**DoD:** public support claims can be traced to tested matrix entries.

**Do not:** infer support for all RPM/DEB systems.

### S045 — Define narrow provider contracts and registry

**Prerequisite:** S011, S024, S042.

**Allowed files:** `Cargo.toml`, `src/backends/traits.rs`, `src/core/registry.rs`, tests.

**Implement:** add the small `async-trait` dependency so async provider methods remain
object-safe. Define only two provider ports: `CatalogBackend` for probe/list/search/details/
updates and `TransactionBackend` for plan/apply. Use `SearchScope`, `UpdateRecord`,
`BackendPlan`, `BackendOutcome`, `PlanError`, `OperationError`, and `EventSink` already
introduced by earlier tasks. Registry stores static `Arc<dyn ...>` provider implementations
and descriptors in deterministic order. Keep unavailable providers visible with a reason.

**Validate:** one fake read-only provider and one fake transaction provider implement the
ports; test registry lookup, missing capability, and deterministic ordering.

**DoD:** provider contracts are small and object-safe; no plugin loader or DI framework is
needed.

### S046 — Add startup provider probing coordinator

**Prerequisite:** S043–S045.

**Allowed files:** `src/app/coordinator.rs`, tests.

**Implement:** run backend probes concurrently under read concurrency limit, publish
incremental results, and isolate one provider's failure from all others.

**Validate:** fake provider test with one success and one failure.

**DoD:** one broken/missing backend cannot prevent Paket startup.

**Do not:** block startup waiting for remote repository refresh.

### S047 — Detect immutable/atomic hosts explicitly

**Prerequisite:** S041.

**Allowed files:** `src/platform/host.rs`, fixtures/tests.

**Implement:** detect known rpm-ostree/bootc-like host markers without treating them as
ordinary mutable Fedora/DNF hosts. Keep exact distro identity separate from deployment
manager.

**Validate:** Fedora Workstation vs Silverblue-like fixture tests.

**DoD:** provider selection does not expose normal DNF writes on an immutable host.

**Do not:** run `rpm-ostree` commands yet.

### S048 — Detect externally running package-manager processes

**Prerequisite:** S042.

**Allowed files:** `src/platform/probe.rs`, tests.

**Implement:** add conservative process/service hints for common package managers. Treat
this as a UX hint only; native lock errors remain authoritative.

**Validate:** process fixture tests.

**DoD:** no code assumes process scan replaces native lock handling.

**Do not:** kill or signal external package-manager processes.

---

## Stage E — DNF5 provider (Fedora / DNF5 hosts)

**Gate G4.** DNF5 has real catalog, updates, transaction resolution and mutation through a
supported authorization path.

### S049 — Create DNF5 provider skeleton

**Prerequisite:** G3.

**Allowed files:** `src/backends/mod.rs`, `src/backends/dnf5/mod.rs`.

**Implement:** create provider type and register its identity/capability placeholder.

**Validate:** compilation and registry unit test.

**DoD:** provider is isolated from DNF4 and raw RPM logic.

**Do not:** add commands.

### S050 — Probe DNF5 and supported version

**Prerequisite:** S049.

**Allowed files:** `src/backends/dnf5/probe.rs`, tests.

**Implement:** require a supported DNF5-capable mutable host, detect `dnf5`, parse version,
and detect `dnf5daemon-server` D-Bus service. Expose read and write capabilities separately.

**Validate:** Fedora fixture + missing DNF5 + missing daemon tests.

**DoD:** unsupported/minimal Fedora host has a clear capability explanation.

**Do not:** classify DNF4 as DNF5.

### S051 — Parse DNF5 installed JSON fixture

**Prerequisite:** S009, S050.

**Allowed files:** `src/backends/dnf5/parse.rs`, `tests/fixtures/dnf5/installed.json`.

**Implement:** capture actual output from the minimum supported DNF5 version. Map only
fields verified from the command's JSON schema. Preserve NEVRA/architecture identity.

**Validate:** normal, empty, missing optional field, malformed JSON tests.

**DoD:** pure parser with fixture provenance recorded.

**Do not:** invent output fields from memory.

### S052 — Implement DNF5 installed listing

**Prerequisite:** S051.

**Allowed files:** `src/backends/dnf5/catalog.rs`.

**Implement:** build verified `dnf5 list --installed --json` invocation; execute via runner;
parse using S051. Unprivileged read-only.

**Validate:** fake-runner exact argv/env and result tests; manual Fedora check.

**DoD:** listing distinguishes successful empty list from command failure.

**Do not:** parse terminal tables.

### S053 — Parse DNF5 repoquery records

**Prerequisite:** S051.

**Allowed files:** `src/backends/dnf5/parse.rs`, query fixtures.

**Implement:** verify `repoquery --queryformat` against installed DNF5 help/man page. Use
stable fields such as full NEVRA, repo ID, name, EVR and architecture with strict delimiter
handling.

**Validate:** real captured fixture plus malformed/duplicate candidate tests.

**DoD:** parser preserves multiple repository candidates with same name.

**Do not:** assume repoquery supports JSON unless the supported version proves it.

### S054 — Implement DNF5 search

**Prerequisite:** S053.

**Allowed files:** `src/backends/dnf5/catalog.rs`.

**Implement:** search with verified repoquery argv. Query remains one argv element; preserve
candidate ID and repo provenance.

**Validate:** fake-runner tests for query containing spaces, no results and command failure.

**DoD:** search cannot turn user query text into command syntax.

**Do not:** deduplicate by package display name.

### S055 — Implement DNF5 package details

**Prerequisite:** S052–S054.

**Allowed files:** `src/backends/dnf5/catalog.rs`, parser/tests.

**Implement:** use verified info/repoquery fields for description, repo, license, URL, size
and dependency info where actually available. Decode only validated DNF5 PackageIds.

**Validate:** wrong backend, wrong architecture, removed package and partial metadata tests.

**DoD:** details return provenance and optional fields without fabricated defaults.

**Do not:** enrich from unverified web metadata.

### S056 — Implement DNF5 update listing

**Prerequisite:** S051.

**Allowed files:** `src/backends/dnf5/catalog.rs`, parser/tests.

**Implement:** capture and parse the supported DNF5 JSON upgrade output (or documented
repoquery upgrade output if more reliable). Keep current and candidate version strings
opaque.

**Validate:** empty, normal, malformed fixtures.

**DoD:** successful no-updates differs from update-query failure.

**Do not:** globally compare DNF versions with other providers.

### S057 — Implement DNF5 repository listing

**Prerequisite:** S050.

**Allowed files:** `src/backends/dnf5/catalog.rs`, parser/tests.

**Implement:** use documented `dnf5 repo list/info --json` output for ID, name, enabled
state and verified optional fields.

**Validate:** enabled/disabled/malformed fixture tests.

**DoD:** source records preserve native repository ID.

**Do not:** add/edit repository config.

### S058 — Add zbus DNF5 daemon session client

**Prerequisite:** S050, S013.

**Allowed files:** `src/backends/dnf5/daemon.rs`, `Cargo.toml`.

**Implement:** add zbus with Tokio support; open/close DNF5 daemon sessions; set bounded
method timeouts; map missing service and D-Bus errors to typed backend errors.

**Validate:** mock/session tests or a Fedora VM smoke test.

**DoD:** D-Bus lifetime and error handling are isolated from catalog parsing.

**Do not:** implement transaction resolution yet.

### S059 — Resolve DNF5 install/remove plans

**Prerequisite:** S058, S012.

**Allowed files:** `src/backends/dnf5/daemon.rs`, `src/backends/dnf5/plan.rs`.

**Implement:** open a session, add exact package specs to the documented DNF5 Goal/RPM
interface, call resolve, and map the returned native transaction list to `BackendPlan`.
Keep dependencies, removals, upgrades, and solver problems. Store a server/session token
only for the approved plan lifecycle.

**Validate:** Fedora VM plan for one package, dependency package, conflict and no-op.

**DoD:** preview is native and complete only when the daemon reports a complete resolution.

**Do not:** execute the transaction.

### S060 — Execute DNF5 plan with progress and authorization

**Prerequisite:** S059.

**Allowed files:** `src/backends/dnf5/daemon.rs`, `src/backends/dnf5/progress.rs`.

**Implement:** after explicit review, call documented transaction method; subscribe to
transaction start/action progress/script events and completion; use polkit authorization
provided by the daemon. Reconcile final installed state after success/failure.

**Validate:** Fedora VM install/remove, authorization denied, external lock, and interrupted
client tests.

**DoD:** UI receives structured transaction progress and final outcome.

**Do not:** add a second Paket root helper path for the same DNF5 operation.

---

## Stage F — Flatpak provider

**Gate G5.** Flatpak apps and runtimes can be listed, searched, planned, installed, removed,
and distinguished by installation scope.

### S061 — Probe Flatpak and enumerate installations

**Prerequisite:** G3.

**Allowed files:** `src/backends/flatpak/probe.rs`.

**Implement:** detect `flatpak`, supported version, user installation and configured system
installations. Report missing CLI separately from missing remotes.

**Validate:** fake-runner fixtures and manual host probe.

**DoD:** no Flatpak installation does not break other providers.

**Do not:** assume only one system installation exists.

### S062 — Parse Flatpak refs and identity

**Prerequisite:** S008, S061.

**Allowed files:** `src/backends/flatpak/parse.rs`.

**Implement:** parse full ref (kind, app/runtime ID, architecture, branch), remote, and
installation identity. Use a verified machine-readable/column output supported by the
minimum Flatpak version.

**Validate:** app/runtime and same-ID multi-branch fixtures.

**DoD:** Flatpak identity includes scope and full native ref.

**Do not:** target a package with only its app display name.

### S063 — List installed Flatpaks

**Prerequisite:** S062.

**Allowed files:** `src/backends/flatpak/catalog.rs`.

**Implement:** list installed apps and runtimes across user/system/named installations;
keep them distinct and use noninteractive read commands.

**Validate:** fake-runner and manual host comparison.

**DoD:** all installation scopes appear as separate records.

**Do not:** merge apps and runtimes.

### S064 — List Flatpak remotes

**Prerequisite:** S061.

**Allowed files:** `src/backends/flatpak/catalog.rs`, parser/tests.

**Implement:** query remote name, URL where safe, collection ID, enabled state, and owning
installation. Redact credentials from URLs in logs and UI.

**Validate:** multiple remote and credential-bearing URL tests.

**DoD:** remote identity is scoped to its installation.

**Do not:** add or trust remotes.

### S065 — Implement Flatpak search

**Prerequisite:** S062–S064.

**Allowed files:** `src/backends/flatpak/catalog.rs`, parser/tests.

**Implement:** search apps/runtimes by query and remote; include source/installation details
where output exposes them. Treat ambiguous results as separate candidates.

**Validate:** duplicate ID across remotes/branches fixtures.

**DoD:** search results preserve their full refs and remotes.

**Do not:** choose the first ambiguous remote automatically.

### S066 — Implement Flatpak details and updates

**Prerequisite:** S063–S065.

**Allowed files:** `src/backends/flatpak/catalog.rs`, parser/tests.

**Implement:** details for an exact ref and list of available updates by installation.
Capture update commit/version info where available.

**Validate:** details, no updates, same ID different branch tests.

**DoD:** updates can be listed without claiming cross-backend version ordering.

**Do not:** run update operations yet.

### S067 — Build Flatpak install/remove plans

**Prerequisite:** S066, S012.

**Allowed files:** `src/backends/flatpak/plan.rs`.

**Implement:** exact remote/ref/scope plan commands or supported transaction preview. A plan
must explicitly say if dependency effects are unavailable or partial.

**Validate:** wrong scope/ref/remote and no-op plan tests.

**DoD:** plan cannot switch system/user scope silently.

**Do not:** include `--delete-data` by default.

### S068 — Apply Flatpak install/remove

**Prerequisite:** S067.

**Allowed files:** `src/backends/flatpak/transaction.rs`.

**Implement:** apply approved native operation with explicit `--user`, `--system`, or named
installation, fixed args, noninteractive/assumeyes behavior only after user approval, and
streamed output. Use Flatpak's native system helper/polkit authorization for system scope.

**Validate:** VM install/remove on user and system scopes; authorization denied.

**DoD:** a click produces one scoped transaction with result reconciliation.

**Do not:** directly call Flatpak's private/privileged system helper D-Bus interface.

### S069 — Add explicit Flatpak app-data removal choice

**Prerequisite:** S068.

**Allowed files:** `src/backends/flatpak/transaction.rs`, tests.

**Implement:** model app-data deletion as a separate optional destructive intent; default
must preserve `~/.var/app` data and permissions.

**Validate:** command builder defaults to preserving data and only adds data deletion when
explicitly requested.

**DoD:** normal uninstall never silently deletes user data.

**Do not:** add “clean everything” defaults.

### S070 — Parse Flatpak progress/result output

**Prerequisite:** S068.

**Allowed files:** `src/backends/flatpak/progress.rs`.

**Implement:** parse documented/stable progress output if possible; otherwise emit
indeterminate progress plus current phase. Terminal result must be authoritative.

**Validate:** fixtures for progress, warning, failure, and no progress lines.

**DoD:** UI never displays fabricated percentage.

**Do not:** infer percentage from elapsed time.

### S071 — Add Flatpak provider contract tests

**Prerequisite:** S063–S070.

**Allowed files:** `tests/providers/flatpak.rs`, fixtures.

**Implement:** test public provider methods with fake runner and at least one real Flatpak
VM run for install/remove/update.

**Validate:** `cargo test flatpak`.

**DoD:** full contract passes without Flatpak installed for fixture tests; VM evidence recorded.

**Do not:** begin APT before this gate passes.

---

## Stage G — Privileged helper foundation

**Gate G6.** The typed privileged-helper boundary is implemented and tested before any
APT, Pacman, Zypper, or rpm-ostree write operation is enabled. Provider-specific command
builders are still implemented later, one backend at a time.

### S072 — Define the typed helper protocol

**Prerequisite:** S039.

**Allowed files:** `src/helper/mod.rs`, `src/helper/protocol.rs`, protocol tests.

**Implement:**

1. Define a versioned request envelope with a request ID and one typed operation.
2. Define operation variants for APT refresh/install/remove/upgrade, Pacman full upgrade/install/remove,
   Zypper install/remove/update, and rpm-ostree install/uninstall/upgrade-deployment.
3. Requests contain backend-native package IDs and operation-specific safe options only.
   They must not contain executable paths, arbitrary argv, environment variables, working
   directories, shell strings, or caller-selected output paths.
4. Define bounded JSONL response events: accepted, started, output/progress, completed,
   failed, and cancelled. Use a protocol version and reject unknown versions.
5. Keep helper protocol types independent from Tokio child-process types.

**Validate:** JSON round-trip tests, unknown-version/operation tests, and a test proving
that the request schema cannot represent an executable or arbitrary argument vector.

**DoD:** the helper wire protocol can express only declared package-management operations.

**Do not:** implement command execution or any provider-specific command builder yet.

### S073 — Validate helper requests independently

**Prerequisite:** S072.

**Allowed files:** `src/helper/validation.rs`, validation tests.

**Implement:**

1. Validate protocol version, request size, request ID, operation/backend compatibility,
   non-empty targets, target count limits, and operation-specific option combinations.
2. Reject control characters, path traversal, NUL bytes, unexpected option-like values,
   unsupported scopes, and identifiers that fail each backend's documented grammar.
3. Treat all client-side validation as advisory; repeat validation in the privileged helper.
4. Bind the request to the authenticated local caller where the platform mechanism exposes
   a trustworthy peer UID. Do not trust a UID field supplied in JSON.
5. Return stable validation error codes without echoing sensitive request contents.

**Validate:** adversarial IDs, malformed scope, oversized request, mismatched backend/action,
empty target, and spoofed caller UID tests.

**DoD:** malformed requests fail closed before any privileged command can be selected.

**Do not:** spawn a process or trust UI validation.

### S074 — Add the fixed helper command dispatcher

**Prerequisite:** S073.

**Allowed files:** `src/helper/commands/mod.rs`, `src/helper/commands/types.rs`, tests.

**Implement:**

1. Define an internal `HelperCommand` containing a fixed absolute executable and structured
   `OsString` arguments. This type is created only inside the helper process.
2. Add one explicit dispatcher matching typed protocol operations to backend-specific builder
   functions. A backend operation without an implemented builder returns `Unsupported`.
3. Do not create a general “run command” operation or a public constructor for arbitrary
   executable/argv input.
4. Provider-specific builders are added only in their own later tasks: APT refresh in S084,
   APT install/remove/upgrade in S086 (executed by the transaction flow in S088), Pacman
   system install/update in S097 and removal/AUR-artifact installation in S098/S105,
   Zypper in S112, and rpm-ostree in S118.
5. Add a test-only constructor behind `cfg(test)` so command dispatch can be validated
   without root privileges.

**Validate:** tests for unsupported builder behavior, fixed executable mapping, and proof
that serialized requests cannot populate `HelperCommand` directly.

**DoD:** all privileged process execution must pass through one typed dispatcher; no generic
root command surface exists.

**Do not:** implement APT/Pacman/Zypper/rpm-ostree command syntax in this task.

### S075 — Implement bounded helper process execution and streaming

**Prerequisite:** S074.

**Allowed files:** `src/bin/helper.rs`, `src/helper/runner.rs`, helper tests.

**Implement:**

1. Read one bounded JSONL request from stdin and reject additional/unbounded payload data.
2. Convert the validated request to one `HelperCommand` through the dispatcher.
3. Spawn only the fixed command via `std::process::Command` with argv arrays; never use a
   shell. Clear the environment and set only the minimal required locale/path variables.
4. Drain stdout and stderr concurrently; emit bounded JSONL events; cap line length, total
   output and retained diagnostic tail.
5. Return a terminal event with exit code and typed classification. Keep the child alive
   until waited/reaped; never treat disconnect as proof the process stopped.

**Validate:** fake-command tests for large output, malformed request, non-zero exit, broken
pipe, client disconnect, and child reaping.

**DoD:** the helper executes only dispatched operations and cannot be flooded with unbounded
input/output.

**Do not:** add arbitrary command support or provider-specific transaction parsing.

### S076 — Implement the unprivileged helper client

**Prerequisite:** S075.

**Allowed files:** `src/helper/client.rs`, client tests.

**Implement:**

1. Resolve the helper from one compile-time/package-configured absolute path; never search
   `$PATH` and never accept a path from the UI/request.
2. Invoke the fixed helper through `pkexec` with no package target in pkexec argv; send the
   typed request through stdin.
3. Parse bounded versioned JSONL events and classify authorization denied/cancelled,
   helper unavailable, protocol mismatch, and native operation failure separately.
4. Support Tokio cancellation without silently dropping the helper child; await/reap it and
   return the final known state.
5. Expose a small async client API to provider adapters.

**Validate:** mocked helper success/failure/auth-cancel/protocol-mismatch tests and a test
that no caller-supplied executable path is used.

**DoD:** the application can request only protocol-defined privileged operations.

**Do not:** install permissive polkit rules or grant passwordless access.

### S077 — Define and verify the installed helper path contract

**Prerequisite:** S076.

**Allowed files:** `src/helper/client.rs`, `docs/security.md`, path tests.

**Implement:**

1. Define the package-configured absolute path contract for each supported packaging
   family; if paths differ, inject the path at build/package time, not runtime PATH lookup.
2. Before invocation, verify the path is absolute and inspect metadata without following
   unexpected symlinks. The final owner/mode check is enforced by packaging and rechecked
   in the release VM task.
3. Ensure no user-writable directory is used for the production helper binary.
4. Keep development/test helper paths explicitly separate from production configuration.

**Validate:** tests for relative path rejection, missing path, unexpected symlink, and
untrusted directory metadata.

**DoD:** production helper resolution is deterministic and cannot fall back to a user-owned
executable found on PATH.

**Do not:** ship a setuid helper or add passwordless polkit policy.

### S078 — Add helper boundary and adversarial protocol tests

**Prerequisite:** S077.

**Allowed files:** `tests/helper/`, `src/helper/` test-only code, `docs/security.md`.

**Implement:**

1. Exercise the complete unprivileged-client → protocol → validation → command-dispatch
   path with fake command builders and no root access.
2. Test unknown operations, wrong backend, argument injection attempts, oversized payloads,
   malformed JSONL, output flooding, client disconnect, symlink/path substitution, and
   authorization cancellation.
3. Assert that no test path can reach a shell or arbitrary executable.
4. Record the helper trust boundary and remaining provider-specific risks in the security doc.

**Validate:** `cargo test helper` plus a source scan for shell invocation and unbounded reads.

**DoD:** helper protocol and dispatch boundary are covered by adversarial tests before any
provider uses the helper.

**Do not:** claim the whole application security review is complete; final threat-model
review remains in S165.

---

## Stage H — APT / dpkg provider

**Gate G7.** APT can list, search, simulate, install, remove, and report progress without
bypassing dpkg or lock protections.

### S079 — Probe APT/dpkg and version support

**Prerequisite:** G3.

**Allowed files:** `src/backends/apt/probe.rs`.

**Implement:** detect `apt-get`, `apt-cache`, `dpkg-query`, architecture and supported APT
version. Probe lock ownership conservatively; do not attempt to acquire a write lock.

**Validate:** Debian/Ubuntu and missing-tool fixtures.

**DoD:** APT read/write capability depends on actual toolset and host qualification.

**Do not:** use `apt` human-oriented output as a stable parser.

### S080 — Parse dpkg-query installed records

**Prerequisite:** S009, S079.

**Allowed files:** `src/backends/apt/parse.rs`, fixtures.

**Implement:** capture `dpkg-query` output with explicit `--showformat` fields for package,
version, architecture, and status; handle multiarch and non-installed states.

**Validate:** normal, multiarch, config-files-only, malformed record fixtures.

**DoD:** only installed/desired package states are marked installed.

**Do not:** split on whitespace where fields can contain spaces.

### S081 — Implement APT installed listing

**Prerequisite:** S080.

**Allowed files:** `src/backends/apt/catalog.rs`.

**Implement:** execute exact dpkg-query argv with bounds and map rows to package identity.

**Validate:** fake-runner assertions and Debian VM comparison.

**DoD:** installed list does not require root.

**Do not:** call dpkg install/remove.

### S082 — Implement APT search

**Prerequisite:** S079.

**Allowed files:** `src/backends/apt/catalog.rs`, parser/tests.

**Implement:** use apt-cache search output with locale pinned and strict parsing; if a
machine-readable alternative is used, document it and test minimum supported versions.

**Validate:** query, no result, names with punctuation, malformed output.

**DoD:** query remains a single argv element and result source is known.

**Do not:** install from search result display text.

### S083 — Implement APT package details

**Prerequisite:** S081–S082.

**Allowed files:** `src/backends/apt/catalog.rs`, parser/tests.

**Implement:** retrieve installed and candidate metadata from apt-cache/dpkg-query. Keep
candidate version, installed version, architecture and repository origin separate.

**Validate:** installed/available/missing candidate tests.

**DoD:** detail view can explain installed versus candidate versions.

**Do not:** compare Debian version strings in shared core.

### S084 — Refresh APT metadata

**Prerequisite:** S083, S072–S078.

**Allowed files:** `src/backends/apt/refresh.rs`, `src/helper/commands/apt.rs`, tests.

**Implement:** add a fixed APT metadata-refresh command builder and execute refresh through
the typed helper. Parse APT status-FD records where feasible and report repository errors.
Do not refresh automatically on every search or app startup.

**Validate:** VM refresh with working and failing repository fixture.

**DoD:** metadata freshness and refresh failure are visible.

**Do not:** continue as if metadata is fresh after a failed refresh.

### S085 — Implement APT updates listing

**Prerequisite:** S084.

**Allowed files:** `src/backends/apt/catalog.rs`, parser/tests.

**Implement:** query upgradable package candidates using a version-appropriate machine-safe
or strict command output. Include held packages and phasing/eligibility states where the
native source exposes them.

**Validate:** upgradable, held, no update, and apt failure fixtures.

**DoD:** “no updates” is distinct from “update query failed”.

**Do not:** assume all candidates are installable.

### S086 — Add APT transaction command builders

**Prerequisite:** S072–S078, S083.

**Allowed files:** `src/helper/commands/apt.rs`, `src/helper/commands/mod.rs`, tests.

**Implement:** add fixed APT command builders for install/remove/upgrade. Accept only
validated package IDs and explicit safe operation options; use a fixed `apt-get` executable
and an exact argv allowlist. Do not accept raw argv, a command string, a user-selected
executable, or unsafe flags such as unauthenticated package installation.

**Validate:** exact argv snapshot tests, malformed ID rejection, empty target rejection,
and a test for a package name beginning with `-`.

**DoD:** approved APT operations map to fixed commands and cannot express arbitrary root
execution.

**Do not:** implement the APT helper process or bypass the native solver.

### S087 — Simulate APT install/remove plans

**Prerequisite:** S083.

**Allowed files:** `src/backends/apt/plan.rs`, fixtures/tests.

**Implement:** run `apt-get -s` simulations with reviewed options and parse dependency,
removal, upgrade and conflict effects. Do not modify installed state during simulation.

**Validate:** dependency install, package removal causing extra removals, no-op, malformed
simulation fixtures.

**DoD:** complete preview is declared only if the simulation parser covers the full output.

**Do not:** use `--fix-broken` or bypass dependency checks.

### S088 — Execute APT transactions with status-fd

**Prerequisite:** S087, S072–S078, S086.

**Allowed files:** `src/backends/apt/transaction.rs`.

**Implement:** run approved `apt-get` operations through typed helper. Use `APT::Status-Fd`
for machine progress events and bounded diagnostic stderr. Treat conffile decisions as an
explicit policy, not an implicit overwrite.

**Validate:** Debian VM install/remove, package scripts, conffile prompt fixture, auth denied.

**DoD:** progress and final outcome are based on APT events/exit status.

**Do not:** invoke `dpkg -i` as default install path.

### S089 — Handle APT/dpkg locks and conffiles

**Prerequisite:** S088.

**Allowed files:** `src/backends/apt/transaction.rs`, tests.

**Implement:** recognize lock-held responses, honor native lock wait support where
available, and implement documented conffile behavior that preserves local configuration
by default. Show when a conffile choice needs user input and do not hang on hidden prompts.

**Validate:** lock contention and conffile fixture tests.

**DoD:** lock contention is Busy/Waiting; local config is not silently discarded.

**Do not:** remove lock files or pass force-confnew.

### S090 — Add APT provider contract and VM tests

**Prerequisite:** S081–S089.

**Allowed files:** `tests/providers/apt.rs`, VM scripts.

**Implement:** run provider contract tests on disposable Debian and Ubuntu images, including
refresh, plan, install, remove, cancellation/reconcile, and lock failure.

**Validate:** `cargo test apt`; VM test report.

**DoD:** no write path is advertised unless its supported VM cases pass.

**Do not:** start pacman implementation before this gate passes.

---

## Stage I — Pacman and AUR providers

**Gate G8.** Arch operations cannot create partial upgrades; AUR execution requires explicit
source review and never runs as root.

### S091 — Probe Pacman and Arch host

**Prerequisite:** G3.

**Allowed files:** `src/backends/pacman/probe.rs`.

**Implement:** detect pacman version, architecture, pacman.conf path, sync DB state hints,
and optional pacman-contrib/checkupdates availability.

**Validate:** Arch fixture and missing-helper fixture.

**DoD:** host capability state is precise.

**Do not:** enable writes on Arch derivatives without qualification.

### S092 — Parse Pacman installed query

**Prerequisite:** S009, S091.

**Allowed files:** `src/backends/pacman/parse.rs`, fixtures.

**Implement:** parse `pacman -Q` and `pacman -Qi` output with strict record boundaries;
model package name/version/architecture/reason as available.

**Validate:** installed, local package, malformed and empty fixtures.

**DoD:** package identity is stable and pure parser tests pass.

**Do not:** parse localized progress output here.

### S093 — Implement Pacman installed listing

**Prerequisite:** S092.

**Allowed files:** `src/backends/pacman/catalog.rs`.

**Implement:** list installed packages unprivileged with exact command arguments.

**Validate:** fake-runner and Arch VM comparison.

**DoD:** installed listing works without root.

**Do not:** refresh sync databases.

### S094 — Implement Pacman repository search/details

**Prerequisite:** S093.

**Allowed files:** `src/backends/pacman/catalog.rs`, parser/tests.

**Implement:** search configured repositories and retrieve info with stable output handling.
Preserve repository and architecture candidate identity.

**Validate:** same-name candidates from multiple repos and no-result tests.

**DoD:** package candidates remain separate by source/arch/version.

**Do not:** treat AUR results as pacman repo results.

### S095 — Implement safe Pacman update listing

**Prerequisite:** S091.

**Allowed files:** `src/backends/pacman/catalog.rs`, parser/tests.

**Implement:** prefer `checkupdates` from pacman-contrib because it uses an isolated sync
database; if unavailable, clearly report update-check limitation rather than running unsafe
`pacman -Sy` alone. Keep package DB state explicit.

**Validate:** installed, update available, helper missing, DB busy fixtures.

**DoD:** update listing never creates a partial-upgrade state.

**Do not:** run `pacman -Sy` as an update check.

### S096 — Design Pacman install/update transaction plan

**Prerequisite:** S094–S095, S012.

**Allowed files:** `src/backends/pacman/plan.rs`.

**Implement:** require full-upgrade semantics (`pacman -Syu` with targets) for repository
install/update operations. Capture all available target/dependency effects using a
qualified resolver path. If exact preview cannot be made complete, declare Estimated/Partial
and show an explicit full-system-upgrade warning.

**Validate:** fixture for target install plus unrelated system upgrades.

**DoD:** user cannot accidentally request a partial system upgrade from Paket.

**Do not:** use `pacman -Sy package` or hide unrelated upgrades.

### S097 — Execute approved Pacman system transaction

**Prerequisite:** S096, S072–S078.

**Allowed files:** `src/backends/pacman/transaction.rs`, `src/helper/commands/pacman.rs`, tests.

**Implement:** run a reviewed full-upgrade operation through typed helper. Ensure confirmation
behavior cannot accept an unreviewed destructive solver change. Use command options only
after validating supported pacman behavior on target Arch versions.

**Validate:** Arch VM install, upgrade, conflict, signature failure and cancellation cases.

**DoD:** full-upgrade constraint is enforced by command builder and tests.

**Do not:** silently downgrade, skip dependencies, or bypass signature verification.

### S098 — Implement Pacman remove planning/apply

**Prerequisite:** S093, S097.

**Allowed files:** `src/backends/pacman/plan.rs`, `src/backends/pacman/transaction.rs`, `src/helper/commands/pacman.rs`, tests.

**Implement:** preview reverse dependencies and remove effects; support recursive removal
only when explicitly reviewed. Keep removal of required packages blocked by native solver.

**Validate:** no dependency, dependent packages, protected package, malformed target tests.

**DoD:** remove plan lists dependency effects or declares preview incomplete.

**Do not:** use `-Rdd` / dependency-check bypass.

### S099 — Handle Pacman lock and external transactions

**Prerequisite:** S097.

**Allowed files:** `src/backends/pacman/transaction.rs`, tests.

**Implement:** classify database lock contention, wait only through native/verified
mechanisms, and re-probe after external pacman activity.

**Validate:** lock-held and recovery fixture tests.

**DoD:** Paket never removes `/var/lib/pacman/db.lck`.

**Do not:** kill external pacman processes.

### S100 — Implement AUR RPC search/details

**Prerequisite:** S091.

**Allowed files:** `src/backends/aur/client.rs`, parser/tests.

**Implement:** use the documented AUR RPC v5 endpoints; add request timeout, bounded
response size, rate-limit/backoff behavior, and schema-version handling. Treat API data as
untrusted metadata.

**Validate:** captured search/info fixtures, HTTP error, rate limit and invalid JSON tests.

**DoD:** read-only AUR catalog works without executing any PKGBUILD.

**Do not:** clone/build packages in this task.

### S101 — Download and pin AUR source revision

**Prerequisite:** S100.

**Allowed files:** `src/backends/aur/source.rs`.

**Implement:** clone/fetch the selected AUR git repo into Paket's build workspace, pin the
reviewed commit hash, record branch/commit and reject unexpected path/symlink behavior.

**Validate:** local test repository fixture with revision changes.

**DoD:** build input is immutable for the reviewed build session.

**Do not:** execute PKGBUILD content.

### S102 — Present PKGBUILD diff for explicit review

**Prerequisite:** S101.

**Allowed files:** `src/backends/aur/review.rs`.

**Implement:** show PKGBUILD and file changes from the last reviewed/current AUR revision;
make source URLs, checksums and validpgpkeys visible. Require a separate explicit
confirmation before build.

**Validate:** diff fixture and missing-prior-revision test.

**DoD:** no AUR build begins without review acknowledgement.

**Do not:** treat signature presence as proof of safe code.

### S103 — Build AUR package as the user

**Prerequisite:** S102.

**Allowed files:** `src/backends/aur/build.rs`.

**Implement:** invoke `makepkg` under the logged-in user, never through the root helper;
use an isolated per-build directory, bounded logs, timeout/cancel handling, and a stable
build environment record. Require user approval for make dependencies.

**Validate:** disposable Arch VM build of a harmless local fixture package.

**DoD:** process UID is not root and logs/revision are recorded.

**Do not:** invoke `sudo makepkg` or `makepkg` as root.

### S104 — Inspect AUR build artifact

**Prerequisite:** S103.

**Allowed files:** `src/backends/aur/artifact.rs`.

**Implement:** inspect output package metadata with read-only pacman tooling; verify
expected package name/version/architecture and checksum. Reject unexpected/missing output.

**Validate:** correct, mismatched, multiple output and corrupt artifact fixtures.

**DoD:** only a verified build artifact is eligible for install.

**Do not:** install yet.

### S105 — Install reviewed AUR artifact

**Prerequisite:** S104, S097.

**Allowed files:** `src/backends/aur/transaction.rs`, helper protocol.

**Implement:** stage the artifact safely for the helper (root-owned copy or securely opened
file descriptor; never trust a mutable user path after authorization), then install via
Pacman's native local-package operation while preserving full-upgrade policy and signature
checks where applicable.

**Validate:** Arch VM install, staged-file replacement race test, invalid package test.

**DoD:** helper cannot be tricked into installing an arbitrary path or changed file.

**Do not:** let the helper accept any filesystem path from the UI.

### S106 — Add AUR build and install contract tests

**Prerequisite:** S100–S105.

**Allowed files:** `tests/providers/aur.rs`, fixtures/VM scripts.

**Implement:** cover review required, diff, unprivileged build, failure, artifact verification,
install, cancellation and log retention.

**Validate:** `cargo test aur`; disposable Arch VM run.

**DoD:** AUR remains disabled unless every safety gate is passed.

**Do not:** add automatic AUR update-all.

### S107 — Add Pacman/AUR combined Arch VM qualification

**Prerequisite:** S093–S106.

**Allowed files:** `tests/vm/arch/`, `docs/support-matrix.md`.

**Implement:** qualify read, update listing, safe full-upgrade install, remove, AUR review,
build and install on disposable Arch image.

**Validate:** run VM suite and record exact Arch/pacman version.

**DoD:** support matrix records pass/fail evidence and known limitations.

**Do not:** advertise all Arch derivatives based on one Arch VM.

---

## Stage J — Zypper and rpm-ostree providers

**Gate G9.** Separate mutable RPM and immutable deployment semantics are handled correctly.

### S108 — Probe Zypper host

**Prerequisite:** G3.

**Allowed files:** `src/backends/zypper/probe.rs`.

**Implement:** detect `zypper`, RPM/SUSE host markers, version and supported distro list.

**Validate:** openSUSE and non-SUSE fixtures.

**DoD:** Zypper capability does not imply DNF support.

**Do not:** use raw RPM as a transaction backend.

### S109 — Parse Zypper installed/search records

**Prerequisite:** S108, S009.

**Allowed files:** `src/backends/zypper/parse.rs`, fixtures.

**Implement:** use a verified structured output mode if supported; otherwise fixture and
strictly parse stable `LC_ALL=C` output. Preserve repo, vendor, arch, version and package
status when exposed.

**Validate:** installed/search/info and malformed fixtures.

**DoD:** parser records exactly which Zypper version/output it supports.

**Do not:** reuse DNF parser.

### S110 — Implement Zypper catalog

**Prerequisite:** S109.

**Allowed files:** `src/backends/zypper/catalog.rs`.

**Implement:** installed list, search, details, repository listing and update listing.
Include patch/update distinction where native commands expose it.

**Validate:** fake-runner tests and openSUSE VM read checks.

**DoD:** installed/search/update states are distinct and source identity is preserved.

**Do not:** enable mutations.

### S111 — Implement Zypper dry-run planning

**Prerequisite:** S110, S012.

**Allowed files:** `src/backends/zypper/plan.rs`.

**Implement:** use `--dry-run` for supported operations and capture dependency/vendor/repo
changes. Mark preview complete only if all effects are parsed. Do not set license auto-agree
or vendor-change options without a separate explicit user policy.

**Validate:** package install/remove, dependency, vendor change and no-op fixtures.

**DoD:** preview exposes resolver effects and required user decisions.

**Do not:** auto-accept licenses/vendor switches.

### S112 — Execute Zypper transactions

**Prerequisite:** S111, S072–S078.

**Allowed files:** `src/backends/zypper/transaction.rs`, `src/helper/commands/zypper.rs`, tests.

**Implement:** typed helper install/remove/update with verified noninteractive behavior only
after preview review; stream progress; capture exit state and repository failures.

**Validate:** openSUSE VM transaction tests and lock contention.

**DoD:** mutation uses Zypper, not raw rpm, and respects review choices.

**Do not:** skip signature verification.

### S113 — Add Zypper contract tests

**Prerequisite:** S110–S112.

**Allowed files:** `tests/providers/zypper.rs`, VM scripts.

**Implement:** read, dry-run, install, remove, update, lock and vendor-change scenarios.

**Validate:** `cargo test zypper`; openSUSE VM suite.

**DoD:** support matrix includes exact qualified release/version.

**Do not:** enable provider on untested SUSE versions.

### S114 — Probe rpm-ostree / atomic deployments

**Prerequisite:** S047.

**Allowed files:** `src/backends/rpm_ostree/probe.rs`.

**Implement:** detect rpm-ostree and booted/pending deployment state with documented
read-only status command/API.

**Validate:** mutable Fedora vs Silverblue/Kinoite-like VM fixtures.

**DoD:** rpm-ostree is independent from DNF5 provider.

**Do not:** run layering commands.

### S115 — Parse rpm-ostree deployment status

**Prerequisite:** S114.

**Allowed files:** `src/backends/rpm_ostree/parse.rs`, fixtures.

**Implement:** map booted deployment, pending deployment, layered packages, base image,
version and reboot-required state.

**Validate:** no pending, pending, rollback, malformed fixtures.

**DoD:** UI can explain active vs pending system state.

**Do not:** label pending as installed in the active system.

### S116 — List layered packages and updates

**Prerequisite:** S115.

**Allowed files:** `src/backends/rpm_ostree/catalog.rs`.

**Implement:** list layered/requested packages and available update/deployment state using
supported rpm-ostree commands. Distinguish base image content from layered additions.

**Validate:** Silverblue VM read comparisons.

**DoD:** layered/base distinction is preserved.

**Do not:** expose normal DNF package install actions.

### S117 — Plan rpm-ostree package layering

**Prerequisite:** S116, S012.

**Allowed files:** `src/backends/rpm_ostree/plan.rs`.

**Implement:** create typed plan for install/remove/upgrade with pending deployment and
reboot-required effect. If operations are chainable, show the combined pending state.

**Validate:** install/remove and pending deployment fixture tests.

**DoD:** plan clearly states that changes may require reboot.

**Do not:** claim live install semantics.

### S118 — Apply rpm-ostree changes

**Prerequisite:** S117, S072–S078.

**Allowed files:** `src/backends/rpm_ostree/transaction.rs`, `src/helper/commands/rpm_ostree.rs`, tests.

**Implement:** run approved `rpm-ostree` operations through the supported authorization
path; reconcile pending deployment after completion.

**Validate:** disposable atomic VM install/remove, failure and reboot-required result.

**DoD:** active and pending deployments remain distinguishable.

**Do not:** automatically reboot.

### S119 — Qualify RPM provider matrix

**Prerequisite:** S052–S060, S110–S118.

**Allowed files:** `tests/vm/rpm/`, `docs/support-matrix.md`.

**Implement:** qualify Fedora DNF5, openSUSE Zypper, and rpm-ostree separately. DNF4 is outside the first release; add it only through a separate provider task pack for named distro/version targets.

**Validate:** VM suite and release support review.

**DoD:** “RPM support” is broken down into actual provider/version support.

**Do not:** claim generic RPM write support.

---

## Stage K — Snap and managed AppImage providers

**Gate G10.** Snap works only when snapd is available; AppImage import/update/launch is safe
and remains local to the user.

### S120 — Probe snapd

**Prerequisite:** G3.

**Allowed files:** `src/backends/snap/probe.rs`.

**Implement:** detect Snap CLI, snapd service/socket health, version and system support.

**Validate:** installed, stopped, absent and unsupported fixtures.

**DoD:** unavailable snapd disables only Snap capabilities.

**Do not:** auto-install snapd.

### S121 — Parse installed Snap records

**Prerequisite:** S120, S009.

**Allowed files:** `src/backends/snap/parse.rs`, fixtures.

**Implement:** parse name, version, revision, tracking channel, publisher, confinement,
notes and disabled state from verified output/API.

**Validate:** normal, disabled, classic, malformed fixtures.

**DoD:** installed revision and tracking channel are explicit.

**Do not:** claim all snaps are strictly confined.

### S122 — Implement Snap list/search/details

**Prerequisite:** S121.

**Allowed files:** `src/backends/snap/catalog.rs`.

**Implement:** use supported snap CLI/search interface with timeouts and response limits.
Keep publisher, channel and confinement visible.

**Validate:** fake-runner tests and Snap-enabled VM read checks.

**DoD:** Snap is searchable without snapd being a hard app dependency.

**Do not:** assume store metadata is always online.

### S123 — Implement Snap update/refresh listing

**Prerequisite:** S122.

**Allowed files:** `src/backends/snap/catalog.rs`, parser/tests.

**Implement:** list available revisions/refreshable snaps. Distinguish pending Paket actions
from snapd automatic refreshes and externally created changes.

**Validate:** pending refresh, no update, snapd change in progress fixtures.

**DoD:** update status comes from snapd state.

**Do not:** claim Paket controls snapd auto-refresh schedule.

### S124 — Plan and execute Snap operations

**Prerequisite:** S123, S012.

**Allowed files:** `src/backends/snap/transaction.rs`.

**Implement:** typed install/remove/refresh using supported Snap CLI or documented snapd
interface. Capture returned change ID and track its task state until terminal.

**Validate:** Snap VM install/remove, authorization failure and daemon unavailable.

**DoD:** completion is confirmed by snapd change/task state, not only process exit.

**Do not:** assume a command returning a change ID means the operation completed.

### S125 — Add Snap provider contract tests

**Prerequisite:** S120–S124.

**Allowed files:** `tests/providers/snap.rs`, VM scripts.

**Implement:** read, search, update, install/remove, daemon busy, interrupted client and
external refresh cases.

**Validate:** `cargo test snap`; snapd VM suite.

**DoD:** exact snapd version and behavior recorded.

**Do not:** enable Snap on all distro images without test evidence.

### S126 — Define Paket AppImage managed store

**Prerequisite:** S013.

**Allowed files:** `src/backends/appimage/store.rs`, `src/backends/appimage/mod.rs`.

**Implement:** choose XDG data path for managed AppImages and a small record schema with
stable ID, canonical path, checksum, import time, metadata source, and update capability.

**Validate:** temp-directory tests for path derivation and duplicate records.

**DoD:** Paket never manages arbitrary files outside its managed store without explicit
user action.

**Do not:** execute imported files.

### S127 — Validate and import AppImage safely

**Prerequisite:** S126.

**Allowed files:** `src/backends/appimage/import.rs`, tests.

**Implement:** validate regular file, size limit, ELF/AppImage marker/format, checksum and
safe destination name; copy atomically into managed directory. Do not execute the file or
invoke its embedded runtime for metadata discovery.

**Validate:** valid, corrupt, non-AppImage, symlink, oversized and path traversal fixtures.

**DoD:** import is a bounded filesystem operation with no code execution.

**Do not:** run `--appimage-extract` or application startup during import.

### S128 — Read safe AppImage metadata

**Prerequisite:** S127.

**Allowed files:** `src/backends/appimage/metadata.rs`.

**Implement:** parse only metadata that can be read safely without executing the AppImage;
allow manual name/icon/desktop metadata if extraction is not safely supported. Mark unknown
fields instead of guessing.

**Validate:** metadata fixtures and malicious archive/file cases.

**DoD:** parser cannot write outside staging directory or execute scripts.

**Do not:** trust embedded URLs or desktop Exec commands without validation.

### S129 — Launch and remove managed AppImages

**Prerequisite:** S127–S128.

**Allowed files:** `src/backends/appimage/launch.rs`, `store.rs`.

**Implement:** launch only on explicit user action as the current user, using canonical
managed path and validated desktop entry. Remove only the Paket-managed file and its own
integration records; never delete unrelated files.

**Validate:** launch argv, path replacement/symlink and remove isolation tests.

**DoD:** no root privileges are used for managed AppImages.

**Do not:** execute imported apps during discovery.

### S130 — Detect AppImage update information

**Prerequisite:** S128.

**Allowed files:** `src/backends/appimage/update.rs`.

**Implement:** support only verified embedded update-information types documented by the
AppImage spec/tooling. If absent or unsupported, report updates as Unsupported/Unknown.

**Validate:** update-info present/absent/unknown fixtures.

**DoD:** Paket never invents a generic AppImage update source.

**Do not:** query arbitrary URLs during import.

### S131 — Apply AppImage updates safely

**Prerequisite:** S130.

**Allowed files:** `src/backends/appimage/update.rs`.

**Implement:** invoke a supported external updater only after explicit consent, stage the
replacement, verify checksum/signature where available, keep previous file until verified,
then atomically replace and update the record.

**Validate:** successful update, failed download, corrupted replacement, missing updater.

**DoD:** a failed update leaves the prior managed AppImage usable.

**Do not:** auto-download on launch or overwrite without backup.

---

## Stage L — Unified catalog, AppStream, persistence, and cache

**Gate G11.** Search and app grouping work across providers without corrupting native
identity or misreporting stale data.

### S132 — Add AppStream metadata parser/adapter

**Prerequisite:** S010, G4–G10.

**Allowed files:** `src/catalog/appstream.rs`, `Cargo.toml` if a parser dependency is needed.

**Implement:** integrate with system AppStream metadata or use a maintained parser crate.
Map component ID, name, summary, description, categories and icon references. Preserve
component origin and refresh timestamp.

**Validate:** XML fixture tests for desktop app, addon, runtime, missing icon and malformed
metadata.

**DoD:** AppStream metadata is optional and cannot break provider catalog loading.

**Do not:** infer package ID from AppStream component ID.

### S133 — Map AppStream components to package candidates

**Prerequisite:** S132, S010.

**Allowed files:** `src/catalog/appstream.rs`, tests.

**Implement:** link an app to candidate PackageIds only through native/AppStream mapping
known for that repository/provider. Preserve multiple package candidates and ambiguous links.

**Validate:** one app/multiple package candidate and unresolved component fixtures.

**DoD:** display grouping never changes install target.

**Do not:** join solely on display name.

### S134 — Add trusted icon cache

**Prerequisite:** S132.

**Allowed files:** `src/store/icon_cache.rs`, tests.

**Implement:** cache icons under XDG cache with size/type limits and content hash; accept only
local system metadata paths or HTTPS URLs from trusted repository metadata. Do not execute
SVG scripts or load remote content automatically.

**Validate:** image type/size, path traversal and stale cache tests.

**DoD:** UI icons are bounded and sourced from known metadata.

**Do not:** fetch arbitrary URLs from package descriptions.

### S135 — Implement bounded federated search

**Prerequisite:** G4–G10, S132.

**Allowed files:** `src/catalog/search.rs`.

**Implement:** dispatch one query to available providers under read scheduler; emit
incremental provider results and provider-specific errors. A failed provider does not erase
successful results from others.

**Validate:** mixed success/failure, timeout, cancellation and empty results tests.

**DoD:** search is responsive and provider failures are visible.

**Do not:** wait indefinitely for one offline repository.

### S136 — Implement safe result grouping/ranking

**Prerequisite:** S133, S135.

**Allowed files:** `src/catalog/ranking.rs`, tests.

**Implement:** rank exact name/ID matches first, then summary/description matches. Group
only by stable AppId; otherwise keep separate package rows. Add user-visible source badges.

**Validate:** same-name different app, same app multiple sources, ambiguous IDs.

**DoD:** results never conflate unrelated software.

**Do not:** apply undocumented “preferred package manager” heuristics.

### S137 — Add repository/source display model

**Prerequisite:** S057, S064, S110, provider source support.

**Allowed files:** `src/core/package.rs`, `src/catalog/mod.rs`, tests.

**Implement:** common read-only source display fields: backend, native ID, name, enabled,
URL (redacted), priority where available, and trust/signature status if exposed.

**Validate:** source records across DNF, Flatpak and Zypper.

**DoD:** source provenance is visible but native source identity is retained.

**Do not:** add arbitrary repository URLs.

### S138 — Add SQLite store and migrations

**Prerequisite:** S013, S027.

**Allowed files:** `src/store/db.rs`, `src/store/migrations/`, `Cargo.toml`.

**Implement:** choose SQLite library after checking Tokio integration and bundle size. Use
schema migrations, WAL where appropriate, foreign keys, busy timeout, and XDG state path.

**Validate:** create/open/migrate/backup tests on temp database.

**DoD:** database schema is versioned and corruption errors are surfaced.

**Do not:** store package manager's canonical installed database.

### S139 — Persist Paket settings and managed-file records

**Prerequisite:** S138, S126.

**Allowed files:** `src/store/settings.rs`, `src/store/migrations/`.

**Implement:** persist user preferences, managed AppImage metadata, source visibility and
safe UI preferences. Use typed defaults and tolerate missing/new fields.

**Validate:** migration and unknown-setting tests.

**DoD:** settings survive restart and are not stored in root-owned locations.

**Do not:** store authentication credentials.

### S140 — Add cache with TTL and invalidation

**Prerequisite:** S138, G4–G10.

**Allowed files:** `src/store/package_cache.rs`.

**Implement:** cache read-only catalog/search results with backend/source key, schema
version, timestamp and TTL. Invalidate after successful/unknown transaction, refresh, and
external state change. Mark stale data in API responses.

**Validate:** TTL expiry, source invalidation, backend-specific invalidation tests.

**DoD:** cache never overrides native installed state after transaction.

**Do not:** cache a successful empty response as a permanent empty catalog.

### S141 — Persist transaction history

**Prerequisite:** S138, S037–S039.

**Allowed files:** `src/store/history.rs`, `src/store/migrations/`.

**Implement:** store Paket batch ID, backend step, selected intent, timestamps, outcome,
error classification and redacted diagnostics. Native provider history remains separately
queryable where supported.

**Validate:** partial success, interrupted task, migration and redaction tests.

**DoD:** history does not claim rollback or native transaction completeness it cannot prove.

**Do not:** store full unbounded stdout/stderr in SQLite.

### S142 — Add provider refresh policies

**Prerequisite:** S140.

**Allowed files:** `src/app/coordinator.rs`, `src/catalog/search.rs`.

**Implement:** refresh on user request, stale cache when needed, and after transaction.
Add conservative periodic checks only when enabled by user settings; do not run write
operations or download updates automatically.

**Validate:** fake clock tests for refresh thresholds.

**DoD:** startup is fast and network behavior is predictable.

**Do not:** silently refresh all repositories at every app start.

### S143 — Add backend history adapters

**Prerequisite:** S141.

**Allowed files:** provider-specific history modules.

**Implement:** expose native DNF5/Snap/other transaction history where supported, with
provider source ID and native timestamp. Keep Paket history and native history as separate
records.

**Validate:** parser fixtures and provider availability tests.

**DoD:** history source is always clear.

**Do not:** synthesize an undo operation from history.

### S144 — Add catalog consistency checks

**Prerequisite:** S135–S143.

**Allowed files:** `src/catalog/consistency.rs`, tests.

**Implement:** validate that every displayed package candidate has a registered provider,
valid backend ID, scope, and source provenance. Flag stale or unresolved metadata.

**Validate:** orphan metadata and provider-disabled fixture tests.

**DoD:** invalid metadata is omitted or marked, never converted into a mutation target.

**Do not:** repair native package databases.

---

## Stage M — Slint + Slint Kit production UI

**Gate G12.** UI is added only after backend and catalog services have stable contracts.

### S145 — Add Slint build integration

**Prerequisite:** G11 and S002 license decision.

**Allowed files:** `Cargo.toml`, `build.rs`, `ui/app.slint`.

**Implement:** pin Slint 1.19.0 and Slint Kit 0.9.0; use the current documented `slint_kit_build`
integration. If license gate is not compatible, stop and replace the Kit dependency with
Slint's standard widgets rather than bypassing the license.

**Validate:** clean Linux build.

**DoD:** build uses pinned versions and license decision is recorded.

**Do not:** build all screens in one task.

### S146 — Create main application window and navigation shell

**Prerequisite:** S145.

**Allowed files:** `ui/app.slint`, `src/main.rs`.

**Implement:** window shell, navigation for Discover, Installed, Updates, Queue, History,
and Settings. Wire only static placeholder state.

**Validate:** launch/close on supported Linux desktop session.

**DoD:** UI event loop runs on main thread and shows stable window.

**Do not:** run backend calls in callbacks.

### S147 — Connect Slint to application commands/events

**Prerequisite:** S146, S024.

**Allowed files:** `src/ui/bridge.rs`, `src/app/events.rs`, `src/main.rs`.

**Implement:** UI callbacks send typed `AppCommand` through bounded channels; coordinator
sends typed UI events; apply model changes only through `slint::invoke_from_event_loop`.

**Validate:** test command delivery, UI-thread update, dropped window handle and full channel.

**DoD:** Slint does not depend on Tokio internals or provider implementations.

**Do not:** call `block_on` from Slint callbacks.

### S148 — Implement virtualized package list model

**Prerequisite:** S147.

**Allowed files:** `src/ui/state.rs`, `ui/components/package-list.slint`.

**Implement:** use Slint models suitable for large lists; keep PackageId identity in each
row and preserve selection when sorting/filtering. Add generation tokens so stale async
results cannot update a newer query.

**Validate:** sorting/filtering/selection identity test with large fixture list.

**DoD:** old query results cannot overwrite newer search results.

**Do not:** use row index as package identity.

### S149 — Implement Discover search screen

**Prerequisite:** S148, S135.

**Allowed files:** `ui/views/discover.slint`, UI state/bridge.

**Implement:** search field, loading/empty/partial-error states, source filters and candidate
rows with provider/source/scope badges.

**Validate:** fake search event stream for success, partial backend failure and no results.

**DoD:** results explain where each candidate comes from.

**Do not:** deduplicate by package name.

### S150 — Implement package/application details panel

**Prerequisite:** S149.

**Allowed files:** `ui/views/package-details.slint`, UI state.

**Implement:** details, version/source/scope, license, size, metadata provenance, runtime
or dependency notes, and install/update/remove action to draft queue.

**Validate:** partial metadata and AppStream missing icon tests.

**DoD:** detail actions enqueue intent only.

**Do not:** execute a native operation directly from detail callback.

### S151 — Implement Installed screen

**Prerequisite:** S148.

**Allowed files:** `ui/views/installed.slint`.

**Implement:** installed package/application list, backend filters, scope, search, refresh
state, and remove-to-draft action.

**Validate:** empty, large list, unsupported provider and stale cache cases.

**DoD:** installed state is supplied by provider refresh, not local UI assumptions.

**Do not:** remove on single click.

### S152 — Implement Updates screen

**Prerequisite:** S148, backend update providers.

**Allowed files:** `ui/views/updates.slint`.

**Implement:** per-backend update groups; display update eligibility, held/blocked status,
full-upgrade requirements, Snap daemon-managed state, and rpm-ostree reboot-required state.

**Validate:** mixed backend fixture with different update semantics.

**DoD:** UI does not flatten incompatible update behavior into one generic button.

**Do not:** apply all updates automatically.

### S153 — Implement pending-change basket

**Prerequisite:** S150–S152, S028–S032.

**Allowed files:** `ui/views/queue.slint`, UI state.

**Implement:** show selected intents grouped by backend/scope; support undoing an intent and
show conflicting intents. While one batch runs, new clicks belong to next draft.

**Validate:** duplicate click, conflict and active-batch tests.

**DoD:** multiple install/remove clicks are safely represented before execution.

**Do not:** spawn one child process per click.

### S154 — Implement transaction review screen

**Prerequisite:** S153, provider plan services.

**Allowed files:** `ui/views/review.slint`.

**Implement:** show user-selected targets separately from solver-added dependencies,
upgrades, downgrades and removals. Show preview quality and warnings per backend. Require
explicit approval before queueing a batch.

**Validate:** native complete/partial/estimated/unavailable preview fixtures.

**DoD:** missing preview detail is visible before apply.

**Do not:** hide dependency removals below the fold.

### S155 — Implement transaction progress view

**Prerequisite:** S154, S024.

**Allowed files:** `ui/views/progress.slint`, `src/ui/state.rs`.

**Implement:** queue position, current backend step, phase, package action, determinate or
indeterminate progress, cancel request, lock waiting and terminal status.

**Validate:** fake event sequence for waiting/running/cancel/partial failure.

**DoD:** progress is based on native events or is clearly indeterminate.

**Do not:** synthesize a percentage from elapsed time.

### S156 — Implement batch result and partial failure view

**Prerequisite:** S155, S038.

**Allowed files:** `ui/views/result.slint`.

**Implement:** show successful steps, failed step, skipped steps, next action, and refresh
requirement. Never label partial success as success for the whole batch.

**Validate:** three-backend success/failure/skipped sample.

**DoD:** user can see exactly what changed and what did not.

**Do not:** offer “undo all” unless a backend has a separately verified native recovery.

### S157 — Implement History screen

**Prerequisite:** S141, S143.

**Allowed files:** `ui/views/history.slint`.

**Implement:** filter Paket batches and native backend history; label each source; show
partial/interrupted states and redacted diagnostic details.

**Validate:** persisted history fixtures.

**DoD:** history is accurate after restart.

**Do not:** imply every action is reversible.

### S158 — Implement source/repository read-only screen

**Prerequisite:** S137.

**Allowed files:** `ui/views/sources.slint`.

**Implement:** show installed/configured sources, backend, enabled state, remote/repo name,
and sanitized URL. Keep editing disabled unless a later task defines trust/key UX.

**Validate:** missing source metadata and credential redaction tests.

**DoD:** source provenance is understandable and safe.

**Do not:** add arbitrary remote URL input.

### S159 — Implement Settings and diagnostics

**Prerequisite:** S139, S025.

**Allowed files:** `ui/views/settings.slint`, `ui/views/diagnostics.slint`.

**Implement:** log level, cache refresh policy, source visibility, theme/accessibility
preferences, support matrix, provider diagnostics and copy-redacted-report action.

**Validate:** settings persistence and redaction tests.

**DoD:** user can diagnose unavailable backend without opening a terminal.

**Do not:** upload logs automatically.

### S160 — Implement accessibility, keyboard, and localization baseline

**Prerequisite:** S146–S159.

**Allowed files:** `ui/`, `docs/accessibility.md`, localization resources.

**Implement:** keyboard-only navigation, focus order, visible focus, semantic labels,
contrast checks, text scaling, screen-reader names where Slint/platform supports them, and
string externalization. Document limitations honestly.

**Validate:** manual keyboard walkthrough and a supported screen-reader/desktop session.

**DoD:** core search/install/review/progress workflows are usable without a mouse.

**Do not:** declare accessibility complete from code review alone.

---

## Stage N — Security, integration qualification, packaging, and release

**Gate G13.** A production release passes security review, VM matrix, packaging,
upgrade/uninstall, and operational documentation.

### S161 — Add parser fuzz tests

**Prerequisite:** provider parsers S051–S144.

**Allowed files:** `fuzz/`, parser tests.

**Implement:** fuzz pure parsers for JSON, delimited output, os-release, Flatpak refs,
APT records, Pacman, AUR responses, AppStream XML and helper protocol.

**Validate:** bounded fuzz smoke run in CI/nightly.

**DoD:** parsers do not panic or allocate unbounded memory for malformed input.

**Do not:** fuzz real package-manager writes.

### S162 — Add async scheduler and cancellation stress tests

**Prerequisite:** S021–S039.

**Allowed files:** `tests/runtime/`.

**Implement:** test duplicate clicks, queue fairness, backpressure, cancellation races,
shutdown during active write, external lock and app restart reconciliation.

**Validate:** deterministic Tokio tests plus repeated stress run.

**DoD:** no duplicate active writer, dropped terminal result, or orphaned owned task in tests.

**Do not:** use sleeps as the only synchronization mechanism.

### S163 — Add backend VM integration matrix

**Prerequisite:** G4–G10.

**Allowed files:** `tests/vm/`, CI workflow files.

**Implement:** disposable VM images for Fedora DNF5, Debian/Ubuntu APT, Arch Pacman/AUR,
openSUSE Zypper, rpm-ostree, Flatpak, and Snap where practical. Run real test packages and
snapshot/revert VM after each write test.

**Validate:** provider read/plan/write/failure suite on each qualified image.

**DoD:** no production mutation test runs on the developer workstation.

**Do not:** treat containers alone as proof of systemd/polkit correctness.

### S164 — Add failure-injection and recovery tests

**Prerequisite:** S163.

**Allowed files:** `tests/vm/failure/`, `tests/providers/`.

**Implement:** simulate network loss, mirror failure, repository signature failure, package
scriptlet failure, lock contention, disk-space failure, user auth denial, process kill and
Paket crash. Verify provider reconciliation and truthful UI result.

**Validate:** VM report with each injected failure.

**DoD:** failed/unknown operations do not become success or auto-retry.

**Do not:** disable package signature checks to simplify tests.

### S165 — Complete production security and threat-model review

**Prerequisite:** S078, S088, S097–S099, S105, S112, S118, S124, S127, S129, S131,
and QA tasks S161–S164.

**Allowed files:** `docs/security.md`, security tests, narrowly scoped fixes in reviewed
security-sensitive modules.

**Implement:** review command injection, helper authorization, path races, symlinks,
environment leakage, credential redaction, untrusted package metadata, AUR build scripts,
AppImage import/launch, stale plans, transaction cancellation, crash recovery, and cache
poisoning. Verify every privileged operation has an explicit allowlist and native identity
validation. Record mitigations, evidence, and accepted residual risks.

**Validate:** complete threat-model checklist, adversarial test suite, and second-person
review of the helper boundary.

**DoD:** every privileged or untrusted-input boundary is documented and has corresponding
tests; unresolved critical/high findings block release.

**Do not:** mark a risk accepted without a named rationale and release-note entry.

### S166 — Package Paket for supported distributions

**Prerequisite:** S145, S072–S078, S163, S165.

**Allowed files:** native packaging files under `packaging/`, desktop/metainfo resources.

**Implement:** create native RPM/DEB/Arch packages for qualified targets; include binary,
helper path, desktop entry, icons, metainfo, docs, and correct runtime dependencies. Treat
Flatpak packaging of Paket itself as a separate future effort because sandbox access to host
package tools/system services requires a deliberate host integration design.

**Validate:** install/upgrade/downgrade/uninstall on clean VMs and verify helper owner/mode.

**DoD:** package lifecycle leaves no broken helper or stale desktop integration.

**Do not:** rely only on `cargo run` for production validation.

### S167 — Release engineering and support documentation

**Prerequisite:** S161–S166.

**Allowed files:** `.github/workflows/`, `docs/`, `CHANGELOG.md`, release scripts.

**Implement:** CI for fmt/test/clippy/fuzz smoke/dependency audit; release artifacts per
qualified distro; generate checksums and sign artifacts; publish SBOM; document supported
versions, limitations, data handling, logs, helper authorization, update policy, AUR/AppImage
risks, and known issues. Tag only after all release gates pass.

**Validate:** reproduce release build from clean runner and verify signatures/checksums.

**DoD:** release is traceable, signed, documented, and matches support matrix.

**Do not:** claim unsupported distros or providers as production-ready.

---

## 6. Cross-cutting test strategy

### 6.1 Unit tests

Every pure parser and domain function has tests for normal, empty, malformed, missing-field,
ambiguous, duplicate, stale, and unsupported input where relevant. Command builders have
exact argv tests. IDs have wrong-backend and wrong-scope tests.

### 6.2 Provider contract tests

Each backend must pass the same high-level contract where it advertises the capability:

- probe and descriptor;
- installed listing;
- search/details;
- updates;
- plan preview quality;
- approved apply;
- lock contention;
- authorization denied;
- cancellation request and final reconciliation;
- external tool changing state;
- partial failure and history.

A provider may explicitly not support an operation. Unsupported is a passing contract result
when the capability descriptor and UI explanation are correct.

### 6.3 VM tests

Use disposable VMs, not the developer's main OS. Snapshot before mutations and revert after.
Containers are useful for parser/catalog tests but do not replace VM testing for polkit,
systemd, package database locks, daemon activation, GUI auth agents, or reboot-required
rpm-ostree deployments.

### 6.4 Concurrency tests

Test one active mutation globally, bounded read concurrency, cancellation races, channel
backpressure, event coalescing, duplicate intent suppression, and app shutdown while a child
is active. Ensure the native child is awaited/reaped.

### 6.5 Security tests

Test no shell invocation, fixed executable allowlists, package ID validation in both user
process and helper, root-owned helper path, request size caps, path traversal, symlink races,
credential redaction, signature failure, untrusted AppImage import, and AUR review gate.

### 6.6 Required commands at every gate

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

At release gate also run the dependency audit, clean build, supported VM suite, and package
install/upgrade/uninstall checks. Fuzz smoke tests should run in CI; longer fuzzing may run
nightly.

---

## 7. Operational and UX safeguards

1. **No hidden writes:** search, startup, detail open, and refresh must not install/remove.
2. **No silent trust changes:** never auto-import repository keys or disable signature checks.
3. **No hidden data deletion:** Flatpak app data, AppImage files, build workspaces, and
   package caches have distinct explicit cleanup actions.
4. **No fake progress:** use native percentage when reliable; otherwise show phase and
   indeterminate progress.
5. **No false success:** a non-zero exit, daemon failure, missing final status, or interrupted
   transaction must trigger reconciliation.
6. **No blind retry:** retry is a user-visible decision after current state is re-read.
7. **Stale plan protection:** if native repository or installed state changed after review,
   present updated preview and request approval again.
8. **External manager contention:** explain that another package manager owns the database;
   never remove its lock or terminate its process.
9. **App shutdown:** do not silently orphan a package transaction. Wait, minimize, or explain
   that the transaction must finish before exit.
10. **Updates are opt-in:** Paket reports updates and asks before applying them.

---

## 8. Important design risks and decisions to preserve

| Risk | Required response |
|---|---|
| Same app exists in RPM, Flatpak, Snap and AppImage | Keep separate package candidates and let the user choose source |
| Same Flatpak ID exists in multiple remotes/branches/installs | Preserve full ref and installation identity |
| Package manager lock held externally | Show Busy/Waiting; never delete lock file |
| Cross-backend batch partly succeeds | Record each backend step; stop and re-plan after failure |
| Native resolver output incomplete | Mark preview partial/estimated or disable write capability |
| AUR PKGBUILD executes arbitrary build logic | Require source review; build as normal user; no root build |
| AppImage is untrusted executable content | Import/inspect without executing; run only after user action |
| DNF5 daemon not installed | Keep read capabilities separate; explain missing service or use a separately qualified fallback |
| Immutable/atomic distro | Use rpm-ostree adapter, not normal DNF transaction assumptions |
| Tokio + Slint event loop mismatch | Dedicated Tokio runtime and `invoke_from_event_loop`; no blocking/`block_on` in callbacks |
| Tokio child dropped during operation | Explicit cancellation, retain handle, await/reap, reconcile state |
| Slint Kit still pre-1.0 | Pin version; isolate components; license and upgrade review |
| AppStream metadata disagrees with provider | Metadata is presentation-only; native package ID remains mutation authority |
| External update happens after preview | Invalidate/re-plan and ask for approval again |
| Package manager command output changes | Version-qualified fixtures and explicit unsupported version state |

---

## 9. Suggested first execution sequence

Do not implement the entire list in one AI run. The first sequence is:

1. S001 — inspect repository;
2. S002 — settle name/license decision;
3. S003–S012 — establish only the toolchain and domain models;
4. S013–S026 — add Tokio and the process/runtime boundary;
5. S027–S039 — prove click batching and one-writer coordination with fake providers;
6. S040–S048 — host detection and provider registry;
7. implement and VM-qualify DNF5 before starting more backend packs;
8. continue provider-by-provider; do not start the production UI before G11.

**Critical recommendation:** implement the batch basket and mutation coordinator before any
real install/remove operation. That makes “multiple clicks” a domain/scheduler feature that
can be proven with fake backends, instead of a concurrency bug discovered after the UI is
built.

---

## Appendix A — Current research notes (2026-11-23)

These are source-backed constraints used in the plan; exact commands must still be verified
against the minimum versions Paket chooses to support.

- Slint 1.19 was released on 2026-11-16. Slint's current Rust docs say the event loop must
  run on the main thread, recommend moving work off that thread, and provide
  `invoke_from_event_loop` to update UI state from workers.
- Slint's `spawn_local` documentation explicitly warns that the Slint event loop does not
  drive Tokio's runtime reliably; it documents compatibility constraints and an optional
  `async_compat` bridge. Paket chooses a separate Tokio runtime instead.
- Slint Kit 0.9.0 currently describes itself as actively developed with breaking changes
  expected, requires Rust 1.95+ and Slint 1.19.0, and states the base kit is GPL-3.0-or-later.
  Pin it and confirm license compatibility before use.
- Tokio 1.54.1 was released 2026-10-21; current Tokio docs describe async process support,
  semaphores, and explicit child-process ownership/reaping caveats.
- DNF5 documents `repoquery --queryformat`, JSON output for selected list/repo commands,
  and a D-Bus daemon API for sessions, goal resolution, transactions and progress. Fedora
  packages `dnf5daemon-server` separately.
- PackageKit exposes an asynchronous D-Bus transaction API and advertises capabilities per
  backend. It can be evaluated as an optional transport, but must not be assumed universal.
- APT provides status-FD progress records; use that rather than scraping a terminal bar.
- Arch documentation explicitly says partial upgrades are unsupported; use full-system
  upgrade semantics and do not use `pacman -Sy package`.
- Arch's AUR guidance warns PKGBUILDs are user-produced and may contain malicious code; it
  recommends reviewing PKGBUILDs and running makepkg as a normal user.
- Flatpak supports multiple user/system/named installations and explicitly treats app-data
  deletion separately from uninstall. Its system helper is a privileged internal interface
  not intended for third-party clients; use supported Flatpak CLI behavior.
- Snap refreshes are daemon-managed and may happen automatically; Paket must reconcile
  snapd changes rather than claiming to own all refresh operations.
- AppImage update information is embedded by the application author and may be absent;
  there is no universal repository update source for all AppImages.
- openSUSE documents Zypper as its own command-line package manager with dry-run and
  non-interactive behavior; do not treat it as DNF.
- rpm-ostree package layering creates a pending deployment and normally takes effect after
  reboot; it must have its own state model/provider.
- `pkexec` runs a specified program as root using a polkit authorization path and does not
  validate arguments itself. The helper must validate its own typed requests and be installed
  in a root-owned, non-user-writable location.

## Appendix B — Source references

Official/upstream documentation used for the current research snapshot:

1. [Slint 1.19 release notes](https://www.slint.dev/blog/slint-1.19-released)
2. [Slint Rust threading and event-loop docs](https://docs.slint.dev/latest/docs/rust/slint/)
3. [Slint `invoke_from_event_loop`](https://docs.slint.dev/latest/docs/rust/slint/fn.invoke_from_event_loop)
4. [Slint `spawn_local` and Tokio compatibility notes](https://docs.slint.dev/latest/docs/rust/slint/fn.spawn_local)
5. [Slint Kit project setup](https://slint-kit.dev/guides/project-setup/), [changelog](https://slint-kit.dev/changelog/), [license statement](https://slint-kit.dev/), and [component reference](https://slint-kit.dev/components/)
6. [Tokio release notes](https://github.com/tokio-rs/tokio/releases)
7. [Tokio async process Command](https://docs.rs/tokio/latest/tokio/process/struct.Command.html)
8. [Tokio Child lifecycle and reaping](https://docs.rs/tokio/latest/tokio/process/struct.Child.html)
9. [Tokio Semaphore](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html)
10. [DNF5 repoquery reference](https://dnf5.readthedocs.io/en/stable/commands/repoquery.8.html)
11. [DNF5 daemon overview](https://dnf5.readthedocs.io/en/latest/dnf_daemon/)
12. [DNF5 daemon D-Bus API](https://dnf5.readthedocs.io/en/stable/dnf_daemon/dnf5daemon_dbus_api.8.html)
13. [Fedora dnf5daemon-server package](https://packages.fedoraproject.org/pkgs/dnf5/dnf5daemon-server/)
14. [PackageKit introduction](https://www.freedesktop.org/software/PackageKit/doc/introduction.html)
15. [PackageKit transaction D-Bus API](https://www.freedesktop.org/software/PackageKit/doc/dbus-org.freedesktop.packagekit1.Transaction.html)
16. [Debian apt-get reference](https://manpages.debian.org/trixie/apt/apt-get.8.en.html)
17. [APT progress reporting via Status-Fd](https://github.com/Debian/apt/blob/main/doc/progress-reporting.md)
18. [Arch pacman manual](https://man.archlinux.org/man/core/pacman/pacman.8.en)
19. [Arch partial-upgrade warning](https://wiki.archlinux.org/title/partial_upgrade)
20. [Arch AUR guidance](https://wiki.archlinux.org/title/AUR)
21. [Flatpak command reference](https://docs.flatpak.org/en/latest/flatpak-command-reference.html)
22. [Flatpak system/user installation guidance](https://docs.flatpak.org/en/latest/using-flatpak.html)
23. [Flatpak libflatpak/system helper API notes](https://docs.flatpak.org/en/latest/libflatpak-api-reference.html)
24. [Snap update and change monitoring docs](https://snapcraft.io/docs/how-to-guides/manage-snaps/manage-updates/)
25. [AppImage specification](https://docs.appimage.org/reference/specification.html)
26. [AppImage update information](https://docs.appimage.org/packaging-guide/optional/updates.html)
27. [openSUSE Zypper CLI documentation](https://doc.opensuse.org/documentation/leap/reference/html/book-reference/cha-sw-cl.html)
28. [rpm-ostree administrator handbook](https://coreos.github.io/rpm-ostree/administrator-handbook/)
29. [polkit `pkexec` manual](https://www.freedesktop.org/software/polkit/docs/latest/pkexec.1.html)
30. [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir/)

---

## Appendix C — Change-control rule

Change a locked decision only in a dedicated design task with:

1. the old decision and reason it no longer fits;
2. supporting upstream documentation or VM evidence;
3. impact on existing interfaces, tests, packaging and supported hosts;
4. migration steps;
5. a bounded implementation sequence.

Do not let an implementation agent silently change the architecture while completing an
ordinary `Sxxx` task.
