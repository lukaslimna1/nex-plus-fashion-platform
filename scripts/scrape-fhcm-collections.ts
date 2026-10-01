import { mkdir, writeFile } from "node:fs/promises";
import { dirname } from "node:path";
import * as cheerio from "cheerio";

export const FHCM_COLLECTIONS_URL = "https://www.fhcm.paris/fr/paris-fashion-week/collections";
export const FHCM_CALENDAR_URL = "https://www.fhcm.paris/fr/paris-fashion-week/calendar";
export const FHCM_ORIGIN = "https://www.fhcm.paris";
export const FHCM_RETRIEVED_AT = "2026-09-30T00:00:00.000Z";

export interface FhcmCollectionIndexEntry {
  name: string;
  localDate: string;
  collectionUrl: string;
  coverUrl?: string;
  maisonUrl?: string;
  discoveryMethod?: "COLLECTIONS_INDEX" | "CALENDAR_MAISON";
}

export interface FhcmCalendarEntry {
  name: string;
  localDate: string;
  maisonUrl: string;
}

export interface FhcmMaisonCollectionLink {
  collectionUrl: string;
  label: string;
  coverUrl?: string;
}

export interface FhcmImageAsset {
  kind: "IMAGE";
  title: string;
  sourcePageUrl: string;
  remoteUrl: string;
  thumbnailUrl: string;
  alternativeUrls: string[];
  provider: "FHCM";
  providerAssetId: string;
  creator: "FHCM";
  photographer?: string;
  creditLine?: string;
  sequenceNumber: number;
  lookNumber?: number;
  coverageType: "RUNWAY" | "EDITORIAL";
  coverageScope: "LOOKBOOK_IMAGE" | "EDITORIAL";
  displayMode: "INLINE" | "THUMBNAIL_ONLY";
  deduplicationKey: string;
}

export interface FhcmVideoAsset {
  kind: "VIDEO";
  title: string;
  sourcePageUrl: string;
  embedUrl: string;
  canonicalUrl: string;
  alternativeUrls: string[];
  provider: string;
  providerAssetId: string;
  creator: "FHCM";
  creditLine?: string;
  coverageType: "EDITORIAL";
  coverageScope: "EDITORIAL";
  displayMode: "EMBED";
  videoType: "RUNWAY_COVERAGE";
  completeness: "UNKNOWN";
  officiality: "PROFESSIONAL_VERIFIED";
  playbackMode: "WEBSITE_EMBED";
  deduplicationKey: string;
}

export interface FhcmCollectionRecord extends FhcmCollectionIndexEntry {
  pageTitle: string;
  sourceCredit?: string;
  images: FhcmImageAsset[];
  videos: FhcmVideoAsset[];
  coverAsset?: FhcmImageAsset;
  pageHasLookbook: boolean;
}

export interface FhcmCollectionManifest {
  schemaVersion: "pfw-ss27-fhcm-collection-manifest-v1";
  source: "FHCM";
  calendarUrl: string;
  indexUrl: string;
  retrievedAt: string;
  scope: { from: string; to: string; edition: string };
  collections: FhcmCollectionRecord[];
  unlistedFromEditionIndex: Array<{ name: string; localDate: string; reason: string }>;
  summary: {
    calendarEntries: number;
    resolvedMaisonPages: number;
    resolvedCollectionPages: number;
    indexedCollectionPages: number;
    collectionsWithLookbooks: number;
    collectionsWithOnlyOfficialCover: number;
    galleryImages: number;
    officialCoverImages: number;
    embeddedVideos: number;
    totalImages: number;
  };
}

function absoluteUrl(value: string | undefined): string | undefined {
  if (!value) return undefined;
  try { return new URL(value, FHCM_ORIGIN).href; } catch { return undefined; }
}

function normalizedLabel(value: string): string {
  return value.normalize("NFD").replace(/[\u0300-\u036f]/g, "").toLowerCase().replace(/[^a-z0-9]+/g, " ").trim();
}

export function normalizeFhcmMediaUrl(value: string): string {
  const url = new URL(value, FHCM_ORIGIN);
  url.searchParams.delete("itok");
  return url.href;
}

