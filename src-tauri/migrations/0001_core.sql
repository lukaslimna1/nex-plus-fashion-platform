CREATE TABLE IF NOT EXISTS schema_migration (
    name TEXT PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS catalog_entity (
    id TEXT PRIMARY KEY,
    entity_kind TEXT NOT NULL CHECK (entity_kind IN (
        'city_hub', 'event', 'edition', 'schedule_entry', 'venue',
        'maison', 'person', 'collection', 'source', 'media_asset',
        'review', 'personal_note'
    )),
    display_name TEXT NOT NULL,
    search_text TEXT NOT NULL DEFAULT '',
    payload_json TEXT NOT NULL DEFAULT '{}',
    is_official INTEGER NOT NULL DEFAULT 1 CHECK (is_official IN (0, 1)),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS catalog_source (
    id TEXT PRIMARY KEY,
    uri TEXT NOT NULL,
    source_kind TEXT NOT NULL,
    publisher_name TEXT,
    trust_state TEXT NOT NULL DEFAULT 'unreviewed',
    payload_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS personal_note (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL REFERENCES catalog_entity(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_catalog_entity_kind ON catalog_entity(entity_kind);
CREATE INDEX IF NOT EXISTS idx_catalog_entity_official ON catalog_entity(is_official);
CREATE INDEX IF NOT EXISTS idx_catalog_source_uri ON catalog_source(uri);
CREATE INDEX IF NOT EXISTS idx_personal_note_entity ON personal_note(entity_id);
