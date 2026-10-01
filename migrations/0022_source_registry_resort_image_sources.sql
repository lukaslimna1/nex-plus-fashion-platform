-- Register user-requested Resort 2027 image sources without scraping or rehosting media.
INSERT OR IGNORE INTO sources
  (id, canonical_name, type, base_url, authority_tier, language, coverage_scope, access_mode,
   rights_notes, automation_notes, last_verified_at, active, created_at, updated_at)
VALUES
  ('source-wwd', 'WWD', 'RUNWAY_COVERAGE', 'https://wwd.com/runway/resort-2027/paris/', 'C', 'en',
   'resort,paris,runway,images,credits', 'PAYWALLED',
   'WWD and its licensors retain editorial/image rights; no download or rehost permission is inferred.',
   'HTTP 402 Payment Required observed on 2026-09-30. Use as provenance/link-only until access or license is available.',
   '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-vogue-resort', 'Vogue Runway', 'RUNWAY_COVERAGE', 'https://www.vogue.com/fashion-shows/resort-2027', 'C', 'en',
   'resort,collections,runway,images,credits', 'PUBLIC',
   'Vogue and its licensors retain editorial/image rights; no download or rehost permission is inferred.',
   'Use the index as sourcePageUrl/provenance and require item-level credit and rights evidence before any ingestion.',
   '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO source_checks (source_id, last_checked_at, last_changed_at)
VALUES
  ('source-wwd', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-vogue-resort', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');
