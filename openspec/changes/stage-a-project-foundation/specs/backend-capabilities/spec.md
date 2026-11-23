# Spec Delta

## Purpose

Makes backend availability and supported operations explicit so the UI and core can handle
missing tools and unsupported operations without panics or false promises.

## ADDED Requirements

### Requirement: Backend descriptor
Each backend SHALL be describable by its identity, human-readable label, availability,
optional version, optional reason when unavailable and its declared capabilities.

#### Scenario: Available backend
- **WHEN** a backend is available
- **THEN** its descriptor reports availability and may include a version

#### Scenario: Unavailable backend
- **WHEN** a backend's command is missing
- **THEN** its descriptor reports unavailability with a human-readable reason and does not
  claim capabilities it cannot provide

### Requirement: Capabilities are declared, not assumed
The system SHALL represent search, installed listing, details, updates, install and remove
as individual capabilities. No backend is assumed to support every capability.

#### Scenario: Limited backend
- **WHEN** a backend supports only search and installed listing
- **THEN** its descriptor lists exactly those capabilities
