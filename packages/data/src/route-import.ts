import type { CoverMatchStatus, CoverReference, Event, ImportStatus, ResearchCompleteness, SourceType } from "@nex-plus/types";

export type RouteNotionRow = Record<string, unknown>;

export interface RouteNotionSnapshot {
  schemaVersion?: string;
  capturedAt?: string;
  authority?: string;
  dataSources?: { cities?: string; events?: string };
  cities: RouteNotionRow[];
  events: RouteNotionRow[];
}

export interface RouteCoverMatch {
  notionPageId: string;
  name: string;
  coverAssetKey: string;
  url: string;
  status: Exclude<CoverMatchStatus, "MISSING">;
  fallback: false;
  sourceFile: string;
}

export interface RouteCoverManifest {
  fallback: { coverAssetKey: string; url: string; sourceFile: string; status: string; fallback: true };
  cityMatches: RouteCoverMatch[];
  eventMatches: RouteCoverMatch[];
}

export interface RouteImportOptions {
  importedAt?: string;
  coverManifest?: RouteCoverManifest;
  existing?: {
    citiesByNotionPageId?: Record<string, { id: string; sourceHash?: string; importStatus?: ImportStatus }>;
    eventsByNotionPageId?: Record<string, { id: string; sourceHash?: string; importStatus?: ImportStatus }>;
  };
  legacyCityIds?: Record<string, string>;
  legacyEventIds?: Record<string, string>;
  legacyRegionIds?: Record<string, string>;
  legacyCountryIds?: Record<string, string>;
}

export interface RouteRegionRecord { id: string; name: string; slug: string; }
export interface RouteCountryRecord { id: string; regionId: string; name: string; slug: string; isoCode?: string | undefined; storageIsoCode: string; }
export interface RouteSourceRecord {
  id: string; canonicalName: string; type: SourceType; baseUrl: string; authorityTier: "A" | "B" | "C" | "D";
  language: string[]; coverageScope: string[]; accessMode: "PUBLIC" | "UNKNOWN"; rightsNotes: string;
  automationNotes: string; lastVerifiedAt?: string | undefined;
}
export interface RouteCityRecord {
  id: string; name: string; slug: string; countryId: string; regionId: string; timezone?: string | undefined;
  countryName: string; countryCode?: string | undefined; regionName: string; subregion?: string | undefined; aliases: string[];
  relatedEventIds: string[]; researchStatus: ResearchCompleteness; hubImportance?: string | undefined;
  primarySourceId?: string | undefined; complementarySourceIds: string[]; notes?: string | undefined; cover: CoverReference;
  notionPageId: string; notionUrl: string; notionLastEditedAt?: string | undefined; sourceHash: string;
  lastImportedAt: string; importStatus: ImportStatus;
}
export interface RouteEventRecord {
  id: string; name: string; slug: string; kind: Event["kind"]; officialUrl?: string | undefined; eventType?: string | undefined;
  aliases: string[]; cityHubIds: string[]; currentStatus?: string | undefined; knownStartYear?: number | undefined;
  knownEndYear?: number | undefined; activeSince2010?: boolean | undefined; organizer?: string | undefined; usualPeriod?: string | undefined;
  lastVerifiedYear?: number | undefined; nextEditionAnnounced?: { start?: string | undefined; end?: string | undefined } | undefined;
  historicalRelation?: string | undefined; about?: string | undefined; historySummary?: string | undefined; verifiedSummaryAt?: string | undefined;
  notes?: string | undefined; socials: Record<string, string>; primarySourceId?: string | undefined; complementarySourceIds: string[];
  cover: CoverReference; researchStatus: ResearchCompleteness; notionPageId: string; notionUrl: string;
  notionLastEditedAt?: string | undefined; sourceHash: string; lastImportedAt: string; importStatus: ImportStatus;
  cityPageIds: string[];
}
export interface RouteEventLocation { eventId: string; cityHubId: string; sourceId?: string; }
export interface RouteImportPlan {
  importedAt: string; regions: RouteRegionRecord[]; countries: RouteCountryRecord[]; sources: RouteSourceRecord[];
  cities: RouteCityRecord[]; events: RouteEventRecord[]; locations: RouteEventLocation[];
  warnings: string[];
}

