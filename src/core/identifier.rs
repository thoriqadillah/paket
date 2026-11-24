use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A package-manager backend supported by Paket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackendId {
    Dnf5,
    Apt,
    Pacman,
    Aur,
    Zypper,
    RpmOstree,
    Flatpak,
    Snap,
    AppImage,
}

impl BackendId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dnf5 => "dnf5",
            Self::Apt => "apt",
            Self::Pacman => "pacman",
            Self::Aur => "aur",
            Self::Zypper => "zypper",
            Self::RpmOstree => "rpm-ostree",
            Self::Flatpak => "flatpak",
            Self::Snap => "snap",
            Self::AppImage => "appimage",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstallationScope {
    System,
    User,
    /// A Flatpak installation selected by name (`flatpak --installation=NAME`).
    NamedInstallation(String),
    /// Root of Paket's managed local store.
    LocalManagedStore(PathBuf),
}

impl InstallationScope {
    /// Stable key fragment included in package and list-model keys.
    pub fn key(&self) -> String {
        match self {
            Self::System => "system".to_string(),
            Self::User => "user".to_string(),
            Self::NamedInstallation(name) => format!("installation:{name}"),
            Self::LocalManagedStore(path) => format!("store:{}", path.display()),
        }
    }
}

/// A backend- and scope-qualified package identity. `native` is opaque to core.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackageId {
    pub backend: BackendId,
    pub scope: InstallationScope,
    pub native: String,
}

impl PackageId {
    /// Stable list-model key. Includes every identity discriminator.
    pub fn key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.backend.as_str(),
            self.scope.key(),
            self.native
        )
    }
}

/// A backend-native repository, remote or channel identity. `native` is opaque to core.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceId {
    pub backend: BackendId,
    pub native: String,
}

impl SourceId {
    pub fn key(&self) -> String {
        format!("{}:{}", self.backend.as_str(), self.native)
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

    fn source(backend: BackendId, native: &str) -> SourceId {
        SourceId {
            backend,
            native: native.to_string(),
        }
    }

    #[test]
    fn all_backends_have_unique_stable_string_forms() {
        let backends = [
            BackendId::Dnf5,
            BackendId::Apt,
            BackendId::Pacman,
            BackendId::Aur,
            BackendId::Zypper,
            BackendId::RpmOstree,
            BackendId::Flatpak,
            BackendId::Snap,
            BackendId::AppImage,
        ];

        let mut names = HashSet::new();
        for backend in backends {
            assert!(!backend.as_str().is_empty());
            assert!(names.insert(backend.as_str()));
            let json = serde_json::to_string(&backend).unwrap();
            assert_eq!(backend, serde_json::from_str(&json).unwrap());
        }
        assert_eq!(names.len(), backends.len());
    }

    #[test]
    fn same_name_on_different_backends_is_distinct() {
        let dnf = id(BackendId::Dnf5, InstallationScope::System, "foo");
        let flatpak = id(BackendId::Flatpak, InstallationScope::System, "foo");

        assert_ne!(dnf, flatpak);
        assert_ne!(dnf.key(), flatpak.key());
        assert_ne!(
            serde_json::to_string(&dnf).unwrap(),
            serde_json::to_string(&flatpak).unwrap()
        );
    }

    #[test]
    fn same_ref_in_different_scopes_is_distinct() {
        let system = id(BackendId::Flatpak, InstallationScope::System, "org.foo.App");
        let user = id(BackendId::Flatpak, InstallationScope::User, "org.foo.App");

        assert_ne!(system, user);
        assert_ne!(system.key(), user.key());

        let round_tripped: PackageId =
            serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap();
        assert_eq!(round_tripped, user);
        assert_ne!(round_tripped, system);
    }

    #[test]
    fn same_ref_across_all_scopes_is_distinct() {
        let scopes = [
            InstallationScope::System,
            InstallationScope::User,
            InstallationScope::NamedInstallation("work".to_string()),
            InstallationScope::NamedInstallation("home".to_string()),
            InstallationScope::LocalManagedStore(PathBuf::from("/srv/paket/appimages")),
        ];

        let ids: HashSet<PackageId> = scopes
            .iter()
            .cloned()
            .map(|scope| id(BackendId::Flatpak, scope, "org.foo.App"))
            .collect();
        let keys: HashSet<String> = ids.iter().map(PackageId::key).collect();

        assert_eq!(ids.len(), scopes.len());
        assert_eq!(keys.len(), scopes.len());

        for pkg in &ids {
            let decoded: PackageId =
                serde_json::from_str(&serde_json::to_string(pkg).unwrap()).unwrap();
            assert_eq!(&decoded, pkg);
        }
    }

    #[test]
    fn named_installation_and_local_store_round_trip() {
        for scope in [
            InstallationScope::NamedInstallation("work".to_string()),
            InstallationScope::LocalManagedStore(PathBuf::from("/srv/paket/appimages")),
        ] {
            let decoded: InstallationScope =
                serde_json::from_str(&serde_json::to_string(&scope).unwrap()).unwrap();

            assert_eq!(decoded, scope);
            assert_eq!(decoded.key(), scope.key());
        }
    }

    #[test]
    fn key_includes_backend_scope_and_native() {
        let pkg = id(BackendId::Dnf5, InstallationScope::System, "foo");

        assert_eq!(pkg.key(), "dnf5:system:foo");
        let first = pkg.key();
        assert_eq!(first, pkg.key());
    }

    #[test]
    fn package_id_round_trips_through_serde() {
        let pkg = id(BackendId::Zypper, InstallationScope::User, "foo.x86_64");

        let encoded = serde_json::to_string(&pkg).unwrap();
        let decoded: PackageId = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, pkg);
        assert_eq!(decoded.key(), pkg.key());
    }

    #[test]
    fn equal_ids_hash_equal() {
        let mut ids = HashSet::new();
        ids.insert(id(BackendId::Dnf5, InstallationScope::System, "foo"));
        ids.insert(id(BackendId::Dnf5, InstallationScope::System, "foo"));
        ids.insert(id(BackendId::Flatpak, InstallationScope::System, "foo"));
        ids.insert(id(BackendId::Flatpak, InstallationScope::User, "foo"));

        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn source_identity_is_backend_qualified() {
        let dnf = source(BackendId::Dnf5, "fedora");
        let flatpak = source(BackendId::Flatpak, "fedora");

        assert_ne!(dnf, flatpak);
        assert_ne!(dnf.key(), flatpak.key());
        assert_eq!(dnf.key(), "dnf5:fedora");

        let decoded: SourceId =
            serde_json::from_str(&serde_json::to_string(&flatpak).unwrap()).unwrap();
        assert_eq!(decoded, flatpak);
    }
}
