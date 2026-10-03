CREATE TABLE IF NOT EXISTS adapter_registry (
    adapter_id TEXT PRIMARY KEY,
    version TEXT NOT NULL,
    source_ids_json TEXT NOT NULL DEFAULT '[]',
    integration_ids_json TEXT NOT NULL DEFAULT '[]',
    capabilities_json TEXT NOT NULL DEFAULT '[]',
    discovery_strategy TEXT NOT NULL,
    supported_content_types_json TEXT NOT NULL DEFAULT '[]',
    fetch_strategy TEXT NOT NULL,
    parse_strategy TEXT NOT NULL,
    normalization_strategy TEXT NOT NULL,
    pagination_json TEXT NOT NULL DEFAULT '{}',
    rate_limit_json TEXT NOT NULL DEFAULT '{}',
    retry_policy_json TEXT NOT NULL DEFAULT '{}',
    provenance_support INTEGER NOT NULL DEFAULT 1 CHECK (provenance_support IN (0, 1)),
    produces_json TEXT NOT NULL DEFAULT '[]',
    fixture_support INTEGER NOT NULL DEFAULT 0 CHECK (fixture_support IN (0, 1)),
    test_support INTEGER NOT NULL DEFAULT 0 CHECK (test_support IN (0, 1)),
    health_state TEXT NOT NULL DEFAULT 'NOT_CONFIGURED',
    health_detail TEXT,
    last_checked_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS integration_registry (
    integration_id TEXT PRIMARY KEY,
    source_id TEXT,
    adapter_id TEXT NOT NULL REFERENCES adapter_registry(adapter_id) ON DELETE CASCADE,
    endpoint_id TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    method TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    auth_policy TEXT NOT NULL DEFAULT 'none',
    rate_limit_json TEXT NOT NULL DEFAULT '{}',
    cost_policy TEXT NOT NULL DEFAULT 'ZERO',
    health_state TEXT NOT NULL DEFAULT 'NOT_CONFIGURED',
    health_detail TEXT,
    last_checked_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (source_id, adapter_id, endpoint)
);

CREATE TABLE IF NOT EXISTS source_health (
    source_id TEXT PRIMARY KEY,
    state TEXT NOT NULL,
    detail TEXT,
    last_checked_at TEXT NOT NULL,
    adapter_id TEXT
);

CREATE TABLE IF NOT EXISTS raw_artifact (
    artifact_id TEXT PRIMARY KEY,
    source_id TEXT,
    adapter_id TEXT NOT NULL,
    integration_id TEXT,
    canonical_url TEXT NOT NULL,
    content_type TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    byte_size INTEGER NOT NULL,
    storage_kind TEXT NOT NULL DEFAULT 'inline',
    content_text TEXT,
    content_ref TEXT,
    etag TEXT,
    last_modified TEXT,
    retrieved_at TEXT NOT NULL,
    retrieval_status TEXT NOT NULL DEFAULT 'acquired',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (adapter_id, canonical_url, content_hash)
);

CREATE INDEX IF NOT EXISTS idx_raw_artifact_source_url
    ON raw_artifact(source_id, canonical_url, retrieved_at);

CREATE TABLE IF NOT EXISTS source_observation (
    observation_id TEXT PRIMARY KEY,
    artifact_id TEXT NOT NULL REFERENCES raw_artifact(artifact_id) ON DELETE CASCADE,
    source_id TEXT,
    adapter_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    stable_key TEXT NOT NULL,
    observed_json TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    parser_version TEXT NOT NULL,
    observed_at TEXT NOT NULL,
    UNIQUE (artifact_id, entity_type, stable_key, content_hash)
);

CREATE INDEX IF NOT EXISTS idx_source_observation_stable
    ON source_observation(adapter_id, entity_type, stable_key, observed_at);

CREATE TABLE IF NOT EXISTS adapter_run (
    run_id TEXT PRIMARY KEY,
    adapter_id TEXT NOT NULL,
    source_id TEXT,
    requested_capability TEXT,
    status TEXT NOT NULL CHECK (status IN ('running', 'succeeded', 'degraded', 'failed')),
    cursor TEXT,
    checkpoint_json TEXT NOT NULL DEFAULT '{}',
    fixture_mode INTEGER NOT NULL DEFAULT 0 CHECK (fixture_mode IN (0, 1)),
    attempts INTEGER NOT NULL DEFAULT 0,
    artifact_count INTEGER NOT NULL DEFAULT 0,
    observation_count INTEGER NOT NULL DEFAULT 0,
    candidate_count INTEGER NOT NULL DEFAULT 0,
    changed_count INTEGER NOT NULL DEFAULT 0,
    error_code TEXT,
    error_message TEXT,
    started_at TEXT NOT NULL,
    completed_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_adapter_run_status
    ON adapter_run(adapter_id, status, started_at);

CREATE TABLE IF NOT EXISTS ingestion_checkpoint (
    adapter_id TEXT NOT NULL,
    source_id TEXT NOT NULL,
    checkpoint_key TEXT NOT NULL,
    cursor TEXT,
    content_hash TEXT,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (adapter_id, source_id, checkpoint_key)
);

CREATE TABLE IF NOT EXISTS ingestion_candidate (
    candidate_id TEXT PRIMARY KEY,
    source_id TEXT,
    adapter_id TEXT NOT NULL,
    run_id TEXT NOT NULL REFERENCES adapter_run(run_id) ON DELETE RESTRICT,
    candidate_kind TEXT NOT NULL,
    target_entity_type TEXT,
    target_entity_id TEXT,
    stable_key TEXT NOT NULL,
    proposed_json TEXT NOT NULL,
    evidence_json TEXT NOT NULL DEFAULT '[]',
    provenance_json TEXT NOT NULL DEFAULT '[]',
    comparison_state TEXT NOT NULL DEFAULT 'UNKNOWN',
    content_hash TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'converted', 'rejected', 'superseded')),
    seen_count INTEGER NOT NULL DEFAULT 1,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (adapter_id, stable_key)
);