function text(value: unknown): string | undefined {
  if (typeof value !== "string") return undefined;
  const trimmed = value.trim();
  return trimmed ? trimmed : undefined;
}
function number(value: unknown): number | undefined {
  return typeof value === "number" && Number.isFinite(value) ? value : undefined;
}
function bool(value: unknown): boolean | undefined {
  if (value === true || value === "__YES__") return true;
  if (value === false || value === "__NO__") return false;
  return undefined;
}
function civilDate(value: unknown): string | undefined {
  const item = text(value);
  return item && /^\d{4}-\d{2}-\d{2}$/.test(item) ? item : undefined;
}
function arrayFromText(value: unknown): string[] {
  if (Array.isArray(value)) return value.flatMap((item) => arrayFromText(item));
  const item = text(value);
  if (!item) return [];
  return Array.from(new Set(item.split(/[;,|]/).map((part) => part.trim()).filter(Boolean)));
}
function pageIds(value: unknown): string[] {
  let values: unknown[] = [];
  if (Array.isArray(value)) values = value;
  else if (typeof value === "string") {
    try { const parsed: unknown = JSON.parse(value); values = Array.isArray(parsed) ? parsed : [value]; }
    catch { values = [value]; }
  }
  return Array.from(new Set(values.map((item) => text(item)).filter((item): item is string => Boolean(item)).map((item) => item.replace(/\?.*$/, "").split("/").pop()!)));
}
function urls(value: unknown): string[] {
  const item = text(value);
  if (!item) return [];
  const matches = item.match(/https?:\/\/[^\s,;]+/g);
  return matches ?? (/^https?:\/\//.test(item) ? [item] : []);
}

export function slugify(value: string): string {
  return value.normalize("NFD").replace(/[\u0300-\u036f]/g, "").toLowerCase().replace(/&/g, " and ").replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "") || "unknown";
}
function stableValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stableValue);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).sort(([a], [b]) => a.localeCompare(b)).map(([key, item]) => [key, stableValue(item)]));
  return value;
}
export function sourceHash(value: unknown): string {
  const input = JSON.stringify(stableValue(value));
  let hash = 2166136261;
  for (let index = 0; index < input.length; index += 1) { hash ^= input.charCodeAt(index); hash = Math.imul(hash, 16777619); }
  return `fnv1a32-${(hash >>> 0).toString(16).padStart(8, "0")}`;
}
function sourceId(url: string): string {
  return `source-route-${sourceHash(url).slice(-8)}`;
}
function coverFor(match: RouteCoverMatch | undefined, manifest: RouteCoverManifest): CoverReference {
  if (match) return { assetKey: match.coverAssetKey, url: match.url, status: match.status, fallback: false };
  return { assetKey: manifest.fallback.coverAssetKey, url: manifest.fallback.url, status: "MISSING", fallback: true };
}
function researchForCity(value: unknown): ResearchCompleteness {
  switch (text(value)) {
    case "Verificado 2010+": return "COMPLETE";
    case "Histórico 2010+": return "PARTIAL";
    case "Candidato 2010+": return "NEEDS_RESEARCH";
    default: return "UNVERIFIED";
  }
}
function researchForEvent(row: RouteNotionRow): ResearchCompleteness {
  if (text(row["Status atual"]) === "A verificar") return "UNVERIFIED";
  if (text(row["Site oficial"]) && number(row["Último ano verificado"])) return "COMPLETE";
  return "PARTIAL";
}
function importState(researchStatus: ResearchCompleteness, sourceHashValue: string, existing: { sourceHash?: string; importStatus?: ImportStatus } | undefined): ImportStatus {
  if (researchStatus === "UNVERIFIED" || researchStatus === "NEEDS_RESEARCH") return "REVIEW_REQUIRED";
  if (!existing) return "NEW";
  if (existing.importStatus === "REVIEW_REQUIRED") return "REVIEW_REQUIRED";
  return existing.sourceHash === sourceHashValue ? "UNCHANGED" : "UPDATED";
}
function eventKind(type: unknown, name: string): Event["kind"] {
  if (text(type) === "Haute Couture" || /haute couture/i.test(name)) return "HAUTE_COUTURE_WEEK";
  if (text(type) === "Fashion Week" || /fashion week|fashionweek|runway/i.test(name)) return "FASHION_WEEK";
  return "OTHER";
}
function sourceRecord(url: string, type: SourceType, lastVerifiedAt?: string): RouteSourceRecord {
  let host = "source";
  try { host = new URL(url).hostname.replace(/^www\./, ""); } catch { /* source URL validation remains a review concern */ }
  return {
    id: sourceId(url), canonicalName: host, type, baseUrl: url, authorityTier: "D", language: [], coverageScope: ["route-global"],
    accessMode: "UNKNOWN", rightsNotes: "Editorial provenance only; media download or rehosting permission is not inferred.",
    automationNotes: "Imported from the Notion Route Global source registry; revalidate before automated fetching.",
    ...(lastVerifiedAt ? { lastVerifiedAt } : {})
  };
}

