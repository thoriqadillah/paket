# Spec Delta

## Purpose

Defines stable package identity across package managers and the minimal metadata Paket
displays for a catalog entry.

## ADDED Requirements

### Requirement: Package identity is backend-qualified
Every package identity SHALL include its backend and an opaque native identifier. Two
packages with the same display name on different backends MUST remain distinct.

#### Scenario: Same name on two backends
- **WHEN** a DNF package and a Flatpak package both have the native name `foo`
- **THEN** their identities are different and compare unequal

#### Scenario: Stable display key
- **WHEN** an identity is converted to a list-model key
- **THEN** the key includes the backend and native identifier and is stable across calls

### Requirement: Backends are enumerated
The system SHALL identify each supported backend with a stable string form covering DNF5,
Flatpak, APT, Pacman, AUR, Snap and AppImage.

#### Scenario: Backend string form
- **WHEN** any supported backend is converted to a string
- **THEN** it yields a non-empty, stable identifier

### Requirement: Minimal catalog package
A catalog package SHALL carry its identity, name, optional version, architecture, summary
and source, plus an installed flag. Missing optional metadata MUST be representable as
absent rather than invented.

#### Scenario: JSON round-trip
- **WHEN** a package with all optional fields present is serialized to JSON and read back
- **THEN** its identity, name and every optional field are unchanged

#### Scenario: Missing metadata
- **WHEN** a package has no version, architecture, summary or source
- **THEN** it serializes and deserializes successfully with those fields absent
