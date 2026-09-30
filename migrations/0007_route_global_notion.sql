-- Route Global da Moda import contract.
-- The rows below are generated from the checked-in Notion snapshot by
-- scripts/import-route-global.ts. Notion page IDs and source hashes make the
-- import safe to re-run; no editorial count is encoded in this migration.

ALTER TABLE city_hubs ADD COLUMN country_name TEXT;
ALTER TABLE city_hubs ADD COLUMN country_code TEXT;
ALTER TABLE city_hubs ADD COLUMN subregion TEXT;
ALTER TABLE city_hubs ADD COLUMN aliases_json TEXT NOT NULL DEFAULT '[]';
ALTER TABLE city_hubs ADD COLUMN related_event_ids TEXT NOT NULL DEFAULT '';
ALTER TABLE city_hubs ADD COLUMN research_status TEXT NOT NULL DEFAULT 'UNVERIFIED';
ALTER TABLE city_hubs ADD COLUMN hub_importance TEXT;
ALTER TABLE city_hubs ADD COLUMN primary_source_id TEXT;
ALTER TABLE city_hubs ADD COLUMN complementary_source_ids TEXT NOT NULL DEFAULT '';
ALTER TABLE city_hubs ADD COLUMN notes TEXT;
ALTER TABLE city_hubs ADD COLUMN cover_asset_key TEXT;
ALTER TABLE city_hubs ADD COLUMN cover_url TEXT;
ALTER TABLE city_hubs ADD COLUMN cover_match_status TEXT NOT NULL DEFAULT 'MISSING';
ALTER TABLE city_hubs ADD COLUMN cover_fallback INTEGER NOT NULL DEFAULT 1;
ALTER TABLE city_hubs ADD COLUMN notion_page_id TEXT;
ALTER TABLE city_hubs ADD COLUMN notion_url TEXT;
ALTER TABLE city_hubs ADD COLUMN notion_last_edited_at TEXT;
ALTER TABLE city_hubs ADD COLUMN source_hash TEXT;
ALTER TABLE city_hubs ADD COLUMN last_imported_at TEXT;
ALTER TABLE city_hubs ADD COLUMN import_status TEXT NOT NULL DEFAULT 'REVIEW_REQUIRED';

ALTER TABLE events ADD COLUMN event_type TEXT;
ALTER TABLE events ADD COLUMN aliases_json TEXT NOT NULL DEFAULT '[]';
ALTER TABLE events ADD COLUMN city_hub_ids TEXT NOT NULL DEFAULT '';
ALTER TABLE events ADD COLUMN current_status TEXT;
ALTER TABLE events ADD COLUMN known_start_year INTEGER;
ALTER TABLE events ADD COLUMN known_end_year INTEGER;
ALTER TABLE events ADD COLUMN active_since_2010 INTEGER;
ALTER TABLE events ADD COLUMN organizer TEXT;
ALTER TABLE events ADD COLUMN usual_period TEXT;
ALTER TABLE events ADD COLUMN last_verified_year INTEGER;
ALTER TABLE events ADD COLUMN next_edition_announced_json TEXT;
ALTER TABLE events ADD COLUMN historical_relation TEXT;
ALTER TABLE events ADD COLUMN about TEXT;
ALTER TABLE events ADD COLUMN history_summary TEXT;
ALTER TABLE events ADD COLUMN verified_summary_at TEXT;
ALTER TABLE events ADD COLUMN notes TEXT;
ALTER TABLE events ADD COLUMN socials_json TEXT NOT NULL DEFAULT '{}';
ALTER TABLE events ADD COLUMN primary_source_id TEXT;
ALTER TABLE events ADD COLUMN complementary_source_ids TEXT NOT NULL DEFAULT '';
ALTER TABLE events ADD COLUMN cover_asset_key TEXT;
ALTER TABLE events ADD COLUMN cover_url TEXT;
ALTER TABLE events ADD COLUMN cover_match_status TEXT NOT NULL DEFAULT 'MISSING';
ALTER TABLE events ADD COLUMN cover_fallback INTEGER NOT NULL DEFAULT 1;
ALTER TABLE events ADD COLUMN research_status TEXT NOT NULL DEFAULT 'UNVERIFIED';
ALTER TABLE events ADD COLUMN notion_page_id TEXT;
ALTER TABLE events ADD COLUMN notion_url TEXT;
ALTER TABLE events ADD COLUMN notion_last_edited_at TEXT;
ALTER TABLE events ADD COLUMN source_hash TEXT;
ALTER TABLE events ADD COLUMN last_imported_at TEXT;
ALTER TABLE events ADD COLUMN import_status TEXT NOT NULL DEFAULT 'REVIEW_REQUIRED';

CREATE UNIQUE INDEX IF NOT EXISTS idx_city_hubs_notion_page ON city_hubs(notion_page_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_events_notion_page ON events(notion_page_id);
CREATE INDEX IF NOT EXISTS idx_city_hubs_route_region ON city_hubs(region_id, country_id, research_status, cover_fallback);
CREATE INDEX IF NOT EXISTS idx_events_route_status ON events(current_status, event_type, research_status, cover_fallback);
CREATE INDEX IF NOT EXISTS idx_event_locations_city ON event_locations(city_hub_id, event_id);

