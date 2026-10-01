-- P0 media completeness for PFW Womenswear SS27 (28-30/09/2026).
-- URL/provenance catalog only: no third-party media is downloaded or rehosted.
ALTER TABLE assets ADD COLUMN original_url TEXT;

CREATE TABLE IF NOT EXISTS collection_schedule_entries (
  collection_id TEXT NOT NULL REFERENCES collections(id),
  schedule_entry_id TEXT NOT NULL REFERENCES schedule_entries(id),
  relation_status TEXT NOT NULL CHECK (relation_status IN ('VERIFIED','PENDING_REVIEW')) DEFAULT 'VERIFIED',
  evidence TEXT NOT NULL,
  verified_at TEXT NOT NULL,
  PRIMARY KEY (collection_id, schedule_entry_id)
);

CREATE INDEX IF NOT EXISTS idx_collection_schedule_entries_schedule
  ON collection_schedule_entries(schedule_entry_id);

-- The canonical PFW first-three-day seed uses the same Maison title in both
-- records. This deterministic join creates the missing explicit relation for
-- exactly the 30 entries in scope, without a manual external list.
INSERT OR IGNORE INTO collection_schedule_entries
  (collection_id, schedule_entry_id, relation_status, evidence, verified_at)
SELECT c.id, s.id, 'VERIFIED',
  'Exact case-insensitive Maison title match within the canonical PFW SS27 edition and civil date window.',
  '2026-09-30T00:00:00.000Z'
FROM collections c
JOIN maisons m ON m.id = c.maison_id
JOIN schedule_entries s
  ON s.edition_id = c.edition_id
 AND lower(trim(s.title)) = lower(trim(m.name))
WHERE c.edition_id = 'edition-pfw-womenswear-ss27-2026'
  AND substr(s.start_time, 1, 10) BETWEEN '2026-09-28' AND '2026-09-30';

CREATE TABLE IF NOT EXISTS media_research_matrix (
  id TEXT PRIMARY KEY,
  collection_id TEXT NOT NULL REFERENCES collections(id),
  source_key TEXT NOT NULL,
  source_id TEXT REFERENCES sources(id),
  media_type TEXT NOT NULL CHECK (media_type IN ('IMAGE','VIDEO')),
  state TEXT NOT NULL CHECK (state IN ('FOUND','EMPTY','BLOCKED','REQUIRES_ACCESS','NOT_APPLICABLE','NEEDS_RESEARCH')),
  checked_at TEXT NOT NULL,
  source_page_url TEXT,
  result_count INTEGER NOT NULL DEFAULT 0,
  reason TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  UNIQUE(collection_id, source_key, media_type)
);

CREATE INDEX IF NOT EXISTS idx_media_research_matrix_collection
  ON media_research_matrix(collection_id, media_type, state);

INSERT OR IGNORE INTO sources
  (id, canonical_name, type, base_url, authority_tier, language, coverage_scope, access_mode, rights_notes, automation_notes, last_verified_at, active, created_at, updated_at)