export function buildRouteImportPlan(snapshot: RouteNotionSnapshot, options: RouteImportOptions = {}): RouteImportPlan {
  if (!Array.isArray(snapshot.cities) || !Array.isArray(snapshot.events)) throw new Error("Route snapshot must contain cities and events arrays");
  const importedAt = options.importedAt ?? snapshot.capturedAt ?? new Date().toISOString();
  const manifest = options.coverManifest ?? { fallback: { coverAssetKey: "cover-system-em-breve", url: "/assets/covers/system/cover-system-em-breve.png", sourceFile: "System/EM BREVE.png", status: "FALLBACK", fallback: true }, cityMatches: [], eventMatches: [] };
  const warnings: string[] = [];
  const regionsBySlug = new Map<string, RouteRegionRecord>();
  const countriesBySlug = new Map<string, RouteCountryRecord>();
  const sourcesByUrl = new Map<string, RouteSourceRecord>();
  const registerSource = (url: string | undefined, type: SourceType, editedAt?: string): string | undefined => {
    if (!url) return undefined;
    if (!sourcesByUrl.has(url)) sourcesByUrl.set(url, sourceRecord(url, type, editedAt));
    return sourcesByUrl.get(url)!.id;
  };
  const regionIdFor = (name: string): string => options.legacyRegionIds?.[slugify(name)] ?? `region-${slugify(name)}`;
  const countryIdFor = (name: string): string => options.legacyCountryIds?.[slugify(name)] ?? `country-route-${slugify(name)}`;
  const cityIdByPageId = new Map<string, string>();
  const cityIdBySlug = new Map<string, string>();
  const cities: RouteCityRecord[] = [];
  for (const row of snapshot.cities) {
    const name = text(row.Cidade);
    const notionPageId = text(row.notionPageId ?? row.id);
    const notionUrl = text(row.notionUrl ?? row.url);
    const countryName = text(row["País/Território"]);
    const regionName = text(row.Região);
    if (!name || !notionPageId || !notionUrl || !countryName || !regionName) { warnings.push(`city missing identity or geography: ${notionPageId ?? "unknown"}`); continue; }
    const slug = slugify(name);
    const id = options.existing?.citiesByNotionPageId?.[notionPageId]?.id ?? options.legacyCityIds?.[slug] ?? `city-route-${notionPageId.replace(/-/g, "")}`;
    if (cityIdBySlug.has(slug) && cityIdBySlug.get(slug) !== id) warnings.push(`duplicate city slug: ${slug}`);
    cityIdByPageId.set(notionPageId.replace(/-/g, ""), id); cityIdByPageId.set(notionPageId, id); cityIdBySlug.set(slug, id);
    const regionId = regionIdFor(regionName);
    if (!regionsBySlug.has(slugify(regionName))) regionsBySlug.set(slugify(regionName), { id: regionId, name: regionName, slug: slugify(regionName) });
    const countrySlug = slugify(countryName);
    const countryCode = text(row["Código país"]);
    if (!countriesBySlug.has(countrySlug)) countriesBySlug.set(countrySlug, { id: countryIdFor(countryName), regionId, name: countryName, slug: countrySlug, ...(countryCode ? { isoCode: countryCode } : {}), storageIsoCode: countryCode ?? `NOTION_UNSET:${countrySlug}` });
    else if (countryCode && !countriesBySlug.get(countrySlug)!.isoCode) { countriesBySlug.get(countrySlug)!.isoCode = countryCode; countriesBySlug.get(countrySlug)!.storageIsoCode = countryCode; }
    const primaryUrl = text(row["Fonte-base"]);
    const complementaryUrls = urls(row["Fonte complementar"]);
    const primarySourceId = registerSource(primaryUrl, "OTHER", text(row.notionLastEditedAt));
    const complementarySourceIds = complementaryUrls.map((url) => registerSource(url, "OTHER", text(row.notionLastEditedAt))!).filter(Boolean);
    const aliases = arrayFromText(row["Aliases/Grafias"]);
    const researchStatus = researchForCity(row["Status de pesquisa"]);
    const hash = sourceHash({ ...row, notionPageId, notionUrl });
    const existing = options.existing?.citiesByNotionPageId?.[notionPageId];
    const match = manifest.cityMatches.find((item) => item.notionPageId === notionPageId);
    cities.push({
      id, name, slug, countryId: countryIdFor(countryName), regionId, countryName, ...(countryCode ? { countryCode } : {}), regionName,
      ...(text(row["Sub-região"]) ? { subregion: text(row["Sub-região"]) } : {}), aliases, relatedEventIds: [], researchStatus,
      ...(text(row["Importância do hub"]) ? { hubImportance: text(row["Importância do hub"]) } : {}), ...(primarySourceId ? { primarySourceId } : {}),
      complementarySourceIds, ...(text(row.Observações) ? { notes: text(row.Observações) } : {}), cover: coverFor(match, manifest), notionPageId,
      notionUrl, ...(text(row.notionLastEditedAt) ? { notionLastEditedAt: text(row.notionLastEditedAt) } : {}), sourceHash: hash, lastImportedAt: importedAt,
      importStatus: importState(researchStatus, hash, existing)
    });
  }
  const eventIdByPageId = new Map<string, string>();
  const events: RouteEventRecord[] = [];
  for (const row of snapshot.events) {
    const name = text(row.Evento); const notionPageId = text(row.notionPageId ?? row.id); const notionUrl = text(row.notionUrl ?? row.url);
    if (!name || !notionPageId || !notionUrl) { warnings.push(`event missing identity: ${notionPageId ?? "unknown"}`); continue; }
    const slug = slugify(name); const id = options.existing?.eventsByNotionPageId?.[notionPageId]?.id ?? options.legacyEventIds?.[slug] ?? `event-route-${notionPageId.replace(/-/g, "")}`;
    eventIdByPageId.set(notionPageId.replace(/-/g, ""), id); eventIdByPageId.set(notionPageId, id);
    const cityPageIds = pageIds(row.Cidade);
    const cityHubIds = cityPageIds.map((pageId) => cityIdByPageId.get(pageId) ?? cityIdByPageId.get(pageId.replace(/-/g, ""))).filter((item): item is string => Boolean(item));
    if (cityPageIds.length !== cityHubIds.length) warnings.push(`event relation unresolved: ${name}`);
    const baseUrl = text(row["Fonte-base"]); const officialUrl = text(row["Site oficial"]);
    const primarySourceId = registerSource(baseUrl, officialUrl ? "OFFICIAL_ORGANIZER" : "OTHER", text(row.notionLastEditedAt));
    const complementaryUrls = [text(row["Fonte complementar"]), officialUrl, text(row.Instagram), text(row.Facebook), text(row.LinkedIn), text(row["X / Twitter"]), text(row.YouTube), text(row.TikTok), ...urls(row["Outras redes"])].filter((item): item is string => Boolean(item));
    const complementarySourceIds = Array.from(new Set(complementaryUrls.map((url) => registerSource(url, url === text(row.Instagram) || url === text(row.Facebook) || url === text(row.LinkedIn) || url === text(row["X / Twitter"]) || url === text(row.YouTube) || url === text(row.TikTok) ? "SOCIAL_NETWORK" : "OTHER", text(row.notionLastEditedAt))!).filter(Boolean)));
    const aliases = arrayFromText(row["Aliases/Nomes anteriores"]); const researchStatus = researchForEvent(row); const hash = sourceHash({ ...row, notionPageId, notionUrl });
    const existing = options.existing?.eventsByNotionPageId?.[notionPageId]; const match = manifest.eventMatches.find((item) => item.notionPageId === notionPageId);
    const start = civilDate(row["date:Próxima edição anunciada:start"]); const end = civilDate(row["date:Próxima edição anunciada:end"]); const verified = civilDate(row["date:Resumo verificado em:start"]);
    const socials: Record<string, string> = {};
    for (const [key, label] of [["Instagram", "instagram"], ["Facebook", "facebook"], ["LinkedIn", "linkedin"], ["X / Twitter", "x"], ["YouTube", "youtube"], ["TikTok", "tiktok"]] as const) { const value = text(row[key]); if (value) socials[label] = value; }
    events.push({
      id, name, slug, kind: eventKind(row.Tipo, name), ...(officialUrl ? { officialUrl } : {}), ...(text(row.Tipo) ? { eventType: text(row.Tipo) } : {}), aliases, cityHubIds,
      ...(text(row["Status atual"]) ? { currentStatus: text(row["Status atual"]) } : {}), ...(number(row["Ano inicial conhecido"]) !== undefined ? { knownStartYear: number(row["Ano inicial conhecido"]) } : {}),
      ...(number(row["Ano final conhecido"]) !== undefined ? { knownEndYear: number(row["Ano final conhecido"]) } : {}), ...(bool(row["Atividade desde 2010"]) !== undefined ? { activeSince2010: bool(row["Atividade desde 2010"]) } : {}),
      ...(text(row.Organizador) ? { organizer: text(row.Organizador) } : {}), ...(text(row["Período habitual"]) ? { usualPeriod: text(row["Período habitual"]) } : {}),
      ...(number(row["Último ano verificado"]) !== undefined ? { lastVerifiedYear: number(row["Último ano verificado"]) } : {}), ...((start || end) ? { nextEditionAnnounced: { ...(start ? { start } : {}), ...(end ? { end } : {}) } } : {}),
      ...(text(row["Relação histórica"]) ? { historicalRelation: text(row["Relação histórica"]) } : {}), ...(text(row["Sobre o evento"]) ? { about: text(row["Sobre o evento"]) } : {}),
      ...(text(row["História resumida"]) ? { historySummary: text(row["História resumida"]) } : {}), ...(verified ? { verifiedSummaryAt: verified } : {}), ...(text(row.Observações) ? { notes: text(row.Observações) } : {}),
      socials, ...(primarySourceId ? { primarySourceId } : {}), complementarySourceIds, cover: coverFor(match, manifest), researchStatus, notionPageId, notionUrl,
      ...(text(row.notionLastEditedAt) ? { notionLastEditedAt: text(row.notionLastEditedAt) } : {}), sourceHash: hash, lastImportedAt: importedAt, importStatus: importState(researchStatus, hash, existing), cityPageIds
    });
  }
  const eventsByCityPage = new Map<string, string[]>();
  for (const event of events) for (const pageId of event.cityPageIds) eventsByCityPage.set(pageId, [...(eventsByCityPage.get(pageId) ?? []), event.id]);
  for (const city of cities) city.relatedEventIds = eventsByCityPage.get(city.notionPageId) ?? eventsByCityPage.get(city.notionPageId.replace(/-/g, "")) ?? [];
  const locations: RouteEventLocation[] = events.flatMap((event) => event.cityHubIds.map((cityHubId) => ({ eventId: event.id, cityHubId, ...(event.primarySourceId ? { sourceId: event.primarySourceId } : {}) })));
  return { importedAt, regions: Array.from(regionsBySlug.values()).sort((a, b) => a.slug.localeCompare(b.slug)), countries: Array.from(countriesBySlug.values()).sort((a, b) => a.slug.localeCompare(b.slug)), sources: Array.from(sourcesByUrl.values()).sort((a, b) => a.id.localeCompare(b.id)), cities, events, locations, warnings };
}

