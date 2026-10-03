CREATE TABLE IF NOT EXISTS ai_execution (
    execution_id TEXT PRIMARY KEY,
    task_type TEXT NOT NULL,
    provider_id TEXT NOT NULL,
    model TEXT NOT NULL,
    capability TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    latency_ms INTEGER,
    attempt INTEGER NOT NULL,
    fallback_step INTEGER NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('succeeded', 'failed', 'skipped', 'degraded')),
    validator_schema TEXT NOT NULL,
    validation_result_json TEXT NOT NULL DEFAULT '{}',
    error_code TEXT,
    error_message TEXT,
    input_hash TEXT NOT NULL,
    usage_json TEXT,
    cost_amount REAL,
    candidate_id TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS ai_candidate (
    candidate_id TEXT PRIMARY KEY,
    proposal_kind TEXT NOT NULL,
    target_entity_type TEXT,
    target_entity_id TEXT,
    proposed_json TEXT NOT NULL,
    evidence_json TEXT NOT NULL DEFAULT '[]',
    provenance_json TEXT NOT NULL DEFAULT '[]',
    rationale TEXT,
    confidence REAL,
    execution_id TEXT NOT NULL REFERENCES ai_execution(execution_id) ON DELETE RESTRICT,
    status TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'converted', 'rejected', 'superseded')),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS curator_proposal (
    proposal_id TEXT PRIMARY KEY,
    candidate_id TEXT NOT NULL REFERENCES ai_candidate(candidate_id) ON DELETE RESTRICT,
    proposal_kind TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending', 'approved', 'rejected', 'needs_more_evidence', 'edited_approved')),
    proposed_json TEXT NOT NULL,
    edited_json TEXT,
    before_json TEXT,
    approved_json TEXT,
    evidence_json TEXT NOT NULL DEFAULT '[]',
    provenance_json TEXT NOT NULL DEFAULT '[]',
    rationale TEXT,
    confidence REAL,
    reviewer TEXT,
    decision_reason TEXT,
    decided_at TEXT,
    official_applied INTEGER NOT NULL DEFAULT 0 CHECK (official_applied IN (0, 1)),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_ai_execution_task ON ai_execution(task_type, created_at);
CREATE INDEX IF NOT EXISTS idx_ai_execution_provider ON ai_execution(provider_id, status, created_at);
CREATE INDEX IF NOT EXISTS idx_ai_candidate_target ON ai_candidate(target_entity_type, target_entity_id);
CREATE INDEX IF NOT EXISTS idx_ai_candidate_status ON ai_candidate(status, created_at);
CREATE INDEX IF NOT EXISTS idx_curator_proposal_state ON curator_proposal(state, created_at);
