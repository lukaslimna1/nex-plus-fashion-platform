CREATE TABLE IF NOT EXISTS personal_favorite (
    entity_id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_personal_favorite_created_at ON personal_favorite(created_at);
