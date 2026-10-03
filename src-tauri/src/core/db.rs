use crate::core::error::CoreError;
use crate::core::milano::{bundled_pack, MilanoSeed, NexPack};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Serialize;
use std::path::Path;

const MIGRATIONS: [(&str, &str); 4] = [
    ("0001_core", include_str!("../../migrations/0001_core.sql")),
    ("0002_fts5", include_str!("../../migrations/0002_fts5.sql")),
    (
        "0003_milano_vertical",
        include_str!("../../migrations/0003_milano_vertical.sql"),
    ),
    (
        "0004_pack_runtime",
        include_str!("../../migrations/0004_pack_runtime.sql"),
    ),
];

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MilanoSnapshot {
    pub city_hub: CityHubRow,
    pub event: EventRow,
    pub edition: EditionRow,
    pub schedule: Vec<ScheduleRow>,
    pub participant_count: i64,
    pub unresolved_participant_count: i64,
    pub source: SourceRow,
    pub gaps: Vec<GapRow>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CityHubRow {
    pub id: String,
    pub city_id: String,
    pub official_name: String,
    pub display_name: String,
    pub slug: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRow {
    pub id: String,
    pub city_hub_id: String,
    pub official_name: String,
    pub display_name: String,
    pub short_name: Option<String>,
    pub event_type: String,
    pub status: String,
    pub official_website: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditionRow {
    pub id: String,
    pub event_id: String,
    pub segment_id: Option<String>,
    pub display_name: String,
    pub season: String,
    pub season_code: String,
    pub season_year: i32,
    pub calendar_year: i32,
    pub start_date: String,
    pub end_date: String,
    pub time_zone: String,
    pub edition_status: String,
    pub official_page_url: Option<String>,
    pub official_calendar_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRow {
    pub id: String,
    pub edition_id: String,
    pub participant_id: String,
    pub participant_name_raw: String,
    pub maison_id: Option<String>,
    pub local_date: String,
    pub start_time_local: Option<String>,
    pub end_time_local: Option<String>,
    pub time_zone: String,
    pub format: String,
    pub schedule_status: String,
    pub location_status: String,
    pub delivery_mode: String,
    pub venue_id: Option<String>,
    pub venue_label: Option<String>,
    pub official_stream_url: Option<String>,
    pub official_entry_url: Option<String>,
    pub official_note: Option<String>,
    pub source_id: String,
    pub source_external_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceRow {
    pub id: String,
    pub name: String,
    pub source_kind: String,
    pub authority_tier: String,
    pub base_url: String,
    pub access_mode: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GapRow {
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: String,
    pub reason: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRow {
    pub entity_id: String,
    pub entity_kind: String,
    pub display_name: String,
    pub snippet: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalNoteRow {
    pub id: String,
    pub entity_id: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct PackRuntimeRow {
    pub pack_id: String,
    pub version: String,
    pub family: String,
    pub scope_json: String,
    pub schema_version: String,
    pub app_compatibility_json: String,
    pub manifest_json: String,
    pub content_hash: String,
    pub artifact_hash: Option<String>,
    pub size_bytes: u64,
    pub state: String,
    pub progress: u8,
    pub staged_path: Option<String>,
    pub installed_path: Option<String>,
    pub last_error: Option<String>,
}

pub struct CatalogDb {
    connection: Connection,
}

impl CatalogDb {
    #[cfg(test)]
    pub fn in_memory() -> Result<Self, CoreError> {
        let connection = Connection::open_in_memory()?;
        Self::from_connection(connection)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| CoreError::InvalidDatabasePath)?;
        }
        let connection = Connection::open(path)?;
        Self::from_connection(connection)
    }

    fn from_connection(connection: Connection) -> Result<Self, CoreError> {
        connection.pragma_update(None, "foreign_keys", "ON")?;
        let mut database = Self { connection };
        database.apply_migrations()?;
        Ok(database)
    }

    fn apply_migrations(&mut self) -> Result<(), CoreError> {
        let transaction = self.connection.transaction()?;
        for (name, sql) in MIGRATIONS {
            transaction.execute_batch(sql)?;
            transaction.execute(
                "INSERT OR IGNORE INTO schema_migration (name) VALUES (?1)",
                [name],
            )?;
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn migration_names(&self) -> Result<Vec<String>, CoreError> {
        let mut statement = self
            .connection
            .prepare("SELECT name FROM schema_migration ORDER BY name")?;
        let rows = statement.query_map([], |row| row.get(0))?;
        Ok(rows.collect::<Result<Vec<String>, _>>()?)
    }

    pub fn has_fts5(&self) -> Result<bool, CoreError> {
        let exists = self
            .connection
            .query_row(
                "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'catalog_fts'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .optional()?;
        Ok(exists.is_some())
    }

    pub fn ensure_pack_available(
        &mut self,
        pack: &NexPack,
        artifact_size: u64,
    ) -> Result<(), CoreError> {
        let transaction = self.connection.transaction()?;
        let manifest_json = serde_json::to_string(&pack.manifest)?;
        let scope_json = serde_json::to_string(&pack.manifest.scope)?;
        let compatibility_json = serde_json::to_string(&pack.manifest.app_compatibility)?;
        let now = pack.manifest.origin.retrieved_at.as_str();
        transaction.execute(
            "INSERT INTO pack_runtime (pack_id, version, family, scope_json, schema_version, app_compatibility_json, manifest_json, content_hash, size_bytes, state, progress, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'available', 0, ?10, ?10)
             ON CONFLICT(pack_id) DO UPDATE SET version=excluded.version, family=excluded.family,
               scope_json=excluded.scope_json, schema_version=excluded.schema_version,
               app_compatibility_json=excluded.app_compatibility_json, manifest_json=excluded.manifest_json,
               content_hash=excluded.content_hash, size_bytes=excluded.size_bytes,
               state=CASE WHEN pack_runtime.version <> excluded.version THEN 'available' ELSE pack_runtime.state END,
               progress=CASE WHEN pack_runtime.version <> excluded.version THEN 0 ELSE pack_runtime.progress END,
               last_error=CASE WHEN pack_runtime.version <> excluded.version THEN NULL ELSE pack_runtime.last_error END,
               updated_at=excluded.updated_at",
            params![
                pack.manifest.pack_id,
                pack.manifest.version,
                pack.manifest.family,
                scope_json,
                pack.manifest.schema_version,
                compatibility_json,
                manifest_json,
                pack.manifest.content_hash,
                artifact_size,
                now
            ],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn pack_runtime_rows(&self) -> Result<Vec<PackRuntimeRow>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT pack_id, version, family, scope_json, schema_version, app_compatibility_json,
                    manifest_json, content_hash, artifact_hash, size_bytes, state, progress,
                    staged_path, installed_path, last_error
             FROM pack_runtime ORDER BY pack_id",
        )?;
        let rows = statement
            .query_map([], |row| {
                Ok(PackRuntimeRow {
                    pack_id: row.get(0)?,
                    version: row.get(1)?,
                    family: row.get(2)?,
                    scope_json: row.get(3)?,
                    schema_version: row.get(4)?,
                    app_compatibility_json: row.get(5)?,
                    manifest_json: row.get(6)?,
                    content_hash: row.get(7)?,
                    artifact_hash: row.get(8)?,
                    size_bytes: row.get::<_, i64>(9)? as u64,
                    state: row.get(10)?,
                    progress: row.get::<_, i64>(11)? as u8,
                    staged_path: row.get(12)?,
                    installed_path: row.get(13)?,
                    last_error: row.get(14)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>();
        rows.map_err(CoreError::from)
    }

    pub fn pack_runtime_row(&self, pack_id: &str) -> Result<PackRuntimeRow, CoreError> {
        self.connection
            .query_row(
                "SELECT pack_id, version, family, scope_json, schema_version, app_compatibility_json,
                        manifest_json, content_hash, artifact_hash, size_bytes, state, progress,
                        staged_path, installed_path, last_error
                 FROM pack_runtime WHERE pack_id = ?1",
                [pack_id],
                |row| {
                    Ok(PackRuntimeRow {
                        pack_id: row.get(0)?,
                        version: row.get(1)?,
                        family: row.get(2)?,
                        scope_json: row.get(3)?,
                        schema_version: row.get(4)?,
                        app_compatibility_json: row.get(5)?,
                        manifest_json: row.get(6)?,
                        content_hash: row.get(7)?,
                        artifact_hash: row.get(8)?,
                        size_bytes: row.get::<_, i64>(9)? as u64,
                        state: row.get(10)?,
                        progress: row.get::<_, i64>(11)? as u8,
                        staged_path: row.get(12)?,
                        installed_path: row.get(13)?,
                        last_error: row.get(14)?,
                    })
                },
            )
            .map_err(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => CoreError::PackNotFound(pack_id.to_string()),
                other => CoreError::Database(other),
            })
    }

    pub fn set_pack_runtime_state(
        &mut self,
        pack_id: &str,
        version: &str,
        state: &str,
        progress: u8,
        staged_path: Option<&str>,
        installed_path: Option<&str>,
        artifact_hash: Option<&str>,
        size_bytes: Option<u64>,
        last_error: Option<&str>,
    ) -> Result<(), CoreError> {
        let changed = self.connection.execute(
            "UPDATE pack_runtime SET version=?2, state=?3, progress=?4, staged_path=COALESCE(?5, staged_path),
                    installed_path=COALESCE(?6, installed_path), artifact_hash=COALESCE(?7, artifact_hash),
                    size_bytes=COALESCE(?8, size_bytes), last_error=?9,
                    updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE pack_id=?1",
            params![
                pack_id,
                version,
                state,
                i64::from(progress),
                staged_path,
                installed_path,
                artifact_hash,
                size_bytes.map(|value| value as i64),
                last_error
            ],
        )?;
        if changed == 0 {
            return Err(CoreError::PackNotFound(pack_id.to_string()));
        }
        Ok(())
    }

    pub fn import_milano_pack(&mut self, pack: &NexPack) -> Result<usize, CoreError> {
        pack.verify()?;
        let imported = self.seed_milano(&pack.payload)?;
        self.connection.execute(
            "UPDATE pack_installation SET manifest_json=?3, content_hash=?4
             WHERE pack_id=?1 AND pack_version=?2",
            params![
                pack.manifest.pack_id,
                pack.manifest.version,
                serde_json::to_string(&pack.manifest)?,
                pack.manifest.content_hash
            ],
        )?;
        Ok(imported)
    }

    pub fn pack_entity_count(&self, pack_id: &str, entity_kind: &str) -> Result<u64, CoreError> {
        self.connection
            .query_row(
                "SELECT COUNT(*) FROM pack_membership WHERE pack_id=?1 AND entity_kind=?2",
                params![pack_id, entity_kind],
                |row| row.get::<_, i64>(0),
            )
            .map(|count| count as u64)
            .map_err(CoreError::from)
    }

    pub fn remove_pack(&mut self, pack_id: &str) -> Result<(), CoreError> {
        let transaction = self.connection.transaction()?;
        transaction
            .execute(
                "DELETE FROM event_segment
                 WHERE event_id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='event')
                   AND segment_id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='segment')",
                [pack_id],
            )
            .map_err(|error| CoreError::Pack(format!("delete event_segment: {error}")))?;
        for (table, kind) in [
            ("source_contribution", "source_contribution"),
            ("media_occurrence", "media_occurrence"),
            ("catalog_review", "catalog_review"),
            ("person_role", "person_role"),
            ("catalog_collection", "catalog_collection"),
            ("schedule_entry", "schedule_entry"),
            ("participant_registry", "participant"),
            ("catalog_maison", "maison"),
            ("catalog_venue", "venue"),
            ("catalog_edition", "edition"),
            ("catalog_segment", "segment"),
            ("catalog_event", "event"),
            ("city_hub", "city_hub"),
            ("geo_city", "geo_city"),
            ("geo_country", "geo_country"),
            ("geo_region", "geo_region"),
            ("source_endpoint", "source_endpoint"),
        ] {
            delete_owned_rows(&transaction, pack_id, table, kind)?;
        }
        delete_owned_rows(&transaction, pack_id, "catalog_gap", "catalog_gap")?;
        delete_owned_rows(&transaction, pack_id, "source_registry", "source")?;
        transaction.execute(
            "UPDATE catalog_entity SET display_name='Removed pack content', search_text='', payload_json='{\"packRemoved\":true}',
                    is_official=0, updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='catalog_entity')
               AND EXISTS (SELECT 1 FROM personal_note WHERE personal_note.entity_id = catalog_entity.id)
               AND NOT EXISTS (SELECT 1 FROM pack_membership other WHERE other.entity_kind='catalog_entity'
                               AND other.entity_id=catalog_entity.id AND other.pack_id<>?1)",
            [pack_id],
        )?;
        transaction.execute(
            "DELETE FROM catalog_entity
             WHERE id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='catalog_entity')
               AND NOT EXISTS (SELECT 1 FROM personal_note WHERE personal_note.entity_id = catalog_entity.id)
               AND NOT EXISTS (SELECT 1 FROM pack_membership other WHERE other.entity_kind='catalog_entity'
                               AND other.entity_id=catalog_entity.id AND other.pack_id<>?1)",
            [pack_id],
        )?;
        transaction.execute("DELETE FROM pack_installation WHERE pack_id=?1", [pack_id])?;
        transaction.execute("DELETE FROM pack_membership WHERE pack_id=?1", [pack_id])?;
        transaction.commit()?;
        Ok(())
    }

    pub fn seed_bundled_milano(&mut self) -> Result<usize, CoreError> {
        let pack = bundled_pack()?;
        self.seed_milano(&pack.payload)
    }

    pub fn seed_milano(&mut self, seed: &MilanoSeed) -> Result<usize, CoreError> {
        let transaction = self.connection.transaction()?;
        let now = seed.retrieved_at.as_str();
        let pack_id = seed.pack_id.as_str();
        let pack_version = seed.version.as_str();
        let source = &seed.source;
        let capabilities = serde_json::to_string(&source.endpoint.capabilities)?;

        transaction.execute(
            "INSERT INTO source_registry (id, name, source_kind, authority_tier, base_url, access_mode, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, source_kind=excluded.source_kind,
               authority_tier=excluded.authority_tier, base_url=excluded.base_url, access_mode=excluded.access_mode,
               status=excluded.status, updated_at=excluded.updated_at",
            params![
                source.id,
                source.name,
                source.source_kind,
                source.authority_tier,
                source.base_url,
                source.access_mode,
                source.status,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source",
            &source.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO source_endpoint (id, source_id, endpoint_type, base_url, access_method, capabilities_json, adapter_id, status, last_verified_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'active', ?8)
             ON CONFLICT(id) DO UPDATE SET base_url=excluded.base_url, access_method=excluded.access_method,
               capabilities_json=excluded.capabilities_json, adapter_id=excluded.adapter_id, last_verified_at=excluded.last_verified_at",
            params![
                source.endpoint.id,
                source.id,
                source.endpoint.endpoint_type,
                source.endpoint.base_url,
                source.endpoint.access_method,
                capabilities,
                source.endpoint.adapter_id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_endpoint",
            &source.endpoint.id,
            now,
        )?;

        transaction.execute(
            "INSERT INTO geo_region (id, name, m49_code, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, m49_code=excluded.m49_code, source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![seed.region.id, seed.region.name, seed.region.m49_code, source.id, now],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "geo_region",
            &seed.region.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO geo_country (id, region_id, name, iso_alpha2, iso_alpha3, m49_code, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
             ON CONFLICT(id) DO UPDATE SET region_id=excluded.region_id, name=excluded.name,
               iso_alpha2=excluded.iso_alpha2, iso_alpha3=excluded.iso_alpha3, m49_code=excluded.m49_code,
               source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.country.id,
                seed.country.region_id,
                seed.country.name,
                seed.country.iso_alpha2,
                seed.country.iso_alpha3,
                seed.country.m49_code,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "geo_country",
            &seed.country.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO geo_city (id, country_id, name, aliases_json, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)
             ON CONFLICT(id) DO UPDATE SET country_id=excluded.country_id, name=excluded.name,
               aliases_json=excluded.aliases_json, source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.city.id,
                seed.city.country_id,
                seed.city.name,
                serde_json::to_string(&seed.city.aliases)?,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "geo_city",
            &seed.city.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO city_hub (id, city_id, official_name, display_name, slug, hub_type, status, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)
             ON CONFLICT(id) DO UPDATE SET city_id=excluded.city_id, official_name=excluded.official_name,
               display_name=excluded.display_name, slug=excluded.slug, hub_type=excluded.hub_type,
               status=excluded.status, source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.city_hub.id,
                seed.city_hub.city_id,
                seed.city_hub.official_name,
                seed.city_hub.display_name,
                seed.city_hub.slug,
                seed.city_hub.hub_type,
                seed.city_hub.status,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "city_hub",
            &seed.city_hub.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO catalog_event (id, city_hub_id, official_name, display_name, short_name, event_type, status, start_year, official_website, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)
             ON CONFLICT(id) DO UPDATE SET city_hub_id=excluded.city_hub_id, official_name=excluded.official_name,
               display_name=excluded.display_name, short_name=excluded.short_name, event_type=excluded.event_type,
               status=excluded.status, start_year=excluded.start_year, official_website=excluded.official_website,
               source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.event.id,
                seed.event.city_hub_id,
                seed.event.official_name,
                seed.event.display_name,
                seed.event.short_name,
                seed.event.event_type,
                seed.event.status,
                seed.event.start_year,
                seed.event.official_website,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "event",
            &seed.event.id,
            now,
        )?;
        transaction.execute(
            "INSERT INTO catalog_segment (id, name, code, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, code=excluded.code, description=excluded.description, updated_at=excluded.updated_at",
            params![seed.segment.id, seed.segment.name, seed.segment.code, seed.segment.description, now],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "segment",
            &seed.segment.id,
            now,
        )?;
        transaction.execute(
            "INSERT OR REPLACE INTO event_segment (event_id, segment_id, source_id) VALUES (?1, ?2, ?3)",
            params![seed.event.id, seed.segment.id, source.id],
        )?;
        transaction.execute(
            "INSERT INTO catalog_edition (id, event_id, segment_id, display_name, season, season_code, season_year, calendar_year, start_date, end_date, time_zone, edition_status, official_page_url, official_calendar_url, source_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?16)
             ON CONFLICT(id) DO UPDATE SET event_id=excluded.event_id, segment_id=excluded.segment_id,
               display_name=excluded.display_name, season=excluded.season, season_code=excluded.season_code,
               season_year=excluded.season_year, calendar_year=excluded.calendar_year, start_date=excluded.start_date,
               end_date=excluded.end_date, time_zone=excluded.time_zone, edition_status=excluded.edition_status,
               official_page_url=excluded.official_page_url, official_calendar_url=excluded.official_calendar_url,
               source_id=excluded.source_id, updated_at=excluded.updated_at",
            params![
                seed.edition.id,
                seed.edition.event_id,
                seed.edition.segment_id,
                seed.edition.display_name,
                seed.edition.season,
                seed.edition.season_code,
                seed.edition.season_year,
                seed.edition.calendar_year,
                seed.edition.start_date,
                seed.edition.end_date,
                seed.edition.time_zone,
                seed.edition.edition_status,
                seed.edition.official_page_url,
                seed.edition.official_calendar_url,
                source.id,
                now
            ],
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "edition",
            &seed.edition.id,
            now,
        )?;

        for venue in &seed.venues {
            transaction.execute(
                "INSERT INTO catalog_venue (id, city_id, name, address, venue_type, source_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
                 ON CONFLICT(id) DO UPDATE SET city_id=excluded.city_id, name=excluded.name,
                   address=excluded.address, venue_type=excluded.venue_type, source_id=excluded.source_id, updated_at=excluded.updated_at",
                params![
                    venue.id,
                    venue.city_id,
                    venue.name,
                    venue.address,
                    venue.venue_type,
                    source.id,
                    now
                ],
            )?;
            upsert_catalog_entity(
                &transaction,
                &venue.id,
                "venue",
                venue.address.as_deref().unwrap_or("Milano venue"),
                venue.address.as_deref().unwrap_or(""),
                "{}",
                now,
            )?;
            register_pack_entity(&transaction, pack_id, pack_version, "venue", &venue.id, now)?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "catalog_entity",
                &venue.id,
                now,
            )?;
        }

        for maison in &seed.maisons {
            transaction.execute(
                "INSERT INTO catalog_maison (id, official_name, display_name, slug, status, official_website, source_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
                 ON CONFLICT(id) DO UPDATE SET official_name=excluded.official_name, display_name=excluded.display_name,
                   slug=excluded.slug, status=excluded.status, official_website=excluded.official_website,
                   source_id=excluded.source_id, updated_at=excluded.updated_at",
                params![
                    maison.id,
                    maison.official_name,
                    maison.display_name,
                    maison.slug,
                    maison.status,
                    maison.official_website,
                    source.id,
                    now
                ],
            )?;
            upsert_catalog_entity(
                &transaction,
                &maison.id,
                "maison",
                &maison.display_name,
                &maison.official_name,
                &serde_json::to_string(maison)?,
                now,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "maison",
                &maison.id,
                now,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "catalog_entity",
                &maison.id,
                now,
            )?;
        }

        for participant in &seed.participants {
            let normalized_name = normalize_participant_name(&participant.display_name);
            transaction.execute(
                "INSERT INTO participant_registry (id, display_name, normalized_name, canonical_kind, reconciliation_status, source_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
                 ON CONFLICT(id) DO UPDATE SET display_name=excluded.display_name, normalized_name=excluded.normalized_name,
                   canonical_kind=excluded.canonical_kind, reconciliation_status=excluded.reconciliation_status,
                   source_id=excluded.source_id, updated_at=excluded.updated_at",
                params![
                    participant.id,
                    participant.display_name,
                    normalized_name,
                    participant.canonical_kind,
                    participant.reconciliation_status,
                    source.id,
                    now
                ],
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "participant",
                &participant.id,
                now,
            )?;
        }

        for entry in &seed.entries {
            transaction.execute(
                "INSERT INTO schedule_entry (id, edition_id, participant_id, maison_id, local_date, start_time_local, end_time_local, time_zone, format, schedule_status, location_status, delivery_mode, venue_id, venue_label, official_stream_url, official_entry_url, official_note, source_id, source_external_id, source_hash, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?21)
                 ON CONFLICT(edition_id, source_id, source_external_id) DO UPDATE SET participant_id=excluded.participant_id,
                   maison_id=excluded.maison_id, local_date=excluded.local_date, start_time_local=excluded.start_time_local,
                   end_time_local=excluded.end_time_local, time_zone=excluded.time_zone, format=excluded.format,
                   schedule_status=excluded.schedule_status, location_status=excluded.location_status, venue_id=excluded.venue_id,
                   delivery_mode=excluded.delivery_mode, venue_label=excluded.venue_label, official_stream_url=excluded.official_stream_url,
                   official_entry_url=excluded.official_entry_url, official_note=excluded.official_note,
                   source_hash=excluded.source_hash, updated_at=excluded.updated_at",
                params![
                    entry.id,
                    seed.edition.id,
                    entry.participant_id,
                    entry.maison_id,
                    entry.local_date,
                    entry.start_time_local,
                    entry.end_time_local,
                    entry.time_zone,
                    entry.format,
                    entry.schedule_status,
                    entry.location_status,
                    entry.delivery_mode,
                    entry.venue_id,
                    entry.venue_label,
                    entry.official_stream_url,
                    entry.official_entry_url,
                    entry.official_note,
                    source.id,
                    entry.source_external_id,
                    entry.source_hash,
                    now
                ],
            )?;
            let search_text = format!(
                "{} {} {}",
                entry.participant_name_raw,
                entry.format,
                entry.official_note.as_deref().unwrap_or("")
            );
            upsert_catalog_entity(
                &transaction,
                &entry.id,
                "schedule_entry",
                &entry.participant_name_raw,
                &search_text,
                &serde_json::to_string(entry)?,
                now,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "schedule_entry",
                &entry.id,
                now,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "catalog_entity",
                &entry.id,
                now,
            )?;
            insert_contribution(
                &transaction,
                &format!("contribution:{}", entry.id),
                &source.id,
                "schedule_entry",
                &entry.id,
                r#"["participant_name_raw","local_date","start_time_local","format","location_status","delivery_mode","venue_label","official_note"]"#,
                entry
                    .official_entry_url
                    .as_deref()
                    .unwrap_or(&seed.edition.official_calendar_url),
                now,
                &source.endpoint.adapter_id,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "source_contribution",
                &format!("contribution:{}", entry.id),
                now,
            )?;
        }

        upsert_catalog_entity(
            &transaction,
            &seed.city_hub.id,
            "city_hub",
            &seed.city_hub.display_name,
            &format!("{} {}", seed.city_hub.display_name, seed.city_hub.slug),
            &serde_json::to_string(&seed.city_hub)?,
            now,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "catalog_entity",
            &seed.city_hub.id,
            now,
        )?;
        upsert_catalog_entity(
            &transaction,
            &seed.event.id,
            "event",
            &seed.event.display_name,
            &format!("{} {}", seed.event.display_name, seed.event.short_name),
            &serde_json::to_string(&seed.event)?,
            now,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "catalog_entity",
            &seed.event.id,
            now,
        )?;
        upsert_catalog_entity(
            &transaction,
            &seed.edition.id,
            "edition",
            &seed.edition.display_name,
            &format!(
                "{} {} {}",
                seed.edition.display_name, seed.edition.season_code, seed.edition.calendar_year
            ),
            &serde_json::to_string(&seed.edition)?,
            now,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "catalog_entity",
            &seed.edition.id,
            now,
        )?;
        upsert_catalog_entity(
            &transaction,
            &source.id,
            "source",
            &source.name,
            &format!("{} {}", source.name, source.endpoint.base_url),
            &serde_json::to_string(source)?,
            now,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "catalog_entity",
            &source.id,
            now,
        )?;

        insert_contribution(
            &transaction,
            "contribution:cityhub:milano",
            &source.id,
            "city_hub",
            &seed.city_hub.id,
            r#"["official_name","display_name","city_id","status"]"#,
            &seed.edition.official_page_url,
            now,
            &source.endpoint.adapter_id,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_contribution",
            "contribution:cityhub:milano",
            now,
        )?;
        insert_contribution(
            &transaction,
            "contribution:event:milano-fashion-week",
            &source.id,
            "event",
            &seed.event.id,
            r#"["official_name","event_type","official_website"]"#,
            &seed.edition.official_page_url,
            now,
            &source.endpoint.adapter_id,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_contribution",
            "contribution:event:milano-fashion-week",
            now,
        )?;
        insert_contribution(
            &transaction,
            "contribution:edition:milano-fashion-week:ss27:2026",
            &source.id,
            "edition",
            &seed.edition.id,
            r#"["season_code","season_year","calendar_year","start_date","end_date","official_calendar_url"]"#,
            &seed.edition.official_calendar_url,
            now,
            &source.endpoint.adapter_id,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_contribution",
            "contribution:edition:milano-fashion-week:ss27:2026",
            now,
        )?;

        for maison in &seed.maisons {
            insert_contribution(
                &transaction,
                &format!("contribution:{}", maison.id),
                &source.id,
                "maison",
                &maison.id,
                r#"["official_name","official_website"]"#,
                maison
                    .official_website
                    .as_deref()
                    .unwrap_or(&seed.edition.official_page_url),
                now,
                &source.endpoint.adapter_id,
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "source_contribution",
                &format!("contribution:{}", maison.id),
                now,
            )?;
        }

        for gap in &seed.gaps {
            transaction.execute(
                "INSERT INTO catalog_gap (id, entity_type, entity_id, field_name, reason, status, source_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, 'open', ?6, ?7, ?7)
                 ON CONFLICT(entity_type, entity_id, field_name) DO UPDATE SET reason=excluded.reason, status=excluded.status, source_id=excluded.source_id, updated_at=excluded.updated_at",
                params![
                    gap.id,
                    gap.entity_type,
                    gap.entity_id,
                    gap.field_name,
                    gap.reason,
                    source.id,
                    now
                ],
            )?;
            register_pack_entity(
                &transaction,
                pack_id,
                pack_version,
                "catalog_gap",
                &gap.id,
                now,
            )?;
        }

        let manifest = serde_json::json!({
            "packId": seed.pack_id,
            "version": seed.version,
            "kind": "base_catalog",
            "title": seed.edition.display_name,
            "contentHash": seed.content_hash,
            "sourceIds": [source.id],
            "entityCounts": {
                "city_hub": 1,
                "event": 1,
                "edition": 1,
                "schedule_entry": seed.entries.len(),
                "venue": seed.venues.len(),
                "maison": seed.maisons.len(),
                "source": 1
            }
        });
        transaction.execute(
            "INSERT INTO pack_installation (id, pack_id, pack_version, manifest_json, content_hash, installed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(pack_id, pack_version) DO UPDATE SET manifest_json=excluded.manifest_json,
               content_hash=excluded.content_hash, installed_at=excluded.installed_at",
            params![
                format!("pack:{}:{}", seed.pack_id, seed.version),
                seed.pack_id,
                seed.version,
                serde_json::to_string(&manifest)?,
                seed.content_hash,
                now
            ],
        )?;

        transaction.commit()?;
        Ok(seed.entries.len())
    }

    pub fn milano_snapshot(&self) -> Result<MilanoSnapshot, CoreError> {
        let city_hub = self.connection.query_row(
            "SELECT id, city_id, official_name, display_name, slug, status FROM city_hub WHERE id = 'cityhub:milano'",
            [],
            |row| Ok(CityHubRow { id: row.get(0)?, city_id: row.get(1)?, official_name: row.get(2)?, display_name: row.get(3)?, slug: row.get(4)?, status: row.get(5)? }),
        )?;
        let event = self.connection.query_row(
            "SELECT id, city_hub_id, official_name, display_name, short_name, event_type, status, official_website FROM catalog_event WHERE id = 'event:milano-fashion-week'",
            [],
            |row| Ok(EventRow { id: row.get(0)?, city_hub_id: row.get(1)?, official_name: row.get(2)?, display_name: row.get(3)?, short_name: row.get(4)?, event_type: row.get(5)?, status: row.get(6)?, official_website: row.get(7)? }),
        )?;
        let edition = self.connection.query_row(
            "SELECT id, event_id, segment_id, display_name, season, season_code, season_year, calendar_year, start_date, end_date, time_zone, edition_status, official_page_url, official_calendar_url FROM catalog_edition WHERE id = 'edition:milano-fashion-week:ss27:2026'",
            [],
            |row| Ok(EditionRow { id: row.get(0)?, event_id: row.get(1)?, segment_id: row.get(2)?, display_name: row.get(3)?, season: row.get(4)?, season_code: row.get(5)?, season_year: row.get(6)?, calendar_year: row.get(7)?, start_date: row.get(8)?, end_date: row.get(9)?, time_zone: row.get(10)?, edition_status: row.get(11)?, official_page_url: row.get(12)?, official_calendar_url: row.get(13)? }),
        )?;
        let mut statement = self.connection.prepare(
            "SELECT s.id, s.edition_id, s.participant_id, p.display_name, s.maison_id, s.local_date,
                    s.start_time_local, s.end_time_local, s.time_zone, s.format, s.schedule_status,
                    s.location_status, s.delivery_mode, s.venue_id, COALESCE(v.address, s.venue_label), s.official_stream_url,
                    s.official_entry_url, s.official_note, s.source_id, s.source_external_id
             FROM schedule_entry s
             JOIN participant_registry p ON p.id = s.participant_id
             LEFT JOIN catalog_venue v ON v.id = s.venue_id
             WHERE s.edition_id = ?1
             ORDER BY s.local_date, s.start_time_local, s.id",
        )?;
        let schedule = statement
            .query_map([&edition.id], |row| {
                Ok(ScheduleRow {
                    id: row.get(0)?,
                    edition_id: row.get(1)?,
                    participant_id: row.get(2)?,
                    participant_name_raw: row.get(3)?,
                    maison_id: row.get(4)?,
                    local_date: row.get(5)?,
                    start_time_local: row.get(6)?,
                    end_time_local: row.get(7)?,
                    time_zone: row.get(8)?,
                    format: row.get(9)?,
                    schedule_status: row.get(10)?,
                    location_status: row.get(11)?,
                    delivery_mode: row.get(12)?,
                    venue_id: row.get(13)?,
                    venue_label: row.get(14)?,
                    official_stream_url: row.get(15)?,
                    official_entry_url: row.get(16)?,
                    official_note: row.get(17)?,
                    source_id: row.get(18)?,
                    source_external_id: row.get(19)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        let (participant_count, unresolved_participant_count) = self.connection.query_row(
            "SELECT COUNT(*), SUM(CASE WHEN reconciliation_status <> 'reconciled' THEN 1 ELSE 0 END) FROM participant_registry",
            [],
            |row| Ok((row.get(0)?, row.get::<_, Option<i64>>(1)?.unwrap_or(0))),
        )?;
        let source = self.connection.query_row(
            "SELECT id, name, source_kind, authority_tier, base_url, access_mode, status FROM source_registry WHERE id = 'source:cnmi'",
            [],
            |row| Ok(SourceRow { id: row.get(0)?, name: row.get(1)?, source_kind: row.get(2)?, authority_tier: row.get(3)?, base_url: row.get(4)?, access_mode: row.get(5)?, status: row.get(6)? }),
        )?;
        let mut gaps_statement = self.connection.prepare(
            "SELECT entity_type, entity_id, field_name, reason, status FROM catalog_gap ORDER BY entity_type, entity_id, field_name",
        )?;
        let gaps = gaps_statement
            .query_map([], |row| {
                Ok(GapRow {
                    entity_type: row.get(0)?,
                    entity_id: row.get(1)?,
                    field_name: row.get(2)?,
                    reason: row.get(3)?,
                    status: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MilanoSnapshot {
            city_hub,
            event,
            edition,
            schedule,
            participant_count,
            unresolved_participant_count,
            source,
            gaps,
        })
    }

    pub fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchRow>, CoreError> {
        let terms = query
            .split_whitespace()
            .filter(|term| !term.is_empty())
            .map(|term| format!("\"{}\"", term.replace('"', "")))
            .collect::<Vec<_>>();
        if terms.is_empty() {
            return Ok(Vec::new());
        }
        let match_query = terms.join(" ");
        let mut statement = self.connection.prepare(
            "SELECT entity_id, entity_kind, display_name, snippet(catalog_fts, 3, '[', ']', '…', 12)
             FROM catalog_fts WHERE catalog_fts MATCH ?1 ORDER BY rank LIMIT ?2",
        )?;
        let rows = statement
            .query_map(params![match_query, i64::from(limit)], |row| {
                Ok(SearchRow {
                    entity_id: row.get(0)?,
                    entity_kind: row.get(1)?,
                    display_name: row.get(2)?,
                    snippet: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn upsert_personal_note(
        &mut self,
        id: &str,
        entity_id: &str,
        body: &str,
    ) -> Result<PersonalNoteRow, CoreError> {
        let transaction = self.connection.transaction()?;
        let now: String =
            transaction.query_row("SELECT strftime('%Y-%m-%dT%H:%M:%fZ', 'now')", [], |row| {
                row.get(0)
            })?;
        transaction.execute(
            "INSERT INTO personal_note (id, entity_id, body, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?4)
             ON CONFLICT(id) DO UPDATE SET entity_id=excluded.entity_id, body=excluded.body, updated_at=excluded.updated_at",
            params![id, entity_id, body, now],
        )?;
        upsert_catalog_entity(&transaction, id, "personal_note", body, body, "{}", &now)?;
        transaction.commit()?;
        self.connection.query_row(
            "SELECT id, entity_id, body, created_at, updated_at FROM personal_note WHERE id = ?1",
            [id],
            |row| Ok(PersonalNoteRow { id: row.get(0)?, entity_id: row.get(1)?, body: row.get(2)?, created_at: row.get(3)?, updated_at: row.get(4)? }),
        ).map_err(CoreError::from)
    }

    pub fn personal_note(&self, id: &str) -> Result<PersonalNoteRow, CoreError> {
        self.connection
            .query_row(
                "SELECT id, entity_id, body, created_at, updated_at FROM personal_note WHERE id = ?1",
                [id],
                |row| {
                    Ok(PersonalNoteRow {
                        id: row.get(0)?,
                        entity_id: row.get(1)?,
                        body: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                },
            )
            .map_err(CoreError::from)
    }
}

fn register_pack_entity(
    transaction: &Transaction<'_>,
    pack_id: &str,
    pack_version: &str,
    entity_kind: &str,
    entity_id: &str,
    created_at: &str,
) -> Result<(), CoreError> {
    transaction.execute(
        "INSERT OR IGNORE INTO pack_membership (pack_id, pack_version, entity_kind, entity_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![pack_id, pack_version, entity_kind, entity_id, created_at],
    )?;
    Ok(())
}

fn delete_owned_rows(
    transaction: &Transaction<'_>,
    pack_id: &str,
    table: &str,
    entity_kind: &str,
) -> Result<(), CoreError> {
    let sql = format!(
        "DELETE FROM {table}
         WHERE id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind=?2)
           AND NOT EXISTS (SELECT 1 FROM pack_membership other
                           WHERE other.entity_kind=?2 AND other.entity_id={table}.id AND other.pack_id<>?1)"
    );
    transaction
        .execute(&sql, params![pack_id, entity_kind])
        .map_err(|error| CoreError::Pack(format!("delete {table}: {error}")))?;
    Ok(())
}

fn normalize_participant_name(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| character.to_lowercase())
        .filter(|character| character.is_alphanumeric() || character.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn insert_contribution(
    transaction: &Transaction<'_>,
    id: &str,
    source_id: &str,
    entity_type: &str,
    entity_id: &str,
    contributed_fields_json: &str,
    evidence_url: &str,
    retrieved_at: &str,
    adapter_id: &str,
) -> Result<(), CoreError> {
    transaction.execute(
        "INSERT INTO source_contribution (id, source_id, entity_type, entity_id, contributed_fields_json, evidence_url, retrieved_at, evidence_status, adapter_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'verified', ?8)
         ON CONFLICT(source_id, entity_type, entity_id, evidence_url) DO UPDATE SET contributed_fields_json=excluded.contributed_fields_json,
           retrieved_at=excluded.retrieved_at, evidence_status=excluded.evidence_status, adapter_id=excluded.adapter_id",
        params![id, source_id, entity_type, entity_id, contributed_fields_json, evidence_url, retrieved_at, adapter_id],
    )?;
    Ok(())
}

fn upsert_catalog_entity(
    transaction: &Transaction<'_>,
    id: &str,
    entity_kind: &str,
    display_name: &str,
    search_text: &str,
    payload_json: &str,
    now: &str,
) -> Result<(), CoreError> {
    transaction.execute(
        "INSERT INTO catalog_entity (id, entity_kind, display_name, search_text, payload_json, is_official, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?6)
         ON CONFLICT(id) DO UPDATE SET entity_kind=excluded.entity_kind, display_name=excluded.display_name,
           search_text=excluded.search_text, payload_json=excluded.payload_json, updated_at=excluded.updated_at",
        params![id, entity_kind, display_name, search_text, payload_json, now],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::CatalogDb;

    #[test]
    fn baseline_migrations_create_sqlite_and_fts5() {
        let database = CatalogDb::in_memory().expect("baseline database should initialize");
        assert_eq!(
            database.migration_names().unwrap(),
            vec![
                "0001_core",
                "0002_fts5",
                "0003_milano_vertical",
                "0004_pack_runtime"
            ]
        );
        assert!(database.has_fts5().unwrap());
    }

    #[test]
    fn milano_seed_is_idempotent_and_preserves_explicit_gaps() {
        let mut database = CatalogDb::in_memory().expect("database should initialize");
        let first = database
            .seed_bundled_milano()
            .expect("first seed should work");
        let second = database
            .seed_bundled_milano()
            .expect("second seed should work");
        assert_eq!(first, 214);
        assert_eq!(second, first);
        let snapshot = database.milano_snapshot().expect("snapshot should exist");
        assert_eq!(snapshot.schedule.len(), 214);
        assert_eq!(snapshot.edition.season_year, 2027);
        assert_eq!(snapshot.edition.calendar_year, 2026);
        assert_eq!(snapshot.schedule[0].local_date, "2026-09-22");
        let dates = snapshot
            .schedule
            .iter()
            .map(|entry| entry.local_date.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            dates,
            [
                "2026-09-22",
                "2026-09-23",
                "2026-09-24",
                "2026-09-25",
                "2026-09-26",
                "2026-09-27",
                "2026-09-28"
            ]
            .into_iter()
            .collect()
        );
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.format == "presentation_by_appointment"));
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.format == "event"));
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.official_note.as_deref() == Some("LIVE")));
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.delivery_mode == "digital"
                && entry.official_note.as_deref() == Some("DIGITAL")));
        assert!(snapshot
            .schedule
            .iter()
            .any(|entry| entry.participant_name_raw == "PRADA"
                && entry.maison_id.as_deref() == Some("maison:prada")));
        assert!(snapshot.unresolved_participant_count > 0);
        assert!(snapshot
            .gaps
            .iter()
            .any(|gap| gap.field_name == "subregion_id"));
        let search_ids = database
            .search("Prada", 10)
            .unwrap()
            .into_iter()
            .map(|result| result.entity_id)
            .collect::<Vec<_>>();
        assert!(search_ids.contains(&"maison:prada".to_string()));
        assert!(search_ids.contains(&"schedule:cnmi:13639".to_string()));
    }

    #[test]
    fn file_database_survives_reopen() {
        let path = std::env::temp_dir().join(format!(
            "nex-fashion-milano-{}-{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        {
            let mut database = CatalogDb::open(&path).expect("file database should open");
            database.seed_bundled_milano().expect("seed should work");
        }
        let database = CatalogDb::open(&path).expect("file database should reopen");
        assert_eq!(database.milano_snapshot().unwrap().schedule.len(), 214);
        drop(database);
        std::fs::remove_file(path).expect("test database should be cleaned up");
    }

    #[test]
    fn personal_note_targets_stable_catalog_ids() {
        let mut database = CatalogDb::in_memory().expect("database should initialize");
        database.seed_bundled_milano().expect("seed should work");
        let note = database
            .upsert_personal_note("note:prada", "maison:prada", "revisit this source")
            .expect("note should write");
        assert_eq!(note.entity_id, "maison:prada");
        database.seed_bundled_milano().expect("reseed should work");
        assert_eq!(
            database.search("revisit", 10).unwrap()[0].entity_id,
            "note:prada"
        );
    }
}
