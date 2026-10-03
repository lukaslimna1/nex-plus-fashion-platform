use crate::core::ai::{AiRouter, AiTaskRequest};
use crate::core::db::{
    AdapterCandidateRead, AdapterRunRead, CatalogDb, IngestionCandidateRecord,
    IntegrationProposalRead, SourceCandidateRead,
};
use crate::core::error::CoreError;
use crate::core::milano::{bundled_pack, SeedScheduleEntry};
use reqwest::blocking::Client;
use reqwest::header::{
    HeaderMap, HeaderValue, ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED, USER_AGENT,
};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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
pub const CAPABILITY_FETCH_RSS: &str = "FETCH_RSS";
pub const CAPABILITY_FETCH_JSON: &str = "FETCH_JSON";
pub const CAPABILITY_FETCH_HTML: &str = "FETCH_HTML";
pub const CAPABILITY_CRAWL: &str = "CRAWL";
pub const CAPABILITY_BROWSER_AUTOMATION: &str = "BROWSER_AUTOMATION";
pub const CAPABILITY_AI_ASSIST: &str = "AI_ASSIST";
pub const CAPABILITY_CHECKPOINT: &str = "CHECKPOINT";

pub const ACQUISITION_API: &str = "api";
pub const ACQUISITION_JSON: &str = "json";
pub const ACQUISITION_RSS: &str = "rss";
pub const ACQUISITION_HTML: &str = "html";
pub const ACQUISITION_BROWSER: &str = "browser";
pub const ACQUISITION_MEDIA_ENDPOINT: &str = "media_endpoint";
pub const ACQUISITION_SITEMAP: &str = "sitemap";
pub const ACQUISITION_CHANNEL_FEED: &str = "channel_feed";
pub const ACQUISITION_FIXTURE: &str = "fixture";

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
pub struct AcquisitionStrategySpec {
    pub id: String,
    pub method: String,
    pub priority: u8,
    pub deterministic: bool,
    pub requires_browser: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterScopeSpec {
    pub allowed_domains: Vec<String>,
    pub same_origin_only: bool,
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
    pub acquisition_strategies: Vec<AcquisitionStrategySpec>,
    pub scope: AdapterScopeSpec,
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

pub fn select_acquisition_strategy(
    spec: &AdapterSpec,
    available_methods: &[&str],
) -> Option<AcquisitionStrategySpec> {
    let mut strategies = spec
        .acquisition_strategies
        .iter()
        .filter(|strategy| {
            available_methods
                .iter()
                .any(|method| *method == strategy.method)
        })
        .cloned()
        .collect::<Vec<_>>();
    strategies.sort_by_key(|strategy| (strategy.requires_browser, strategy.priority));
    strategies.into_iter().next()
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
    #[serde(default = "default_max_pages")]
    pub max_pages: u32,
    #[serde(default = "default_max_depth")]
    pub max_depth: u32,
    #[serde(default = "default_max_bytes")]
    pub max_bytes: u64,
    #[serde(default = "default_max_duration_ms")]
    pub max_duration_ms: u64,
    #[serde(default = "default_resume")]
    pub resume: bool,
    #[serde(default)]
    pub ai_assist: bool,
    #[serde(default)]
    pub cancel_key: Option<String>,
}

fn default_max_pages() -> u32 {
    100
}

fn default_max_depth() -> u32 {
    2
}

fn default_max_bytes() -> u64 {
    5 * 1024 * 1024
}

fn default_max_duration_ms() -> u64 {
    120_000
}

fn default_resume() -> bool {
    true
}

#[derive(Debug, Clone)]
pub struct DiscoveryItem {
    pub discovery_key: String,
    pub canonical_url: String,
    pub content_type: String,
    pub external_key: Option<String>,
    pub acquisition_method: String,
    pub strategy_id: String,
    pub depth: u32,
    pub parent_url: Option<String>,
    pub referrer_url: Option<String>,
    pub pagination_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DiscoveryBatch {
    pub items: Vec<DiscoveryItem>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub observations: Vec<ObservationInput>,
    pub discoveries: Vec<DiscoveryItem>,
}

#[derive(Debug, Clone)]
pub struct FetchContext {
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub max_bytes: u64,
    pub timeout_ms: u64,
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
    pub final_url: Option<String>,
    pub acquisition_method: String,
    pub http_status: Option<u16>,
    pub parent_url: Option<String>,
    pub referrer_url: Option<String>,
    pub pagination: Value,
    pub retrieval_status: String,
}

#[derive(Debug, Clone)]
pub struct NotModifiedArtifact {
    pub canonical_url: String,
    pub final_url: Option<String>,
    pub http_status: u16,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub acquisition_method: String,
    pub parent_url: Option<String>,
    pub referrer_url: Option<String>,
    pub pagination: Value,
}

#[derive(Debug, Clone)]
pub enum FetchOutcome {
    Acquired(RawArtifactInput),
    NotModified(NotModifiedArtifact),
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
    retry_after_ms: Option<u64>,
}

impl AdapterError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            retry_after_ms: None,
        }
    }

    fn retry_after(mut self, delay_ms: u64) -> Self {
        self.retry_after_ms = Some(delay_ms);
        self
    }
}

trait SourceAdapter: Send + Sync {
    fn spec(&self) -> AdapterSpec;
    fn health(&self) -> AdapterHealth;
    fn discover(&self, request: &AdapterRunRequest) -> Result<DiscoveryBatch, AdapterError>;
    fn fetch(
        &self,
        request: &AdapterRunRequest,
        item: &DiscoveryItem,
        context: &FetchContext,
        http: &HttpAcquisitionClient,
    ) -> Result<FetchOutcome, AdapterError>;
    fn parse(&self, artifact: &RawArtifactInput) -> Result<ParseResult, AdapterError>;
    fn ai_request(
        &self,
        _request: &AdapterRunRequest,
        _artifact: &RawArtifactInput,
        _parsed: &ParseResult,
    ) -> Result<Option<AiTaskRequest>, AdapterError> {
        Ok(None)
    }
    fn normalize(
        &self,
        request: &AdapterRunRequest,
        observation: &ObservationInput,
        ai_output: Option<&Value>,
    ) -> Result<Vec<NormalizedCandidate>, AdapterError>;
}

pub struct HttpAcquisitionClient {
    client: Client,
    user_agent: HeaderValue,
}

