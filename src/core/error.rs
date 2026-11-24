use thiserror::Error;

/// A failure while reading or interpreting package catalog data.
#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("the package backend is unavailable")]
    UnavailableBackend,
    #[error("the requested package was not found")]
    NotFound,
    #[error("the package identity is invalid")]
    InvalidIdentity,
    #[error("the package catalog output could not be parsed")]
    ParseFailure {
        /// Native output kept for diagnostics; never part of the user-facing message.
        diagnostic: Option<String>,
    },
}

impl CatalogError {
    /// Native output that explains the failure without replacing its user message.
    pub fn diagnostic(&self) -> Option<&str> {
        match self {
            Self::ParseFailure { diagnostic } => diagnostic.as_deref(),
            Self::UnavailableBackend | Self::NotFound | Self::InvalidIdentity => None,
        }
    }
}

/// A failure while planning changes on a backend.
#[derive(Debug, Error)]
pub enum PlanError {
    #[error("the backend does not support this operation")]
    UnsupportedCapability,
    #[error("the plan target is invalid")]
    InvalidIdentity,
    #[error("the plan is stale and must be recalculated")]
    StalePlan,
    #[error("the backend could not plan the requested changes")]
    NativeFailure {
        /// Native output kept for diagnostics; never part of the user-facing message.
        diagnostic: Option<String>,
    },
}

impl PlanError {
    /// Native output that explains the failure without replacing its user message.
    pub fn diagnostic(&self) -> Option<&str> {
        match self {
            Self::NativeFailure { diagnostic } => diagnostic.as_deref(),
            Self::UnsupportedCapability | Self::InvalidIdentity | Self::StalePlan => None,
        }
    }
}

/// A failure while running a mutation on a backend.
#[derive(Debug, Error)]
pub enum OperationError {
    #[error("the package manager is busy or locked")]
    Busy,
    #[error("authorization was denied")]
    AuthorizationDenied,
    #[error("the network request failed")]
    NetworkFailure,
    #[error("a package dependency could not be satisfied")]
    DependencyFailure,
    #[error("package signature or trust verification failed")]
    SignatureFailure,
    #[error("there is not enough free disk space")]
    NoSpace,
    #[error("the operation timed out")]
    Timeout,
    #[error("the operation was cancelled")]
    Cancelled,
    #[error("the native package manager failed")]
    NativeFailure {
        /// Native output kept for diagnostics; never part of the user-facing message.
        diagnostic: Option<String>,
    },
}

impl OperationError {
    /// Native output that explains the failure without replacing its user message.
    pub fn diagnostic(&self) -> Option<&str> {
        match self {
            Self::NativeFailure { diagnostic } => diagnostic.as_deref(),
            Self::Busy
            | Self::AuthorizationDenied
            | Self::NetworkFailure
            | Self::DependencyFailure
            | Self::SignatureFailure
            | Self::NoSpace
            | Self::Timeout
            | Self::Cancelled => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_safe_message(error: &impl std::fmt::Display) {
        assert!(
            !error.to_string().trim().is_empty(),
            "error has no user-facing message"
        );
    }

    #[test]
    fn catalog_variants_are_distinct_and_have_safe_messages() {
        let errors = [
            CatalogError::UnavailableBackend,
            CatalogError::NotFound,
            CatalogError::InvalidIdentity,
            CatalogError::ParseFailure { diagnostic: None },
        ];

        for error in errors {
            assert_safe_message(&error);
            match &error {
                CatalogError::UnavailableBackend => {}
                CatalogError::NotFound => {}
                CatalogError::InvalidIdentity => {}
                CatalogError::ParseFailure { .. } => {}
            }
        }
    }

    #[test]
    fn plan_variants_are_distinct_and_have_safe_messages() {
        let errors = [
            PlanError::UnsupportedCapability,
            PlanError::InvalidIdentity,
            PlanError::StalePlan,
            PlanError::NativeFailure { diagnostic: None },
        ];

        for error in errors {
            assert_safe_message(&error);
            match &error {
                PlanError::UnsupportedCapability => {}
                PlanError::InvalidIdentity => {}
                PlanError::StalePlan => {}
                PlanError::NativeFailure { .. } => {}
            }
        }
    }

    #[test]
    fn operation_variants_are_distinct_and_have_safe_messages() {
        let errors = [
            OperationError::Busy,
            OperationError::AuthorizationDenied,
            OperationError::NetworkFailure,
            OperationError::DependencyFailure,
            OperationError::SignatureFailure,
            OperationError::NoSpace,
            OperationError::Timeout,
            OperationError::Cancelled,
            OperationError::NativeFailure { diagnostic: None },
        ];

        for error in errors {
            assert_safe_message(&error);
            match &error {
                OperationError::Busy => {}
                OperationError::AuthorizationDenied => {}
                OperationError::NetworkFailure => {}
                OperationError::DependencyFailure => {}
                OperationError::SignatureFailure => {}
                OperationError::NoSpace => {}
                OperationError::Timeout => {}
                OperationError::Cancelled => {}
                OperationError::NativeFailure { .. } => {}
            }
        }
    }

    #[test]
    fn catalog_diagnostic_is_separate_from_user_message() {
        let diagnostic = "unexpected token at line 3";
        let error = CatalogError::ParseFailure {
            diagnostic: Some(diagnostic.to_string()),
        };

        assert!(!error.to_string().contains(diagnostic));
        assert_eq!(error.diagnostic(), Some(diagnostic));
    }

    #[test]
    fn plan_diagnostic_is_separate_from_user_message() {
        let diagnostic = "native solver exited with status 1";
        let error = PlanError::NativeFailure {
            diagnostic: Some(diagnostic.to_string()),
        };

        assert!(!error.to_string().contains(diagnostic));
        assert_eq!(error.diagnostic(), Some(diagnostic));
    }

    #[test]
    fn operation_diagnostic_is_separate_from_user_message() {
        let diagnostic = "error: failed to synchronize database";
        let error = OperationError::NativeFailure {
            diagnostic: Some(diagnostic.to_string()),
        };

        assert!(!error.to_string().contains(diagnostic));
        assert_eq!(error.diagnostic(), Some(diagnostic));
    }

    #[test]
    fn cancellation_is_not_reported_as_a_native_failure() {
        let error = OperationError::Cancelled;

        assert_eq!(error.diagnostic(), None);
        assert!(!matches!(error, OperationError::NativeFailure { .. }));
    }
}
