use crate::core::adapter::{
    AdapterCandidateRequest, AdapterRunRequest, IntegrationProposalRequest, SourceCandidateRequest,
};
use crate::core::ai::{
    AiExecutionRead, AiHealthReport, AiRunResult, AiTaskRequest, CuratorProposalDecisionRequest,
    CuratorProposalRead,
};
use crate::core::error::CoreError;
use crate::core::read::{PageRequest, ReadPage, ReadState, READ_CONTRACT_VERSION};
use crate::core::state::CoreState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreHealth {
    pub status: &'static str,
    pub protocol_version: &'static str,
    pub contract_version: &'static str,
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
        contract_version: READ_CONTRACT_VERSION,
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
pub fn ai_health(state: State<'_, CoreState>) -> AiHealthReport {
    state.ai.health()
}

#[tauri::command]
pub fn ai_executions(
    state: State<'_, CoreState>,
    limit: Option<u32>,
) -> Result<Vec<AiExecutionRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.ai_executions(limit.unwrap_or(50))
}

#[tauri::command]
pub fn curator_ai_run(
    state: State<'_, CoreState>,
    request: AiTaskRequest,
) -> Result<AiRunResult, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    state.ai.run(&mut database, request)
}

#[tauri::command]
pub fn curator_proposals(
    state: State<'_, CoreState>,
    proposal_state: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<CuratorProposalRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.curator_proposals(proposal_state.as_deref(), limit.unwrap_or(50))
}

#[tauri::command]
pub fn curator_proposal_decide(
    state: State<'_, CoreState>,
    request: CuratorProposalDecisionRequest,
) -> Result<CuratorProposalRead, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.decide_curator_proposal(&request)
}

#[tauri::command]
pub fn curator_adapters(state: State<'_, CoreState>) -> crate::core::adapter::AdapterRegistryRead {
    state.adapters.read()
}

#[tauri::command]
pub fn curator_adapter_runs(
    state: State<'_, CoreState>,
    limit: Option<u32>,
) -> Result<Vec<crate::core::db::AdapterRunRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.adapter_runs(limit.unwrap_or(50))
}

#[tauri::command]
pub fn curator_raw_artifacts(
    state: State<'_, CoreState>,
    adapter_id: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<crate::core::db::RawArtifactRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.raw_artifacts(adapter_id.as_deref(), limit.unwrap_or(50))
}

#[tauri::command]
pub fn curator_ingestion_candidates(
    state: State<'_, CoreState>,
    adapter_id: Option<String>,
    status: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<crate::core::db::IngestionCandidateRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.ingestion_candidates(
        adapter_id.as_deref(),
        status.as_deref(),
        limit.unwrap_or(50),
    )
}

#[tauri::command]
pub fn curator_adapter_run(
    state: State<'_, CoreState>,
    request: AdapterRunRequest,
) -> Result<crate::core::db::AdapterRunRead, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    state.adapters.run(&mut database, &state.ai, request)
}

#[tauri::command]
pub fn curator_adapter_cancel(
    state: State<'_, CoreState>,
    cancel_key: String,
) -> Result<(), CoreError> {
    if cancel_key.trim().is_empty() {
        return Err(CoreError::InvalidRequest(
            "cancelKey is required".to_string(),
        ));
    }
    state.adapters.cancel(&cancel_key);
    Ok(())
}

#[tauri::command]
pub fn curator_source_candidates(
    state: State<'_, CoreState>,
    status: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<crate::core::db::SourceCandidateRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.source_candidates(status.as_deref(), limit.unwrap_or(50))
}

#[tauri::command]
pub fn curator_source_candidate_create(
    state: State<'_, CoreState>,
    request: SourceCandidateRequest,
) -> Result<crate::core::db::SourceCandidateRead, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    crate::core::adapter::create_source_candidate(&mut database, request)
}

#[tauri::command]
pub fn curator_adapter_candidates(
    state: State<'_, CoreState>,
    status: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<crate::core::db::AdapterCandidateRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.adapter_candidates(status.as_deref(), limit.unwrap_or(50))
}

#[tauri::command]
pub fn curator_adapter_candidate_create(
    state: State<'_, CoreState>,
    request: AdapterCandidateRequest,
) -> Result<crate::core::db::AdapterCandidateRead, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    crate::core::adapter::create_adapter_candidate(&mut database, request)
}

#[tauri::command]
pub fn curator_integration_proposals(
    state: State<'_, CoreState>,
    proposal_state: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<crate::core::db::IntegrationProposalRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.integration_proposals(proposal_state.as_deref(), limit.unwrap_or(50))
}

#[tauri::command]
pub fn curator_integration_proposal_create(
    state: State<'_, CoreState>,
    request: IntegrationProposalRequest,
) -> Result<crate::core::db::IntegrationProposalRead, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    crate::core::adapter::create_integration_proposal(&mut database, request)
}

#[tauri::command]
pub fn catalog_milano_ss27(
    state: State<'_, CoreState>,
) -> Result<crate::core::db::MilanoSnapshot, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    match database.read_state_for_pack("nex.fashion.milano.ss27")? {
        ReadState::Available => {}
        ReadState::PackRemoved => {
            return Err(CoreError::PackRemoved(
                "nex.fashion.milano.ss27".to_string(),
            ))
        }
        _ => {
            return Err(CoreError::PackNotInstalled(
                "nex.fashion.milano.ss27".to_string(),
            ))
        }
    }
    database.milano_snapshot()
}

