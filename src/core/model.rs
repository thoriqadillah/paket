use serde::{Deserialize, Serialize};

/// A package-manager backend supported by Paket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackendId {
    Dnf5,
    Flatpak,
    Apt,
    Pacman,
    Aur,
    Snap,
    AppImage,
}

impl BackendId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dnf5 => "dnf5",
            Self::Flatpak => "flatpak",
            Self::Apt => "apt",
            Self::Pacman => "pacman",
            Self::Aur => "aur",
            Self::Snap => "snap",
            Self::AppImage => "appimage",
        }
    }
}

/// A backend-qualified package identity. `native` is opaque to core.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PkgId {
    pub backend: BackendId,
    pub native: String,
}

impl PkgId {
    pub fn key(&self) -> String {
        format!("{}:{}", self.backend.as_str(), self.native)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(backend: BackendId, native: &str) -> PkgId {
        PkgId {
            backend,
            native: native.to_string(),
        }
    }

    #[test]
    fn same_name_on_different_backends_is_distinct() {
        let dnf = id(BackendId::Dnf5, "foo");
        let flatpak = id(BackendId::Flatpak, "foo");
        assert_ne!(dnf, flatpak);
        assert_ne!(dnf.key(), flatpak.key());
    }

    #[test]
    fn key_is_stable_and_backend_qualified() {
        let pkg = id(BackendId::Dnf5, "foo");
        assert_eq!(pkg.key(), "dnf5:foo");
        assert_eq!(pkg.key(), pkg.key());
    }

    #[test]
    fn all_backends_have_stable_string_forms() {
        let backends = [
            BackendId::Dnf5,
            BackendId::Flatpak,
            BackendId::Apt,
            BackendId::Pacman,
            BackendId::Aur,
            BackendId::Snap,
            BackendId::AppImage,
        ];
        for backend in backends {
            assert!(!backend.as_str().is_empty());
            assert_eq!(backend.as_str(), backend.as_str());
        }
    }

    #[test]
    fn equal_ids_hash_equal() {
        use std::collections::HashSet;

        let mut ids = HashSet::new();
        ids.insert(id(BackendId::Dnf5, "foo"));
        ids.insert(id(BackendId::Dnf5, "foo"));
        ids.insert(id(BackendId::Flatpak, "foo"));
        assert_eq!(ids.len(), 2);
    }
}
