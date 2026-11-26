# Spec Delta

## Purpose

Makes runtime and process failures diagnosable through structured logs with configurable
levels and bounded size, without recording credentials or other sensitive values.

## ADDED Requirements

### Requirement: Log levels are configurable and output is bounded
The logging subscriber SHALL honor configurable levels and SHALL keep log output bounded
through rotation or a size bound, so logs cannot grow without limit.

#### Scenario: Sample record
- **WHEN** a command runs and finishes
- **THEN** a structured record of its outcome is emitted at the configured level

### Requirement: Sensitive values are redacted
Logs SHALL redact credential-bearing URLs, authorization data, environment variables and
sensitive privileged-helper payload fields before writing.

#### Scenario: URL with credentials
- **WHEN** a URL containing an embedded credential is about to be logged
- **THEN** the credential portion is redacted in the written record

#### Scenario: Environment values
- **WHEN** environment overrides are about to be logged
- **THEN** their values are redacted rather than written in clear

### Requirement: No telemetry or remote reporting
The logging subsystem MUST NOT emit telemetry or send logs to any remote destination.

#### Scenario: Offline run
- **WHEN** the application runs with no network access
- **THEN** logging behaves identically and attempts no outbound connection