impl HttpAcquisitionClient {
    fn new() -> Self {
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::limited(5))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| Client::new());
        let user_agent = std::env::var("NEX_ACQUISITION_USER_AGENT")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "NEX+Fashion-Core/2.0 (source-acquisition)".to_string());
        Self {
            client,
            user_agent: HeaderValue::from_str(&user_agent)
                .unwrap_or_else(|_| HeaderValue::from_static("NEX+Fashion-Core/2.0")),
        }
    }

    fn fetch(
        &self,
        item: &DiscoveryItem,
        context: &FetchContext,
        supported_content_types: &[String],
    ) -> Result<FetchOutcome, AdapterError> {
        let url = canonicalize_url(&item.canonical_url)
            .map_err(|error| AdapterError::new("INVALID_URL", error))?;
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, self.user_agent.clone());
        if let Some(etag) = context.etag.as_deref() {
            headers.insert(
                IF_NONE_MATCH,
                HeaderValue::from_str(etag)
                    .map_err(|_| AdapterError::new("INVALID_HEADER", "invalid ETag"))?,
            );
        }
        if let Some(last_modified) = context.last_modified.as_deref() {
            headers.insert(
                IF_MODIFIED_SINCE,
                HeaderValue::from_str(last_modified)
                    .map_err(|_| AdapterError::new("INVALID_HEADER", "invalid Last-Modified"))?,
            );
        }
        let response = self
            .client
            .get(url.as_str())
            .headers(headers)
            .timeout(Duration::from_millis(context.timeout_ms.max(1_000)))
            .send()
            .map_err(|error| AdapterError::new("UNAVAILABLE", error.to_string()))?;
        let status = response.status();
        let final_url = response.url().to_string();
        let etag = header_string(response.headers(), ETAG);
        let last_modified = header_string(response.headers(), LAST_MODIFIED);
        if status == StatusCode::NOT_MODIFIED {
            return Ok(FetchOutcome::NotModified(NotModifiedArtifact {
                canonical_url: item.canonical_url.clone(),
                final_url: Some(final_url),
                http_status: status.as_u16(),
                etag,
                last_modified,
                acquisition_method: item.acquisition_method.clone(),
                parent_url: item.parent_url.clone(),
                referrer_url: item.referrer_url.clone(),
                pagination: json!({
                    "cursor": item.pagination_cursor,
                    "depth": item.depth
                }),
            }));
        }
        if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<u64>().ok())
                .map(|seconds| seconds.saturating_mul(1_000));
            return Err(AdapterError::new(
                if status == StatusCode::TOO_MANY_REQUESTS {
                    "RATE_LIMITED"
                } else {
                    "UNAVAILABLE"
                },
                format!("source returned HTTP {}", status.as_u16()),
            )
            .retry_after(retry_after.unwrap_or_default()));
        }
        if !status.is_success() {
            return Err(AdapterError::new(
                if status == StatusCode::FORBIDDEN || status == StatusCode::UNAUTHORIZED {
                    "BLOCKED"
                } else {
                    "UNAVAILABLE"
                },
                format!("source returned HTTP {}", status.as_u16()),
            ));
        }
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("application/octet-stream")
            .split(';')
            .next()
            .unwrap_or("application/octet-stream")
            .trim()
            .to_ascii_lowercase();
        if !supported_content_types.is_empty()
            && !supported_content_types.iter().any(|supported| {
                supported.eq_ignore_ascii_case(&content_type)
                    || supported == "*/*"
                    || (supported.ends_with("/*")
                        && content_type.starts_with(&supported[..supported.len() - 1]))
            })
        {
            return Err(AdapterError::new(
                "MIME_REJECTED",
                format!("unsupported source MIME type: {content_type}"),
            ));
        }
        let mut body = Vec::new();
        let max_bytes = context.max_bytes.max(1);
        response
            .take(max_bytes.saturating_add(1))
            .read_to_end(&mut body)
            .map_err(|error| AdapterError::new("UNAVAILABLE", error.to_string()))?;
        if body.len() as u64 > max_bytes {
            return Err(AdapterError::new(
                "RESPONSE_TOO_LARGE",
                format!("response exceeded {} bytes", max_bytes),
            ));
        }
        let content = String::from_utf8(body)
            .map_err(|_| AdapterError::new("MIME_REJECTED", "text acquisition was not UTF-8"))?;
        Ok(FetchOutcome::Acquired(RawArtifactInput {
            source_id: String::new(),
            integration_id: String::new(),
            canonical_url: item.canonical_url.clone(),
            content_type,
            content,
            content_ref: None,
            etag,
            last_modified,
            final_url: Some(final_url),
            acquisition_method: item.acquisition_method.clone(),
            http_status: Some(status.as_u16()),
            parent_url: item.parent_url.clone(),
            referrer_url: item.referrer_url.clone(),
            pagination: json!({
                "cursor": item.pagination_cursor,
                "depth": item.depth
            }),
            retrieval_status: "acquired".to_string(),
        }))
    }
}

fn header_string(headers: &HeaderMap, name: reqwest::header::HeaderName) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

#[derive(Default)]
struct RateLimiter {
    next_allowed: Mutex<HashMap<String, Instant>>,
}

impl RateLimiter {
    fn wait(&self, source_id: &str, url: &str, spec: &AdapterRateLimitSpec, fixture: bool) {
        if fixture {
            return;
        }
        let Some(requests_per_minute) = spec.requests_per_minute.filter(|value| *value > 0) else {
            return;
        };
        let interval = Duration::from_secs_f64(60.0 / requests_per_minute as f64);
        let key = format!(
            "{source_id}:{}",
            host_of(url).unwrap_or_else(|| url.to_string())
        );
        let now = Instant::now();
        let wait_until = self.next_allowed.lock().ok().and_then(|mut slots| {
            let wait_until = slots.get(&key).copied();
            slots.insert(key, now + interval);
            wait_until
        });
        if let Some(wait_until) = wait_until {
            if wait_until > now {
                thread::sleep(wait_until - now);
            }
        }
    }
}

pub struct AdapterRegistry {
    adapters: Vec<Box<dyn SourceAdapter>>,
    http: HttpAcquisitionClient,
    rate_limiter: RateLimiter,
    cancellations: Arc<Mutex<HashSet<String>>>,
}

