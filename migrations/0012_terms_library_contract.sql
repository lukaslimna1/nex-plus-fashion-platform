-- Additive Term contract for the backend Library. Existing Term rows remain canonical;
-- nullable enrichment fields are only populated when a verified source provides them.
ALTER TABLE terms ADD COLUMN slug TEXT;
ALTER TABLE terms ADD COLUMN canonical_name TEXT;
ALTER TABLE terms ADD COLUMN pt_br_name TEXT;
ALTER TABLE terms ADD COLUMN international_name TEXT;
ALTER TABLE terms ADD COLUMN aliases_json TEXT NOT NULL DEFAULT '[]';
ALTER TABLE terms ADD COLUMN context TEXT;
ALTER TABLE terms ADD COLUMN category TEXT;
ALTER TABLE terms ADD COLUMN external_uri TEXT;
ALTER TABLE terms ADD COLUMN related_term_ids TEXT NOT NULL DEFAULT '';
ALTER TABLE terms ADD COLUMN examples_json TEXT NOT NULL DEFAULT '[]';
ALTER TABLE terms ADD COLUMN retrieved_at TEXT;

UPDATE terms
SET slug = CASE id WHEN 'term-izihlwele-zivumile' THEN 'izihlwele-zivumile' ELSE id END,
    canonical_name = value,
    international_name = COALESCE(international_name, definition)
WHERE slug IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_terms_slug ON terms(slug);
CREATE INDEX IF NOT EXISTS idx_terms_category ON terms(category);

INSERT OR IGNORE INTO sources (id, canonical_name, type, base_url, authority_tier, language, coverage_scope, access_mode, rights_notes, automation_notes, last_verified_at, active, created_at, updated_at)
VALUES ('source-getty-aat', 'Getty Art & Architecture Thesaurus', 'VOCABULARY', 'https://vocab.getty.edu/', 'B', 'en', 'fashion-vocabulary,materials,techniques,styles', 'PUBLIC', 'Getty Vocabularies data is available under ODC-By 1.0; preserve URI and attribution.', 'Use SPARQL/LOD; do not scrape the visual vocabulary pages.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO terms (id, value, language, definition, source_ids, slug, canonical_name, international_name, category, external_uri, retrieved_at)
VALUES
  ('term-getty-knitting-process', 'knitting (process)', 'en', NULL, 'source-getty-aat', 'knitting-process', 'knitting (process)', 'knitting (process)', 'technique', 'http://vocab.getty.edu/aat/300053634', '2026-09-30T00:00:00.000Z'),
  ('term-getty-machine-knitting', 'machine knitting', 'en', NULL, 'source-getty-aat', 'machine-knitting', 'machine knitting', 'machine knitting', 'technique', 'http://vocab.getty.edu/aat/300252068', '2026-09-30T00:00:00.000Z');
