use crate::core::db::CatalogDb;
use crate::core::error::CoreError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub const AI_CONTRACT_VERSION: &str = "1.0";
pub const ALLOWED_COST: &str = "ZERO";
pub const PROVIDER_GEMINI: &str = "gemini";
pub const PROVIDER_GROQ: &str = "groq";
pub const PROVIDER_CLOUDFLARE_WORKERS_AI: &str = "cloudflare_workers_ai";
pub const PROVIDER_LOCAL: &str = "local";
pub const CAPABILITY_STRUCTURED_EXTRACTION: &str = "STRUCTURED_EXTRACTION";
pub const CAPABILITY_SUMMARIZATION: &str = "SUMMARIZATION";
pub const CAPABILITY_TRANSLATION_PT_BR: &str = "TRANSLATION_PT_BR";
pub const CAPABILITY_ENTITY_MATCHING: &str = "ENTITY_MATCHING";
pub const CAPABILITY_TAG_SUGGESTION: &str = "TAG_SUGGESTION";
pub const CAPABILITY_VALIDATION_ASSIST: &str = "VALIDATION_ASSIST";
pub const CAPABILITY_VISION_ANALYSIS: &str = "VISION_ANALYSIS";
pub const CAPABILITY_LOCAL_ONLY: &str = "LOCAL_ONLY";

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiProviderState {
    Available,
    Degraded,
    RateLimited,
    Unavailable,
    NotConfigured,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProviderHealth {
    pub provider_id: String,
    pub model: String,
    pub state: AiProviderState,
    pub zero_cost_eligible: bool,
    pub zero_cost_capabilities: Vec<String>,
    pub capabilities: Vec<String>,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiHealthReport {
    pub contract_version: &'static str,
    pub allowed_cost: &'static str,
    pub router_state: AiProviderState,
    pub providers: Vec<AiProviderHealth>,
    pub capabilities: Vec<String>,
    pub fully_operational: bool,
    pub offline_safe: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiTaskRequest {
    pub task_type: String,
    pub capability: String,
    pub input_text: String,
    #[serde(default)]
    pub target: Option<AiTargetRef>,
    #[serde(default)]
    pub source_ids: Vec<String>,
    #[serde(default)]
    pub evidence_urls: Vec<String>,
    #[serde(default)]
    pub local_only: bool,
    #[serde(default)]
    pub provider_order: Vec<String>,
    #[serde(default)]
    pub max_attempts: Option<u8>,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub output_schema: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiTargetRef {
    pub entity_type: String,
    pub entity_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiExecutionRead {
    pub execution_id: String,
    pub task_type: String,
    pub provider_id: String,
    pub model: String,
    pub capability: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub latency_ms: Option<i64>,
    pub attempt: u8,
    pub fallback_step: u8,
    pub status: String,
    pub validator_schema: String,
    pub validation_result: Value,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub input_hash: String,
    pub usage: Option<Value>,
    pub cost: Option<f64>,
    pub candidate_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiCandidateRead {
    pub candidate_id: String,
    pub proposal_kind: String,
    pub target: Option<AiTargetRef>,
    pub proposed: Value,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub rationale: Option<String>,
    pub confidence: Option<f64>,
    pub execution_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CuratorProposalRead {
    pub proposal_id: String,
    pub candidate_id: String,
    pub proposal_kind: String,
    pub state: String,
    pub target: Option<AiTargetRef>,
    pub proposed: Value,
    pub edited: Option<Value>,
    pub before: Option<Value>,
    pub approved: Option<Value>,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub rationale: Option<String>,
    pub confidence: Option<f64>,
    pub reviewer: Option<String>,
    pub decision_reason: Option<String>,
    pub decided_at: Option<String>,
    pub official_applied: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CuratorProposalDecisionRequest {
    pub proposal_id: String,
    pub decision: String,
    pub reviewer: String,
    #[serde(default)]
    pub edited: Option<Value>,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRunResult {
    pub execution: AiExecutionRead,
    pub candidate: AiCandidateRead,
    pub proposal: CuratorProposalRead,
}

#[derive(Debug, Clone)]
pub struct AiExecutionRecord {
    pub execution_id: String,
    pub task_type: String,
    pub provider_id: String,
    pub model: String,
    pub capability: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub latency_ms: Option<i64>,
    pub attempt: u8,
    pub fallback_step: u8,
    pub status: String,
    pub validator_schema: String,
    pub validation_result: Value,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub input_hash: String,
    pub usage: Option<Value>,
    pub cost: Option<f64>,
    pub candidate_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AiCandidateRecord {
    pub candidate_id: String,
    pub proposal_kind: String,
    pub target: Option<AiTargetRef>,
    pub proposed: Value,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub rationale: Option<String>,
    pub confidence: Option<f64>,
    pub execution_id: String,
}

#[derive(Debug, Clone)]
pub struct CuratorProposalRecord {
    pub proposal_id: String,
    pub candidate_id: String,
    pub proposal_kind: String,
    pub target: Option<AiTargetRef>,
    pub proposed: Value,
    pub evidence: Vec<Value>,
    pub provenance: Vec<Value>,
    pub rationale: Option<String>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone)]
struct ProviderRequest {
    pub input_text: String,
    pub output_schema: Value,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone)]
struct ProviderResponse {
    pub output: Value,
    pub usage: Option<Value>,
    pub cost: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProviderErrorKind {
    NotConfigured,
    PolicyDenied,
    RateLimited,
    Timeout,
    Unavailable,
    InvalidResponse,
    RequestInvalid,
}

#[derive(Debug, Clone)]
struct ProviderError {
    kind: ProviderErrorKind,
    message: String,
}

impl ProviderError {
    fn new(kind: ProviderErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    fn code(self) -> &'static str {
        match self.kind {
            ProviderErrorKind::NotConfigured => "AI_NOT_CONFIGURED",
            ProviderErrorKind::PolicyDenied => "AI_ZERO_COST_POLICY",
            ProviderErrorKind::RateLimited => "AI_RATE_LIMITED",
            ProviderErrorKind::Timeout => "AI_TIMEOUT",
            ProviderErrorKind::Unavailable => "AI_PROVIDER_UNAVAILABLE",
            ProviderErrorKind::InvalidResponse => "AI_INVALID_RESPONSE",
            ProviderErrorKind::RequestInvalid => "AI_REQUEST_INVALID",
        }
    }
}

trait AiProvider: Send + Sync {
    fn health(&self) -> AiProviderHealth;
    fn complete(&self, request: &ProviderRequest) -> Result<ProviderResponse, ProviderError>;
}

#[derive(Debug, Clone)]
struct ZeroCostPolicy {
    model_eligible: bool,
    capabilities: Vec<String>,
}

impl ZeroCostPolicy {
    fn local(capabilities: Vec<String>) -> Self {
        Self {
            model_eligible: true,
            capabilities,
        }
    }
}

struct JsonHttpProvider {
    provider_id: String,
    model: String,
    endpoint: String,
    api_key: Option<String>,
    configured: bool,
    policy: ZeroCostPolicy,
    capabilities: Vec<String>,
    client: Client,
}

impl JsonHttpProvider {
    fn new(
        provider_id: &str,
        model: String,
        endpoint: &str,
        api_key: Option<String>,
        policy: ZeroCostPolicy,
        capabilities: Vec<String>,
        client: Client,
    ) -> Self {
        let configured = api_key.is_some();
        Self::with_configured(
            provider_id,
            model,
            endpoint,
            api_key,
            configured,
            policy,
            capabilities,
            client,
        )
    }

    fn with_configured(
        provider_id: &str,
        model: String,
        endpoint: &str,
        api_key: Option<String>,
        configured: bool,
        policy: ZeroCostPolicy,
        capabilities: Vec<String>,
        client: Client,
    ) -> Self {
        Self {
            provider_id: provider_id.to_string(),
            model,
            endpoint: endpoint.to_string(),
            api_key,
            configured,
            policy,
            capabilities,
            client,
        }
    }
}

impl AiProvider for JsonHttpProvider {
    fn health(&self) -> AiProviderHealth {
        AiProviderHealth {
            provider_id: self.provider_id.clone(),
            model: self.model.clone(),
            state: if !self.configured {
                AiProviderState::NotConfigured
            } else if !self.policy.model_eligible || self.policy.capabilities.is_empty() {
                AiProviderState::Unavailable
            } else {
                AiProviderState::Available
            },
            zero_cost_eligible: self.policy.model_eligible,
            zero_cost_capabilities: self.policy.capabilities.clone(),
            capabilities: self.capabilities.clone(),
            detail: if !self.configured {
                Some("credentials or endpoint are not configured".to_string())
            } else if !self.policy.model_eligible || self.policy.capabilities.is_empty() {
                Some(
                    "model or capability is not explicitly eligible for the ZERO cost policy"
                        .to_string(),
                )
            } else {
                Some("configured; runtime health is confirmed per execution".to_string())
            },
        }
    }

    fn complete(&self, request: &ProviderRequest) -> Result<ProviderResponse, ProviderError> {
        if !self.configured {
            return Err(ProviderError::new(
                ProviderErrorKind::NotConfigured,
                "provider is not configured",
            ));
        }
        let body = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": "You are the NEX+ Curator research assistant. Explore the supplied evidence broadly, return only the requested JSON, preserve uncertainty, and never claim canonical authority."},
                {"role": "user", "content": request.input_text}
            ],
            "temperature": 0.1,
            "response_format": {
                "type": "json_schema",
                "json_schema": {
                    "name": "nex_curator_result",
                    "strict": true,
                    "schema": request.output_schema
                }
            }
        });
        let mut request = self
            .client
            .post(&self.endpoint)
            .json(&body)
            .timeout(Duration::from_millis(request.timeout_ms));
        if let Some(api_key) = self.api_key.as_deref() {
            request = request.bearer_auth(api_key);
        }
        let response = request.send().map_err(classify_reqwest_error)?;
        let status = response.status();
        let body: Value = response.json().map_err(|error| {
            ProviderError::new(
                ProviderErrorKind::InvalidResponse,
                format!("provider response was not JSON: {error}"),
            )
        })?;
        if !status.is_success() {
            return Err(classify_http_status(status.as_u16()));
        }
        let content = body
            .pointer("/choices/0/message/content")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                ProviderError::new(
                    ProviderErrorKind::InvalidResponse,
                    "provider response did not contain message content",
                )
            })?;
        let output = serde_json::from_str(content).map_err(|error| {
            ProviderError::new(
                ProviderErrorKind::InvalidResponse,
                format!("provider content was not structured JSON: {error}"),
            )
        })?;
        Ok(ProviderResponse {
            output,
            usage: body.get("usage").cloned(),
            cost: None,
        })
    }
}

struct GeminiProvider {
    model: String,
    api_key: Option<String>,
    policy: ZeroCostPolicy,
    capabilities: Vec<String>,
    client: Client,
}

impl GeminiProvider {
    fn new(
        model: String,
        api_key: Option<String>,
        policy: ZeroCostPolicy,
        capabilities: Vec<String>,
        client: Client,
    ) -> Self {
        Self {
            model,
            api_key,
            policy,
            capabilities,
            client,
        }
    }
}

impl AiProvider for GeminiProvider {
    fn health(&self) -> AiProviderHealth {
        AiProviderHealth {
            provider_id: "gemini".to_string(),
            model: self.model.clone(),
            state: if self.api_key.is_none() {
                AiProviderState::NotConfigured
            } else if !self.policy.model_eligible || self.policy.capabilities.is_empty() {
                AiProviderState::Unavailable
            } else {
                AiProviderState::Available
            },
            zero_cost_eligible: self.policy.model_eligible,
            zero_cost_capabilities: self.policy.capabilities.clone(),
            capabilities: self.capabilities.clone(),
            detail: if self.api_key.is_none() {
                Some("API key is not configured".to_string())
            } else if !self.policy.model_eligible || self.policy.capabilities.is_empty() {
                Some(
                    "model or capability is not explicitly eligible for the ZERO cost policy"
                        .to_string(),
                )
            } else {
                Some("configured; runtime health is confirmed per execution".to_string())
            },
        }
    }

    fn complete(&self, request: &ProviderRequest) -> Result<ProviderResponse, ProviderError> {
        let Some(api_key) = self.api_key.as_deref() else {
            return Err(ProviderError::new(
                ProviderErrorKind::NotConfigured,
                "provider is not configured",
            ));
        };
        let endpoint = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model
        );
        let body = json!({
            "contents": [{"parts": [{"text": request.input_text}]}],
            "generationConfig": {
                "responseMimeType": "application/json",
                "responseSchema": request.output_schema
            }
        });
        let response = self
            .client
            .post(endpoint)
            .header("x-goog-api-key", api_key)
            .json(&body)
            .timeout(Duration::from_millis(request.timeout_ms))
            .send()
            .map_err(classify_reqwest_error)?;
        let status = response.status();
        let body: Value = response.json().map_err(|error| {
            ProviderError::new(
                ProviderErrorKind::InvalidResponse,
                format!("provider response was not JSON: {error}"),
            )
        })?;
        if !status.is_success() {
            return Err(classify_http_status(status.as_u16()));
        }
        let content = body
            .pointer("/candidates/0/content/parts/0/text")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                ProviderError::new(
                    ProviderErrorKind::InvalidResponse,
                    "provider response did not contain generated text",
                )
            })?;
        let output = serde_json::from_str(content).map_err(|error| {
            ProviderError::new(
                ProviderErrorKind::InvalidResponse,
                format!("provider content was not structured JSON: {error}"),
            )
        })?;
        Ok(ProviderResponse {
            output,
            usage: body.get("usageMetadata").cloned(),
            cost: None,
        })
    }
}

struct CloudflareWorkersAiProvider {
    account_id: Option<String>,
    api_token: Option<String>,
    model: String,
    policy: ZeroCostPolicy,
    capabilities: Vec<String>,
    client: Client,
}

impl CloudflareWorkersAiProvider {
    fn new(
        account_id: Option<String>,
        api_token: Option<String>,
        model: String,
        policy: ZeroCostPolicy,
        capabilities: Vec<String>,
        client: Client,
    ) -> Self {
        Self {
            account_id,
            api_token,
            model,
            policy,
            capabilities,
            client,
        }
    }

    fn configured(&self) -> bool {
        self.account_id.is_some() && self.api_token.is_some()
    }
}

impl AiProvider for CloudflareWorkersAiProvider {
    fn health(&self) -> AiProviderHealth {
        let configured = self.configured();
        AiProviderHealth {
            provider_id: PROVIDER_CLOUDFLARE_WORKERS_AI.to_string(),
            model: self.model.clone(),
            state: if !configured {
                AiProviderState::NotConfigured
            } else if !self.policy.model_eligible || self.policy.capabilities.is_empty() {
                AiProviderState::Unavailable
            } else {
                AiProviderState::Available
            },
            zero_cost_eligible: self.policy.model_eligible,
            zero_cost_capabilities: self.policy.capabilities.clone(),
            capabilities: self.capabilities.clone(),
            detail: if !configured {
                Some("account ID or API token is not configured".to_string())
            } else if !self.policy.model_eligible {
                Some(
                    "model or capability is not explicitly eligible for the ZERO cost policy"
                        .to_string(),
                )
            } else {
                Some("Workers Free eligibility is explicit; paid models are denied".to_string())
            },
        }
    }

    fn complete(&self, request: &ProviderRequest) -> Result<ProviderResponse, ProviderError> {
        if !self.configured() {
            return Err(ProviderError::new(
                ProviderErrorKind::NotConfigured,
                "Cloudflare Workers AI is not configured",
            ));
        }
        if !self.policy.model_eligible {
            return Err(ProviderError::new(
                ProviderErrorKind::PolicyDenied,
                "Cloudflare model is not eligible for the ZERO cost policy",
            ));
        }
        let endpoint = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
            self.account_id.as_deref().unwrap_or_default(),
            self.model
        );
        let response = self
            .client
            .post(endpoint)
            .bearer_auth(self.api_token.as_deref().unwrap_or_default())
            .json(&json!({
                "prompt": format!(
                    "Return only JSON matching this schema: {}\\n\\nEvidence/task:\\n{}",
                    request.output_schema, request.input_text
                )
            }))
            .timeout(Duration::from_millis(request.timeout_ms))
            .send()
            .map_err(classify_reqwest_error)?;
        let status = response.status();
        let body: Value = response.json().map_err(|error| {
            ProviderError::new(
                ProviderErrorKind::InvalidResponse,
                format!("Cloudflare response was not JSON: {error}"),
            )
        })?;
        if status.as_u16() == 403 && body.to_string().contains("5035") {
            return Err(ProviderError::new(
                ProviderErrorKind::PolicyDenied,
                "Cloudflare model requires Workers Paid and was rejected",
            ));
        }
        if !status.is_success() {
            return Err(classify_http_status(status.as_u16()));
        }
        let response_value = body
            .pointer("/result/response")
            .cloned()
            .or_else(|| body.get("result").cloned())
            .ok_or_else(|| {
                ProviderError::new(
                    ProviderErrorKind::InvalidResponse,
                    "Cloudflare response did not contain result.response",
                )
            })?;
        let output = match response_value {
            Value::String(content) => serde_json::from_str(&content).map_err(|error| {
                ProviderError::new(
                    ProviderErrorKind::InvalidResponse,
                    format!("Cloudflare response was not structured JSON: {error}"),
                )
            })?,
            other => other,
        };
        Ok(ProviderResponse {
            output,
            usage: body.get("usage").cloned(),
            cost: Some(0.0),
        })
    }
}

