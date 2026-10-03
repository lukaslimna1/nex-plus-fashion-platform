use crate::core::ai::AiRouter;
use crate::core::db::CatalogDb;
use crate::core::error::CoreError;
use crate::core::pack::PackRuntime;
use std::sync::Mutex;

pub struct CoreState {
    pub database: Mutex<CatalogDb>,
    pub packs: PackRuntime,
    pub ai: AiRouter,
}

impl CoreState {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, CoreError> {
        let path = path.as_ref();
        let pack_root = path
            .parent()
            .ok_or(CoreError::InvalidDatabasePath)?
            .join("packs");
        let packs = PackRuntime::new(pack_root)?;
        let mut database = CatalogDb::open(path)?;
        packs.ensure_available(&mut database)?;
        Ok(Self {
            database: Mutex::new(database),
            packs,
            ai: AiRouter::from_env(),
        })
    }
}
