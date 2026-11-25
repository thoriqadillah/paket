# Spec Delta

## Purpose

Lets core code report expected catalog, plan and operation failures as typed values with
safe messages instead of leaking raw process errors or stringly-typed failures.

## ADDED Requirements

### Requirement: Catalog failures are typed
Catalog failures SHALL distinguish unavailable backends, missing packages, invalid identity
and parse failures. Raw diagnostic output MAY accompany a failure as context but MUST NOT be
the only meaning.

#### Scenario: Unavailable backend
- **WHEN** a catalog operation targets an unavailable backend
- **THEN** it returns the unavailable variant with a readable message

#### Scenario: Package not found
- **WHEN** a requested package does not exist
- **THEN** it returns the not-found variant

### Requirement: Plan failures are typed
Plan failures SHALL distinguish unsupported capability, invalid target, stale plan and
native planning failure.

#### Scenario: Stale plan
- **WHEN** an apply request references a plan that is no longer current
- **THEN** it returns the stale-plan variant and no mutation runs

#### Scenario: Invalid target
- **WHEN** a plan target fails identity validation
- **THEN** it returns the invalid-identity variant and no command is built

### Requirement: Operation failures are typed
Operation failures SHALL distinguish busy or locked state, authorization denial, network
failure, dependency failure, signature or trust failure, no disk space, timeout, cancellation
and a generic native failure.

#### Scenario: Locked package manager
- **WHEN** the native package manager reports a lock held by another process
- **THEN** the operation returns the busy variant with a readable reason

#### Scenario: Cancelled operation
- **WHEN** the user cancels an in-flight operation
- **THEN** it returns the cancellation variant and is not reported as a native failure

### Requirement: User messages are safe and diagnostics stay separate
Every typed failure SHALL provide a non-empty user-facing message and MAY carry native
diagnostic detail separately. Diagnostic text MUST NOT be the only meaning of the failure.

#### Scenario: Readable display
- **WHEN** any typed failure is displayed
- **THEN** it produces a non-empty human-readable message

#### Scenario: Diagnostic context
- **WHEN** a native command fails with stderr output
- **THEN** the failure exposes that output as separate context without replacing its variant