fn classify_reqwest_error(error: reqwest::Error) -> ProviderError {
    if error.is_timeout() {
        ProviderError::new(ProviderErrorKind::Timeout, "provider request timed out")
    } else {
        ProviderError::new(
            ProviderErrorKind::Unavailable,
            "provider request was unavailable",
        )
    }
}

fn env_value(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn csv_env(name: &str) -> Vec<String> {
    env_value(name)
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}

fn zero_cost_policy(prefix: &str, model: &str) -> ZeroCostPolicy {
    let allowed_cost_name = format!("{prefix}_ALLOWED_COST");
    let models_name = format!("{prefix}_ZERO_COST_MODELS");
    let capabilities_name = format!("{prefix}_ZERO_COST_CAPABILITIES");
    let allowed_cost = env_value(&allowed_cost_name).unwrap_or_default();
    let models = csv_env(&models_name);
    let capabilities = csv_env(&capabilities_name);
    zero_cost_policy_from_values(&allowed_cost, model, &models, capabilities)
}

fn zero_cost_policy_from_values(
    allowed_cost: &str,
    model: &str,
    models: &[String],
    capabilities: Vec<String>,
) -> ZeroCostPolicy {
    ZeroCostPolicy {
        model_eligible: allowed_cost.eq_ignore_ascii_case(ALLOWED_COST)
            && models
                .iter()
                .any(|allowed_model| allowed_model == "*" || allowed_model == model),
        capabilities,
    }
}

fn cloudflare_model_requires_paid(model: &str) -> bool {
    matches!(
        model,
        "@cf/moonshotai/kimi-k2.6"
            | "@cf/moonshotai/kimi-k2.7-code"
            | "@cf/zai-org/glm-5.2"
            | "@cf/zai-org/glm-5.3"
            | "@cf/zai-org/glm-5.3-flash"
            | "@cf/deepseek-ai/deepseek-v4-flash-0731"
            | "@cf/deepseek-ai/deepseek-v4-pro-0813"
    )
}

fn cloudflare_zero_cost_policy(model: &str) -> ZeroCostPolicy {
    let mut policy = zero_cost_policy("NEX_AI_CLOUDFLARE_WORKERS_AI", model);
    if cloudflare_model_requires_paid(model) {
        policy.model_eligible = false;
    }
    policy
}

fn classify_http_status(status: u16) -> ProviderError {
    match status {
        408 | 504 => ProviderError::new(
            ProviderErrorKind::Timeout,
            format!("provider HTTP {status}"),
        ),
        429 => ProviderError::new(
            ProviderErrorKind::RateLimited,
            "provider rate limit reached",
        ),
        400..=499 => ProviderError::new(
            ProviderErrorKind::RequestInvalid,
            format!("provider HTTP {status}"),
        ),
        _ => ProviderError::new(
            ProviderErrorKind::Unavailable,
            format!("provider HTTP {status}"),
        ),
    }
}

pub struct AiRouter {
    providers: Vec<Box<dyn AiProvider>>,
    runtime_state: Mutex<HashMap<String, AiProviderState>>,
}

impl AiRouter {
    pub fn from_env() -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(90))
            .build()
            .unwrap_or_else(|_| Client::new());
        let text_capabilities = vec![
            CAPABILITY_STRUCTURED_EXTRACTION.to_string(),
            CAPABILITY_SUMMARIZATION.to_string(),
            CAPABILITY_TRANSLATION_PT_BR.to_string(),
            CAPABILITY_ENTITY_MATCHING.to_string(),
            CAPABILITY_TAG_SUGGESTION.to_string(),
            CAPABILITY_VALIDATION_ASSIST.to_string(),
        ];
        let vision_capabilities = vec![
            CAPABILITY_VISION_ANALYSIS.to_string(),
            CAPABILITY_STRUCTURED_EXTRACTION.to_string(),
        ];
        let mut gemini_capabilities = text_capabilities.clone();
        gemini_capabilities.extend(vision_capabilities.clone());
        let mut providers: Vec<Box<dyn AiProvider>> = Vec::new();
        let gemini_model = env_or("NEX_AI_GEMINI_MODEL", "gemini-2.5-flash");
        providers.push(Box::new(GeminiProvider::new(
            gemini_model.clone(),
            env_value("NEX_AI_GEMINI_API_KEY"),
            zero_cost_policy("NEX_AI_GEMINI", &gemini_model),
            gemini_capabilities,
            client.clone(),
        )));

        let groq_model = env_or("NEX_AI_GROQ_MODEL", "llama-3.1-8b-instant");
        providers.push(Box::new(JsonHttpProvider::new(
            PROVIDER_GROQ,
            groq_model.clone(),
            "https://api.groq.com/openai/v1/chat/completions",
            env_value("NEX_AI_GROQ_API_KEY"),
            zero_cost_policy("NEX_AI_GROQ", &groq_model),
            text_capabilities.clone(),
            client.clone(),
        )));

        let cloudflare_model = env_or("NEX_AI_CLOUDFLARE_WORKERS_AI_MODEL", "unset");
        providers.push(Box::new(CloudflareWorkersAiProvider::new(
            env_value("NEX_AI_CLOUDFLARE_WORKERS_AI_ACCOUNT_ID"),
            env_value("NEX_AI_CLOUDFLARE_WORKERS_AI_API_TOKEN"),
            cloudflare_model.clone(),
            cloudflare_zero_cost_policy(&cloudflare_model),
            text_capabilities.clone(),
            client.clone(),
        )));

        let local_endpoint = env_or(
            "NEX_AI_LOCAL_ENDPOINT",
            "http://127.0.0.1:8080/v1/chat/completions",
        );
        let local_configured = env_value("NEX_AI_LOCAL_ENDPOINT").is_some();
        let mut local_capabilities = text_capabilities;
        local_capabilities.push(CAPABILITY_LOCAL_ONLY.to_string());
        providers.push(Box::new(JsonHttpProvider::with_configured(
            PROVIDER_LOCAL,
            env_or("NEX_AI_LOCAL_MODEL", "llama.cpp"),
            &local_endpoint,
            env_value("NEX_AI_LOCAL_API_KEY"),
            local_configured,
            ZeroCostPolicy::local(local_capabilities.clone()),
            local_capabilities,
            client,
        )));
        Self {
            providers,
            runtime_state: Mutex::new(HashMap::new()),
        }
    }

    #[cfg(test)]
    fn with_providers(providers: Vec<Box<dyn AiProvider>>) -> Self {
        Self {
            providers,
            runtime_state: Mutex::new(HashMap::new()),
        }
    }

    fn effective_health(&self, provider: &dyn AiProvider) -> AiProviderHealth {
        let mut health = provider.health();
        if let Some(state) = self
            .runtime_state
            .lock()
            .ok()
            .and_then(|states| states.get(&health.provider_id).copied())
        {
            health.state = state;
            health.detail = Some(format!("runtime state: {state:?}"));
        }
        health
    }

    fn set_runtime_state(&self, provider_id: &str, state: AiProviderState) {
        if let Ok(mut states) = self.runtime_state.lock() {
            states.insert(provider_id.to_string(), state);
        }
    }

    pub fn health(&self) -> AiHealthReport {
        let providers = self
            .providers
            .iter()
            .map(|provider| self.effective_health(provider.as_ref()))
            .collect::<Vec<_>>();
        let configured = providers.iter().any(|provider| {
            matches!(
                provider.state,
                AiProviderState::Available | AiProviderState::Degraded
            ) && provider.zero_cost_eligible
                && !provider.zero_cost_capabilities.is_empty()
        });
        let capabilities = vec![
            CAPABILITY_STRUCTURED_EXTRACTION,
            CAPABILITY_SUMMARIZATION,
            CAPABILITY_TRANSLATION_PT_BR,
            CAPABILITY_ENTITY_MATCHING,
            CAPABILITY_TAG_SUGGESTION,
            CAPABILITY_VALIDATION_ASSIST,
            CAPABILITY_VISION_ANALYSIS,
            CAPABILITY_LOCAL_ONLY,
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        AiHealthReport {
            contract_version: AI_CONTRACT_VERSION,
            allowed_cost: ALLOWED_COST,
            router_state: if configured {
                AiProviderState::Available
            } else {
                AiProviderState::Degraded
            },
            providers,
            capabilities,
            fully_operational: configured,
            offline_safe: true,
        }
    }

    pub fn run(
        &self,
        database: &mut CatalogDb,
        request: AiTaskRequest,
    ) -> Result<AiRunResult, CoreError> {
        validate_request(&request)?;
        let started_at = database.current_timestamp()?;
        let input_hash = sha256_string(&request.input_text);
        let schema = request
            .output_schema
            .clone()
            .unwrap_or_else(|| default_output_schema(&request.task_type));
        let provider_ids = self.provider_order(&request);
        let max_attempts = request.max_attempts.unwrap_or(3).clamp(1, 6) as usize;
        let mut attempted = 0usize;
        let mut fallback_step = 0u8;
        let mut last_error = None;
        let mut last_error_code = None;
        let mut configured_seen = false;
        let mut seen = HashSet::new();

        for provider_id in provider_ids {
            if attempted >= max_attempts || !seen.insert(provider_id.clone()) {
                break;
            }
            let Some(provider) = self
                .providers
                .iter()
                .find(|candidate| candidate.health().provider_id == provider_id)
            else {
                continue;
            };
            let health = self.effective_health(provider.as_ref());
            if !matches!(
                health.state,
                AiProviderState::Available | AiProviderState::Degraded
            ) {
                continue;
            }
            if !provider_supports(&health, &request.capability, request.local_only) {
                continue;
            }
            configured_seen = true;
            attempted += 1;
            let attempt = attempted as u8;
            let started = Instant::now();
            let result = provider.complete(&ProviderRequest {
                input_text: request.input_text.clone(),
                output_schema: schema.clone(),
                timeout_ms: request.timeout_ms.unwrap_or(90_000).clamp(1_000, 300_000),
            });
            let latency_ms = started.elapsed().as_millis() as i64;
            match result {
                Ok(response) => {
                    if response.cost.is_some_and(|cost| cost != 0.0) {
                        let error = ProviderError::new(
                            ProviderErrorKind::PolicyDenied,
                            "non-zero provider cost was rejected by the ZERO cost policy",
                        );
                        let error_code = error.clone().code().to_string();
                        let error_message = error.message.clone();
                        last_error = Some(error.message.clone());
                        last_error_code = Some(error.code());
                        database.persist_ai_failure(&AiExecutionRecord {
                            execution_id: new_id("ai-execution"),
                            task_type: request.task_type.clone(),
                            provider_id: provider_id.clone(),
                            model: health.model.clone(),
                            capability: request.capability.clone(),
                            started_at: started_at.clone(),
                            completed_at: Some(database.current_timestamp()?),
                            latency_ms: Some(latency_ms),
                            attempt,
                            fallback_step,
                            status: "failed".to_string(),
                            validator_schema: request.task_type.clone(),
                            validation_result: json!({ "valid": false }),
                            error_code: Some(error_code),
                            error_message: Some(error_message),
                            input_hash: input_hash.clone(),
                            usage: response.usage,
                            cost: None,
                            candidate_id: None,
                        })?;
                        self.set_runtime_state(&provider_id, AiProviderState::Unavailable);
                        continue;
                    }
                    match validate_output(&request, &response.output) {
                        Ok(validation_result) => {
                            let execution_id = new_id("ai-execution");
                            let candidate_id = new_id("ai-candidate");
                            let proposal_id = new_id("proposal");
                            let completed_at = database.current_timestamp()?;
                            let execution = AiExecutionRecord {
                                execution_id: execution_id.clone(),
                                task_type: request.task_type.clone(),
                                provider_id: provider_id.clone(),
                                model: health.model,
                                capability: request.capability.clone(),
                                started_at,
                                completed_at: Some(completed_at),
                                latency_ms: Some(latency_ms),
                                attempt,
                                fallback_step,
                                status: "succeeded".to_string(),
                                validator_schema: request.task_type.clone(),
                                validation_result,
                                error_code: None,
                                error_message: None,
                                input_hash: input_hash.clone(),
                                usage: response.usage,
                                cost: Some(0.0),
                                candidate_id: Some(candidate_id.clone()),
                            };
                            let evidence = request
                                .evidence_urls
                                .iter()
                                .map(|url| json!({ "url": url }))
                                .collect::<Vec<_>>();
                            let provenance = request
                                .source_ids
                                .iter()
                                .map(|source_id| json!({ "sourceId": source_id }))
                                .collect::<Vec<_>>();
                            let candidate = AiCandidateRecord {
                                candidate_id: candidate_id.clone(),
                                proposal_kind: request.task_type.clone(),
                                target: request.target.clone(),
                                proposed: response.output.clone(),
                                evidence: evidence.clone(),
                                provenance: provenance.clone(),
                                rationale: response
                                    .output
                                    .get("rationale")
                                    .and_then(Value::as_str)
                                    .map(str::to_string),
                                confidence: response
                                    .output
                                    .get("confidence")
                                    .and_then(Value::as_f64),
                                execution_id,
                            };
                            let proposal = CuratorProposalRecord {
                                proposal_id,
                                candidate_id,
                                proposal_kind: request.task_type.clone(),
                                target: request.target.clone(),
                                proposed: response.output,
                                evidence,
                                provenance,
                                rationale: candidate.rationale.clone(),
                                confidence: candidate.confidence,
                            };
                            return database.persist_ai_success(&execution, &candidate, &proposal);
                        }
                        Err(error) => {
                            let message = error.to_string();
                            last_error = Some(message.clone());
                            last_error_code = Some("AI_VALIDATION_FAILED");
                            database.persist_ai_failure(&AiExecutionRecord {
                                execution_id: new_id("ai-execution"),
                                task_type: request.task_type.clone(),
                                provider_id: provider_id.clone(),
                                model: health.model,
                                capability: request.capability.clone(),
                                started_at: started_at.clone(),
                                completed_at: Some(database.current_timestamp()?),
                                latency_ms: Some(latency_ms),
                                attempt,
                                fallback_step,
                                status: "failed".to_string(),
                                validator_schema: request.task_type.clone(),
                                validation_result: json!({ "valid": false, "message": message }),
                                error_code: Some("AI_VALIDATION_FAILED".to_string()),
                                error_message: Some(message),
                                input_hash: input_hash.clone(),
                                usage: response.usage,
                                cost: response.cost,
                                candidate_id: None,
                            })?;
                            self.set_runtime_state(&provider_id, AiProviderState::Degraded);
                        }
                    }
                }
                Err(error) => {
                    let error_code = error.clone().code();
                    let code = error_code.to_string();
                    last_error = Some(error.message.clone());
                    last_error_code = Some(error_code);
                    database.persist_ai_failure(&AiExecutionRecord {
                        execution_id: new_id("ai-execution"),
                        task_type: request.task_type.clone(),
                        provider_id: provider_id.clone(),
                        model: health.model,
                        capability: request.capability.clone(),
                        started_at: started_at.clone(),
                        completed_at: Some(database.current_timestamp()?),
                        latency_ms: Some(latency_ms),
                        attempt,
                        fallback_step,
                        status: "failed".to_string(),
                        validator_schema: request.task_type.clone(),
                        validation_result: json!({ "valid": false }),
                        error_code: Some(code),
                        error_message: Some(error.message),
                        input_hash: input_hash.clone(),
                        usage: None,
                        cost: None,
                        candidate_id: None,
                    })?;
                    let runtime_state = match error_code {
                        "AI_RATE_LIMITED" => AiProviderState::RateLimited,
                        "AI_TIMEOUT" | "AI_PROVIDER_UNAVAILABLE" | "AI_ZERO_COST_POLICY" => {
                            AiProviderState::Unavailable
                        }
                        _ => AiProviderState::Degraded,
                    };
                    self.set_runtime_state(&provider_id, runtime_state);
                }
            }
            fallback_step = fallback_step.saturating_add(1);
        }

        if !configured_seen {
            database.persist_ai_failure(&AiExecutionRecord {
                execution_id: new_id("ai-execution"),
                task_type: request.task_type,
                provider_id: "router".to_string(),
                model: "-".to_string(),
                capability: request.capability,
                started_at,
                completed_at: Some(database.current_timestamp()?),
                latency_ms: None,
                attempt: 0,
                fallback_step: 0,
                status: "degraded".to_string(),
                validator_schema: "router".to_string(),
                validation_result: json!({ "valid": false }),
                error_code: Some("AI_NOT_CONFIGURED".to_string()),
                error_message: Some("no configured provider matched this policy".to_string()),
                input_hash,
                usage: None,
                cost: None,
                candidate_id: None,
            })?;
            return Err(CoreError::AiNotConfigured(
                "no configured provider matched this policy".to_string(),
            ));
        }
        if last_error_code == Some("AI_ZERO_COST_POLICY") {
            return Err(CoreError::AiZeroCostPolicy(last_error.unwrap_or_else(
                || "provider was blocked by the ZERO cost policy".to_string(),
            )));
        }
        Err(CoreError::AiProvider {
            provider: "router".to_string(),
            message: last_error.unwrap_or_else(|| "all providers failed".to_string()),
        })
    }

    fn provider_order(&self, request: &AiTaskRequest) -> Vec<String> {
        if request.local_only || request.capability == CAPABILITY_LOCAL_ONLY {
            return vec![PROVIDER_LOCAL.to_string()];
        }
        if !request.provider_order.is_empty() {
            return request.provider_order.clone();
        }
        match request.capability.as_str() {
            CAPABILITY_VISION_ANALYSIS => vec![
                PROVIDER_GEMINI,
                PROVIDER_GROQ,
                PROVIDER_CLOUDFLARE_WORKERS_AI,
                PROVIDER_LOCAL,
            ],
            CAPABILITY_TRANSLATION_PT_BR => vec![
                PROVIDER_GEMINI,
                PROVIDER_GROQ,
                PROVIDER_CLOUDFLARE_WORKERS_AI,
                PROVIDER_LOCAL,
            ],
            CAPABILITY_STRUCTURED_EXTRACTION => vec![
                PROVIDER_GEMINI,
                PROVIDER_GROQ,
                PROVIDER_CLOUDFLARE_WORKERS_AI,
                PROVIDER_LOCAL,
            ],
            _ => vec![
                PROVIDER_GEMINI,
                PROVIDER_GROQ,
                PROVIDER_CLOUDFLARE_WORKERS_AI,
                PROVIDER_LOCAL,
            ],
        }
        .into_iter()
        .map(str::to_string)
        .collect()
    }
}

