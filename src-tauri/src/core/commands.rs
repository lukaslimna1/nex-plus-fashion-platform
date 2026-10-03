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
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    Ok(CoreHealth {
        status: "ok",
        protocol_version: "0.1",
        storage: "sqlite",
        fts5: if database.has_fts5()? {
            "ready"
        } else {
            "missing"
        },
        migration_count: database.migration_names()?.len(),
    })
}

#[tauri::command]
pub fn core_migrations(state: State<'_, CoreState>) -> Result<Vec<String>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.migration_names()
}

#[tauri::command]
pub fn catalog_milano_ss27(
    state: State<'_, CoreState>,
) -> Result<crate::core::db::MilanoSnapshot, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.milano_snapshot()
}

#[tauri::command]
pub fn catalog_search(
    state: State<'_, CoreState>,
    query: String,
    limit: Option<u32>,
) -> Result<Vec<crate::core::db::SearchRow>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.search(&query, limit.unwrap_or(50).min(200))
}

#[tauri::command]
pub fn personal_note_upsert(
    state: State<'_, CoreState>,
    id: String,
    entity_id: String,
    body: String,
) -> Result<crate::core::db::PersonalNoteRow, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.upsert_personal_note(&id, &entity_id, &body)
}

#[tauri::command]
pub fn packs_list(
    state: State<'_, CoreState>,
) -> Result<Vec<crate::core::pack::PackSummary>, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    state.packs.list(&mut database)
}

#[tauri::command]
pub fn pack_install(
    state: State<'_, CoreState>,
    pack_id: String,
) -> Result<crate::core::pack::PackOperationResult, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    state.packs.install(&mut database, &pack_id)
}

#[tauri::command]
pub fn pack_remove(
    state: State<'_, CoreState>,
    pack_id: String,
) -> Result<crate::core::pack::PackOperationResult, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    state.packs.remove(&mut database, &pack_id)
}

#[tauri::command]
pub fn pack_repair(
    state: State<'_, CoreState>,
    pack_id: String,
) -> Result<crate::core::pack::PackOperationResult, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    state.packs.repair(&mut database, &pack_id)
}
