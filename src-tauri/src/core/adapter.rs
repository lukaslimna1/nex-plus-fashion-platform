use crate::core::db::{
    AdapterCandidateRead, AdapterRunRead, CatalogDb, IngestionCandidateRecord,
    IntegrationProposalRead, SourceCandidateRead,
};
use crate::core::error::CoreError;
use crate::core::milano::{bundled_pack, SeedScheduleEntry};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const ADAPTER_CONTRACT_VERSION: &str = "1.0";
pub const CAPABILITY_DISCOVER: &str = "DISCOVER";
pub const CAPABILITY_FETCH_PAGE: &str = "FETCH_PAGE";
pub const CAPABILITY_FETCH_API: &str = "FETCH_API";
pub const CAPABILITY_PARSE_HTML: &str = "PARSE_HTML";
pub const CAPABILITY_EXTRACT_STRUCTURED_DATA: &str = "EXTRACT_STRUCTURED_DATA";
pub const CAPABILITY_DISCOVER_IMAGES: &str = "DISCOVER_IMAGES";
pub const CAPABILITY_DISCOVER_VIDEO: &str = "DISCOVER_VIDEO";
pub const CAPABILITY_DISCOVER_REVIEWS: &str = "DISCOVER_REVIEWS";
pub const CAPABILITY_DISCOVER_BACKSTAGE: &str = "DISCOVER_BACKSTAGE";
pub const CAPABILITY_DISCOVER_DETAILS: &str = "DISCOVER_DETAILS";
pub const CAPABILITY_DISCOVER_EDITORIAL: &str = "DISCOVER_EDITORIAL";
pub const CAPABILITY_DISCOVER_EXTRAS: &str = "DISCOVER_EXTRAS";
pub const CAPABILITY_DISCOVER_RUNWAY: &str = "DISCOVER_RUNWAY";
pub const CAPABILITY_DISCOVER_LOOKS: &str = "DISCOVER_LOOKS";
pub const CAPABILITY_DISCOVER_PERSON: &str = "DISCOVER_PERSON";
pub const CAPABILITY_DISCOVER_COLLECTION: &str = "DISCOVER_COLLECTION";
pub const CAPABILITY_DISCOVER_EVENT: &str = "DISCOVER_EVENT";
pub const CAPABILITY_DISCOVER_EDITION: &str = "DISCOVER_EDITION";
pub const CAPABILITY_FETCH_MEDIA_METADATA: &str = "FETCH_MEDIA_METADATA";
pub const CAPABILITY_FETCH_REVIEW_METADATA: &str = "FETCH_REVIEW_METADATA";
pub const CAPABILITY_PAGINATE: &str = "PAGINATE";
pub const CAPABILITY_DELTA_SYNC: &str = "DELTA_SYNC";
pub const CAPABILITY_HEALTH_CHECK: &str = "HEALTH_CHECK";

