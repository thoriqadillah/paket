# Paket — Agent Instructions

## Read first

- `docs/plan.md` — implementation plan; source of truth for scope, order and validation.
- `docs/baseline.md` — repository, toolchain and host snapshot recorded in S01.

## Progress is tracked in OpenSpec

This repository uses OpenSpec. Do not create or maintain a separate progress file.

- Use one OpenSpec change per plan stage or task pack. Do not invent ad-hoc changes.
- The change's `tasks.md` mirrors that stage's `Sxx` tasks from `docs/plan.md`.
- Mark an `Sxx` task `- [x]` only when its Definition of Done passes.
- Archive the change when its tasks are done; gates control when the next stage starts.
- If `tasks.md` and `docs/plan.md` disagree, `docs/plan.md` wins.

| Plan section | Change name | Tasks | Gate |
|---|---|---|---|
| Stage A — Project foundation | `stage-a-project-foundation` | S01–S06 | G0 |
| Stage B — Safe command execution and host detection | `stage-b-execution-host-detection` | S07–S12 | G1 |
| Stage C — DNF5 read-only catalog | `stage-c-dnf-catalog` | S13–S21 | G2 |
| Stage D — DNF planning and privileged mutations | `stage-d-dnf-mutations` | S22–S34 | G3 |
| Stage E — Flatpak catalog and operations | `stage-e-flatpak` | S35–S41 | G4 (with Stage F) |
| Stage F — Core service and backend contract stabilization | `stage-f-core-contract` | S42–S45 | G4 |
| Stage G — Minimal Slint UI | `stage-g-ui-mvp` | S46–S54 | G5 |
| Pack A — APT / DEB | `pack-a-apt` | S55–S61 | G6 |
| Pack B — Pacman | `pack-b-pacman` | S62–S67 | G6 |
| Pack C — AUR | `pack-c-aur` | S68–S70 | G6 |
| Pack D — Snap | `pack-d-snap` | S71–S73 | G6 |
| Pack E — AppImage | `pack-e-appimage` | S74–S76 | G6 |
| Shared improvements | `shared-improvements` | S77–S81 | — |
| Packaging and release | `packaging-release` | S82–S84 | — |

## Per-run workflow

1. Run `openspec list` and find the change for the next incomplete stage.
2. If the change does not exist, use the OpenSpec propose workflow to create it from the
   matching `docs/plan.md` section, then stop. Planning only; do not implement.
3. Otherwise use the OpenSpec apply workflow, but implement exactly one unchecked `Sxx`
   task per run. Do not continue to the next task even though the apply workflow normally
   loops over all of them.
4. Verify prerequisites. If a stage task is already implemented, verify it against the
   plan and mark it complete instead of redoing it.
5. Run the task's validation commands. Update only that task's checkbox.
6. Report files changed, behavior, validation output, limitations and DoD result. Stop.

Use the standard prompt in `docs/plan.md` §0.3 when assigning a task.

## Rules

- Never commit; the human commits (plan §0.6).
- Record factual host/tool/plan mismatches in `docs/deviations.md`. It is not a progress log.
- Do not expand a task beyond its allowed files. Stop and report blockers.
- The plan's safety rules always apply: no shell, no arbitrary helper argv, honest plan
  previews, no root for AUR builds.
