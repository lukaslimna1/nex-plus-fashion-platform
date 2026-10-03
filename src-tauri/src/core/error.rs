use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("local database path has no parent directory")]
    InvalidDatabasePath,
    #[error("pack error: {0}")]
    Pack(String),
    #[error("pack not found: {0}")]
    PackNotFound(String),
    #[error("{resource} not found: {id}")]
    NotFound { resource: String, id: String },
    #[error("{resource} data is pending: {id}")]
    DataPending { resource: String, id: String },
    #[error("source unavailable: {0}")]
    SourceUnavailable(String),
    #[error("pack not installed: {0}")]
    PackNotInstalled(String),
    #[error("pack removed: {0}")]
    PackRemoved(String),
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("AI provider is not configured: {0}")]
    AiNotConfigured(String),
    #[error("AI provider error ({provider}): {message}")]
    AiProvider { provider: String, message: String },
    #[error("AI output validation failed: {0}")]
    AiValidation(String),
    #[error("AI policy error: {0}")]
    AiPolicy(String),
    #[error("AI ZERO cost policy blocked execution: {0}")]
    AiZeroCostPolicy(String),
    #[error("AI zero-cost guarantee is unavailable: {0}")]
    AiZeroCostNotGuaranteed(String),
    #[error("AI provider quota is exhausted: {0}")]
    AiQuotaExhausted(String),
    #[error("adapter not found: {0}")]
    AdapterNotFound(String),
    #[error("adapter capability is unsupported: {adapter_id} / {capability}")]
    AdapterCapabilityUnsupported {
        adapter_id: String,
        capability: String,
    },
    #[error("adapter error: {0}")]
    Adapter(String),
    #[error("adapter run cancelled")]
    AdapterCancelled,
    #[error("adapter browser automation is required but no browser worker is configured")]
    AdapterBrowserRequired,
    #[error("adapter response exceeded the configured maximum size")]
    AdapterResponseTooLarge,
    #[error("core state lock is poisoned")]
    StatePoisoned,
}

impl Serialize for CoreError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut error = serializer.serialize_struct("IpcError", 2)?;
        error.serialize_field("code", self.code())?;
        error.serialize_field("message", &self.to_string())?;
        error.end()
    }
}

impl CoreError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound { .. } => "NOT_FOUND",
            Self::DataPending { .. } => "DATA_PENDING",
            Self::SourceUnavailable(_) => "SOURCE_UNAVAILABLE",
            Self::PackNotInstalled(_) | Self::PackNotFound(_) => "PACK_NOT_INSTALLED",
            Self::PackRemoved(_) => "PACK_REMOVED",
            Self::InvalidRequest(_) => "INVALID_REQUEST",
            Self::AiNotConfigured(_) => "AI_NOT_CONFIGURED",
            Self::AiProvider { .. } => "AI_PROVIDER_ERROR",
            Self::AiValidation(_) => "AI_VALIDATION_FAILED",
            Self::AiPolicy(_) => "AI_POLICY_ERROR",
            Self::AiZeroCostPolicy(_) => "AI_ZERO_COST_POLICY",
            Self::AiZeroCostNotGuaranteed(_) => "AI_ZERO_COST_NOT_GUARANTEED",
            Self::AiQuotaExhausted(_) => "AI_QUOTA_EXHAUSTED",
            Self::AdapterNotFound(_) => "ADAPTER_NOT_FOUND",
            Self::AdapterCapabilityUnsupported { .. } => "ADAPTER_CAPABILITY_UNSUPPORTED",
            Self::Adapter(_) => "ADAPTER_ERROR",
            Self::AdapterCancelled => "ADAPTER_CANCELLED",
            Self::AdapterBrowserRequired => "ADAPTER_BROWSER_REQUIRED",
            Self::AdapterResponseTooLarge => "ADAPTER_RESPONSE_TOO_LARGE",
            Self::Pack(_) => "PACK_ERROR",
            Self::Database(_)
            | Self::Io(_)
            | Self::Serialization(_)
            | Self::InvalidDatabasePath
            | Self::StatePoisoned => "INTERNAL_ERROR",
        }
    }
}
