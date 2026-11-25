# backend-capabilities Specification

## Purpose

Makes backend availability and supported operations explicit so core and UI can handle
missing tools, read-only providers and unsupported operations without false promises.

## Requirements

### Requirement: Backend descriptor
Each backend SHALL be describable by identity, label, availability, optional version,
optional reason when unavailable and its declared capability set.

#### Scenario: Available backend
- **WHEN** a backend is available
- **THEN** its descriptor reports availability and may include a version

#### Scenario: Unavailable backend
- **WHEN** a backend's command or service is missing
- **THEN** the descriptor reports unavailability with a human-readable reason and claims no
  capability it cannot provide

### Requirement: Capabilities are declared, not assumed
The system SHALL represent installed listing, search, details, updates, plan, install,
remove, sources, local import, launch and history as individual capabilities.

#### Scenario: Read-only backend
- **WHEN** a backend supports only installed listing, search and details
- **THEN** its descriptor declares those capabilities and no install or remove capability

#### Scenario: Missing tool on a supported distro
- **WHEN** a known distro lacks a backend's executable or service
- **THEN** the backend is unavailable rather than assumed usable from the distro identity

### Requirement: Unavailable, unsupported and empty are distinct
The system SHALL distinguish an unavailable backend, an unsupported operation and a
successful empty result.

#### Scenario: Empty result
- **WHEN** an available backend returns no matches
- **THEN** the result is empty and is not reported as unavailable or unsupported
