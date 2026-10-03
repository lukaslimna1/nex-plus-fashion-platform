CREATE TABLE IF NOT EXISTS pack_runtime (
    pack_id TEXT PRIMARY KEY,
    version TEXT NOT NULL,
    family TEXT NOT NULL,
    scope_json TEXT NOT NULL,
    schema_version TEXT NOT NULL,
    app_compatibility_json TEXT NOT NULL,
    manifest_json TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    artifact_hash TEXT,
    size_bytes INTEGER NOT NULL DEFAULT 0,
    state TEXT NOT NULL CHECK (state IN ('available', 'staged', 'verifying', 'verified', 'installing', 'active', 'removed', 'error', 'repair_required')),
    progress INTEGER NOT NULL DEFAULT 0 CHECK (progress >= 0 AND progress <= 100),
    staged_path TEXT,
    installed_path TEXT,
    last_error TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pack_membership (
    pack_id TEXT NOT NULL,
    pack_version TEXT NOT NULL,
    entity_kind TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (pack_id, pack_version, entity_kind, entity_id)
);

CREATE INDEX IF NOT EXISTS idx_pack_membership_entity ON pack_membership(entity_kind, entity_id);
CREATE INDEX IF NOT EXISTS idx_pack_runtime_state ON pack_runtime(state);
