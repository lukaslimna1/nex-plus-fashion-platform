use crate::core::db::CatalogDb;
use crate::core::error::CoreError;
use std::sync::Mutex;

pub struct CoreState {
    pub database: Mutex<CatalogDb>,
}

impl CoreState {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, CoreError> {
        let mut database = CatalogDb::open(path)?;
        database.seed_bundled_milano()?;
        Ok(Self {
            database: Mutex::new(database),
        })
    }
}
