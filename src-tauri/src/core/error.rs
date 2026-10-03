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
            Self::Pack(_) => "PACK_ERROR",
            Self::Database(_)
            | Self::Io(_)
            | Self::Serialization(_)
            | Self::InvalidDatabasePath
            | Self::StatePoisoned => "INTERNAL_ERROR",
        }
    }
}
