import { mkdir, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

export const VOGUE_RETRIEVED_AT = "2026-09-30T00:00:00.000Z";

export interface VogueFallbackSpec {
  name: string;
  localDate: string;
  slug: string;
  filenamePrefix: string;
  creditSource: "GORUNWAY" | "BRAND";
}

export interface VogueFallbackImage {
  kind: "IMAGE";
  title: string;
  sourcePageUrl: string;
  remoteUrl: string;
  alternativeUrls: string[];
  thumbnailUrl: string;
  provider: "Vogue Runway";
  providerAssetId: string;
  creator: "Vogue Runway";
  creditLine: string;
  sequenceNumber: number;
  lookNumber: number;
  coverageType: "RUNWAY" | "DETAILS";
  coverageScope: "LOOKBOOK_IMAGE" | "DETAIL";
  displayMode: "INLINE";
  downloadPolicy: "DOWNLOAD_BLOCKED";
  rightsStatus: "UNKNOWN";
  catalogOrder: number;
}

export interface VogueFallbackRecord {
  name: string;
  localDate: string;
  collectionId: string;
  sourcePageUrl: string;
  images: VogueFallbackImage[];
}

function collectionIdForName(name: string): string {
  const slug = name.normalize("NFKD").replace(/[\u0300-\u036f]/g, "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
  return `collection-pfw-ss27-${slug}-2026`;
}

export const VOGUE_FALLBACK_SPECS: VogueFallbackSpec[] = [
  { name: "HODAKOVA", localDate: "2026-09-28", slug: "hodakova", filenamePrefix: "hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg", creditSource: "GORUNWAY" },
  { name: "MATIÈRES FÉCALES", localDate: "2026-09-28", slug: "matieres-fecales", filenamePrefix: "matieres-fecales-spring-2027-ready-to-wear-credit-gorunway.jpg", creditSource: "GORUNWAY" },
  { name: "ESTER MANAS", localDate: "2026-09-29", slug: "ester-manas", filenamePrefix: "ester-manas-spring-2027-ready-to-wear-credit-gorunway.jpg", creditSource: "GORUNWAY" },
  { name: "MARIE ADAM-LEENAERDT", localDate: "2026-09-29", slug: "marie-adam-leenaerdt", filenamePrefix: "marie-adam-leenaerdt-spring-2027-ready-to-wear-credit-gorunway.jpg", creditSource: "GORUNWAY" },
  { name: "VAQUERA", localDate: "2026-09-30", slug: "vaquera", filenamePrefix: "vaquera-spring-2027-ready-to-wear-credit-gorunway.jpg", creditSource: "GORUNWAY" },
  { name: "TOM FORD", localDate: "2026-09-30", slug: "tom-ford", filenamePrefix: "tom-ford-spring-2027-ready-to-wear-credit-brand.jpg", creditSource: "BRAND" }
];

function sourcePageUrl(spec: VogueFallbackSpec): string {
  return `https://www.vogue.com/fashion-shows/spring-2027-ready-to-wear/${spec.slug}`;
}

export function parseVogueRunwayFallbackImages(spec: VogueFallbackSpec, html: string): VogueFallbackImage[] {
  const pattern = /https:\/\/assets\.vogue\.com\/photos\/([^/]+)\/master\/w_1600,c_limit\/([^"\\\s]+\.jpg)/g;
  const byProviderAsset = new Map<string, VogueFallbackImage>();
  let catalogOrder = 0;
  for (const match of html.matchAll(pattern)) {
    const photoId = match[1];
    const filename = match[2];
    const filenameStem = spec.filenamePrefix.replace(/\.jpg$/, "").replace(/-credit-[^-]+$/, "");
    if (!photoId || !filename || !filename.includes(filenameStem)) continue;
    const sequenceMatch = filename.match(/^(\d+)-/);
    const sequenceNumber = Number(sequenceMatch?.[1]);
    if (!Number.isSafeInteger(sequenceNumber)) continue;
    const remoteUrl = `https://assets.vogue.com/photos/${photoId}/master/w_1600,c_limit/${filename}`;
    const providerAssetId = `${photoId}/${filename}`;
    if (byProviderAsset.has(providerAssetId)) continue;
    const creditLine = spec.creditSource === "GORUNWAY" ? "Gorunway.com credit as published by Vogue Runway" : "Tom Ford brand credit as published by Vogue Runway";
    const isDetail = /-details?-credit-/i.test(filename);
    byProviderAsset.set(providerAssetId, {
      kind: "IMAGE",
      title: `${spec.name} SS27 — Vogue Runway ${isDetail ? "detail" : "look"} ${String(sequenceNumber).padStart(2, "0")}`,
      sourcePageUrl: sourcePageUrl(spec),
      remoteUrl,
      alternativeUrls: [`https://assets.vogue.com/photos/${photoId}/master/w_960,c_limit/${filename}`],
      thumbnailUrl: `https://assets.vogue.com/photos/${photoId}/master/w_360%2Cc_limit/${filename}`,
      provider: "Vogue Runway",
      providerAssetId,
      creator: "Vogue Runway",
      creditLine,
      sequenceNumber,
      lookNumber: sequenceNumber,
      coverageType: isDetail ? "DETAILS" : "RUNWAY",
      coverageScope: isDetail ? "DETAIL" : "LOOKBOOK_IMAGE",
      displayMode: "INLINE",
      downloadPolicy: "DOWNLOAD_BLOCKED",
      rightsStatus: "UNKNOWN",
      catalogOrder: ++catalogOrder
    });
  }
  return [...byProviderAsset.values()].sort((left, right) => left.catalogOrder - right.catalogOrder);
}

function sql(value: string | number | boolean | null | undefined): string {
  if (value === null || value === undefined) return "NULL";
  if (typeof value === "number") return String(value);
  if (typeof value === "boolean") return value ? "1" : "0";
  return `'${value.replaceAll("'", "''")}'`;
}

function assetId(record: VogueFallbackRecord, image: VogueFallbackImage): string {
  return `asset-vogue-${record.collectionId.replace(/^collection-/, "")}-${image.coverageScope === "DETAIL" ? "detail" : "look"}-${String(image.sequenceNumber).padStart(3, "0")}`;
}

function imageSql(record: VogueFallbackRecord, image: VogueFallbackImage): string {
  const id = assetId(record, image);
  const metadata = JSON.stringify({ source: "Vogue Runway", fallbackReason: "FHCM Maison page did not expose an exact SS27 collection page at retrieval time", rightsStatus: image.rightsStatus, deduplication: "providerAssetId; identical files only" });
  const columns = "id, collection_id, title, source_page_url, remote_url, provider, provider_asset_id, thumbnail_url, alternative_urls, creator, photographer, credit_line, source_ids, rights_status, download_policy, display_mode, canonical_status, asset_kind, coverage_type, coverage_scope, copyright_holder, attribution_required, embed_allowed, remote_render_allowed, rehost_allowed, verified_at, sequence_number, look_number, original_url, metadata_json, created_at, updated_at";
  const values = [id, record.collectionId, image.title, image.sourcePageUrl, image.remoteUrl, image.provider, image.providerAssetId, image.thumbnailUrl, JSON.stringify(image.alternativeUrls), image.creator, null, image.creditLine, "source-vogue-runway", image.rightsStatus, image.downloadPolicy, image.displayMode, "CANONICAL", "IMAGE", image.coverageType, image.coverageScope, "Vogue Runway / Gorunway.com or brand credit", 1, 0, 1, 0, VOGUE_RETRIEVED_AT, image.catalogOrder, image.lookNumber, null, metadata, VOGUE_RETRIEVED_AT, VOGUE_RETRIEVED_AT];
  return `INSERT INTO assets (${columns}) SELECT ${values.map(sql).join(", ")} WHERE NOT EXISTS (SELECT 1 FROM assets WHERE collection_id = ${sql(record.collectionId)} AND remote_url = ${sql(image.remoteUrl)});\nINSERT OR IGNORE INTO asset_sources (asset_id, source_id, contribution, checked_at) SELECT id, 'source-vogue-runway', ${sql(`ordered Vogue Runway fallback image ${image.sequenceNumber}; ${image.creditLine}`)}, ${sql(VOGUE_RETRIEVED_AT)} FROM assets WHERE id = ${sql(id)};`;
}

export function buildVogueFallbackMigration(records: VogueFallbackRecord[]): string {
  const lines = [
    "-- Generated from verified Vogue Runway SS27 fallback collection pages.",
    "-- Fallback catalog only. Images remain remote third-party media with DOWNLOAD_BLOCKED.",
    `-- Retrieved at ${VOGUE_RETRIEVED_AT}; records ${records.length}; images ${records.reduce((sum, record) => sum + record.images.length, 0)}.`
  ];
  for (const record of records) {
    lines.push(`UPDATE collections SET source_ids = CASE WHEN instr(',' || source_ids || ',', ',source-vogue-runway,') > 0 THEN source_ids ELSE source_ids || ',source-vogue-runway' END, updated_at = ${sql(VOGUE_RETRIEVED_AT)} WHERE id = ${sql(record.collectionId)};`);
    for (const image of record.images) lines.push(imageSql(record, image));
    lines.push(`UPDATE media_research_matrix SET state = 'FOUND', checked_at = ${sql(VOGUE_RETRIEVED_AT)}, source_page_url = ${sql(record.sourcePageUrl)}, result_count = ${record.images.length}, reason = ${sql("Verified Vogue Runway fallback lookbook; FHCM Maison page had no exact SS27 womenswear collection link at retrieval time.")}, metadata_json = ${sql(JSON.stringify({ sourcePageUrl: record.sourcePageUrl, fallback: true, imageCount: record.images.length }))} WHERE collection_id = ${sql(record.collectionId)} AND source_key = 'VOGUE' AND media_type = 'IMAGE';`);
    lines.push(`UPDATE media_research_jobs SET status = 'SUCCEEDED', last_attempt_at = ${sql(VOGUE_RETRIEVED_AT)}, result_count = ${record.images.length}, metadata_json = ${sql(JSON.stringify({ sourcePageUrl: record.sourcePageUrl, fallback: true }))}, updated_at = ${sql(VOGUE_RETRIEVED_AT)} WHERE collection_id = ${sql(record.collectionId)} AND source_id = 'source-vogue-runway' AND media_type = 'IMAGE';`);
  }
  return `${lines.join("\n\n")}\n`;
}

async function fetchText(url: string): Promise<string> {
  const response = await fetch(url, { headers: { "user-agent": "NEX+ Fashion research collector/1.0" } });
  if (!response.ok) throw new Error(`Vogue request failed: ${response.status} ${url}`);
  return response.text();
}

export async function scrapeVogueFallbacks(): Promise<VogueFallbackRecord[]> {
  const records: VogueFallbackRecord[] = [];
  for (const spec of VOGUE_FALLBACK_SPECS) {
    const sourcePageUrl = sourcePageUrlForSpec(spec);
    const html = await fetchText(sourcePageUrl);
    const images = parseVogueRunwayFallbackImages(spec, html);
    if (images.length === 0) throw new Error(`No verified Vogue Runway images found for ${spec.name}`);
    records.push({ name: spec.name, localDate: spec.localDate, collectionId: collectionIdForName(spec.name), sourcePageUrl, images });
  }
  return records;
}

function sourcePageUrlForSpec(spec: VogueFallbackSpec): string {
  return sourcePageUrl(spec);
}

async function main(): Promise<void> {
  const records = await scrapeVogueFallbacks();
  const outIndex = process.argv.indexOf("--out");
  const migrationIndex = process.argv.indexOf("--migration");
  if (outIndex >= 0 && process.argv[outIndex + 1]) {
    const output = process.argv[outIndex + 1]!;
    await mkdir(dirname(output), { recursive: true });
    await writeFile(output, `${JSON.stringify({ schemaVersion: "pfw-ss27-vogue-fallback-manifest-v1", source: "Vogue Runway", retrievedAt: VOGUE_RETRIEVED_AT, records, summary: { records: records.length, images: records.reduce((sum, record) => sum + record.images.length, 0) } }, null, 2)}\n`, "utf8");
  }
  if (migrationIndex >= 0 && process.argv[migrationIndex + 1]) {
    const output = process.argv[migrationIndex + 1]!;
    await mkdir(dirname(output), { recursive: true });
    await writeFile(output, buildVogueFallbackMigration(records), "utf8");
  }
  console.log(JSON.stringify({ records: records.length, images: records.reduce((sum, record) => sum + record.images.length, 0), byCollection: records.map((record) => ({ name: record.name, images: record.images.length })) }, null, 2));
}

if (process.argv[1]?.replace(/[\\/]+$/, "").endsWith("scrape-vogue-pfw-fallbacks.ts")) await main();