fn env_or(name: &str, fallback: &str) -> String {
    std::env::var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

fn validate_request(request: &AiTaskRequest) -> Result<(), CoreError> {
    if request.task_type.trim().is_empty() {
        return Err(CoreError::InvalidRequest(
            "taskType is required".to_string(),
        ));
    }
    if request.capability.trim().is_empty() {
        return Err(CoreError::InvalidRequest(
            "capability is required".to_string(),
        ));
    }
    if request.input_text.trim().is_empty() {
        return Err(CoreError::InvalidRequest(
            "inputText is required".to_string(),
        ));
    }
    if request.local_only && request.provider_order.iter().any(|id| id != PROVIDER_LOCAL) {
        return Err(CoreError::AiPolicy(
            "LOCAL_ONLY cannot include cloud providers".to_string(),
        ));
    }
    Ok(())
}

fn provider_supports(health: &AiProviderHealth, capability: &str, local_only: bool) -> bool {
    if local_only || capability == CAPABILITY_LOCAL_ONLY {
        return health.provider_id == PROVIDER_LOCAL && health.zero_cost_eligible;
    }
    health.zero_cost_eligible
        && health.capabilities.iter().any(|item| item == capability)
        && health
            .zero_cost_capabilities
            .iter()
            .any(|item| item == "*" || item == capability)
}

fn default_output_schema(task_type: &str) -> Value {
    if task_type.contains("review") || task_type.contains("summary") {
        json!({
            "type": "object",
            "properties": {
                "summary": {"type": "string"},
                "keyPoints": {"type": "array", "items": {"type": "string"}},
                "translatedText": {"type": "string"},
                "rationale": {"type": "string"},
                "confidence": {"type": "number"}
            },
            "required": ["summary", "keyPoints", "translatedText"],
            "additionalProperties": false
        })
    } else {
        json!({
            "type": "object",
            "properties": {
                "result": {"type": "object"},
                "rationale": {"type": "string"},
                "confidence": {"type": "number"}
            },
            "required": ["result"],
            "additionalProperties": false
        })
    }
}

fn validate_output(request: &AiTaskRequest, output: &Value) -> Result<Value, CoreError> {
    let object = output
        .as_object()
        .ok_or_else(|| CoreError::AiValidation("result must be a JSON object".to_string()))?;
    if request.task_type.contains("review") || request.task_type.contains("summary") {
        let summary = object
            .get("summary")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| CoreError::AiValidation("summary is required".to_string()))?;
        let key_points = object
            .get("keyPoints")
            .and_then(Value::as_array)
            .ok_or_else(|| CoreError::AiValidation("keyPoints must be an array".to_string()))?;
        if key_points.iter().any(|point| point.as_str().is_none()) {
            return Err(CoreError::AiValidation(
                "keyPoints must contain only strings".to_string(),
            ));
        }
        if object
            .get("translatedText")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .is_none()
        {
            return Err(CoreError::AiValidation(
                "translatedText is required for review/summary tasks".to_string(),
            ));
        }
        return Ok(json!({
            "valid": true,
            "schema": request.task_type,
            "summaryLength": summary.chars().count(),
            "keyPointCount": key_points.len()
        }));
    }
    if object.is_empty() {
        return Err(CoreError::AiValidation(
            "result object cannot be empty".to_string(),
        ));
    }
    Ok(json!({ "valid": true, "schema": request.task_type }))
}

