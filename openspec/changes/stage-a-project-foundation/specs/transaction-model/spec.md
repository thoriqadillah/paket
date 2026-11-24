# Spec Delta

## Purpose

Defines how Paket represents selected intents, native solver effects, multi-backend batch
plans, preview quality and partial outcomes without claiming atomicity.

## ADDED Requirements

### Requirement: Selected intent is separate from resolved changes
A transaction SHALL record the changes the user selected and, separately, the dependency
changes the backend solver resolved. The two sets MUST NOT be merged into one list.

#### Scenario: Dependency added by solver
- **WHEN** a user selects one package and the backend adds dependencies
- **THEN** the plan shows the selected intent and the resolved dependency changes separately

### Requirement: Preview quality is explicit
Every backend step SHALL report whether its preview is native, partial or estimated, or
unavailable. Unavailable previews MUST NOT be presented as complete.

#### Scenario: Unavailable preview
- **WHEN** a backend cannot produce a reliable native preview
- **THEN** the step reports unavailable preview and still identifies the selected target

### Requirement: Batch plans are grouped and non-atomic
A batch plan SHALL group steps by backend and preserve a deterministic order. A batch
spanning several backends MUST NOT claim atomic success or rollback across them.

#### Scenario: Mixed-backend batch
- **WHEN** a batch contains DNF and Flatpak steps
- **THEN** the plan keeps separate steps and makes no atomicity claim

### Requirement: Outcomes report per-step results and partial success
An outcome SHALL report each executed step's result and MUST distinguish full success,
partial success and failure without inventing results for steps that did not run.

#### Scenario: Later step not reached
- **WHEN** an earlier step fails and stop-on-failure applies
- **THEN** later steps are reported as not run and the batch is not reported as successful