VALUES
  ('source-oui-speak-fashion', 'Oui Speak Fashion', 'RUNWAY_COVERAGE', 'https://ouispeakfashion.com/', 'C', 'en', 'runway,collections,editorial', 'PUBLIC', 'Editorial source; no download or rehost permission is inferred.', 'Use article URL as sourcePageUrl and preserve courtesy/creator credits from each article.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-nowfashion', 'NOWFASHION', 'RUNWAY_COVERAGE', 'https://nowfashion.com/', 'B', 'en', 'runway,collections', 'PUBLIC', 'Runway archive source; no download or rehost permission is inferred.', 'Use collection-specific page URLs only after deterministic discovery.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-tagwalk', 'Tagwalk', 'RUNWAY_COVERAGE', 'https://www.tag-walk.com/en/', 'B', 'en,fr', 'runway,collections,trends', 'PUBLIC', 'Runway search source; no download or rehost permission is inferred.', 'Use collection-specific page URLs only after deterministic discovery.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-ff-channel', 'FF Channel', 'VIDEO_CHANNEL', 'https://www.youtube.com/@FFChannel', 'B', 'en', 'runway,video', 'PUBLIC', 'Channel-hosted video; no download or rehost permission is inferred.', 'Classify show completeness only from source evidence.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-fashion-channel', 'Fashion Channel', 'VIDEO_CHANNEL', 'https://www.youtube.com/@FashionChannel', 'B', 'en,it', 'runway,video,backstage', 'PUBLIC', 'Channel-hosted video; no download or rehost permission is inferred.', 'Classify show completeness only from source evidence.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

UPDATE collections
SET source_ids = CASE
  WHEN instr(',' || source_ids || ',', ',source-oui-speak-fashion,') > 0 THEN source_ids
  WHEN source_ids = '' THEN 'source-oui-speak-fashion'
  ELSE source_ids || ',source-oui-speak-fashion'
END,
updated_at = '2026-09-30T00:00:00.000Z'
WHERE id = 'collection-pfw-ss27-acne-studios-2026';

WITH image_data(sequence_number, remote_url, thumbnail_url, alternative_url) AS (
VALUES
  (1, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-001.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-001.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-001.webp?fit=683%2C1024&ssl=1'),
  (2, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-002.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-002.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-002.webp?fit=683%2C1024&ssl=1'),
  (3, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-003.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-003.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-003.webp?fit=683%2C1024&ssl=1'),
  (4, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-004.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-004.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-004.webp?fit=683%2C1024&ssl=1'),
  (5, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-005.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-005.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-005.webp?fit=683%2C1024&ssl=1'),
  (6, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-006.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-006.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-006.webp?fit=683%2C1024&ssl=1'),
  (7, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-007.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-007.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-007.webp?fit=683%2C1024&ssl=1'),
  (8, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-008.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-008.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-008.webp?fit=683%2C1024&ssl=1'),
  (9, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-009.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-009.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-009.webp?fit=683%2C1024&ssl=1'),
  (10, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-010.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-010.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-010.webp?fit=683%2C1024&ssl=1'),
  (11, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-011.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-011.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-011.webp?fit=683%2C1024&ssl=1'),
  (12, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-012.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-012.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-012.webp?fit=683%2C1024&ssl=1'),
  (13, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-013.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-013.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-013.webp?fit=683%2C1024&ssl=1'),
  (14, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-014.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-014.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-014.webp?fit=683%2C1024&ssl=1'),
  (15, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-015.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-015.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-015.webp?fit=683%2C1024&ssl=1'),
  (16, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-016.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-016.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-016.webp?fit=683%2C1024&ssl=1'),
  (17, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-017.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-017.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-017.webp?fit=683%2C1024&ssl=1'),
  (18, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-018.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-018.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-018.webp?fit=683%2C1024&ssl=1'),
  (19, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-019.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-019.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-019.webp?fit=683%2C1024&ssl=1'),
  (20, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-020.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-020.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-020.webp?fit=683%2C1024&ssl=1'),
  (21, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-021.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-021.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-021.webp?fit=683%2C1024&ssl=1'),
  (22, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-022.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-022.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-022.webp?fit=683%2C1024&ssl=1'),
  (23, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-023.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-023.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-023.webp?fit=683%2C1024&ssl=1'),
  (24, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-024.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-024.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-024.webp?fit=683%2C1024&ssl=1'),
  (25, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-025.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-025.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-025.webp?fit=683%2C1024&ssl=1'),
  (26, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-026.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-026.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-026.webp?fit=683%2C1024&ssl=1'),
  (27, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-027.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-027.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-027.webp?fit=683%2C1024&ssl=1'),
  (28, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-028.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-028.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-028.webp?fit=683%2C1024&ssl=1'),
  (29, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-029.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-029.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-029.webp?fit=683%2C1024&ssl=1'),
  (30, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-030.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-030.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-030.webp?fit=683%2C1024&ssl=1'),
  (31, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-031.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-031.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-031.webp?fit=683%2C1024&ssl=1'),
  (32, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-032.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-032.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-032.webp?fit=683%2C1024&ssl=1'),
  (33, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-033.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-033.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-033.webp?fit=683%2C1024&ssl=1'),
  (34, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-034.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-034.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-034.webp?fit=683%2C1024&ssl=1'),
  (35, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-035.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-035.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-035.webp?fit=683%2C1024&ssl=1'),
  (36, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-036.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-036.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-036.webp?fit=683%2C1024&ssl=1'),
  (37, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-037.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-037.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-037.webp?fit=683%2C1024&ssl=1'),
  (38, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-038.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-038.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-038.webp?fit=683%2C1024&ssl=1'),
  (39, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-039.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-039.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-039.webp?fit=683%2C1024&ssl=1'),
  (40, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-040.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-040.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-040.webp?fit=683%2C1024&ssl=1'),
  (41, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-041.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-041.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-041.webp?fit=683%2C1024&ssl=1'),
  (42, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-042.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-042.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-042.webp?fit=683%2C1024&ssl=1'),
  (43, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-043.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-043.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-043.webp?fit=683%2C1024&ssl=1'),
  (44, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-044.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-044.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-044.webp?fit=683%2C1024&ssl=1'),
  (45, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-045.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-045.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-045.webp?fit=683%2C1024&ssl=1'),
  (46, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-046.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-046.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-046.webp?fit=683%2C1024&ssl=1'),
  (47, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-047.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-047.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-047.webp?fit=683%2C1024&ssl=1'),
  (48, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-048.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-048.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-048.webp?fit=683%2C1024&ssl=1'),
  (49, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-049.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-049.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-049.webp?fit=683%2C1024&ssl=1'),
  (50, 'https://ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-050.webp', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-050.webp?fit=200%2C300&ssl=1', 'https://i0.wp.com/ouispeakfashion.com/wp-content/uploads/2026/09/acne-studios-pfw-ss27-look-050.webp?fit=683%2C1024&ssl=1')
)
INSERT OR IGNORE INTO assets
  (id, collection_id, title, source_page_url, remote_url, thumbnail_url, alternative_urls,
   creator, credit_line, source_ids, rights_status, download_policy, display_mode,
   canonical_status, asset_kind, coverage_type, coverage_scope, copyright_holder,
   attribution_required, embed_allowed, remote_render_allowed, rehost_allowed,
   verified_at, sequence_number, look_number, original_url, metadata_json, created_at, updated_at)
SELECT
  'asset-acne-studios-ss27-oui-look-' || printf('%03d', sequence_number),
  'collection-pfw-ss27-acne-studios-2026',
  'Acne Studios SS27 — Oui Speak Fashion look ' || printf('%02d', sequence_number),
  'https://ouispeakfashion.com/acne-studios-spring-summer-2027-collection/',
  remote_url,
  thumbnail_url,
  alternative_url,
  'OSF Team',
  'Courtesy of Acne Studios',
  'source-oui-speak-fashion',
  'UNKNOWN',
  'DOWNLOAD_BLOCKED',
  'INLINE',
  'CANONICAL',
  'IMAGE',
  'RUNWAY',
  'LOOKBOOK_IMAGE',
  'Acne Studios / Oui Speak Fashion',
  1, 0, 1, 0,
  '2026-09-30T00:00:00.000Z',
  sequence_number,
  sequence_number,
  NULL,
  '{"sourceRole":"editorial gallery","photographerStatus":"NOT_PUBLISHED_BY_SOURCE","deduplication":"source filename and final URL","availabilityCheck":"HTTP page and image URLs observed"}',
  '2026-09-30T00:00:00.000Z',
  '2026-09-30T00:00:00.000Z'
FROM image_data;

INSERT OR IGNORE INTO asset_sources (asset_id, source_id, contribution, checked_at)
SELECT id, 'source-oui-speak-fashion',
  'Ordered remote lookbook image; article credits courtesy of Acne Studios; creator OSF Team.',
  '2026-09-30T00:00:00.000Z'
FROM assets
WHERE collection_id = 'collection-pfw-ss27-acne-studios-2026'
  AND source_ids = 'source-oui-speak-fashion'
  AND asset_kind = 'IMAGE';

INSERT OR IGNORE INTO professional_reviews
  (id, collection_id, source_id, title, url, published_at, language, canonical_status, author, publication, summary)
VALUES
  ('review-acne-studios-ss27-oui',
   'collection-pfw-ss27-acne-studios-2026',
   'source-oui-speak-fashion',
   'Acne Studios Spring/Summer 2027 Collection',
   'https://ouispeakfashion.com/acne-studios-spring-summer-2027-collection/',
   '2026-09-30T00:00:00.000Z',
   'en',
   'CANONICAL',
   'OSF Team',
   'Oui Speak Fashion',
   'The source identifies the Paris Fashion Week SS27 presentation and publishes the collection gallery; no translated summary is inferred.');

UPDATE assets
SET original_url = CASE
      WHEN id = 'asset-julie-kegels-ss27-reel' THEN 'https://www.instagram.com/reel/Dd1bejCMjsx/'
      ELSE COALESCE(original_url, source_page_url)
    END,
    embed_url = NULL,
    display_mode = 'LINK_ONLY',
    playback_mode = 'EXTERNAL_LINK',
    availability_status = CASE
      WHEN id = 'asset-julie-kegels-ss27-reel' THEN 'REMOVED'
      ELSE COALESCE(availability_status, 'UNKNOWN')
    END,
    metadata_json = CASE
      WHEN id = 'asset-julie-kegels-ss27-reel' THEN
        '{"availability":"LINK_ONLY","needsAlternativeVideoSource":true,"needsAlternativeReason":"NEEDS_ALTERNATIVE_VIDEO_SOURCE","playbackPolicy":"PROVENANCE_ONLY","accountOrChannel":"nssfrance","creator":"NSS France","retrievedAt":"2026-09-30T00:00:00.000Z","provenance":"Meta URL retained as radar evidence; no reproducible playback alternative verified."}'
      ELSE metadata_json
    END,
    updated_at = '2026-09-30T00:00:00.000Z'
WHERE lower(COALESCE(provider, '')) IN ('instagram', 'facebook');

CREATE TABLE IF NOT EXISTS asset_provenance_audits (
  id TEXT PRIMARY KEY,
  asset_id TEXT NOT NULL REFERENCES assets(id),
  provider TEXT NOT NULL,
  provider_asset_id TEXT,
  original_url TEXT NOT NULL,
  account_or_channel TEXT,
  creator TEXT,
  credit_line TEXT,
  retrieved_at TEXT NOT NULL,
  provenance_json TEXT NOT NULL,
  alternative_status TEXT NOT NULL CHECK (alternative_status IN ('FOUND','NEEDS_ALTERNATIVE_VIDEO_SOURCE','NOT_REQUIRED')),
  alternative_source_id TEXT REFERENCES sources(id),
  alternative_url TEXT,
  created_at TEXT NOT NULL
);

INSERT OR IGNORE INTO asset_provenance_audits
  (id, asset_id, provider, provider_asset_id, original_url, account_or_channel, creator, credit_line, retrieved_at, provenance_json, alternative_status, created_at)
VALUES
  ('audit-julie-kegels-instagram-reel',
   'asset-julie-kegels-ss27-reel',
   'instagram',
   'Dd1bejCMjsx',
   'https://www.instagram.com/reel/Dd1bejCMjsx/',
   'nssfrance',
   'NSS France',
   NULL,
   '2026-09-30T00:00:00.000Z',
   '{"role":"radar_only","availability":"REMOVED","playback":"not_reproducible","originalSourcePage":"https://www.nssmag.com/en/fashion/47057/julie-kegels-runway-show-spring-summer-2027-paris-fashion-week"}',
   'NEEDS_ALTERNATIVE_VIDEO_SOURCE',
   '2026-09-30T00:00:00.000Z');

WITH source_specs(source_key, source_id, source_page_url) AS (
  VALUES
    ('FHCM', 'source-fhcm', 'https://www.fhcm.paris/en/paris-fashion-week/calendar'),
    ('VOGUE', 'source-vogue-runway', 'https://www.vogue.com/fashion-shows'),
    ('MAISON', NULL, NULL),
    ('NOWFASHION', 'source-nowfashion', 'https://nowfashion.com/season/spring-summer'),
    ('OUI', 'source-oui-speak-fashion', 'https://ouispeakfashion.com/'),
    ('TAGWALK', 'source-tagwalk', 'https://www.tag-walk.com/en/'),
    ('LAUNCHMETRICS', NULL, 'https://www.launchmetrics.com/'),
    ('YOUTUBE', NULL, 'https://www.youtube.com/'),
    ('FF_CHANNEL', 'source-ff-channel', 'https://www.youtube.com/@FFChannel'),
    ('FASHION_CHANNEL', 'source-fashion-channel', 'https://www.youtube.com/@FashionChannel'),
    ('OTHER', NULL, 'https://www.fhcm.paris/en/paris-fashion-week/calendar')
),
media_types(media_type) AS (VALUES ('IMAGE'), ('VIDEO'))
INSERT OR IGNORE INTO media_research_matrix
  (id, collection_id, source_key, source_id, media_type, state, checked_at, source_page_url, result_count, reason, metadata_json)
SELECT
  'matrix-' || c.id || '-' || lower(spec.source_key) || '-' || lower(mt.media_type),
  c.id,
  spec.source_key,
  spec.source_id,
  mt.media_type,
  CASE WHEN COUNT(a.id) > 0 THEN 'FOUND' ELSE 'NEEDS_RESEARCH' END,
  '2026-09-30T00:00:00.000Z',
  CASE WHEN spec.source_key = 'MAISON' THEN m.website_url ELSE spec.source_page_url END,
  COUNT(a.id),
  CASE
    WHEN COUNT(a.id) > 0 THEN 'Canonical D1 asset relation found for this Collection/source/media type.'
    WHEN spec.source_key = 'MAISON' AND m.website_url IS NULL THEN 'No verified Maison URL is registered for this Maison; no URL was invented. Maison discovery remains pending at the Collection/Maison registry boundary.'
    ELSE 'Registered research entrypoint checked for this matrix row; no verified asset relation is present in canonical D1 at this audit time. Source-specific collector remains required.'
  END,
  json_object('scope','PFW_SS27_MEDIA_COMPLETENESS','researchOrder',CASE spec.source_key WHEN 'MAISON' THEN 1 WHEN 'FHCM' THEN 3 WHEN 'VOGUE' THEN 4 WHEN 'NOWFASHION' THEN 5 WHEN 'OUI' THEN 6 WHEN 'TAGWALK' THEN 7 WHEN 'LAUNCHMETRICS' THEN 8 WHEN 'YOUTUBE' THEN 10 WHEN 'FF_CHANNEL' THEN 12 WHEN 'FASHION_CHANNEL' THEN 13 ELSE 14 END)
FROM collections c
JOIN maisons m ON m.id = c.maison_id
CROSS JOIN source_specs spec
CROSS JOIN media_types mt
LEFT JOIN assets a
  ON a.collection_id = c.id
 AND a.asset_kind = mt.media_type
 AND (
   (spec.source_key = 'FHCM' AND instr(',' || a.source_ids || ',', ',source-fhcm,') > 0)
   OR (spec.source_key = 'VOGUE' AND instr(',' || a.source_ids || ',', ',source-vogue-runway,') > 0)
   OR (spec.source_key = 'OUI' AND instr(',' || a.source_ids || ',', ',source-oui-speak-fashion,') > 0)
   OR (spec.source_key IN ('NOWFASHION','TAGWALK','FF_CHANNEL','FASHION_CHANNEL') AND instr(',' || a.source_ids || ',', ',' || spec.source_id || ',') > 0)
   OR (spec.source_key = 'YOUTUBE' AND EXISTS (SELECT 1 FROM sources ys WHERE ys.id IN (SELECT value FROM json_each('["' || replace(a.source_ids, ',', '","') || '"]')) AND ys.type = 'VIDEO_CHANNEL'))
   OR (spec.source_key = 'MAISON' AND EXISTS (SELECT 1 FROM sources ms WHERE ms.id IN (SELECT value FROM json_each('["' || replace(a.source_ids, ',', '","') || '"]')) AND ms.type = 'MAISON_OFFICIAL'))
 )
WHERE c.edition_id = 'edition-pfw-womenswear-ss27-2026'
GROUP BY c.id, spec.source_key, spec.source_id, spec.source_page_url, m.website_url, mt.media_type;

-- A FOUND cell points to the exact canonical page used by its asset relation,
-- rather than only the source registry home.
UPDATE media_research_matrix AS matrix
SET source_page_url = (
  SELECT MIN(a.source_page_url)
  FROM assets a
  WHERE a.collection_id = matrix.collection_id
    AND a.asset_kind = matrix.media_type
    AND (
      (matrix.source_key = 'FHCM' AND instr(',' || a.source_ids || ',', ',source-fhcm,') > 0)
      OR (matrix.source_key = 'VOGUE' AND instr(',' || a.source_ids || ',', ',source-vogue-runway,') > 0)
      OR (matrix.source_key = 'OUI' AND instr(',' || a.source_ids || ',', ',source-oui-speak-fashion,') > 0)
      OR (matrix.source_key IN ('NOWFASHION','TAGWALK','FF_CHANNEL','FASHION_CHANNEL') AND instr(',' || a.source_ids || ',', ',' || matrix.source_id || ',') > 0)
    )
)
WHERE matrix.state = 'FOUND';

CREATE INDEX IF NOT EXISTS idx_assets_original_url ON assets(original_url);
