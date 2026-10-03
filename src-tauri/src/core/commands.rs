use crate::core::error::CoreError;
use crate::core::state::CoreState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreHealth {
    pub status: &'static str,
    pub protocol_version: &'static str,
    pub storage: &'static str,
    pub fts5: &'static str,
    pub migration_count: usize,
}

#[tauri::command]
pub fn core_health(state: State<'_, CoreState>) -> Result<CoreHealth, CoreError> {
    let database = state.database.lock().map_err(|_| CoreError::StatePoisoned)?;
    Ok(CoreHealth {
        status: "ok",
        protocol_version: "0.1",
        storage: "sqlite",
        fts5: if database.has_fts5()? { "ready" } else { "missing" },
        migration_count: database.migration_names()?.len(),
    })
}

#[tauri::command]
pub fn core_migrations(state: State<'_, CoreState>) -> Result<Vec<String>, CoreError> {
    let database = state.database.lock().map_err(|_| CoreError::StatePoisoned)?;
    database.migration_names()
}