function providerAssetId(value: string): string {
  const pathname = new URL(value).pathname;
  const marker = "/styles/lkt/public/";
  const coverMarker = "/styles/clist/public/";
  const start = pathname.indexOf(marker);
  if (start >= 0) return decodeURIComponent(pathname.slice(start + marker.length));
  const coverStart = pathname.indexOf(coverMarker);
  if (coverStart >= 0) return decodeURIComponent(pathname.slice(coverStart + coverMarker.length));
  return pathname.replace(/^\//, "");
}

function videoProvider(value: string): string {
  const host = new URL(value).hostname.toLowerCase();
  if (host === "player.castr.com") return "Castr";
  if (host.includes("youtube.com") || host === "youtu.be") return "YouTube";
  if (host.includes("vimeo.com")) return "Vimeo";
  return host;
}

function videoAssetId(value: string): string {
  const url = new URL(value);
  const path = url.hostname.includes("youtube.com") ? url.pathname.split("/").filter(Boolean).at(-1) ?? "" : url.pathname.replace(/^\//, "");
  return path || url.href;
}

function uniqueByDeduplicationKey<T extends { deduplicationKey: string }>(items: T[]): T[] {
  const seen = new Set<string>();
  return items.filter((item) => {
    if (seen.has(item.deduplicationKey)) return false;
    seen.add(item.deduplicationKey);
    return true;
  });
}

export function parseFhcmCollectionsIndex(html: string, origin = FHCM_ORIGIN): FhcmCollectionIndexEntry[] {
  const $ = cheerio.load(html);
  return $("div.edition-collections .list a[href*='/collection/']").map((_, element) => {
    const link = $(element);
    const collectionUrl = new URL(link.attr("href") ?? "", origin).href;
    const coverUrl = absoluteUrl(link.attr("data-img"));
    return {
      name: link.find("h2").first().text().trim().toUpperCase(),
      localDate: link.find("time").attr("datetime")?.slice(0, 10) ?? "",
      collectionUrl,
      ...(coverUrl ? { coverUrl } : {}),
      discoveryMethod: "COLLECTIONS_INDEX" as const
    };
  }).get().filter((entry) => entry.name && entry.localDate && entry.collectionUrl);
}

export function parseFhcmCalendar(html: string, origin = FHCM_ORIGIN): FhcmCalendarEntry[] {
  const $ = cheerio.load(html);
  return $(".calendar .day[data-day]").map((_, day) => {
    const localDate = $(day).attr("data-day")?.replace(/^(\d{4})(\d{2})(\d{2})$/, "$1-$2-$3") ?? "";
    return $(day).find(".cal-item .house-details").map((__, element) => {
      const link = $(element);
      const maisonUrl = absoluteUrl(link.attr("href")) ?? new URL(link.attr("href") ?? "", origin).href;
      return {
        name: link.find("h3").first().text().replace(/\s+/g, " ").trim().toUpperCase(),
        localDate,
        maisonUrl
      } satisfies FhcmCalendarEntry;
    }).get();
  }).get().filter((entry) => entry.localDate && entry.name && entry.maisonUrl);
}

export function parseFhcmMaisonCollections(html: string, maisonUrl: string): FhcmMaisonCollectionLink[] {
  const $ = cheerio.load(html);
  return [...new Map($("a[href*='/fr/collection/']").toArray().map((element) => {
    const link = $(element);
    const collectionUrl = new URL(link.attr("href") ?? "", maisonUrl).href;
    const label = link.text().replace(/\s+/g, " ").trim();
    const coverUrl = absoluteUrl(link.attr("data-img"));
    return [collectionUrl, { collectionUrl, label, ...(coverUrl ? { coverUrl } : {}) } satisfies FhcmMaisonCollectionLink];
  }))].map(([, value]) => value);
}

export function selectFhcmSs27WomenswearCollection(links: FhcmMaisonCollectionLink[]): FhcmMaisonCollectionLink | undefined {
  return links.find((link) => {
    const label = normalizedLabel(`${link.label} ${link.collectionUrl}`);
    return label.includes("2027") && label.includes("mode feminine") && label.includes("printemps ete");
  });
}

export function parseFhcmCollectionPage(entry: FhcmCollectionIndexEntry, html: string): FhcmCollectionRecord {
  const $ = cheerio.load(html);
  const article = $("article.collection").first();
  const sourceCredit = article.find(".field-credits .item").map((_, element) => $(element).text().trim()).get().filter(Boolean).join("; ") || undefined;
  const creditLine = sourceCredit ? `FHCM page credit: ${sourceCredit}` : undefined;
  const imageItems = article.find(".field-gallery img.image-style-lkt").map((index, element) => {
    const remoteUrl = absoluteUrl($(element).attr("src"));
    if (!remoteUrl) return undefined;
    const deduplicationKey = normalizeFhcmMediaUrl(remoteUrl);
    return {
      kind: "IMAGE" as const,
      title: `${entry.name} SS27 — FHCM look ${String(index + 1).padStart(2, "0")}`,
      sourcePageUrl: entry.collectionUrl,
      remoteUrl,
      thumbnailUrl: remoteUrl,
      alternativeUrls: [] as string[],
      provider: "FHCM" as const,
      providerAssetId: providerAssetId(remoteUrl),
      creator: "FHCM" as const,
      ...(creditLine ? { creditLine } : {}),
      sequenceNumber: index + 1,
      lookNumber: index + 1,
      coverageType: "RUNWAY" as const,
      coverageScope: "LOOKBOOK_IMAGE" as const,
      displayMode: "INLINE" as const,
      deduplicationKey
    } satisfies FhcmImageAsset;
  }).get().filter(Boolean) as FhcmImageAsset[];

  const coverUrl = entry.coverUrl;
  const coverAsset = coverUrl ? {
    kind: "IMAGE" as const,
    title: `${entry.name} SS27 — FHCM collection cover`,
    sourcePageUrl: entry.collectionUrl,
    remoteUrl: coverUrl,
    thumbnailUrl: coverUrl,
    alternativeUrls: [] as string[],
    provider: "FHCM" as const,
    providerAssetId: providerAssetId(coverUrl),
    creator: "FHCM" as const,
    ...(creditLine ? { creditLine } : {}),
    sequenceNumber: 0,
    coverageType: "EDITORIAL" as const,
    coverageScope: "EDITORIAL" as const,
    displayMode: "THUMBNAIL_ONLY" as const,
    deduplicationKey: normalizeFhcmMediaUrl(coverUrl)
  } satisfies FhcmImageAsset : undefined;

  const embedUrls = article.find("iframe[src], video source[src], video[src]").map((_, element) => {
    const value = $(element).attr("src");
    return absoluteUrl(value);
  }).get().filter((value): value is string => Boolean(value));
  const videos = uniqueByDeduplicationKey(embedUrls.map((embedUrl) => ({
    kind: "VIDEO" as const,
    title: `${entry.name} SS27 — FHCM embedded coverage`,
    sourcePageUrl: entry.collectionUrl,
    embedUrl,
    canonicalUrl: embedUrl,
    alternativeUrls: [] as string[],
    provider: videoProvider(embedUrl),
    providerAssetId: videoAssetId(embedUrl),
    creator: "FHCM" as const,
    ...(creditLine ? { creditLine } : {}),
    coverageType: "EDITORIAL" as const,
    coverageScope: "EDITORIAL" as const,
    displayMode: "EMBED" as const,
    videoType: "RUNWAY_COVERAGE" as const,
    completeness: "UNKNOWN" as const,
    officiality: "PROFESSIONAL_VERIFIED" as const,
    playbackMode: "WEBSITE_EMBED" as const,
    deduplicationKey: embedUrl
  } satisfies FhcmVideoAsset)));

  return {
    ...entry,
    pageTitle: article.attr("data-name") ?? article.find(".page-title").text().trim(),
    ...(sourceCredit ? { sourceCredit } : {}),
    images: uniqueByDeduplicationKey(imageItems),
    videos,
    ...(coverAsset ? { coverAsset } : {}),
    pageHasLookbook: imageItems.length > 0
  };
}

function slugForCollection(name: string): string {
  return name.normalize("NFKD").replace(/[\u0300-\u036f]/g, "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
}

function sql(value: string | number | boolean | null | undefined): string {
  if (value === null || value === undefined) return "NULL";
  if (typeof value === "number") return String(value);
  if (typeof value === "boolean") return value ? "1" : "0";
  return `'${value.replaceAll("'", "''")}'`;
}

async function headOk(url: string, request: typeof fetch): Promise<boolean> {
  try {
    const response = await request(url, { method: "HEAD", headers: { "user-agent": "NEX+ Fashion research collector/1.0" } });
    return response.ok;
  } catch {
    return false;
  }
}

export async function verifyFhcmAlternativeUrls(collections: FhcmCollectionRecord[], request: typeof fetch = fetch): Promise<FhcmCollectionRecord[]> {
  const candidates: Array<{ collectionIndex: number; kind: "image" | "cover"; imageIndex?: number; asset: FhcmImageAsset }> = [];
  collections.forEach((collection, collectionIndex) => {
    collection.images.forEach((asset, imageIndex) => candidates.push({ collectionIndex, kind: "image", imageIndex, asset }));
    if (collection.coverAsset) candidates.push({ collectionIndex, kind: "cover", asset: collection.coverAsset });
  });
  const approved = new Map<string, string>();
  for (let index = 0; index < candidates.length; index += 20) {
    const batch = candidates.slice(index, index + 20);
    const results = await Promise.all(batch.map(async ({ asset }) => {
      const candidate = normalizeFhcmMediaUrl(asset.remoteUrl);
      return candidate !== asset.remoteUrl && await headOk(candidate, request) ? [asset.remoteUrl, candidate] as const : undefined;
    }));
    results.forEach((result) => { if (result) approved.set(result[0], result[1]); });
  }
  return collections.map((collection) => ({
    ...collection,
    images: collection.images.map((asset) => ({ ...asset, alternativeUrls: approved.has(asset.remoteUrl) ? [approved.get(asset.remoteUrl)!] : asset.alternativeUrls })),
    ...(collection.coverAsset ? { coverAsset: { ...collection.coverAsset, alternativeUrls: approved.has(collection.coverAsset.remoteUrl) ? [approved.get(collection.coverAsset.remoteUrl)!] : collection.coverAsset.alternativeUrls } } : {})
  }));
}

function collectionIdForName(name: string): string {
  if (name === "JULIE KEGELS") return "collection-julie-kegels-ss27-2026";
  return `collection-pfw-ss27-${slugForCollection(name)}-2026`;
}

function assetIdForImage(collectionId: string, image: FhcmImageAsset, cover: boolean): string {
  const slug = collectionId.replace(/^collection-/, "");
  return `asset-fhcm-${slug}-${cover ? "cover" : `look-${String(image.sequenceNumber).padStart(3, "0")}`}`;
}

function assetIdForVideo(collectionId: string, video: FhcmVideoAsset): string {
  const slug = collectionId.replace(/^collection-/, "");
  return `asset-fhcm-${slug}-video-${video.providerAssetId.replace(/[^a-zA-Z0-9]+/g, "-").replace(/^-|-$/g, "")}`;
}

function imageSql(collectionId: string, image: FhcmImageAsset, cover = false): string {
  const id = assetIdForImage(collectionId, image, cover);
  const title = image.title;
  const metadata = JSON.stringify({ source: "FHCM", sourceCredit: image.creditLine ?? null, deduplication: "normalized FHCM media URL; identical files only", availabilityCheck: "HTML image URL observed on official collection page", role: cover ? "official_collection_cover" : "official_lookbook" });
  const columns = "id, collection_id, title, source_page_url, remote_url, provider, provider_asset_id, thumbnail_url, alternative_urls, creator, photographer, credit_line, source_ids, rights_status, download_policy, display_mode, canonical_status, asset_kind, coverage_type, coverage_scope, copyright_holder, attribution_required, embed_allowed, remote_render_allowed, rehost_allowed, verified_at, sequence_number, look_number, original_url, metadata_json, created_at, updated_at";
  const values = [id, collectionId, title, image.sourcePageUrl, image.remoteUrl, "FHCM", image.providerAssetId, image.thumbnailUrl, JSON.stringify(image.alternativeUrls), image.creator, null, image.creditLine ?? null, "source-fhcm", "UNKNOWN", "DOWNLOAD_BLOCKED", image.displayMode, "CANONICAL", "IMAGE", image.coverageType, image.coverageScope, image.creditLine ?? "FHCM", 1, 0, 1, 0, FHCM_RETRIEVED_AT, image.sequenceNumber, image.lookNumber ?? null, null, metadata, FHCM_RETRIEVED_AT, FHCM_RETRIEVED_AT];
  return `INSERT INTO assets (${columns}) SELECT ${values.map(sql).join(", ")} WHERE NOT EXISTS (SELECT 1 FROM assets WHERE collection_id = ${sql(collectionId)} AND remote_url = ${sql(image.remoteUrl)});\nINSERT OR IGNORE INTO asset_sources (asset_id, source_id, contribution, checked_at) SELECT id, 'source-fhcm', ${sql(cover ? "official FHCM collection cover image" : `ordered official FHCM lookbook image ${image.sequenceNumber}; ${image.creditLine ?? "no photographer credit published"}`)}, ${sql(FHCM_RETRIEVED_AT)} FROM assets WHERE id = ${sql(id)};`;
}

function videoSql(collectionId: string, video: FhcmVideoAsset): string {
  const id = assetIdForVideo(collectionId, video);
  const metadata = JSON.stringify({ source: "FHCM", classificationReason: "Official FHCM collection page exposes an embedded player; completeness is not established.", deduplication: "embed URL" });
  const columns = "id, collection_id, title, source_page_url, embed_url, provider, provider_asset_id, alternative_urls, creator, photographer, credit_line, source_ids, rights_status, download_policy, display_mode, canonical_status, asset_kind, coverage_type, coverage_scope, copyright_holder, attribution_required, embed_allowed, remote_render_allowed, rehost_allowed, verified_at, sequence_number, canonical_url, video_type, completeness, officiality, playback_mode, availability_status, metadata_json, created_at, updated_at";
  const values = [id, collectionId, video.title, video.sourcePageUrl, video.embedUrl, video.provider, video.providerAssetId, JSON.stringify(video.alternativeUrls), video.creator, null, video.creditLine ?? null, "source-fhcm", "UNKNOWN", "DOWNLOAD_BLOCKED", "EMBED", "CANONICAL", "VIDEO", video.coverageType, video.coverageScope, "FHCM", 1, 1, 0, 0, FHCM_RETRIEVED_AT, 200, video.canonicalUrl, video.videoType, video.completeness, video.officiality, video.playbackMode, "UNKNOWN", metadata, FHCM_RETRIEVED_AT, FHCM_RETRIEVED_AT];
  return `INSERT OR IGNORE INTO assets (${columns}) VALUES (${values.map(sql).join(", ")});\nINSERT OR IGNORE INTO asset_sources (asset_id, source_id, contribution, checked_at) SELECT id, 'source-fhcm', ${sql("official FHCM embedded player; coverage completeness not established")}, ${sql(FHCM_RETRIEVED_AT)} FROM assets WHERE id = ${sql(id)};`;
}

export function buildFhcmMigration(manifest: FhcmCollectionManifest): string {
  const lines = [
    "-- Generated from the official FHCM SS27 Collections index and collection pages.",
    "-- URL/provenance catalog only. Third-party images and video are not downloaded or rehosted.",
    `-- Retrieved at ${manifest.retrievedAt}; pages indexed ${manifest.summary.indexedCollectionPages}; gallery images ${manifest.summary.galleryImages}; cover images ${manifest.summary.officialCoverImages}.`,
  ];
  for (const collection of manifest.collections) {
    const collectionId = collectionIdForName(collection.name);
    for (const image of collection.images) lines.push(imageSql(collectionId, image));
    if (collection.coverAsset) lines.push(imageSql(collectionId, collection.coverAsset, true));
    for (const video of collection.videos) lines.push(videoSql(collectionId, video));
    const imageCount = collection.images.length + (collection.coverAsset ? 1 : 0);
    const videoCount = collection.videos.length;
    const reasonImage = collection.pageHasLookbook ? "Official FHCM lookbook and collection cover URLs were observed in DOM order." : "No FHCM lookbook was published on this page; the official collection cover URL is retained as thumbnail-only media.";
    const reasonVideo = videoCount > 0 ? "Official FHCM embedded player URL observed; video completeness is not established." : "Official FHCM collection page was checked and no playable embedded video URL was published.";
    lines.push(`UPDATE media_research_matrix SET state = 'FOUND', checked_at = ${sql(manifest.retrievedAt)}, source_page_url = ${sql(collection.collectionUrl)}, result_count = ${imageCount}, reason = ${sql(reasonImage)}, metadata_json = ${sql(JSON.stringify({ collectionPage: collection.collectionUrl, pageHasLookbook: collection.pageHasLookbook }))} WHERE collection_id = ${sql(collectionId)} AND source_key = 'FHCM' AND media_type = 'IMAGE';`);
    lines.push(`UPDATE media_research_matrix SET state = ${sql(videoCount > 0 ? "FOUND" : "EMPTY")}, checked_at = ${sql(manifest.retrievedAt)}, source_page_url = ${sql(collection.collectionUrl)}, result_count = ${videoCount}, reason = ${sql(reasonVideo)}, metadata_json = ${sql(JSON.stringify({ collectionPage: collection.collectionUrl, embeddedVideoCount: videoCount }))} WHERE collection_id = ${sql(collectionId)} AND source_key = 'FHCM' AND media_type = 'VIDEO';`);
    lines.push(`UPDATE media_research_jobs SET status = 'SUCCEEDED', last_attempt_at = ${sql(manifest.retrievedAt)}, result_count = ${imageCount}, metadata_json = ${sql(JSON.stringify({ sourcePageUrl: collection.collectionUrl, pageHasLookbook: collection.pageHasLookbook, officialCoverIncluded: Boolean(collection.coverAsset) }))}, updated_at = ${sql(manifest.retrievedAt)} WHERE collection_id = ${sql(collectionId)} AND source_id = 'source-fhcm' AND media_type = 'IMAGE';`);
    lines.push(`UPDATE media_research_jobs SET status = 'SUCCEEDED', last_attempt_at = ${sql(manifest.retrievedAt)}, result_count = ${videoCount}, metadata_json = ${sql(JSON.stringify({ sourcePageUrl: collection.collectionUrl, embeddedVideoCount: videoCount, completeness: "UNKNOWN" }))}, updated_at = ${sql(manifest.retrievedAt)} WHERE collection_id = ${sql(collectionId)} AND source_id = 'source-fhcm' AND media_type = 'VIDEO';`);
  }
  return `${lines.join("\n\n")}\n`;
}

async function fetchText(url: string): Promise<string> {
  const response = await fetch(url, { headers: { "user-agent": "NEX+ Fashion research collector/1.0" } });
  if (!response.ok) throw new Error(`FHCM request failed: ${response.status} ${url}`);
  return response.text();
}

export async function scrapeFhcmCollections(): Promise<FhcmCollectionManifest> {
  const indexHtml = await fetchText(FHCM_COLLECTIONS_URL);
  const calendarHtml = await fetchText(FHCM_CALENDAR_URL);
  const indexEntries = parseFhcmCollectionsIndex(indexHtml);
  const calendarEntries = parseFhcmCalendar(calendarHtml).filter((entry) => entry.localDate >= "2026-09-28" && entry.localDate <= "2026-09-30");
  const calendarByName = new Map(calendarEntries.map((entry) => [normalizedLabel(entry.name), entry]));
  const collectionsByUrl = new Map<string, FhcmCollectionRecord>();
  for (const entry of indexEntries) {
    const html = await fetchText(entry.collectionUrl);
    const calendarEntry = calendarByName.get(normalizedLabel(entry.name));
    collectionsByUrl.set(entry.collectionUrl, parseFhcmCollectionPage({
      ...entry,
      ...(calendarEntry ? { name: calendarEntry.name, localDate: calendarEntry.localDate, maisonUrl: calendarEntry.maisonUrl } : {})
    }, html));
  }
  const maisonPages = new Set<string>();
  const unlistedFromEditionIndex: FhcmCollectionManifest["unlistedFromEditionIndex"] = [];
  for (const calendarEntry of calendarEntries) {
    maisonPages.add(calendarEntry.maisonUrl);
    const maisonHtml = await fetchText(calendarEntry.maisonUrl);
    const collectionLink = selectFhcmSs27WomenswearCollection(parseFhcmMaisonCollections(maisonHtml, calendarEntry.maisonUrl));
    if (!collectionLink) {
      unlistedFromEditionIndex.push({ name: calendarEntry.name, localDate: calendarEntry.localDate, reason: "The official FHCM calendar Maison page did not expose an exact Womenswear Spring/Summer 2027 collection link at retrieval time; no URL was invented." });
      continue;
    }
    if (collectionsByUrl.has(collectionLink.collectionUrl)) {
      const existing = collectionsByUrl.get(collectionLink.collectionUrl)!;
      collectionsByUrl.set(collectionLink.collectionUrl, { ...existing, maisonUrl: calendarEntry.maisonUrl, localDate: calendarEntry.localDate });
      continue;
    }
    const collectionHtml = await fetchText(collectionLink.collectionUrl);
    collectionsByUrl.set(collectionLink.collectionUrl, parseFhcmCollectionPage({
      name: calendarEntry.name,
      localDate: calendarEntry.localDate,
      collectionUrl: collectionLink.collectionUrl,
      ...(collectionLink.coverUrl ? { coverUrl: collectionLink.coverUrl } : {}),
      maisonUrl: calendarEntry.maisonUrl,
      discoveryMethod: "CALENDAR_MAISON"
    }, collectionHtml));
  }
  const collections = [...collectionsByUrl.values()];
  const galleryImages = collections.reduce((count, collection) => count + collection.images.length, 0);
  const officialCoverImages = collections.filter((collection) => Boolean(collection.coverAsset)).length;
  const embeddedVideos = collections.reduce((count, collection) => count + collection.videos.length, 0);
  return {
    schemaVersion: "pfw-ss27-fhcm-collection-manifest-v1",
    source: "FHCM",
    calendarUrl: FHCM_CALENDAR_URL,
    indexUrl: FHCM_COLLECTIONS_URL,
    retrievedAt: FHCM_RETRIEVED_AT,
    scope: { from: "2026-09-28", to: "2026-09-30", edition: "Womenswear Spring/Summer 2027" },
    collections,
    unlistedFromEditionIndex,
    summary: {
      calendarEntries: calendarEntries.length,
      resolvedMaisonPages: maisonPages.size,
      resolvedCollectionPages: collections.length,
      indexedCollectionPages: indexEntries.length,
      collectionsWithLookbooks: collections.filter((collection) => collection.pageHasLookbook).length,
      collectionsWithOnlyOfficialCover: collections.filter((collection) => !collection.pageHasLookbook && Boolean(collection.coverAsset)).length,
      galleryImages,
      officialCoverImages,
      embeddedVideos,
      totalImages: galleryImages + officialCoverImages
    }
  };
}

async function main(): Promise<void> {
  const manifest = await scrapeFhcmCollections();
  const outIndex = process.argv.indexOf("--out");
  const migrationIndex = process.argv.indexOf("--migration");
  if (outIndex >= 0 && process.argv[outIndex + 1]) {
    const output = process.argv[outIndex + 1]!;
    await mkdir(dirname(output), { recursive: true });
    await writeFile(output, `${JSON.stringify(manifest, null, 2)}\n`, "utf8");
  }
  if (migrationIndex >= 0 && process.argv[migrationIndex + 1]) {
    const output = process.argv[migrationIndex + 1]!;
    await mkdir(dirname(output), { recursive: true });
    await writeFile(output, buildFhcmMigration(manifest), "utf8");
  }
  console.log(JSON.stringify(manifest.summary));
  console.log(JSON.stringify({ unlistedFromEditionIndex: manifest.unlistedFromEditionIndex }, null, 2));
}

if (process.argv[1]?.replace(/[\\/]+$/, "").endsWith("scrape-fhcm-collections.ts")) await main();
