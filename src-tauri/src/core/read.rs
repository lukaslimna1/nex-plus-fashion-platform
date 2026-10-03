use serde::{Deserialize, Serialize};

pub const READ_CONTRACT_VERSION: &str = "1.0";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRequest {
    #[serde(default)]
    pub offset: u32,
    #[serde(default = "default_page_limit")]
    pub limit: u32,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: default_page_limit(),
        }
    }
}

impl PageRequest {
    pub fn normalized(self) -> Self {
        Self {
            offset: self.offset,
            limit: self.limit.clamp(1, 200),
        }
    }
}

fn default_page_limit() -> u32 {
    50
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadState {
    Available,
    DataPending,
    SourceUnavailable,
    PackNotInstalled,
    PackRemoved,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadPage<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub offset: u32,
    pub limit: u32,
    pub has_more: bool,
    pub next_offset: Option<u32>,
    pub state: ReadState,
}

impl<T> ReadPage<T> {
    pub fn new(items: Vec<T>, total: u64, request: PageRequest, state: ReadState) -> Self {
        let request = request.normalized();
        let next_offset = if request.offset + (items.len() as u32) < total as u32 {
            Some(request.offset + (items.len() as u32))
        } else {
            None
        };
        Self {
            has_more: next_offset.is_some(),
            items,
            total,
            offset: request.offset,
            limit: request.limit,
            next_offset,
            state,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadEnvelope<T> {
    pub data: Option<T>,
    pub state: ReadState,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityRef {
    pub id: String,
    pub entity_kind: String,
    pub title: String,
    pub slug: Option<String>,
    pub subtitle: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackReadRef {
    pub pack_id: String,
    pub version: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSummary {
    pub source_id: String,
    pub evidence_url: String,
    pub retrieved_at: String,
    pub evidence_status: String,
    pub adapter_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoCityRead {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub country: EntityRef,
    pub region: EntityRef,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CityHubRead {
    pub id: String,
    pub city: EntityRef,
    pub official_name: String,
    pub display_name: String,
    pub slug: String,
    pub status: String,
    pub pack: PackReadRef,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRead {
    pub id: String,
    pub city_hub: EntityRef,
    pub official_name: String,
    pub display_name: String,
    pub short_name: Option<String>,
    pub event_type: String,
    pub status: String,
    pub official_website: Option<String>,
    pub pack: PackReadRef,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditionRead {
    pub id: String,
    pub event: EntityRef,
    pub segment: Option<EntityRef>,
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
    pub pack: PackReadRef,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleEntryRead {
    pub id: String,
    pub edition_id: String,
    pub participant: EntityRef,
    pub maison: Option<EntityRef>,
    pub local_date: String,
    pub start_time_local: Option<String>,
    pub end_time_local: Option<String>,
    pub time_zone: String,
    pub format: String,
    pub schedule_status: String,
    pub location_status: String,
    pub delivery_mode: String,
    pub venue: Option<EntityRef>,
    pub venue_label: Option<String>,
    pub official_stream_url: Option<String>,
    pub official_entry_url: Option<String>,
    pub official_note: Option<String>,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VenueRead {
    pub id: String,
    pub city_id: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub venue_type: String,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaisonRead {
    pub id: String,
    pub official_name: String,
    pub display_name: String,
    pub slug: String,
    pub status: String,
    pub official_website: Option<String>,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonRead {
    pub id: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub biography: Option<String>,
    pub status: String,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleRead {
    pub id: String,
    pub name_pt_br: String,
    pub international_name: Option<String>,
    pub role_category: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonRoleRead {
    pub id: String,
    pub person: EntityRef,
    pub role: RoleRead,
    pub context_entity_type: String,
    pub context_entity_id: String,
    pub official_role_title: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionRead {
    pub id: String,
    pub maison: Option<EntityRef>,
    pub edition: Option<EntityRef>,
    pub schedule_entry: Option<EntityRef>,
    pub display_name: String,
    pub about: Option<String>,
    pub presented_at: Option<String>,
    pub presentation_format: Option<String>,
    pub season: Option<String>,
    pub season_year: Option<i32>,
    pub official_collection_url: Option<String>,
    pub press_release_url: Option<String>,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LookRead {
    pub id: String,
    pub collection_id: String,
    pub display_name: String,
    pub look_number: Option<String>,
    pub media_ids: Vec<String>,
    pub provenance: Vec<ProvenanceSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAssetRead {
    pub id: String,
    pub media_type: String,
    pub title: Option<String>,
    pub remote_render_policy: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaOccurrenceRead {
    pub id: String,
    pub asset: Option<MediaAssetRead>,
    pub source_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub remote_url: String,
    pub page_url: Option<String>,
    pub source_asset_key: Option<String>,
    pub source_sequence: Option<String>,
    pub credit: Option<String>,
    pub asset_health: String,
    pub verified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAvailabilityRead {
    pub entity_type: String,
    pub entity_id: String,
    pub media_available: bool,
    pub video_available: bool,
    pub image_available: bool,
    pub state: ReadState,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewRead {
    pub id: String,
    pub source_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub author: Option<String>,
    pub title: Option<String>,
    pub published_at: Option<String>,
    pub language: Option<String>,
    pub original_url: String,
    pub summary: Option<String>,
    pub key_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceEndpointRead {
    pub id: String,
    pub source_id: String,
    pub endpoint_type: String,
    pub base_url: String,
    pub access_method: String,
    pub capabilities: Vec<String>,
    pub adapter_id: Option<String>,
    pub status: String,
    pub last_verified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceRead {
    pub id: String,
    pub name: String,
    pub source_kind: String,
    pub authority_tier: String,
    pub base_url: String,
    pub access_mode: String,
    pub status: String,
    pub terms: TermsBasic,
    pub endpoints: Vec<SourceEndpointRead>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TermsBasic {
    pub source_id: String,
    pub terms_url: Option<String>,
    pub rights_notes: Option<String>,
    pub access_mode: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalNoteRead {
    pub id: String,
    pub entity_id: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalRelatedRead {
    pub entity_id: String,
    pub is_favorite: bool,
    pub notes: Vec<PersonalNoteRead>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationTarget {
    pub entity_kind: String,
    pub entity_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultRead {
    pub entity_id: String,
    pub entity_kind: String,
    pub title: String,
    pub slug: Option<String>,
    pub subtitle: Option<String>,
    pub snippet: String,
    pub navigation: NavigationTarget,
    pub thumbnail_url: Option<String>,
    pub source_ids: Vec<String>,
    pub state: ReadState,
}
