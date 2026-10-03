use crate::core::error::CoreError;
use crate::core::milano::{bundled_pack, MilanoSeed, NexPack};
use crate::core::read::{
    CityHubRead, CollectionRead, EditionRead, EntityRef, EventRead, GeoCityRead, LookRead,
    MaisonRead, MediaAssetRead, MediaAvailabilityRead, MediaOccurrenceRead, PackReadRef,
    PageRequest, PersonRead, PersonRoleRead, PersonalNoteRead, PersonalRelatedRead,
    ProvenanceSummary, ReadEnvelope, ReadPage, ReadState, ReviewRead, RoleRead, ScheduleEntryRead,
    SearchResultRead, SourceEndpointRead, SourceRead, TermsBasic, VenueRead,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Serialize;
use std::path::Path;

const MIGRATIONS: [(&str, &str); 5] = [
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
    (
        "0005_personal_favorite",
        include_str!("../../migrations/0005_personal_favorite.sql"),
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

pub type SearchRow = SearchResultRead;

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

    pub fn read_state_for_pack(&self, pack_id: &str) -> Result<ReadState, CoreError> {
        let state = self
            .connection
            .query_row(
                "SELECT state FROM pack_runtime WHERE pack_id=?1",
                [pack_id],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        Ok(match state.as_deref() {
            Some("active") => ReadState::Available,
            Some("removed") => ReadState::PackRemoved,
            Some(_) | None => ReadState::PackNotInstalled,
        })
    }

    pub fn read_pack_ref(&self, pack_id: &str) -> Result<PackReadRef, CoreError> {
        self.connection
            .query_row(
                "SELECT pack_id, version, state FROM pack_runtime WHERE pack_id=?1",
                [pack_id],
                |row| {
                    Ok(PackReadRef {
                        pack_id: row.get(0)?,
                        version: row.get(1)?,
                        state: row.get(2)?,
                    })
                },
            )
            .map_err(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => {
                    CoreError::PackNotInstalled(pack_id.to_string())
                }
                other => CoreError::Database(other),
            })
    }

    fn require_milano_pack(&self) -> Result<PackReadRef, CoreError> {
        let pack = self.read_pack_ref("nex.fashion.milano.ss27")?;
        match pack.state.as_str() {
            "active" => Ok(pack),
            "removed" => Err(CoreError::PackRemoved(pack.pack_id)),
            _ => Err(CoreError::PackNotInstalled(pack.pack_id)),
        }
    }

    fn provenance_for(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Vec<ProvenanceSummary>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT source_id, evidence_url, retrieved_at, evidence_status, adapter_id
             FROM source_contribution
             WHERE entity_type=?1 AND entity_id=?2
             ORDER BY retrieved_at DESC, id",
        )?;
        let rows = statement
            .query_map(params![entity_type, entity_id], |row| {
                Ok(ProvenanceSummary {
                    source_id: row.get(0)?,
                    evidence_url: row.get(1)?,
                    retrieved_at: row.get(2)?,
                    evidence_status: row.get(3)?,
                    adapter_id: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn read_geography(&self, request: PageRequest) -> Result<ReadPage<GeoCityRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM geo_city", [], |row| {
                row.get::<_, i64>(0)
            })? as u64;
        let mut statement = self.connection.prepare(
            "SELECT c.id, c.name, c.aliases_json, co.id, co.name, r.id, r.name
             FROM geo_city c
             JOIN geo_country co ON co.id=c.country_id
             JOIN geo_region r ON r.id=co.region_id
             ORDER BY c.name
             LIMIT ?1 OFFSET ?2",
        )?;
        let rows = statement
            .query_map(
                params![i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    let aliases_json: String = row.get(2)?;
                    Ok(GeoCityRead {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        aliases: serde_json::from_str(&aliases_json).map_err(|error| {
                            rusqlite::Error::FromSqlConversionFailure(
                                2,
                                rusqlite::types::Type::Text,
                                Box::new(error),
                            )
                        })?,
                        country: EntityRef {
                            id: row.get(3)?,
                            entity_kind: "country".to_string(),
                            title: row.get(4)?,
                            slug: None,
                            subtitle: None,
                        },
                        region: EntityRef {
                            id: row.get(5)?,
                            entity_kind: "region".to_string(),
                            title: row.get(6)?,
                            slug: None,
                            subtitle: None,
                        },
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("city", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_city_hubs(&self, request: PageRequest) -> Result<ReadPage<CityHubRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let pack = self.require_milano_pack()?;
        let total: u64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM city_hub", [], |row| {
                row.get::<_, i64>(0)
            })? as u64;
        let mut statement = self.connection.prepare(
            "SELECT h.id, h.city_id, c.name, h.official_name, h.display_name, h.slug, h.status
             FROM city_hub h JOIN geo_city c ON c.id=h.city_id
             ORDER BY h.display_name
             LIMIT ?1 OFFSET ?2",
        )?;
        let rows = statement
            .query_map(
                params![i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(CityHubRead {
                        id: row.get(0)?,
                        city: EntityRef {
                            id: row.get(1)?,
                            entity_kind: "city".to_string(),
                            title: row.get(2)?,
                            slug: None,
                            subtitle: None,
                        },
                        official_name: row.get(3)?,
                        display_name: row.get(4)?,
                        slug: row.get(5)?,
                        status: row.get(6)?,
                        pack: pack.clone(),
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("city_hub", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_city_hub(&self, id: &str) -> Result<CityHubRead, CoreError> {
        let pack = self.require_milano_pack()?;
        let row = self.connection.query_row(
            "SELECT h.id, h.city_id, c.name, h.official_name, h.display_name, h.slug, h.status
             FROM city_hub h JOIN geo_city c ON c.id=h.city_id WHERE h.id=?1",
            [id],
            |row| {
                Ok(CityHubRead {
                    id: row.get(0)?,
                    city: EntityRef {
                        id: row.get(1)?,
                        entity_kind: "city".to_string(),
                        title: row.get(2)?,
                        slug: None,
                        subtitle: None,
                    },
                    official_name: row.get(3)?,
                    display_name: row.get(4)?,
                    slug: row.get(5)?,
                    status: row.get(6)?,
                    pack: pack.clone(),
                    provenance: Vec::new(),
                })
            },
        );
        match row {
            Ok(mut value) => {
                value.provenance = self.provenance_for("city_hub", id)?;
                Ok(value)
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(CoreError::NotFound {
                resource: "city_hub".to_string(),
                id: id.to_string(),
            }),
            Err(error) => Err(CoreError::Database(error)),
        }
    }

    pub fn read_events(
        &self,
        city_hub_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<EventRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let pack = self.require_milano_pack()?;
        let filter = city_hub_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_event WHERE (?1='' OR city_hub_id=?1)",
            [filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT e.id, e.city_hub_id, h.display_name, h.slug, e.official_name, e.display_name,
                    e.short_name, e.event_type, e.status, e.official_website
             FROM catalog_event e JOIN city_hub h ON h.id=e.city_hub_id
             WHERE (?1='' OR e.city_hub_id=?1)
             ORDER BY e.display_name
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![filter, i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(EventRead {
                        id: row.get(0)?,
                        city_hub: EntityRef {
                            id: row.get(1)?,
                            entity_kind: "city_hub".to_string(),
                            title: row.get(2)?,
                            slug: row.get(3)?,
                            subtitle: None,
                        },
                        official_name: row.get(4)?,
                        display_name: row.get(5)?,
                        short_name: row.get(6)?,
                        event_type: row.get(7)?,
                        status: row.get(8)?,
                        official_website: row.get(9)?,
                        pack: pack.clone(),
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("event", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_event(&self, id: &str) -> Result<EventRead, CoreError> {
        let mut items = self
            .read_events(
                None,
                PageRequest {
                    offset: 0,
                    limit: 200,
                },
            )?
            .items;
        if let Some(item) = items.drain(..).find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "event".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_editions(
        &self,
        event_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<EditionRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let pack = self.require_milano_pack()?;
        let filter = event_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_edition WHERE (?1='' OR event_id=?1)",
            [filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT d.id, d.event_id, e.display_name, e.official_name, d.segment_id,
                    s.name, d.display_name, d.season, d.season_code, d.season_year,
                    d.calendar_year, d.start_date, d.end_date, d.time_zone, d.edition_status,
                    d.official_page_url, d.official_calendar_url
             FROM catalog_edition d
             JOIN catalog_event e ON e.id=d.event_id
             LEFT JOIN catalog_segment s ON s.id=d.segment_id
             WHERE (?1='' OR d.event_id=?1)
             ORDER BY d.calendar_year DESC, d.display_name
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![filter, i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    let segment_id: Option<String> = row.get(4)?;
                    let segment_name: Option<String> = row.get(5)?;
                    Ok(EditionRead {
                        id: row.get(0)?,
                        event: EntityRef {
                            id: row.get(1)?,
                            entity_kind: "event".to_string(),
                            title: row.get(2)?,
                            slug: None,
                            subtitle: Some(row.get(3)?),
                        },
                        segment: segment_id.map(|id| EntityRef {
                            id,
                            entity_kind: "segment".to_string(),
                            title: segment_name.unwrap_or_default(),
                            slug: None,
                            subtitle: None,
                        }),
                        display_name: row.get(6)?,
                        season: row.get(7)?,
                        season_code: row.get(8)?,
                        season_year: row.get(9)?,
                        calendar_year: row.get(10)?,
                        start_date: row.get(11)?,
                        end_date: row.get(12)?,
                        time_zone: row.get(13)?,
                        edition_status: row.get(14)?,
                        official_page_url: row.get(15)?,
                        official_calendar_url: row.get(16)?,
                        pack: pack.clone(),
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("edition", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_edition(&self, id: &str) -> Result<EditionRead, CoreError> {
        let mut items = self
            .read_editions(
                None,
                PageRequest {
                    offset: 0,
                    limit: 200,
                },
            )?
            .items;
        if let Some(item) = items.drain(..).find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "edition".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_schedule(
        &self,
        edition_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<ScheduleEntryRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let _pack = self.require_milano_pack()?;
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM schedule_entry WHERE edition_id=?1",
            [edition_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT s.id, s.edition_id, s.participant_id, p.display_name, p.canonical_kind,
                    p.reconciliation_status, s.maison_id, m.display_name, m.slug,
                    s.local_date, s.start_time_local, s.end_time_local, s.time_zone,
                    s.format, s.schedule_status, s.location_status, s.delivery_mode,
                    s.venue_id, COALESCE(v.name, v.address), v.address, s.venue_label,
                    s.official_stream_url, s.official_entry_url, s.official_note
             FROM schedule_entry s
             JOIN participant_registry p ON p.id=s.participant_id
             LEFT JOIN catalog_maison m ON m.id=s.maison_id
             LEFT JOIN catalog_venue v ON v.id=s.venue_id
             WHERE s.edition_id=?1
             ORDER BY s.local_date, s.start_time_local, s.id
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![
                    edition_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let participant_kind: Option<String> = row.get(4)?;
                    let maison_id: Option<String> = row.get(6)?;
                    let maison_title: Option<String> = row.get(7)?;
                    let maison_slug: Option<String> = row.get(8)?;
                    let venue_id: Option<String> = row.get(17)?;
                    let venue_title: Option<String> = row.get(18)?;
                    let venue_address: Option<String> = row.get(19)?;
                    Ok(ScheduleEntryRead {
                        id: row.get(0)?,
                        edition_id: row.get(1)?,
                        participant: EntityRef {
                            id: row.get(2)?,
                            entity_kind: participant_kind
                                .unwrap_or_else(|| "participant".to_string()),
                            title: row.get(3)?,
                            slug: None,
                            subtitle: Some(row.get::<_, String>(5)?),
                        },
                        maison: maison_id.map(|id| EntityRef {
                            id,
                            entity_kind: "maison".to_string(),
                            title: maison_title.unwrap_or_default(),
                            slug: maison_slug,
                            subtitle: None,
                        }),
                        local_date: row.get(9)?,
                        start_time_local: row.get(10)?,
                        end_time_local: row.get(11)?,
                        time_zone: row.get(12)?,
                        format: row.get(13)?,
                        schedule_status: row.get(14)?,
                        location_status: row.get(15)?,
                        delivery_mode: row.get(16)?,
                        venue: venue_id.map(|id| EntityRef {
                            id,
                            entity_kind: "venue".to_string(),
                            title: venue_title.unwrap_or_default(),
                            slug: None,
                            subtitle: venue_address,
                        }),
                        venue_label: row.get(20)?,
                        official_stream_url: row.get(21)?,
                        official_entry_url: row.get(22)?,
                        official_note: row.get(23)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("schedule_entry", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_venues(
        &self,
        city_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<VenueRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let filter = city_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_venue WHERE (?1='' OR city_id=?1)",
            [filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT id, city_id, name, address, venue_type
             FROM catalog_venue
             WHERE (?1='' OR city_id=?1)
             ORDER BY COALESCE(name, address), id
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![filter, i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(VenueRead {
                        id: row.get(0)?,
                        city_id: row.get(1)?,
                        name: row.get(2)?,
                        address: row.get(3)?,
                        venue_type: row.get(4)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("venue", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_venue(&self, id: &str) -> Result<VenueRead, CoreError> {
        let mut items = self
            .read_venues(
                None,
                PageRequest {
                    offset: 0,
                    limit: 200,
                },
            )?
            .items;
        if let Some(item) = items.drain(..).find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "venue".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_maisons(
        &self,
        edition_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<MaisonRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let filter = edition_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(DISTINCT m.id)
             FROM catalog_maison m
             LEFT JOIN schedule_entry s ON s.maison_id=m.id
             WHERE (?1='' OR s.edition_id=?1)",
            [filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT DISTINCT m.id, m.official_name, m.display_name, m.slug, m.status, m.official_website
             FROM catalog_maison m
             LEFT JOIN schedule_entry s ON s.maison_id=m.id
             WHERE (?1='' OR s.edition_id=?1)
             ORDER BY m.display_name
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![filter, i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(MaisonRead {
                        id: row.get(0)?,
                        official_name: row.get(1)?,
                        display_name: row.get(2)?,
                        slug: row.get(3)?,
                        status: row.get(4)?,
                        official_website: row.get(5)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("maison", &item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    pub fn read_maison(&self, id: &str) -> Result<MaisonRead, CoreError> {
        let mut items = self
            .read_maisons(
                None,
                PageRequest {
                    offset: 0,
                    limit: 200,
                },
            )?
            .items;
        if let Some(item) = items.drain(..).find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "maison".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_persons(&self, request: PageRequest) -> Result<ReadPage<PersonRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 =
            self.connection
                .query_row("SELECT COUNT(*) FROM catalog_person", [], |row| {
                    row.get::<_, i64>(0)
                })? as u64;
        let mut statement = self.connection.prepare(
            "SELECT id, full_name, display_name, biography, status
             FROM catalog_person ORDER BY COALESCE(display_name, full_name), id
             LIMIT ?1 OFFSET ?2",
        )?;
        let rows = statement
            .query_map(
                params![i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(PersonRead {
                        id: row.get(0)?,
                        full_name: row.get(1)?,
                        display_name: row.get(2)?,
                        biography: row.get(3)?,
                        status: row.get(4)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("person", &item.id)?;
        }
        Ok(ReadPage::new(
            items,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_person(&self, id: &str) -> Result<PersonRead, CoreError> {
        let _pack = self.require_milano_pack()?;
        let result = self.connection.query_row(
            "SELECT id, full_name, display_name, biography, status
             FROM catalog_person WHERE id=?1",
            [id],
            |row| {
                Ok(PersonRead {
                    id: row.get(0)?,
                    full_name: row.get(1)?,
                    display_name: row.get(2)?,
                    biography: row.get(3)?,
                    status: row.get(4)?,
                    provenance: Vec::new(),
                })
            },
        );
        match result {
            Ok(mut person) => {
                person.provenance = self.provenance_for("person", id)?;
                Ok(person)
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(CoreError::NotFound {
                resource: "person".to_string(),
                id: id.to_string(),
            }),
            Err(error) => Err(CoreError::Database(error)),
        }
    }

    pub fn read_person_roles(
        &self,
        person_id: Option<&str>,
        context_entity_type: Option<&str>,
        context_entity_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<PersonRoleRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let person_filter = person_id.unwrap_or("");
        let context_type_filter = context_entity_type.unwrap_or("");
        let context_id_filter = context_entity_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM person_role
             WHERE (?1='' OR person_id=?1)
               AND (?2='' OR context_entity_type=?2)
               AND (?3='' OR context_entity_id=?3)",
            params![person_filter, context_type_filter, context_id_filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT pr.id, pr.person_id, p.display_name, p.full_name, pr.role_id,
                    r.name_pt_br, r.international_name, r.role_category,
                    pr.context_entity_type, pr.context_entity_id, pr.official_role_title,
                    pr.start_date, pr.end_date, pr.is_current
             FROM person_role pr
             JOIN catalog_person p ON p.id=pr.person_id
             JOIN catalog_role r ON r.id=pr.role_id
             WHERE (?1='' OR pr.person_id=?1)
               AND (?2='' OR pr.context_entity_type=?2)
               AND (?3='' OR pr.context_entity_id=?3)
             ORDER BY p.display_name, r.name_pt_br, pr.id
             LIMIT ?4 OFFSET ?5",
        )?;
        let rows = statement
            .query_map(
                params![
                    person_filter,
                    context_type_filter,
                    context_id_filter,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    Ok(PersonRoleRead {
                        id: row.get(0)?,
                        person: EntityRef {
                            id: row.get(1)?,
                            entity_kind: "person".to_string(),
                            title: row
                                .get::<_, Option<String>>(2)?
                                .unwrap_or_else(|| row.get::<_, String>(3).unwrap_or_default()),
                            slug: None,
                            subtitle: None,
                        },
                        role: RoleRead {
                            id: row.get(4)?,
                            name_pt_br: row.get(5)?,
                            international_name: row.get(6)?,
                            role_category: row.get(7)?,
                        },
                        context_entity_type: row.get(8)?,
                        context_entity_id: row.get(9)?,
                        official_role_title: row.get(10)?,
                        start_date: row.get(11)?,
                        end_date: row.get(12)?,
                        is_current: row.get::<_, i64>(13)? != 0,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_roles(&self, request: PageRequest) -> Result<ReadPage<RoleRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 =
            self.connection
                .query_row("SELECT COUNT(*) FROM catalog_role", [], |row| {
                    row.get::<_, i64>(0)
                })? as u64;
        let mut statement = self.connection.prepare(
            "SELECT id, name_pt_br, international_name, role_category
             FROM catalog_role ORDER BY name_pt_br, id LIMIT ?1 OFFSET ?2",
        )?;
        let rows = statement
            .query_map(
                params![i64::from(request.limit), i64::from(request.offset)],
                |row| {
                    Ok(RoleRead {
                        id: row.get(0)?,
                        name_pt_br: row.get(1)?,
                        international_name: row.get(2)?,
                        role_category: row.get(3)?,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_collections(
        &self,
        edition_id: Option<&str>,
        maison_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<CollectionRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let edition_filter = edition_id.unwrap_or("");
        let maison_filter = maison_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_collection
             WHERE (?1='' OR edition_id=?1) AND (?2='' OR maison_id=?2)",
            params![edition_filter, maison_filter],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT c.id, c.maison_id, m.display_name, m.slug, c.edition_id, e.display_name,
                    c.schedule_entry_id, s.participant_id, p.display_name, c.display_name,
                    c.about, c.presented_at, c.presentation_format, c.season, c.season_year,
                    c.official_collection_url, c.press_release_url
             FROM catalog_collection c
             LEFT JOIN catalog_maison m ON m.id=c.maison_id
             LEFT JOIN catalog_edition e ON e.id=c.edition_id
             LEFT JOIN schedule_entry s ON s.id=c.schedule_entry_id
             LEFT JOIN participant_registry p ON p.id=s.participant_id
             WHERE (?1='' OR c.edition_id=?1) AND (?2='' OR c.maison_id=?2)
             ORDER BY c.display_name, c.id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    edition_filter,
                    maison_filter,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let maison_id: Option<String> = row.get(1)?;
                    let maison_title: Option<String> = row.get(2)?;
                    let maison_slug: Option<String> = row.get(3)?;
                    let edition_id: Option<String> = row.get(4)?;
                    let edition_title: Option<String> = row.get(5)?;
                    let schedule_entry_id: Option<String> = row.get(6)?;
                    let schedule_participant_id: Option<String> = row.get(7)?;
                    let schedule_title: Option<String> = row.get(8)?;
                    Ok(CollectionRead {
                        id: row.get(0)?,
                        maison: maison_id.map(|id| EntityRef {
                            id,
                            entity_kind: "maison".to_string(),
                            title: maison_title.unwrap_or_default(),
                            slug: maison_slug,
                            subtitle: None,
                        }),
                        edition: edition_id.map(|id| EntityRef {
                            id,
                            entity_kind: "edition".to_string(),
                            title: edition_title.unwrap_or_default(),
                            slug: None,
                            subtitle: None,
                        }),
                        schedule_entry: schedule_entry_id.map(|id| EntityRef {
                            id,
                            entity_kind: "schedule_entry".to_string(),
                            title: schedule_title.unwrap_or_default(),
                            slug: None,
                            subtitle: schedule_participant_id,
                        }),
                        display_name: row.get(9)?,
                        about: row.get(10)?,
                        presented_at: row.get(11)?,
                        presentation_format: row.get(12)?,
                        season: row.get(13)?,
                        season_year: row.get(14)?,
                        official_collection_url: row.get(15)?,
                        press_release_url: row.get(16)?,
                        provenance: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.provenance = self.provenance_for("collection", &item.id)?;
        }
        Ok(ReadPage::new(
            items,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_collection(&self, id: &str) -> Result<CollectionRead, CoreError> {
        let page = self.read_collections(
            None,
            None,
            PageRequest {
                offset: 0,
                limit: 200,
            },
        )?;
        if let Some(item) = page.items.into_iter().find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::DataPending {
            resource: "collection".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_looks(
        &self,
        _collection_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<LookRead>, CoreError> {
        let _pack = self.require_milano_pack()?;
        Ok(ReadPage::new(
            Vec::new(),
            0,
            request,
            ReadState::DataPending,
        ))
    }

    pub fn read_media(
        &self,
        entity_type: &str,
        entity_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<MediaOccurrenceRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM media_occurrence WHERE entity_type=?1 AND entity_id=?2",
            params![entity_type, entity_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT o.id, o.media_asset_id, a.media_type, a.title, a.remote_render_policy,
                    o.source_id, o.entity_type, o.entity_id, o.remote_url, o.page_url,
                    o.source_asset_key, o.source_sequence, o.credit, o.asset_health, o.verified_at
             FROM media_occurrence o
             LEFT JOIN catalog_media_asset a ON a.id=o.media_asset_id
             WHERE o.entity_type=?1 AND o.entity_id=?2
             ORDER BY o.source_sequence, o.id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    entity_type,
                    entity_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let asset_id: Option<String> = row.get(1)?;
                    let media_type: Option<String> = row.get(2)?;
                    let title: Option<String> = row.get(3)?;
                    let remote_render_policy: Option<String> = row.get(4)?;
                    Ok(MediaOccurrenceRead {
                        id: row.get(0)?,
                        asset: asset_id.map(|id| MediaAssetRead {
                            id,
                            media_type: media_type.unwrap_or_else(|| "unknown".to_string()),
                            title,
                            remote_render_policy: remote_render_policy
                                .unwrap_or_else(|| "remote_render".to_string()),
                        }),
                        source_id: row.get(5)?,
                        entity_type: row.get(6)?,
                        entity_id: row.get(7)?,
                        remote_url: row.get(8)?,
                        page_url: row.get(9)?,
                        source_asset_key: row.get(10)?,
                        source_sequence: row.get(11)?,
                        credit: row.get(12)?,
                        asset_health: row.get(13)?,
                        verified_at: row.get(14)?,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_media_availability(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<MediaAvailabilityRead, CoreError> {
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(MediaAvailabilityRead {
                entity_type: entity_type.to_string(),
                entity_id: entity_id.to_string(),
                media_available: false,
                video_available: false,
                image_available: false,
                state,
            });
        }
        let (count, video_count, image_count): (i64, i64, i64) = self.connection.query_row(
            "SELECT COUNT(*),
                    SUM(CASE WHEN COALESCE(a.media_type,'') IN ('video','stream') THEN 1 ELSE 0 END),
                    SUM(CASE WHEN COALESCE(a.media_type,'') IN ('image','photo','gallery') THEN 1 ELSE 0 END)
             FROM media_occurrence o
             LEFT JOIN catalog_media_asset a ON a.id=o.media_asset_id
             WHERE o.entity_type=?1 AND o.entity_id=?2",
            params![entity_type, entity_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get::<_, Option<i64>>(1)?.unwrap_or(0),
                    row.get::<_, Option<i64>>(2)?.unwrap_or(0),
                ))
            },
        )?;
        Ok(MediaAvailabilityRead {
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            media_available: count > 0,
            video_available: video_count > 0,
            image_available: image_count > 0,
            state: if count == 0 {
                ReadState::DataPending
            } else {
                state
            },
        })
    }

    pub fn read_reviews(
        &self,
        entity_type: &str,
        entity_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<ReviewRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_review WHERE entity_type=?1 AND entity_id=?2",
            params![entity_type, entity_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT id, source_id, entity_type, entity_id, author, title, published_at,
                    language, original_url, summary, key_points_json
             FROM catalog_review
             WHERE entity_type=?1 AND entity_id=?2
             ORDER BY published_at DESC, id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    entity_type,
                    entity_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let key_points_json: String = row.get(10)?;
                    Ok(ReviewRead {
                        id: row.get(0)?,
                        source_id: row.get(1)?,
                        entity_type: row.get(2)?,
                        entity_id: row.get(3)?,
                        author: row.get(4)?,
                        title: row.get(5)?,
                        published_at: row.get(6)?,
                        language: row.get(7)?,
                        original_url: row.get(8)?,
                        summary: row.get(9)?,
                        key_points: serde_json::from_str(&key_points_json).map_err(|error| {
                            rusqlite::Error::FromSqlConversionFailure(
                                10,
                                rusqlite::types::Type::Text,
                                Box::new(error),
                            )
                        })?,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                state
            },
        ))
    }

    pub fn read_sources(
        &self,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        request: PageRequest,
    ) -> Result<ReadPage<SourceRead>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let entity_type = entity_type.unwrap_or("");
        let entity_id = entity_id.unwrap_or("");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(DISTINCT s.id)
             FROM source_registry s
             LEFT JOIN source_contribution c ON c.source_id=s.id
             WHERE (?1='' OR (c.entity_type=?1 AND c.entity_id=?2))",
            params![entity_type, entity_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT DISTINCT s.id, s.name, s.source_kind, s.authority_tier, s.base_url,
                    s.access_mode, s.status, s.terms_url, s.rights_notes
             FROM source_registry s
             LEFT JOIN source_contribution c ON c.source_id=s.id
             WHERE (?1='' OR (c.entity_type=?1 AND c.entity_id=?2))
             ORDER BY s.name, s.id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    entity_type,
                    entity_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    Ok(SourceRead {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        source_kind: row.get(2)?,
                        authority_tier: row.get(3)?,
                        base_url: row.get(4)?,
                        access_mode: row.get(5)?,
                        status: row.get(6)?,
                        terms: TermsBasic {
                            source_id: row.get(0)?,
                            terms_url: row.get(7)?,
                            rights_notes: row.get(8)?,
                            access_mode: row.get(5)?,
                        },
                        endpoints: Vec::new(),
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        let mut items = rows;
        for item in &mut items {
            item.endpoints = self.read_source_endpoints(&item.id)?;
        }
        Ok(ReadPage::new(items, total, request, state))
    }

    fn read_source_endpoints(&self, source_id: &str) -> Result<Vec<SourceEndpointRead>, CoreError> {
        let mut statement = self.connection.prepare(
            "SELECT id, source_id, endpoint_type, base_url, access_method,
                    capabilities_json, adapter_id, status, last_verified_at
             FROM source_endpoint WHERE source_id=?1 ORDER BY endpoint_type, id",
        )?;
        let rows = statement
            .query_map([source_id], |row| {
                let capabilities_json: String = row.get(5)?;
                Ok(SourceEndpointRead {
                    id: row.get(0)?,
                    source_id: row.get(1)?,
                    endpoint_type: row.get(2)?,
                    base_url: row.get(3)?,
                    access_method: row.get(4)?,
                    capabilities: serde_json::from_str(&capabilities_json).map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            5,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?,
                    adapter_id: row.get(6)?,
                    status: row.get(7)?,
                    last_verified_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>();
        rows.map_err(CoreError::from)
    }

    pub fn read_source(&self, id: &str) -> Result<SourceRead, CoreError> {
        let page = self.read_sources(
            None,
            None,
            PageRequest {
                offset: 0,
                limit: 200,
            },
        )?;
        if let Some(item) = page.items.into_iter().find(|item| item.id == id) {
            return Ok(item);
        }
        Err(CoreError::NotFound {
            resource: "source".to_string(),
            id: id.to_string(),
        })
    }

    pub fn read_terms(&self, source_id: &str) -> Result<ReadEnvelope<TermsBasic>, CoreError> {
        let source = self.read_source(source_id)?;
        let state = if source.terms.terms_url.is_some() || source.terms.rights_notes.is_some() {
            ReadState::Available
        } else {
            ReadState::DataPending
        };
        Ok(ReadEnvelope {
            data: Some(source.terms),
            state,
        })
    }

    pub fn read_provenance(
        &self,
        entity_type: &str,
        entity_id: &str,
        request: PageRequest,
    ) -> Result<ReadPage<ProvenanceSummary>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        if !matches!(state, ReadState::Available) && entity_type != "personal_note" {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM source_contribution WHERE entity_type=?1 AND entity_id=?2",
            params![entity_type, entity_id],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT source_id, evidence_url, retrieved_at, evidence_status, adapter_id
             FROM source_contribution
             WHERE entity_type=?1 AND entity_id=?2
             ORDER BY retrieved_at DESC, id
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(
                params![
                    entity_type,
                    entity_id,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    Ok(ProvenanceSummary {
                        source_id: row.get(0)?,
                        evidence_url: row.get(1)?,
                        retrieved_at: row.get(2)?,
                        evidence_status: row.get(3)?,
                        adapter_id: row.get(4)?,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(
            rows,
            total,
            request,
            if total == 0 {
                ReadState::DataPending
            } else {
                ReadState::Available
            },
        ))
    }

    pub fn read_personal_related(&self, entity_id: &str) -> Result<PersonalRelatedRead, CoreError> {
        let is_favorite = self.connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM personal_favorite WHERE entity_id=?1)",
            [entity_id],
            |row| row.get::<_, i64>(0),
        )? != 0;
        let mut statement = self.connection.prepare(
            "SELECT id, entity_id, body, created_at, updated_at
             FROM personal_note WHERE entity_id=?1 ORDER BY updated_at DESC, id",
        )?;
        let notes = statement
            .query_map([entity_id], |row| {
                Ok(PersonalNoteRead {
                    id: row.get(0)?,
                    entity_id: row.get(1)?,
                    body: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PersonalRelatedRead {
            entity_id: entity_id.to_string(),
            is_favorite,
            notes,
        })
    }

    pub fn set_personal_favorite(
        &mut self,
        entity_id: &str,
        is_favorite: bool,
    ) -> Result<PersonalRelatedRead, CoreError> {
        if is_favorite {
            self.connection.execute(
                "INSERT OR IGNORE INTO personal_favorite (entity_id, created_at)
                 VALUES (?1, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
                [entity_id],
            )?;
        } else {
            self.connection.execute(
                "DELETE FROM personal_favorite WHERE entity_id=?1",
                [entity_id],
            )?;
        }
        self.read_personal_related(entity_id)
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
        transaction.execute(
            "DELETE FROM catalog_fts
             WHERE entity_id IN (SELECT entity_id FROM pack_membership WHERE pack_id=?1 AND entity_kind='geo_city')",
            [pack_id],
        )?;
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
            "DELETE FROM catalog_fts WHERE entity_id=?1",
            [&seed.city.id],
        )?;
        let city_search_rowid: i64 = transaction.query_row(
            "SELECT COALESCE(MIN(rowid), 0) - 1 FROM catalog_fts",
            [],
            |row| row.get(0),
        )?;
        transaction
            .execute(
                "INSERT INTO catalog_fts (rowid, entity_id, display_name, entity_kind, search_text)
                 VALUES (?1, ?2, ?3, 'city', ?4)",
                params![
                    city_search_rowid,
                    seed.city.id,
                    seed.city.name,
                    format!("{} {}", seed.city.name, seed.city.aliases.join(" "))
                ],
            )
            .map_err(|error| CoreError::Pack(format!("city fts insert: {error}")))?;
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
            "contribution:city:milano",
            &source.id,
            "city",
            &seed.city.id,
            r#"["name","aliases"]"#,
            &seed.edition.official_page_url,
            now,
            &source.endpoint.adapter_id,
        )?;
        register_pack_entity(
            &transaction,
            pack_id,
            pack_version,
            "source_contribution",
            "contribution:city:milano",
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
        Ok(self
            .search_page(query, PageRequest { offset: 0, limit })?
            .items)
    }

    pub fn search_page(
        &self,
        query: &str,
        request: PageRequest,
    ) -> Result<ReadPage<SearchRow>, CoreError> {
        let request = request.normalized();
        let state = self.read_state_for_pack("nex.fashion.milano.ss27")?;
        let terms = query
            .split_whitespace()
            .filter(|term| !term.is_empty())
            .map(|term| format!("\"{}\"", term.replace('"', "")))
            .collect::<Vec<_>>();
        if terms.is_empty() {
            return Ok(ReadPage::new(Vec::new(), 0, request, state));
        }
        let match_query = terms.join(" ");
        let total: u64 = self.connection.query_row(
            "SELECT COUNT(*) FROM catalog_fts WHERE catalog_fts MATCH ?1",
            [&match_query],
            |row| row.get::<_, i64>(0),
        )? as u64;
        let mut statement = self.connection.prepare(
            "SELECT f.entity_id, f.entity_kind, f.display_name,
                    snippet(f.catalog_fts, 3, '[', ']', '…', 12),
                    c.payload_json,
                    COALESCE((SELECT group_concat(DISTINCT source_id)
                              FROM source_contribution sc
                              WHERE sc.entity_id=f.entity_id
                                AND (sc.entity_type=f.entity_kind
                                     OR (f.entity_kind='city' AND sc.entity_type='city'))), '')
             FROM catalog_fts f
             LEFT JOIN catalog_entity c ON c.id=f.entity_id
             WHERE f.catalog_fts MATCH ?1
             ORDER BY rank, f.display_name, f.entity_id
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement
            .query_map(
                params![
                    match_query,
                    i64::from(request.limit),
                    i64::from(request.offset)
                ],
                |row| {
                    let entity_id: String = row.get(0)?;
                    let entity_kind: String = row.get(1)?;
                    let payload_json: Option<String> = row.get(4)?;
                    let payload = payload_json
                        .as_deref()
                        .and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok());
                    let slug = payload
                        .as_ref()
                        .and_then(|value| value.get("slug"))
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                    let subtitle = payload.as_ref().and_then(|value| {
                        value
                            .get("seasonCode")
                            .or_else(|| value.get("localDate"))
                            .or_else(|| value.get("officialName"))
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_string)
                    });
                    let thumbnail_url = payload.as_ref().and_then(|value| {
                        value
                            .get("thumbnailUrl")
                            .or_else(|| value.get("thumbUrl"))
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_string)
                    });
                    let source_ids = row
                        .get::<_, String>(5)?
                        .split(',')
                        .filter(|value| !value.is_empty())
                        .map(str::to_string)
                        .collect::<Vec<_>>();
                    Ok(SearchRow {
                        entity_id: entity_id.clone(),
                        entity_kind: entity_kind.clone(),
                        title: row.get(2)?,
                        slug,
                        subtitle,
                        snippet: row.get(3)?,
                        navigation: crate::core::read::NavigationTarget {
                            entity_kind: entity_kind.clone(),
                            entity_id,
                        },
                        thumbnail_url,
                        source_ids,
                        state: if entity_kind == "personal_note" {
                            ReadState::Available
                        } else {
                            state.clone()
                        },
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadPage::new(rows, total, request, state))
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
    transaction
        .execute(
            "INSERT INTO source_contribution (id, source_id, entity_type, entity_id, contributed_fields_json, evidence_url, retrieved_at, evidence_status, adapter_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'verified', ?8)
             ON CONFLICT(source_id, entity_type, entity_id, evidence_url) DO UPDATE SET contributed_fields_json=excluded.contributed_fields_json,
               retrieved_at=excluded.retrieved_at, evidence_status=excluded.evidence_status, adapter_id=excluded.adapter_id",
            params![id, source_id, entity_type, entity_id, contributed_fields_json, evidence_url, retrieved_at, adapter_id],
        )
        .map_err(|error| CoreError::Pack(format!("contribution {id}: {error}")))?;
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
    use crate::core::error::CoreError;
    use crate::core::pack::{PackRuntime, BUNDLED_PACK_ID};
    use crate::core::read::{PageRequest, ReadState};
    use serde_json::Value;

    fn installed_runtime() -> (CatalogDb, PackRuntime, std::path::PathBuf) {
        let root = std::env::temp_dir().join(format!(
            "nex-fashion-read-api-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be valid")
                .as_nanos()
        ));
        let runtime = PackRuntime::new(&root).expect("runtime should initialize");
        let mut database = CatalogDb::in_memory().expect("database should initialize");
        runtime
            .ensure_available(&mut database)
            .expect("pack should be available");
        runtime
            .install(&mut database, BUNDLED_PACK_ID)
            .expect("pack should install");
        (database, runtime, root)
    }

    #[test]
    fn baseline_migrations_create_sqlite_and_fts5() {
        let database = CatalogDb::in_memory().expect("baseline database should initialize");
        assert_eq!(
            database.migration_names().unwrap(),
            vec![
                "0001_core",
                "0002_fts5",
                "0003_milano_vertical",
                "0004_pack_runtime",
                "0005_personal_favorite"
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

    #[test]
    fn public_read_api_navigates_milano_and_marks_missing_data_explicitly() {
        let (mut database, runtime, root) = installed_runtime();
        let city_hubs = database
            .read_city_hubs(PageRequest {
                offset: 0,
                limit: 10,
            })
            .expect("city hubs should be readable");
        assert_eq!(city_hubs.state, ReadState::Available);
        assert_eq!(city_hubs.items[0].id, "cityhub:milano");
        assert_eq!(city_hubs.items[0].city.id, "geo:city:milano");

        let events = database
            .read_events(Some("cityhub:milano"), PageRequest::default())
            .expect("events should be readable");
        assert_eq!(events.items[0].id, "event:milano-fashion-week");
        let editions = database
            .read_editions(Some("event:milano-fashion-week"), PageRequest::default())
            .expect("editions should be readable");
        assert_eq!(
            editions.items[0].id,
            "edition:milano-fashion-week:ss27:2026"
        );

        let schedule = database
            .read_schedule(
                "edition:milano-fashion-week:ss27:2026",
                PageRequest {
                    offset: 0,
                    limit: 25,
                },
            )
            .expect("schedule should be readable");
        assert_eq!(schedule.items.len(), 25);
        assert_eq!(schedule.total, 214);
        assert!(schedule.has_more);
        assert_eq!(schedule.next_offset, Some(25));

        let city_search = database
            .search_page(
                "Milano",
                PageRequest {
                    offset: 0,
                    limit: 20,
                },
            )
            .expect("search should be readable");
        assert!(city_search
            .items
            .iter()
            .any(|result| result.entity_kind == "city"));
        let maison_search = database
            .search_page("Prada", PageRequest::default())
            .expect("maison search should be readable");
        assert!(maison_search
            .items
            .iter()
            .any(|result| result.entity_id == "maison:prada"
                && result.navigation.entity_id == "maison:prada"
                && result.slug.as_deref() == Some("prada")));

        let maison = database
            .read_maison("maison:prada")
            .expect("maison should be readable");
        assert_eq!(maison.slug, "prada");
        assert!(database
            .read_collections(
                Some("edition:milano-fashion-week:ss27:2026"),
                Some(&maison.id),
                PageRequest::default()
            )
            .unwrap()
            .state
            .eq(&ReadState::DataPending));
        assert_eq!(
            database
                .read_looks("collection:missing", PageRequest::default())
                .unwrap()
                .state,
            ReadState::DataPending
        );
        assert_eq!(
            database
                .read_media("maison", &maison.id, PageRequest::default())
                .unwrap()
                .state,
            ReadState::DataPending
        );
        assert_eq!(
            database
                .read_reviews("maison", &maison.id, PageRequest::default())
                .unwrap()
                .state,
            ReadState::DataPending
        );
        assert_eq!(
            database.read_persons(PageRequest::default()).unwrap().state,
            ReadState::DataPending
        );
        assert_eq!(
            database.read_terms("source:cnmi").unwrap().state,
            ReadState::DataPending
        );

        let source_page = database
            .read_sources(None, None, PageRequest::default())
            .expect("sources should be readable");
        assert_eq!(source_page.items[0].id, "source:cnmi");
        database
            .upsert_personal_note("note:read-api", &maison.id, "keep this")
            .expect("note should be created");
        let personal = database
            .set_personal_favorite(&maison.id, true)
            .expect("favorite should be created");
        assert!(personal.is_favorite);
        assert_eq!(personal.notes[0].entity_id, maison.id);

        let serialized = serde_json::to_value(&schedule).expect("page should serialize");
        assert_eq!(serialized["hasMore"], Value::Bool(true));
        assert_eq!(
            serialized["items"][0]["editionId"],
            Value::String("edition:milano-fashion-week:ss27:2026".to_string())
        );
        let serialized_error =
            serde_json::to_value(CoreError::PackRemoved(BUNDLED_PACK_ID.to_string()))
                .expect("ipc error should serialize");
        assert_eq!(
            serialized_error["code"],
            Value::String("PACK_REMOVED".to_string())
        );

        assert!(matches!(
            database.read_maison("maison:does-not-exist"),
            Err(CoreError::NotFound { .. })
        ));

        runtime
            .remove(&mut database, BUNDLED_PACK_ID)
            .expect("pack should be removable");
        assert_eq!(
            database
                .read_city_hubs(PageRequest::default())
                .unwrap()
                .state,
            ReadState::PackRemoved
        );
        assert_eq!(
            database.read_personal_related(&maison.id).unwrap().notes[0].id,
            "note:read-api"
        );
        runtime
            .install(&mut database, BUNDLED_PACK_ID)
            .expect("pack should reinstall");
        assert_eq!(
            database
                .read_edition("edition:milano-fashion-week:ss27:2026")
                .unwrap()
                .season_code,
            "SS27"
        );

        drop(database);
        std::fs::remove_dir_all(root).expect("read api test directory should be cleaned");
    }
}