impl AdapterRegistry {
    pub fn built_in() -> Self {
        Self {
            adapters: vec![Box::new(CnmiMilanoAdapter), Box::new(BofReviewAdapter)],
            http: HttpAcquisitionClient::new(),
            rate_limiter: RateLimiter::default(),
            cancellations: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn cancel(&self, cancel_key: &str) {
        if let Ok(mut cancellations) = self.cancellations.lock() {
            cancellations.insert(cancel_key.to_string());
        }
    }

    fn is_cancelled(&self, cancel_key: Option<&str>) -> bool {
        cancel_key
            .and_then(|key| {
                self.cancellations
                    .lock()
                    .ok()
                    .map(|items| items.contains(key))
            })
            .unwrap_or(false)
    }

    fn clear_cancellation(&self, cancel_key: Option<&str>) {
        if let Some(key) = cancel_key {
            if let Ok(mut cancellations) = self.cancellations.lock() {
                cancellations.remove(key);
            }
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
        ai: &AiRouter,
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
            ai,
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
                    result.attempts,
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
                    0,
                    None,
                    Some(code),
                    Some(&message),
                    &database.current_timestamp()?,
                )?;
                Err(error)
            }
        }
        .inspect(|_| self.clear_cancellation(request.cancel_key.as_deref()))
        .inspect_err(|_| self.clear_cancellation(request.cancel_key.as_deref()))
    }

