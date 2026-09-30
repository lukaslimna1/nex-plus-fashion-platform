import type { Asset, CityHub, Collection, CollectionDetail, CollectionMediaStatus, Country, CoverageType, Edition, Event, ImageGroup, Maison, MaisonDetail, ProfessionalReview, Region, ScheduleEntry, Source, Tag, Term } from "@nex-plus/types";

export interface D1DatabaseLike {
  prepare(query: string): D1PreparedStatementLike;
}
export interface D1PreparedStatementLike {
  bind(...values: unknown[]): D1PreparedStatementLike;
  all<T = Record<string, unknown>>(): Promise<{ results: T[] }>;
  first<T = Record<string, unknown>>(): Promise<T | null>;
  run(): Promise<{ success: boolean; meta?: { changes?: number } }>;
}

export interface CatalogRepository {
  listRegions(): Promise<Region[]>;
  listCountries(): Promise<Country[]>;
  listCities(params?: CityFilters): Promise<CityHub[]>;
  listEvents(params?: EventFilters): Promise<Event[]>;
  listEditions(): Promise<Edition[]>;
  listSchedule(params?: { from?: string; to?: string }): Promise<ScheduleEntry[]>;
  listMaisons(): Promise<Maison[]>;
  listCollections(): Promise<Collection[]>;
  listAssets(params?: AssetFilters): Promise<Asset[]>;
  getCollectionBySlug(slug: string): Promise<CollectionDetail | null>;
  getMaisonBySlug(slug: string): Promise<MaisonDetail | null>;
  listSources(): Promise<Source[]>;
  listReviews(): Promise<ProfessionalReview[]>;
  listTerms(): Promise<Term[]>;
  listTags(): Promise<Tag[]>;
  search(query: string): Promise<Array<{ id: string; type: string; name: string; slug: string }>>;
}

export interface AssetFilters {
  collection?: string;
  source?: string;
  coverageType?: CoverageType;
  mediaType?: Asset["assetKind"];
}
export interface CityFilters { region?: string; country?: string; status?: string; hasCover?: boolean; }
export interface EventFilters { status?: string; type?: string; city?: string; hasCover?: boolean; }

function asBoolean(value: unknown): boolean { return value === 1 || value === true; }
function listIds(value: unknown): string[] { return typeof value === "string" && value ? value.split(",") : []; }
function jsonArray(value: unknown): string[] {
  if (typeof value !== "string" || !value) return [];
  try { const parsed: unknown = JSON.parse(value); return Array.isArray(parsed) ? parsed.filter((item): item is string => typeof item === "string") : []; }
  catch { return []; }
}
function jsonObject(value: unknown): Record<string, string> {
  if (typeof value !== "string" || !value) return {};
  try { const parsed: unknown = JSON.parse(value); return parsed && typeof parsed === "object" && !Array.isArray(parsed) ? Object.fromEntries(Object.entries(parsed).filter((entry): entry is [string, string] => typeof entry[1] === "string")) : {}; }
  catch { return {}; }
}
function jsonRecord(value: unknown): Record<string, unknown> {
  if (typeof value !== "string" || !value) return {};
  try {
    const parsed: unknown = JSON.parse(value);
    return parsed && typeof parsed === "object" && !Array.isArray(parsed) ? parsed as Record<string, unknown> : {};
  } catch { return {}; }
}
function cover(value: Record<string, unknown>) {
  if (typeof value.coverAssetKey !== "string" || !value.coverAssetKey) return undefined;
  const status = typeof value.coverMatchStatus === "string" ? value.coverMatchStatus as "MATCHED" | "ALIAS_MATCH" | "UNMATCHED" | "AMBIGUOUS" | "MISSING" : "MISSING";
  return { assetKey: value.coverAssetKey, ...(typeof value.coverUrl === "string" && value.coverUrl ? { url: value.coverUrl } : {}), status, fallback: asBoolean(value.coverFallback) };
}
type AssetRow = Omit<Asset, "sourceIds" | "alternativeUrls" | "attributionRequired" | "embedAllowed" | "remoteRenderAllowed" | "rehostAllowed"> & {
  sourceIds: string; alternativeUrls: string; attributionRequired: number; embedAllowed: number; remoteRenderAllowed: number; rehostAllowed: number;
};

