# Spec Delta

## Purpose

Keeps the flow of backend progress and result events toward the UI bounded and loss-free for
terminal outcomes, so a fast child process cannot flood the UI queue and completed work can
never be silently dropped.

## ADDED Requirements

### Requirement: Event delivery is bounded
Commands and terminal events SHALL flow through bounded channels. A fast producer MUST NOT
cause unbounded queue growth; when the queue is full, the producer MUST NOT silently discard
terminal results.

#### Scenario: Flood does not grow without bound
- **WHEN** a child produces events faster than the consumer drains them
- **THEN** total queued events stay within the configured bound

### Requirement: Progress is coalesced, terminal events are not
Intermediate progress events SHALL be coalesced through a latest-value mechanism, and terminal
results SHALL have a delivery path that remains reliable even while progress is coalesced.

#### Scenario: Coalesced progress
- **WHEN** several progress updates arrive before the consumer reads them
- **THEN** the consumer observes the latest value rather than every intermediate one

#### Scenario: Terminal event is never silently dropped
- **WHEN** a terminal result is sent alongside coalesced progress
- **THEN** the terminal result is delivered exactly once and is not replaced by progress

### Requirement: Backend events are typed
Events emitted by backends SHALL be typed values carrying their source stream and nature
(chunk, progress, completion) rather than opaque strings.

#### Scenario: Chunk event carries its stream
- **WHEN** output chunks arrive from stdout and stderr
- **THEN** each event identifies which stream it came from
