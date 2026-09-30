export type Id = string;
export type ISODateTime = string;
export type CivilDate = `${number}-${number}-${number}`;

export type AuthorityTier = "A" | "B" | "C" | "D";
export type SourceType =
  | "OFFICIAL_ORGANIZER"
  | "MAISON_OFFICIAL"
  | "RUNWAY_COVERAGE"
  | "REVIEW_PUBLICATION"
  | "VIDEO_CHANNEL"
  | "MUSEUM_ARCHIVE"
  | "ACADEMIC"
  | "VOCABULARY"
  | "SOCIAL_NETWORK"
  | "OTHER";
export type AccessMode = "PUBLIC" | "LOGIN_REQUIRED" | "PAYWALL" | "API" | "EMBED" | "UNKNOWN";
export type RightsStatus = "CLEARED" | "RESTRICTED" | "UNKNOWN" | "NOT_APPLICABLE";
export type DownloadPolicy = "DOWNLOAD_ALLOWED" | "DOWNLOAD_BLOCKED" | "DOWNLOAD_UNKNOWN";
export type DisplayMode = "INLINE" | "EMBED" | "LINK_ONLY" | "THUMBNAIL_ONLY" | "PLACEHOLDER";
export type AssetKind = "IMAGE" | "VIDEO";
export type CoverageType = "RUNWAY" | "BACKSTAGE" | "DETAILS" | "BEAUTY" | "FRONT_ROW" | "ARRIVALS" | "EDITORIAL" | "CAMPAIGN" | "ATMOSPHERE" | "VENUE" | "PRESENTATION" | "EXTRAS" | "UNKNOWN";
export type AssetCoverageScope = "LOOKBOOK_IMAGE" | "RUNWAY_COVERAGE_REEL" | "FULL_COLLECTION" | "DETAIL" | "EDITORIAL" | "UNKNOWN";
export type VideoType = "FULL_SHOW" | "OFFICIAL_FILM" | "LIVESTREAM_REPLAY" | "RUNWAY_COVERAGE" | "BACKSTAGE" | "INTERVIEW" | "BEHIND_THE_SCENES" | "HIGHLIGHTS" | "REEL_SHORT" | "PRESS_VIDEO" | "OTHER_VERIFIED";
export type VideoCompleteness = "FULL" | "PARTIAL" | "UNKNOWN";
export type VideoOfficiality = "OFFICIAL" | "PARTNER_VERIFIED" | "PROFESSIONAL_VERIFIED" | "UNKNOWN";
export type MediaProvider = "youtube" | "instagram" | "vimeo" | "tiktok" | "facebook" | "dailymotion" | "weibo" | "website" | "direct" | "other";
export type VideoOrientation = "LANDSCAPE" | "PORTRAIT" | "SQUARE" | "UNKNOWN";
export type PlaybackMode = "YOUTUBE_EMBED" | "INSTAGRAM_EMBED" | "VIMEO_EMBED" | "TIKTOK_EMBED" | "FACEBOOK_EMBED" | "WEBSITE_EMBED" | "HTML5_VIDEO" | "EXTERNAL_LINK";
export type VideoAvailabilityStatus = "AVAILABLE" | "REGION_RESTRICTED" | "REMOVED" | "UNKNOWN";
export type MediaStatus = "COMPLETE" | "PARTIAL" | "NEEDS_RESEARCH" | "NO_MEDIA_FOUND";
export interface CollectionMediaStatus {
  status: MediaStatus;
  runwayImages: number;
  backstageImages: number;
  detailImages: number;
  fullShowVideo: boolean;
  otherVideos: number;
  officialSource: boolean;
  editorialSources: number;
}
export type CanonicalStatus = "DRAFT" | "PENDING_REVIEW" | "VALIDATED" | "REJECTED" | "CANONICAL";
export type AIOutputStatus = "AI_DRAFT" | "PENDING_REVIEW" | "VALIDATED" | "REJECTED";
export type ScheduleVerificationStatus = "VERIFIED" | "UNVERIFIED" | "CANCELLED" | "UNKNOWN";
export type ScheduleState = "UPCOMING" | "SOON" | "NOW" | "ENDED" | "UNKNOWN";
export type ResearchCompleteness = "COMPLETE" | "PARTIAL" | "NEEDS_RESEARCH" | "UNVERIFIED";
export type ImportStatus = "NEW" | "UPDATED" | "UNCHANGED" | "REVIEW_REQUIRED";
export type CoverMatchStatus = "MATCHED" | "ALIAS_MATCH" | "UNMATCHED" | "AMBIGUOUS" | "MISSING";
export type TrendProvenance = "CURATED" | "SOURCE" | "COMMUNITY" | "MODEL_SUGGESTION";
export type TrendStatus = "CONFIRMED" | "PENDING_REVIEW" | "MODEL_SUGGESTION" | "NO_EVIDENCE";
export type FavoriteTargetType = "COLLECTION" | "ASSET" | "MAISON" | "CITY" | "EVENT" | "TERM";