function mediaStatusForAssets(assets: Asset[], sources: Source[]): CollectionMediaStatus {
  const runwayImages = assets.filter((asset) => asset.assetKind === "IMAGE" && asset.coverageType === "RUNWAY").length;
  const backstageImages = assets.filter((asset) => asset.assetKind === "IMAGE" && asset.coverageType === "BACKSTAGE").length;
  const detailImages = assets.filter((asset) => asset.assetKind === "IMAGE" && asset.coverageType === "DETAILS").length;
  const fullShowVideo = assets.some((asset) => asset.assetKind === "VIDEO" && (asset.videoType === "FULL_SHOW" || asset.completeness === "FULL"));
  const otherVideos = assets.filter((asset) => asset.assetKind === "VIDEO" && !(asset.videoType === "FULL_SHOW" || asset.completeness === "FULL")).length;
  const officialSource = sources.some((source) => source.authorityTier === "A");
  const editorialSources = new Set(sources.filter((source) => source.authorityTier !== "A").map((source) => source.id)).size;
  const status = assets.length === 0 ? "NO_MEDIA_FOUND" : runwayImages > 0 && fullShowVideo ? "COMPLETE" : "PARTIAL";
  return { status, runwayImages, backstageImages, detailImages, fullShowVideo, otherVideos, officialSource, editorialSources };
}

export class D1CatalogRepository implements CatalogRepository {
  public constructor(private readonly db: D1DatabaseLike) {}