fn sha256_string(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
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
    use super::{
        cloudflare_model_requires_paid, zero_cost_policy_from_values, AiProvider, AiProviderHealth,
        AiProviderState, AiRouter, AiTargetRef, AiTaskRequest, CloudflareWorkersAiProvider,
        CuratorProposalDecisionRequest, ProviderError, ProviderErrorKind, ProviderRequest,
        ProviderResponse, ZeroCostPolicy, CAPABILITY_LOCAL_ONLY, CAPABILITY_STRUCTURED_EXTRACTION,
        PROVIDER_CLOUDFLARE_WORKERS_AI, PROVIDER_GEMINI, PROVIDER_GROQ, PROVIDER_LOCAL,
    };
    use crate::core::db::CatalogDb;
    use crate::core::pack::{PackRuntime, BUNDLED_PACK_ID};
    use serde_json::json;

    struct FakeProvider {
        id: &'static str,
        state: AiProviderState,
        zero_cost_eligible: bool,
        outputs: std::sync::Mutex<Vec<Result<ProviderResponse, ProviderError>>>,
    }

    impl AiProvider for FakeProvider {
        fn health(&self) -> AiProviderHealth {
            AiProviderHealth {
                provider_id: self.id.to_string(),
                model: "fake-model".to_string(),
                state: self.state,
                zero_cost_eligible: self.zero_cost_eligible,
                zero_cost_capabilities: vec![
                    CAPABILITY_STRUCTURED_EXTRACTION.to_string(),
                    CAPABILITY_LOCAL_ONLY.to_string(),
                ],
                capabilities: vec![
                    CAPABILITY_STRUCTURED_EXTRACTION.to_string(),
                    CAPABILITY_LOCAL_ONLY.to_string(),
                ],
                detail: None,
            }
        }

        fn complete(&self, _request: &ProviderRequest) -> Result<ProviderResponse, ProviderError> {
            self.outputs.lock().unwrap().remove(0)
        }
    }

    fn review_output() -> ProviderResponse {
        ProviderResponse {
            output: json!({
                "summary": "Resumo para revisão humana.",
                "keyPoints": ["Ponto extraído da evidência."],
                "translatedText": "Texto preparado em PT-BR.",
                "rationale": "A proposta mantém a evidência separada do dado oficial.",
                "confidence": 0.71
            }),
            usage: Some(json!({ "totalTokens": 10 })),
            cost: None,
        }
    }

    fn request(local_only: bool) -> AiTaskRequest {
        AiTaskRequest {
            task_type: "review_summary".to_string(),
            capability: CAPABILITY_STRUCTURED_EXTRACTION.to_string(),
            input_text: "Evidence from Milano SS27 for a human-reviewed summary.".to_string(),
            target: Some(AiTargetRef {
                entity_type: "maison".to_string(),
                entity_id: "maison:prada".to_string(),
            }),
            source_ids: vec!["source:cnmi".to_string()],
            evidence_urls: vec!["https://milanofashionweek.cameramoda.it/en/".to_string()],
            local_only,
            provider_order: if local_only {
                vec!["local".to_string()]
            } else {
                vec!["primary".to_string(), "fallback".to_string()]
            },
            max_attempts: Some(3),
            timeout_ms: Some(5_000),
            output_schema: None,
        }
    }

    fn free_provider(
        id: &'static str,
        output: Result<ProviderResponse, ProviderError>,
    ) -> Box<dyn AiProvider> {
        Box::new(FakeProvider {
            id,
            state: AiProviderState::Available,
            zero_cost_eligible: true,
            outputs: std::sync::Mutex::new(vec![output]),
        })
    }

    #[test]
    fn active_provider_inventory_is_zero_cost_only() {
        let report = AiRouter::from_env().health();
        let ids = report
            .providers
            .iter()
            .map(|provider| provider.provider_id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(report.allowed_cost, "ZERO");
        assert!(ids.contains(&PROVIDER_GEMINI));
        assert!(ids.contains(&PROVIDER_GROQ));
        assert!(ids.contains(&PROVIDER_CLOUDFLARE_WORKERS_AI));
        assert!(ids.contains(&PROVIDER_LOCAL));
        assert!(!ids.contains(&"xai"));
        assert!(!ids.contains(&"openrouter"));
    }

    #[test]
    fn model_and_capability_must_be_explicitly_marked_zero_cost() {
        let models = vec!["gemini-free-model".to_string()];
        let capabilities = vec![CAPABILITY_STRUCTURED_EXTRACTION.to_string()];
        assert!(
            zero_cost_policy_from_values(
                "ZERO",
                "gemini-free-model",
                &models,
                capabilities.clone()
            )
            .model_eligible
        );
        assert!(
            !zero_cost_policy_from_values(
                "PAID",
                "gemini-free-model",
                &models,
                capabilities.clone()
            )
            .model_eligible
        );
        assert!(
            !zero_cost_policy_from_values("ZERO", "unlisted-model", &models, capabilities)
                .model_eligible
        );
    }

    #[test]
    fn free_tier_provider_ids_can_each_be_selected() {
        for provider_id in [
            PROVIDER_GEMINI,
            PROVIDER_GROQ,
            PROVIDER_CLOUDFLARE_WORKERS_AI,
        ] {
            let router =
                AiRouter::with_providers(vec![free_provider(provider_id, Ok(review_output()))]);
            let mut database = CatalogDb::in_memory().unwrap();
            database.seed_bundled_milano().unwrap();
            let mut task = request(false);
            task.provider_order = vec![provider_id.to_string()];
            let result = router.run(&mut database, task).unwrap();
            assert_eq!(result.execution.provider_id, provider_id);
            assert_eq!(result.execution.cost, Some(0.0));
        }
    }

    #[test]
    fn paid_provider_is_skipped_even_when_requested_first() {
        let router = AiRouter::with_providers(vec![
            Box::new(FakeProvider {
                id: "paid-model",
                state: AiProviderState::Available,
                zero_cost_eligible: false,
                outputs: std::sync::Mutex::new(vec![]),
            }),
            free_provider(PROVIDER_GROQ, Ok(review_output())),
        ]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let mut task = request(false);
        task.provider_order = vec!["paid-model".to_string(), PROVIDER_GROQ.to_string()];
        let result = router.run(&mut database, task).unwrap();
        assert_eq!(result.execution.provider_id, PROVIDER_GROQ);
    }

    #[test]
    fn cloudflare_paid_model_is_unavailable_without_network_call() {
        assert!(cloudflare_model_requires_paid("@cf/moonshotai/kimi-k2.6"));
        assert!(!cloudflare_model_requires_paid("@cf/zai-org/glm-4.7-flash"));
        let provider = CloudflareWorkersAiProvider::new(
            Some("account".to_string()),
            Some("token".to_string()),
            "@cf/moonshotai/kimi-k2.6".to_string(),
            ZeroCostPolicy {
                model_eligible: false,
                capabilities: vec![],
            },
            vec![CAPABILITY_STRUCTURED_EXTRACTION.to_string()],
            reqwest::blocking::Client::new(),
        );
        let health = provider.health();
        assert_eq!(health.state, AiProviderState::Unavailable);
        assert!(!health.zero_cost_eligible);

        let free_provider = CloudflareWorkersAiProvider::new(
            Some("account".to_string()),
            Some("token".to_string()),
            "@cf/zai-org/glm-4.7-flash".to_string(),
            ZeroCostPolicy {
                model_eligible: true,
                capabilities: vec![CAPABILITY_STRUCTURED_EXTRACTION.to_string()],
            },
            vec![CAPABILITY_STRUCTURED_EXTRACTION.to_string()],
            reqwest::blocking::Client::new(),
        );
        assert_eq!(free_provider.health().state, AiProviderState::Available);
    }

    #[test]
    fn free_quota_falls_back_to_free_provider_then_local() {
        let router = AiRouter::with_providers(vec![
            free_provider(
                PROVIDER_GEMINI,
                Err(ProviderError::new(
                    ProviderErrorKind::RateLimited,
                    "quota exhausted",
                )),
            ),
            free_provider(
                PROVIDER_GROQ,
                Err(ProviderError::new(
                    ProviderErrorKind::RateLimited,
                    "quota exhausted",
                )),
            ),
            free_provider(PROVIDER_LOCAL, Ok(review_output())),
        ]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let mut task = request(false);
        task.provider_order = vec![
            PROVIDER_GEMINI.to_string(),
            PROVIDER_GROQ.to_string(),
            PROVIDER_LOCAL.to_string(),
        ];
        let result = router.run(&mut database, task).unwrap();
        assert_eq!(result.execution.provider_id, PROVIDER_LOCAL);
        assert_eq!(result.execution.fallback_step, 2);
    }

    #[test]
    fn exhausted_free_paths_degrade_curator_without_paid_fallback() {
        let router = AiRouter::with_providers(vec![
            free_provider(
                PROVIDER_GEMINI,
                Err(ProviderError::new(
                    ProviderErrorKind::RateLimited,
                    "quota exhausted",
                )),
            ),
            free_provider(
                PROVIDER_GROQ,
                Err(ProviderError::new(
                    ProviderErrorKind::RateLimited,
                    "quota exhausted",
                )),
            ),
        ]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let mut task = request(false);
        task.provider_order = vec![PROVIDER_GEMINI.to_string(), PROVIDER_GROQ.to_string()];
        assert!(matches!(
            router.run(&mut database, task),
            Err(crate::core::error::CoreError::AiProvider { .. })
        ));
        let report = router.health();
        assert_eq!(report.router_state, AiProviderState::Degraded);
        assert!(report
            .providers
            .iter()
            .all(|provider| provider.state == AiProviderState::RateLimited));
    }

    #[test]
    fn non_zero_provider_cost_is_rejected_and_never_persisted_as_success() {
        let router = AiRouter::with_providers(vec![
            free_provider(
                PROVIDER_GEMINI,
                Ok(ProviderResponse {
                    cost: Some(0.01),
                    ..review_output()
                }),
            ),
            free_provider(PROVIDER_LOCAL, Ok(review_output())),
        ]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let mut task = request(false);
        task.provider_order = vec![PROVIDER_GEMINI.to_string(), PROVIDER_LOCAL.to_string()];
        let result = router.run(&mut database, task).unwrap();
        assert_eq!(result.execution.provider_id, PROVIDER_LOCAL);
        assert_eq!(result.execution.cost, Some(0.0));
        let executions = database.ai_executions(20).unwrap();
        assert!(executions
            .iter()
            .any(|execution| execution.error_code.as_deref() == Some("AI_ZERO_COST_POLICY")));
    }

    #[test]
    fn no_provider_is_degraded_and_does_not_block_the_catalog() {
        let router = AiRouter::with_providers(vec![Box::new(FakeProvider {
            id: "primary",
            state: AiProviderState::NotConfigured,
            zero_cost_eligible: false,
            outputs: std::sync::Mutex::new(vec![]),
        })]);
        assert_eq!(router.health().router_state, AiProviderState::Degraded);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let error = router.run(&mut database, request(false)).unwrap_err();
        assert!(matches!(
            error,
            crate::core::error::CoreError::AiNotConfigured(_)
        ));
        assert_eq!(database.milano_snapshot().unwrap().schedule.len(), 214);
    }

    #[test]
    fn invalid_output_falls_back_and_persists_a_candidate_without_official_write() {
        let router = AiRouter::with_providers(vec![
            Box::new(FakeProvider {
                id: "primary",
                state: AiProviderState::Available,
                zero_cost_eligible: true,
                outputs: std::sync::Mutex::new(vec![Ok(ProviderResponse {
                    output: json!({ "summary": "missing required fields" }),
                    usage: None,
                    cost: None,
                })]),
            }),
            Box::new(FakeProvider {
                id: "fallback",
                state: AiProviderState::Available,
                zero_cost_eligible: true,
                outputs: std::sync::Mutex::new(vec![Ok(review_output())]),
            }),
        ]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let result = router.run(&mut database, request(false)).unwrap();
        assert_eq!(result.execution.provider_id, "fallback");
        assert_eq!(result.proposal.state, "pending");
        assert!(!result.proposal.official_applied);
        assert_eq!(database.milano_snapshot().unwrap().schedule.len(), 214);
    }

    #[test]
    fn local_only_never_uses_cloud_provider() {
        let router = AiRouter::with_providers(vec![Box::new(FakeProvider {
            id: "cloud",
            state: AiProviderState::Available,
            zero_cost_eligible: true,
            outputs: std::sync::Mutex::new(vec![Ok(review_output())]),
        })]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let mut request = request(true);
        request.provider_order = vec!["cloud".to_string()];
        let error = router.run(&mut database, request).unwrap_err();
        assert!(matches!(error, crate::core::error::CoreError::AiPolicy(_)));
    }

    #[test]
    fn rate_limit_falls_back_to_the_next_provider() {
        let router = AiRouter::with_providers(vec![
            Box::new(FakeProvider {
                id: "primary",
                state: AiProviderState::Available,
                zero_cost_eligible: true,
                outputs: std::sync::Mutex::new(vec![Err(ProviderError::new(
                    ProviderErrorKind::RateLimited,
                    "rate limited",
                ))]),
            }),
            Box::new(FakeProvider {
                id: "fallback",
                state: AiProviderState::Available,
                zero_cost_eligible: true,
                outputs: std::sync::Mutex::new(vec![Ok(review_output())]),
            }),
        ]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let result = router.run(&mut database, request(false)).unwrap();
        assert_eq!(result.execution.provider_id, "fallback");
        assert_eq!(result.execution.fallback_step, 1);
    }

    #[test]
    fn timeout_falls_back_without_retrying_forever() {
        let router = AiRouter::with_providers(vec![
            Box::new(FakeProvider {
                id: "primary",
                state: AiProviderState::Available,
                zero_cost_eligible: true,
                outputs: std::sync::Mutex::new(vec![Err(ProviderError::new(
                    ProviderErrorKind::Timeout,
                    "timed out",
                ))]),
            }),
            Box::new(FakeProvider {
                id: "fallback",
                state: AiProviderState::Available,
                zero_cost_eligible: true,
                outputs: std::sync::Mutex::new(vec![Ok(review_output())]),
            }),
        ]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let result = router.run(&mut database, request(false)).unwrap();
        assert_eq!(result.execution.provider_id, "fallback");
        assert_eq!(database.ai_executions(20).unwrap().len(), 2);
    }

    #[test]
    fn human_approval_is_the_only_gate_that_promotes_a_new_source() {
        let router = AiRouter::with_providers(vec![Box::new(FakeProvider {
            id: "primary",
            state: AiProviderState::Available,
            zero_cost_eligible: true,
            outputs: std::sync::Mutex::new(vec![Ok(ProviderResponse {
                output: json!({
                    "source": {
                        "id": "source:curator-discovered",
                        "name": "Curator-discovered evidence source",
                        "sourceKind": "editorial",
                        "authorityTier": "E",
                        "baseUrl": "https://example.invalid/source",
                        "accessMode": "remote_render"
                    },
                    "rationale": "Candidate source requires human review.",
                    "confidence": 0.4
                }),
                usage: None,
                cost: None,
            })]),
        })]);
        let mut database = CatalogDb::in_memory().unwrap();
        database.seed_bundled_milano().unwrap();
        let root =
            std::env::temp_dir().join(format!("nex-fashion-ai-approval-{}", std::process::id()));
        let runtime = PackRuntime::new(&root).unwrap();
        runtime.ensure_available(&mut database).unwrap();
        runtime.install(&mut database, BUNDLED_PACK_ID).unwrap();
        let mut request = request(false);
        request.task_type = "source_discovery".to_string();
        request.provider_order = vec!["primary".to_string()];
        let result = router.run(&mut database, request).unwrap();
        assert!(!result.proposal.official_applied);
        assert!(matches!(
            database.read_source("source:curator-discovered"),
            Err(crate::core::error::CoreError::NotFound { .. })
        ));

        let approved = database
            .decide_curator_proposal(&CuratorProposalDecisionRequest {
                proposal_id: result.proposal.proposal_id,
                decision: "APPROVE".to_string(),
                reviewer: "admin:test".to_string(),
                edited: None,
                reason: Some("Evidence reviewed manually.".to_string()),
            })
            .unwrap();
        assert!(approved.official_applied);
        assert_eq!(
            database
                .read_source("source:curator-discovered")
                .unwrap()
                .status,
            "active"
        );
        drop(database);
        std::fs::remove_dir_all(root).unwrap();
    }
}