    fn run_pipeline(
        &self,
        database: &mut CatalogDb,
        ai: &AiRouter,
        adapter: &dyn SourceAdapter,
        spec: &AdapterSpec,
        source_id: &str,
        run_id: &str,
        request: &AdapterRunRequest,
    ) -> Result<PipelineResult, CoreError> {
        let started = Instant::now();
        let checkpoint_key = request
            .checkpoint_key
            .as_deref()
            .or(spec.pagination.checkpoint_key.as_deref())
            .unwrap_or("default");
        let mut page_cursor = request.cursor.clone();
        if request.resume && page_cursor.is_none() {
            page_cursor = database
                .ingestion_checkpoint(&spec.adapter_id, source_id, checkpoint_key)?
                .and_then(|checkpoint| checkpoint.0)
                .filter(|cursor| cursor != "complete");
        }
        let mut page_count = 0u32;
        let mut item_count = 0u32;
        let mut attempts = 0u32;
        let mut queue = VecDeque::new();
        let mut seen_urls = HashSet::new();
        let mut artifacts = 0u32;
        let mut observations = 0u32;
        let mut candidates = 0u32;
        let mut changed = 0u32;
        let mut last_artifact_hash = None;
        let max_items = request.max_items.unwrap_or(u32::MAX);
        let max_pages = request.max_pages.max(1);
        let max_depth = request.max_depth;
        let max_duration = Duration::from_millis(request.max_duration_ms.max(1_000));
        let mut has_page = true;
        while page_count < max_pages && item_count < max_items {
            if started.elapsed() >= max_duration {
                return Err(CoreError::Adapter("max duration exceeded".to_string()));
            }
            if self.is_cancelled(request.cancel_key.as_deref()) {
                return Err(CoreError::AdapterCancelled);
            }
            let page_request = AdapterRunRequest {
                cursor: page_cursor.clone(),
                ..request.clone()
            };
            let batch = adapter.discover(&page_request).map_err(|error| {
                CoreError::Adapter(format!("{}: {}", error.code, error.message))
            })?;
            page_count = page_count.saturating_add(1);
            for item in batch.items {
                if item.depth > max_depth || !url_in_scope(&item.canonical_url, &spec.scope) {
                    continue;
                }
                let canonical_url =
                    canonicalize_url(&item.canonical_url).map_err(CoreError::Adapter)?;
                if seen_urls.insert(canonical_url) {
                    queue.push_back(item);
                }
            }
            page_cursor = batch.next_cursor;
            while let Some(item) = queue.pop_front() {
                if item_count >= max_items {
                    break;
                }
                if started.elapsed() >= max_duration {
                    return Err(CoreError::Adapter("max duration exceeded".to_string()));
                }
                if self.is_cancelled(request.cancel_key.as_deref()) {
                    return Err(CoreError::AdapterCancelled);
                }
                self.rate_limiter.wait(
                    source_id,
                    &item.canonical_url,
                    &spec.rate_limit,
                    request.fixture_mode,
                );
                let validators =
                    database.raw_artifact_validators(&spec.adapter_id, &item.canonical_url)?;
                let context = FetchContext {
                    etag: validators.as_ref().and_then(|values| values.0.clone()),
                    last_modified: validators.as_ref().and_then(|values| values.1.clone()),
                    max_bytes: request.max_bytes,
                    timeout_ms: request.max_duration_ms.min(300_000),
                };
                let (outcome, item_attempts) =
                    fetch_with_retries(adapter, request, &item, &context, &self.http, spec)
                        .map_err(|error| {
                            CoreError::Adapter(format!("{}: {}", error.code, error.message))
                        })?;
                attempts = attempts.saturating_add(item_attempts);
                item_count = item_count.saturating_add(1);
                match outcome {
                    FetchOutcome::NotModified(not_modified) => {
                        database.mark_raw_artifact_not_modified(
                            &spec.adapter_id,
                            &not_modified.canonical_url,
                            not_modified.final_url.as_deref(),
                            not_modified.http_status,
                            not_modified.etag.as_deref(),
                            not_modified.last_modified.as_deref(),
                            &not_modified.acquisition_method,
                            not_modified.parent_url.as_deref(),
                            not_modified.referrer_url.as_deref(),
                            &not_modified.pagination,
                        )?;
                        database.upsert_ingestion_checkpoint(
                            &spec.adapter_id,
                            source_id,
                            checkpoint_key,
                            item.pagination_cursor.as_deref().or(page_cursor.as_deref()),
                            last_artifact_hash.as_deref(),
                            &database.current_timestamp()?,
                        )?;
                        continue;
                    }
                    FetchOutcome::Acquired(mut raw) => {
                        raw.source_id = source_id.to_string();
                        raw.integration_id =
                            spec.integration_ids.first().cloned().unwrap_or_default();
                        let stored = database.upsert_raw_artifact(&raw, &spec.adapter_id)?;
                        last_artifact_hash = Some(stored.content_hash.clone());
                        artifacts = artifacts.saturating_add(1);
                        let parsed = adapter.parse(&raw).map_err(|error| {
                            CoreError::Adapter(format!("{}: {}", error.code, error.message))
                        })?;
                        for discovered in parsed.discoveries.iter().cloned() {
                            if discovered.depth <= max_depth
                                && url_in_scope(&discovered.canonical_url, &spec.scope)
                            {
                                let canonical_url = canonicalize_url(&discovered.canonical_url)
                                    .map_err(CoreError::Adapter)?;
                                if seen_urls.insert(canonical_url) {
                                    queue.push_back(discovered);
                                }
                            }
                        }
                        let ai_output = if request.ai_assist {
                            if let Some(ai_request) = adapter
                                .ai_request(request, &raw, &parsed)
                                .map_err(|error| {
                                    CoreError::Adapter(format!("{}: {}", error.code, error.message))
                                })?
                            {
                                Some(ai.run(database, ai_request)?.candidate.proposed)
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        for observation in parsed.observations {
                            let _observation_id = database.upsert_source_observation(
                                &stored.artifact_id,
                                source_id,
                                &spec.adapter_id,
                                &observation,
                            )?;
                            observations = observations.saturating_add(1);
                            for normalized in adapter
                                .normalize(request, &observation, ai_output.as_ref())
                                .map_err(|error| {
                                    CoreError::Adapter(format!("{}: {}", error.code, error.message))
                                })?
                            {
                                let comparison = database.compare_ingestion_candidate(
                                    normalized.target_entity_type.as_deref(),
                                    normalized.target_entity_id.as_deref(),
                                    &normalized.proposed,
                                )?;
                                let content_hash = sha256_json(&normalized.proposed)?;
                                let candidate_id = format!(
                                    "candidate:{}:{}",
                                    spec.adapter_id, normalized.stable_key
                                );
                                let mut evidence = normalized.evidence;
                                let fetched_at = database.current_timestamp()?;
                                evidence.push(json!({
                                    "url": raw.canonical_url.clone(),
                                    "finalUrl": raw.final_url.clone(),
                                    "artifactId": stored.artifact_id.clone(),
                                    "contentHash": stored.content_hash.clone(),
                                    "fetchedAt": fetched_at.clone(),
                                    "httpStatus": raw.http_status,
                                    "acquisitionMethod": raw.acquisition_method.clone(),
                                }));
                                let mut provenance = normalized.provenance;
                                provenance.push(json!({
                                    "sourceId": source_id,
                                    "adapterId": spec.adapter_id,
                                    "artifactId": stored.artifact_id.clone(),
                                    "originalUrl": raw.canonical_url.clone(),
                                    "finalUrl": raw.final_url.clone(),
                                    "fetchedAt": fetched_at,
                                }));
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
                                    evidence,
                                    provenance,
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
                        database.upsert_ingestion_checkpoint(
                            &spec.adapter_id,
                            source_id,
                            checkpoint_key,
                            item.pagination_cursor.as_deref().or(page_cursor.as_deref()),
                            last_artifact_hash.as_deref(),
                            &database.current_timestamp()?,
                        )?;
                    }
                }
            }
            if page_cursor.is_none() {
                has_page = false;
                break;
            }
        }
        database.upsert_ingestion_checkpoint(
            &spec.adapter_id,
            source_id,
            checkpoint_key,
            if !has_page {
                Some("complete")
            } else {
                page_cursor.as_deref()
            },
            last_artifact_hash.as_deref(),
            &database.current_timestamp()?,
        )?;
        let run = database.adapter_run(run_id)?.ok_or_else(|| {
            CoreError::Adapter("adapter run disappeared before completion".to_string())
        })?;
        Ok(PipelineResult {
            run,
            attempts,
            artifact_count: artifacts,
            observation_count: observations,
            candidate_count: candidates,
            changed_count: changed,
        })
    }
}

struct PipelineResult {
    run: AdapterRunRead,
    attempts: u32,
    artifact_count: u32,
    observation_count: u32,
    candidate_count: u32,
    changed_count: u32,
}

fn fetch_with_retries(
    adapter: &dyn SourceAdapter,
    request: &AdapterRunRequest,
    item: &DiscoveryItem,
    context: &FetchContext,
    http: &HttpAcquisitionClient,
    spec: &AdapterSpec,
) -> Result<(FetchOutcome, u32), AdapterError> {
    let max_attempts = spec.retry_policy.max_attempts.max(1);
    let mut last_error = None;
    for attempt in 0..max_attempts {
        match adapter.fetch(request, item, context, http) {
            Ok(outcome) => return Ok((outcome, u32::from(attempt) + 1)),
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
                let delay_ms = last_error
                    .as_ref()
                    .and_then(|error| error.retry_after_ms)
                    .filter(|delay| *delay > 0)
                    .unwrap_or_else(|| {
                        spec.retry_policy
                            .backoff_ms
                            .get(attempt as usize)
                            .copied()
                            .unwrap_or_default()
                    });
                if delay_ms > 0 && !request.fixture_mode {
                    thread::sleep(Duration::from_millis(delay_ms));
                }
            }
        }
    }
    Err(last_error.unwrap_or_else(|| AdapterError::new("UNAVAILABLE", "adapter fetch failed")))
}

fn canonicalize_url(raw: &str) -> Result<String, String> {
    let mut url = Url::parse(raw).map_err(|error| error.to_string())?;
    url.set_fragment(None);
    if url.path().len() > 1 && url.path().ends_with('/') {
        let trimmed = url.path().trim_end_matches('/').to_string();
        url.set_path(&trimmed);
    }
    Ok(url.to_string())
}

fn host_of(raw: &str) -> Option<String> {
    Url::parse(raw)
        .ok()
        .and_then(|url| url.host_str().map(str::to_ascii_lowercase))
}

fn url_in_scope(raw: &str, scope: &AdapterScopeSpec) -> bool {
    let Some(host) = host_of(raw) else {
        return false;
    };
    if scope.allowed_domains.is_empty() {
        return true;
    }
    scope.allowed_domains.iter().any(|allowed| {
        let allowed = allowed.to_ascii_lowercase();
        host == allowed || (!scope.same_origin_only && host.ends_with(&format!(".{allowed}")))
    })
}

fn header_content(value: &str, name: &str) -> Option<String> {
    let lower = value.to_ascii_lowercase();
    let marker = format!("{name}=");
    let start = lower.find(&marker)? + marker.len();
    let rest = &value[start..];
    let quote = rest.chars().next()?;
    let rest = if quote == '"' || quote == '\'' {
        &rest[1..]
    } else {
        rest
    };
    let end = rest
        .find(|character: char| character == quote || character.is_whitespace() || character == '>')
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

fn strip_html(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut in_tag = false;
    for character in value.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => output.push(character),
            _ => {}
        }
    }
    output.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_html_metadata(html: &str, source_url: &str) -> Result<Value, AdapterError> {
    let mut output = serde_json::Map::new();
    let canonical_url = html
        .split("<link")
        .skip(1)
        .filter_map(|tag| tag.split('>').next())
        .find(|tag| {
            tag.to_ascii_lowercase().contains("rel=\"canonical\"")
                || tag.to_ascii_lowercase().contains("rel='canonical'")
        })
        .and_then(|tag| header_content(tag, "href"))
        .unwrap_or_else(|| source_url.to_string());
    output.insert("originalUrl".to_string(), Value::String(canonical_url));
    for (field, key) in [
        ("title", "og:title"),
        ("description", "og:description"),
        ("language", "og:locale"),
    ] {
        if let Some(value) = html
            .split("<meta")
            .skip(1)
            .filter_map(|tag| tag.split('>').next())
            .find(|tag| {
                let lower = tag.to_ascii_lowercase();
                lower.contains(&format!("property=\"{key}\""))
                    || lower.contains(&format!("property='{key}'"))
                    || lower.contains(&format!("name=\"{key}\""))
                    || lower.contains(&format!("name='{key}'"))
            })
            .and_then(|tag| header_content(tag, "content"))
        {
            output.insert(field.to_string(), Value::String(value));
        }
    }
    let lower = html.to_ascii_lowercase();
    if let Some(start) = lower.find("<script") {
        let scripts = &html[start..];
        if let Some(script_start) = scripts.find('>') {
            if let Some(script_end) = scripts[script_start + 1..]
                .to_ascii_lowercase()
                .find("</script>")
            {
                let body = &scripts[script_start + 1..script_start + 1 + script_end];
                if let Ok(json_ld) = serde_json::from_str::<Value>(body.trim()) {
                    if let Some(headline) = json_ld.get("headline").and_then(Value::as_str) {
                        output.insert("title".to_string(), Value::String(headline.to_string()));
                    }
                    if let Some(date) = json_ld.get("datePublished").and_then(Value::as_str) {
                        output.insert("publishedAt".to_string(), Value::String(date.to_string()));
                    }
                    if let Some(author) = json_ld
                        .get("author")
                        .and_then(|value| value.get("name"))
                        .and_then(Value::as_str)
                    {
                        output.insert("author".to_string(), Value::String(author.to_string()));
                    }
                }
            }
        }
    }
    let body_text = strip_html(html);
    if !body_text.is_empty() {
        output.insert("bodyText".to_string(), Value::String(body_text));
    }
    output.insert(
        "access".to_string(),
        Value::String("metadata_only".to_string()),
    );
    output.insert(
        "rightsNote".to_string(),
        Value::String("Only public metadata and the original URL are retained.".to_string()),
    );
    Ok(Value::Object(output))
}

fn discover_html_links(html: &str, source_url: &str, depth: u32) -> Vec<DiscoveryItem> {
    let Ok(base) = Url::parse(source_url) else {
        return Vec::new();
    };
    html.split("<a")
        .skip(1)
        .filter_map(|tag| tag.split('>').next())
        .filter_map(|tag| header_content(tag, "href"))
        .filter_map(|href| base.join(&href).ok())
        .filter_map(|url| canonicalize_url(url.as_str()).ok())
        .filter(|url| url != source_url)
        .map(|url| DiscoveryItem {
            discovery_key: format!("html:{url}"),
            canonical_url: url,
            content_type: "text/html".to_string(),
            external_key: None,
            acquisition_method: ACQUISITION_HTML.to_string(),
            strategy_id: "static-html-link".to_string(),
            depth: depth.saturating_add(1),
            parent_url: Some(source_url.to_string()),
            referrer_url: Some(source_url.to_string()),
            pagination_cursor: None,
        })
        .collect()
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
                CAPABILITY_FETCH_API.to_string(),
                CAPABILITY_FETCH_JSON.to_string(),
                CAPABILITY_PAGINATE.to_string(),
                CAPABILITY_CRAWL.to_string(),
                CAPABILITY_CHECKPOINT.to_string(),
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
            acquisition_strategies: vec![
                AcquisitionStrategySpec {
                    id: "cnmi-api-json".to_string(),
                    method: ACQUISITION_API.to_string(),
                    priority: 1,
                    deterministic: true,
                    requires_browser: false,
                },
                AcquisitionStrategySpec {
                    id: "cnmi-official-html".to_string(),
                    method: ACQUISITION_HTML.to_string(),
                    priority: 2,
                    deterministic: true,
                    requires_browser: false,
                },
            ],
            scope: AdapterScopeSpec {
                allowed_domains: vec!["milanofashionweek.cameramoda.it".to_string()],
                same_origin_only: true,
            },
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

    fn discover(&self, _request: &AdapterRunRequest) -> Result<DiscoveryBatch, AdapterError> {
        Ok(DiscoveryBatch {
            items: vec![DiscoveryItem {
                discovery_key: "cnmi:milano:ss27:calendar".to_string(),
                canonical_url: "https://milanofashionweek.cameramoda.it/en/calendar".to_string(),
                content_type: "application/json".to_string(),
                external_key: Some("edition:milano-fashion-week:ss27:2026".to_string()),
                acquisition_method: ACQUISITION_API.to_string(),
                strategy_id: "cnmi-api-json".to_string(),
                depth: 0,
                parent_url: None,
                referrer_url: None,
                pagination_cursor: None,
            }],
            next_cursor: None,
        })
    }

    fn fetch(
        &self,
        request: &AdapterRunRequest,
        item: &DiscoveryItem,
        context: &FetchContext,
        http: &HttpAcquisitionClient,
    ) -> Result<FetchOutcome, AdapterError> {
        if !request.fixture_mode {
            return http.fetch(item, context, &["application/json".to_string()]);
        }
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
        Ok(FetchOutcome::Acquired(RawArtifactInput {
            source_id: SOURCE_CNMI.to_string(),
            integration_id: "integration:cnmi:milano-calendar".to_string(),
            canonical_url: item.canonical_url.clone(),
            content_type: item.content_type.clone(),
            content,
            content_ref: Some("pack:nex.fashion.milano.ss27/schedule".to_string()),
            etag: Some("\"cnmi-fixture-v1\"".to_string()),
            last_modified: Some("Sat, 03 Oct 2026 00:00:00 GMT".to_string()),
            final_url: Some(item.canonical_url.clone()),
            acquisition_method: ACQUISITION_API.to_string(),
            http_status: Some(200),
            parent_url: item.parent_url.clone(),
            referrer_url: item.referrer_url.clone(),
            pagination: json!({"cursor": item.pagination_cursor, "depth": item.depth}),
            retrieval_status: "acquired".to_string(),
        }))
    }

    fn parse(&self, artifact: &RawArtifactInput) -> Result<ParseResult, AdapterError> {
        let parsed: CnmiCalendarArtifact = serde_json::from_str(&artifact.content)
            .map_err(|error| AdapterError::new("PARSE_FAILED", error.to_string()))?;
        if parsed.entries.is_empty() {
            return Err(AdapterError::new(
                "EMPTY_SOURCE",
                "CNMI calendar contained no entries",
            ));
        }
        let observations = parsed
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
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ParseResult {
            observations,
            discoveries: Vec::new(),
        })
    }

    fn normalize(
        &self,
        _request: &AdapterRunRequest,
        observation: &ObservationInput,
        _ai_output: Option<&Value>,
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
                CAPABILITY_FETCH_HTML.to_string(),
                CAPABILITY_CRAWL.to_string(),
                CAPABILITY_CHECKPOINT.to_string(),
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
            acquisition_strategies: vec![
                AcquisitionStrategySpec {
                    id: "bof-static-html".to_string(),
                    method: ACQUISITION_HTML.to_string(),
                    priority: 1,
                    deterministic: true,
                    requires_browser: false,
                },
                AcquisitionStrategySpec {
                    id: "bof-browser-fallback".to_string(),
                    method: ACQUISITION_BROWSER.to_string(),
                    priority: 2,
                    deterministic: false,
                    requires_browser: true,
                },
            ],
            scope: AdapterScopeSpec {
                allowed_domains: vec!["businessoffashion.com".to_string()],
                same_origin_only: false,
            },
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

    fn discover(&self, _request: &AdapterRunRequest) -> Result<DiscoveryBatch, AdapterError> {
        Ok(DiscoveryBatch {
            items: vec![DiscoveryItem {
            discovery_key: "bof:milano:ss27:review:prada-thom-browne-ralph-lauren".to_string(),
            canonical_url: "https://www.businessoffashion.com/reviews/fashion-week/prada-thom-browne-ralph-lauren-menswear-spring-summer-2027/".to_string(),
            content_type: "text/html".to_string(),
            external_key: Some("review:bof:prada-thom-browne-ralph-lauren-ss27".to_string()),
            acquisition_method: ACQUISITION_HTML.to_string(),
            strategy_id: "bof-static-html".to_string(),
            depth: 0,
            parent_url: None,
            referrer_url: None,
            pagination_cursor: None,
        }],
            next_cursor: None,
        })
    }

    fn fetch(
        &self,
        request: &AdapterRunRequest,
        item: &DiscoveryItem,
        context: &FetchContext,
        http: &HttpAcquisitionClient,
    ) -> Result<FetchOutcome, AdapterError> {
        if !request.fixture_mode {
            return http.fetch(item, context, &["text/html".to_string()]);
        }
        let content = format!(
            "<!doctype html><html><head><link rel=\"canonical\" href=\"{url}\"><meta property=\"og:title\" content=\"Prada, Thom Browne, Ralph Lauren Menswear Spring/Summer 2027\"><meta property=\"og:locale\" content=\"en_US\"><script type=\"application/ld+json\">{{\"@type\":\"NewsArticle\",\"headline\":\"Prada, Thom Browne, Ralph Lauren Menswear Spring/Summer 2027\",\"datePublished\":\"2026-09-30\",\"author\":{{\"name\":\"Business of Fashion\"}}}}</script></head><body>Metadata only. Subscription-limited article.</body></html>",
            url = item.canonical_url
        );
        Ok(FetchOutcome::Acquired(RawArtifactInput {
            source_id: SOURCE_BOF.to_string(),
            integration_id: "integration:bof:review-page".to_string(),
            canonical_url: item.canonical_url.clone(),
            content_type: item.content_type.clone(),
            content,
            content_ref: Some("fixture:bof-review-html".to_string()),
            etag: Some("\"bof-fixture-v1\"".to_string()),
            last_modified: Some("Sat, 03 Oct 2026 00:00:00 GMT".to_string()),
            final_url: Some(item.canonical_url.clone()),
            acquisition_method: ACQUISITION_HTML.to_string(),
            http_status: Some(200),
            parent_url: item.parent_url.clone(),
            referrer_url: item.referrer_url.clone(),
            pagination: json!({"cursor": item.pagination_cursor, "depth": item.depth}),
            retrieval_status: "acquired".to_string(),
        }))
    }

    fn parse(&self, artifact: &RawArtifactInput) -> Result<ParseResult, AdapterError> {
        let value = parse_html_metadata(&artifact.content, &artifact.canonical_url)?;
        let url = value
            .get("originalUrl")
            .and_then(Value::as_str)
            .ok_or_else(|| AdapterError::new("PARSE_FAILED", "review metadata lacks originalUrl"))?
            .to_string();
        Ok(ParseResult {
            observations: vec![ObservationInput {
                entity_type: "review".to_string(),
                stable_key: format!("review:{url}"),
                payload: value,
                source_url: url,
            }],
            discoveries: discover_html_links(
                &artifact.content,
                &artifact.canonical_url,
                artifact
                    .pagination
                    .get("depth")
                    .and_then(Value::as_u64)
                    .unwrap_or_default() as u32,
            ),
        })
    }

    fn normalize(
        &self,
        _request: &AdapterRunRequest,
        observation: &ObservationInput,
        _ai_output: Option<&Value>,
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
    use std::io::Write;
    use std::net::TcpListener;

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

    fn test_request(adapter_id: &str, source_id: &str, capability: &str) -> AdapterRunRequest {
        AdapterRunRequest {
            adapter_id: adapter_id.to_string(),
            source_id: Some(source_id.to_string()),
            capability: Some(capability.to_string()),
            cursor: None,
            checkpoint_key: None,
            fixture_mode: true,
            max_items: None,
            max_pages: 100,
            max_depth: 2,
            max_bytes: 5 * 1024 * 1024,
            max_duration_ms: 120_000,
            resume: true,
            ai_assist: false,
            cancel_key: None,
        }
    }

    fn raw_input(url: &str, content: &str) -> RawArtifactInput {
        RawArtifactInput {
            source_id: SOURCE_CNMI.to_string(),
            integration_id: "integration:test".to_string(),
            canonical_url: url.to_string(),
            content_type: "application/json".to_string(),
            content: content.to_string(),
            content_ref: Some("fixture:test".to_string()),
            etag: Some("\"test-v1\"".to_string()),
            last_modified: Some("Sat, 03 Oct 2026 00:00:00 GMT".to_string()),
            final_url: Some(url.to_string()),
            acquisition_method: ACQUISITION_API.to_string(),
            http_status: Some(200),
            parent_url: None,
            referrer_url: None,
            pagination: json!({"cursor": null, "depth": 0}),
            retrieval_status: "acquired".to_string(),
        }
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
    fn strategy_selection_prefers_deterministic_static_acquisition_over_browser() {
        let spec = AdapterRegistry::built_in()
            .specs()
            .into_iter()
            .find(|spec| spec.adapter_id == ADAPTER_BOF_REVIEWS)
            .unwrap();
        let selected =
            select_acquisition_strategy(&spec, &[ACQUISITION_BROWSER, ACQUISITION_HTML]).unwrap();
        assert_eq!(selected.method, ACQUISITION_HTML);
        let browser_only = select_acquisition_strategy(&spec, &[ACQUISITION_BROWSER]).unwrap();
        assert!(browser_only.requires_browser);
    }

    #[test]
    fn retry_policy_retries_only_retryable_fetch_failures() {
        struct FlakyAdapter {
            attempts: Mutex<u32>,
        }

        impl SourceAdapter for FlakyAdapter {
            fn spec(&self) -> AdapterSpec {
                AdapterSpec {
                    contract_version: ADAPTER_CONTRACT_VERSION,
                    adapter_id: "adapter:test:flaky".to_string(),
                    version: "1.0.0".to_string(),
                    source_ids: vec!["source:test".to_string()],
                    integration_ids: vec![],
                    capabilities: vec![CAPABILITY_FETCH_API.to_string()],
                    discovery_strategy: "test".to_string(),
                    supported_content_types: vec!["application/json".to_string()],
                    fetch_strategy: "test".to_string(),
                    parse_strategy: "test".to_string(),
                    normalization_strategy: "test".to_string(),
                    pagination: AdapterPaginationSpec {
                        supported: false,
                        strategy: "none".to_string(),
                        cursor_kind: None,
                        checkpoint_key: None,
                    },
                    rate_limit: AdapterRateLimitSpec {
                        requests_per_minute: None,
                        burst: None,
                        retry_after_header: None,
                        notes: None,
                    },
                    retry_policy: AdapterRetrySpec {
                        max_attempts: 3,
                        backoff_ms: vec![0, 0],
                        retryable_states: vec!["UNAVAILABLE".to_string()],
                    },
                    provenance_support: true,
                    produces: vec![],
                    fixture_support: true,
                    test_support: true,
                    acquisition_strategies: vec![],
                    scope: AdapterScopeSpec {
                        allowed_domains: vec![],
                        same_origin_only: true,
                    },
                }
            }

            fn health(&self) -> AdapterHealth {
                AdapterHealth {
                    adapter_id: "adapter:test:flaky".to_string(),
                    state: AdapterHealthState::Healthy,
                    detail: None,
                    checked_at: None,
                }
            }

            fn discover(
                &self,
                _request: &AdapterRunRequest,
            ) -> Result<DiscoveryBatch, AdapterError> {
                Ok(DiscoveryBatch {
                    items: Vec::new(),
                    next_cursor: None,
                })
            }

            fn fetch(
                &self,
                _request: &AdapterRunRequest,
                _item: &DiscoveryItem,
                _context: &FetchContext,
                _http: &HttpAcquisitionClient,
            ) -> Result<FetchOutcome, AdapterError> {
                let mut attempts = self.attempts.lock().unwrap();
                *attempts += 1;
                if *attempts < 3 {
                    return Err(AdapterError::new("UNAVAILABLE", "retry me"));
                }
                Ok(FetchOutcome::Acquired(raw_input(
                    "https://example.invalid/flaky",
                    "{}",
                )))
            }

            fn parse(&self, _artifact: &RawArtifactInput) -> Result<ParseResult, AdapterError> {
                Ok(ParseResult {
                    observations: Vec::new(),
                    discoveries: Vec::new(),
                })
            }

            fn normalize(
                &self,
                _request: &AdapterRunRequest,
                _observation: &ObservationInput,
                _ai_output: Option<&Value>,
            ) -> Result<Vec<NormalizedCandidate>, AdapterError> {
                Ok(Vec::new())
            }
        }

        let adapter = FlakyAdapter {
            attempts: Mutex::new(0),
        };
        let request = test_request("adapter:test:flaky", "source:test", CAPABILITY_FETCH_API);
        let item = DiscoveryItem {
            discovery_key: "test:flaky".to_string(),
            canonical_url: "https://example.invalid/flaky".to_string(),
            content_type: "application/json".to_string(),
            external_key: None,
            acquisition_method: ACQUISITION_API.to_string(),
            strategy_id: "test".to_string(),
            depth: 0,
            parent_url: None,
            referrer_url: None,
            pagination_cursor: None,
        };
        let spec = adapter.spec();
        let (_, attempts) = fetch_with_retries(
            &adapter,
            &request,
            &item,
            &FetchContext {
                etag: None,
                last_modified: None,
                max_bytes: 1_024,
                timeout_ms: 5_000,
            },
            &HttpAcquisitionClient::new(),
            &spec,
        )
        .unwrap();
        assert_eq!(attempts, 3);
    }

    #[test]
    fn unsupported_capability_is_rejected_without_running_adapter() {
        let registry = AdapterRegistry::built_in();
        let (mut database, _runtime, root) = installed_database();
        let mut request = test_request(
            ADAPTER_CNMI_MILANO,
            SOURCE_CNMI,
            "FUTURE_UNREGISTERED_CAPABILITY",
        );
        request.source_id = None;
        let error = registry
            .run(&mut database, &AiRouter::from_env(), request)
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
                &AiRouter::from_env(),
                test_request(ADAPTER_CNMI_MILANO, SOURCE_CNMI, CAPABILITY_DISCOVER),
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
        let candidate = database
            .ingestion_candidates(Some(ADAPTER_CNMI_MILANO), Some("open"), 1)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        assert!(candidate
            .evidence
            .iter()
            .any(|item| item.get("artifactId").is_some()));
        assert!(candidate
            .provenance
            .iter()
            .any(|item| item.get("originalUrl").is_some()));
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
        let request = test_request(ADAPTER_BOF_REVIEWS, SOURCE_BOF, CAPABILITY_DISCOVER_REVIEWS);
        let ai = AiRouter::from_env();
        let first = registry.run(&mut database, &ai, request.clone()).unwrap();
        let second = registry.run(&mut database, &ai, request).unwrap();
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
        let artifact = database
            .raw_artifacts(Some(ADAPTER_BOF_REVIEWS), 10)
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(artifact.content_type, "text/html");
        assert_eq!(artifact.acquisition_method, ACQUISITION_HTML);
        assert_eq!(artifact.http_status, Some(200));
        assert!(artifact.final_url.is_some());
        assert_eq!(database.official_review_count().unwrap(), 0);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn http_acquisition_sends_validators_and_handles_304() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let mut requests = Vec::new();
            for index in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                let mut buffer = [0u8; 4096];
                let size = stream.read(&mut buffer).unwrap();
                requests.push(String::from_utf8_lossy(&buffer[..size]).to_string());
                let response = if index == 0 {
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nETag: \"v1\"\r\nLast-Modified: Sat, 03 Oct 2026 00:00:00 GMT\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}".to_string()
                } else {
                    "HTTP/1.1 304 Not Modified\r\nETag: \"v1\"\r\nLast-Modified: Sat, 03 Oct 2026 00:00:00 GMT\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
                };
                stream.write_all(response.as_bytes()).unwrap();
            }
            requests
        });
        let client = HttpAcquisitionClient::new();
        let item = DiscoveryItem {
            discovery_key: "test:conditional".to_string(),
            canonical_url: format!("http://{address}/source/#fragment"),
            content_type: "application/json".to_string(),
            external_key: None,
            acquisition_method: ACQUISITION_API.to_string(),
            strategy_id: "test-api".to_string(),
            depth: 0,
            parent_url: None,
            referrer_url: None,
            pagination_cursor: None,
        };
        let first = client
            .fetch(
                &item,
                &FetchContext {
                    etag: None,
                    last_modified: None,
                    max_bytes: 1_024,
                    timeout_ms: 5_000,
                },
                &["application/json".to_string()],
            )
            .unwrap();
        let FetchOutcome::Acquired(first) = first else {
            panic!("first request should acquire a body");
        };
        assert_eq!(first.http_status, Some(200));
        assert_eq!(first.etag.as_deref(), Some("\"v1\""));
        let second = client
            .fetch(
                &item,
                &FetchContext {
                    etag: first.etag,
                    last_modified: first.last_modified,
                    max_bytes: 1_024,
                    timeout_ms: 5_000,
                },
                &["application/json".to_string()],
            )
            .unwrap();
        assert!(matches!(second, FetchOutcome::NotModified(_)));
        let requests = server.join().unwrap();
        let second_request = requests[1].to_ascii_lowercase();
        assert!(second_request.contains("if-none-match: \"v1\""));
        assert!(second_request.contains("if-modified-since: sat, 03 oct 2026 00:00:00 gmt"));
    }

    #[test]
    fn http_acquisition_rejects_bodies_above_the_configured_limit() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let body = "0123456789abcdef";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(), body
            );
            let mut request = [0u8; 1024];
            let _ = stream.read(&mut request);
            stream.write_all(response.as_bytes()).unwrap();
        });
        let item = DiscoveryItem {
            discovery_key: "test:large".to_string(),
            canonical_url: format!("http://{address}/large"),
            content_type: "application/json".to_string(),
            external_key: None,
            acquisition_method: ACQUISITION_API.to_string(),
            strategy_id: "test-api".to_string(),
            depth: 0,
            parent_url: None,
            referrer_url: None,
            pagination_cursor: None,
        };
        let error = HttpAcquisitionClient::new()
            .fetch(
                &item,
                &FetchContext {
                    etag: None,
                    last_modified: None,
                    max_bytes: 4,
                    timeout_ms: 5_000,
                },
                &["application/json".to_string()],
            )
            .unwrap_err();
        assert_eq!(error.code, "RESPONSE_TOO_LARGE");
        server.join().unwrap();
    }

    #[test]
    fn canonical_urls_and_scope_rules_are_deterministic() {
        assert_eq!(
            canonicalize_url("https://EXAMPLE.invalid/path/#fragment").unwrap(),
            "https://example.invalid/path"
        );
        let scope = AdapterScopeSpec {
            allowed_domains: vec!["example.invalid".to_string()],
            same_origin_only: false,
        };
        assert!(url_in_scope("https://cdn.example.invalid/image", &scope));
        assert!(!url_in_scope("https://other.invalid/image", &scope));
    }

    #[test]
    fn raw_body_is_deduplicated_and_304_updates_retrieval_state() {
        let mut database = CatalogDb::in_memory().unwrap();
        let first = raw_input("https://example.invalid/a", "{\"same\":true}");
        let second = raw_input("https://example.invalid/b", "{\"same\":true}");
        let stored_first = database
            .upsert_raw_artifact(&first, ADAPTER_CNMI_MILANO)
            .unwrap();
        database
            .upsert_raw_artifact(&second, ADAPTER_CNMI_MILANO)
            .unwrap();
        let artifacts = database
            .raw_artifacts(Some(ADAPTER_CNMI_MILANO), 10)
            .unwrap();
        assert_eq!(artifacts.len(), 2);
        assert!(artifacts
            .iter()
            .all(|artifact| artifact.storage_kind == "external"));
        let body_ref = format!("body:{}", stored_first.content_hash);
        assert!(artifacts
            .iter()
            .all(|artifact| artifact.content_ref.as_deref() == Some(body_ref.as_str())));
        assert!(database
            .mark_raw_artifact_not_modified(
                ADAPTER_CNMI_MILANO,
                "https://example.invalid/a",
                Some("https://example.invalid/a"),
                304,
                Some("\"test-v2\""),
                None,
                ACQUISITION_API,
                None,
                None,
                &json!({"cursor": "page-2"}),
            )
            .unwrap());
        let updated = database
            .raw_artifacts(Some(ADAPTER_CNMI_MILANO), 10)
            .unwrap();
        assert!(updated
            .iter()
            .any(|artifact| artifact.retrieval_status == "not_modified"
                && artifact.http_status == Some(304)));
    }

    #[test]
    fn cancellation_is_explicit_and_never_promotes_official_data() {
        let registry = AdapterRegistry::built_in();
        let (mut database, _runtime, root) = installed_database();
        registry.cancel("cancel-before-start");
        let mut request = test_request(ADAPTER_CNMI_MILANO, SOURCE_CNMI, CAPABILITY_DISCOVER);
        request.cancel_key = Some("cancel-before-start".to_string());
        let error = registry
            .run(&mut database, &AiRouter::from_env(), request)
            .unwrap_err();
        assert!(matches!(error, CoreError::AdapterCancelled));
        assert_eq!(database.official_schedule_count().unwrap(), 214);
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