CREATE INDEX IF NOT EXISTS idx_ingestion_candidate_status
    ON ingestion_candidate(status, updated_at);

CREATE TABLE IF NOT EXISTS ingestion_proposal (
    proposal_id TEXT PRIMARY KEY,
    candidate_id TEXT NOT NULL UNIQUE REFERENCES ingestion_candidate(candidate_id) ON DELETE RESTRICT,
    proposal_kind TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending', 'approved', 'rejected', 'needs_more_evidence')),
    proposed_json TEXT NOT NULL,
    evidence_json TEXT NOT NULL DEFAULT '[]',
    provenance_json TEXT NOT NULL DEFAULT '[]',
    comparison_state TEXT NOT NULL DEFAULT 'UNKNOWN',
    reviewer TEXT,
    decision_reason TEXT,
    decided_at TEXT,
    official_applied INTEGER NOT NULL DEFAULT 0 CHECK (official_applied IN (0, 1)),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ingestion_proposal_state
    ON ingestion_proposal(state, updated_at);

CREATE TABLE IF NOT EXISTS source_candidate (
    source_candidate_id TEXT PRIMARY KEY,
    source_key TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    base_url TEXT,
    source_kind TEXT,
    domains_json TEXT NOT NULL DEFAULT '[]',
    locale_json TEXT NOT NULL DEFAULT '{}',
    capabilities_json TEXT NOT NULL DEFAULT '[]',
    evidence_json TEXT NOT NULL DEFAULT '[]',
    provenance_json TEXT NOT NULL DEFAULT '[]',
    discovered_by TEXT NOT NULL DEFAULT 'curator',
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS adapter_candidate (
    adapter_candidate_id TEXT PRIMARY KEY,
    source_candidate_id TEXT REFERENCES source_candidate(source_candidate_id) ON DELETE RESTRICT,
    adapter_id TEXT,
    proposed_json TEXT NOT NULL,
    capabilities_json TEXT NOT NULL DEFAULT '[]',
    evidence_json TEXT NOT NULL DEFAULT '[]',
    provenance_json TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS integration_proposal (
    integration_proposal_id TEXT PRIMARY KEY,
    source_candidate_id TEXT REFERENCES source_candidate(source_candidate_id) ON DELETE RESTRICT,
    adapter_candidate_id TEXT REFERENCES adapter_candidate(adapter_candidate_id) ON DELETE RESTRICT,
    source_id TEXT,
    adapter_id TEXT,
    proposal_json TEXT NOT NULL,
    evidence_json TEXT NOT NULL DEFAULT '[]',
    provenance_json TEXT NOT NULL DEFAULT '[]',
    state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending', 'approved', 'rejected')),
    reviewer TEXT,
    decision_reason TEXT,
    decided_at TEXT,
    official_applied INTEGER NOT NULL DEFAULT 0 CHECK (official_applied IN (0, 1)),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_source_candidate_status
    ON source_candidate(status, updated_at);

CREATE INDEX IF NOT EXISTS idx_adapter_candidate_status
    ON adapter_candidate(status, updated_at);

CREATE INDEX IF NOT EXISTS idx_integration_proposal_state
    ON integration_proposal(state, updated_at);
