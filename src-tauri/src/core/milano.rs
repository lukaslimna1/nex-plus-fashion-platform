use crate::core::error::CoreError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NexPack {
    pub format: String,
    pub format_version: u32,
    pub manifest: NexPackManifest,
    pub payload: MilanoSeed,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NexPackManifest {
    pub format: String,
    pub format_version: u32,
    pub pack_id: String,
    pub version: String,
    pub family: String,
    pub title: String,
    pub scope: PackScope,
    pub schema_version: String,
    pub app_compatibility: PackAppCompatibility,
    pub content_hash: String,
    pub size_bytes: u64,
    pub origin: PackOrigin,
    pub integrity: PackIntegrity,
    pub entity_counts: BTreeMap<String, usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackScope {
    pub city_hub_id: String,
    pub event_id: String,
    pub edition_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackAppCompatibility {
    pub min_core_protocol: String,
    pub min_app_version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackOrigin {
    pub source_ids: Vec<String>,
    pub evidence_urls: Vec<String>,
    pub retrieved_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackIntegrity {
    pub algorithm: String,
    pub payload_encoding: String,
    pub payload_hash: String,
}

impl NexPack {
    pub fn verify(&self) -> Result<(), CoreError> {
        if self.format != "nexpack" || self.manifest.format != "nexpack" {
            return Err(CoreError::Pack("unsupported pack format".to_string()));
        }
        if self.format_version != 1 || self.manifest.format_version != 1 {
            return Err(CoreError::Pack(
                "unsupported pack format version".to_string(),
            ));
        }
        if self.manifest.pack_id != self.payload.pack_id
            || self.manifest.version != self.payload.version
        {
            return Err(CoreError::Pack(
                "manifest and payload identity do not match".to_string(),
            ));
        }
        if self.manifest.schema_version != "catalog.milano.v1" {
            return Err(CoreError::Pack(
                "unsupported catalog schema version".to_string(),
            ));
        }
        if self.manifest.integrity.algorithm != "sha256"
            || self.manifest.integrity.payload_encoding != "utf8-json"
            || self.manifest.integrity.payload_hash != self.manifest.content_hash
        {
            return Err(CoreError::Pack(
                "unsupported or inconsistent integrity metadata".to_string(),
            ));
        }
        let payload_json = canonical_payload_json(&self.payload)?;
        let digest = Sha256::digest(payload_json.as_bytes());
        let actual_hash = format!("{digest:x}");
        if actual_hash != self.manifest.content_hash {
            return Err(CoreError::Pack(format!(
                "payload hash mismatch: expected {}, got {actual_hash}",
                self.manifest.content_hash
            )));
        }
        if self.manifest.scope.edition_id != self.payload.edition.id
            || self.manifest.scope.event_id != self.payload.event.id
            || self.manifest.scope.city_hub_id != self.payload.city_hub.id
        {
            return Err(CoreError::Pack(
                "manifest scope does not match payload".to_string(),
            ));
        }
        Ok(())
    }
}

fn canonical_payload_json(payload: &MilanoSeed) -> Result<String, CoreError> {
    Ok(serde_json::to_string(&serde_json::to_value(payload)?)?)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MilanoSeed {
    pub pack_id: String,
    pub version: String,
    pub content_hash: String,
    pub retrieved_at: String,
    pub source: SeedSource,
    pub region: SeedRegion,
    pub country: SeedCountry,
    pub city: SeedCity,
    pub city_hub: SeedCityHub,
    pub event: SeedEvent,
    pub segment: SeedSegment,
    pub edition: SeedEdition,
    pub venues: Vec<SeedVenue>,
    pub participants: Vec<SeedParticipant>,
    pub maisons: Vec<SeedMaison>,
    pub entries: Vec<SeedScheduleEntry>,
    pub gaps: Vec<SeedGap>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedSource {
    pub id: String,
    pub name: String,
    pub source_kind: String,
    pub authority_tier: String,
    pub base_url: String,
    pub access_mode: String,
    pub status: String,
    pub endpoint: SeedSourceEndpoint,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedSourceEndpoint {
    pub id: String,
    pub endpoint_type: String,
    pub base_url: String,
    pub access_method: String,
    pub capabilities: Vec<String>,
    pub adapter_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedRegion {
    pub id: String,
    pub name: String,
    pub m49_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedCountry {
    pub id: String,
    pub region_id: String,
    pub name: String,
    pub iso_alpha2: String,
    pub iso_alpha3: String,
    pub m49_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedCity {
    pub id: String,
    pub country_id: String,
    pub name: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedCityHub {
    pub id: String,
    pub city_id: String,
    pub official_name: String,
    pub display_name: String,
    pub slug: String,
    pub hub_type: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedEvent {
    pub id: String,
    pub city_hub_id: String,
    pub official_name: String,
    pub display_name: String,
    pub short_name: String,
    pub event_type: String,
    pub status: String,
    pub start_year: i32,
    pub official_website: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedSegment {
    pub id: String,
    pub name: String,
    pub code: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedEdition {
    pub id: String,
    pub event_id: String,
    pub segment_id: String,
    pub display_name: String,
    pub season: String,
    pub season_code: String,
    pub season_year: i32,
    pub calendar_year: i32,
    pub start_date: String,
    pub end_date: String,
    pub time_zone: String,
    pub edition_status: String,
    pub official_page_url: String,
    pub official_calendar_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedVenue {
    pub id: String,
    pub city_id: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub venue_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedParticipant {
    pub id: String,
    pub display_name: String,
    pub canonical_kind: Option<String>,
    pub reconciliation_status: String,
    pub maison_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedMaison {
    pub id: String,
    pub official_name: String,
    pub display_name: String,
    pub slug: String,
    pub status: String,
    pub official_website: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedScheduleEntry {
    pub id: String,
    pub source_external_id: String,
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
    pub source_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedGap {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: String,
    pub reason: String,
}

pub fn bundled_pack() -> Result<NexPack, CoreError> {
    let pack: NexPack = serde_json::from_str(include_str!(
        "../../../packs/milano-ss27-2026.10.03.nexpack"
    ))?;
    pack.verify()?;
    Ok(pack)
}
