import type { Asset, CityHub, Collection, CollectionDetail, Country, CoverageType, Edition, Event, ImageGroup, Maison, MaisonDetail, ProfessionalReview, Region, ScheduleEntry, Source, Tag, Term } from "@nex-plus/types";

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
  listCities(): Promise<CityHub[]>;
  listEvents(): Promise<Event[]>;
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

function asBoolean(value: unknown): boolean { return value === 1 || value === true; }
function listIds(value: unknown): string[] { return typeof value === "string" && value ? value.split(",") : []; }
type AssetRow = Omit<Asset, "sourceIds" | "alternativeUrls" | "attributionRequired" | "embedAllowed" | "remoteRenderAllowed" | "rehostAllowed"> & {
  sourceIds: string; alternativeUrls: string; attributionRequired: number; embedAllowed: number; remoteRenderAllowed: number; rehostAllowed: number;
};

export class D1CatalogRepository implements CatalogRepository {
  public constructor(private readonly db: D1DatabaseLike) {}

  async listRegions(): Promise<Region[]> {
    const { results } = await this.db.prepare("SELECT id, name, slug FROM regions WHERE deleted_at IS NULL ORDER BY name").all<Region>();
    return results;
  }
  async listCountries(): Promise<Country[]> {
    const { results } = await this.db.prepare("SELECT id, region_id AS regionId, name, iso_code AS isoCode, slug FROM countries WHERE deleted_at IS NULL ORDER BY name").all<Country>();
    return results;
  }
  async listCities(): Promise<CityHub[]> {
    const { results } = await this.db.prepare("SELECT id, name, slug, country_id AS countryId, region_id AS regionId, administrative_area_id AS administrativeAreaId, timezone, latitude, longitude FROM city_hubs WHERE deleted_at IS NULL ORDER BY name").all<CityHub>();
    return results;
  }
  async listEvents(): Promise<Event[]> {
    const { results } = await this.db.prepare("SELECT id, name, slug, kind, official_url AS officialUrl FROM events WHERE deleted_at IS NULL ORDER BY name").all<Event>();
    return results;
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
    const query = `SELECT a.id, a.collection_id AS collectionId, a.title, a.source_page_url AS sourcePageUrl, a.remote_url AS remoteUrl, a.embed_url AS embedUrl, a.provider, a.provider_asset_id AS providerAssetId, a.thumbnail_url AS thumbnailUrl, a.alternative_urls AS alternativeUrls, a.creator, a.photographer, a.credit_line AS creditLine, a.source_ids AS sourceIds, a.rights_status AS rightsStatus, a.download_policy AS downloadPolicy, a.display_mode AS displayMode, a.local_path AS localPath, a.cache_url AS cacheUrl, a.canonical_status AS canonicalStatus, a.asset_kind AS assetKind, COALESCE(a.coverage_type_label, a.coverage_type) AS coverageType, a.coverage_scope AS coverageScope, a.copyright_holder AS copyrightHolder, a.license_name AS licenseName, a.license_url AS licenseUrl, a.attribution_required AS attributionRequired, a.embed_allowed AS embedAllowed, a.remote_render_allowed AS remoteRenderAllowed, a.rehost_allowed AS rehostAllowed, a.verified_at AS verifiedAt, a.sequence_number AS sequenceNumber, a.look_number AS lookNumber, a.canonical_url AS canonicalUrl, a.channel_name AS channelName, a.duration_seconds AS durationSeconds, a.published_at AS publishedAt, a.video_type AS videoType, a.completeness, a.officiality FROM assets a LEFT JOIN collections c ON c.id = a.collection_id WHERE ${conditions.join(" AND ")} ORDER BY COALESCE(a.sequence_number, 999999), a.id`;
    const { results } = await this.db.prepare(query).bind(...values).all<AssetRow>();
    return results.map((row) => {
      const sourceIds = listIds(row.sourceIds);
      return { ...row, sourceIds, ...(sourceIds[0] ? { sourceId: sourceIds[0] } : {}), alternativeUrls: listIds(row.alternativeUrls), attributionRequired: asBoolean(row.attributionRequired), embedAllowed: asBoolean(row.embedAllowed), remoteRenderAllowed: asBoolean(row.remoteRenderAllowed), rehostAllowed: asBoolean(row.rehostAllowed) };
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
      tags
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
