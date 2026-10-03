CREATE TABLE IF NOT EXISTS source_registry (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    source_kind TEXT NOT NULL,
    authority_tier TEXT NOT NULL CHECK (authority_tier IN ('A', 'B', 'C', 'D', 'E')),
    base_url TEXT NOT NULL,
    access_mode TEXT NOT NULL DEFAULT 'remote_render',
    status TEXT NOT NULL DEFAULT 'active',
    terms_url TEXT,
    rights_notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS source_endpoint (
    id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL REFERENCES source_registry(id) ON DELETE CASCADE,
    endpoint_type TEXT NOT NULL,
    base_url TEXT NOT NULL,
    access_method TEXT NOT NULL,
    capabilities_json TEXT NOT NULL DEFAULT '[]',
    adapter_id TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    last_verified_at TEXT,
    UNIQUE (source_id, endpoint_type, base_url)
);

CREATE TABLE IF NOT EXISTS geo_region (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    m49_code TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS geo_subregion (
    id TEXT PRIMARY KEY,
    region_id TEXT NOT NULL REFERENCES geo_region(id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    m49_code TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS geo_country (
    id TEXT PRIMARY KEY,
    subregion_id TEXT REFERENCES geo_subregion(id) ON DELETE RESTRICT,
    region_id TEXT REFERENCES geo_region(id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    iso_alpha2 TEXT NOT NULL,
    iso_alpha3 TEXT,
    m49_code TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (iso_alpha2)
);

CREATE TABLE IF NOT EXISTS geo_admin_division (
    id TEXT PRIMARY KEY,
    country_id TEXT NOT NULL REFERENCES geo_country(id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    code TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS geo_city (
    id TEXT PRIMARY KEY,
    country_id TEXT NOT NULL REFERENCES geo_country(id) ON DELETE RESTRICT,
    admin_division_id TEXT REFERENCES geo_admin_division(id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    aliases_json TEXT NOT NULL DEFAULT '[]',
    latitude REAL,
    longitude REAL,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS city_hub (
    id TEXT PRIMARY KEY,
    city_id TEXT NOT NULL REFERENCES geo_city(id) ON DELETE RESTRICT,
    official_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    hub_type TEXT NOT NULL DEFAULT 'fashion_city',
    status TEXT NOT NULL DEFAULT 'active',
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS catalog_event (
    id TEXT PRIMARY KEY,
    city_hub_id TEXT NOT NULL REFERENCES city_hub(id) ON DELETE RESTRICT,
    official_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    short_name TEXT,
    event_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    start_year INTEGER,
    official_website TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS catalog_segment (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS event_segment (
    event_id TEXT NOT NULL REFERENCES catalog_event(id) ON DELETE CASCADE,
    segment_id TEXT NOT NULL REFERENCES catalog_segment(id) ON DELETE RESTRICT,
    source_id TEXT REFERENCES source_registry(id),
    PRIMARY KEY (event_id, segment_id)
);

CREATE TABLE IF NOT EXISTS catalog_edition (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES catalog_event(id) ON DELETE RESTRICT,
    segment_id TEXT REFERENCES catalog_segment(id) ON DELETE RESTRICT,
    display_name TEXT NOT NULL,
    season TEXT NOT NULL,
    season_code TEXT NOT NULL,
    season_year INTEGER NOT NULL,
    calendar_year INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    time_zone TEXT NOT NULL,
    edition_status TEXT NOT NULL CHECK (edition_status IN ('announced', 'ongoing', 'postponed', 'cancelled', 'finished')),
    official_page_url TEXT,
    official_calendar_url TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS catalog_venue (
    id TEXT PRIMARY KEY,
    city_id TEXT NOT NULL REFERENCES geo_city(id) ON DELETE RESTRICT,
    name TEXT,
    address TEXT,
    venue_type TEXT NOT NULL DEFAULT 'calendar_location',
    latitude REAL,
    longitude REAL,
    official_website TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (city_id, name, address)
);

CREATE TABLE IF NOT EXISTS participant_registry (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    normalized_name TEXT NOT NULL UNIQUE,
    canonical_kind TEXT,
    reconciliation_status TEXT NOT NULL DEFAULT 'unreconciled' CHECK (reconciliation_status IN ('unreconciled', 'candidate', 'reconciled', 'rejected')),
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS catalog_maison (
    id TEXT PRIMARY KEY,
    official_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL DEFAULT 'active',
    official_website TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS catalog_person (
    id TEXT PRIMARY KEY,
    full_name TEXT NOT NULL,
    display_name TEXT,
    biography TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS catalog_role (
    id TEXT PRIMARY KEY,
    name_pt_br TEXT NOT NULL,
    international_name TEXT,
    description TEXT,
    role_category TEXT,
    aliases_json TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS person_role (
    id TEXT PRIMARY KEY,
    person_id TEXT NOT NULL REFERENCES catalog_person(id) ON DELETE CASCADE,
    role_id TEXT NOT NULL REFERENCES catalog_role(id) ON DELETE RESTRICT,
    context_entity_type TEXT NOT NULL,
    context_entity_id TEXT NOT NULL,
    official_role_title TEXT,
    start_date TEXT,
    end_date TEXT,
    is_current INTEGER NOT NULL DEFAULT 0 CHECK (is_current IN (0, 1)),
    source_id TEXT REFERENCES source_registry(id),
    notes TEXT,
    UNIQUE (person_id, role_id, context_entity_type, context_entity_id, start_date)
);

CREATE TABLE IF NOT EXISTS catalog_collection (
    id TEXT PRIMARY KEY,
    maison_id TEXT REFERENCES catalog_maison(id) ON DELETE RESTRICT,
    edition_id TEXT REFERENCES catalog_edition(id) ON DELETE RESTRICT,
    schedule_entry_id TEXT REFERENCES schedule_entry(id) ON DELETE RESTRICT,
    display_name TEXT NOT NULL,
    about TEXT,
    presented_at TEXT,
    presentation_format TEXT,
    season TEXT,
    season_year INTEGER,
    official_collection_url TEXT,
    press_release_url TEXT,
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS schedule_entry (
    id TEXT PRIMARY KEY,
    edition_id TEXT NOT NULL REFERENCES catalog_edition(id) ON DELETE CASCADE,
    participant_id TEXT NOT NULL REFERENCES participant_registry(id) ON DELETE RESTRICT,
    maison_id TEXT REFERENCES catalog_maison(id) ON DELETE RESTRICT,
    local_date TEXT NOT NULL,
    start_time_local TEXT,
    end_time_local TEXT,
    time_zone TEXT NOT NULL,
    format TEXT NOT NULL CHECK (format IN ('fashion_show', 'presentation', 'presentation_by_appointment', 'event')),
    schedule_status TEXT NOT NULL CHECK (schedule_status IN ('scheduled', 'live', 'completed', 'postponed', 'cancelled')),
    location_status TEXT NOT NULL CHECK (location_status IN ('published', 'invitation', 'not_published')),
    delivery_mode TEXT NOT NULL DEFAULT 'unknown' CHECK (delivery_mode IN ('physical', 'digital', 'hybrid', 'unknown')),
    venue_id TEXT REFERENCES catalog_venue(id) ON DELETE RESTRICT,
    venue_label TEXT,
    official_stream_url TEXT,
    official_entry_url TEXT,
    official_note TEXT,
    source_id TEXT NOT NULL REFERENCES source_registry(id) ON DELETE RESTRICT,
    source_external_id TEXT NOT NULL,
    source_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (edition_id, source_id, source_external_id)
);

CREATE TABLE IF NOT EXISTS catalog_media_asset (
    id TEXT PRIMARY KEY,
    media_type TEXT NOT NULL,
    title TEXT,
    remote_render_policy TEXT NOT NULL DEFAULT 'remote_render',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS media_occurrence (
    id TEXT PRIMARY KEY,
    media_asset_id TEXT REFERENCES catalog_media_asset(id) ON DELETE SET NULL,
    source_id TEXT NOT NULL REFERENCES source_registry(id) ON DELETE RESTRICT,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    remote_url TEXT NOT NULL,
    page_url TEXT,
    source_asset_key TEXT,
    source_sequence TEXT,
    credit TEXT,
    asset_health TEXT NOT NULL DEFAULT 'unknown',
    verified_at TEXT,
    raw_label TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (source_id, entity_type, entity_id, remote_url)
);

CREATE TABLE IF NOT EXISTS catalog_review (
    id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL REFERENCES source_registry(id) ON DELETE RESTRICT,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    author TEXT,
    title TEXT,
    published_at TEXT,
    language TEXT,
    original_url TEXT NOT NULL,
    summary TEXT,
    key_points_json TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (source_id, original_url)
);

CREATE TABLE IF NOT EXISTS source_contribution (
    id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL REFERENCES source_registry(id) ON DELETE CASCADE,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    contributed_fields_json TEXT NOT NULL,
    evidence_url TEXT NOT NULL,
    retrieved_at TEXT NOT NULL,
    evidence_status TEXT NOT NULL DEFAULT 'verified',
    notes TEXT,
    adapter_id TEXT,
    UNIQUE (source_id, entity_type, entity_id, evidence_url)
);

CREATE TABLE IF NOT EXISTS catalog_gap (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    field_name TEXT NOT NULL,
    reason TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'resolved', 'not_applicable')),
    source_id TEXT REFERENCES source_registry(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (entity_type, entity_id, field_name)
);

CREATE TABLE IF NOT EXISTS pack_installation (
    id TEXT PRIMARY KEY,
    pack_id TEXT NOT NULL,
    pack_version TEXT NOT NULL,
    manifest_json TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    installed_at TEXT NOT NULL,
    UNIQUE (pack_id, pack_version)
);

CREATE INDEX IF NOT EXISTS idx_schedule_entry_edition_day ON schedule_entry(edition_id, local_date, start_time_local);
CREATE INDEX IF NOT EXISTS idx_schedule_entry_maison ON schedule_entry(maison_id);
CREATE INDEX IF NOT EXISTS idx_source_contribution_entity ON source_contribution(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_catalog_gap_entity ON catalog_gap(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_media_occurrence_entity ON media_occurrence(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_review_entity ON catalog_review(entity_type, entity_id);
