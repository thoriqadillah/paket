# Spec Delta

## Purpose

Gives the application a dedicated asynchronous runtime with a single owner thread, bounded
command/result flow to and from the UI thread, and deterministic startup and shutdown
behavior independent of the UI framework.

## ADDED Requirements

### Requirement: The runtime has one owner and the UI thread never blocks
The asynchronous runtime SHALL be a multi-thread runtime owned by one dedicated named thread
that runs the application coordinator. The UI event loop SHALL stay on the main OS thread,
and UI callbacks SHALL submit commands through a bounded channel with non-blocking sends and
MUST NOT block waiting for asynchronous work.

#### Scenario: Command round-trip
- **WHEN** the runtime owner starts and one command is sent through the bounded channel
- **THEN** the coordinator's result is observed, shutdown is requested, and the owner thread
  joins cleanly

#### Scenario: UI callback submits without waiting
- **WHEN** a UI callback submits a command while the coordinator is busy
- **THEN** the callback returns immediately rather than blocking the UI thread

### Requirement: Startup and shutdown are explicit
Runtime startup and shutdown SHALL be explicit, with the owning thread joined before process
exit, and shutdown behavior SHALL be deterministic for idle, queued and active states.

#### Scenario: Shutdown while work is queued
- **WHEN** shutdown is requested with queued read-only work
- **THEN** new work is refused, queued work is cancelled and reported, and the runtime exits
  without waiting on already-cancelled reads

#### Scenario: Shutdown while a system transaction is active
- **WHEN** shutdown is requested while a system transaction is running
- **THEN** the application does not exit silently and the child handle is not dropped as if
  the operation had ended

#### Scenario: Shutdown while idle
- **WHEN** shutdown is requested with no work in flight
- **THEN** the runtime stops promptly and joins

### Requirement: No work is accepted after shutdown begins
Once shutdown begins, the coordinator SHALL stop accepting new work and SHALL report
rejection to the submitter instead of silently enqueuing it.

#### Scenario: Late command
- **WHEN** a command is submitted after shutdown has begun
- **THEN** the submitter receives an explicit rejection
