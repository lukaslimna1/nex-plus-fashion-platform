export const CORE_PROTOCOL_VERSION = "0.1" as const;

export const BASELINE_MIGRATIONS = [
  "0001_core",
  "0002_fts5",
  "0003_milano_vertical",
  "0004_pack_runtime",
  "0005_personal_favorite",
] as const;

export const MILANO_SS27_PACK_ID = "nex.fashion.milano.ss27" as const;
export const MILANO_SS27_EDITION_ID = "edition:milano-fashion-week:ss27:2026" as const;

export type EntityKind =
  | "city"
  | "country"
  | "region"
  | "city_hub"
  | "event"
  | "edition"
  | "schedule_entry"
  | "participant"
  | "segment"
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
  contractVersion: typeof READ_CONTRACT_VERSION;
  storage: "sqlite";
  fts5: "ready";
  migrationCount: number;
}

export interface IpcError {
  code: IpcErrorCode;
  message: string;
}

export type IpcErrorCode =
  | "NOT_FOUND"
  | "DATA_PENDING"
  | "SOURCE_UNAVAILABLE"
  | "PACK_NOT_INSTALLED"
  | "PACK_REMOVED"
  | "INVALID_REQUEST"
  | "PACK_ERROR"
  | "INTERNAL_ERROR";

export interface IpcResponse<T> {
  ok: boolean;
  data?: T;
  error?: IpcError;
}

export const READ_CONTRACT_VERSION = "1.0" as const;

export type ReadState =
  | "available"
  | "data_pending"
  | "source_unavailable"
  | "pack_not_installed"
  | "pack_removed";

export interface PageRequest {
  offset?: number;
  limit?: number;
}

export interface ReadPage<T> {
  items: T[];
  total: number;
  offset: number;
  limit: number;
  hasMore: boolean;
  nextOffset?: number;
  state: ReadState;
}

export interface ReadEnvelope<T> {
  data?: T;
  state: ReadState;
}

export interface EntityRef {
  id: string;
  entityKind: string;
  title: string;
  slug?: string;
  subtitle?: string;
}

export interface PackReadRef {
  packId: string;
  version: string;
  state: PackState;
}

