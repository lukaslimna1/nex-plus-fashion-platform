use crate::core::db::CatalogDb;
use crate::core::error::CoreError;
use std::sync::Mutex;

pub struct CoreState {
    pub database: Mutex<CatalogDb>,
}

impl CoreState {
    pub fn new() -> Result<Self, CoreError> {
        Ok(Self {
            database: Mutex::new(CatalogDb::in_memory()?),
        })
    }
}