export interface CoverReference {
  assetKey: string;
  url?: string;
  status: CoverMatchStatus;
  fallback: boolean;
}

export interface Region {
  id: Id; name: string; slug: string;
}
export interface Country {
  id: Id; regionId: Id; name: string; isoCode?: string; slug: string;
}
export interface AdministrativeArea {
  id: Id; countryId: Id; name: string; slug: string;
}
export interface CityHub {
  id: Id; name: string; slug: string; countryId: Id; regionId: Id;
  administrativeAreaId?: Id; timezone?: string; latitude?: number; longitude?: number;
  countryName?: string; countryCode?: string; regionName?: string; subregion?: string;
  aliases: string[]; relatedEventIds: Id[]; researchStatus?: ResearchCompleteness;
  hubImportance?: string; primarySourceId?: Id; complementarySourceIds: Id[]; notes?: string;
  cover?: CoverReference; notionPageId?: string; notionUrl?: string; notionLastEditedAt?: ISODateTime;
  sourceHash?: string; lastImportedAt?: ISODateTime; importStatus?: ImportStatus;
}
export interface Event {
  id: Id; name: string; slug: string; kind: "FASHION_WEEK" | "HAUTE_COUTURE_WEEK" | "OTHER";
  officialUrl?: string;
  eventType?: string; aliases: string[]; cityHubIds: Id[]; currentStatus?: string;
  knownStartYear?: number; knownEndYear?: number; activeSince2010?: boolean; organizer?: string;
  usualPeriod?: string; lastVerifiedYear?: number; nextEditionAnnounced?: { start?: CivilDate; end?: CivilDate };
  historicalRelation?: string; about?: string; historySummary?: string; verifiedSummaryAt?: CivilDate;
  notes?: string; socials: Record<string, string>; primarySourceId?: Id; complementarySourceIds: Id[];
  cover?: CoverReference; researchStatus?: ResearchCompleteness; notionPageId?: string; notionUrl?: string;
  notionLastEditedAt?: ISODateTime; sourceHash?: string; lastImportedAt?: ISODateTime; importStatus?: ImportStatus;
}
export interface EventLocation {
  eventId: Id; cityHubId: Id; venueName?: string; address?: string; sourceId?: Id;
}
export interface Segment {
  id: Id; name: string; code: string;
}
export interface EventSegment { eventId: Id; segmentId: Id; }
export interface Edition {
  id: Id; eventId: Id; segmentId?: Id; cityHubId?: Id;
  calendarYear: number; seasonYear: number; seasonCode: string; seasonLabel: string;
  startsOn?: CivilDate; endsOn?: CivilDate; status: CanonicalStatus;
}
export interface ScheduleEntry {
  id: Id; editionId: Id; eventId: Id; segmentId?: Id; cityHubId: Id;
  title: string; format: "SHOW" | "PRESENTATION" | "FILM" | "OTHER";
  startTime: ISODateTime; endTime?: ISODateTime | undefined; timezone: string;
  verificationStatus: ScheduleVerificationStatus; officialUrl?: string; sourcePageUrl?: string; livestreamUrl?: string;
  venueName?: string; verifiedAt?: ISODateTime;
  sourceIds: Id[]; state?: ScheduleState;
}
export interface Maison {
  id: Id; name: string; slug: string; websiteUrl?: string; foundedYear?: number;
  logoUrl?: string; foundedBy?: string[]; country?: string; city?: string; headquarters?: string;
  artisticDirection?: string; currentCreativeDirector?: string; about?: string; history?: string;
  socials?: Record<string, string>; otherOfficialLinks?: Record<string, string>;
  researchStatus?: ResearchCompleteness; verifiedAt?: ISODateTime; officialSourceIds: Id[];
}
export interface CreativeDirectionHistory {
  id: Id; maisonId: Id; personOrTeam: string; startsOn?: CivilDate; endsOn?: CivilDate; sourceIds: Id[];
}
export interface Collection {
  id: Id; maisonId: Id; editionId: Id; name: string; slug: string;
  calendarYear: number; seasonYear: number; seasonCode: string; seasonLabel: string;
  presentedOn?: CivilDate; canonicalStatus: CanonicalStatus; sourceIds: Id[];
  venueName?: string; presentationFormat?: ScheduleEntry["format"];
  creativeDirectorAtCollection?: string; context?: string; organizer?: string;
}
export interface Source {
  id: Id; canonicalName: string; type: SourceType; baseUrl: string;
  authorityTier: AuthorityTier; language: string[]; coverageScope: string[];
  regionId?: Id; countryId?: Id; accessMode: AccessMode; rightsNotes?: string;
  automationNotes?: string; lastVerifiedAt?: ISODateTime; active: boolean;
}
export interface Asset {
  id: Id; collectionId?: Id; title?: string; sourcePageUrl: string; remoteUrl?: string;
  embedUrl?: string; provider?: string; providerAssetId?: string; thumbnailUrl?: string;
  alternativeUrls: string[]; creator?: string; photographer?: string; creditLine?: string;
  sourceIds: Id[]; sourceId?: Id; rightsStatus: RightsStatus; downloadPolicy: DownloadPolicy;
  displayMode: DisplayMode; localPath?: string; cacheUrl?: string; canonicalStatus: CanonicalStatus;
  assetKind?: AssetKind; coverageType?: CoverageType; coverageScope?: AssetCoverageScope; copyrightHolder?: string;
  licenseName?: string; licenseUrl?: string; attributionRequired?: boolean;
  embedAllowed?: boolean; remoteRenderAllowed?: boolean; rehostAllowed?: boolean;
  verifiedAt?: ISODateTime; sequenceNumber?: number; lookNumber?: number;
  canonicalUrl?: string; channelName?: string; durationSeconds?: number; publishedAt?: ISODateTime;
  videoType?: VideoType; completeness?: VideoCompleteness; officiality?: VideoOfficiality;
  width?: number; height?: number; aspectRatio?: string; orientation?: VideoOrientation; playbackMode?: PlaybackMode;
  language?: string; availabilityStatus?: VideoAvailabilityStatus; uploaderName?: string; uploaderUrl?: string;
  metadata?: Record<string, unknown>;
}
export interface VideoAsset extends Omit<Asset, "assetKind" | "remoteUrl" | "embedUrl" | "provider" | "providerAssetId" | "playbackMode" | "orientation"> {
  assetKind: "VIDEO";
  remoteUrl?: string;
  embedUrl?: string;
  provider: MediaProvider;
  providerAssetId?: string;
  orientation: VideoOrientation;
  playbackMode: PlaybackMode;
}
export interface AssetSource { assetId: Id; sourceId: Id; contribution: string; checkedAt: ISODateTime; }
export interface ProfessionalReview {
  id: Id; collectionId?: Id; sourceId: Id; title: string; url: string;
  publishedAt?: ISODateTime; language: string; canonicalStatus: CanonicalStatus;
  author?: string; publication?: string; summary?: string;
}
export interface Term {
  id: Id; value: string; language: string; definition?: string; sourceIds: Id[];
  slug?: string; canonicalName?: string; ptBrName?: string; internationalName?: string;
  aliases: string[]; context?: string; category?: string; externalUri?: string;
  relatedTermIds: Id[]; examples: Array<Record<string, unknown>>; retrievedAt?: ISODateTime;
}
export interface Tag { id: Id; value: string; slug: string; sourceIds: Id[]; }
export interface CollectionTag { collectionId: Id; tagId: Id; evidence?: string; }
export interface AssetTag { assetId: Id; tagId: Id; evidence?: string; }
export interface LookTag { assetId: Id; tagId: Id; lookNumber?: number; }
export interface Favorite { id: Id; identityId: string; targetType: FavoriteTargetType; targetId: Id; createdAt: ISODateTime; }
export interface CommunityReaction { id: Id; identityKey: string; targetType: "COLLECTION" | "ASSET"; targetId: Id; reaction: string; createdAt: ISODateTime; }
export interface CommunityTag { id: Id; identityKey: string; targetType: "COLLECTION" | "ASSET"; targetId: Id; tagId: Id; createdAt: ISODateTime; }
export interface SyncRecord { entityType: string; entityId: Id; revision: number; updatedAt: ISODateTime; deletedAt?: ISODateTime; }
export interface SyncTombstone { entityType: string; entityId: Id; revision: number; deletedAt: ISODateTime; }

