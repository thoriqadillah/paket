use serde::{Deserialize, Serialize};

use super::identifier::{BackendId, InstallationScope, PackageId, SourceId};

/// Whether a backend reports a package as installed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstalledState {
    Installed,
    NotInstalled,
    /// The backend did not report installed state; never treat this as installed.
    Unknown,
}

/// The kind of native installable unit a record describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackageKind {
    Native,
    Source,
    FlatpakApp,
    FlatpakRuntime,
    Snap,
    AppImage,
    /// The backend did not report a kind.
    Unknown,
}

/// Where a record's non-identity metadata came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetadataOrigin {
    Native,
    AppStream,
    Cache,
    /// The origin is not known, so the metadata may be incomplete.
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackageRecord {
    pub id: PackageId,
    pub name: String,
    pub version: Option<String>,
    pub architecture: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub source: Option<SourceId>,
    pub installed: InstalledState,
    pub kind: PackageKind,
    pub size: Option<u64>,
    pub metadata_origin: MetadataOrigin,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UpdateMetadata {
    pub download_size: Option<u64>,
    pub security: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UpdateRecord {
    pub id: PackageId,
    pub current_version: Option<String>,
    pub candidate_version: Option<String>,
    pub metadata: Option<UpdateMetadata>,
}

/// Optional filters for a catalog search.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SearchScope {
    pub backend: Option<BackendId>,
    pub scope: Option<InstallationScope>,
    pub source: Option<SourceId>,
}

impl SearchScope {
    /// Whether a record passes every supplied filter, keeping its identity intact.
    pub fn matches(&self, record: &PackageRecord) -> bool {
        self.backend
            .is_none_or(|backend| record.id.backend == backend)
            && self
                .scope
                .as_ref()
                .is_none_or(|scope| record.id.scope == *scope)
            && self
                .source
                .as_ref()
                .is_none_or(|source| record.source.as_ref() == Some(source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn id(backend: BackendId, scope: InstallationScope, native: &str) -> PackageId {
        PackageId {
            backend,
            scope,
            native: native.to_string(),
        }
    }

    fn record(id: PackageId, name: &str) -> PackageRecord {
        PackageRecord {
            id,
            name: name.to_string(),
            version: None,
            architecture: None,
            summary: None,
            description: None,
            source: None,
            installed: InstalledState::Unknown,
            kind: PackageKind::Unknown,
            size: None,
            metadata_origin: MetadataOrigin::Unknown,
        }
    }

    #[test]
    fn minimal_record_round_trips_with_missing_optional_fields() {
        let json = r#"{
            "id": {"backend": "Dnf5", "scope": "System", "native": "foo-1.fc42.x86_64"},
            "name": "foo",
            "installed": "NotInstalled",
            "kind": "Native",
            "metadata_origin": "Native"
        }"#;
        let decoded: PackageRecord = serde_json::from_str(json).unwrap();

        assert_eq!(decoded.name, "foo");
        assert_eq!(decoded.installed, InstalledState::NotInstalled);
        assert_eq!(decoded.kind, PackageKind::Native);
        assert_eq!(decoded.metadata_origin, MetadataOrigin::Native);
        assert_eq!(decoded.version, None);
        assert_eq!(decoded.architecture, None);
        assert_eq!(decoded.summary, None);
        assert_eq!(decoded.description, None);
        assert_eq!(decoded.source, None);
        assert_eq!(decoded.size, None);

        let encoded = serde_json::to_string(&decoded).unwrap();
        let round_tripped: PackageRecord = serde_json::from_str(&encoded).unwrap();
        assert_eq!(round_tripped, decoded);
    }

    #[test]
    fn full_record_round_trips_with_known_metadata() {
        let mut pkg = record(
            id(BackendId::Flatpak, InstallationScope::User, "org.foo.App"),
            "Foo",
        );
        pkg.version = Some("2.0".to_string());
        pkg.architecture = Some("x86_64".to_string());
        pkg.summary = Some("A foo app".to_string());
        pkg.description = Some("Longer description".to_string());
        pkg.source = Some(SourceId {
            backend: BackendId::Flatpak,
            native: "flathub".to_string(),
        });
        pkg.installed = InstalledState::Installed;
        pkg.kind = PackageKind::FlatpakApp;
        pkg.size = Some(4096);
        pkg.metadata_origin = MetadataOrigin::AppStream;

        let encoded = serde_json::to_string(&pkg).unwrap();
        let decoded: PackageRecord = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, pkg);
    }

    #[test]
    fn same_name_records_stay_distinct() {
        let first = record(id(BackendId::Dnf5, InstallationScope::System, "foo"), "foo");
        let second = record(
            id(BackendId::Flatpak, InstallationScope::System, "foo"),
            "foo",
        );
        let third = record(
            id(BackendId::Flatpak, InstallationScope::User, "foo"),
            "foo",
        );

        assert_ne!(first, second);
        assert_ne!(second, third);
        assert_ne!(first.id.key(), second.id.key());
        assert_ne!(
            serde_json::to_string(&first).unwrap(),
            serde_json::to_string(&second).unwrap()
        );

        let records: HashSet<PackageRecord> = [first, second, third].into();
        assert_eq!(records.len(), 3);
    }

    #[test]
    fn update_record_references_exact_identity_with_optional_metadata() {
        let pkg_id = id(BackendId::Apt, InstallationScope::System, "foo:amd64");
        let minimal = format!(r#"{{"id": {}}}"#, serde_json::to_string(&pkg_id).unwrap());
        let decoded: UpdateRecord = serde_json::from_str(&minimal).unwrap();
        assert_eq!(decoded.id, pkg_id);
        assert_eq!(decoded.current_version, None);
        assert_eq!(decoded.candidate_version, None);
        assert_eq!(decoded.metadata, None);

        let update = UpdateRecord {
            id: pkg_id,
            current_version: Some("1.0".to_string()),
            candidate_version: Some("1.1".to_string()),
            metadata: Some(UpdateMetadata {
                download_size: Some(2048),
                security: Some(true),
            }),
        };
        let encoded = serde_json::to_string(&update).unwrap();
        let round_tripped: UpdateRecord = serde_json::from_str(&encoded).unwrap();
        assert_eq!(round_tripped, update);
        assert_ne!(round_tripped.id.key(), "");
    }

    #[test]
    fn search_scope_filters_by_backend_scope_and_source_without_changing_identity() {
        let dnf = record(id(BackendId::Dnf5, InstallationScope::System, "foo"), "foo");
        let mut flatpak = record(
            id(BackendId::Flatpak, InstallationScope::User, "org.foo.App"),
            "foo",
        );
        flatpak.source = Some(SourceId {
            backend: BackendId::Flatpak,
            native: "flathub".to_string(),
        });
        let flatpak_identity = flatpak.id.clone();

        let mut scope = SearchScope {
            backend: Some(BackendId::Flatpak),
            scope: Some(InstallationScope::User),
            source: None,
        };
        assert!(!scope.matches(&dnf));
        assert!(scope.matches(&flatpak));

        scope.source = flatpak.source.clone();
        assert!(scope.matches(&flatpak));
        scope.source = Some(SourceId {
            backend: BackendId::Flatpak,
            native: "other".to_string(),
        });
        assert!(!scope.matches(&flatpak));

        let encoded = serde_json::to_string(&scope).unwrap();
        let decoded: SearchScope = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, scope);
        assert_eq!(flatpak.id, flatpak_identity);
        assert_eq!(flatpak.id.key(), "flatpak:user:org.foo.App");
    }
}
