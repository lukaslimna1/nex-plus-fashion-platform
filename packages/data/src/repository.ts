import type { Asset, CityHub, Collection, Country, Edition, Event, Maison, ProfessionalReview, Region, ScheduleEntry, Source, Tag, Term } from "@nex-plus/types";

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
  listAssets(): Promise<Asset[]>;
  listSources(): Promise<Source[]>;
  listReviews(): Promise<ProfessionalReview[]>;
  listTerms(): Promise<Term[]>;
  listTags(): Promise<Tag[]>;
  search(query: string): Promise<Array<{ id: string; type: string; name: string; slug: string }>>;
}

function asBoolean(value: unknown): boolean { return value === 1 || value === true; }
function listIds(value: unknown): string[] { return typeof value === "string" && value ? value.split(",") : []; }

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
    return results.map((row) => ({ ...row, officialSourceIds: listIds(row.sourceIds) }));
  }
  async listCollections(): Promise<Collection[]> {
    const { results } = await this.db.prepare("SELECT id, maison_id AS maisonId, edition_id AS editionId, name, slug, calendar_year AS calendarYear, season_year AS seasonYear, season_code AS seasonCode, season_label AS seasonLabel, presented_on AS presentedOn, canonical_status AS canonicalStatus, source_ids AS sourceIds FROM collections WHERE deleted_at IS NULL ORDER BY presented_on DESC, name").all<Omit<Collection, "sourceIds"> & { sourceIds: string }>();
    return results.map((row) => ({ ...row, sourceIds: listIds(row.sourceIds) }));
  }
  async listAssets(): Promise<Asset[]> {
    const { results } = await this.db.prepare("SELECT id, collection_id AS collectionId, title, source_page_url AS sourcePageUrl, remote_url AS remoteUrl, embed_url AS embedUrl, provider, provider_asset_id AS providerAssetId, thumbnail_url AS thumbnailUrl, alternative_urls AS alternativeUrls, creator, photographer, credit_line AS creditLine, source_ids AS sourceIds, rights_status AS rightsStatus, download_policy AS downloadPolicy, display_mode AS displayMode, local_path AS localPath, cache_url AS cacheUrl, canonical_status AS canonicalStatus FROM assets WHERE deleted_at IS NULL ORDER BY id").all<Asset & { sourceIds: string; alternativeUrls: string }>();
    return results.map((row) => ({ ...row, sourceIds: listIds(row.sourceIds), alternativeUrls: listIds(row.alternativeUrls) }));
  }
  async listSources(): Promise<Source[]> {
    const { results } = await this.db.prepare("SELECT id, canonical_name AS canonicalName, type, base_url AS baseUrl, authority_tier AS authorityTier, language, coverage_scope AS coverageScope, region_id AS regionId, country_id AS countryId, access_mode AS accessMode, rights_notes AS rightsNotes, automation_notes AS automationNotes, last_verified_at AS lastVerifiedAt, active FROM sources WHERE deleted_at IS NULL ORDER BY canonical_name").all<Omit<Source, "language" | "coverageScope" | "active"> & { language: string; coverageScope: string; active: number }>();
    return results.map((row) => ({ ...row, language: row.language.split(","), coverageScope: row.coverageScope.split(","), active: asBoolean(row.active) }));
  }
  async listReviews(): Promise<ProfessionalReview[]> {
    const { results } = await this.db.prepare("SELECT id, collection_id AS collectionId, source_id AS sourceId, title, url, published_at AS publishedAt, language, canonical_status AS canonicalStatus FROM professional_reviews ORDER BY published_at DESC").all<ProfessionalReview>();
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