export type AITask =
  | "extractCalendar" | "extractScheduleEntries" | "extractCollectionMetadata" | "extractMaisonMetadata" | "extractEventMetadata"
  | "normalizeNames" | "normalizeSeason" | "normalizeCredits" | "translateToPtBr"
  | "summarizePressRelease" | "summarizeProfessionalReview" | "detectSourceChanges"
  | "suggestTags" | "suggestTerms" | "suggestTrendEvidence" | "extractImageMetadata" | "extractVideoMetadata";

export interface AIEnvelope<T> {
  provider: string; model: string; generatedAt: ISODateTime; timestamp: ISODateTime;
  fallbackUsed: boolean; attempts: number; inputSourceIds: Id[];
  confidence?: number; schemaVersion: string; status: AIOutputStatus; data: T;
}
export interface ScheduleExtraction { entries: Array<Pick<ScheduleEntry, "title" | "format" | "startTime" | "endTime" | "timezone" | "officialUrl">>; }
export interface CollectionExtraction { name: string; maisonName: string; seasonCode: string; seasonYear: number; seasonLabel: string; presentedOn?: CivilDate; sourceNotes?: string; }
export interface MaisonExtraction { name: string; artisticDirection?: string; foundedYear?: number; websiteUrl?: string; }
export interface EventExtraction { name: string; kind: Event["kind"]; officialUrl?: string; }

