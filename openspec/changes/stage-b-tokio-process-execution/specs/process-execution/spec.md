# Spec Delta

## Purpose

Lets the application run native package-manager tools as structured, bounded, explicitly
cancellable child processes instead of shell strings, without blocking the UI thread or
losing output, timeout or cancellation semantics.

## ADDED Requirements

### Requirement: Commands are structured values
All process invocation data SHALL be a structured specification carrying a fixed executable,
an argv array, explicit environment overrides, a timeout policy, output limits and a capture
mode. Invocation data MUST NOT be accepted as a shell string, and a working directory MUST NOT
be accepted from UI input.

#### Scenario: Query with spaces stays one argument
- **WHEN** a search query containing spaces is turned into argv
- **THEN** it remains exactly one argv element and never becomes shell syntax

#### Scenario: No shell interpretation
- **WHEN** a command specification is executed
- **THEN** the executable is invoked directly with the argv array and no shell is started

### Requirement: Child processes run with an explicit environment
A spawned child SHALL have its environment set explicitly by the specification, including a
stable locale for parsers that need one, with stdio configured explicitly.

#### Scenario: Parser locale
- **WHEN** a parser that requires stable English output runs
- **THEN** the child's locale is set explicitly rather than inherited from the host

### Requirement: Output streams are drained concurrently with bounded memory
A running child SHALL have stdout and stderr drained concurrently as typed chunk events
carrying their source stream, preserving a final partial line. Output SHALL NOT be assumed to
be UTF-8, and per-command limits SHALL bound total captured output with a bounded diagnostic
tail.

#### Scenario: Heavy output on both streams
- **WHEN** a child writes heavily to stdout and stderr at the same time
- **THEN** neither stream blocks the other and the completion result reports both

#### Scenario: Output above the limit
- **WHEN** captured output exceeds the configured limit
- **THEN** the result is a typed output-limit error and the child is treated per the
  cancellation policy rather than reported as success

### Requirement: Timeouts have an explicit result state
A timed-out command SHALL produce a typed timeout outcome, not a success or a generic
failure. Read-command timeout defaults and long-transaction timeout policy SHALL be separate.

#### Scenario: Sleeping child exceeds the read timeout
- **WHEN** a read command exceeds its timeout
- **THEN** it is terminated and reported as timed out, never as success

#### Scenario: Transaction timeout requests cancellation
- **WHEN** a long-running transaction exceeds its timeout policy
- **THEN** graceful cancellation is requested and the final state is reconciled before it is
  reported

### Requirement: Cancellation is an explicit request with retained child ownership
A caller SHALL be able to request cancellation while the child handle stays owned. Queued
work SHALL be cancellable without spawning; running work SHALL enter a cancel-requested state
until the child exits and is reaped. Dropping a future or handle MUST NOT be treated as proof
that a process stopped.

#### Scenario: Cancel before spawn
- **WHEN** work is cancelled before its child spawns
- **THEN** no child is started and the work reports cancelled

#### Scenario: Cancel while running
- **WHEN** a running child is cancelled
- **THEN** the work stays owned until the child exits and is reaped, then reports cancelled

### Requirement: Process groups give staged termination for descendants
When a provider may spawn descendants, the child SHALL run in a dedicated Unix process group
with a staged termination policy: graceful interrupt, bounded wait, then provider-approved
termination. The parent child handle SHALL remain alive until the group is reaped.

#### Scenario: Parent and child process both die
- **WHEN** a staged termination runs against a parent that spawned a helper
- **THEN** both processes exit and are reaped without zombies

### Requirement: Providers depend on a runner port
Process execution SHALL be exposed through one small async runner port that providers depend
on, and tests SHALL be able to substitute a fake runner.

#### Scenario: Provider tests without host tools
- **WHEN** provider tests run
- **THEN** they execute against a fake runner and need no installed package tools or root

### Requirement: A deterministic fake runner exists for tests
The test harness SHALL provide a fake runner that queues predetermined results and events,
asserts the exact executable, argv and environment it received, and fails the test on any
unexpected command.

#### Scenario: Unexpected command fails the test
- **WHEN** code under test invokes a command the fake runner did not expect
- **THEN** the fake runner reports the mismatch and the test fails