pub const ADAPTER_CNMI_MILANO: &str = "adapter:cnmi:milano-calendar";
pub const ADAPTER_BOF_REVIEWS: &str = "adapter:bof:fashion-week-reviews";
pub const SOURCE_CNMI: &str = "source:cnmi";
pub const SOURCE_BOF: &str = "source:business-of-fashion";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterPaginationSpec {
    pub supported: bool,
    pub strategy: String,
    pub cursor_kind: Option<String>,
    pub checkpoint_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterRateLimitSpec {
    pub requests_per_minute: Option<u32>,
    pub burst: Option<u32>,
    pub retry_after_header: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterRetrySpec {
    pub max_attempts: u8,
    pub backoff_ms: Vec<u64>,
    pub retryable_states: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterSpec {
    pub contract_version: &'static str,
    pub adapter_id: String,
    pub version: String,
    pub source_ids: Vec<String>,
    pub integration_ids: Vec<String>,
    pub capabilities: Vec<String>,
    pub discovery_strategy: String,
    pub supported_content_types: Vec<String>,
    pub fetch_strategy: String,
    pub parse_strategy: String,
    pub normalization_strategy: String,
    pub pagination: AdapterPaginationSpec,
    pub rate_limit: AdapterRateLimitSpec,
    pub retry_policy: AdapterRetrySpec,
    pub provenance_support: bool,
    pub produces: Vec<String>,
    pub fixture_support: bool,
    pub test_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdapterHealthState {
    Healthy,
    Degraded,
    RateLimited,
    AuthRequired,
    Blocked,
    Changed,
    Unavailable,
    NotConfigured,
}

impl AdapterHealthState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "HEALTHY",
            Self::Degraded => "DEGRADED",
            Self::RateLimited => "RATE_LIMITED",
            Self::AuthRequired => "AUTH_REQUIRED",
            Self::Blocked => "BLOCKED",
            Self::Changed => "CHANGED",
            Self::Unavailable => "UNAVAILABLE",
            Self::NotConfigured => "NOT_CONFIGURED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterHealth {
    pub adapter_id: String,
    pub state: AdapterHealthState,
    pub detail: Option<String>,
    pub checked_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterRegistryRead {
    pub contract_version: &'static str,
    pub adapters: Vec<AdapterSpec>,
    pub health: Vec<AdapterHealth>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterRunRequest {
    pub adapter_id: String,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub checkpoint_key: Option<String>,
    #[serde(default)]
    pub fixture_mode: bool,
    #[serde(default)]
    pub max_items: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DiscoveryItem {
    pub discovery_key: String,
    pub canonical_url: String,
    pub content_type: String,
    pub external_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RawArtifactInput {
    pub source_id: String,
    pub integration_id: String,
    pub canonical_url: String,
    pub content_type: String,
    pub content: String,
    pub content_ref: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ObservationInput {
    pub entity_type: String,
    pub stable_key: String,
    pub payload: Value,
    pub source_url: String,
}

#[derive(Debug, Clone)]
pub struct NormalizedCandidate {
    pub candidate_kind: String,
    pub target_entity_type: Option<String>,
    pub target_entity_id: Option<String>,
    pub stable_key: String,
    pub proposed: Value,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
}

#[derive(Debug, Clone)]
struct AdapterError {
    code: &'static str,
    message: String,
}

impl AdapterError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

trait SourceAdapter: Send + Sync {
    fn spec(&self) -> AdapterSpec;
    fn health(&self) -> AdapterHealth;
    fn discover(&self, request: &AdapterRunRequest) -> Result<Vec<DiscoveryItem>, AdapterError>;
    fn fetch(
        &self,
        request: &AdapterRunRequest,
        item: &DiscoveryItem,
    ) -> Result<RawArtifactInput, AdapterError>;
    fn parse(&self, artifact: &RawArtifactInput) -> Result<Vec<ObservationInput>, AdapterError>;
    fn normalize(
        &self,
        request: &AdapterRunRequest,
        observation: &ObservationInput,
    ) -> Result<Vec<NormalizedCandidate>, AdapterError>;
}

pub struct AdapterRegistry {
    adapters: Vec<Box<dyn SourceAdapter>>,
}

impl AdapterRegistry {
    pub fn built_in() -> Self {
        Self {
            adapters: vec![Box::new(CnmiMilanoAdapter), Box::new(BofReviewAdapter)],
        }
    }

    pub fn specs(&self) -> Vec<AdapterSpec> {
        self.adapters.iter().map(|adapter| adapter.spec()).collect()
    }

    pub fn health(&self) -> Vec<AdapterHealth> {
        self.adapters
            .iter()
            .map(|adapter| adapter.health())
            .collect()
    }

    pub fn read(&self) -> AdapterRegistryRead {
        AdapterRegistryRead {
            contract_version: ADAPTER_CONTRACT_VERSION,
            adapters: self.specs(),
            health: self.health(),
        }
    }

    pub fn run(
        &self,
        database: &mut CatalogDb,
        request: AdapterRunRequest,
    ) -> Result<AdapterRunRead, CoreError> {
        let adapter = self
            .adapters
            .iter()
            .find(|adapter| adapter.spec().adapter_id == request.adapter_id)
            .ok_or_else(|| CoreError::AdapterNotFound(request.adapter_id.clone()))?;
        let spec = adapter.spec();
        let source_id = request
            .source_id
            .clone()
            .or_else(|| spec.source_ids.first().cloned())
            .ok_or_else(|| CoreError::Adapter("adapter has no related Source".to_string()))?;
        if !spec.source_ids.iter().any(|item| item == &source_id) {
            return Err(CoreError::Adapter(format!(
                "source {} is not registered for adapter {}",
                source_id, spec.adapter_id
            )));
        }
        let capability = request
            .capability
            .clone()
            .unwrap_or_else(|| CAPABILITY_DISCOVER.to_string());
        if !spec.capabilities.iter().any(|item| item == &capability) {
            return Err(CoreError::AdapterCapabilityUnsupported {
                adapter_id: spec.adapter_id,
                capability,
            });
        }

        let run_id = new_id("adapter-run");
        let now = database.current_timestamp()?;
        database.upsert_adapter_spec(&spec, &adapter.health(), &now)?;
        database.start_adapter_run(
            &run_id,
            &spec.adapter_id,
            &source_id,
            Some(&capability),
            request.cursor.as_deref(),
            request
                .checkpoint_key
                .as_deref()
                .or(spec.pagination.checkpoint_key.as_deref()),
            request.fixture_mode,
            &now,
        )?;

        let result = self.run_pipeline(
            database,
            adapter.as_ref(),
            &spec,
            &source_id,
            &run_id,
            &request,
        );
        match result {
            Ok(result) => {
                database.finish_adapter_run(
                    &run_id,
                    "succeeded",
                    result.artifact_count,
                    result.observation_count,
                    result.candidate_count,
                    result.changed_count,
                    request.cursor.as_deref().or(Some("complete")),
                    None,
                    None,
                    &database.current_timestamp()?,
                )?;
                Ok(database.adapter_run(&run_id)?.unwrap_or(result.run))
            }
            Err(error) => {
                let (code, message) = match &error {
                    CoreError::Adapter(message) => ("ADAPTER_ERROR", message.clone()),
                    CoreError::AdapterCapabilityUnsupported { .. } => {
                        ("ADAPTER_CAPABILITY_UNSUPPORTED", error.to_string())
                    }
                    _ => (error.code(), error.to_string()),
                };
                database.finish_adapter_run(
                    &run_id,
                    "failed",
                    0,
                    0,
                    0,
                    0,
                    None,
                    Some(code),
                    Some(&message),
                    &database.current_timestamp()?,
                )?;
                Err(error)
            }
        }
    }

    fn run_pipeline(
        &self,
        database: &mut CatalogDb,
        adapter: &dyn SourceAdapter,
        spec: &AdapterSpec,
        source_id: &str,
        run_id: &str,
        request: &AdapterRunRequest,
    ) -> Result<PipelineResult, CoreError> {
        let discoveries = adapter
            .discover(request)
            .map_err(|error| CoreError::Adapter(format!("{}: {}", error.code, error.message)))?;
        let mut artifacts = 0u32;
        let mut observations = 0u32;
        let mut candidates = 0u32;
        let mut changed = 0u32;
        let mut last_artifact_hash = None;
        for item in discoveries
            .into_iter()
            .take(request.max_items.unwrap_or(u32::MAX) as usize)
        {
            let raw = fetch_with_retries(adapter, request, &item, spec).map_err(|error| {
                CoreError::Adapter(format!("{}: {}", error.code, error.message))
            })?;
            let stored = database.upsert_raw_artifact(&raw, &spec.adapter_id)?;
            last_artifact_hash = Some(stored.content_hash.clone());
            artifacts = artifacts.saturating_add(1);
            for observation in adapter
                .parse(&raw)
                .map_err(|error| CoreError::Adapter(format!("{}: {}", error.code, error.message)))?
            {
                let _observation_id = database.upsert_source_observation(
                    &stored.artifact_id,
                    source_id,
                    &spec.adapter_id,
                    &observation,
                )?;
                observations = observations.saturating_add(1);
                for normalized in adapter.normalize(request, &observation).map_err(|error| {
                    CoreError::Adapter(format!("{}: {}", error.code, error.message))
                })? {
                    let comparison = database.compare_ingestion_candidate(
                        normalized.target_entity_type.as_deref(),
                        normalized.target_entity_id.as_deref(),
                        &normalized.proposed,
                    )?;
                    let content_hash = sha256_json(&normalized.proposed)?;
                    let candidate_id =
                        format!("candidate:{}:{}", spec.adapter_id, normalized.stable_key);
                    let record = IngestionCandidateRecord {
                        candidate_id: candidate_id.clone(),
                        source_id: Some(source_id.to_string()),
                        adapter_id: spec.adapter_id.clone(),
                        run_id: run_id.to_string(),
                        candidate_kind: normalized.candidate_kind,
                        target_entity_type: normalized.target_entity_type,
                        target_entity_id: normalized.target_entity_id,
                        stable_key: normalized.stable_key,
                        proposed: normalized.proposed,
                        evidence: normalized.evidence,
                        provenance: normalized.provenance,
                        comparison_state: comparison.clone(),
                        content_hash,
                    };
                    let upserted = database.upsert_ingestion_candidate(&record)?;
                    database.upsert_ingestion_proposal(
                        &candidate_id,
                        &record.candidate_kind,
                        &record.proposed,
                        &record.evidence,
                        &record.provenance,
                        &comparison,
                        upserted.changed,
                    )?;
                    candidates = candidates.saturating_add(1);
                    changed += u32::from(upserted.changed);
                }
            }
        }
        database.upsert_ingestion_checkpoint(
            &spec.adapter_id,
            source_id,
            request
                .checkpoint_key
                .as_deref()
                .or(spec.pagination.checkpoint_key.as_deref())
                .unwrap_or("default"),
            request.cursor.as_deref().or(Some("complete")),
            last_artifact_hash.as_deref(),
            &database.current_timestamp()?,
        )?;
        let run = database.adapter_run(run_id)?.ok_or_else(|| {
            CoreError::Adapter("adapter run disappeared before completion".to_string())
        })?;
        Ok(PipelineResult {
            run,
            artifact_count: artifacts,
            observation_count: observations,
            candidate_count: candidates,
            changed_count: changed,
        })
    }
}

struct PipelineResult {
    run: AdapterRunRead,
    artifact_count: u32,
    observation_count: u32,
    candidate_count: u32,
    changed_count: u32,
}

fn fetch_with_retries(
    adapter: &dyn SourceAdapter,
    request: &AdapterRunRequest,
    item: &DiscoveryItem,
    spec: &AdapterSpec,
) -> Result<RawArtifactInput, AdapterError> {
    let max_attempts = spec.retry_policy.max_attempts.max(1);
    let mut last_error = None;
    for attempt in 0..max_attempts {
        match adapter.fetch(request, item) {
            Ok(raw) => return Ok(raw),
            Err(error) => {
                let retryable = spec
                    .retry_policy
                    .retryable_states
                    .iter()
                    .any(|state| state == adapter_error_state(error.code));
                last_error = Some(error);
                if !retryable || attempt + 1 >= max_attempts {
                    break;
                }
            }
        }
    }
    Err(last_error.unwrap_or_else(|| AdapterError::new("UNAVAILABLE", "adapter fetch failed")))
}

fn adapter_error_state(code: &str) -> &'static str {
    match code {
        "RATE_LIMITED" => "RATE_LIMITED",
        "AUTH_REQUIRED" => "AUTH_REQUIRED",
        "BLOCKED" => "BLOCKED",
        "CHANGED" => "CHANGED",
        "UNAVAILABLE" => "UNAVAILABLE",
        _ => "UNAVAILABLE",
    }
}

struct CnmiMilanoAdapter;

impl SourceAdapter for CnmiMilanoAdapter {
    fn spec(&self) -> AdapterSpec {
        AdapterSpec {
            contract_version: ADAPTER_CONTRACT_VERSION,
            adapter_id: ADAPTER_CNMI_MILANO.to_string(),
            version: "1.0.0".to_string(),
            source_ids: vec![SOURCE_CNMI.to_string()],
            integration_ids: vec!["integration:cnmi:milano-calendar".to_string()],
            capabilities: vec![
                CAPABILITY_DISCOVER.to_string(),
                CAPABILITY_FETCH_PAGE.to_string(),
                CAPABILITY_EXTRACT_STRUCTURED_DATA.to_string(),
                CAPABILITY_DISCOVER_EVENT.to_string(),
                CAPABILITY_DISCOVER_EDITION.to_string(),
                CAPABILITY_DISCOVER_COLLECTION.to_string(),
                CAPABILITY_DISCOVER_RUNWAY.to_string(),
                CAPABILITY_PAGINATE.to_string(),
                CAPABILITY_HEALTH_CHECK.to_string(),
            ],
            discovery_strategy: "official-calendar-entry".to_string(),
            supported_content_types: vec!["application/json".to_string(), "text/html".to_string()],
            fetch_strategy: "fixture-backed-official-calendar; HTTP GET seam retained".to_string(),
            parse_strategy: "deterministic JSON schedule parser".to_string(),
            normalization_strategy: "stable CNMI external ID to ScheduleEntry candidate"
                .to_string(),
            pagination: AdapterPaginationSpec {
                supported: true,
                strategy: "calendar-day cursor".to_string(),
                cursor_kind: Some("day-or-entry".to_string()),
                checkpoint_key: Some("cnmi-milano-ss27".to_string()),
            },
            rate_limit: AdapterRateLimitSpec {
                requests_per_minute: Some(30),
                burst: Some(2),
                retry_after_header: Some("Retry-After".to_string()),
                notes: Some(
                    "Respect official Source limits when HTTP transport is enabled".to_string(),
                ),
            },
            retry_policy: AdapterRetrySpec {
                max_attempts: 3,
                backoff_ms: vec![250, 1_000, 3_000],
                retryable_states: vec!["RATE_LIMITED".to_string(), "UNAVAILABLE".to_string()],
            },
            provenance_support: true,
            produces: vec![
                "schedule_entry".to_string(),
                "event".to_string(),
                "edition".to_string(),
            ],
            fixture_support: true,
            test_support: true,
        }
    }

    fn health(&self) -> AdapterHealth {
        AdapterHealth {
            adapter_id: ADAPTER_CNMI_MILANO.to_string(),
            state: AdapterHealthState::Healthy,
            detail: Some(
                "official CNMI Milano SS27 fixture is available for deterministic proof"
                    .to_string(),
            ),
            checked_at: None,
        }
    }

    fn discover(&self, _request: &AdapterRunRequest) -> Result<Vec<DiscoveryItem>, AdapterError> {
        Ok(vec![DiscoveryItem {
            discovery_key: "cnmi:milano:ss27:calendar".to_string(),
            canonical_url: "https://milanofashionweek.cameramoda.it/en/calendar".to_string(),
            content_type: "application/json".to_string(),
            external_key: Some("edition:milano-fashion-week:ss27:2026".to_string()),
        }])
    }

    fn fetch(
        &self,
        _request: &AdapterRunRequest,
        item: &DiscoveryItem,
    ) -> Result<RawArtifactInput, AdapterError> {
        let pack = bundled_pack()
            .map_err(|error| AdapterError::new("FIXTURE_INVALID", error.to_string()))?;
        let payload = json!({
            "sourceId": SOURCE_CNMI,
            "editionId": pack.payload.edition.id,
            "sourceUrl": item.canonical_url,
            "retrievedAt": pack.payload.retrieved_at,
            "entries": pack.payload.entries,
        });
        let content = serde_json::to_string(&payload)
            .map_err(|error| AdapterError::new("SERIALIZATION", error.to_string()))?;
        Ok(RawArtifactInput {
            source_id: SOURCE_CNMI.to_string(),
            integration_id: "integration:cnmi:milano-calendar".to_string(),
            canonical_url: item.canonical_url.clone(),
            content_type: item.content_type.clone(),
            content,
            content_ref: Some("pack:nex.fashion.milano.ss27/schedule".to_string()),
            etag: None,
            last_modified: None,
        })
    }

    fn parse(&self, artifact: &RawArtifactInput) -> Result<Vec<ObservationInput>, AdapterError> {
        let parsed: CnmiCalendarArtifact = serde_json::from_str(&artifact.content)
            .map_err(|error| AdapterError::new("PARSE_FAILED", error.to_string()))?;
        if parsed.entries.is_empty() {
            return Err(AdapterError::new(
                "EMPTY_SOURCE",
                "CNMI calendar contained no entries",
            ));
        }
        parsed
            .entries
            .into_iter()
            .map(|entry| {
                let stable_key = format!("schedule_entry:{}", entry.source_external_id);
                Ok(ObservationInput {
                    entity_type: "schedule_entry".to_string(),
                    stable_key,
                    payload: serde_json::to_value(entry)
                        .map_err(|error| AdapterError::new("SERIALIZATION", error.to_string()))?,
                    source_url: artifact.canonical_url.clone(),
                })
            })
            .collect()
    }

    fn normalize(
        &self,
        _request: &AdapterRunRequest,
        observation: &ObservationInput,
    ) -> Result<Vec<NormalizedCandidate>, AdapterError> {
        let entry: SeedScheduleEntry = serde_json::from_value(observation.payload.clone())
            .map_err(|error| AdapterError::new("NORMALIZATION_FAILED", error.to_string()))?;
        Ok(vec![NormalizedCandidate {
            candidate_kind: "schedule_entry_observation".to_string(),
            target_entity_type: Some("schedule_entry".to_string()),
            target_entity_id: Some(entry.id.clone()),
            stable_key: format!("schedule_entry:{}", entry.source_external_id),
            proposed: serde_json::to_value(entry)
                .map_err(|error| AdapterError::new("SERIALIZATION", error.to_string()))?,
            evidence: vec![json!({"url": observation.source_url, "kind": "official_calendar"})],
            provenance: vec![json!({
                "sourceId": SOURCE_CNMI,
                "adapterId": ADAPTER_CNMI_MILANO,
                "authority": "official"
            })],
        }])
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CnmiCalendarArtifact {
    entries: Vec<SeedScheduleEntry>,
}

struct BofReviewAdapter;

impl SourceAdapter for BofReviewAdapter {
    fn spec(&self) -> AdapterSpec {
        AdapterSpec {
            contract_version: ADAPTER_CONTRACT_VERSION,
            adapter_id: ADAPTER_BOF_REVIEWS.to_string(),
            version: "1.0.0".to_string(),
            source_ids: vec![SOURCE_BOF.to_string()],
            integration_ids: vec!["integration:bof:review-page".to_string()],
            capabilities: vec![
                CAPABILITY_DISCOVER.to_string(),
                CAPABILITY_FETCH_PAGE.to_string(),
                CAPABILITY_PARSE_HTML.to_string(),
                CAPABILITY_FETCH_REVIEW_METADATA.to_string(),
                CAPABILITY_DISCOVER_REVIEWS.to_string(),
                CAPABILITY_DISCOVER_EDITORIAL.to_string(),
                CAPABILITY_HEALTH_CHECK.to_string(),
            ],
            discovery_strategy: "documented-review-url".to_string(),
            supported_content_types: vec!["text/html".to_string(), "application/json".to_string()],
            fetch_strategy: "metadata fixture; subscription-limited content is not copied"
                .to_string(),
            parse_strategy: "deterministic metadata parser".to_string(),
            normalization_strategy: "review metadata candidate preserving original URL".to_string(),
            pagination: AdapterPaginationSpec {
                supported: false,
                strategy: "single documented page".to_string(),
                cursor_kind: None,
                checkpoint_key: Some("bof-review-metadata".to_string()),
            },
            rate_limit: AdapterRateLimitSpec {
                requests_per_minute: Some(10),
                burst: Some(1),
                retry_after_header: Some("Retry-After".to_string()),
                notes: Some("Subscription and rights limits remain explicit".to_string()),
            },
            retry_policy: AdapterRetrySpec {
                max_attempts: 2,
                backoff_ms: vec![500, 2_000],
                retryable_states: vec!["RATE_LIMITED".to_string(), "UNAVAILABLE".to_string()],
            },
            provenance_support: true,
            produces: vec![
                "review_metadata".to_string(),
                "source_candidate".to_string(),
            ],
            fixture_support: true,
            test_support: true,
        }
    }

    fn health(&self) -> AdapterHealth {
        AdapterHealth {
            adapter_id: ADAPTER_BOF_REVIEWS.to_string(),
            state: AdapterHealthState::Degraded,
            detail: Some(
                "metadata-only fixture; article body and subscription content are not acquired"
                    .to_string(),
            ),
            checked_at: None,
        }
    }

    fn discover(&self, _request: &AdapterRunRequest) -> Result<Vec<DiscoveryItem>, AdapterError> {
        Ok(vec![DiscoveryItem {
            discovery_key: "bof:milano:ss27:review:prada-thom-browne-ralph-lauren".to_string(),
            canonical_url: "https://www.businessoffashion.com/reviews/fashion-week/prada-thom-browne-ralph-lauren-menswear-spring-summer-2027/".to_string(),
            content_type: "application/json".to_string(),
            external_key: Some("review:bof:prada-thom-browne-ralph-lauren-ss27".to_string()),
        }])
    }

    fn fetch(
        &self,
        _request: &AdapterRunRequest,
        item: &DiscoveryItem,
    ) -> Result<RawArtifactInput, AdapterError> {
        let content = serde_json::to_string(&json!({
            "sourceId": SOURCE_BOF,
            "originalUrl": item.canonical_url,
            "title": "Prada, Thom Browne, Ralph Lauren Menswear Spring/Summer 2027",
            "language": "en",
            "access": "metadata_only",
            "rightsNote": "Some articles/resources are limited by subscription; preserve metadata, authorship, link and current availability.",
        })).map_err(|error| AdapterError::new("SERIALIZATION", error.to_string()))?;
        Ok(RawArtifactInput {
            source_id: SOURCE_BOF.to_string(),
            integration_id: "integration:bof:review-page".to_string(),
            canonical_url: item.canonical_url.clone(),
            content_type: item.content_type.clone(),
            content,
            content_ref: Some("fixture:bof:review-metadata".to_string()),
            etag: None,
            last_modified: None,
        })
    }

    fn parse(&self, artifact: &RawArtifactInput) -> Result<Vec<ObservationInput>, AdapterError> {
        let value: Value = serde_json::from_str(&artifact.content)
            .map_err(|error| AdapterError::new("PARSE_FAILED", error.to_string()))?;
        let url = value
            .get("originalUrl")
            .and_then(Value::as_str)
            .ok_or_else(|| AdapterError::new("PARSE_FAILED", "review metadata lacks originalUrl"))?
            .to_string();
        Ok(vec![ObservationInput {
            entity_type: "review".to_string(),
            stable_key: format!("review:{url}"),
            payload: value,
            source_url: url,
        }])
    }

    fn normalize(
        &self,
        _request: &AdapterRunRequest,
        observation: &ObservationInput,
    ) -> Result<Vec<NormalizedCandidate>, AdapterError> {
        let url = observation
            .payload
            .get("originalUrl")
            .and_then(Value::as_str)
            .ok_or_else(|| AdapterError::new("NORMALIZATION_FAILED", "review lacks originalUrl"))?;
        Ok(vec![NormalizedCandidate {
            candidate_kind: "review_metadata".to_string(),
            target_entity_type: Some("review".to_string()),
            target_entity_id: Some("review:bof:prada-thom-browne-ralph-lauren-ss27".to_string()),
            stable_key: format!("review:{url}"),
            proposed: observation.payload.clone(),
            evidence: vec![json!({"url": url, "kind": "professional_review_metadata"})],
            provenance: vec![json!({
                "sourceId": SOURCE_BOF,
                "adapterId": ADAPTER_BOF_REVIEWS,
                "access": "metadata_only"
            })],
        }])
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceCandidateRequest {
    pub source_key: String,
    pub display_name: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub source_kind: Option<String>,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub locale: BTreeMap<String, String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub evidence: Vec<Value>,
    #[serde(default)]
    pub provenance: Vec<Value>,
    #[serde(default)]
    pub discovered_by: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterCandidateRequest {
    #[serde(default)]
    pub source_candidate_id: Option<String>,
    #[serde(default)]
    pub adapter_id: Option<String>,
    pub proposed: Value,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub evidence: Vec<Value>,
    #[serde(default)]
    pub provenance: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationProposalRequest {
    #[serde(default)]
    pub source_candidate_id: Option<String>,
    #[serde(default)]
    pub adapter_candidate_id: Option<String>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub adapter_id: Option<String>,
    pub proposal: Value,
    #[serde(default)]
    pub evidence: Vec<Value>,
    #[serde(default)]
    pub provenance: Vec<Value>,
}

pub fn create_source_candidate(
    database: &mut CatalogDb,
    request: SourceCandidateRequest,
) -> Result<SourceCandidateRead, CoreError> {
    if request.source_key.trim().is_empty() || request.display_name.trim().is_empty() {
        return Err(CoreError::InvalidRequest(
            "sourceKey and displayName are required".to_string(),
        ));
    }
    database.upsert_source_candidate(&request)
}

pub fn create_adapter_candidate(
    database: &mut CatalogDb,
    request: AdapterCandidateRequest,
) -> Result<AdapterCandidateRead, CoreError> {
    database.create_adapter_candidate(&request)
}

pub fn create_integration_proposal(
    database: &mut CatalogDb,
    request: IntegrationProposalRequest,
) -> Result<IntegrationProposalRead, CoreError> {
    database.create_integration_proposal(&request)
}

fn sha256_json(value: &Value) -> Result<String, CoreError> {
    let canonical = serde_json::to_string(value)?;
    Ok(format!("{:x}", Sha256::digest(canonical.as_bytes())))
}

fn new_id(prefix: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{prefix}:{nanos:x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pack::{PackRuntime, BUNDLED_PACK_ID};
    use crate::core::read::PageRequest;

    fn installed_database() -> (CatalogDb, PackRuntime, std::path::PathBuf) {
        let root = std::env::temp_dir().join(format!(
            "nex-fashion-adapter-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let runtime = PackRuntime::new(&root).unwrap();
        let mut database = CatalogDb::in_memory().unwrap();
        runtime.ensure_available(&mut database).unwrap();
        runtime.install(&mut database, BUNDLED_PACK_ID).unwrap();
        (database, runtime, root)
    }

    #[test]
    fn registry_exposes_two_independent_adapters_and_extensible_capabilities() {
        let registry = AdapterRegistry::built_in();
        let specs = registry.specs();
        assert_eq!(specs.len(), 2);
        assert!(specs
            .iter()
            .any(|spec| spec.adapter_id == ADAPTER_CNMI_MILANO));
        assert!(specs
            .iter()
            .any(|spec| spec.adapter_id == ADAPTER_BOF_REVIEWS));
        assert!(specs.iter().all(|spec| spec.provenance_support));
        assert!(specs.iter().any(|spec| {
            spec.capabilities
                .iter()
                .any(|item| item == CAPABILITY_PARSE_HTML)
        }));
    }

    #[test]
    fn unsupported_capability_is_rejected_without_running_adapter() {
        let registry = AdapterRegistry::built_in();
        let (mut database, _runtime, root) = installed_database();
        let error = registry
            .run(
                &mut database,
                AdapterRunRequest {
                    adapter_id: ADAPTER_CNMI_MILANO.to_string(),
                    source_id: None,
                    capability: Some("FUTURE_UNREGISTERED_CAPABILITY".to_string()),
                    cursor: None,
                    checkpoint_key: None,
                    fixture_mode: true,
                    max_items: None,
                },
            )
            .unwrap_err();
        assert!(matches!(
            error,
            CoreError::AdapterCapabilityUnsupported { .. }
        ));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cnmi_pipeline_creates_schedule_candidates_without_official_promotion() {
        let registry = AdapterRegistry::built_in();
        let (mut database, _runtime, root) = installed_database();
        let result = registry
            .run(
                &mut database,
                AdapterRunRequest {
                    adapter_id: ADAPTER_CNMI_MILANO.to_string(),
                    source_id: Some(SOURCE_CNMI.to_string()),
                    capability: Some(CAPABILITY_DISCOVER.to_string()),
                    cursor: None,
                    checkpoint_key: None,
                    fixture_mode: true,
                    max_items: None,
                },
            )
            .unwrap();
        assert_eq!(result.status, "succeeded");
        assert_eq!(result.artifact_count, 1);
        assert_eq!(result.observation_count, 214);
        assert_eq!(result.candidate_count, 214);
        assert_eq!(
            database
                .raw_artifacts(Some(ADAPTER_CNMI_MILANO), 10)
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            database
                .ingestion_candidates(Some(ADAPTER_CNMI_MILANO), Some("open"), 300)
                .unwrap()
                .len(),
            200
        );
        assert_eq!(
            database
                .ingestion_candidate_count(ADAPTER_CNMI_MILANO)
                .unwrap(),
            214
        );
        assert_eq!(
            database
                .ingestion_proposal_count(ADAPTER_CNMI_MILANO)
                .unwrap(),
            214
        );
        assert_eq!(database.official_schedule_count().unwrap(), 214);
        assert_eq!(
            database.ingestion_candidate_statuses().unwrap(),
            vec!["open"]
        );
        assert_eq!(
            database.ingestion_candidate_comparison_states().unwrap(),
            vec!["MATCHED"]
        );
        assert!(database
            .ingestion_proposal_states()
            .unwrap()
            .iter()
            .all(|state| state == "pending"));
        assert!(database
            .read_schedule(
                "edition:milano-fashion-week:ss27:2026",
                PageRequest::default()
            )
            .unwrap()
            .items
            .iter()
            .any(|entry| entry.id == "schedule:cnmi:13639"));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn changed_schedule_content_is_detected_without_official_mutation() {
        let (database, _runtime, root) = installed_database();
        let changed = json!({"sourceHash": "changed-content-hash"});
        assert_eq!(
            database
                .compare_ingestion_candidate(
                    Some("schedule_entry"),
                    Some("schedule:cnmi:13639"),
                    &changed
                )
                .unwrap(),
            "CHANGED"
        );
        assert_eq!(database.official_schedule_count().unwrap(), 214);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn second_adapter_preserves_metadata_only_rights_and_is_idempotent() {
        let registry = AdapterRegistry::built_in();
        let (mut database, _runtime, root) = installed_database();
        let request = AdapterRunRequest {
            adapter_id: ADAPTER_BOF_REVIEWS.to_string(),
            source_id: Some(SOURCE_BOF.to_string()),
            capability: Some(CAPABILITY_DISCOVER_REVIEWS.to_string()),
            cursor: None,
            checkpoint_key: None,
            fixture_mode: true,
            max_items: None,
        };
        let first = registry.run(&mut database, request.clone()).unwrap();
        let second = registry.run(&mut database, request).unwrap();
        assert_eq!(first.status, "succeeded");
        assert_eq!(second.status, "succeeded");
        assert_eq!(database.raw_artifact_count(ADAPTER_BOF_REVIEWS).unwrap(), 1);
        assert_eq!(
            database
                .ingestion_candidate_count(ADAPTER_BOF_REVIEWS)
                .unwrap(),
            1
        );
        assert_eq!(
            database
                .ingestion_proposal_count(ADAPTER_BOF_REVIEWS)
                .unwrap(),
            1
        );
        let candidate = database
            .ingestion_candidate_payload(ADAPTER_BOF_REVIEWS)
            .unwrap();
        assert_eq!(candidate["access"], "metadata_only");
        assert!(candidate.get("rightsNote").is_some());
        assert_eq!(database.official_review_count().unwrap(), 0);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn unknown_source_stays_candidate_until_human_review() {
        let mut database = CatalogDb::in_memory().unwrap();
        let source = create_source_candidate(
            &mut database,
            SourceCandidateRequest {
                source_key: "source:discovered:atelier-example".to_string(),
                display_name: "New editorial source".to_string(),
                base_url: Some("https://example.invalid/fashion".to_string()),
                source_kind: Some("editorial".to_string()),
                domains: vec!["example.invalid".to_string()],
                locale: BTreeMap::new(),
                capabilities: vec![CAPABILITY_DISCOVER_IMAGES.to_string()],
                evidence: vec![json!({"url": "https://example.invalid/fashion"})],
                provenance: vec![json!({"agent": "ai-router"})],
                discovered_by: Some("ai-router".to_string()),
            },
        )
        .unwrap();
        assert_eq!(source.status, "pending");
        let adapter = create_adapter_candidate(
            &mut database,
            AdapterCandidateRequest {
                source_candidate_id: Some(source.source_candidate_id.clone()),
                adapter_id: None,
                proposed: json!({"strategy": "html-gallery"}),
                capabilities: vec![CAPABILITY_DISCOVER_IMAGES.to_string()],
                evidence: vec![json!({"url": "https://example.invalid/fashion"})],
                provenance: vec![json!({"agent": "ai-router"})],
            },
        )
        .unwrap();
        let proposal = create_integration_proposal(
            &mut database,
            IntegrationProposalRequest {
                source_candidate_id: Some(source.source_candidate_id),
                adapter_candidate_id: Some(adapter.adapter_candidate_id),
                source_id: None,
                adapter_id: None,
                proposal: json!({"endpoint": "https://example.invalid/fashion"}),
                evidence: vec![],
                provenance: vec![json!({"agent": "ai-router"})],
            },
        )
        .unwrap();
        assert_eq!(proposal.state, "pending");
        assert!(!proposal.official_applied);
    }
}
