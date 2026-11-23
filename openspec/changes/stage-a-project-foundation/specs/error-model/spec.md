# Spec Delta

## Purpose

Lets core code report expected catalog and operation failures as typed values with readable
messages instead of leaking raw process errors or stringly-typed failures.

## ADDED Requirements

### Requirement: Catalog failures are typed
Catalog failures SHALL distinguish unavailable backends, command failures, parse failures
and missing packages. Raw diagnostic output MAY accompany a failure as context but MUST NOT
be the only meaning.

#### Scenario: Unavailable backend
- **WHEN** a catalog operation targets an unavailable backend
- **THEN** it returns the unavailable variant with a readable message

#### Scenario: Package not found
- **WHEN** a requested package does not exist
- **THEN** it returns the not-found variant

### Requirement: Operation failures are typed
Operation failures SHALL distinguish unsupported operations, busy state, authorization
denial, network failure, dependency failure, signature failure, no disk space, cancellation
and a generic native failure.

#### Scenario: Unsupported operation
- **WHEN** an operation is requested from a backend that does not support it
- **THEN** it returns the unsupported variant

#### Scenario: Readable display
- **WHEN** any typed error is displayed
- **THEN** it produces a non-empty human-readable message
