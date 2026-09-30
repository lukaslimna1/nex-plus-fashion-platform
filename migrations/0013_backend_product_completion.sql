-- Backend product completion. Additive only: AI output remains pending_review.
ALTER TABLE maisons ADD COLUMN logo_url TEXT;
ALTER TABLE maisons ADD COLUMN founded_by_json TEXT NOT NULL DEFAULT '[]';
ALTER TABLE maisons ADD COLUMN country_name TEXT;
ALTER TABLE maisons ADD COLUMN city_name TEXT;
ALTER TABLE maisons ADD COLUMN headquarters TEXT;
ALTER TABLE maisons ADD COLUMN current_creative_director TEXT;
ALTER TABLE maisons ADD COLUMN about TEXT;
ALTER TABLE maisons ADD COLUMN history TEXT;
ALTER TABLE maisons ADD COLUMN socials_json TEXT NOT NULL DEFAULT '{}';
ALTER TABLE maisons ADD COLUMN other_official_links_json TEXT NOT NULL DEFAULT '{}';
ALTER TABLE maisons ADD COLUMN research_status TEXT NOT NULL DEFAULT 'NEEDS_RESEARCH';
ALTER TABLE maisons ADD COLUMN verified_at TEXT;

CREATE TABLE IF NOT EXISTS trends (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  definition TEXT,
  scope_json TEXT NOT NULL DEFAULT '{}',
  evidence_count INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL CHECK (status IN ('CONFIRMED','PENDING_REVIEW','MODEL_SUGGESTION','NO_EVIDENCE')),
  provenance TEXT NOT NULL CHECK (provenance IN ('CURATED','SOURCE','COMMUNITY','MODEL_SUGGESTION')),
  source_ids TEXT NOT NULL DEFAULT '',
  retrieved_at TEXT,
  revision INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);

CREATE TABLE IF NOT EXISTS trend_evidence (
  trend_id TEXT NOT NULL REFERENCES trends(id),
  collection_id TEXT REFERENCES collections(id),
  asset_id TEXT REFERENCES assets(id),
  review_id TEXT REFERENCES professional_reviews(id),
  tag_id TEXT REFERENCES tags(id),
  evidence TEXT,
  ordinal INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (trend_id, collection_id, asset_id, review_id, tag_id)
);

CREATE INDEX IF NOT EXISTS idx_trends_status ON trends(status, provenance);
CREATE INDEX IF NOT EXISTS idx_trend_evidence_trend ON trend_evidence(trend_id, ordinal);

CREATE TABLE IF NOT EXISTS ai_enrichment_jobs (
  id TEXT PRIMARY KEY,
  entity_type TEXT NOT NULL CHECK (entity_type IN ('TERM','MAISON','COLLECTION','EVENT')),
  entity_id TEXT NOT NULL,
  task TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('PENDING','RUNNING','SUCCEEDED','FAILED','BLOCKED')),
  input_source_ids TEXT NOT NULL DEFAULT '',
  context_json TEXT NOT NULL,
  result_json TEXT,
  review_status TEXT NOT NULL CHECK (review_status IN ('AI_DRAFT','PENDING_REVIEW','VALIDATED','REJECTED')) DEFAULT 'PENDING_REVIEW',
  attempts INTEGER NOT NULL DEFAULT 0,
  provider TEXT,
  model TEXT,
  error TEXT,
  scheduled_at TEXT NOT NULL,
  completed_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE(entity_type, entity_id, task)
);

CREATE INDEX IF NOT EXISTS idx_ai_enrichment_jobs_due ON ai_enrichment_jobs(status, scheduled_at);

