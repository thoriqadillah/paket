pub mod capability;
pub mod error;
pub mod identifier;
pub mod package;

pub use capability::{BackendAvailability, BackendInfo, Capability};
pub use error::{CatalogError, OperationError, PlanError};
pub use identifier::{BackendId, InstallationScope, PackageId, SourceId};
pub use package::{
    AppId, AppRecord, InstalledState, MetadataOrigin, PackageKind, PackageRecord, SearchScope,
    UpdateMetadata, UpdateRecord,
};