#[tauri::command]
pub fn catalog_search(
    state: State<'_, CoreState>,
    query: String,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::db::SearchRow>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.search_page(&query, request.unwrap_or_default())
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

#[tauri::command]
pub fn catalog_geography(
    state: State<'_, CoreState>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::GeoCityRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_geography(request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_city_hubs(
    state: State<'_, CoreState>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::CityHubRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_city_hubs(request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_city_hub(
    state: State<'_, CoreState>,
    id: String,
) -> Result<crate::core::read::CityHubRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_city_hub(&id)
}

#[tauri::command]
pub fn catalog_events(
    state: State<'_, CoreState>,
    city_hub_id: Option<String>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::EventRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_events(city_hub_id.as_deref(), request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_event(
    state: State<'_, CoreState>,
    id: String,
) -> Result<crate::core::read::EventRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_event(&id)
}

#[tauri::command]
pub fn catalog_editions(
    state: State<'_, CoreState>,
    event_id: Option<String>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::EditionRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_editions(event_id.as_deref(), request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_edition(
    state: State<'_, CoreState>,
    id: String,
) -> Result<crate::core::read::EditionRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_edition(&id)
}

#[tauri::command]
pub fn catalog_schedule(
    state: State<'_, CoreState>,
    edition_id: String,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::ScheduleEntryRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_schedule(&edition_id, request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_venues(
    state: State<'_, CoreState>,
    city_id: Option<String>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::VenueRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_venues(city_id.as_deref(), request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_venue(
    state: State<'_, CoreState>,
    id: String,
) -> Result<crate::core::read::VenueRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_venue(&id)
}

#[tauri::command]
pub fn catalog_maisons(
    state: State<'_, CoreState>,
    edition_id: Option<String>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::MaisonRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_maisons(edition_id.as_deref(), request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_maison(
    state: State<'_, CoreState>,
    id: String,
) -> Result<crate::core::read::MaisonRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_maison(&id)
}

#[tauri::command]
pub fn catalog_persons(
    state: State<'_, CoreState>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::PersonRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_persons(request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_person(
    state: State<'_, CoreState>,
    id: String,
) -> Result<crate::core::read::PersonRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_person(&id)
}

#[tauri::command]
pub fn catalog_roles(
    state: State<'_, CoreState>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::RoleRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_roles(request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_person_roles(
    state: State<'_, CoreState>,
    person_id: Option<String>,
    context_entity_type: Option<String>,
    context_entity_id: Option<String>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::PersonRoleRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_person_roles(
        person_id.as_deref(),
        context_entity_type.as_deref(),
        context_entity_id.as_deref(),
        request.unwrap_or_default(),
    )
}

#[tauri::command]
pub fn catalog_collections(
    state: State<'_, CoreState>,
    edition_id: Option<String>,
    maison_id: Option<String>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::CollectionRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_collections(
        edition_id.as_deref(),
        maison_id.as_deref(),
        request.unwrap_or_default(),
    )
}

#[tauri::command]
pub fn catalog_collection(
    state: State<'_, CoreState>,
    id: String,
) -> Result<crate::core::read::CollectionRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_collection(&id)
}

#[tauri::command]
pub fn catalog_looks(
    state: State<'_, CoreState>,
    collection_id: String,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::LookRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_looks(&collection_id, request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_media(
    state: State<'_, CoreState>,
    entity_type: String,
    entity_id: String,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::MediaOccurrenceRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_media(&entity_type, &entity_id, request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_media_availability(
    state: State<'_, CoreState>,
    entity_type: String,
    entity_id: String,
) -> Result<crate::core::read::MediaAvailabilityRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_media_availability(&entity_type, &entity_id)
}

#[tauri::command]
pub fn catalog_reviews(
    state: State<'_, CoreState>,
    entity_type: String,
    entity_id: String,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::ReviewRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_reviews(&entity_type, &entity_id, request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_sources(
    state: State<'_, CoreState>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::SourceRead>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_sources(
        entity_type.as_deref(),
        entity_id.as_deref(),
        request.unwrap_or_default(),
    )
}

#[tauri::command]
pub fn catalog_source(
    state: State<'_, CoreState>,
    id: String,
) -> Result<crate::core::read::SourceRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_source(&id)
}

#[tauri::command]
pub fn catalog_provenance(
    state: State<'_, CoreState>,
    entity_type: String,
    entity_id: String,
    request: Option<PageRequest>,
) -> Result<ReadPage<crate::core::read::ProvenanceSummary>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_provenance(&entity_type, &entity_id, request.unwrap_or_default())
}

#[tauri::command]
pub fn catalog_terms(
    state: State<'_, CoreState>,
    source_id: String,
) -> Result<crate::core::read::ReadEnvelope<crate::core::read::TermsBasic>, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_terms(&source_id)
}

#[tauri::command]
pub fn personal_related(
    state: State<'_, CoreState>,
    entity_id: String,
) -> Result<crate::core::read::PersonalRelatedRead, CoreError> {
    let database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.read_personal_related(&entity_id)
}

#[tauri::command]
pub fn personal_favorite_set(
    state: State<'_, CoreState>,
    entity_id: String,
    is_favorite: bool,
) -> Result<crate::core::read::PersonalRelatedRead, CoreError> {
    let mut database = state
        .database
        .lock()
        .map_err(|_| CoreError::StatePoisoned)?;
    database.set_personal_favorite(&entity_id, is_favorite)
}