CREATE TABLE IF NOT EXISTS user_preferences (
  identity_key TEXT PRIMARY KEY,
  language TEXT NOT NULL DEFAULT 'pt-BR',
  timezone TEXT NOT NULL DEFAULT 'UTC',
  autoplay_preview INTEGER NOT NULL DEFAULT 0,
  reduced_motion INTEGER NOT NULL DEFAULT 0,
  media_preference TEXT NOT NULL DEFAULT 'REMOTE',
  editorial_preferences_json TEXT NOT NULL DEFAULT '{}',
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS personal_sync_records (
  identity_key TEXT NOT NULL,
  scope TEXT NOT NULL CHECK (scope IN ('favorites','preferences','settings','reading-state')),
  provider TEXT NOT NULL CHECK (provider IN ('GOOGLE_DRIVE_APP_DATA')),
  remote_key TEXT,
  last_synced_at TEXT,
  revision INTEGER NOT NULL DEFAULT 1,
  PRIMARY KEY (identity_key, scope)
);

INSERT OR IGNORE INTO sources (id, canonical_name, type, base_url, authority_tier, language, coverage_scope, access_mode, rights_notes, automation_notes, last_verified_at, active, created_at, updated_at)
VALUES
  ('source-dior-official', 'Dior official', 'MAISON_OFFICIAL', 'https://www.dior.com/en_us/fashion', 'A', 'en,fr', 'maison,collections', 'PUBLIC', 'Official Maison source; link-only provenance unless a reproducible media URL is verified.', 'Prefer official pages for Maison metadata; do not infer missing fields.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-balmain-official', 'Balmain official', 'MAISON_OFFICIAL', 'https://balmain.com/en-fr', 'A', 'en,fr', 'maison,collections', 'PUBLIC', 'Official Maison source; preserve source URL and rights metadata.', 'Prefer official pages for Maison metadata; do not infer missing fields.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-courreges-official', 'Courrèges official', 'MAISON_OFFICIAL', 'https://www.courreges.com/', 'A', 'en,fr', 'maison,collections', 'PUBLIC', 'Official Maison source; preserve source URL and rights metadata.', 'Prefer official pages for Maison metadata; do not infer missing fields.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

UPDATE maisons SET website_url = 'https://www.dior.com/en_us/fashion', official_source_ids = CASE WHEN instr(',' || official_source_ids || ',', ',source-dior-official,') > 0 THEN official_source_ids ELSE trim(official_source_ids || ',source-dior-official', ',') END, research_status = 'PARTIAL', verified_at = '2026-09-30T00:00:00.000Z' WHERE slug = 'christian-dior';
UPDATE maisons SET website_url = 'https://balmain.com/en-fr', official_source_ids = CASE WHEN instr(',' || official_source_ids || ',', ',source-balmain-official,') > 0 THEN official_source_ids ELSE trim(official_source_ids || ',source-balmain-official', ',') END, research_status = 'PARTIAL', verified_at = '2026-09-30T00:00:00.000Z' WHERE slug = 'balmain';
UPDATE maisons SET website_url = 'https://www.courreges.com/', official_source_ids = CASE WHEN instr(',' || official_source_ids || ',', ',source-courreges-official,') > 0 THEN official_source_ids ELSE trim(official_source_ids || ',source-courreges-official', ',') END, research_status = 'PARTIAL', verified_at = '2026-09-30T00:00:00.000Z' WHERE slug = 'courreges';

-- This is intentionally scoped evidence, not a general trend claim.
INSERT OR IGNORE INTO trends (id, name, slug, definition, scope_json, evidence_count, status, provenance, source_ids, retrieved_at, created_at, updated_at)
VALUES ('trend-maxhosa-ss27-earth-tones', 'Earth-toned knitwear in MAXHOSA AFRICA SS27', 'maxhosa-ss27-earth-tones', 'A scoped observation from the MAXHOSA AFRICA SS27 collection; not a general market trend.', '{"seasonCodes":["SS27"],"collectionIds":["collection-maxhosa-africa-ss27-2026"]}', 1, 'PENDING_REVIEW', 'SOURCE', 'source-fhcm,source-vogue-runway', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');
INSERT OR IGNORE INTO trend_evidence (trend_id, collection_id, evidence, ordinal)
VALUES ('trend-maxhosa-ss27-earth-tones', 'collection-maxhosa-africa-ss27-2026', 'Scoped collection evidence from the canonical SS27 record; human review required before promotion.', 0);

-- Queue small, inspectable batches. The scheduler only writes result_json and ai_runs;
-- a human review step remains mandatory before any catalog field changes.
INSERT OR IGNORE INTO ai_enrichment_jobs (id, entity_type, entity_id, task, status, input_source_ids, context_json, review_status, scheduled_at, created_at, updated_at)
SELECT 'ai-term-' || id, 'TERM', id, 'translateToPtBr', 'PENDING', source_ids,
  json_object('entityType','TERM','entityId',id,'canonicalName',COALESCE(canonical_name,value),'originalLabel',value,'language',language,'sourceUrls',json_array(external_uri),'retrievedText',COALESCE(definition,''),'currentState',json_object('category',category,'aliases',aliases_json),'allowedOutputSchema',json_object('type','object','properties',json_object('ptBrName',json_object('type','string'),'aliases',json_object('type','array')))),
  'PENDING_REVIEW', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'
FROM terms WHERE id IN ('term-izihlwele-zivumile','term-getty-knitting-process','term-getty-machine-knitting');

INSERT OR IGNORE INTO ai_enrichment_jobs (id, entity_type, entity_id, task, status, input_source_ids, context_json, review_status, scheduled_at, created_at, updated_at)
SELECT 'ai-maison-' || id, 'MAISON', id, 'extractMaisonMetadata', 'PENDING', official_source_ids,
  json_object('entityType','MAISON','entityId',id,'canonicalName',name,'language','en','sourceUrls',json_array(website_url),'retrievedText','','currentState',json_object('websiteUrl',website_url,'foundedYear',founded_year,'artisticDirection',artistic_direction),'allowedOutputSchema',json_object('type','object','properties',json_object('about',json_object('type','string'),'history',json_object('type','string'),'foundedBy',json_object('type','array')))),
  'PENDING_REVIEW', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'
FROM maisons WHERE deleted_at IS NULL;

-- The previously catalogued NSS Instagram reel is no longer a reproducible source.
-- Keep provenance for audit, but make it impossible for API consumers to render it as playable media.
UPDATE assets
SET embed_url = NULL,
    display_mode = 'LINK_ONLY',
    availability_status = 'REMOVED',
    canonical_status = 'CANONICAL',
    metadata_json = '{"availabilityEvidence":"source reported removed/unavailable","playbackPolicy":"do_not_render","historicalProvider":"instagram"}',
    updated_at = '2026-09-30T00:00:00.000Z'
WHERE id = 'asset-julie-kegels-ss27-reel';

UPDATE assets
SET embed_url = NULL,
    display_mode = 'LINK_ONLY',
    availability_status = 'UNKNOWN',
    metadata_json = '{"availabilityEvidence":"original Vogue page retained; embed not reproducibly verified by current audit","playbackPolicy":"link_only_until_reverified"}',
    updated_at = '2026-09-30T00:00:00.000Z'
WHERE id = 'asset-christian-dior-ss27-vogue-video';
