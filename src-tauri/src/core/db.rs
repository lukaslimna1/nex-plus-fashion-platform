use crate::core::adapter::{
    AdapterCandidateRequest, AdapterHealth, AdapterSpec, IntegrationProposalRequest,
    ObservationInput, RawArtifactInput, SourceCandidateRequest,
};
use crate::core::ai::{
    AiCandidateRead, AiCandidateRecord, AiExecutionRead, AiExecutionRecord, AiRunResult,
    AiTargetRef, CuratorProposalDecisionRequest, CuratorProposalRead, CuratorProposalRecord,
};
use crate::core::error::CoreError;
use crate::core::milano::{bundled_pack, MilanoSeed, NexPack};
use crate::core::read::{
    CityHubRead, CollectionRead, EditionRead, EntityRef, EventRead, GeoCityRead, LookRead,
    MaisonRead, MediaAssetRead, MediaAvailabilityRead, MediaOccurrenceRead, PackReadRef,
    PageRequest, PersonRead, PersonRoleRead, PersonalNoteRead, PersonalRelatedRead,
    ProvenanceSummary, ReadEnvelope, ReadPage, ReadState, ReviewRead, RoleRead, ScheduleEntryRead,
    SearchResultRead, SourceEndpointRead, SourceRead, TermsBasic, VenueRead,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

const MIGRATIONS: [(&str, &str); 7] = [
    ("0001_core", include_str!("../../migrations/0001_core.sql")),
    ("0002_fts5", include_str!("../../migrations/0002_fts5.sql")),
    (
        "0003_milano_vertical",
        include_str!("../../migrations/0003_milano_vertical.sql"),
    ),
    (
        "0004_pack_runtime",
        include_str!("../../migrations/0004_pack_runtime.sql"),
    ),
    (
        "0005_personal_favorite",
        include_str!("../../migrations/0005_personal_favorite.sql"),
    ),
    (
        "0006_ai_curator",
        include_str!("../../migrations/0006_ai_curator.sql"),
    ),
    (
        "0007_adapter_framework",
        include_str!("../../migrations/0007_adapter_framework.sql"),
    ),
];

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MilanoSnapshot {
    pub city_hub: CityHubRow,
    pub event: EventRow,
    pub edition: EditionRow,
    pub schedule: Vec<ScheduleRow>,
    pub participant_count: i64,
    pub unresolved_participant_count: i64,
    pub source: SourceRow,
    pub gaps: Vec<GapRow>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CityHubRow {
    pub id: String,
    pub city_id: String,
    pub official_name: String,
    pub display_name: String,
    pub slug: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRow {
    pub id: String,
    pub city_hub_id: String,
    pub official_name: String,
    pub display_name: String,
    pub short_name: Option<String>,
    pub event_type: String,
    pub status: String,
    pub official_website: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditionRow {
    pub id: String,
    pub event_id: String,
    pub segment_id: Option<String>,
    pub display_name: String,
    pub season: String,
    pub season_code: String,
    pub season_year: i32,
    pub calendar_year: i32,
    pub start_date: String,
    pub end_date: String,
    pub time_zone: String,
    pub edition_status: String,
    pub official_page_url: Option<String>,
    pub official_calendar_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRow {
    pub id: String,
    pub edition_id: String,
    pub participant_id: String,
    pub participant_name_raw: String,
    pub maison_id: Option<String>,
    pub local_date: String,
    pub start_time_local: Option<String>,
    pub end_time_local: Option<String>,
    pub time_zone: String,
    pub format: String,
    pub schedule_status: String,
    pub location_status: String,
    pub delivery_mode: String,
    pub venue_id: Option<String>,
    pub venue_label: Option<String>,
    pub official_stream_url: Option<String>,
    pub official_entry_url: Option<String>,
    pub official_note: Option<String>,
    pub source_id: String,
    pub source_external_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceRow {
    pub id: String,
    pub name: String,
    pub source_kind: String,
    pub authority_tier: String,
    pub base_url: String,
    pub access_mode: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GapRow {
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: String,
    pub reason: String,
    pub status: String,
}

pub type SearchRow = SearchResultRead;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalNoteRow {
    pub id: String,
    pub entity_id: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct PackRuntimeRow {
    pub pack_id: String,
    pub version: String,
    pub family: String,
    pub scope_json: String,
    pub schema_version: String,
    pub app_compatibility_json: String,
    pub manifest_json: String,
    pub content_hash: String,
    pub artifact_hash: Option<String>,
    pub size_bytes: u64,
    pub state: String,
    pub progress: u8,
    pub staged_path: Option<String>,
    pub installed_path: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterRunRead {
    pub run_id: String,
    pub adapter_id: String,
    pub source_id: Option<String>,
    pub requested_capability: Option<String>,
    pub status: String,
    pub cursor: Option<String>,
    pub checkpoint: Value,
    pub fixture_mode: bool,
    pub attempts: u32,
    pub artifact_count: u32,
    pub observation_count: u32,
    pub candidate_count: u32,
    pub changed_count: u32,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub started_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RawArtifactStored {
    pub artifact_id: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawArtifactRead {
    pub artifact_id: String,
    pub source_id: Option<String>,
    pub adapter_id: String,
    pub integration_id: Option<String>,
    pub canonical_url: String,
    pub content_type: String,
    pub content_hash: String,
    pub byte_size: u64,
    pub storage_kind: String,
    pub content_ref: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub retrieved_at: String,
    pub retrieval_status: String,
}

#[derive(Debug, Clone)]
pub struct IngestionCandidateRecord {
    pub candidate_id: String,
    pub source_id: Option<String>,
    pub adapter_id: String,
    pub run_id: String,
    pub candidate_kind: String,
    pub target_entity_type: Option<String>,
    pub target_entity_id: Option<String>,
    pub stable_key: String,
    pub proposed: Value,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub comparison_state: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestionCandidateRead {
    pub candidate_id: String,
    pub source_id: Option<String>,
    pub adapter_id: String,
    pub run_id: String,
    pub candidate_kind: String,
    pub target_entity_type: Option<String>,
    pub target_entity_id: Option<String>,
    pub stable_key: String,
    pub proposed: Value,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub comparison_state: String,
    pub content_hash: String,
    pub status: String,
    pub seen_count: u32,
    pub first_seen_at: String,
    pub last_seen_at: String,
}

#[derive(Debug, Clone)]
pub struct IngestionCandidateUpsert {
    pub changed: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceCandidateRead {
    pub source_candidate_id: String,
    pub source_key: String,
    pub display_name: String,
    pub base_url: Option<String>,
    pub source_kind: Option<String>,
    pub domains: Vec<String>,
    pub locale: BTreeMap<String, String>,
    pub capabilities: Vec<String>,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub discovered_by: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterCandidateRead {
    pub adapter_candidate_id: String,
    pub source_candidate_id: Option<String>,
    pub adapter_id: Option<String>,
    pub proposed: Value,
    pub capabilities: Vec<String>,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationProposalRead {
    pub integration_proposal_id: String,
    pub source_candidate_id: Option<String>,
    pub adapter_candidate_id: Option<String>,
    pub source_id: Option<String>,
    pub adapter_id: Option<String>,
    pub proposal: Value,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub state: String,
    pub reviewer: Option<String>,
    pub decision_reason: Option<String>,
    pub decided_at: Option<String>,
    pub official_applied: bool,
}

pub struct CatalogDb {
    connection: Connection,
}

impl CatalogDb {
    #[cfg(test)]
    pub fn in_memory() -> Result<Self, CoreError> {
        let connection = Connection::open_in_memory()?;
        Self::from_connection(connection)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| CoreError::InvalidDatabasePath)?;
        }
        let connection = Connection::open(path)?;
        Self::from_connection(connection)
    }

    fn from_connection(connection: Connection) -> Result<Self, CoreError> {
        connection.pragma_update(None, "foreign_keys", "ON")?;
        let mut database = Self { connection };
        database.apply_migrations()?;
        Ok(database)
    }

    fn apply_migrations(&mut self) -> Result<(), CoreError> {
        let transaction = self.connection.transaction()?;
        for (name, sql) in MIGRATIONS {
            transaction.execute_batch(sql)?;
            transaction.execute(
                "INSERT OR IGNORE INTO schema_migration (name) VALUES (?1)",
                [name],
            )?;
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn migration_names(&self) -> Result<Vec<String>, CoreError> {
        let mut statement = self
            .connection
            .prepare("SELECT name FROM schema_migration ORDER BY name")?;
        let rows = statement.query_map([], |row| row.get(0))?;
        Ok(rows.collect::<Result<Vec<String>, _>>()?)
    }

    pub fn has_fts5(&self) -> Result<bool, CoreError> {
        let exists = self
            .connection
            .query_row(
                "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'catalog_fts'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .optional()?;
        Ok(exists.is_some())
    }

    pub fn current_timestamp(&self) -> Result<String, CoreError> {
        self.connection
            .query_row("SELECT strftime('%Y-%m-%dT%H:%M:%fZ', 'now')", [], |row| {
                row.get(0)
            })
            .map_err(CoreError::from)
    }

    pub fn upsert_adapter_spec(
        &mut self,
        spec: &AdapterSpec,
        health: &AdapterHealth,
        now: &str,
    ) -> Result<(), CoreError> {
        let source_ids = serde_json::to_string(&spec.source_ids)?;
        let integration_ids = serde_json::to_string(&spec.integration_ids)?;
        let capabilities = serde_json::to_string(&spec.capabilities)?;
        let content_types = serde_json::to_string(&spec.supported_content_types)?;
        let pagination = serde_json::to_string(&spec.pagination)?;
        let rate_limit = serde_json::to_string(&spec.rate_limit)?;
        let retry_policy = serde_json::to_string(&spec.retry_policy)?;
        let produces = serde_json::to_string(&spec.produces)?;
        self.connection.execute(
            "INSERT INTO adapter_registry
             (adapter_id, version, source_ids_json, integration_ids_json, capabilities_json,
              discovery_strategy, supported_content_types_json, fetch_strategy, parse_strategy,
              normalization_strategy, pagination_json, rate_limit_json, retry_policy_json,
              provenance_support, produces_json, fixture_support, test_support, health_state,
              health_detail, last_checked_at, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
                     ?16, ?17, ?18, ?19, ?20, ?21, ?21)
             ON CONFLICT(adapter_id) DO UPDATE SET
              version=excluded.version, source_ids_json=excluded.source_ids_json,
              integration_ids_json=excluded.integration_ids_json,
              capabilities_json=excluded.capabilities_json,
              discovery_strategy=excluded.discovery_strategy,
              supported_content_types_json=excluded.supported_content_types_json,
              fetch_strategy=excluded.fetch_strategy, parse_strategy=excluded.parse_strategy,
              normalization_strategy=excluded.normalization_strategy,
              pagination_json=excluded.pagination_json, rate_limit_json=excluded.rate_limit_json,
              retry_policy_json=excluded.retry_policy_json,
              provenance_support=excluded.provenance_support, produces_json=excluded.produces_json,
              fixture_support=excluded.fixture_support, test_support=excluded.test_support,
              health_state=excluded.health_state, health_detail=excluded.health_detail,
              last_checked_at=excluded.last_checked_at, updated_at=excluded.updated_at",
            params![
                spec.adapter_id,
                spec.version,
                source_ids,
                integration_ids,
                capabilities,
                spec.discovery_strategy,
                content_types,
                spec.fetch_strategy,
                spec.parse_strategy,
                spec.normalization_strategy,
                pagination,
                rate_limit,
                retry_policy,
                i32::from(spec.provenance_support),
                produces,
                i32::from(spec.fixture_support),
                i32::from(spec.test_support),
                health.state.as_str(),
                health.detail,
                health.checked_at.as_deref(),
                now,
            ],
        )?;
        for integration_id in &spec.integration_ids {
            let endpoint = match integration_id.as_str() {
                "integration:cnmi:milano-calendar" => {
                    "https://milanofashionweek.cameramoda.it/en/calendar"
                }
                "integration:bof:review-page" => {
                    "https://www.businessoffashion.com/reviews/fashion-week/"
                }
                _ => "",
            };
            self.connection.execute(
                "INSERT INTO integration_registry
                 (integration_id, source_id, adapter_id, endpoint_id, status, method, endpoint,
                  auth_policy, rate_limit_json, cost_policy, health_state, health_detail,
                  last_checked_at, created_at, updated_at)
                 VALUES (?1, ?2, ?3, NULL, 'active', ?4, ?5, 'none', ?6, 'ZERO', ?7, ?8,
                         ?9, ?9, ?9)
                 ON CONFLICT(integration_id) DO UPDATE SET source_id=excluded.source_id,
                  adapter_id=excluded.adapter_id, status=excluded.status, method=excluded.method,
                  endpoint=excluded.endpoint, rate_limit_json=excluded.rate_limit_json,
                  cost_policy=excluded.cost_policy, health_state=excluded.health_state,
                  health_detail=excluded.health_detail, last_checked_at=excluded.last_checked_at,
                  updated_at=excluded.updated_at",
                params![
                    integration_id,
                    spec.source_ids.first(),
                    spec.adapter_id,
                    spec.fetch_strategy,
                    endpoint,
                    serde_json::to_string(&spec.rate_limit)?,
                    health.state.as_str(),
                    health.detail,
                    now,
                ],
            )?;
        }
        for source_id in &spec.source_ids {
            self.connection.execute(
                "INSERT INTO source_health
                 (source_id, state, detail, last_checked_at, adapter_id)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(source_id) DO UPDATE SET
                  state=excluded.state, detail=excluded.detail,
                  last_checked_at=excluded.last_checked_at, adapter_id=excluded.adapter_id",
                params![
                    source_id,
                    health.state.as_str(),
                    health.detail,
                    health.checked_at.as_deref().unwrap_or(now),
                    spec.adapter_id
                ],
            )?;
        }
        Ok(())
    }

    pub fn start_adapter_run(
        &mut self,
        run_id: &str,
        adapter_id: &str,
        source_id: &str,
        capability: Option<&str>,
        cursor: Option<&str>,
        checkpoint_key: Option<&str>,
        fixture_mode: bool,
        now: &str,
    ) -> Result<(), CoreError> {
        let checkpoint = json!({
            "key": checkpoint_key,
            "cursor": cursor,
        });
        self.connection.execute(
            "INSERT INTO adapter_run
             (run_id, adapter_id, source_id, requested_capability, status, cursor,
              checkpoint_json, fixture_mode, attempts, started_at)
             VALUES (?1, ?2, ?3, ?4, 'running', ?5, ?6, ?7, 1, ?8)",
            params![
                run_id,
                adapter_id,
                source_id,
                capability,
                cursor,
                serde_json::to_string(&checkpoint)?,
                i32::from(fixture_mode),
                now
            ],
        )?;
        Ok(())
    }

    pub fn finish_adapter_run(
        &mut self,
        run_id: &str,
        status: &str,
        artifact_count: u32,
        observation_count: u32,
        candidate_count: u32,
        changed_count: u32,
        cursor: Option<&str>,
        error_code: Option<&str>,
        error_message: Option<&str>,
        completed_at: &str,
    ) -> Result<(), CoreError> {
        let changed = self.connection.execute(
            "UPDATE adapter_run
             SET status=?1, artifact_count=?2, observation_count=?3, candidate_count=?4,
                 changed_count=?5, cursor=?6, error_code=?7, error_message=?8, completed_at=?9,
                 checkpoint_json=json_set(checkpoint_json, '$.cursor', ?6)
             WHERE run_id=?10",
            params![
                status,
                i64::from(artifact_count),
                i64::from(observation_count),
                i64::from(candidate_count),
                i64::from(changed_count),
                cursor,
                error_code,
                error_message,
                completed_at,
                run_id
            ],
        )?;
        if changed == 0 {
            return Err(CoreError::NotFound {
                resource: "adapter_run".to_string(),
                id: run_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn adapter_run(&self, run_id: &str) -> Result<Option<AdapterRunRead>, CoreError> {
        self.connection
            .query_row(
                "SELECT run_id, adapter_id, source_id, requested_capability, status, cursor,
                        checkpoint_json, fixture_mode, attempts, artifact_count, observation_count,
                        candidate_count, changed_count, error_code, error_message, started_at,
                        completed_at
                 FROM adapter_run WHERE run_id=?1",
                [run_id],
                read_adapter_run,
            )
            .optional()
            .map_err(CoreError::from)
    }

    pub fn adapter_runs(&self, limit: u32) -> Result<Vec<AdapterRunRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT run_id, adapter_id, source_id, requested_capability, status, cursor,
                    checkpoint_json, fixture_mode, attempts, artifact_count, observation_count,
                    candidate_count, changed_count, error_code, error_message, started_at,
                    completed_at
             FROM adapter_run ORDER BY started_at DESC, run_id DESC LIMIT ?1",
        )?;
        let rows = statement
            .query_map([i64::from(limit.clamp(1, 200))], read_adapter_run)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn raw_artifacts(
        &self,
        adapter_id: Option<&str>,
        limit: u32,
    ) -> Result<Vec<RawArtifactRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT artifact_id, source_id, adapter_id, integration_id, canonical_url,
                    content_type, content_hash, byte_size, storage_kind, content_ref, etag,
                    last_modified, retrieved_at, retrieval_status
             FROM raw_artifact
             WHERE (?1='' OR adapter_id=?1)
             ORDER BY retrieved_at DESC, artifact_id DESC LIMIT ?2",
        )?;
        let rows = statement
            .query_map(
                params![adapter_id.unwrap_or(""), i64::from(limit.clamp(1, 200))],
                read_raw_artifact,
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn ingestion_candidates(
        &self,
        adapter_id: Option<&str>,
        status: Option<&str>,
        limit: u32,
    ) -> Result<Vec<IngestionCandidateRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT candidate_id, source_id, adapter_id, run_id, candidate_kind,
                    target_entity_type, target_entity_id, stable_key, proposed_json,
                    evidence_json, provenance_json, comparison_state, content_hash, status,
                    seen_count, first_seen_at, last_seen_at
             FROM ingestion_candidate
             WHERE (?1='' OR adapter_id=?1) AND (?2='' OR status=?2)
             ORDER BY updated_at DESC, candidate_id DESC LIMIT ?3",
        )?;
        let rows = statement
            .query_map(
                params![
                    adapter_id.unwrap_or(""),
                    status.unwrap_or(""),
                    i64::from(limit.clamp(1, 200))
                ],
                read_ingestion_candidate,
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn upsert_raw_artifact(
        &mut self,
        raw: &RawArtifactInput,
        adapter_id: &str,
    ) -> Result<RawArtifactStored, CoreError> {
        let canonical_url = normalize_source_url(&raw.canonical_url);
        let content_hash = sha256_bytes(raw.content.as_bytes());
        let existing = self
            .connection
            .query_row(
                "SELECT artifact_id FROM raw_artifact
                 WHERE adapter_id=?1 AND canonical_url=?2 AND content_hash=?3",
                params![adapter_id, canonical_url, content_hash],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        let artifact_id =
            existing.unwrap_or_else(|| format!("artifact:{adapter_id}:{content_hash}"));
        self.connection.execute(
            "INSERT INTO raw_artifact
             (artifact_id, source_id, adapter_id, integration_id, canonical_url, content_type,
              content_hash, byte_size, storage_kind, content_text, content_ref, etag,
              last_modified, retrieved_at, retrieval_status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'inline', ?9, ?10, ?11, ?12,
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), 'acquired',
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
             ON CONFLICT(adapter_id, canonical_url, content_hash) DO UPDATE SET
              source_id=excluded.source_id, integration_id=excluded.integration_id,
              content_type=excluded.content_type, byte_size=excluded.byte_size,
              content_text=excluded.content_text, content_ref=excluded.content_ref,
              etag=excluded.etag, last_modified=excluded.last_modified,
              retrieved_at=excluded.retrieved_at, retrieval_status=excluded.retrieval_status,
              updated_at=excluded.updated_at",
            params![
                artifact_id,
                raw.source_id,
                adapter_id,
                raw.integration_id,
                canonical_url,
                raw.content_type,
                content_hash,
                i64::try_from(raw.content.len()).unwrap_or(i64::MAX),
                raw.content,
                raw.content_ref,
                raw.etag,
                raw.last_modified,
            ],
        )?;
        Ok(RawArtifactStored {
            artifact_id,
            content_hash,
        })
    }

    pub fn upsert_source_observation(
        &mut self,
        artifact_id: &str,
        source_id: &str,
        adapter_id: &str,
        observation: &ObservationInput,
    ) -> Result<String, CoreError> {
        let observed_json = serde_json::to_string(&observation.payload)?;
        let content_hash = sha256_bytes(observed_json.as_bytes());
        let observation_id = format!(
            "observation:{adapter_id}:{}",
            sha256_bytes(format!("{}:{content_hash}", observation.stable_key).as_bytes())
        );
        self.connection.execute(
            "INSERT INTO source_observation
             (observation_id, artifact_id, source_id, adapter_id, entity_type, stable_key,
              observed_json, content_hash, parser_version, observed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, '1.0', strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
             ON CONFLICT(artifact_id, entity_type, stable_key, content_hash) DO UPDATE SET
              observed_json=excluded.observed_json, observed_at=excluded.observed_at",
            params![
                observation_id,
                artifact_id,
                source_id,
                adapter_id,
                observation.entity_type,
                observation.stable_key,
                observed_json,
                content_hash
            ],
        )?;
        Ok(observation_id)
    }

    pub fn compare_ingestion_candidate(
        &self,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        proposed: &Value,
    ) -> Result<String, CoreError> {
        let (Some(entity_type), Some(entity_id)) = (entity_type, entity_id) else {
            return Ok("UNKNOWN".to_string());
        };
        match entity_type {
            "schedule_entry" => {
                let official = self
                    .connection
                    .query_row(
                        "SELECT source_hash FROM schedule_entry WHERE id=?1",
                        [entity_id],
                        |row| row.get::<_, String>(0),
                    )
                    .optional()?;
                let Some(official) = official else {
                    return Ok("MISSING".to_string());
                };
                let proposed_hash = proposed
                    .get("sourceHash")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                Ok(if official == proposed_hash {
                    "MATCHED"
                } else {
                    "CHANGED"
                }
                .to_string())
            }
            "review" => {
                let exists = self
                    .connection
                    .query_row(
                        "SELECT 1 FROM catalog_review WHERE id=?1",
                        [entity_id],
                        |_| Ok(true),
                    )
                    .optional()?
                    .unwrap_or(false);
                Ok(if exists { "MATCHED" } else { "MISSING" }.to_string())
            }
            _ => {
                let exists = self
                    .connection
                    .query_row(
                        "SELECT 1 FROM catalog_entity WHERE id=?1",
                        [entity_id],
                        |_| Ok(true),
                    )
                    .optional()?
                    .unwrap_or(false);
                Ok(if exists { "MATCHED" } else { "MISSING" }.to_string())
            }
        }
    }

    pub fn upsert_ingestion_candidate(
        &mut self,
        record: &IngestionCandidateRecord,
    ) -> Result<IngestionCandidateUpsert, CoreError> {
        let proposed = serde_json::to_string(&record.proposed)?;
        let evidence = serde_json::to_string(&record.evidence)?;
        let provenance = serde_json::to_string(&record.provenance)?;
        let old_hash = self
            .connection
            .query_row(
                "SELECT content_hash FROM ingestion_candidate
                 WHERE adapter_id=?1 AND stable_key=?2",
                params![record.adapter_id, record.stable_key],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        let changed = old_hash
            .as_deref()
            .is_some_and(|hash| hash != record.content_hash);
        self.connection.execute(
            "INSERT INTO ingestion_candidate
             (candidate_id, source_id, adapter_id, run_id, candidate_kind, target_entity_type,
              target_entity_id, stable_key, proposed_json, evidence_json, provenance_json,
              comparison_state, content_hash, status, seen_count, first_seen_at, last_seen_at,
              created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'open', 1,
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
             ON CONFLICT(adapter_id, stable_key) DO UPDATE SET
              source_id=excluded.source_id, run_id=excluded.run_id,
              proposed_json=excluded.proposed_json, evidence_json=excluded.evidence_json,
              provenance_json=excluded.provenance_json,
              comparison_state=excluded.comparison_state, content_hash=excluded.content_hash,
              seen_count=ingestion_candidate.seen_count + 1,
              last_seen_at=excluded.last_seen_at, updated_at=excluded.updated_at",
            params![
                record.candidate_id,
                record.source_id,
                record.adapter_id,
                record.run_id,
                record.candidate_kind,
                record.target_entity_type,
                record.target_entity_id,
                record.stable_key,
                proposed,
                evidence,
                provenance,
                record.comparison_state,
                record.content_hash,
            ],
        )?;
        Ok(IngestionCandidateUpsert { changed })
    }

    pub fn upsert_ingestion_proposal(
        &mut self,
        candidate_id: &str,
        proposal_kind: &str,
        proposed: &Value,
        evidence: &[Value],
        provenance: &[Value],
        comparison_state: &str,
        reset_review: bool,
    ) -> Result<(), CoreError> {
        self.connection.execute(
            "INSERT INTO ingestion_proposal
             (proposal_id, candidate_id, proposal_kind, state, proposed_json, evidence_json,
              provenance_json, comparison_state, official_applied, created_at, updated_at)
             VALUES (?1, ?2, ?3, 'pending', ?4, ?5, ?6, ?7, 0,
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
             ON CONFLICT(candidate_id) DO UPDATE SET
              proposal_kind=excluded.proposal_kind, proposed_json=excluded.proposed_json,
              evidence_json=excluded.evidence_json, provenance_json=excluded.provenance_json,
              comparison_state=excluded.comparison_state,
              state=CASE WHEN ?8=1 THEN 'pending' ELSE ingestion_proposal.state END,
              official_applied=CASE WHEN ?8=1 THEN 0 ELSE ingestion_proposal.official_applied END,
              reviewer=CASE WHEN ?8=1 THEN NULL ELSE ingestion_proposal.reviewer END,
              decision_reason=CASE WHEN ?8=1 THEN NULL ELSE ingestion_proposal.decision_reason END,
              decided_at=CASE WHEN ?8=1 THEN NULL ELSE ingestion_proposal.decided_at END,
              updated_at=excluded.updated_at",
            params![
                format!("proposal:ingestion:{candidate_id}"),
                candidate_id,
                proposal_kind,
                serde_json::to_string(proposed)?,
                serde_json::to_string(evidence)?,
                serde_json::to_string(provenance)?,
                comparison_state,
                i32::from(reset_review)
            ],
        )?;
        Ok(())
    }

    pub fn upsert_ingestion_checkpoint(
        &mut self,
        adapter_id: &str,
        source_id: &str,
        checkpoint_key: &str,
        cursor: Option<&str>,
        content_hash: Option<&str>,
        updated_at: &str,
    ) -> Result<(), CoreError> {
        self.connection.execute(
            "INSERT INTO ingestion_checkpoint
             (adapter_id, source_id, checkpoint_key, cursor, content_hash, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(adapter_id, source_id, checkpoint_key) DO UPDATE SET
              cursor=excluded.cursor, content_hash=excluded.content_hash,
              updated_at=excluded.updated_at",
            params![
                adapter_id,
                source_id,
                checkpoint_key,
                cursor,
                content_hash,
                updated_at
            ],
        )?;
        Ok(())
    }

    pub fn upsert_source_candidate(
        &mut self,
        request: &SourceCandidateRequest,
    ) -> Result<SourceCandidateRead, CoreError> {
        let id = format!("source-candidate:{}", request.source_key);
        self.connection.execute(
            "INSERT INTO source_candidate
             (source_candidate_id, source_key, display_name, base_url, source_kind,
              domains_json, locale_json, capabilities_json, evidence_json, provenance_json,
              discovered_by, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 'pending',
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
             ON CONFLICT(source_key) DO UPDATE SET
              display_name=excluded.display_name, base_url=excluded.base_url,
              source_kind=excluded.source_kind, domains_json=excluded.domains_json,
              locale_json=excluded.locale_json, capabilities_json=excluded.capabilities_json,
              evidence_json=excluded.evidence_json, provenance_json=excluded.provenance_json,
              discovered_by=excluded.discovered_by, updated_at=excluded.updated_at",
            params![
                id,
                request.source_key,
                request.display_name,
                request.base_url,
                request.source_kind,
                serde_json::to_string(&request.domains)?,
                serde_json::to_string(&request.locale)?,
                serde_json::to_string(&request.capabilities)?,
                serde_json::to_string(&request.evidence)?,
                serde_json::to_string(&request.provenance)?,
                request.discovered_by.as_deref().unwrap_or("curator")
            ],
        )?;
        self.source_candidate(&id)?
            .ok_or_else(|| CoreError::NotFound {
                resource: "source_candidate".to_string(),
                id,
            })
    }

    pub fn create_adapter_candidate(
        &mut self,
        request: &AdapterCandidateRequest,
    ) -> Result<AdapterCandidateRead, CoreError> {
        let id = new_db_id("adapter-candidate");
        self.connection.execute(
            "INSERT INTO adapter_candidate
             (adapter_candidate_id, source_candidate_id, adapter_id, proposed_json,
              capabilities_json, evidence_json, provenance_json, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending',
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
            params![
                id,
                request.source_candidate_id,
                request.adapter_id,
                serde_json::to_string(&request.proposed)?,
                serde_json::to_string(&request.capabilities)?,
                serde_json::to_string(&request.evidence)?,
                serde_json::to_string(&request.provenance)?
            ],
        )?;
        self.adapter_candidate(&id)?
            .ok_or_else(|| CoreError::NotFound {
                resource: "adapter_candidate".to_string(),
                id,
            })
    }

    pub fn create_integration_proposal(
        &mut self,
        request: &IntegrationProposalRequest,
    ) -> Result<IntegrationProposalRead, CoreError> {
        let id = new_db_id("integration-proposal");
        self.connection.execute(
            "INSERT INTO integration_proposal
             (integration_proposal_id, source_candidate_id, adapter_candidate_id, source_id,
              adapter_id, proposal_json, evidence_json, provenance_json, state, official_applied,
              created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'pending', 0,
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
            params![
                id,
                request.source_candidate_id,
                request.adapter_candidate_id,
                request.source_id,
                request.adapter_id,
                serde_json::to_string(&request.proposal)?,
                serde_json::to_string(&request.evidence)?,
                serde_json::to_string(&request.provenance)?
            ],
        )?;
        self.integration_proposal(&id)?
            .ok_or_else(|| CoreError::NotFound {
                resource: "integration_proposal".to_string(),
                id,
            })
    }

    pub fn source_candidate(&self, id: &str) -> Result<Option<SourceCandidateRead>, CoreError> {
        self.connection
            .query_row(
                "SELECT source_candidate_id, source_key, display_name, base_url, source_kind,
                    domains_json, locale_json, capabilities_json, evidence_json,
                    provenance_json, discovered_by, status
             FROM source_candidate WHERE source_candidate_id=?1",
                [id],
                read_source_candidate,
            )
            .optional()
            .map_err(CoreError::from)
    }

    pub fn adapter_candidate(&self, id: &str) -> Result<Option<AdapterCandidateRead>, CoreError> {
        self.connection
            .query_row(
                "SELECT adapter_candidate_id, source_candidate_id, adapter_id, proposed_json,
                    capabilities_json, evidence_json, provenance_json, status
             FROM adapter_candidate WHERE adapter_candidate_id=?1",
                [id],
                read_adapter_candidate,
            )
            .optional()
            .map_err(CoreError::from)
    }

    pub fn integration_proposal(
        &self,
        id: &str,
    ) -> Result<Option<IntegrationProposalRead>, CoreError> {
        self.connection
            .query_row(
                "SELECT integration_proposal_id, source_candidate_id, adapter_candidate_id,
                    source_id, adapter_id, proposal_json, evidence_json, provenance_json,
                    state, reviewer, decision_reason, decided_at, official_applied
             FROM integration_proposal WHERE integration_proposal_id=?1",
                [id],
                read_integration_proposal,
            )
            .optional()
            .map_err(CoreError::from)
    }

    pub fn source_candidates(
        &self,
        status: Option<&str>,
        limit: u32,
    ) -> Result<Vec<SourceCandidateRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT source_candidate_id, source_key, display_name, base_url, source_kind,
                    domains_json, locale_json, capabilities_json, evidence_json,
                    provenance_json, discovered_by, status
             FROM source_candidate WHERE (?1='' OR status=?1)
             ORDER BY updated_at DESC, source_candidate_id DESC LIMIT ?2",
        )?;
        let rows = statement
            .query_map(
                params![status.unwrap_or(""), i64::from(limit.clamp(1, 200))],
                read_source_candidate,
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn adapter_candidates(
        &self,
        status: Option<&str>,
        limit: u32,
    ) -> Result<Vec<AdapterCandidateRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT adapter_candidate_id, source_candidate_id, adapter_id, proposed_json,
                    capabilities_json, evidence_json, provenance_json, status
             FROM adapter_candidate WHERE (?1='' OR status=?1)
             ORDER BY updated_at DESC, adapter_candidate_id DESC LIMIT ?2",
        )?;
        let rows = statement
            .query_map(
                params![status.unwrap_or(""), i64::from(limit.clamp(1, 200))],
                read_adapter_candidate,
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn integration_proposals(
        &self,
        state: Option<&str>,
        limit: u32,
    ) -> Result<Vec<IntegrationProposalRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT integration_proposal_id, source_candidate_id, adapter_candidate_id,
                    source_id, adapter_id, proposal_json, evidence_json, provenance_json,
                    state, reviewer, decision_reason, decided_at, official_applied
             FROM integration_proposal WHERE (?1='' OR state=?1)
             ORDER BY updated_at DESC, integration_proposal_id DESC LIMIT ?2",
        )?;
        let rows = statement
            .query_map(
                params![state.unwrap_or(""), i64::from(limit.clamp(1, 200))],
                read_integration_proposal,
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn ingestion_candidate_count(&self, adapter_id: &str) -> Result<i64, CoreError> {
        self.connection
            .query_row(
                "SELECT COUNT(*) FROM ingestion_candidate WHERE adapter_id=?1",
                [adapter_id],
                |row| row.get(0),
            )
            .map_err(CoreError::from)
    }

    pub fn ingestion_proposal_count(&self, adapter_id: &str) -> Result<i64, CoreError> {
        self.connection
            .query_row(
                "SELECT COUNT(*) FROM ingestion_proposal p
             JOIN ingestion_candidate c ON c.candidate_id=p.candidate_id
             WHERE c.adapter_id=?1",
                [adapter_id],
                |row| row.get(0),
            )
            .map_err(CoreError::from)
    }

    pub fn raw_artifact_count(&self, adapter_id: &str) -> Result<i64, CoreError> {
        self.connection
            .query_row(
                "SELECT COUNT(*) FROM raw_artifact WHERE adapter_id=?1",
                [adapter_id],
                |row| row.get(0),
            )
            .map_err(CoreError::from)
    }

    pub fn official_schedule_count(&self) -> Result<i64, CoreError> {
        self.connection
            .query_row("SELECT COUNT(*) FROM schedule_entry", [], |row| row.get(0))
            .map_err(CoreError::from)
    }

    pub fn official_review_count(&self) -> Result<i64, CoreError> {
        self.connection
            .query_row("SELECT COUNT(*) FROM catalog_review", [], |row| row.get(0))
            .map_err(CoreError::from)
    }

    pub fn ingestion_candidate_statuses(&self) -> Result<Vec<String>, CoreError> {
        let mut statement = self
            .connection
            .prepare("SELECT DISTINCT status FROM ingestion_candidate ORDER BY status")?;
        let rows = statement
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn ingestion_candidate_comparison_states(&self) -> Result<Vec<String>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT DISTINCT comparison_state
             FROM ingestion_candidate ORDER BY comparison_state",
        )?;
        let rows = statement
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn ingestion_proposal_states(&self) -> Result<Vec<String>, CoreError> {
        let mut statement = self
            .connection
            .prepare("SELECT DISTINCT state FROM ingestion_proposal ORDER BY state")?;
        let rows = statement
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn ingestion_candidate_payload(&self, adapter_id: &str) -> Result<Value, CoreError> {
        let payload: String = self.connection.query_row(
            "SELECT proposed_json FROM ingestion_candidate
             WHERE adapter_id=?1 ORDER BY candidate_id LIMIT 1",
            [adapter_id],
            |row| row.get(0),
        )?;
        Ok(serde_json::from_str(&payload)?)
    }

    pub fn persist_ai_success(
        &mut self,
        execution: &AiExecutionRecord,
        candidate: &AiCandidateRecord,
        proposal: &CuratorProposalRecord,
    ) -> Result<AiRunResult, CoreError> {
        let transaction = self.connection.transaction()?;
        insert_ai_execution(&transaction, execution)?;
        insert_ai_candidate(&transaction, candidate)?;
        insert_curator_proposal(&transaction, proposal)?;
        transaction.commit()?;
        Ok(AiRunResult {
            execution: execution_read(execution),
            candidate: candidate_read(candidate),
            proposal: proposal_read(proposal),
        })
    }

    pub fn persist_ai_failure(
        &mut self,
        execution: &AiExecutionRecord,
    ) -> Result<AiExecutionRead, CoreError> {
        let transaction = self.connection.transaction()?;
        insert_ai_execution(&transaction, execution)?;
        transaction.commit()?;
        Ok(execution_read(execution))
    }

    pub fn ai_executions(&self, limit: u32) -> Result<Vec<AiExecutionRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT execution_id, task_type, provider_id, model, capability, started_at,
                    completed_at, latency_ms, attempt, fallback_step, status, validator_schema,
                    validation_result_json, error_code, error_message, input_hash, usage_json,
                    cost_amount, candidate_id
             FROM ai_execution
             ORDER BY created_at DESC, execution_id DESC
             LIMIT ?1",
        )?;
        let rows = statement
            .query_map([i64::from(limit.clamp(1, 200))], |row| {
                read_ai_execution_row(row)
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn curator_proposals(
        &self,
        state: Option<&str>,
        limit: u32,
    ) -> Result<Vec<CuratorProposalRead>, CoreError> {
        let state = state.unwrap_or("");
        let mut statement = self.connection.prepare(
            "SELECT p.proposal_id, p.candidate_id, p.proposal_kind, p.state,
                    c.target_entity_type, c.target_entity_id, p.proposed_json, p.edited_json,
                    p.before_json, p.approved_json, p.evidence_json, p.provenance_json,
                    p.rationale, p.confidence, p.reviewer, p.decision_reason, p.decided_at,
                    p.official_applied
             FROM curator_proposal p
             JOIN ai_candidate c ON c.candidate_id=p.candidate_id
             WHERE (?1='' OR p.state=?1)
             ORDER BY p.created_at DESC, p.proposal_id DESC
             LIMIT ?2",
        )?;
        let rows = statement
            .query_map(params![state, i64::from(limit.clamp(1, 200))], |row| {
                read_curator_proposal_row(row)
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn decide_curator_proposal(
        &mut self,
        request: &CuratorProposalDecisionRequest,
    ) -> Result<CuratorProposalRead, CoreError> {
        if request.reviewer.trim().is_empty() {
            return Err(CoreError::InvalidRequest(
                "reviewer is required for a curator decision".to_string(),
            ));
        }
        let decision = request.decision.trim().to_ascii_uppercase();
        let (state, needs_edited) = match decision.as_str() {
            "APPROVE" => ("approved", false),
            "EDIT_APPROVE" | "EDIT_AND_APPROVE" => ("edited_approved", true),
            "REJECT" => ("rejected", false),
            "NEEDS_MORE_EVIDENCE" | "REQUEST_MORE_EVIDENCE" => ("needs_more_evidence", false),
            _ => {
                return Err(CoreError::InvalidRequest(
                    "decision must be APPROVE, EDIT_APPROVE, REJECT or NEEDS_MORE_EVIDENCE"
                        .to_string(),
                ))
            }
        };
        if needs_edited && request.edited.is_none() {
            return Err(CoreError::InvalidRequest(
                "edited payload is required for EDIT_APPROVE".to_string(),
            ));
        }

        let transaction = self.connection.transaction()?;
        let proposal = transaction
            .query_row(
                "SELECT p.proposal_id, p.candidate_id, p.proposal_kind, p.state,
                        c.target_entity_type, c.target_entity_id, p.proposed_json,
                        p.edited_json, p.before_json, p.approved_json, p.evidence_json,
                        p.provenance_json, p.rationale, p.confidence, p.reviewer,
                        p.decision_reason, p.decided_at, p.official_applied
                 FROM curator_proposal p
                 JOIN ai_candidate c ON c.candidate_id=p.candidate_id
                 WHERE p.proposal_id=?1",
                [&request.proposal_id],
                |row| read_curator_proposal_row(row),
            )
            .optional()?
            .ok_or_else(|| CoreError::NotFound {
                resource: "curator_proposal".to_string(),
                id: request.proposal_id.clone(),
            })?;
        if proposal.state != "pending" {
            return Err(CoreError::InvalidRequest(format!(
                "proposal is already {}",
                proposal.state
            )));
        }

        let selected = request
            .edited
            .clone()
            .unwrap_or_else(|| proposal.proposed.clone());
        let mut before = proposal.before.clone();
        let mut approved = None;
        let mut official_applied = false;
        if state == "approved" || state == "edited_approved" {
            let applied = apply_official_promotion(
                &transaction,
                &proposal.proposal_kind,
                &proposal.target,
                &selected,
            )?;
            before = applied.0.or(before);
            official_applied = applied.1;
            approved = Some(selected.clone());
        }
        let now: String =
            transaction.query_row("SELECT strftime('%Y-%m-%dT%H:%M:%fZ', 'now')", [], |row| {
                row.get(0)
            })?;
        transaction.execute(
            "UPDATE curator_proposal
             SET state=?1, edited_json=?2, before_json=?3, approved_json=?4,
                 reviewer=?5, decision_reason=?6, decided_at=?7,
                 official_applied=?8, updated_at=?7
             WHERE proposal_id=?9",
            params![
                state,
                request
                    .edited
                    .as_ref()
                    .map(serde_json::to_string)
                    .transpose()
                    .map_err(CoreError::from)?,
                before
                    .as_ref()
                    .map(serde_json::to_string)
                    .transpose()
                    .map_err(CoreError::from)?,
                approved
                    .as_ref()
                    .map(serde_json::to_string)
                    .transpose()
                    .map_err(CoreError::from)?,
                request.reviewer,
                request.reason,
                now,
                i32::from(official_applied),
                request.proposal_id,
            ],
        )?;
        transaction.execute(
            "UPDATE ai_candidate SET status=?1, updated_at=?2 WHERE candidate_id=?3",
            params![
                if official_applied || state == "rejected" {
                    if official_applied {
                        "converted"
                    } else {
                        "rejected"
                    }
                } else {
                    "open"
                },
                now,
                proposal.candidate_id,
            ],
        )?;
        transaction.commit()?;
        self.curator_proposals(None, 200)?
            .into_iter()
            .find(|item| item.proposal_id == request.proposal_id)
            .ok_or_else(|| CoreError::NotFound {
                resource: "curator_proposal".to_string(),
                id: request.proposal_id.clone(),
            })
    }

    pub fn read_state_for_pack(&self, pack_id: &str) -> Result<ReadState, CoreError> {
        let state = self
            .connection
            .query_row(
                "SELECT state FROM pack_runtime WHERE pack_id=?1",
                [pack_id],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        Ok(match state.as_deref() {
            Some("active") => ReadState::Available,
            Some("removed") => ReadState::PackRemoved,
            Some(_) | None => ReadState::PackNotInstalled,
        })
    }

    pub fn read_pack_ref(&self, pack_id: &str) -> Result<PackReadRef, CoreError> {
        self.connection
            .query_row(
                "SELECT pack_id, version, state FROM pack_runtime WHERE pack_id=?1",
                [pack_id],
                |row| {
                    Ok(PackReadRef {
                        pack_id: row.get(0)?,
                        version: row.get(1)?,
                        state: row.get(2)?,
                    })
                },
            )
            .map_err(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => {
                    CoreError::PackNotInstalled(pack_id.to_string())
                }
                other => CoreError::Database(other),
            })
    }

    fn require_milano_pack(&self) -> Result<PackReadRef, CoreError> {
        let pack = self.read_pack_ref("nex.fashion.milano.ss27")?;
        match pack.state.as_str() {
            "active" => Ok(pack),
            "removed" => Err(CoreError::PackRemoved(pack.pack_id)),
            _ => Err(CoreError::PackNotInstalled(pack.pack_id)),
        }
    }

    fn provenance_for(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Vec<ProvenanceSummary>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT source_id, evidence_url, retrieved_at, evidence_status, adapter_id
             FROM source_contribution
             WHERE entity_type=?1 AND entity_id=?2
             ORDER BY retrieved_at DESC, id",
        )?;
        let rows = statement
            .query_map(params![entity_type, entity_id], |row| {
                Ok(ProvenanceSummary {
                    source_id: row.get(0)?,
                    evidence_url: row.get(1)?,
                    retrieved_at: row.get(2)?,
                    evidence_status: row.get(3)?,
                    adapter_id: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn read_geography(&self, request: PageRequest) -> Result<ReadPage<GeoCityRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM geo_city", [], |row| {
                row.get::<_, i64>(0)
            })? as u64;
        let mut statement = self.connection.prepare(
            "SELECT c.id, c.name, c.aliases_json, co.id, co.name, r.id, r.name
             FROM geo_city c
             JOIN geo_country co ON co.id=c.country_id
             JOIN geo_region r ON r.id=co.region_id
             ORDER BY c.name
             LIMIT ?1 OFFSET ?2",
        )?;
        let rows = statement
            .query_map(
                params![i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    let aliases_json: String = row.get(2)?;
                    Ok(GeoCityRead {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        aliases: serde_json::from_str(&aliases_json).map_err(|error| {
                            rusqlite::Error::FromSqlConversionFailure(
                                2,
                                rusqlite::types::Type::Text,
                                Box::new(error),
                            )
                        })?,
                        country: EntityRef {
                            id: row.get(3)?,
                            entity_kind: "country".to_string(),
                            title: row.get(4)?,
                            slug: None,
                            subtitle: None,
                        },
                        region: EntityRef {
                            id: row.get(5)?,
                            entity_kind: "region".to_string(),
                            title: row.get(6)?,
                            slug: None,
                            subtitle: None,
                        },
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("city", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_city_hubs(&self, request: PageRequest) -> Result<ReadPage<CityHubRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let pack = self.require_milano_pack()?;
        let total: u64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM city_hub", [], |row| {
                row.get::<_, i64>(0)
            })? as u64;
        let mut statement = self.connection.prepare(
            "SELECT h.id, h.city_id, c.name, h.official_name, h.display_name, h.slug, h.status
             FROM city_hub h JOIN geo_city c ON c.id=h.city_id
             ORDER BY h.display_name
             LIMIT ?1 OFFSET ?2",
        )?;
        let rows = statement
            .query_map(
                params![i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(CityHubRead {
                        id: row.get(0)?,
                        city: EntityRef {
                            id: row.get(1)?,
                            entity_kind: "city".to_string(),
                            title: row.get(2)?,
                            slug: None,
                            subtitle: None,
                        },
                        official_name: row.get(3)?,
                        display_name: row.get(4)?,
                        slug: row.get(5)?,
                        status: row.get(6)?,
                        pack: pack.clone(),
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("city_hub", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_city_hub(&self, id: &str) -> Result<CityHubRead, CoreError> {
        let pack = self.require_milano_pack()?;
        let row = self.connection.query_row(
            "SELECT h.id, h.city_id, c.name, h.official_name, h.display_name, h.slug, h.status
             FROM city_hub h JOIN geo_city c ON c.id=h.city_id WHERE h.id=?1",
            [id],
            |row| {
                Ok(CityHubRead {
                    id: row.get(0)?,
                    city: EntityRef {
                        id: row.get(1)?,
                        entity_kind: "city".to_string(),
                        title: row.get(2)?,
                        slug: None,
                        subtitle: None,
                    },
                    official_name: row.get(3)?,
                    display_name: row.get(4)?,
                    slug: row.get(5)?,
                    status: row.get(6)?,
                    pack: pack.clone(),
                    provenance: Vec::new(),
                })
            },
        );
        match row {
            Ok(mut value) => {
                value.provenance = self.provenance_for("city_hub", id)?;
                Ok(value)
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(CoreError::NotFound {
                resource: "city_hub".to_string(),
                id: id.to_string(),
            }),
            Err(error) => Err(CoreError::Database(error)),
        }
    }

    pub fn read_events(
        &self,
        city_hub_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<EventRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let pack = self.require_milano_pack()?;
        let filter = city_hub_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_event WHERE (?1='' OR city_hub_id=?1)",
            [filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT e.id, e.city_hub_id, h.display_name, h.slug, e.official_name, e.display_name,
                    e.short_name, e.event_type, e.status, e.official_website
             FROM catalog_event e JOIN city_hub h ON h.id=e.city_hub_id
             WHERE (?1='' OR e.city_hub_id=?1)
             ORDER BY e.display_name
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![filter, i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(EventRead {
                        id: row.get(0)?,
                        city_hub: EntityRef {
                            id: row.get(1)?,
                            entity_kind: "city_hub".to_string(),
                            title: row.get(2)?,
                            slug: row.get(3)?,
                            subtitle: None,
                        },
                        official_name: row.get(4)?,
                        display_name: row.get(5)?,
                        short_name: row.get(6)?,
                        event_type: row.get(7)?,
                        status: row.get(8)?,
                        official_website: row.get(9)?,
                        pack: pack.clone(),
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("event", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_event(&self, id: &str) -> Result<EventRead, CoreError> {
        let mut items = self
            .read_events(
                None,
                PageRequest {
                    offset: 0,
                    limit: 200,
                },
            )?
            .items;
        if let Some(item) = items.drain(..).find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "event".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_editions(
        &self,
        event_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<EditionRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let pack = self.require_milano_pack()?;
        let filter = event_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_edition WHERE (?1='' OR event_id=?1)",
            [filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT d.id, d.event_id, e.display_name, e.official_name, d.segment_id,
                    s.name, d.display_name, d.season, d.season_code, d.season_year,
                    d.calendar_year, d.start_date, d.end_date, d.time_zone, d.edition_status,
                    d.official_page_url, d.official_calendar_url
             FROM catalog_edition d
             JOIN catalog_event e ON e.id=d.event_id
             LEFT JOIN catalog_segment s ON s.id=d.segment_id
             WHERE (?1='' OR d.event_id=?1)
             ORDER BY d.calendar_year DESC, d.display_name
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![filter, i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    let segment_id: Option<String> = row.get(4)?;
                    let segment_name: Option<String> = row.get(5)?;
                    Ok(EditionRead {
                        id: row.get(0)?,
                        event: EntityRef {
                            id: row.get(1)?,
                            entity_kind: "event".to_string(),
                            title: row.get(2)?,
                            slug: None,
                            subtitle: Some(row.get(3)?),
                        },
                        segment: segment_id.map(|id| EntityRef {
                            id,
                            entity_kind: "segment".to_string(),
                            title: segment_name.unwrap_or_default(),
                            slug: None,
                            subtitle: None,
                        }),
                        display_name: row.get(6)?,
                        season: row.get(7)?,
                        season_code: row.get(8)?,
                        season_year: row.get(9)?,
                        calendar_year: row.get(10)?,
                        start_date: row.get(11)?,
                        end_date: row.get(12)?,
                        time_zone: row.get(13)?,
                        edition_status: row.get(14)?,
                        official_page_url: row.get(15)?,
                        official_calendar_url: row.get(16)?,
                        pack: pack.clone(),
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("edition", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_edition(&self, id: &str) -> Result<EditionRead, CoreError> {
        let mut items = self
            .read_editions(
                None,
                PageRequest {
                    offset: 0,
                    limit: 200,
                },
            )?
            .items;
        if let Some(item) = items.drain(..).find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "edition".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_schedule(
        &self,
        edition_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<ScheduleEntryRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let _pack = self.require_milano_pack()?;
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM schedule_entry WHERE edition_id=?1",
            [edition_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT s.id, s.edition_id, s.participant_id, p.display_name, p.canonical_kind,
                    p.reconciliation_status, s.maison_id, m.display_name, m.slug,
                    s.local_date, s.start_time_local, s.end_time_local, s.time_zone,
                    s.format, s.schedule_status, s.location_status, s.delivery_mode,
                    s.venue_id, COALESCE(v.name, v.address), v.address, s.venue_label,
                    s.official_stream_url, s.official_entry_url, s.official_note
             FROM schedule_entry s
             JOIN participant_registry p ON p.id=s.participant_id
             LEFT JOIN catalog_maison m ON m.id=s.maison_id
             LEFT JOIN catalog_venue v ON v.id=s.venue_id
             WHERE s.edition_id=?1
             ORDER BY s.local_date, s.start_time_local, s.id
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![
                    edition_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let participant_kind: Option<String> = row.get(4)?;
                    let maison_id: Option<String> = row.get(6)?;
                    let maison_title: Option<String> = row.get(7)?;
                    let maison_slug: Option<String> = row.get(8)?;
                    let venue_id: Option<String> = row.get(17)?;
                    let venue_title: Option<String> = row.get(18)?;
                    let venue_address: Option<String> = row.get(19)?;
                    Ok(ScheduleEntryRead {
                        id: row.get(0)?,
                        edition_id: row.get(1)?,
                        participant: EntityRef {
                            id: row.get(2)?,
                            entity_kind: participant_kind
                                .unwrap_or_else(|| "participant".to_string()),
                            title: row.get(3)?,
                            slug: None,
                            subtitle: Some(row.get::<_, String>(5)?),
                        },
                        maison: maison_id.map(|id| EntityRef {
                            id,
                            entity_kind: "maison".to_string(),
                            title: maison_title.unwrap_or_default(),
                            slug: maison_slug,
                            subtitle: None,
                        }),
                        local_date: row.get(9)?,
                        start_time_local: row.get(10)?,
                        end_time_local: row.get(11)?,
                        time_zone: row.get(12)?,
                        format: row.get(13)?,
                        schedule_status: row.get(14)?,
                        location_status: row.get(15)?,
                        delivery_mode: row.get(16)?,
                        venue: venue_id.map(|id| EntityRef {
                            id,
                            entity_kind: "venue".to_string(),
                            title: venue_title.unwrap_or_default(),
                            slug: None,
                            subtitle: venue_address,
                        }),
                        venue_label: row.get(20)?,
                        official_stream_url: row.get(21)?,
                        official_entry_url: row.get(22)?,
                        official_note: row.get(23)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("schedule_entry", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_venues(
        &self,
        city_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<VenueRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let filter = city_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_venue WHERE (?1='' OR city_id=?1)",
            [filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT id, city_id, name, address, venue_type
             FROM catalog_venue
             WHERE (?1='' OR city_id=?1)
             ORDER BY COALESCE(name, address), id
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![filter, i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(VenueRead {
                        id: row.get(0)?,
                        city_id: row.get(1)?,
                        name: row.get(2)?,
                        address: row.get(3)?,
                        venue_type: row.get(4)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("venue", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_venue(&self, id: &str) -> Result<VenueRead, CoreError> {
        let mut items = self
            .read_venues(
                None,
                PageRequest {
                    offset: 0,
                    limit: 200,
                },
            )?
            .items;
        if let Some(item) = items.drain(..).find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "venue".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_maisons(
        &self,
        edition_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<MaisonRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let filter = edition_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(DISTINCT m.id)
             FROM catalog_maison m
             LEFT JOIN schedule_entry s ON s.maison_id=m.id
             WHERE (?1='' OR s.edition_id=?1)",
            [filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT DISTINCT m.id, m.official_name, m.display_name, m.slug, m.status, m.official_website
             FROM catalog_maison m
             LEFT JOIN schedule_entry s ON s.maison_id=m.id
             WHERE (?1='' OR s.edition_id=?1)
             ORDER BY m.display_name
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![filter, i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(MaisonRead {
                        id: row.get(0)?,
                        official_name: row.get(1)?,
                        display_name: row.get(2)?,
                        slug: row.get(3)?,
                        status: row.get(4)?,
                        official_website: row.get(5)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("maison", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_maison(&self, id: &str) -> Result<MaisonRead, CoreError> {
        let mut items = self
            .read_maisons(
                None,
                PageRequest {
                    offset: 0,
                    limit: 200,
                },
            )?
            .items;
        if let Some(item) = items.drain(..).find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "maison".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_persons(&self, request: PageRequest) -> Result<ReadPage<PersonRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 =
            self.connection
                .query_row("SELECT COUNT(*) FROM catalog_person", [], |row| {
                    row.get::<_, i64>(0)
                })? as u64;
        let mut statement = self.connection.prepare(
            "SELECT id, full_name, display_name, biography, status
             FROM catalog_person ORDER BY COALESCE(display_name, full_name), id
             LIMIT ?1 OFFSET ?2",
        )?;
        let rows = statement
            .query_map(
                params![i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(PersonRead {
                        id: row.get(0)?,
                        full_name: row.get(1)?,
                        display_name: row.get(2)?,
                        biography: row.get(3)?,
                        status: row.get(4)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("person", &item.id)?;
        }
        Ok(ReadPage::new(
            items,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_person(&self, id: &str) -> Result<PersonRead, CoreError> {
        let _pack = self.require_milano_pack()?;
        let result = self.connection.query_row(
            "SELECT id, full_name, display_name, biography, status
             FROM catalog_person WHERE id=?1",
            [id],
            |row| {
                Ok(PersonRead {
                    id: row.get(0)?,
                    full_name: row.get(1)?,
                    display_name: row.get(2)?,
                    biography: row.get(3)?,
                    status: row.get(4)?,
                    provenance: Vec::new(),
                })
            },
        );
        match result {
            Ok(mut person) => {
                person.provenance = self.provenance_for("person", id)?;
                Ok(person)
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(CoreError::NotFound {
                resource: "person".to_string(),
                id: id.to_string(),
            }),
            Err(error) => Err(CoreError::Database(error)),
        }
    }

    pub fn read_person_roles(
        &self,
        person_id: Option<&str>,
        context_entity_type: Option<&str>,
        context_entity_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<PersonRoleRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let person_filter = person_id.unwrap_or("");
        let context_type_filter = context_entity_type.unwrap_or("");
        let context_id_filter = context_entity_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM person_role
             WHERE (?1='' OR person_id=?1)
               AND (?2='' OR context_entity_type=?2)
               AND (?3='' OR context_entity_id=?3)",
            params![person_filter, context_type_filter, context_id_filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT pr.id, pr.person_id, p.display_name, p.full_name, pr.role_id,
                    r.name_pt_br, r.international_name, r.role_category,
                    pr.context_entity_type, pr.context_entity_id, pr.official_role_title,
                    pr.start_date, pr.end_date, pr.is_current
             FROM person_role pr
             JOIN catalog_person p ON p.id=pr.person_id
             JOIN catalog_role r ON r.id=pr.role_id
             WHERE (?1='' OR pr.person_id=?1)
               AND (?2='' OR pr.context_entity_type=?2)
               AND (?3='' OR pr.context_entity_id=?3)
             ORDER BY p.display_name, r.name_pt_br, pr.id
             LIMIT ?4 OFFSET ?5",
        )?;
        let rows = statement
            .query_map(
                params![
                    person_filter,
                    context_type_filter,
                    context_id_filter,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    Ok(PersonRoleRead {
                        id: row.get(0)?,
                        person: EntityRef {
                            id: row.get(1)?,
                            entity_kind: "person".to_string(),
                            title: row
                                .get::<_, Option<String>>(2)?
                                .unwrap_or_else(|| row.get::<_, String>(3).unwrap_or_default()),
                            slug: None,
                            subtitle: None,
                        },
                        role: RoleRead {
                            id: row.get(4)?,
                            name_pt_br: row.get(5)?,
                            international_name: row.get(6)?,
                            role_category: row.get(7)?,
                        },
                        context_entity_type: row.get(8)?,
                        context_entity_id: row.get(9)?,
                        official_role_title: row.get(10)?,
                        start_date: row.get(11)?,
                        end_date: row.get(12)?,
                        is_current: row.get::<_, i64>(13)? != 0,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_roles(&self, request: PageRequest) -> Result<ReadPage<RoleRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 =
            self.connection
                .query_row("SELECT COUNT(*) FROM catalog_role", [], |row| {
                    row.get::<_, i64>(0)
                })? as u64;
        let mut statement = self.connection.prepare(
            "SELECT id, name_pt_br, international_name, role_category
             FROM catalog_role ORDER BY name_pt_br, id LIMIT ?1 OFFSET ?2",
        )?;
        let rows = statement
            .query_map(
                params![i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(RoleRead {
                        id: row.get(0)?,
                        name_pt_br: row.get(1)?,
                        international_name: row.get(2)?,
                        role_category: row.get(3)?,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_collections(
        &self,
        edition_id: Option<&str>,
        maison_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<CollectionRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let edition_filter = edition_id.unwrap_or("");
        let maison_filter = maison_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_collection
             WHERE (?1='' OR edition_id=?1) AND (?2='' OR maison_id=?2)",
            params![edition_filter, maison_filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT c.id, c.maison_id, m.display_name, m.slug, c.edition_id, e.display_name,
                    c.schedule_entry_id, s.participant_id, p.display_name, c.display_name,
                    c.about, c.presented_at, c.presentation_format, c.season, c.season_year,
                    c.official_collection_url, c.press_release_url
             FROM catalog_collection c
             LEFT JOIN catalog_maison m ON m.id=c.maison_id
             LEFT JOIN catalog_edition e ON e.id=c.edition_id
             LEFT JOIN schedule_entry s ON s.id=c.schedule_entry_id
             LEFT JOIN participant_registry p ON p.id=s.participant_id
             WHERE (?1='' OR c.edition_id=?1) AND (?2='' OR c.maison_id=?2)
             ORDER BY c.display_name, c.id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    edition_filter,
                    maison_filter,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let maison_id: Option<String> = row.get(1)?;
                    let maison_title: Option<String> = row.get(2)?;
                    let maison_slug: Option<String> = row.get(3)?;
                    let edition_id: Option<String> = row.get(4)?;
                    let edition_title: Option<String> = row.get(5)?;
                    let schedule_entry_id: Option<String> = row.get(6)?;
                    let schedule_participant_id: Option<String> = row.get(7)?;
                    let schedule_title: Option<String> = row.get(8)?;
                    Ok(CollectionRead {
                        id: row.get(0)?,
                        maison: maison_id.map(|id| EntityRef {
                            id,
                            entity_kind: "maison".to_string(),
                            title: maison_title.unwrap_or_default(),
                            slug: maison_slug,
                            subtitle: None,
                        }),
                        edition: edition_id.map(|id| EntityRef {
                            id,
                            entity_kind: "edition".to_string(),
                            title: edition_title.unwrap_or_default(),
                            slug: None,
                            subtitle: None,
                        }),
                        schedule_entry: schedule_entry_id.map(|id| EntityRef {
                            id,
                            entity_kind: "schedule_entry".to_string(),
                            title: schedule_title.unwrap_or_default(),
                            slug: None,
                            subtitle: schedule_participant_id,
                        }),
                        display_name: row.get(9)?,
                        about: row.get(10)?,
                        presented_at: row.get(11)?,
                        presentation_format: row.get(12)?,
                        season: row.get(13)?,
                        season_year: row.get(14)?,
                        official_collection_url: row.get(15)?,
                        press_release_url: row.get(16)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("collection", &item.id)?;
        }
        Ok(ReadPage::new(
            items,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_collection(&self, id: &str) -> Result<CollectionRead, CoreError> {
        let page = self.read_collections(
            None,
            None,
            PageRequest {
                offset: 0,
                limit: 200,
            },
        )?;
        if let Some(item) = page.items.into_iter().find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::DataPending {
            resource: "collection".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_looks(
        &self,
        _collection_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<LookRead>, CoreError> {
        let _pack = self.require_milano_pack()?;
        Ok(ReadPage::new(
            Vec::new(),
            0,
            request,
            ReadState::DataPending,
        ))
    }

    pub fn read_media(
        &self,
        entity_type: &str,
        entity_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<MediaOccurrenceRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM media_occurrence WHERE entity_type=?1 AND entity_id=?2",
            params![entity_type, entity_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT o.id, o.media_asset_id, a.media_type, a.title, a.remote_render_policy,
                    o.source_id, o.entity_type, o.entity_id, o.remote_url, o.page_url,
                    o.source_asset_key, o.source_sequence, o.credit, o.asset_health, o.verified_at
             FROM media_occurrence o
             LEFT JOIN catalog_media_asset a ON a.id=o.media_asset_id
             WHERE o.entity_type=?1 AND o.entity_id=?2
             ORDER BY o.source_sequence, o.id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    entity_type,
                    entity_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let asset_id: Option<String> = row.get(1)?;
                    let media_type: Option<String> = row.get(2)?;
                    let title: Option<String> = row.get(3)?;
                    let remote_render_policy: Option<String> = row.get(4)?;
                    Ok(MediaOccurrenceRead {
                        id: row.get(0)?,
                        asset: asset_id.map(|id| MediaAssetRead {
                            id,
                            media_type: media_type.unwrap_or_else(|| "unknown".to_string()),
                            title,
                            remote_render_policy: remote_render_policy
                                .unwrap_or_else(|| "remote_render".to_string()),
                        }),
                        source_id: row.get(5)?,
                        entity_type: row.get(6)?,
                        entity_id: row.get(7)?,
                        remote_url: row.get(8)?,
                        page_url: row.get(9)?,
                        source_asset_key: row.get(10)?,
                        source_sequence: row.get(11)?,
                        credit: row.get(12)?,
                        asset_health: row.get(13)?,
                        verified_at: row.get(14)?,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_media_availability(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<MediaAvailabilityRead, CoreError> {
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(MediaAvailabilityRead {
                entity_type: entity_type.to_string(),
                entity_id: entity_id.to_string(),
                media_available: false,
                video_available: false,
                image_available: false,
                state,
            });
        }
        let (count, video_count, image_count): (i64, i64, i64) = self.connection.query_row(
            "SELECT COUNT(*),
                    SUM(CASE WHEN COALESCE(a.media_type,'') IN ('video','stream') THEN 1 ELSE 0 END),
                    SUM(CASE WHEN COALESCE(a.media_type,'') IN ('image','photo','gallery') THEN 1 ELSE 0 END)
             FROM media_occurrence o
             LEFT JOIN catalog_media_asset a ON a.id=o.media_asset_id
             WHERE o.entity_type=?1 AND o.entity_id=?2",
            params![entity_type, entity_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get::<_, Option<i64>>(1)?.unwrap_or(0),
                    row.get::<_, Option<i64>>(2)?.unwrap_or(0),
                ))
            },
        )?;
        Ok(MediaAvailabilityRead {
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            media_available: count > 0,
            video_available: video_count > 0,
            image_available: image_count > 0,
            state: if count == 0 {
                ReadState::DataPending
            } else {
                state
            },
        })
    }

    pub fn read_reviews(
        &self,
        entity_type: &str,
        entity_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<ReviewRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_review WHERE entity_type=?1 AND entity_id=?2",
            params![entity_type, entity_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT id, source_id, entity_type, entity_id, author, title, published_at,
                    language, original_url, summary, key_points_json
             FROM catalog_review
             WHERE entity_type=?1 AND entity_id=?2
             ORDER BY published_at DESC, id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    entity_type,
                    entity_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let key_points_json: String = row.get(10)?;
                    Ok(ReviewRead {
                        id: row.get(0)?,
                        source_id: row.get(1)?,
                        entity_type: row.get(2)?,
                        entity_id: row.get(3)?,
                        author: row.get(4)?,
                        title: row.get(5)?,
                        published_at: row.get(6)?,
                        language: row.get(7)?,
                        original_url: row.get(8)?,
                        summary: row.get(9)?,
                        key_points: serde_json::from_str(&key_points_json).map_err(|error| {
                            rusqlite::Error::FromSqlConversionFailure(
                                10,
                                rusqlite::types::Type::Text,
                                Box::new(error),
                            )
                        })?,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_sources(
        &self,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<SourceRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let entity_type = entity_type.unwrap_or("");
        let entity_id = entity_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(DISTINCT s.id)
             FROM source_registry s
             LEFT JOIN source_contribution c ON c.source_id=s.id
             WHERE (?1='' OR (c.entity_type=?1 AND c.entity_id=?2))",
            params![entity_type, entity_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT DISTINCT s.id, s.name, s.source_kind, s.authority_tier, s.base_url,
                    s.access_mode, s.status, s.terms_url, s.rights_notes
             FROM source_registry s
             LEFT JOIN source_contribution c ON c.source_id=s.id
             WHERE (?1='' OR (c.entity_type=?1 AND c.entity_id=?2))
             ORDER BY s.name, s.id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    entity_type,
                    entity_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    Ok(SourceRead {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        source_kind: row.get(2)?,
                        authority_tier: row.get(3)?,
                        base_url: row.get(4)?,
                        access_mode: row.get(5)?,
                        status: row.get(6)?,
                        terms: TermsBasic {
                            source_id: row.get(0)?,
                            terms_url: row.get(7)?,
                            rights_notes: row.get(8)?,
                            access_mode: row.get(5)?,
                        },
                        endpoints: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.endpoints = self.read_source_endpoints(&item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    fn read_source_endpoints(&self, source_id: &str) -> Result<Vec<SourceEndpointRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT id, source_id, endpoint_type, base_url, access_method,
                    capabilities_json, adapter_id, status, last_verified_at
             FROM source_endpoint WHERE source_id=?1 ORDER BY endpoint_type, id",
        )?;
        let rows = statement
            .query_map([source_id], |row| {
                let capabilities_json: String = row.get(5)?;
                Ok(SourceEndpointRead {
                    id: row.get(0)?,
                    source_id: row.get(1)?,
                    endpoint_type: row.get(2)?,
                    base_url: row.get(3)?,
                    access_method: row.get(4)?,
                    capabilities: serde_json::from_str(&capabilities_json).map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            5,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?,
                    adapter_id: row.get(6)?,
                    status: row.get(7)?,
                    last_verified_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>();
        rows.map_err(CoreError::from)
    }

    pub fn read_source(&self, id: &str) -> Result<SourceRead, CoreError> {
        let page = self.read_sources(
            None,
            None,
            PageRequest {
                offset: 0,
                limit: 200,
            },
        )?;
        if let Some(item) = page.items.into_iter().find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "source".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_terms(&self, source_id: &str) -> Result<ReadEnvelope<TermsBasic>, CoreError> {
        let source = self.read_source(source_id)?;
        let state = if source.terms.terms_url.is_some() || source.terms.rights_notes.is_some() {
            ReadState::Available
        } else {
            ReadState::DataPending
        };
        Ok(ReadEnvelope {
            data: Some(source.terms),
            state,
        })
    }

    pub fn read_provenance(
        &self,
        entity_type: &str,
        entity_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<ProvenanceSummary>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) && entity_type != "personal_note" {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM source_contribution WHERE entity_type=?1 AND entity_id=?2",
            params![entity_type, entity_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT source_id, evidence_url, retrieved_at, evidence_status, adapter_id
             FROM source_contribution
             WHERE entity_type=?1 AND entity_id=?2
             ORDER BY retrieved_at DESC, id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    entity_type,
                    entity_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    Ok(ProvenanceSummary {
                        source_id: row.get(0)?,
                        evidence_url: row.get(1)?,
                        retrieved_at: row.get(2)?,
                        evidence_status: row.get(3)?,
                        adapter_id: row.get(4)?,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                ReadState::Available
            },
        ))
    }

    pub fn read_personal_related(&self, entity_id: &str) -> Result<PersonalRelatedRead, CoreError> {
        let is_favorite = self.connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM personal_favorite WHERE entity_id=?1)",
            [entity_id],
            |row| row.get::<_, i64>(0),
        )? != 0;
        let mut statement = self.connection.prepare(
            "SELECT id, entity_id, body, created_at, updated_at
             FROM personal_note WHERE entity_id=?1 ORDER BY updated_at DESC, id",
        )?;
        let notes = statement
            .query_map([entity_id], |row| {
                Ok(PersonalNoteRead {
                    id: row.get(0)?,
                    entity_id: row.get(1)?,
                    body: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PersonalRelatedRead {
            entity_id: entity_id.to_string(),
            is_favorite,
            notes,
        })
    }

    pub fn set_personal_favorite(
        &mut self,
        entity_id: &str,
        is_favorite: bool,
    ) -> Result<PersonalRelatedRead, CoreError> {
        if is_favorite {
            self.connection.execute(
                "INSERT OR IGNORE INTO personal_favorite (entity_id, created_at)
                 VALUES (?1, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
                [entity_id],
            )?;
        } else {
            self.connection.execute(
                "DELETE FROM personal_favorite WHERE entity_id=?1",
                [entity_id],
            )?;
        }
        self.read_personal_related(entity_id)
    }

    pub fn ensure_pack_available(
        &mut self,
        pack: &NexPack,
        artifact_size: u64,
    ) -> Result<(), CoreError> {
        let transaction = self.connection.transaction()?;
        let manifest_json = serde_json::to_string(&pack.manifest)?;
        let scope_json = serde_json::to_string(&pack.manifest.scope)?;
        let compatibility_json = serde_json::to_string(&pack.manifest.app_compatibility)?;
        let now = pack.manifest.origin.retrieved_at.as_str();
        transaction.execute(
            "INSERT INTO pack_runtime (pack_id, version, family, scope_json, schema_version, app_compatibility_json, manifest_json, content_hash, size_bytes, state, progress, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'available', 0, ?10, ?10)
             ON CONFLICT(pack_id) DO UPDATE SET version=excluded.version, family=excluded.family,
               scope_json=excluded.scope_json, schema_version=excluded.schema_version,
               app_compatibility_json=excluded.app_compatibility_json, manifest_json=excluded.manifest_json,
               content_hash=excluded.content_hash, size_bytes=excluded.size_bytes,
               state=CASE WHEN pack_runtime.version <> excluded.version THEN 'available' ELSE pack_runtime.state END,
               progress=CASE WHEN pack_runtime.version <> excluded.version THEN 0 ELSE pack_runtime.progress END,
               last_error=CASE WHEN pack_runtime.version <> excluded.version THEN NULL ELSE pack_runtime.last_error END,
               updated_at=excluded.updated_at",
            params![
                pack.manifest.pack_id,
                pack.manifest.version,
                pack.manifest.family,
                scope_json,
                pack.manifest.schema_version,
                compatibility_json,
                manifest_json,
                pack.manifest.content_hash,
                artifact_size,
                now
            ],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn pack_runtime_rows(&self) -> Result<Vec<PackRuntimeRow>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT pack_id, version, family, scope_json, schema_version, app_compatibility_json,
                    manifest_json, content_hash, artifact_hash, size_bytes, state, progress,
                    staged_path, installed_path, last_error
             FROM pack_runtime ORDER BY pack_id",
        )?;
        let rows = statement
            .query_map([], |row| {
                Ok(PackRuntimeRow {
                    pack_id: row.get(0)?,
                    version: row.get(1)?,
                    family: row.get(2)?,
                    scope_json: row.get(3)?,
                    schema_version: row.get(4)?,
                    app_compatibility_json: row.get(5)?,
                    manifest_json: row.get(6)?,
                    content_hash: row.get(7)?,
                    artifact_hash: row.get(8)?,
                    size_bytes: row.get::<_, i64>(9)? as u64,
                    state: row.get(10)?,
                    progress: row.get::<_, i64>(11)? as u8,
                    staged_path: row.get(12)?,
                    installed_path: row.get(13)?,
                    last_error: row.get(14)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>();
        rows.map_err(CoreError::from)
    }

    pub fn pack_runtime_row(&self, pack_id: &str) -> Result<PackRuntimeRow, CoreError> {
        self.connection
            .query_row(
                "SELECT pack_id, version, family, scope_json, schema_version, app_compatibility_json,
                        manifest_json, content_hash, artifact_hash, size_bytes, state, progress,
                        staged_path, installed_path, last_error
                 FROM pack_runtime WHERE pack_id = ?1",
                [pack_id],
                |row| {
                    Ok(PackRuntimeRow {
                        pack_id: row.get(0)?,
                        version: row.get(1)?,
                        family: row.get(2)?,
                        scope_json: row.get(3)?,
                        schema_version: row.get(4)?,
                        app_compatibility_json: row.get(5)?,
                        manifest_json: row.get(6)?,
                        content_hash: row.get(7)?,
                        artifact_hash: row.get(8)?,
                        size_bytes: row.get::<_, i64>(9)? as u64,
                        state: row.get(10)?,
                        progress: row.get::<_, i64>(11)? as u8,
                        staged_path: row.get(12)?,
                        installed_path: row.get(13)?,
                        last_error: row.get(14)?,
                    })
                },
            )
            .map_err(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => CoreError::PackNotFound(pack_id.to_string()),
                other => CoreError::Database(other),
            })
    }

    pub fn set_pack_runtime_state(
        &mut self,
        pack_id: &str,
        version: &str,
        state: &str,
        progress: u8,
        staged_path: Option<&str>,
        installed_path: Option<&str>,
        artifact_hash: Option<&str>,
        size_bytes: Option<u64>,
        last_error: Option<&str>,
    ) -> Result<(), CoreError> {
        let changed = self.connection.execute(
            "UPDATE pack_runtime SET version=?2, state=?3, progress=?4, staged_path=COALESCE(?5, staged_path),
                    installed_path=COALESCE(?6, installed_path), artifact_hash=COALESCE(?7, artifact_hash),
                    size_bytes=COALESCE(?8, size_bytes), last_error=?9,
                    updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE pack_id=?1",
            params![
                pack_id,
                version,
                state,
                i64::from(progress),
                staged_path,
                installed_path,
                artifact_hash,
                size_bytes.map(|value| value as i64),
                last_error
            ],
        )?;
        if changed == 0 {
            return Err(CoreError::PackNotFound(pack_id.to_string()));
        }
        Ok(())
    }

    pub fn import_milano_pack(&mut self, pack: &NexPack) -> Result<usize, CoreError> {
        pack.verify()?;
        let imported = self.seed_milano(&pack.payload)?;
        self.connection.execute(
            "UPDATE pack_installation SET manifest_json=?3, content_hash=?4
             WHERE pack_id=?1 AND pack_version=?2",
            params![
                pack.manifest.pack_id,
                pack.manifest.version,
                serde_json::to_string(&pack.manifest)?,
                pack.manifest.content_hash
            ],
        )?;
        Ok(imported)
    }

    pub fn pack_entity_count(&self, pack_id: &str, entity_kind: &str) -> Result<u64, CoreError> {
        self.connection
            .query_row(
                "SELECT COUNT(*) FROM pack_membership WHERE pack_id=?1 AND entity_kind=?2",
                params![pack_id, entity_kind],
                |row| row.get::<_, i64>(0),
            )
            .map(|count| count as u64)
            .map_err(CoreError::from)
    }

    pub fn remove_pack(&mut self, pack_id: &str) -> Result<(), CoreError> {
        let transaction = self.connection.transaction()?;
        transaction
            .execute(
                "DELETE FROM event_segment
                 WHERE event_id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='event')
                   AND segment_id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='segment')",
                [pack_id],
            )
            .map_err(|error| CoreError::Pack(format!("delete event_segment: {error}")))?;
        transaction.execute(
            "DELETE FROM catalog_fts
             WHERE entity_id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='geo_city')",
            [pack_id],
        )?;
        for (table, kind) in [
            ("source_contribution", "source_contribution"),
            ("media_occurrence", "media_occurrence"),
            ("catalog_review", "catalog_review"),
            ("person_role", "person_role"),
            ("catalog_collection", "catalog_collection"),
            ("schedule_entry", "schedule_entry"),
            ("participant_registry", "participant"),
            ("catalog_maison", "maison"),
            ("catalog_venue", "venue"),
            ("catalog_edition", "edition"),
            ("catalog_segment", "segment"),
            ("catalog_event", "event"),
            ("city_hub", "city_hub"),
            ("geo_city", "geo_city"),
            ("geo_country", "geo_country"),
            ("geo_region", "geo_region"),
            ("source_endpoint", "source_endpoint"),
        ] {
            delete_owned_rows(&transaction, pack_id, table, kind)?;
        }
        delete_owned_rows(&transaction, pack_id, "catalog_gap", "catalog_gap")?;
        delete_owned_rows(&transaction, pack_id, "source_registry", "source")?;
        transaction.execute(
            "UPDATE catalog_entity SET display_name='Removed pack content', search_text='', payload_json='{\"packRemoved\":true}',
                    is_official=0, updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='catalog_entity')
               AND EXISTS (SELECT 1 FROM personal_note WHERE personal_note.entity_id = catalog_entity.id)
               AND NOT EXISTS (SELECT 1 FROM pack_membership other WHERE other.entity_kind='catalog_entity'
                               AND other.entity_id=catalog_entity.id AND other.pack_id<>?1)",
            [pack_id],
        )?;
        transaction.execute(
            "DELETE FROM catalog_entity
             WHERE id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='catalog_entity')
               AND NOT EXISTS (SELECT 1 FROM personal_note WHERE personal_note.entity_id = catalog_entity.id)
               AND NOT EXISTS (SELECT 1 FROM pack_membership other WHERE other.entity_kind='catalog_entity'
                               AND other.entity_id=catalog_entity.id AND other.pack_id<>?1)",
            [pack_id],
        )?;
        transaction.execute("DELETE FROM pack_installation WHERE pack_id=?1", [pack_id])?;
        transaction.execute("DELETE FROM pack_membership WHERE pack_id=?1", [pack_id])?;
        transaction.commit()?;
        Ok(())
    }

    pub fn seed_bundled_milano(&mut self) -> Result<usize, CoreError> {
        let pack = bundled_pack()?;
        self.seed_milano(&pack.payload)
    }

    pub fn seed_milano(&mut self, seed: &MilanoSeed) -> Result<usize, CoreError> {
        let transaction = self.connection.transaction()?;
        let now = seed.retrieved_at.as_str();
        let pack_id = seed.pack_id.as_str();
        let pack_version = seed.version.as_str();
        let source = &seed.source;
        let capabilities = serde_json::to_string(&source.endpoint.capabilities)?;

        transaction.execute(
            "INSERT INTO source_registry (id, name, source_kind, authority_tier, base_url, access_mode, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, source_kind=excluded.source_kind,
               authority_tier=excluded.authority_tier, base_url=excluded.base_url, access_mode=excluded.access_mode,
               status=excluded.status, updated_at=excluded.updated_at",
            params![
                source.id,
                source.name,
                source.source_kind,
                source.authority_tier,
                source.base_url,
                source.access_mode,
                source.status,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source",
            &source.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO source_endpoint (id, source_id, endpoint_type, base_url, access_method, capabilities_json, adapter_id, status, last_verified_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'active', ?8)
             ON CONFLICT(id) DO UPDATE SET base_url=excluded.base_url, access_method=excluded.access_method,
               capabilities_json=excluded.capabilities_json, adapter_id=excluded.adapter_id, last_verified_at=excluded.last_verified_at",
            params![
                source.endpoint.id,
                source.id,
                source.endpoint.endpoint_type,
                source.endpoint.base_url,
                source.endpoint.access_method,
                capabilities,
                source.endpoint.adapter_id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_endpoint",
            &source.endpoint.id,
            now,
        )?;

        transaction.execute(
            "INSERT INTO geo_region (id, name, m49_code, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, m49_code=excluded.m49_code, source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![seed.region.id, seed.region.name, seed.region.m49_code, source.id, now],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "geo_region",
            &seed.region.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO geo_country (id, region_id, name, iso_alpha2, iso_alpha3, m49_code, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
             ON CONFLICT(id) DO UPDATE SET region_id=excluded.region_id, name=excluded.name,
               iso_alpha2=excluded.iso_alpha2, iso_alpha3=excluded.iso_alpha3, m49_code=excluded.m49_code,
               source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.country.id,
                seed.country.region_id,
                seed.country.name,
                seed.country.iso_alpha2,
                seed.country.iso_alpha3,
                seed.country.m49_code,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "geo_country",
            &seed.country.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO geo_city (id, country_id, name, aliases_json, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)
             ON CONFLICT(id) DO UPDATE SET country_id=excluded.country_id, name=excluded.name,
               aliases_json=excluded.aliases_json, source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.city.id,
                seed.city.country_id,
                seed.city.name,
                serde_json::to_string(&seed.city.aliases)?,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "geo_city",
            &seed.city.id,
            now,
        )?;
        transaction.execute(
            "DELETE FROM catalog_fts WHERE entity_id=?1",
            [&seed.city.id],
        )?;
        let city_search_rowid: i64 = transaction.query_row(
            "SELECT COALESCE(MIN(rowid), 0) - 1 FROM catalog_fts",
            [],
            |row| row.get(0),
        )?;
        transaction
            .execute(
                "INSERT INTO catalog_fts (rowid, entity_id, display_name, entity_kind, search_text)
                 VALUES (?1, ?2, ?3, 'city', ?4)",
                params![
                    city_search_rowid,
                    seed.city.id,
                    seed.city.name,
                    format!("{} {}", seed.city.name, seed.city.aliases.join(" "))
                ],
            )
            .map_err(|error| CoreError::Pack(format!("city fts insert: {error}")))?;
        transaction.execute(
            "INSERT INTO city_hub (id, city_id, official_name, display_name, slug, hub_type, status, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)
             ON CONFLICT(id) DO UPDATE SET city_id=excluded.city_id, official_name=excluded.official_name,
               display_name=excluded.display_name, slug=excluded.slug, hub_type=excluded.hub_type,
               status=excluded.status, source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.city_hub.id,
                seed.city_hub.city_id,
                seed.city_hub.official_name,
                seed.city_hub.display_name,
                seed.city_hub.slug,
                seed.city_hub.hub_type,
                seed.city_hub.status,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "city_hub",
            &seed.city_hub.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO catalog_event (id, city_hub_id, official_name, display_name, short_name, event_type, status, start_year, official_website, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)
             ON CONFLICT(id) DO UPDATE SET city_hub_id=excluded.city_hub_id, official_name=excluded.official_name,
               display_name=excluded.display_name, short_name=excluded.short_name, event_type=excluded.event_type,
               status=excluded.status, start_year=excluded.start_year, official_website=excluded.official_website,
               source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.event.id,
                seed.event.city_hub_id,
                seed.event.official_name,
                seed.event.display_name,
                seed.event.short_name,
                seed.event.event_type,
                seed.event.status,
                seed.event.start_year,
                seed.event.official_website,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "event",
            &seed.event.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO catalog_segment (id, name, code, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, code=excluded.code, description=excluded.description, updated_at=excluded.updated_at",
            params![seed.segment.id, seed.segment.name, seed.segment.code, seed.segment.description, now],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "segment",
            &seed.segment.id,
            now,
        )?;
        transaction.execute(
            "INSERT OR REPLACE INTO event_segment (event_id, segment_id, source_id) VALUES (?1, ?2, ?3)",
            params![seed.event.id, seed.segment.id, source.id],
        )?;
        transaction.execute(
            "INSERT INTO catalog_edition (id, event_id, segment_id, display_name, season, season_code, season_year, calendar_year, start_date, end_date, time_zone, edition_status, official_page_url, official_calendar_url, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?16)
             ON CONFLICT(id) DO UPDATE SET event_id=excluded.event_id, segment_id=excluded.segment_id,
               display_name=excluded.display_name, season=excluded.season, season_code=excluded.season_code,
               season_year=excluded.season_year, calendar_year=excluded.calendar_year, start_date=excluded.start_date,
               end_date=excluded.end_date, time_zone=excluded.time_zone, edition_status=excluded.edition_status,
               official_page_url=excluded.official_page_url, official_calendar_url=excluded.official_calendar_url,
               source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.edition.id,
                seed.edition.event_id,
                seed.edition.segment_id,
                seed.edition.display_name,
                seed.edition.season,
                seed.edition.season_code,
                seed.edition.season_year,
                seed.edition.calendar_year,
                seed.edition.start_date,
                seed.edition.end_date,
                seed.edition.time_zone,
                seed.edition.edition_status,
                seed.edition.official_page_url,
                seed.edition.official_calendar_url,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "edition",
            &seed.edition.id,
            now,
        )?;

        for venue in &seed.venues {
            transaction.execute(
                "INSERT INTO catalog_venue (id, city_id, name, address, venue_type, source_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
                 ON CONFLICT(id) DO UPDATE SET city_id=excluded.city_id, name=excluded.name,
                   address=excluded.address, venue_type=excluded.venue_type, source_id=excluded.source_id, updated_at=excluded.updated_at",
                params![
                    venue.id,
                    venue.city_id,
                    venue.name,
                    venue.address,
                    venue.venue_type,
                    source.id,
                    now
                ],
            )?;
            upsert_catalog_entity(
                &transaction,
                &venue.id,
                "venue",
                venue.address.as_deref().unwrap_or("Milano venue"),
                venue.address.as_deref().unwrap_or(""),
                "{}",
                now,
            )?;
            register_pack_entity(&transaction, pack_id, pack_version, "venue", &venue.id, now)?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "catalog_entity",
                &venue.id,
                now,
            )?;
        }

        for maison in &seed.maisons {
            transaction.execute(
                "INSERT INTO catalog_maison (id, official_name, display_name, slug, status, official_website, source_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
                 ON CONFLICT(id) DO UPDATE SET official_name=excluded.official_name, display_name=excluded.display_name,
                   slug=excluded.slug, status=excluded.status, official_website=excluded.official_website,
                   source_id=excluded.source_id, updated_at=excluded.updated_at",
                params![
                    maison.id,
                    maison.official_name,
                    maison.display_name,
                    maison.slug,
                    maison.status,
                    maison.official_website,
                    source.id,
                    now
                ],
            )?;
            upsert_catalog_entity(
                &transaction,
                &maison.id,
                "maison",
                &maison.display_name,
                &maison.official_name,
                &serde_json::to_string(maison)?,
                now,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "maison",
                &maison.id,
                now,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "catalog_entity",
                &maison.id,
                now,
            )?;
        }

        for participant in &seed.participants {
            let normalized_name = normalize_participant_name(&participant.display_name);
            transaction.execute(
                "INSERT INTO participant_registry (id, display_name, normalized_name, canonical_kind, reconciliation_status, source_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
                 ON CONFLICT(id) DO UPDATE SET display_name=excluded.display_name, normalized_name=excluded.normalized_name,
                   canonical_kind=excluded.canonical_kind, reconciliation_status=excluded.reconciliation_status,
                   source_id=excluded.source_id, updated_at=excluded.updated_at",
                params![
                    participant.id,
                    participant.display_name,
                    normalized_name,
                    participant.canonical_kind,
                    participant.reconciliation_status,
                    source.id,
                    now
                ],
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "participant",
                &participant.id,
                now,
            )?;
        }

        for entry in &seed.entries {
            transaction.execute(
                "INSERT INTO schedule_entry (id, edition_id, participant_id, maison_id, local_date, start_time_local, end_time_local, time_zone, format, schedule_status, location_status, delivery_mode, venue_id, venue_label, official_stream_url, official_entry_url, official_note, source_id, source_external_id, source_hash, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?21)
                 ON CONFLICT(edition_id, source_id, source_external_id) DO UPDATE SET participant_id=excluded.participant_id,
                   maison_id=excluded.maison_id, local_date=excluded.local_date, start_time_local=excluded.start_time_local,
                   end_time_local=excluded.end_time_local, time_zone=excluded.time_zone, format=excluded.format,
                   schedule_status=excluded.schedule_status, location_status=excluded.location_status, venue_id=excluded.venue_id,
                   delivery_mode=excluded.delivery_mode, venue_label=excluded.venue_label, official_stream_url=excluded.official_stream_url,
                   official_entry_url=excluded.official_entry_url, official_note=excluded.official_note,
                   source_hash=excluded.source_hash, updated_at=excluded.updated_at",
                params![
                    entry.id,
                    seed.edition.id,
                    entry.participant_id,
                    entry.maison_id,
                    entry.local_date,
                    entry.start_time_local,
                    entry.end_time_local,
                    entry.time_zone,
                    entry.format,
                    entry.schedule_status,
                    entry.location_status,
                    entry.delivery_mode,
                    entry.venue_id,
                    entry.venue_label,
                    entry.official_stream_url,
                    entry.official_entry_url,
                    entry.official_note,
                    source.id,
                    entry.source_external_id,
                    entry.source_hash,
                    now
                ],
            )?;
            let search_text = format!(
                "{} {} {}",
                entry.participant_name_raw,
                entry.format,
                entry.official_note.as_deref().unwrap_or("")
            );
            upsert_catalog_entity(
                &transaction,
                &entry.id,
                "schedule_entry",
                &entry.participant_name_raw,
                &search_text,
                &serde_json::to_string(entry)?,
                now,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "schedule_entry",
                &entry.id,
                now,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "catalog_entity",
                &entry.id,
                now,
            )?;
            insert_contribution(
                &transaction,
                &format!("contribution:{}", entry.id),
                &source.id,
                "schedule_entry",
                &entry.id,
                r#"["participant_name_raw","local_date","start_time_local","format","location_status","delivery_mode","venue_label","official_note"]"#,
                entry
                    .official_entry_url
                    .as_deref()
                    .unwrap_or(&seed.edition.official_calendar_url),
                now,
                &source.endpoint.adapter_id,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "source_contribution",
                &format!("contribution:{}", entry.id),
                now,
            )?;
        }

        upsert_catalog_entity(
            &transaction,
            &seed.city_hub.id,
            "city_hub",
            &seed.city_hub.display_name,
            &format!("{} {}", seed.city_hub.display_name, seed.city_hub.slug),
            &serde_json::to_string(&seed.city_hub)?,
            now,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "catalog_entity",
            &seed.city_hub.id,
            now,
        )?;
        upsert_catalog_entity(
            &transaction,
            &seed.event.id,
            "event",
            &seed.event.display_name,
            &format!("{} {}", seed.event.display_name, seed.event.short_name),
            &serde_json::to_string(&seed.event)?,
            now,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "catalog_entity",
            &seed.event.id,
            now,
        )?;
        upsert_catalog_entity(
            &transaction,
            &seed.edition.id,
            "edition",
            &seed.edition.display_name,
            &format!(
                "{} {} {}",
                seed.edition.display_name, seed.edition.season_code, seed.edition.calendar_year
            ),
            &serde_json::to_string(&seed.edition)?,
            now,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "catalog_entity",
            &seed.edition.id,
            now,
        )?;
        upsert_catalog_entity(
            &transaction,
            &source.id,
            "source",
            &source.name,
            &format!("{} {}", source.name, source.endpoint.base_url),
            &serde_json::to_string(source)?,
            now,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "catalog_entity",
            &source.id,
            now,
        )?;

        insert_contribution(
            &transaction,
            "contribution:cityhub:milano",
            &source.id,
            "city_hub",
            &seed.city_hub.id,
            r#"["official_name","display_name","city_id","status"]"#,
            &seed.edition.official_page_url,
            now,
            &source.endpoint.adapter_id,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_contribution",
            "contribution:cityhub:milano",
            now,
        )?;
        insert_contribution(
            &transaction,
            "contribution:city:milano",
            &source.id,
            "city",
            &seed.city.id,
            r#"["name","aliases"]"#,
            &seed.edition.official_page_url,
            now,
            &source.endpoint.adapter_id,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_contribution",
            "contribution:city:milano",
            now,
        )?;
        insert_contribution(
            &transaction,
            "contribution:event:milano-fashion-week",
            &source.id,
            "event",
            &seed.event.id,
            r#"["official_name","event_type","official_website"]"#,
            &seed.edition.official_page_url,
            now,
            &source.endpoint.adapter_id,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_contribution",
            "contribution:event:milano-fashion-week",
            now,
        )?;
        insert_contribution(
            &transaction,
            "contribution:edition:milano-fashion-week:ss27:2026",
            &source.id,
            "edition",
            &seed.edition.id,
            r#"["season_code","season_year","calendar_year","start_date","end_date","official_calendar_url"]"#,
            &seed.edition.official_calendar_url,
            now,
            &source.endpoint.adapter_id,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_contribution",
            "contribution:edition:milano-fashion-week:ss27:2026",
            now,
        )?;

        for maison in &seed.maisons {
            insert_contribution(
                &transaction,
                &format!("contribution:{}", maison.id),
                &source.id,
                "maison",
                &maison.id,
                r#"["official_name","official_website"]"#,
                maison
                    .official_website
                    .as_deref()
                    .unwrap_or(&seed.edition.official_page_url),
                now,
                &source.endpoint.adapter_id,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "source_contribution",
                &format!("contribution:{}", maison.id),
                now,
            )?;
        }

        for gap in &seed.gaps {
            transaction.execute(
                "INSERT INTO catalog_gap (id, entity_type, entity_id, field_name, reason, status, source_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, 'open', ?6, ?7, ?7)
                 ON CONFLICT(entity_type, entity_id, field_name) DO UPDATE SET reason=excluded.reason, status=excluded.status, source_id=excluded.source_id, updated_at=excluded.updated_at",
                params![
                    gap.id,
                    gap.entity_type,
                    gap.entity_id,
                    gap.field_name,
                    gap.reason,
                    source.id,
                    now
                ],
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "catalog_gap",
                &gap.id,
                now,
            )?;
        }

        let manifest = serde_json::json!({
            "packId": seed.pack_id,
            "version": seed.version,
            "kind": "base_catalog",
            "title": seed.edition.display_name,
            "contentHash": seed.content_hash,
            "sourceIds": [source.id],
            "entityCounts": {
                "city_hub": 1,
                "event": 1,
                "edition": 1,
                "schedule_entry": seed.entries.len(),
                "venue": seed.venues.len(),
                "maison": seed.maisons.len(),
                "source": 1
            }
        });
        transaction.execute(
            "INSERT INTO pack_installation (id, pack_id, pack_version, manifest_json, content_hash, installed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(pack_id, pack_version) DO UPDATE SET manifest_json=excluded.manifest_json,
               content_hash=excluded.content_hash, installed_at=excluded.installed_at",
            params![
                format!("pack:{}:{}", seed.pack_id, seed.version),
                seed.pack_id,
                seed.version,
                serde_json::to_string(&manifest)?,
                seed.content_hash,
                now
            ],
        )?;

        transaction.commit()?;
        Ok(seed.entries.len())
    }

    pub fn milano_snapshot(&self) -> Result<MilanoSnapshot, CoreError> {
        let city_hub = self.connection.query_row(
            "SELECT id, city_id, official_name, display_name, slug, status FROM city_hub WHERE id = 'cityhub:milano'",
            [],
            |row| Ok(CityHubRow { id: row.get(0)?, city_id: row.get(1)?, official_name: row.get(2)?, display_name: row.get(3)?, slug: row.get(4)?, status: row.get(5)? }),
        )?;
        let event = self.connection.query_row(
            "SELECT id, city_hub_id, official_name, display_name, short_name, event_type, status, official_website FROM catalog_event WHERE id = 'event:milano-fashion-week'",
            [],
            |row| Ok(EventRow { id: row.get(0)?, city_hub_id: row.get(1)?, official_name: row.get(2)?, display_name: row.get(3)?, short_name: row.get(4)?, event_type: row.get(5)?, status: row.get(6)?, official_website: row.get(7)? }),
        )?;
        let edition = self.connection.query_row(
            "SELECT id, event_id, segment_id, display_name, season, season_code, season_year, calendar_year, start_date, end_date, time_zone, edition_status, official_page_url, official_calendar_url FROM catalog_edition WHERE id = 'edition:milano-fashion-week:ss27:2026'",
            [],
            |row| Ok(EditionRow { id: row.get(0)?, event_id: row.get(1)?, segment_id: row.get(2)?, display_name: row.get(3)?, season: row.get(4)?, season_code: row.get(5)?, season_year: row.get(6)?, calendar_year: row.get(7)?, start_date: row.get(8)?, end_date: row.get(9)?, time_zone: row.get(10)?, edition_status: row.get(11)?, official_page_url: row.get(12)?, official_calendar_url: row.get(13)? }),
        )?;
        let mut statement = self.connection.prepare(
            "SELECT s.id, s.edition_id, s.participant_id, p.display_name, s.maison_id, s.local_date,
                    s.start_time_local, s.end_time_local, s.time_zone, s.format, s.schedule_status,
                    s.location_status, s.delivery_mode, s.venue_id, COALESCE(v.address, s.venue_label), s.official_stream_url,
                    s.official_entry_url, s.official_note, s.source_id, s.source_external_id
             FROM schedule_entry s
             JOIN participant_registry p ON p.id = s.participant_id
             LEFT JOIN catalog_venue v ON v.id = s.venue_id
             WHERE s.edition_id = ?1
             ORDER BY s.local_date, s.start_time_local, s.id",
        )?;
        let schedule = statement
            .query_map([&edition.id], |row| {
                Ok(ScheduleRow {
                    id: row.get(0)?,
                    edition_id: row.get(1)?,
                    participant_id: row.get(2)?,
                    participant_name_raw: row.get(3)?,
                    maison_id: row.get(4)?,
                    local_date: row.get(5)?,
                    start_time_local: row.get(6)?,
                    end_time_local: row.get(7)?,
                    time_zone: row.get(8)?,
                    format: row.get(9)?,
                    schedule_status: row.get(10)?,
                    location_status: row.get(11)?,
                    delivery_mode: row.get(12)?,
                    venue_id: row.get(13)?,
                    venue_label: row.get(14)?,
                    official_stream_url: row.get(15)?,
                    official_entry_url: row.get(16)?,
                    official_note: row.get(17)?,
                    source_id: row.get(18)?,
                    source_external_id: row.get(19)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        let (participant_count, unresolved_participant_count) = self.connection.query_row(
            "SELECT COUNT(*), SUM(CASE WHEN reconciliation_status <> 'reconciled' THEN 1 ELSE 0 END) FROM participant_registry",
            [],
            |row| Ok((row.get(0)?, row.get::<_, Option<i64>>(1)?.unwrap_or(0))),
        )?;
        let source = self.connection.query_row(
            "SELECT id, name, source_kind, authority_tier, base_url, access_mode, status FROM source_registry WHERE id = 'source:cnmi'",
            [],
            |row| Ok(SourceRow { id: row.get(0)?, name: row.get(1)?, source_kind: row.get(2)?, authority_tier: row.get(3)?, base_url: row.get(4)?, access_mode: row.get(5)?, status: row.get(6)? }),
        )?;
        let mut gaps_statement = self.connection.prepare(
            "SELECT entity_type, entity_id, field_name, reason, status FROM catalog_gap ORDER BY entity_type, entity_id, field_name",
        )?;
        let gaps = gaps_statement
            .query_map([], |row| {
                Ok(GapRow {
                    entity_type: row.get(0)?,
                    entity_id: row.get(1)?,
                    field_name: row.get(2)?,
                    reason: row.get(3)?,
                    status: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MilanoSnapshot {
            city_hub,
            event,
            edition,
            schedule,
            participant_count,
            unresolved_participant_count,
            source,
            gaps,
        })
    }

    pub fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchRow>, CoreError> {
        Ok(self
            .search_page(query, PageRequest { offset: 0, limit })?
            .items)
    }

    pub fn search_page(
        &self,
        query: &str,
        request: PageRequest,
    ) -> Result<ReadPage<SearchRow>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        let terms = query
            .split_whitespace()
            .filter(|term| !term.is_empty())
            .map(|term| format!("\"{}\"", term.replace('"', "")))
            .collect::<Vec<_>>();
        if terms.is_empty() {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let match_query = terms.join(" ");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_fts WHERE catalog_fts MATCH ?1",
            [&match_query],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT f.entity_id, f.entity_kind, f.display_name,
                    snippet(f.catalog_fts, 3, '[', ']', '…', 12),
                    c.payload_json,
                    COALESCE((SELECT group_concat(DISTINCT source_id)
                              FROM source_contribution sc
                              WHERE sc.entity_id=f.entity_id
                                AND (sc.entity_type=f.entity_kind
                                     OR (f.entity_kind='city' AND sc.entity_type='city'))), '')
             FROM catalog_fts f
             LEFT JOIN catalog_entity c ON c.id=f.entity_id
             WHERE f.catalog_fts MATCH ?1
             ORDER BY rank, f.display_name, f.entity_id
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![
                    match_query,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let entity_id: String = row.get(0)?;
                    let entity_kind: String = row.get(1)?;
                    let payload_json: Option<String> = row.get(4)?;
                    let payload = payload_json
                        .as_deref()
                        .and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok());
                    let slug = payload
                        .as_ref()
                        .and_then(|value| value.get("slug"))
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                    let subtitle = payload.as_ref().and_then(|value| {
                        value
                            .get("seasonCode")
                            .or_else(|| value.get("localDate"))
                            .or_else(|| value.get("officialName"))
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_string)
                    });
                    let thumbnail_url = payload.as_ref().and_then(|value| {
                        value
                            .get("thumbnailUrl")
                            .or_else(|| value.get("thumbUrl"))
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_string)
                    });
                    let source_ids = row
                        .get::<_, String>(5)?
                        .split(',')
                        .filter(|value| !value.is_empty())
                        .map(str::to_string)
                        .collect::<Vec<_>>();
                    Ok(SearchRow {
                        entity_id: entity_id.clone(),
                        entity_kind: entity_kind.clone(),
                        title: row.get(2)?,
                        slug,
                        subtitle,
                        snippet: row.get(3)?,
                        navigation: crate::core::read::NavigationTarget {
                            entity_kind: entity_kind.clone(),
                            entity_id,
                        },
                        thumbnail_url,
                        source_ids,
                        state: if entity_kind == "personal_note" {
                            ReadState::Available
                        } else {
                            state.clone()
                        },
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(rows, total, request, state))
    }

    pub fn upsert_personal_note(
        &mut self,
        id: &str,
        entity_id: &str,
        body: &str,
    ) -> Result<PersonalNoteRow, CoreError> {
        let transaction = self.connection.transaction()?;
        let now: String =
            transaction.query_row("SELECT strftime('%Y-%m-%dT%H:%M:%fZ', 'now')", [], |row| {
                row.get(0)
            })?;
        transaction.execute(
            "INSERT INTO personal_note (id, entity_id, body, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?4)
             ON CONFLICT(id) DO UPDATE SET entity_id=excluded.entity_id, body=excluded.body, updated_at=excluded.updated_at",
            params![id, entity_id, body, now],
        )?;
        upsert_catalog_entity(&transaction, id, "personal_note", body, body, "{}", &now)?;
        transaction.commit()?;
        self.connection.query_row(
            "SELECT id, entity_id, body, created_at, updated_at FROM personal_note WHERE id = ?1",
            [id],
            |row| Ok(PersonalNoteRow { id: row.get(0)?, entity_id: row.get(1)?, body: row.get(2)?, created_at: row.get(3)?, updated_at: row.get(4)? }),
        ).map_err(CoreError::from)
    }

    pub fn personal_note(&self, id: &str) -> Result<PersonalNoteRow, CoreError> {
        self.connection
            .query_row(
                "SELECT id, entity_id, body, created_at, updated_at FROM personal_note WHERE id = ?1",
                [id],
                |row| {
                    Ok(PersonalNoteRow {
                        id: row.get(0)?,
                        entity_id: row.get(1)?,
                        body: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                },
            )
            .map_err(CoreError::from)
    }
}

fn register_pack_entity(
    transaction: &Transaction<'_>,
    pack_id: &str,
    pack_version: &str,
    entity_kind: &str,
    entity_id: &str,
    created_at: &str,
) -> Result<(), CoreError> {
    transaction.execute(
        "INSERT OR IGNORE INTO pack_membership (pack_id, pack_version, entity_kind, entity_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![pack_id, pack_version, entity_kind, entity_id, created_at],
    )?;
    Ok(())
}

fn insert_ai_execution(
    transaction: &Transaction<'_>,
    record: &AiExecutionRecord,
) -> Result<(), CoreError> {
    transaction.execute(
        "INSERT INTO ai_execution
         (execution_id, task_type, provider_id, model, capability, started_at, completed_at,
          latency_ms, attempt, fallback_step, status, validator_schema, validation_result_json,
          error_code, error_message, input_hash, usage_json, cost_amount, candidate_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        params![
            record.execution_id,
            record.task_type,
            record.provider_id,
            record.model,
            record.capability,
            record.started_at,
            record.completed_at,
            record.latency_ms,
            i64::from(record.attempt),
            i64::from(record.fallback_step),
            record.status,
            record.validator_schema,
            serde_json::to_string(&record.validation_result)?,
            record.error_code,
            record.error_message,
            record.input_hash,
            record.usage.as_ref().map(serde_json::to_string).transpose()?,
            record.cost,
            record.candidate_id,
        ],
    )?;
    Ok(())
}

fn insert_ai_candidate(
    transaction: &Transaction<'_>,
    record: &AiCandidateRecord,
) -> Result<(), CoreError> {
    transaction.execute(
        "INSERT INTO ai_candidate
         (candidate_id, proposal_kind, target_entity_type, target_entity_id, proposed_json,
          evidence_json, provenance_json, rationale, confidence, execution_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            record.candidate_id,
            record.proposal_kind,
            record
                .target
                .as_ref()
                .map(|target| target.entity_type.as_str()),
            record
                .target
                .as_ref()
                .map(|target| target.entity_id.as_str()),
            serde_json::to_string(&record.proposed)?,
            serde_json::to_string(&record.evidence)?,
            serde_json::to_string(&record.provenance)?,
            record.rationale,
            record.confidence,
            record.execution_id,
        ],
    )?;
    Ok(())
}

fn insert_curator_proposal(
    transaction: &Transaction<'_>,
    record: &CuratorProposalRecord,
) -> Result<(), CoreError> {
    transaction.execute(
        "INSERT INTO curator_proposal
         (proposal_id, candidate_id, proposal_kind, proposed_json, evidence_json,
          provenance_json, rationale, confidence)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            record.proposal_id,
            record.candidate_id,
            record.proposal_kind,
            serde_json::to_string(&record.proposed)?,
            serde_json::to_string(&record.evidence)?,
            serde_json::to_string(&record.provenance)?,
            record.rationale,
            record.confidence,
        ],
    )?;
    Ok(())
}

fn execution_read(record: &AiExecutionRecord) -> AiExecutionRead {
    AiExecutionRead {
        execution_id: record.execution_id.clone(),
        task_type: record.task_type.clone(),
        provider_id: record.provider_id.clone(),
        model: record.model.clone(),
        capability: record.capability.clone(),
        started_at: record.started_at.clone(),
        completed_at: record.completed_at.clone(),
        latency_ms: record.latency_ms,
        attempt: record.attempt,
        fallback_step: record.fallback_step,
        status: record.status.clone(),
        validator_schema: record.validator_schema.clone(),
        validation_result: record.validation_result.clone(),
        error_code: record.error_code.clone(),
        error_message: record.error_message.clone(),
        input_hash: record.input_hash.clone(),
        usage: record.usage.clone(),
        cost: record.cost,
        candidate_id: record.candidate_id.clone(),
    }
}

fn candidate_read(record: &AiCandidateRecord) -> AiCandidateRead {
    AiCandidateRead {
        candidate_id: record.candidate_id.clone(),
        proposal_kind: record.proposal_kind.clone(),
        target: record.target.clone(),
        proposed: record.proposed.clone(),
        evidence: record.evidence.clone(),
        provenance: record.provenance.clone(),
        rationale: record.rationale.clone(),
        confidence: record.confidence,
        execution_id: record.execution_id.clone(),
        status: "open".to_string(),
    }
}

fn proposal_read(record: &CuratorProposalRecord) -> CuratorProposalRead {
    CuratorProposalRead {
        proposal_id: record.proposal_id.clone(),
        candidate_id: record.candidate_id.clone(),
        proposal_kind: record.proposal_kind.clone(),
        state: "pending".to_string(),
        target: record.target.clone(),
        proposed: record.proposed.clone(),
        edited: None,
        before: None,
        approved: None,
        evidence: record.evidence.clone(),
        provenance: record.provenance.clone(),
        rationale: record.rationale.clone(),
        confidence: record.confidence,
        reviewer: None,
        decision_reason: None,
        decided_at: None,
        official_applied: false,
    }
}

fn json_value_from_row(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<Value> {
    let text: String = row.get(index)?;
    serde_json::from_str(&text).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            index,
            rusqlite::types::Type::Text,
            Box::new(error),
        )
    })
}

fn optional_json_value_from_row(
    row: &rusqlite::Row<'_>,
    index: usize,
) -> rusqlite::Result<Option<Value>> {
    let text: Option<String> = row.get(index)?;
    text.map(|value| {
        serde_json::from_str(&value).map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                index,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })
    })
    .transpose()
}

fn target_from_row(
    row: &rusqlite::Row<'_>,
    type_index: usize,
    id_index: usize,
) -> rusqlite::Result<Option<AiTargetRef>> {
    let entity_type: Option<String> = row.get(type_index)?;
    let entity_id: Option<String> = row.get(id_index)?;
    Ok(entity_type
        .zip(entity_id)
        .map(|(entity_type, entity_id)| AiTargetRef {
            entity_type,
            entity_id,
        }))
}

fn read_ai_execution_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AiExecutionRead> {
    Ok(AiExecutionRead {
        execution_id: row.get(0)?,
        task_type: row.get(1)?,
        provider_id: row.get(2)?,
        model: row.get(3)?,
        capability: row.get(4)?,
        started_at: row.get(5)?,
        completed_at: row.get(6)?,
        latency_ms: row.get(7)?,
        attempt: row.get::<_, i64>(8)? as u8,
        fallback_step: row.get::<_, i64>(9)? as u8,
        status: row.get(10)?,
        validator_schema: row.get(11)?,
        validation_result: json_value_from_row(row, 12)?,
        error_code: row.get(13)?,
        error_message: row.get(14)?,
        input_hash: row.get(15)?,
        usage: optional_json_value_from_row(row, 16)?,
        cost: row.get(17)?,
        candidate_id: row.get(18)?,
    })
}

fn read_curator_proposal_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<CuratorProposalRead> {
    Ok(CuratorProposalRead {
        proposal_id: row.get(0)?,
        candidate_id: row.get(1)?,
        proposal_kind: row.get(2)?,
        state: row.get(3)?,
        target: target_from_row(row, 4, 5)?,
        proposed: json_value_from_row(row, 6)?,
        edited: optional_json_value_from_row(row, 7)?,
        before: optional_json_value_from_row(row, 8)?,
        approved: optional_json_value_from_row(row, 9)?,
        evidence: json_value_from_row(row, 10)?
            .as_array()
            .cloned()
            .unwrap_or_default(),
        provenance: json_value_from_row(row, 11)?
            .as_array()
            .cloned()
            .unwrap_or_default(),
        rationale: row.get(12)?,
        confidence: row.get(13)?,
        reviewer: row.get(14)?,
        decision_reason: row.get(15)?,
        decided_at: row.get(16)?,
        official_applied: row.get::<_, i64>(17)? != 0,
    })
}

fn apply_official_promotion(
    transaction: &Transaction<'_>,
    proposal_kind: &str,
    target: &Option<AiTargetRef>,
    proposed: &Value,
) -> Result<(Option<Value>, bool), CoreError> {
    match proposal_kind {
        "review_summary" | "review_translation" => {
            let target = target.as_ref().ok_or_else(|| {
                CoreError::InvalidRequest("review proposal requires a target".to_string())
            })?;
            if target.entity_type != "review" {
                return Ok((None, false));
            }
            let summary = proposed
                .get("summary")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    CoreError::AiValidation("approved review lacks summary".to_string())
                })?;
            let key_points = proposed
                .get("keyPoints")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    CoreError::AiValidation("approved review lacks keyPoints".to_string())
                })?;
            let before = transaction
                .query_row(
                    "SELECT json_object('summary', summary, 'keyPoints', json(key_points_json))
                     FROM catalog_review WHERE id=?1",
                    [&target.entity_id],
                    |row| row.get::<_, String>(0),
                )
                .optional()?
                .map(|value| serde_json::from_str(&value))
                .transpose()?;
            let changed = transaction.execute(
                "UPDATE catalog_review
                 SET summary=?1, key_points_json=?2, updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                 WHERE id=?3",
                params![summary, serde_json::to_string(key_points)?, target.entity_id],
            )?;
            if changed == 0 {
                return Err(CoreError::NotFound {
                    resource: "review".to_string(),
                    id: target.entity_id.clone(),
                });
            }
            Ok((before, true))
        }
        "source_discovery" | "source_candidate" => {
            let source = proposed
                .get("source")
                .and_then(Value::as_object)
                .ok_or_else(|| {
                    CoreError::AiValidation("approved source proposal lacks source".to_string())
                })?;
            let id = source.get("id").and_then(Value::as_str).ok_or_else(|| {
                CoreError::AiValidation("approved source proposal lacks source.id".to_string())
            })?;
            let name = source.get("name").and_then(Value::as_str).ok_or_else(|| {
                CoreError::AiValidation("approved source proposal lacks source.name".to_string())
            })?;
            let source_kind = source
                .get("sourceKind")
                .and_then(Value::as_str)
                .unwrap_or("discovered");
            let authority_tier = source
                .get("authorityTier")
                .and_then(Value::as_str)
                .unwrap_or("E");
            let base_url = source
                .get("baseUrl")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    CoreError::AiValidation(
                        "approved source proposal lacks source.baseUrl".to_string(),
                    )
                })?;
            let access_mode = source
                .get("accessMode")
                .and_then(Value::as_str)
                .unwrap_or("remote_render");
            transaction.execute(
                "INSERT INTO source_registry
                 (id, name, source_kind, authority_tier, base_url, access_mode, status, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'active',
                         strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                         strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
                 ON CONFLICT(id) DO UPDATE SET name=excluded.name,
                   source_kind=excluded.source_kind, authority_tier=excluded.authority_tier,
                   base_url=excluded.base_url, access_mode=excluded.access_mode,
                   status='active', updated_at=excluded.updated_at",
                params![id, name, source_kind, authority_tier, base_url, access_mode],
            )?;
            Ok((None, true))
        }
        _ => Ok((None, false)),
    }
}

fn delete_owned_rows(
    transaction: &Transaction<'_>,
    pack_id: &str,
    table: &str,
    entity_kind: &str,
) -> Result<(), CoreError> {
    let sql = format!(
        "DELETE FROM {table}
         WHERE id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind=?2)
           AND NOT EXISTS (SELECT 1 FROM pack_membership other
                           WHERE other.entity_kind=?2 AND other.entity_id={table}.id AND other.pack_id<>?1)"
    );
    transaction
        .execute(&sql, params![pack_id, entity_kind])
        .map_err(|error| CoreError::Pack(format!("delete {table}: {error}")))?;
    Ok(())
}

fn normalize_participant_name(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| character.to_lowercase())
        .filter(|character| character.is_alphanumeric() || character.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn insert_contribution(
    transaction: &Transaction<'_>,
    id: &str,
    source_id: &str,
    entity_type: &str,
    entity_id: &str,
    contributed_fields_json: &str,
    evidence_url: &str,
    retrieved_at: &str,
    adapter_id: &str,
) -> Result<(), CoreError> {
    transaction
        .execute(
            "INSERT INTO source_contribution (id, source_id, entity_type, entity_id, contributed_fields_json, evidence_url, retrieved_at, evidence_status, adapter_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'verified', ?8)
             ON CONFLICT(source_id, entity_type, entity_id, evidence_url) DO UPDATE SET contributed_fields_json=excluded.contributed_fields_json,
               retrieved_at=excluded.retrieved_at, evidence_status=excluded.evidence_status, adapter_id=excluded.adapter_id",
            params![id, source_id, entity_type, entity_id, contributed_fields_json, evidence_url, retrieved_at, adapter_id],
        )
        .map_err(|error| CoreError::Pack(format!("contribution {id}: {error}")))?;
    Ok(())
}

fn upsert_catalog_entity(
    transaction: &Transaction<'_>,
    id: &str,
    entity_kind: &str,
    display_name: &str,
    search_text: &str,
    payload_json: &str,
    now: &str,
) -> Result<(), CoreError> {
    transaction.execute(
        "INSERT INTO catalog_entity (id, entity_kind, display_name, search_text, payload_json, is_official, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?6)
         ON CONFLICT(id) DO UPDATE SET entity_kind=excluded.entity_kind, display_name=excluded.display_name,
           search_text=excluded.search_text, payload_json=excluded.payload_json, updated_at=excluded.updated_at",
        params![id, entity_kind, display_name, search_text, payload_json, now],
    )?;
    Ok(())
}

fn sha256_bytes(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

fn normalize_source_url(value: &str) -> String {
    value.trim().trim_end_matches('/').to_string()
}

fn new_db_id(prefix: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{prefix}:{nanos:x}")
}

fn read_adapter_run(row: &rusqlite::Row<'_>) -> rusqlite::Result<AdapterRunRead> {
    let checkpoint_json: String = row.get(6)?;
    Ok(AdapterRunRead {
        run_id: row.get(0)?,
        adapter_id: row.get(1)?,
        source_id: row.get(2)?,
        requested_capability: row.get(3)?,
        status: row.get(4)?,
        cursor: row.get(5)?,
        checkpoint: serde_json::from_str(&checkpoint_json).map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                6,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })?,
        fixture_mode: row.get::<_, i64>(7)? != 0,
        attempts: row.get::<_, i64>(8)? as u32,
        artifact_count: row.get::<_, i64>(9)? as u32,
        observation_count: row.get::<_, i64>(10)? as u32,
        candidate_count: row.get::<_, i64>(11)? as u32,
        changed_count: row.get::<_, i64>(12)? as u32,
        error_code: row.get(13)?,
        error_message: row.get(14)?,
        started_at: row.get(15)?,
        completed_at: row.get(16)?,
    })
}

fn read_raw_artifact(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawArtifactRead> {
    Ok(RawArtifactRead {
        artifact_id: row.get(0)?,
        source_id: row.get(1)?,
        adapter_id: row.get(2)?,
        integration_id: row.get(3)?,
        canonical_url: row.get(4)?,
        content_type: row.get(5)?,
        content_hash: row.get(6)?,
        byte_size: row.get::<_, i64>(7)?.max(0) as u64,
        storage_kind: row.get(8)?,
        content_ref: row.get(9)?,
        etag: row.get(10)?,
        last_modified: row.get(11)?,
        retrieved_at: row.get(12)?,
        retrieval_status: row.get(13)?,
    })
}

fn read_ingestion_candidate(row: &rusqlite::Row<'_>) -> rusqlite::Result<IngestionCandidateRead> {
    Ok(IngestionCandidateRead {
        candidate_id: row.get(0)?,
        source_id: row.get(1)?,
        adapter_id: row.get(2)?,
        run_id: row.get(3)?,
        candidate_kind: row.get(4)?,
        target_entity_type: row.get(5)?,
        target_entity_id: row.get(6)?,
        stable_key: row.get(7)?,
        proposed: json_value_from_row(row, 8)?,
        evidence: json_array_from_row(row, 9)?,
        provenance: json_array_from_row(row, 10)?,
        comparison_state: row.get(11)?,
        content_hash: row.get(12)?,
        status: row.get(13)?,
        seen_count: row.get::<_, i64>(14)?.max(0) as u32,
        first_seen_at: row.get(15)?,
        last_seen_at: row.get(16)?,
    })
}

fn read_source_candidate(row: &rusqlite::Row<'_>) -> rusqlite::Result<SourceCandidateRead> {
    Ok(SourceCandidateRead {
        source_candidate_id: row.get(0)?,
        source_key: row.get(1)?,
        display_name: row.get(2)?,
        base_url: row.get(3)?,
        source_kind: row.get(4)?,
        domains: json_vec_from_row(row, 5)?,
        locale: json_map_from_row(row, 6)?,
        capabilities: json_vec_from_row(row, 7)?,
        evidence: json_array_from_row(row, 8)?,
        provenance: json_array_from_row(row, 9)?,
        discovered_by: row.get(10)?,
        status: row.get(11)?,
    })
}

fn read_adapter_candidate(row: &rusqlite::Row<'_>) -> rusqlite::Result<AdapterCandidateRead> {
    Ok(AdapterCandidateRead {
        adapter_candidate_id: row.get(0)?,
        source_candidate_id: row.get(1)?,
        adapter_id: row.get(2)?,
        proposed: json_value_from_row(row, 3)?,
        capabilities: json_vec_from_row(row, 4)?,
        evidence: json_array_from_row(row, 5)?,
        provenance: json_array_from_row(row, 6)?,
        status: row.get(7)?,
    })
}

fn read_integration_proposal(row: &rusqlite::Row<'_>) -> rusqlite::Result<IntegrationProposalRead> {
    Ok(IntegrationProposalRead {
        integration_proposal_id: row.get(0)?,
        source_candidate_id: row.get(1)?,
        adapter_candidate_id: row.get(2)?,
        source_id: row.get(3)?,
        adapter_id: row.get(4)?,
        proposal: json_value_from_row(row, 5)?,
        evidence: json_array_from_row(row, 6)?,
        provenance: json_array_from_row(row, 7)?,
        state: row.get(8)?,
        reviewer: row.get(9)?,
        decision_reason: row.get(10)?,
        decided_at: row.get(11)?,
        official_applied: row.get::<_, i64>(12)? != 0,
    })
}

fn json_vec_from_row(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<Vec<String>> {
    let value = json_value_from_row(row, index)?;
    Ok(value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default())
}

fn json_map_from_row(
    row: &rusqlite::Row<'_>,
    index: usize,
) -> rusqlite::Result<BTreeMap<String, String>> {
    let text: String = row.get(index)?;
    serde_json::from_str(&text).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            index,
            rusqlite::types::Type::Text,
            Box::new(error),
        )
    })
}

fn json_array_from_row(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<Vec<Value>> {
    let value = json_value_from_row(row, index)?;
    Ok(value.as_array().cloned().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::CatalogDb;
    use crate::core::error::CoreError;
    use crate::core::pack::{PackRuntime, BUNDLED_PACK_ID};
    use crate::core::read::{PageRequest, ReadState};
    use serde_json::Value;

    fn installed_runtime() -> (CatalogDb, PackRuntime, std::path::PathBuf) {
        let root = std::env::temp_dir().join(format!(
            "nex-fashion-read-api-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be valid")
                .as_nanos()
        ));
        let runtime = PackRuntime::new(&root).expect("runtime should initialize");
        let mut database = CatalogDb::in_memory().expect("database should initialize");
        runtime
            .ensure_available(&mut database)
            .expect("pack should be available");
        runtime
            .install(&mut database, BUNDLED_PACK_ID)
            .expect("pack should install");
        (database, runtime, root)
    }

    #[test]
    fn baseline_migrations_create_sqlite_and_fts5() {
        let database = CatalogDb::in_memory().expect("baseline database should initialize");
        assert_eq!(
            database.migration_names().unwrap(),
            vec![
                "0001_core",
                "0002_fts5",
                "0003_milano_vertical",
                "0004_pack_runtime",
                "0005_personal_favorite",
                "0006_ai_curator",
                "0007_adapter_framework"
            ]
        );
        assert!(database.has_fts5().unwrap());
    }

    #[test]
    fn milano_seed_is_idempotent_and_preserves_explicit_gaps() {
        let mut database = CatalogDb::in_memory().expect("database should initialize");
        let first = database
            .seed_bundled_milano()
            .expect("first seed should work");
        let second = database
            .seed_bundled_milano()
            .expect("second seed should work");
        assert_eq!(first, 214);
        assert_eq!(second, first);
        let snapshot = database.milano_snapshot().expect("snapshot should exist");
        assert_eq!(snapshot.schedule.len(), 214);
        assert_eq!(snapshot.edition.season_year, 2027);
        assert_eq!(snapshot.edition.calendar_year, 2026);
        assert_eq!(snapshot.schedule[0].local_date, "2026-09-22");
        let dates = snapshot
            .schedule
            .iter()
            .map(|entry| entry.local_date.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            dates,
            [
                "2026-09-22",
                "2026-09-23",
                "2026-09-24",
                "2026-09-25",
                "2026-09-26",
                "2026-09-27",
                "2026-09-28"
            ]
            .into_iter()
            .collect()
        );
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.format == "presentation_by_appointment"));
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.format == "event"));
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.official_note.as_deref() == Some("LIVE")));
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.delivery_mode == "digital"
                && entry.official_note.as_deref() == Some("DIGITAL")));
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.participant_name_raw == "PRADA"
                && entry.maison_id.as_deref() == Some("maison:prada")));
        assert!(snapshot.unresolved_participant_count > 0);
        assert!(snapshot
            .gaps
            .iter()
            .any(|gap| gap.field_name == "subregion_id"));
        let search_ids = database
            .search("Prada", 10)
            .unwrap()
            .into_iter()
            .map(|result| result.entity_id)
            .collect::<Vec<_>>();
        assert!(search_ids.contains(&"maison:prada".to_string()));
        assert!(search_ids.contains(&"schedule:cnmi:13639".to_string()));
    }

    #[test]
    fn file_database_survives_reopen() {
        let path = std::env::temp_dir().join(format!(
            "nex-fashion-milano-{}-{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        {
            let mut database = CatalogDb::open(&path).expect("file database should open");
            database.seed_bundled_milano().expect("seed should work");
        }
        let database = CatalogDb::open(&path).expect("file database should reopen");
        assert_eq!(database.milano_snapshot().unwrap().schedule.len(), 214);
        drop(database);
        std::fs::remove_file(path).expect("test database should be cleaned up");
    }

    #[test]
    fn personal_note_targets_stable_catalog_ids() {
        let mut database = CatalogDb::in_memory().expect("database should initialize");
        database.seed_bundled_milano().expect("seed should work");
        let note = database
            .upsert_personal_note("note:prada", "maison:prada", "revisit this source")
            .expect("note should write");
        assert_eq!(note.entity_id, "maison:prada");
        database.seed_bundled_milano().expect("reseed should work");
        assert_eq!(
            database.search("revisit", 10).unwrap()[0].entity_id,
            "note:prada"
        );
    }

    #[test]
    fn public_read_api_navigates_milano_and_marks_missing_data_explicitly() {
        let (mut database, runtime, root) = installed_runtime();
        let city_hubs = database
            .read_city_hubs(PageRequest {
                offset: 0,
                limit: 10,
            })
            .expect("city hubs should be readable");
        assert_eq!(city_hubs.state, ReadState::Available);
        assert_eq!(city_hubs.items[0].id, "cityhub:milano");
        assert_eq!(city_hubs.items[0].city.id, "geo:city:milano");

        let events = database
            .read_events(Some("cityhub:milano"), PageRequest::default())
            .expect("events should be readable");
        assert_eq!(events.items[0].id, "event:milano-fashion-week");
        let editions = database
            .read_editions(Some("event:milano-fashion-week"), PageRequest::default())
            .expect("editions should be readable");
        assert_eq!(
            editions.items[0].id,
            "edition:milano-fashion-week:ss27:2026"
        );

        let schedule = database
            .read_schedule(
                "edition:milano-fashion-week:ss27:2026",
                PageRequest {
                    offset: 0,
                    limit: 25,
                },
            )
            .expect("schedule should be readable");
        assert_eq!(schedule.items.len(), 25);
        assert_eq!(schedule.total, 214);
        assert!(schedule.has_more);
        assert_eq!(schedule.next_offset, Some(25));

        let city_search = database
            .search_page(
                "Milano",
                PageRequest {
                    offset: 0,
                    limit: 20,
                },
            )
            .expect("search should be readable");
        assert!(city_search
            .items
            .iter()
            .any(|result| result.entity_kind == "city"));
        let maison_search = database
            .search_page("Prada", PageRequest::default())
            .expect("maison search should be readable");
        assert!(maison_search
            .items
            .iter()
            .any(|result| result.entity_id == "maison:prada"
                && result.navigation.entity_id == "maison:prada"
                && result.slug.as_deref() == Some("prada")));

        let maison = database
            .read_maison("maison:prada")
            .expect("maison should be readable");
        assert_eq!(maison.slug, "prada");
        assert!(database
            .read_collections(
                Some("edition:milano-fashion-week:ss27:2026"),
                Some(&maison.id),
                PageRequest::default()
            )
            .unwrap()
            .state
            .eq(&ReadState::DataPending));
        assert_eq!(
            database
                .read_looks("collection:missing", PageRequest::default())
                .unwrap()
                .state,
            ReadState::DataPending
        );
        assert_eq!(
            database
                .read_media("maison", &maison.id, PageRequest::default())
                .unwrap()
                .state,
            ReadState::DataPending
        );
        assert_eq!(
            database
                .read_reviews("maison", &maison.id, PageRequest::default())
                .unwrap()
                .state,
            ReadState::DataPending
        );
        assert_eq!(
            database.read_persons(PageRequest::default()).unwrap().state,
            ReadState::DataPending
        );
        assert_eq!(
            database.read_terms("source:cnmi").unwrap().state,
            ReadState::DataPending
        );

        let source_page = database
            .read_sources(None, None, PageRequest::default())
            .expect("sources should be readable");
        assert_eq!(source_page.items[0].id, "source:cnmi");
        database
            .upsert_personal_note("note:read-api", &maison.id, "keep this")
            .expect("note should be created");
        let personal = database
            .set_personal_favorite(&maison.id, true)
            .expect("favorite should be created");
        assert!(personal.is_favorite);
        assert_eq!(personal.notes[0].entity_id, maison.id);

        let serialized = serde_json::to_value(&schedule).expect("page should serialize");
        assert_eq!(serialized["hasMore"], Value::Bool(true));
        assert_eq!(
            serialized["items"][0]["editionId"],
            Value::String("edition:milano-fashion-week:ss27:2026".to_string())
        );
        let serialized_error =
            serde_json::to_value(CoreError::PackRemoved(BUNDLED_PACK_ID.to_string()))
                .expect("ipc error should serialize");
        assert_eq!(
            serialized_error["code"],
            Value::String("PACK_REMOVED".to_string())
        );

        assert!(matches!(
            database.read_maison("maison:does-not-exist"),
            Err(CoreError::NotFound { .. })
        ));

        runtime
            .remove(&mut database, BUNDLED_PACK_ID)
            .expect("pack should be removable");
        assert_eq!(
            database
                .read_city_hubs(PageRequest::default())
                .unwrap()
                .state,
            ReadState::PackRemoved
        );
        assert_eq!(
            database.read_personal_related(&maison.id).unwrap().notes[0].id,
            "note:read-api"
        );
        runtime
            .install(&mut database, BUNDLED_PACK_ID)
            .expect("pack should reinstall");
        assert_eq!(
            database
                .read_edition("edition:milano-fashion-week:ss27:2026")
                .unwrap()
                .season_code,
            "SS27"
        );

        drop(database);
        std::fs::remove_dir_all(root).expect("read api test directory should be cleaned");
    }
}
