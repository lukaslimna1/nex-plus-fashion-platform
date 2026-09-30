CREATE TABLE IF NOT EXISTS regions (
  id TEXT PRIMARY KEY, name TEXT NOT NULL, slug TEXT NOT NULL UNIQUE,
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS countries (
  id TEXT PRIMARY KEY, region_id TEXT NOT NULL REFERENCES regions(id), name TEXT NOT NULL, iso_code TEXT NOT NULL UNIQUE, slug TEXT NOT NULL UNIQUE,
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS administrative_areas (
  id TEXT PRIMARY KEY, country_id TEXT NOT NULL REFERENCES countries(id), name TEXT NOT NULL, slug TEXT NOT NULL,
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS city_hubs (
  id TEXT PRIMARY KEY, country_id TEXT NOT NULL REFERENCES countries(id), region_id TEXT NOT NULL REFERENCES regions(id), administrative_area_id TEXT REFERENCES administrative_areas(id),
  name TEXT NOT NULL, slug TEXT NOT NULL UNIQUE, timezone TEXT NOT NULL, latitude REAL, longitude REAL,
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS events (
  id TEXT PRIMARY KEY, name TEXT NOT NULL, slug TEXT NOT NULL UNIQUE, kind TEXT NOT NULL CHECK (kind IN ('FASHION_WEEK','HAUTE_COUTURE_WEEK','OTHER')), official_url TEXT,
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS event_locations (
  event_id TEXT NOT NULL REFERENCES events(id), city_hub_id TEXT NOT NULL REFERENCES city_hubs(id), venue_name TEXT, address TEXT, source_id TEXT,
  PRIMARY KEY (event_id, city_hub_id)
);
CREATE TABLE IF NOT EXISTS segments (
  id TEXT PRIMARY KEY, name TEXT NOT NULL, code TEXT NOT NULL UNIQUE
);
CREATE TABLE IF NOT EXISTS event_segments (
  event_id TEXT NOT NULL REFERENCES events(id), segment_id TEXT NOT NULL REFERENCES segments(id), PRIMARY KEY (event_id, segment_id)
);
CREATE TABLE IF NOT EXISTS editions (
  id TEXT PRIMARY KEY, event_id TEXT NOT NULL REFERENCES events(id), segment_id TEXT REFERENCES segments(id), city_hub_id TEXT REFERENCES city_hubs(id),
  calendar_year INTEGER NOT NULL, season_year INTEGER NOT NULL, season_code TEXT NOT NULL, season_label TEXT NOT NULL, starts_on TEXT, ends_on TEXT,
  status TEXT NOT NULL CHECK (status IN ('DRAFT','PENDING_REVIEW','VALIDATED','REJECTED','CANONICAL')),
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS sources (
  id TEXT PRIMARY KEY, canonical_name TEXT NOT NULL, type TEXT NOT NULL, base_url TEXT NOT NULL, authority_tier TEXT NOT NULL,
  language TEXT NOT NULL DEFAULT '', coverage_scope TEXT NOT NULL DEFAULT '', region_id TEXT REFERENCES regions(id), country_id TEXT REFERENCES countries(id),
  access_mode TEXT NOT NULL DEFAULT 'UNKNOWN', rights_notes TEXT, automation_notes TEXT, last_verified_at TEXT, active INTEGER NOT NULL DEFAULT 1,
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS schedule_entries (
  id TEXT PRIMARY KEY, edition_id TEXT NOT NULL REFERENCES editions(id), event_id TEXT NOT NULL REFERENCES events(id), segment_id TEXT REFERENCES segments(id), city_hub_id TEXT NOT NULL REFERENCES city_hubs(id),
  title TEXT NOT NULL, format TEXT NOT NULL CHECK (format IN ('SHOW','PRESENTATION','FILM','OTHER')), start_time TEXT NOT NULL, end_time TEXT, timezone TEXT NOT NULL,
  verification_status TEXT NOT NULL CHECK (verification_status IN ('VERIFIED','UNVERIFIED','CANCELLED','UNKNOWN')), official_url TEXT, livestream_url TEXT, source_ids TEXT NOT NULL DEFAULT '',
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS maisons (
  id TEXT PRIMARY KEY, name TEXT NOT NULL, slug TEXT NOT NULL UNIQUE, website_url TEXT, founded_year INTEGER, artistic_direction TEXT, official_source_ids TEXT NOT NULL DEFAULT '',
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS creative_direction_history (
  id TEXT PRIMARY KEY, maison_id TEXT NOT NULL REFERENCES maisons(id), person_or_team TEXT NOT NULL, starts_on TEXT, ends_on TEXT, source_ids TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS collections (
  id TEXT PRIMARY KEY, maison_id TEXT NOT NULL REFERENCES maisons(id), edition_id TEXT NOT NULL REFERENCES editions(id), name TEXT NOT NULL, slug TEXT NOT NULL UNIQUE,
  calendar_year INTEGER NOT NULL, season_year INTEGER NOT NULL, season_code TEXT NOT NULL, season_label TEXT NOT NULL, presented_on TEXT,
  canonical_status TEXT NOT NULL CHECK (canonical_status IN ('DRAFT','PENDING_REVIEW','VALIDATED','REJECTED','CANONICAL')), source_ids TEXT NOT NULL DEFAULT '',
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS assets (
  id TEXT PRIMARY KEY, collection_id TEXT REFERENCES collections(id), title TEXT, source_page_url TEXT NOT NULL, remote_url TEXT, embed_url TEXT, provider TEXT, provider_asset_id TEXT, thumbnail_url TEXT,
  alternative_urls TEXT NOT NULL DEFAULT '', creator TEXT, photographer TEXT, credit_line TEXT, source_ids TEXT NOT NULL DEFAULT '',
  rights_status TEXT NOT NULL CHECK (rights_status IN ('CLEARED','RESTRICTED','UNKNOWN','NOT_APPLICABLE')), download_policy TEXT NOT NULL CHECK (download_policy IN ('DOWNLOAD_ALLOWED','DOWNLOAD_BLOCKED','DOWNLOAD_UNKNOWN')),
  display_mode TEXT NOT NULL CHECK (display_mode IN ('INLINE','EMBED','LINK_ONLY','THUMBNAIL_ONLY','PLACEHOLDER')), local_path TEXT, cache_url TEXT,
  canonical_status TEXT NOT NULL CHECK (canonical_status IN ('DRAFT','PENDING_REVIEW','VALIDATED','REJECTED','CANONICAL')),
  revision INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS asset_sources (
  asset_id TEXT NOT NULL REFERENCES assets(id), source_id TEXT NOT NULL REFERENCES sources(id), contribution TEXT NOT NULL, checked_at TEXT NOT NULL, PRIMARY KEY (asset_id, source_id)
);
CREATE TABLE IF NOT EXISTS professional_reviews (
  id TEXT PRIMARY KEY, collection_id TEXT REFERENCES collections(id), source_id TEXT NOT NULL REFERENCES sources(id), title TEXT NOT NULL, url TEXT NOT NULL, published_at TEXT, language TEXT NOT NULL, canonical_status TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS terms (id TEXT PRIMARY KEY, value TEXT NOT NULL, language TEXT NOT NULL, definition TEXT, source_ids TEXT NOT NULL DEFAULT '');
CREATE TABLE IF NOT EXISTS tags (id TEXT PRIMARY KEY, value TEXT NOT NULL, slug TEXT NOT NULL UNIQUE, source_ids TEXT NOT NULL DEFAULT '');
CREATE TABLE IF NOT EXISTS collection_tags (collection_id TEXT NOT NULL REFERENCES collections(id), tag_id TEXT NOT NULL REFERENCES tags(id), evidence TEXT, PRIMARY KEY (collection_id, tag_id));
CREATE TABLE IF NOT EXISTS asset_tags (asset_id TEXT NOT NULL REFERENCES assets(id), tag_id TEXT NOT NULL REFERENCES tags(id), evidence TEXT, PRIMARY KEY (asset_id, tag_id));
CREATE TABLE IF NOT EXISTS look_tags (asset_id TEXT NOT NULL REFERENCES assets(id), tag_id TEXT NOT NULL REFERENCES tags(id), look_number INTEGER, PRIMARY KEY (asset_id, tag_id));
CREATE TABLE IF NOT EXISTS favorites (
  id TEXT PRIMARY KEY, identity_key TEXT NOT NULL, target_type TEXT NOT NULL, target_id TEXT NOT NULL, created_at TEXT NOT NULL,
  UNIQUE(identity_key, target_type, target_id)
);
CREATE TABLE IF NOT EXISTS community_reactions (
  id TEXT PRIMARY KEY, identity_key TEXT NOT NULL, target_type TEXT NOT NULL, target_id TEXT NOT NULL, reaction TEXT NOT NULL, created_at TEXT NOT NULL,
  UNIQUE(identity_key, target_type, target_id, reaction)
);
CREATE TABLE IF NOT EXISTS community_tags (
  id TEXT PRIMARY KEY, identity_key TEXT NOT NULL, target_type TEXT NOT NULL, target_id TEXT NOT NULL, tag_id TEXT NOT NULL REFERENCES tags(id), created_at TEXT NOT NULL,
  UNIQUE(identity_key, target_type, target_id, tag_id)
);
CREATE TABLE IF NOT EXISTS sync_records (entity_type TEXT NOT NULL, entity_id TEXT NOT NULL, revision INTEGER NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT, PRIMARY KEY (entity_type, entity_id));
CREATE TABLE IF NOT EXISTS sync_tombstones (entity_type TEXT NOT NULL, entity_id TEXT NOT NULL, revision INTEGER NOT NULL, deleted_at TEXT NOT NULL, PRIMARY KEY (entity_type, entity_id));
CREATE TABLE IF NOT EXISTS source_checks (
  source_id TEXT PRIMARY KEY REFERENCES sources(id), etag TEXT, last_modified TEXT, content_hash TEXT, last_checked_at TEXT, last_changed_at TEXT
);
CREATE TABLE IF NOT EXISTS ai_runs (
  id TEXT PRIMARY KEY, task TEXT NOT NULL, provider TEXT NOT NULL, model TEXT NOT NULL, generated_at TEXT NOT NULL, input_source_ids TEXT NOT NULL,
  confidence REAL, schema_version TEXT NOT NULL, status TEXT NOT NULL, raw_output TEXT NOT NULL, validated_output TEXT
);
CREATE INDEX IF NOT EXISTS idx_schedule_start ON schedule_entries(start_time);
CREATE INDEX IF NOT EXISTS idx_collections_edition ON collections(edition_id);
CREATE INDEX IF NOT EXISTS idx_assets_collection ON assets(collection_id);
