use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::identifier::BackendId;

/// An operation a backend can provide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Capability {
    ListInstalled,
    Search,
    Details,
    Updates,
    Plan,
    Install,
    Remove,
    Sources,
    LocalImport,
    Launch,
    History,
}

/// Whether a backend's command or service is usable on this host.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackendAvailability {
    Available,
    Unavailable { reason: String },
}

impl BackendAvailability {
    pub fn is_available(&self) -> bool {
        matches!(self, Self::Available)
    }

    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Available => None,
            Self::Unavailable { reason } => Some(reason),
        }
    }
}

/// A backend's identity, availability and declared capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendInfo {
    pub backend: BackendId,
    pub label: String,
    pub availability: BackendAvailability,
    pub version: Option<String>,
    pub capabilities: BTreeSet<Capability>,
}

impl BackendInfo {
    /// Whether the descriptor declares `capability`.
    pub fn declares(&self, capability: Capability) -> bool {
        self.availability.is_available() && self.capabilities.contains(&capability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::package::PackageRecord;

    const ALL_CAPABILITIES: [Capability; 11] = [
        Capability::ListInstalled,
        Capability::Search,
        Capability::Details,
        Capability::Updates,
        Capability::Plan,
        Capability::Install,
        Capability::Remove,
        Capability::Sources,
        Capability::LocalImport,
        Capability::Launch,
        Capability::History,
    ];

    fn info(availability: BackendAvailability, capabilities: &[Capability]) -> BackendInfo {
        BackendInfo {
            backend: BackendId::Dnf5,
            label: "DNF5".to_string(),
            availability,
            version: None,
            capabilities: capabilities.iter().copied().collect(),
        }
    }

    fn absent() -> BackendInfo {
        info(
            BackendAvailability::Unavailable {
                reason: "dnf5 is not installed".to_string(),
            },
            &[],
        )
    }

    fn read_only() -> BackendInfo {
        let mut read_only = info(
            BackendAvailability::Available,
            &[
                Capability::ListInstalled,
                Capability::Search,
                Capability::Details,
            ],
        );
        read_only.version = Some("5.2".to_string());
        read_only
    }

    fn fully_available() -> BackendInfo {
        let mut full = info(BackendAvailability::Available, &ALL_CAPABILITIES);
        full.version = Some("5.2".to_string());
        full
    }

    #[test]
    fn absent_descriptor_is_unavailable_with_a_reason() {
        let absent = absent();

        assert!(!absent.availability.is_available());
        assert_eq!(absent.availability.reason(), Some("dnf5 is not installed"));
        assert_eq!(absent.version, None);
        assert!(absent.capabilities.is_empty());

        let decoded: BackendInfo =
            serde_json::from_str(&serde_json::to_string(&absent).unwrap()).unwrap();
        assert_eq!(decoded, absent);
        assert_eq!(decoded.availability.reason(), Some("dnf5 is not installed"));
    }

    #[test]
    fn read_only_descriptor_declares_queries_but_not_mutations() {
        let read_only = read_only();

        for capability in [
            Capability::ListInstalled,
            Capability::Search,
            Capability::Details,
        ] {
            assert!(read_only.declares(capability));
        }
        for capability in [
            Capability::Updates,
            Capability::Plan,
            Capability::Install,
            Capability::Remove,
            Capability::Sources,
            Capability::LocalImport,
            Capability::Launch,
            Capability::History,
        ] {
            assert!(!read_only.declares(capability));
        }
    }

    #[test]
    fn fully_available_descriptor_declares_every_capability() {
        let full = fully_available();

        assert_eq!(full.capabilities.len(), ALL_CAPABILITIES.len());
        for capability in ALL_CAPABILITIES {
            assert!(full.declares(capability));
        }
    }

    #[test]
    fn absent_read_only_and_full_descriptors_are_distinguishable() {
        let absent = absent();
        let read_only = read_only();
        let full = fully_available();

        assert!(!absent.availability.is_available());
        assert!(read_only.availability.is_available());
        assert!(full.availability.is_available());
        assert_ne!(absent, read_only);
        assert_ne!(absent, full);
        assert_ne!(read_only, full);
        assert_ne!(absent.capabilities, read_only.capabilities);
        assert_ne!(read_only.capabilities, full.capabilities);

        assert!(!absent.declares(Capability::Search));
        assert!(read_only.declares(Capability::Search));
        assert!(full.declares(Capability::Search));
    }

    #[test]
    fn unavailable_backend_declares_nothing_even_with_a_populated_set() {
        let absent = info(
            BackendAvailability::Unavailable {
                reason: "service is missing".to_string(),
            },
            &[Capability::Search],
        );

        assert!(!absent.declares(Capability::Search));
    }

    #[test]
    fn empty_result_is_not_unavailable_or_unsupported() {
        let backend = read_only();
        let results: Vec<PackageRecord> = Vec::new();

        assert!(results.is_empty());
        assert!(backend.availability.is_available());
        assert!(backend.declares(Capability::ListInstalled));
        assert!(!backend.declares(Capability::Install));
    }
}
