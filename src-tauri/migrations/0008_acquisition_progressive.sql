CREATE TABLE IF NOT EXISTS raw_artifact_body (
    content_hash TEXT PRIMARY KEY,
    content_type TEXT NOT NULL,
    byte_size INTEGER NOT NULL,
    content_text TEXT,
    content_ref TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

ALTER TABLE raw_artifact ADD COLUMN final_url TEXT;
ALTER TABLE raw_artifact ADD COLUMN acquisition_method TEXT NOT NULL DEFAULT 'fixture';
ALTER TABLE raw_artifact ADD COLUMN http_status INTEGER;
ALTER TABLE raw_artifact ADD COLUMN parent_url TEXT;
ALTER TABLE raw_artifact ADD COLUMN referrer_url TEXT;
ALTER TABLE raw_artifact ADD COLUMN pagination_json TEXT NOT NULL DEFAULT '{}';

INSERT OR IGNORE INTO raw_artifact_body
    (content_hash, content_type, byte_size, content_text, content_ref, created_at, updated_at)
SELECT content_hash, content_type, byte_size, content_text, content_ref, created_at, updated_at
FROM raw_artifact
WHERE content_text IS NOT NULL OR content_ref IS NOT NULL;

UPDATE raw_artifact
SET final_url = COALESCE(final_url, canonical_url),
    content_ref = COALESCE(content_ref, 'body:' || content_hash),
    content_text = NULL
WHERE content_text IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_raw_artifact_final_url
    ON raw_artifact(adapter_id, final_url, retrieved_at);

CREATE INDEX IF NOT EXISTS idx_raw_artifact_content_hash
    ON raw_artifact(content_hash);
