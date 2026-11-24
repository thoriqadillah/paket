# Paket — Agent Instructions

## Read first

- `docs/plan.md` — implementation plan; source of truth for scope, order and validation.
- `docs/baseline.md` — repository, toolchain and host snapshot recorded in S001.

## Progress is tracked in OpenSpec

This repository uses OpenSpec. Do not create or maintain a separate progress file.

- Use one OpenSpec change per plan stage. Do not invent ad-hoc changes.
- The change's `tasks.md` mirrors that stage's `Sxxx` tasks from `docs/plan.md`.
- Mark an `Sxxx` task `- [x]` only when its Definition of Done passes.
- Archive the change when its tasks are done; gates control when the next stage starts.
- If `tasks.md` and `docs/plan.md` disagree, `docs/plan.md` wins.

The Stage A change already exists as `stage-a-project-foundation`; reuse it. Create the
later changes only when their stage starts.

| Plan section | Change name | Tasks | Gate |
|---|---|---|---|
| Stage A — Product and domain foundation | `stage-a-project-foundation` | S001–S012 | G0 |
| Stage B — Tokio runtime and process execution | `stage-b-tokio-process-execution` | S013–S026 | G1 |
| Stage C — Batch intents, scheduler, and transaction lifecycle | `stage-c-batch-scheduler-transactions` | S027–S039 | G2 |
| Stage D — Host detection and provider registry | `stage-d-host-detection-provider-registry` | S040–S048 | G3 |
| Stage E — DNF5 provider | `stage-e-dnf5-provider` | S049–S060 | G4 |
| Stage F — Flatpak provider | `stage-f-flatpak-provider` | S061–S071 | G5 |
| Stage G — Privileged helper foundation | `stage-g-privileged-helper` | S072–S078 | G6 |
| Stage H — APT / dpkg provider | `stage-h-apt-provider` | S079–S090 | G7 |
| Stage I — Pacman and AUR providers | `stage-i-pacman-aur` | S091–S107 | G8 |
| Stage J — Zypper and rpm-ostree providers | `stage-j-zypper-rpm-ostree` | S108–S119 | G9 |
| Stage K — Snap and managed AppImage providers | `stage-k-snap-appimage` | S120–S131 | G10 |
| Stage L — Unified catalog, AppStream, persistence, and cache | `stage-l-catalog-persistence` | S132–S144 | G11 |
| Stage M — Slint + Slint Kit production UI | `stage-m-slint-ui` | S145–S160 | G12 |
| Stage N — Security, integration qualification, packaging, and release | `stage-n-security-release` | S161–S167 | G13 |

## Per-run workflow

1. Run `openspec list` and find the change for the next incomplete stage.
2. If the change does not exist, use the OpenSpec propose workflow to create it from the
   matching `docs/plan.md` section, then stop. Planning only; do not implement.
3. Otherwise use the OpenSpec apply workflow, but implement exactly one unchecked `Sxxx`
   task per run. Do not continue to the next task even though the apply workflow normally
   loops over all of them.
4. Verify prerequisites. If a stage task is already implemented, verify it against the
   plan and mark it complete instead of redoing it.
5. Run the task's validation commands. Update only that task's checkbox.
6. Report files changed, behavior, validation output, limitations and DoD result. Stop.

Use the standard prompt in `docs/plan.md` §0.3 when assigning a task.

## Rules

- Never commit unless the human asks (plan §0.4).
- Record factual host/tool/plan mismatches in `docs/deviations.md`. It is not a progress log.
- Do not expand a task beyond its allowed files. Stop and report blockers.
- The plan's safety rules always apply: no shell, no arbitrary helper argv, honest plan
  previews, no signature bypass, no root for AUR builds.
