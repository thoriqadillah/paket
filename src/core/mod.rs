pub mod capability;
pub mod error;
pub mod identifier;
pub mod package;
pub mod transaction;

pub use capability::{BackendAvailability, BackendInfo, Capability};
pub use error::{CatalogError, OperationError, PlanError};
pub use identifier::{BackendId, InstallationScope, PackageId, SourceId};
pub use package::{
    AppId, AppRecord, InstalledState, MetadataOrigin, PackageKind, PackageRecord, SearchScope,
    UpdateMetadata, UpdateRecord,
};
pub use transaction::{
    BackendOutcome, BackendPlan, BatchOutcome, BatchPlan, BatchSummary, ChangeAction, ChangeIntent,
    ChangeOrigin, JobStatus, PreviewQuality, ResolvedChange, StepStatus,
};