export interface ProvenanceSummary {
  sourceId: string;
  evidenceUrl: string;
  retrievedAt: string;
  evidenceStatus: EvidenceStatus;
  adapterId?: string;
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

export interface GeoCityRead {
  id: string;
  name: string;
  aliases: string[];
  country: EntityRef;
  region: EntityRef;
  provenance: ProvenanceSummary[];
}

export interface CityHubRead {
  id: string;
  city: EntityRef;
  officialName: string;
  displayName: string;
  slug: string;
  status: string;
  pack: PackReadRef;
  provenance: ProvenanceSummary[];
}

export interface EventRead {
  id: string;
  cityHub: EntityRef;
  officialName: string;
  displayName: string;
  shortName: string | null;
  eventType: string;
  status: string;
  officialWebsite: string | null;
  pack: PackReadRef;
  provenance: ProvenanceSummary[];
}

export interface EditionRead {
  id: string;
  event: EntityRef;
  segment: EntityRef | null;
  displayName: string;
  season: string;
  seasonCode: string;
  seasonYear: number;
  calendarYear: number;
  startDate: string;
  endDate: string;
  timeZone: string;
  editionStatus: string;
  officialPageUrl: string | null;
  officialCalendarUrl: string | null;
  pack: PackReadRef;
  provenance: ProvenanceSummary[];
}

export interface ScheduleEntryRead {
  id: string;
  editionId: string;
  participant: EntityRef;
  maison: EntityRef | null;
  localDate: string;
  startTimeLocal: string | null;
  endTimeLocal: string | null;
  timeZone: string;
  format: string;
  scheduleStatus: string;
  locationStatus: string;
  deliveryMode: string;
  venue: EntityRef | null;
  venueLabel: string | null;
  officialStreamUrl: string | null;
  officialEntryUrl: string | null;
  officialNote: string | null;
  provenance: ProvenanceSummary[];
}

export interface VenueRead {
  id: string;
  cityId: string;
  name: string | null;
  address: string | null;
  venueType: string;
  provenance: ProvenanceSummary[];
}

export interface MaisonRead {
  id: string;
  officialName: string;
  displayName: string;
  slug: string;
  status: string;
  officialWebsite: string | null;
  provenance: ProvenanceSummary[];
}

export interface PersonRead {
  id: string;
  fullName: string;
  displayName: string | null;
  biography: string | null;
  status: string;
  provenance: ProvenanceSummary[];
}

export interface RoleRead {
  id: string;
  namePtBr: string;
  internationalName: string | null;
  roleCategory: string | null;
}

export interface PersonRoleRead {
  id: string;
  person: EntityRef;
  role: RoleRead;
  contextEntityType: string;
  contextEntityId: string;
  officialRoleTitle: string | null;
  startDate: string | null;
  endDate: string | null;
  isCurrent: boolean;
}

export interface CollectionRead {
  id: string;
  maison: EntityRef | null;
  edition: EntityRef | null;
  scheduleEntry: EntityRef | null;
  displayName: string;
  about: string | null;
  presentedAt: string | null;
  presentationFormat: string | null;
  season: string | null;
  seasonYear: number | null;
  officialCollectionUrl: string | null;
  pressReleaseUrl: string | null;
  provenance: ProvenanceSummary[];
}

export interface LookRead {
  id: string;
  collectionId: string;
  displayName: string;
  lookNumber: string | null;
  mediaIds: string[];
  provenance: ProvenanceSummary[];
}

export interface MediaAssetRead {
  id: string;
  mediaType: string;
  title: string | null;
  remoteRenderPolicy: string;
}

export interface MediaOccurrenceRead {
  id: string;
  asset: MediaAssetRead | null;
  sourceId: string;
  entityType: string;
  entityId: string;
  remoteUrl: string;
  pageUrl: string | null;
  sourceAssetKey: string | null;
  sourceSequence: string | null;
  credit: string | null;
  assetHealth: string;
  verifiedAt: string | null;
}

export interface MediaAvailabilityRead {
  entityType: string;
  entityId: string;
  mediaAvailable: boolean;
  videoAvailable: boolean;
  imageAvailable: boolean;
  state: ReadState;
}

export interface ReviewRead {
  id: string;
  sourceId: string;
  entityType: string;
  entityId: string;
  author: string | null;
  title: string | null;
  publishedAt: string | null;
  language: string | null;
  originalUrl: string;
  summary: string | null;
  keyPoints: string[];
}

export interface SourceEndpointRead {
  id: string;
  sourceId: string;
  endpointType: string;
  baseUrl: string;
  accessMethod: string;
  capabilities: string[];
  adapterId: string | null;
  status: string;
  lastVerifiedAt: string | null;
}

export interface TermsBasic {
  sourceId: string;
  termsUrl: string | null;
  rightsNotes: string | null;
  accessMode: string;
}

export interface SourceRead {
  id: string;
  name: string;
  sourceKind: string;
  authorityTier: SourceAuthorityTier;
  baseUrl: string;
  accessMode: string;
  status: string;
  terms: TermsBasic;
  endpoints: SourceEndpointRead[];
}

export interface PersonalNoteRead {
  id: string;
  entityId: string;
  body: string;
  createdAt: string;
  updatedAt: string;
}

export interface PersonalRelatedRead {
  entityId: string;
  isFavorite: boolean;
  notes: PersonalNoteRead[];
}

export interface NavigationTarget {
  entityKind: string;
  entityId: string;
}

export interface SearchResultRead {
  entityId: string;
  entityKind: string;
  title: string;
  slug: string | null;
  subtitle: string | null;
  snippet: string;
  navigation: NavigationTarget;
  thumbnailUrl: string | null;
  sourceIds: string[];
  state: ReadState;
}

export type PackFamily = "base_catalog" | "collection" | "media" | "personal";

export type PackState =
  | "available"
  | "staged"
  | "verifying"
  | "verified"
  | "installing"
  | "active"
  | "removed"
  | "error"
  | "repair_required";

export interface PackScope {
  cityHubId: string;
  eventId: string;
  editionId: string;
}

export interface PackAppCompatibility {
  minCoreProtocol: string;
  minAppVersion: string;
}

export interface PackOrigin {
  sourceIds: string[];
  evidenceUrls: string[];
  retrievedAt: string;
}

export interface PackIntegrity {
  algorithm: "sha256";
  payloadEncoding: "utf8-json";
  payloadHash: string;
}

export interface PackManifest {
  format: "nexpack";
  formatVersion: number;
  packId: string;
  version: string;
  family: PackFamily;
  title: string;
  scope: PackScope;
  schemaVersion: string;
  appCompatibility: PackAppCompatibility;
  contentHash: string;
  sizeBytes: number;
  origin: PackOrigin;
  integrity: PackIntegrity;
  entityCounts: Partial<Record<EntityKind, number>>;
}

export interface PackSummary extends PackManifest {
  artifactHash?: string;
  state: PackState;
  progress: number;
  available: boolean;
  updateAvailable: boolean;
  lastError?: string;
}

export type PackOperation = "stage" | "verify" | "install" | "remove" | "repair";

export interface PackOperationResult {
  operation: PackOperation;
  pack: PackSummary;
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

export type CatalogSearchResult = SearchResultRead;

export function isEntityKind(value: unknown): value is EntityKind {
  return typeof value === "string" && [
    "city",
    "country",
    "region",
    "city_hub",
    "event",
    "edition",
    "schedule_entry",
    "participant",
    "segment",
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
