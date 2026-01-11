pub mod errors;
pub mod provider;

pub use errors::{AppError, Result};
pub use provider::{AgentProvider, DetectedProvider, ProviderConfig, ProviderRegistry, SpawnCommand};