  async listRegions(): Promise<Region[]> {
    const { results } = await this.db.prepare("SELECT id, name, slug FROM regions WHERE deleted_at IS NULL ORDER BY name").all<Region>();
    return results;
  }
  async listCountries(): Promise<Country[]> {
    const { results } = await this.db.prepare("SELECT id, region_id AS regionId, name, iso_code AS isoCode, slug FROM countries WHERE deleted_at IS NULL ORDER BY name").all<Omit<Country, "isoCode"> & { isoCode: string }>();
    return results.map(({ isoCode, ...row }) => ({ ...row, ...(!isoCode.startsWith("NOTION_UNSET:") ? { isoCode } : {}) }));
  }
  async listCities(params: CityFilters = {}): Promise<CityHub[]> {
    const conditions = ["h.deleted_at IS NULL"];
    const values: unknown[] = [];
    if (params.region) { conditions.push("(r.id = ? OR r.slug = ? OR r.name = ?)"); values.push(params.region, params.region, params.region); }
    if (params.country) { conditions.push("(c.id = ? OR c.slug = ? OR c.name = ? OR c.iso_code = ? OR h.country_code = ?)"); values.push(params.country, params.country, params.country, params.country, params.country); }
    if (params.status) { conditions.push("(h.research_status = ? OR h.import_status = ?)"); values.push(params.status, params.status); }
    if (params.hasCover !== undefined) { conditions.push("h.cover_fallback = ?"); values.push(params.hasCover ? 0 : 1); }
    const query = `SELECT h.id, h.name, h.slug, h.country_id AS countryId, h.region_id AS regionId, h.administrative_area_id AS administrativeAreaId, CASE WHEN h.timezone = 'UNKNOWN' THEN NULL ELSE h.timezone END AS timezone, h.latitude, h.longitude, COALESCE(h.country_name, c.name) AS countryName, h.country_code AS countryCode, r.name AS regionName, h.subregion, h.aliases_json AS aliases, h.related_event_ids AS relatedEventIds, h.research_status AS researchStatus, h.hub_importance AS hubImportance, h.primary_source_id AS primarySourceId, h.complementary_source_ids AS complementarySourceIds, h.notes, h.cover_asset_key AS coverAssetKey, h.cover_url AS coverUrl, h.cover_match_status AS coverMatchStatus, h.cover_fallback AS coverFallback, h.notion_page_id AS notionPageId, h.notion_url AS notionUrl, h.notion_last_edited_at AS notionLastEditedAt, h.source_hash AS sourceHash, h.last_imported_at AS lastImportedAt, h.import_status AS importStatus FROM city_hubs h LEFT JOIN countries c ON c.id = h.country_id LEFT JOIN regions r ON r.id = h.region_id WHERE ${conditions.join(" AND ")} ORDER BY h.name`;
    const { results } = await this.db.prepare(query).bind(...values).all<Record<string, unknown>>();
    return results.map((row) => ({
      id: String(row.id), name: String(row.name), slug: String(row.slug), countryId: String(row.countryId), regionId: String(row.regionId),
      ...(typeof row.administrativeAreaId === "string" ? { administrativeAreaId: row.administrativeAreaId } : {}), ...(typeof row.timezone === "string" ? { timezone: row.timezone } : {}),
      ...(typeof row.latitude === "number" ? { latitude: row.latitude } : {}), ...(typeof row.longitude === "number" ? { longitude: row.longitude } : {}),
      ...(typeof row.countryName === "string" ? { countryName: row.countryName } : {}), ...(typeof row.countryCode === "string" ? { countryCode: row.countryCode } : {}),
      ...(typeof row.regionName === "string" ? { regionName: row.regionName } : {}), ...(typeof row.subregion === "string" ? { subregion: row.subregion } : {}), aliases: jsonArray(row.aliases), relatedEventIds: listIds(row.relatedEventIds),
      ...(typeof row.researchStatus === "string" ? { researchStatus: row.researchStatus as Exclude<CityHub["researchStatus"], undefined> } : {}), ...(typeof row.hubImportance === "string" ? { hubImportance: row.hubImportance } : {}),
      ...(typeof row.primarySourceId === "string" ? { primarySourceId: row.primarySourceId } : {}), complementarySourceIds: listIds(row.complementarySourceIds), ...(typeof row.notes === "string" ? { notes: row.notes } : {}),
      ...(cover(row) ? { cover: cover(row)! } : {}), ...(typeof row.notionPageId === "string" ? { notionPageId: row.notionPageId } : {}), ...(typeof row.notionUrl === "string" ? { notionUrl: row.notionUrl } : {}),
      ...(typeof row.notionLastEditedAt === "string" ? { notionLastEditedAt: row.notionLastEditedAt } : {}), ...(typeof row.sourceHash === "string" ? { sourceHash: row.sourceHash } : {}), ...(typeof row.lastImportedAt === "string" ? { lastImportedAt: row.lastImportedAt } : {}),
      ...(typeof row.importStatus === "string" ? { importStatus: row.importStatus as Exclude<CityHub["importStatus"], undefined> } : {})
    }));
  }
  async listEvents(params: EventFilters = {}): Promise<Event[]> {
    const conditions = ["e.deleted_at IS NULL"];
    const values: unknown[] = [];
    if (params.status) { conditions.push("(e.current_status = ? OR e.research_status = ? OR e.import_status = ?)"); values.push(params.status, params.status, params.status); }
    if (params.type) { conditions.push("(e.event_type = ? OR e.kind = ?)"); values.push(params.type, params.type); }
    if (params.city) { conditions.push("EXISTS (SELECT 1 FROM event_locations filter_location LEFT JOIN city_hubs filter_city ON filter_city.id = filter_location.city_hub_id WHERE filter_location.event_id = e.id AND (filter_city.id = ? OR filter_city.slug = ? OR filter_city.name = ?))"); values.push(params.city, params.city, params.city); }
    if (params.hasCover !== undefined) { conditions.push("e.cover_fallback = ?"); values.push(params.hasCover ? 0 : 1); }
    const query = `SELECT e.id, e.name, e.slug, e.kind, e.official_url AS officialUrl, e.event_type AS eventType, e.aliases_json AS aliases, e.city_hub_ids AS cityHubIds, e.current_status AS currentStatus, e.known_start_year AS knownStartYear, e.known_end_year AS knownEndYear, e.active_since_2010 AS activeSince2010, e.organizer, e.usual_period AS usualPeriod, e.last_verified_year AS lastVerifiedYear, e.next_edition_announced_json AS nextEditionAnnounced, e.historical_relation AS historicalRelation, e.about, e.history_summary AS historySummary, e.verified_summary_at AS verifiedSummaryAt, e.notes, e.socials_json AS socials, e.primary_source_id AS primarySourceId, e.complementary_source_ids AS complementarySourceIds, e.cover_asset_key AS coverAssetKey, e.cover_url AS coverUrl, e.cover_match_status AS coverMatchStatus, e.cover_fallback AS coverFallback, e.research_status AS researchStatus, e.notion_page_id AS notionPageId, e.notion_url AS notionUrl, e.notion_last_edited_at AS notionLastEditedAt, e.source_hash AS sourceHash, e.last_imported_at AS lastImportedAt, e.import_status AS importStatus FROM events e WHERE ${conditions.join(" AND ")} ORDER BY e.name`;
    const { results } = await this.db.prepare(query).bind(...values).all<Record<string, unknown>>();
    return results.map((row) => ({
      id: String(row.id), name: String(row.name), slug: String(row.slug), kind: row.kind as Event["kind"], ...(typeof row.officialUrl === "string" ? { officialUrl: row.officialUrl } : {}),
      ...(typeof row.eventType === "string" ? { eventType: row.eventType } : {}), aliases: jsonArray(row.aliases), cityHubIds: listIds(row.cityHubIds), ...(typeof row.currentStatus === "string" ? { currentStatus: row.currentStatus } : {}),
      ...(typeof row.knownStartYear === "number" ? { knownStartYear: row.knownStartYear } : {}), ...(typeof row.knownEndYear === "number" ? { knownEndYear: row.knownEndYear } : {}), ...(row.activeSince2010 !== null && row.activeSince2010 !== undefined ? { activeSince2010: asBoolean(row.activeSince2010) } : {}),
      ...(typeof row.organizer === "string" ? { organizer: row.organizer } : {}), ...(typeof row.usualPeriod === "string" ? { usualPeriod: row.usualPeriod } : {}), ...(typeof row.lastVerifiedYear === "number" ? { lastVerifiedYear: row.lastVerifiedYear } : {}),
      ...(typeof row.nextEditionAnnounced === "string" && Object.keys(jsonObject(row.nextEditionAnnounced)).length > 0 ? { nextEditionAnnounced: jsonObject(row.nextEditionAnnounced) as Exclude<Event["nextEditionAnnounced"], undefined> } : {}), ...(typeof row.historicalRelation === "string" ? { historicalRelation: row.historicalRelation } : {}),
      ...(typeof row.about === "string" ? { about: row.about } : {}), ...(typeof row.historySummary === "string" ? { historySummary: row.historySummary } : {}), ...(typeof row.verifiedSummaryAt === "string" && /^\d{4}-\d{2}-\d{2}$/.test(row.verifiedSummaryAt) ? { verifiedSummaryAt: row.verifiedSummaryAt as `${number}-${number}-${number}` } : {}), ...(typeof row.notes === "string" ? { notes: row.notes } : {}),
      socials: jsonObject(row.socials), ...(typeof row.primarySourceId === "string" ? { primarySourceId: row.primarySourceId } : {}), complementarySourceIds: listIds(row.complementarySourceIds), ...(cover(row) ? { cover: cover(row)! } : {}),
      ...(typeof row.researchStatus === "string" ? { researchStatus: row.researchStatus as Exclude<Event["researchStatus"], undefined> } : {}), ...(typeof row.notionPageId === "string" ? { notionPageId: row.notionPageId } : {}), ...(typeof row.notionUrl === "string" ? { notionUrl: row.notionUrl } : {}),
      ...(typeof row.notionLastEditedAt === "string" ? { notionLastEditedAt: row.notionLastEditedAt } : {}), ...(typeof row.sourceHash === "string" ? { sourceHash: row.sourceHash } : {}), ...(typeof row.lastImportedAt === "string" ? { lastImportedAt: row.lastImportedAt } : {}), ...(typeof row.importStatus === "string" ? { importStatus: row.importStatus as Exclude<Event["importStatus"], undefined> } : {})
    }));
  }
  async listEditions(): Promise<Edition[]> {
    const { results } = await this.db.prepare("SELECT id, event_id AS eventId, segment_id AS segmentId, city_hub_id AS cityHubId, calendar_year AS calendarYear, season_year AS seasonYear, season_code AS seasonCode, season_label AS seasonLabel, starts_on AS startsOn, ends_on AS endsOn, status FROM editions WHERE deleted_at IS NULL ORDER BY starts_on DESC").all<Edition>();
    return results;
  }
  async listSchedule(params: { from?: string; to?: string } = {}): Promise<ScheduleEntry[]> {
    const conditions = ["s.deleted_at IS NULL"];
    const values: string[] = [];
    if (params.from) { conditions.push("s.start_time >= ?"); values.push(params.from); }
    if (params.to) { conditions.push("s.start_time <= ?"); values.push(params.to); }
    const query = `SELECT s.id, s.edition_id AS editionId, s.event_id AS eventId, s.segment_id AS segmentId, s.city_hub_id AS cityHubId, s.title, s.format, s.start_time AS startTime, s.end_time AS endTime, s.timezone, s.verification_status AS verificationStatus, s.official_url AS officialUrl, s.livestream_url AS livestreamUrl, s.source_ids AS sourceIds FROM schedule_entries s WHERE ${conditions.join(" AND ")} ORDER BY s.start_time`;
    const { results } = await this.db.prepare(query).bind(...values).all<Omit<ScheduleEntry, "sourceIds"> & { sourceIds: string }>();
    return results.map((row) => ({ ...row, endTime: row.endTime ?? undefined, sourceIds: listIds(row.sourceIds) }));
  }
  async listMaisons(): Promise<Maison[]> {
    const { results } = await this.db.prepare("SELECT id, name, slug, website_url AS websiteUrl, founded_year AS foundedYear, artistic_direction AS artisticDirection, official_source_ids AS sourceIds FROM maisons WHERE deleted_at IS NULL ORDER BY name").all<Omit<Maison, "officialSourceIds"> & { sourceIds: string }>();
    return results.map(({ sourceIds, ...row }) => ({ ...row, officialSourceIds: listIds(sourceIds) }));
  }
  async listCollections(): Promise<Collection[]> {
    const { results } = await this.db.prepare("SELECT id, maison_id AS maisonId, edition_id AS editionId, name, slug, calendar_year AS calendarYear, season_year AS seasonYear, season_code AS seasonCode, season_label AS seasonLabel, presented_on AS presentedOn, canonical_status AS canonicalStatus, source_ids AS sourceIds, venue_name AS venueName, presentation_format AS presentationFormat, creative_director_at_collection AS creativeDirectorAtCollection, context, organizer FROM collections WHERE deleted_at IS NULL ORDER BY presented_on DESC, name").all<Omit<Collection, "sourceIds"> & { sourceIds: string }>();
    return results.map((row) => ({ ...row, sourceIds: listIds(row.sourceIds) }));
  }
  async listAssets(params: AssetFilters = {}): Promise<Asset[]> {
    const conditions = ["a.deleted_at IS NULL"];
    const values: unknown[] = [];
    if (params.collection) { conditions.push("(a.collection_id = ? OR c.slug = ?)"); values.push(params.collection, params.collection); }
    if (params.source) { conditions.push("(',' || a.source_ids || ',') LIKE ?"); values.push(`%,${params.source},%`); }
    if (params.coverageType) { conditions.push("COALESCE(a.coverage_type_label, a.coverage_type) = ?"); values.push(params.coverageType); }
    if (params.mediaType) { conditions.push("a.asset_kind = ?"); values.push(params.mediaType); }
    const query = `SELECT a.id, a.collection_id AS collectionId, a.title, a.source_page_url AS sourcePageUrl, a.remote_url AS remoteUrl, a.embed_url AS embedUrl, a.provider, a.provider_asset_id AS providerAssetId, a.thumbnail_url AS thumbnailUrl, a.alternative_urls AS alternativeUrls, a.creator, a.photographer, a.credit_line AS creditLine, a.source_ids AS sourceIds, a.rights_status AS rightsStatus, a.download_policy AS downloadPolicy, a.display_mode AS displayMode, a.local_path AS localPath, a.cache_url AS cacheUrl, a.canonical_status AS canonicalStatus, a.asset_kind AS assetKind, COALESCE(a.coverage_type_label, a.coverage_type) AS coverageType, a.coverage_scope AS coverageScope, a.copyright_holder AS copyrightHolder, a.license_name AS licenseName, a.license_url AS licenseUrl, a.attribution_required AS attributionRequired, a.embed_allowed AS embedAllowed, a.remote_render_allowed AS remoteRenderAllowed, a.rehost_allowed AS rehostAllowed, a.verified_at AS verifiedAt, a.sequence_number AS sequenceNumber, a.look_number AS lookNumber, a.canonical_url AS canonicalUrl, a.channel_name AS channelName, a.duration_seconds AS durationSeconds, a.published_at AS publishedAt, a.video_type AS videoType, a.completeness, a.officiality, a.width, a.height, a.aspect_ratio AS aspectRatio, a.orientation, a.playback_mode AS playbackMode, a.language, a.availability_status AS availabilityStatus, a.uploader_name AS uploaderName, a.uploader_url AS uploaderUrl, a.metadata_json AS metadata FROM assets a LEFT JOIN collections c ON c.id = a.collection_id WHERE ${conditions.join(" AND ")} ORDER BY COALESCE(a.sequence_number, 999999), a.id`;
    const { results } = await this.db.prepare(query).bind(...values).all<AssetRow>();
    return results.map((row) => {
      const sourceIds = listIds(row.sourceIds);
      return {
        ...row,
        sourceIds,
        ...(sourceIds[0] ? { sourceId: sourceIds[0] } : {}),
        alternativeUrls: listIds(row.alternativeUrls),
        attributionRequired: asBoolean(row.attributionRequired),
        embedAllowed: asBoolean(row.embedAllowed),
        remoteRenderAllowed: asBoolean(row.remoteRenderAllowed),
        rehostAllowed: asBoolean(row.rehostAllowed),
        ...(Object.keys(jsonRecord(row.metadata)).length > 0 ? { metadata: jsonRecord(row.metadata) } : {})
      };
    });
  }
  async getCollectionBySlug(slug: string): Promise<CollectionDetail | null> {
    const collection = (await this.listCollections()).find((item) => item.slug === slug);
    if (!collection) return null;
    const [maisons, editions, events, cities, schedule, assets, sources, reviews, tags] = await Promise.all([
      this.listMaisons(), this.listEditions(), this.listEvents(), this.listCities(), this.listSchedule(), this.listAssets(), this.listSources(), this.listReviews(), this.listTagsForCollection(collection.id)
    ]);
    const maison = maisons.find((item) => item.id === collection.maisonId);
    if (!maison) return null;
    const edition = editions.find((item) => item.id === collection.editionId);
    const event = edition ? events.find((item) => item.id === edition.eventId) : undefined;
    const city = edition ? cities.find((item) => item.id === edition.cityHubId) : undefined;
    const collectionAssets = assets.filter((item) => item.collectionId === collection.id);
    const collectionSources = sources.filter((item) => collection.sourceIds.includes(item.id));
    const imageGroups = new Map<string, ImageGroup>();
    for (const asset of collectionAssets.filter((item) => item.assetKind !== "VIDEO")) {
      const coverageType = asset.coverageType ?? "UNKNOWN";
      for (const sourceId of asset.sourceIds) {
        const key = `${sourceId}:${coverageType}`;
        const existing = imageGroups.get(key);
        if (existing) {
          existing.assets.push(asset);
          continue;
        }
        const group: ImageGroup = { sourceId, coverageType, assets: [asset] };
        const source = collectionSources.find((item) => item.id === sourceId);
        if (source) group.source = source;
        imageGroups.set(key, group);
      }
    }
    return {
      collection,
      maison,
      ...(edition ? { edition } : {}),
      ...(event ? { event } : {}),
      ...(city ? { city } : {}),
      schedule: schedule.filter((item) => item.editionId === collection.editionId),
      assets: collectionAssets,
      imageGroups: Array.from(imageGroups.values()),
      videos: collectionAssets.filter((item) => item.assetKind === "VIDEO"),
      sources: collectionSources,
      reviews: reviews.filter((item) => item.collectionId === collection.id),
      tags,
      mediaStatus: mediaStatusForAssets(collectionAssets, collectionSources)
    };
  }
  private async listTagsForCollection(collectionId: string): Promise<Tag[]> {
    const { results } = await this.db.prepare("SELECT t.id, t.value, t.slug, t.source_ids AS sourceIds FROM tags t INNER JOIN collection_tags ct ON ct.tag_id = t.id WHERE ct.collection_id = ? ORDER BY t.value").bind(collectionId).all<Omit<Tag, "sourceIds"> & { sourceIds: string }>();
    return results.map(({ sourceIds, ...row }) => ({ ...row, sourceIds: listIds(sourceIds) }));
  }
  async getMaisonBySlug(slug: string): Promise<MaisonDetail | null> {
    const maison = (await this.listMaisons()).find((item) => item.slug === slug);
    if (!maison) return null;
    const [collections, assets, sources] = await Promise.all([this.listCollections(), this.listAssets(), this.listSources()]);
    const maisonCollections = collections.filter((item) => item.maisonId === maison.id);
    const collectionIds = new Set(maisonCollections.map((item) => item.id));
    return { maison, collections: maisonCollections, assets: assets.filter((item) => item.collectionId && collectionIds.has(item.collectionId)), sources: sources.filter((item) => maison.officialSourceIds.includes(item.id)) };
  }
  async listSources(): Promise<Source[]> {
    const { results } = await this.db.prepare("SELECT id, canonical_name AS canonicalName, type, base_url AS baseUrl, authority_tier AS authorityTier, language, coverage_scope AS coverageScope, region_id AS regionId, country_id AS countryId, access_mode AS accessMode, rights_notes AS rightsNotes, automation_notes AS automationNotes, last_verified_at AS lastVerifiedAt, active FROM sources WHERE deleted_at IS NULL ORDER BY canonical_name").all<Omit<Source, "language" | "coverageScope" | "active"> & { language: string; coverageScope: string; active: number }>();
    return results.map((row) => ({ ...row, language: row.language.split(","), coverageScope: row.coverageScope.split(","), active: asBoolean(row.active) }));
  }
  async listReviews(): Promise<ProfessionalReview[]> {
    const { results } = await this.db.prepare("SELECT id, collection_id AS collectionId, source_id AS sourceId, title, url, published_at AS publishedAt, language, canonical_status AS canonicalStatus, author, publication, summary FROM professional_reviews ORDER BY published_at DESC").all<ProfessionalReview>();
    return results;
  }
  async listTerms(): Promise<Term[]> {
    const { results } = await this.db.prepare("SELECT id, value, language, definition, source_ids AS sourceIds FROM terms ORDER BY value").all<Omit<Term, "sourceIds"> & { sourceIds: string }>();
    return results.map((row) => ({ ...row, sourceIds: listIds(row.sourceIds) }));
  }
  async listTags(): Promise<Tag[]> {
    const { results } = await this.db.prepare("SELECT id, value, slug, source_ids AS sourceIds FROM tags ORDER BY value").all<Omit<Tag, "sourceIds"> & { sourceIds: string }>();
    return results.map((row) => ({ ...row, sourceIds: listIds(row.sourceIds) }));
  }
  async search(query: string): Promise<Array<{ id: string; type: string; name: string; slug: string }>> {
    const needle = `%${query.trim().replace(/[%_]/g, "\\$&").slice(0, 120)}%`;
    const { results } = await this.db.prepare(`
      SELECT id, 'MAISON' AS type, name, slug FROM maisons WHERE deleted_at IS NULL AND (name LIKE ? ESCAPE '\\' OR slug LIKE ? ESCAPE '\\')
      UNION ALL SELECT id, 'COLLECTION', name, slug FROM collections WHERE deleted_at IS NULL AND (name LIKE ? ESCAPE '\\' OR slug LIKE ? ESCAPE '\\')
      ORDER BY name LIMIT 50`).bind(needle, needle, needle, needle).all<{ id: string; type: string; name: string; slug: string }>();
    return results;
  }
}

export interface VersionedMutation { entityType: string; entityId: string; revision: number; updatedAt: string; }
export class IdempotencyLedger {
  private readonly revisions = new Map<string, number>();
  public accept(mutation: VersionedMutation): boolean {
    const key = `${mutation.entityType}:${mutation.entityId}`;
    const current = this.revisions.get(key) ?? 0;
    if (mutation.revision <= current) return false;
    this.revisions.set(key, mutation.revision);
    return true;
  }
}