export function coverManifestStatus(plan: RouteImportPlan): Record<string, number> {
  return [...plan.cities, ...plan.events].reduce<Record<string, number>>((result, item) => { const key = item.cover.status; result[key] = (result[key] ?? 0) + 1; return result; }, {});
}

function sql(value: unknown): string {
  if (value === undefined || value === null) return "NULL";
  if (typeof value === "number") return Number.isFinite(value) ? String(value) : "NULL";
  if (typeof value === "boolean") return value ? "1" : "0";
  return `'${String(value).replace(/'/g, "''")}'`;
}
function json(value: unknown): string { return sql(JSON.stringify(value ?? [])); }
function columns(values: unknown[]): string { return values.map(sql).join(", "); }

export function renderRouteImportSql(plan: RouteImportPlan): string {
  const out: string[] = [`-- Generated from the Notion Route Global snapshot at ${plan.importedAt}`, "PRAGMA foreign_keys = ON;"];
  for (const region of plan.regions) out.push(`INSERT INTO regions (id,name,slug,created_at,updated_at) VALUES (${columns([region.id,region.name,region.slug,plan.importedAt,plan.importedAt])}) ON CONFLICT(id) DO UPDATE SET name=excluded.name,slug=excluded.slug,updated_at=excluded.updated_at;`);
  for (const country of plan.countries) out.push(`INSERT INTO countries (id,region_id,name,iso_code,slug,created_at,updated_at) VALUES (${columns([country.id,country.regionId,country.name,country.storageIsoCode,country.slug,plan.importedAt,plan.importedAt])}) ON CONFLICT(id) DO UPDATE SET region_id=excluded.region_id,name=excluded.name,iso_code=excluded.iso_code,slug=excluded.slug,updated_at=excluded.updated_at;`);
  for (const source of plan.sources) out.push(`INSERT INTO sources (id,canonical_name,type,base_url,authority_tier,language,coverage_scope,access_mode,rights_notes,automation_notes,last_verified_at,active,created_at,updated_at) VALUES (${columns([source.id,source.canonicalName,source.type,source.baseUrl,source.authorityTier,source.language.join(","),source.coverageScope.join(","),source.accessMode,source.rightsNotes,source.automationNotes,source.lastVerifiedAt,1,plan.importedAt,plan.importedAt])}) ON CONFLICT(id) DO UPDATE SET canonical_name=excluded.canonical_name,type=excluded.type,base_url=excluded.base_url,authority_tier=excluded.authority_tier,coverage_scope=excluded.coverage_scope,access_mode=excluded.access_mode,rights_notes=excluded.rights_notes,automation_notes=excluded.automation_notes,last_verified_at=excluded.last_verified_at,active=1,updated_at=excluded.updated_at;`);
  for (const city of plan.cities) out.push(`INSERT INTO city_hubs (id,country_id,region_id,name,slug,timezone,country_name,country_code,subregion,aliases_json,related_event_ids, research_status,hub_importance,primary_source_id,complementary_source_ids,notes,cover_asset_key,cover_url,cover_match_status,cover_fallback,notion_page_id,notion_url,notion_last_edited_at,source_hash,last_imported_at,import_status,created_at,updated_at) VALUES (${columns([city.id,city.countryId,city.regionId,city.name,city.slug,city.timezone ?? "UNKNOWN",city.countryName,city.countryCode,city.subregion,json(city.aliases),city.relatedEventIds.join(","),city.researchStatus,city.hubImportance,city.primarySourceId,city.complementarySourceIds.join(","),city.notes,city.cover.assetKey,city.cover.url,city.cover.status,city.cover.fallback ? 1 : 0,city.notionPageId,city.notionUrl,city.notionLastEditedAt,city.sourceHash,city.lastImportedAt,city.importStatus,city.lastImportedAt,city.lastImportedAt])}) ON CONFLICT(id) DO UPDATE SET country_id=excluded.country_id,region_id=excluded.region_id,name=excluded.name,slug=excluded.slug,timezone=excluded.timezone,country_name=excluded.country_name,country_code=excluded.country_code,subregion=excluded.subregion,aliases_json=excluded.aliases_json,related_event_ids=excluded.related_event_ids,research_status=excluded.research_status,hub_importance=excluded.hub_importance,primary_source_id=excluded.primary_source_id,complementary_source_ids=excluded.complementary_source_ids,notes=excluded.notes,cover_asset_key=excluded.cover_asset_key,cover_url=excluded.cover_url,cover_match_status=excluded.cover_match_status,cover_fallback=excluded.cover_fallback,notion_page_id=excluded.notion_page_id,notion_url=excluded.notion_url,notion_last_edited_at=excluded.notion_last_edited_at,source_hash=excluded.source_hash,last_imported_at=excluded.last_imported_at,import_status=CASE WHEN excluded.import_status='REVIEW_REQUIRED' THEN 'REVIEW_REQUIRED' WHEN city_hubs.source_hash IS NULL THEN excluded.import_status WHEN city_hubs.source_hash=excluded.source_hash THEN 'UNCHANGED' ELSE 'UPDATED' END,updated_at=excluded.updated_at;`);
  for (const event of plan.events) out.push(`INSERT INTO events (id,name,slug,kind,official_url,event_type,aliases_json,city_hub_ids,current_status,known_start_year,known_end_year,active_since_2010,organizer,usual_period,last_verified_year,next_edition_announced_json,historical_relation,about,history_summary,verified_summary_at,notes,socials_json,primary_source_id,complementary_source_ids,cover_asset_key,cover_url,cover_match_status,cover_fallback,research_status,notion_page_id,notion_url,notion_last_edited_at,source_hash,last_imported_at,import_status,created_at,updated_at) VALUES (${columns([event.id,event.name,event.slug,event.kind,event.officialUrl,event.eventType,json(event.aliases),event.cityHubIds.join(","),event.currentStatus,event.knownStartYear,event.knownEndYear,event.activeSince2010,event.organizer,event.usualPeriod,event.lastVerifiedYear,json(event.nextEditionAnnounced),event.historicalRelation,event.about,event.historySummary,event.verifiedSummaryAt,event.notes,json(event.socials),event.primarySourceId,event.complementarySourceIds.join(","),event.cover.assetKey,event.cover.url,event.cover.status,event.cover.fallback ? 1 : 0,event.researchStatus,event.notionPageId,event.notionUrl,event.notionLastEditedAt,event.sourceHash,event.lastImportedAt,event.importStatus,event.lastImportedAt,event.lastImportedAt])}) ON CONFLICT(id) DO UPDATE SET name=excluded.name,slug=excluded.slug,kind=excluded.kind,official_url=excluded.official_url,event_type=excluded.event_type,aliases_json=excluded.aliases_json,city_hub_ids=excluded.city_hub_ids,current_status=excluded.current_status,known_start_year=excluded.known_start_year,known_end_year=excluded.known_end_year,active_since_2010=excluded.active_since_2010,organizer=excluded.organizer,usual_period=excluded.usual_period,last_verified_year=excluded.last_verified_year,next_edition_announced_json=excluded.next_edition_announced_json,historical_relation=excluded.historical_relation,about=excluded.about,history_summary=excluded.history_summary,verified_summary_at=excluded.verified_summary_at,notes=excluded.notes,socials_json=excluded.socials_json,primary_source_id=excluded.primary_source_id,complementary_source_ids=excluded.complementary_source_ids,cover_asset_key=excluded.cover_asset_key,cover_url=excluded.cover_url,cover_match_status=excluded.cover_match_status,cover_fallback=excluded.cover_fallback,research_status=excluded.research_status,notion_page_id=excluded.notion_page_id,notion_url=excluded.notion_url,notion_last_edited_at=excluded.notion_last_edited_at,source_hash=excluded.source_hash,last_imported_at=excluded.last_imported_at,import_status=CASE WHEN excluded.import_status='REVIEW_REQUIRED' THEN 'REVIEW_REQUIRED' WHEN events.source_hash IS NULL THEN excluded.import_status WHEN events.source_hash=excluded.source_hash THEN 'UNCHANGED' ELSE 'UPDATED' END,updated_at=excluded.updated_at;`);
  for (const event of plan.events) out.push(`DELETE FROM event_locations WHERE event_id=${sql(event.id)};`);
  for (const location of plan.locations) out.push(`INSERT INTO event_locations (event_id,city_hub_id,source_id) VALUES (${columns([location.eventId,location.cityHubId,location.sourceId])}) ON CONFLICT(event_id,city_hub_id) DO UPDATE SET source_id=excluded.source_id;`);
  return `${out.join("\n")}\n`;
}
