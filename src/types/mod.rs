pub mod errors;
pub mod permissions;
pub mod provider;

pub use errors::{AppError, Result};
pub use permissions::{PermissionConfig, PermissionPreset};
pub use provider::{AgentProvider, DetectedProvider, ProviderRegistry};
