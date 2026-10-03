export const CORE_PROTOCOL_VERSION = "0.1" as const;

export const BASELINE_MIGRATIONS = ["0001_core", "0002_fts5", "0003_milano_vertical"] as const;

export const MILANO_SS27_PACK_ID = "nex.fashion.milano.ss27" as const;
export const MILANO_SS27_EDITION_ID = "edition:milano-fashion-week:ss27:2026" as const;

export type EntityKind =
  | "city_hub"
  | "event"
  | "edition"
  | "schedule_entry"
  | "venue"
  | "maison"
  | "person"
  | "collection"
  | "source"
  | "media_asset"
  | "review"
  | "personal_note";

export interface CoreHealth {
  status: "ok";
  protocolVersion: typeof CORE_PROTOCOL_VERSION;
  storage: "sqlite";
  fts5: "ready";
  migrationCount: number;
}

export interface IpcError {
  code: string;
  message: string;
}

export interface IpcResponse<T> {
  ok: boolean;
  data?: T;
  error?: IpcError;
}

export type SourceAuthorityTier = "A" | "B" | "C" | "D" | "E";
export type EvidenceStatus = "verified" | "partial" | "unverified" | "broken";

export interface GeoCity {
  id: string;
  name: string;
  countryId: string;
  aliases: string[];
}

export interface CityHub {
  id: string;
  cityId: string;
  officialName: string;
  displayName: string;
  slug: string;
  status: "active" | "inactive";
}

export interface FashionEvent {
  id: string;
  cityHubId: string;
  officialName: string;
  displayName: string;
  shortName?: string;
  eventType: string;
  status: "active" | "inactive";
  officialWebsite?: string;
}

export interface Edition {
  id: string;
  eventId: string;
  segmentId?: string;
  displayName: string;
  season: string;
  seasonCode: string;
  seasonYear: number;
  calendarYear: number;
  startDate: string;
  endDate: string;
  timeZone: string;
  editionStatus: "announced" | "ongoing" | "postponed" | "cancelled" | "finished";
  officialPageUrl?: string;
  officialCalendarUrl?: string;
}

export type ScheduleFormat = "fashion_show" | "presentation" | "presentation_by_appointment" | "event";
export type ScheduleStatus = "scheduled" | "live" | "completed" | "postponed" | "cancelled";

export interface ScheduleEntry {
  id: string;
  editionId: string;
  participantId: string;
  participantNameRaw: string;
  maisonId?: string;
  localDate: string;
  startTimeLocal?: string;
  endTimeLocal?: string;
  timeZone: string;
  format: ScheduleFormat;
  scheduleStatus: ScheduleStatus;
  locationStatus: "published" | "invitation" | "not_published";
  deliveryMode: "physical" | "digital" | "hybrid" | "unknown";
  venueId?: string;
  venueLabel?: string;
  officialStreamUrl?: string;
  officialEntryUrl?: string;
  sourceId: string;
  sourceExternalId: string;
}

export interface Participant {
  id: string;
  displayName: string;
  canonicalKind?: "maison" | "organization" | "person" | "unknown";
  reconciliationStatus: "unreconciled" | "candidate" | "reconciled" | "rejected";
  maisonId?: string;
}

export interface Maison {
  id: string;
  officialName: string;
  displayName: string;
  slug: string;
  status: "active" | "inactive";
  officialWebsite?: string;
}

export interface Person {
  id: string;
  fullName: string;
  displayName?: string;
  biography?: string;
}

export interface Role {
  id: string;
  namePtBr: string;
  internationalName?: string;
  roleCategory?: string;
}

export interface PersonRole {
  id: string;
  personId: string;
  roleId: string;
  contextEntityType: string;
  contextEntityId: string;
  officialRoleTitle?: string;
  startDate?: string;
  endDate?: string;
  isCurrent: boolean;
}

export interface Collection {
  id: string;
  maisonId?: string;
  editionId?: string;
  scheduleEntryId?: string;
  displayName: string;
  presentedAt?: string;
  presentationFormat?: string;
  season?: string;
  seasonYear?: number;
  officialCollectionUrl?: string;
  pressReleaseUrl?: string;
}

export interface Source {
  id: string;
  name: string;
  sourceKind: string;
  authorityTier: SourceAuthorityTier;
  baseUrl: string;
  accessMode: string;
  status: "active" | "degraded" | "blocked" | "broken" | "unknown";
}

export interface SourceContribution {
  id: string;
  sourceId: string;
  entityType: string;
  entityId: string;
  contributedFields: string[];
  evidenceUrl: string;
  retrievedAt: string;
  evidenceStatus: EvidenceStatus;
  notes?: string;
  adapterId?: string;
}

export interface MediaAssetContract {
  id: string;
  mediaType: string;
  title?: string;
  remoteRenderPolicy: "remote_render" | "downloaded" | "blocked";
}

export interface MediaOccurrenceContract {
  id: string;
  mediaAssetId?: string;
  sourceId: string;
  entityType: string;
  entityId: string;
  remoteUrl: string;
  pageUrl?: string;
  sourceAssetKey?: string;
  sourceSequence?: string;
  credit?: string;
  assetHealth: "healthy" | "degraded" | "broken" | "unknown";
  verifiedAt?: string;
}

export interface ReviewContract {
  id: string;
  sourceId: string;
  entityType: string;
  entityId: string;
  author?: string;
  title?: string;
  publishedAt?: string;
  language?: string;
  originalUrl: string;
  summary?: string;
  keyPoints: string[];
}

export interface PersonalNoteContract {
  id: string;
  entityId: string;
  body: string;
  createdAt: string;
  updatedAt: string;
}

export interface PackManifest {
  packId: string;
  version: string;
  kind: "base_catalog" | "collection" | "media" | "personal";
  title: string;
  contentHash: string;
  sourceIds: string[];
  entityCounts: Partial<Record<EntityKind, number>>;
}

export interface MilanoVerticalSlice {
  cityHub: CityHub;
  event: FashionEvent;
  edition: Edition;
  schedule: ScheduleEntry[];
  participantCount: number;
  unresolvedParticipantCount: number;
  source: Source;
  gaps: Array<{ entityType: string; entityId: string; fieldName: string; reason: string }>;
}

export interface CatalogSearchResult {
  entityId: string;
  entityKind: EntityKind;
  displayName: string;
  snippet: string;
}

export function isEntityKind(value: unknown): value is EntityKind {
  return typeof value === "string" && [
    "city_hub",
    "event",
    "edition",
    "schedule_entry",
    "venue",
    "maison",
    "person",
    "collection",
    "source",
    "media_asset",
    "review",
    "personal_note",
  ].includes(value);
}