export interface APIListResponse<T> { data: T[]; meta: { count: number; generatedAt: ISODateTime; revision: number; }; }
export interface APISingleResponse<T> { data: T; meta: { generatedAt: ISODateTime; revision: number; }; }
export interface CollectionDetail {
  collection: Collection;
  maison: Maison;
  edition?: Edition;
  event?: Event;
  city?: CityHub;
  schedule: ScheduleEntry[];
  assets: Asset[];
  imageGroups: ImageGroup[];
  videos: Asset[];
  sources: Source[];
  reviews: ProfessionalReview[];
  tags: Tag[];
  mediaStatus: CollectionMediaStatus;
}
export interface ImageGroup { sourceId: Id; coverageType: CoverageType; source?: Source; assets: Asset[]; }
export interface MaisonDetail {
  maison: Maison;
  collections: Collection[];
  creativeDirectorHistory: CreativeDirectionHistory[];
  events: Event[];
  assets: Asset[];
  media: Asset[];
  reviews: ProfessionalReview[];
  sources: Source[];
}
export interface CityDetail {
  city: CityHub;
  country?: Country;
  region?: Region;
  events: Event[];
  editions: Edition[];
  schedule: ScheduleEntry[];
  collections: Collection[];
  assets: Asset[];
  sources: Source[];
}
export interface EventDetail {
  event: Event;
  city?: CityHub;
  cities: CityHub[];
  organizer?: string;
  editions: Edition[];
  segments: Segment[];
  schedule: ScheduleEntry[];
  collections: Collection[];
  maisons: Maison[];
  assets: Asset[];
  sources: Source[];
}
export interface TermDetail {
  term: Term;
  sources: Source[];
  relatedTerms: Term[];
  collections: Collection[];
}
export interface TrendScope {
  periodStart?: CivilDate;
  periodEnd?: CivilDate;
  eventIds?: Id[];
  cityIds?: Id[];
  editionIds?: Id[];
  seasonCodes?: string[];
  collectionIds?: Id[];
}
export interface Trend {
  id: Id; name: string; slug: string; definition?: string; scope: TrendScope;
  evidenceCount: number; status: TrendStatus; provenance: TrendProvenance;
  sourceIds: Id[]; retrievedAt?: ISODateTime;
}
export interface TrendDetail {
  trend: Trend; collections: Collection[]; looks: Asset[]; sources: Source[]; relatedTerms: Term[];
}
export interface SearchResult {
  type: "CITY" | "EVENT" | "EDITION" | "COLLECTION" | "MAISON" | "TERM" | "TREND";
  id: Id; slug: string; title: string; subtitle?: string; thumbnail?: string; routeTarget: string;
}
export interface HomeRail<T> { key: string; title: string; data: T[]; }
export interface HomeResponse {
  rails: {
    happeningNow: HomeRail<ScheduleEntry>;
    upcoming: HomeRail<ScheduleEntry>;
    recentCollections: HomeRail<Collection>;
    latestPresentations: HomeRail<ScheduleEntry>;
    videos: HomeRail<Asset>;
    maisons: HomeRail<Maison>;
    reviews: HomeRail<ProfessionalReview>;
    trends: HomeRail<Trend>;
    library: HomeRail<Term>;
  };
  generatedAt: ISODateTime;
  revision: number;
}
export interface MediaResearchJob {
  id: Id; collectionId: Id; sourceId: Id; mediaType: AssetKind; status: "PENDING" | "RUNNING" | "SUCCEEDED" | "FAILED" | "BLOCKED";
  lastAttemptAt?: ISODateTime; nextEligibleAttemptAt?: ISODateTime; resultCount: number; error?: string; metadata?: Record<string, unknown>;
}
export type AIEnrichmentEntityType = "TERM" | "MAISON" | "COLLECTION" | "EVENT";
export type AIEnrichmentJobStatus = "PENDING" | "RUNNING" | "SUCCEEDED" | "FAILED" | "BLOCKED";
export interface AIEntityContext {
  entityType: AIEnrichmentEntityType;
  entityId?: Id;
  canonicalName: string;
  originalLabel?: string;
  language: string;
  sourceUrls: string[];
  retrievedText?: string;
  currentState: Record<string, unknown>;
  allowedOutputSchema: Record<string, unknown>;
}
export interface AIRequest {
  task: AITask; input: string; inputSourceIds: Id[]; schemaVersion: string;
  context: AIEntityContext; outputSchema: Record<string, unknown>;
}
export interface AIEnrichmentJob {
  id: Id; entityType: AIEnrichmentEntityType; entityId: Id; task: AITask;
  status: AIEnrichmentJobStatus; inputSourceIds: Id[]; context: AIEntityContext;
  result?: Record<string, unknown>; reviewStatus: AIOutputStatus;
  attempts: number; provider?: string; model?: string; error?: string;
  scheduledAt?: ISODateTime; completedAt?: ISODateTime;
}
export interface AuthSession {
  authenticated: boolean; provider: "GOOGLE_OIDC"; subject?: string; email?: string;
  displayName?: string; pictureUrl?: string; expiresAt?: ISODateTime; loginUrl?: string;
}
export interface UserPreferences {
  language: string; timezone: string; autoplayPreview: boolean; reducedMotion: boolean;
  mediaPreference: "REMOTE" | "EMBED" | "LINK_ONLY" | "LOCAL_FIRST";
  editorialPreferences: Record<string, unknown>;
}
export interface PersonalSyncContract {
  provider: "GOOGLE_DRIVE_APP_DATA"; scopes: Array<"favorites" | "preferences" | "settings" | "reading-state">;
  publicCatalogExcluded: boolean; consentRequired: boolean;
}
export interface SourceAdapterCapabilities {
  supportsImages: boolean; supportsVideo: boolean; supportsSchedule: boolean; supportsMetadata: boolean; supportsEmbed: boolean; supportsPagination: boolean;
}
export interface SourceAdapter { id: string; provider: MediaProvider | "fhcm" | "vogue-runway"; capabilities: SourceAdapterCapabilities; }
export interface APIError { error: { code: string; message: string; }; }
export interface HealthResponse {
  status: "ok" | "degraded"; environment: string; version: string; commit?: string;
  database: { reachable: boolean }; aiProviders: { geminiConfigured: boolean; workersAIConfigured: boolean };
}

export interface GoogleIdentity { issuer: "https://accounts.google.com"; subject: string; email?: string; }
export interface PseudonymousIdentity { identityKey: string; provider: "GOOGLE_OIDC"; }
export interface GoogleDriveAppDataAdapter {
  list(scope: "favorites" | "preferences" | "settings" | "reading-state"): Promise<unknown[]>;
  put(scope: "favorites" | "preferences" | "settings" | "reading-state", value: unknown): Promise<void>;
}
