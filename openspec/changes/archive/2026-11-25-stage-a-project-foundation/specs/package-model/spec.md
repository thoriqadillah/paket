# Spec Delta

## Purpose

Defines stable package and source identity, catalog records and application grouping without
letting display names become mutation identity.

## ADDED Requirements

### Requirement: Package identity is backend- and scope-qualified
Every package identity SHALL include its backend, installation scope and an opaque native
identifier. Two packages with the same display name or native identifier in different
backends or scopes MUST remain distinct.

#### Scenario: Same name on two backends
- **WHEN** a DNF package and a Flatpak package both display `foo`
- **THEN** their identities compare unequal and produce different list keys

#### Scenario: Same ref in user and system scope
- **WHEN** the same Flatpak ref is installed for the user and the system
- **THEN** the two identities remain distinct

#### Scenario: Stable display key
- **WHEN** an identity is converted to a list-model key
- **THEN** the key includes backend, scope and native identifier and is stable across calls

### Requirement: Source identity is explicit
A package record SHALL identify the repository, remote or local source it came from when the
backend exposes one, and SHALL represent an unknown source explicitly.

#### Scenario: Known source
- **WHEN** a backend reports a repository or remote for a package
- **THEN** the record carries a stable source identity

#### Scenario: Unknown source
- **WHEN** the backend does not expose a source
- **THEN** the record represents the source as unknown rather than inventing a value

### Requirement: Catalog records preserve uncertain metadata
A catalog record SHALL carry identity, name, installed state and optional version,
architecture, summary, description, source, size, package kind and metadata origin. Absent
metadata MUST remain absent or explicitly unknown.

#### Scenario: Missing optional metadata
- **WHEN** a record has no version, architecture, summary, description or size
- **THEN** it serializes and deserializes successfully with those values absent or unknown

#### Scenario: Installed state
- **WHEN** a record is read from the catalog
- **THEN** its installed state is explicit and is not inferred from the display name

### Requirement: Updates are records, not cross-backend comparisons
An update record SHALL reference a package identity and the current and candidate versions,
with optional download or security metadata. Versions MUST NOT be compared across backends.

#### Scenario: Update for one package
- **WHEN** a backend reports a newer candidate for an installed package
- **THEN** the update record references that exact identity and both versions

### Requirement: Application grouping is many-to-many
An application SHALL have an identity independent of package identity and MAY reference
several package candidates. Grouping MUST NOT collapse, replace or reassign candidate
install identities.

#### Scenario: App with RPM and Flatpak candidates
- **WHEN** one application maps to an RPM package and a Flatpak package
- **THEN** both candidates remain individually addressable and the app identity is unchanged

### Requirement: Search scope narrows results
A search scope SHALL optionally filter by backend, installation scope and source without
changing the identity of returned packages.

#### Scenario: Scoped search
- **WHEN** a search supplies a backend and scope filter
- **THEN** only matching candidates are returned with their full identities
