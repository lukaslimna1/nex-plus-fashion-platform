import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { DIOR_VOGUE_SLIDESHOW_URL, DIOR_VOGUE_VIDEO_URL, FHCM_PFW_CALENDAR_URL, PFW_SS27_28_30_SCHEDULE, extractVogueRunwayImages, toParisIso } from "./pfw-media.js";

const capturedAt = "2026-09-30T00:00:00.000Z";
const diorFilename = "christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg";
const knownCollections = new Map<string, { maisonId: string; collectionId: string; scheduleId: string }>([
  ["Julie Kegels", { maisonId: "maison-julie-kegels", collectionId: "collection-julie-kegels-ss27-2026", scheduleId: "schedule-julie-kegels-ss27-2026-09-28" }],
  ["MAXHOSA AFRICA", { maisonId: "maison-maxhosa-africa", collectionId: "collection-maxhosa-africa-ss27-2026", scheduleId: "schedule-maxhosa-africa-ss27-2026-09-28" }]
]);

function slugify(value: string): string {
  return value.normalize("NFD").replace(/[\u0300-\u036f]/g, "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
}
function idsFor(title: string, localDate = "2026-09-29") {
  const known = knownCollections.get(title);
  if (known) return known;
  const slug = slugify(title);
  return { maisonId: `maison-pfw-ss27-${slug}`, collectionId: `collection-pfw-ss27-${slug}-2026`, scheduleId: `schedule-pfw-ss27-${slug}-${localDate}` };
}
function sql(value: unknown): string {
  if (value === null || value === undefined) return "NULL";
  if (typeof value === "number") return String(value);
  if (typeof value === "boolean") return value ? "1" : "0";
  return `'${String(value).replaceAll("'", "''")}'`;
}
function insert(table: string, columns: string[], values: unknown[]): string {
  return `INSERT OR IGNORE INTO ${table} (${columns.join(",")}) VALUES (${values.map(sql).join(",")});`;
}

function seedSql(images: ReturnType<typeof extractVogueRunwayImages>): string {
  const lines: string[] = [
    "-- Generated from the official FHCM calendar snapshot and the public Vogue Runway slideshow HTML.",
    "-- This migration stores URLs and provenance only; it does not download or rehost third-party media.",
    insert("sources", ["id", "canonical_name", "type", "base_url", "authority_tier", "language", "coverage_scope", "country_id", "access_mode", "rights_notes", "automation_notes", "last_verified_at", "active", "created_at", "updated_at"], ["source-vogue-video", "Vogue video", "VIDEO_CHANNEL", "https://www.vogue.com/video", "B", "en", "runway,video,editorial", "country-france", "PUBLIC", "Editorial video remains hosted by Vogue; no download or rehost permission is inferred.", "Preserve canonical page URL and classify completeness from the source evidence.", capturedAt, 1, capturedAt, capturedAt]),
    insert("source_checks", ["source_id", "last_checked_at", "last_changed_at"], ["source-vogue-video", capturedAt, capturedAt])
  ];

  for (const entry of PFW_SS27_28_30_SCHEDULE) {
    const ids = idsFor(entry.title, entry.localDate);
    const slug = slugify(entry.title);
    if (!knownCollections.has(entry.title)) {
      lines.push(insert("maisons", ["id", "name", "slug", "official_source_ids", "created_at", "updated_at"], [ids.maisonId, entry.title, slug, "source-fhcm", capturedAt, capturedAt]));
      const sourceIds = entry.title === "Christian Dior" ? "source-fhcm,source-vogue-runway,source-vogue-video" : "source-fhcm";
      lines.push(insert("collections", ["id", "maison_id", "edition_id", "name", "slug", "calendar_year", "season_year", "season_code", "season_label", "presented_on", "canonical_status", "source_ids", "presentation_format", "created_at", "updated_at"], [ids.collectionId, ids.maisonId, "edition-pfw-womenswear-ss27-2026", `${entry.title} — Womenswear Spring/Summer 2027`, `${slug}-womenswear-spring-summer-2027`, 2026, 2027, "SS27", "Spring/Summer 2027", entry.localDate, "CANONICAL", sourceIds, entry.format, capturedAt, capturedAt]));
    }
    if (!knownCollections.has(entry.title)) {
      lines.push(insert("schedule_entries", ["id", "edition_id", "event_id", "segment_id", "city_hub_id", "title", "format", "start_time", "timezone", "verification_status", "official_url", "source_ids", "created_at", "updated_at"], [ids.scheduleId, "edition-pfw-womenswear-ss27-2026", "event-paris-fashion-week", "segment-womenswear", "city-paris", entry.title, entry.format, toParisIso(entry.localDate, entry.localTime), "Europe/Paris", "VERIFIED", FHCM_PFW_CALENDAR_URL, "source-fhcm", capturedAt, capturedAt]));
    }
    for (const mediaType of ["IMAGE", "VIDEO"] as const) {
      lines.push(insert("media_research_jobs", [`id`, `collection_id`, `source_id`, `media_type`, `status`, `result_count`, `metadata_json`, `created_at`, `updated_at`], [`research-pfw-ss27-${slug}-${mediaType.toLowerCase()}-fhcm`, ids.collectionId, "source-fhcm", mediaType, "PENDING", 0, JSON.stringify({ scope: "PFW_SS27_FIRST_THREE_DAYS", sourcePageUrl: FHCM_PFW_CALENDAR_URL }), capturedAt, capturedAt]));
    }
  }

  const dior = idsFor("Christian Dior");
  lines.push(insert("media_research_jobs", ["id", "collection_id", "source_id", "media_type", "status", "last_attempt_at", "result_count", "metadata_json", "created_at", "updated_at"], ["research-christian-dior-ss27-vogue-images", dior.collectionId, "source-vogue-runway", "IMAGE", "SUCCEEDED", capturedAt, images.length, JSON.stringify({ sourcePageUrl: DIOR_VOGUE_SLIDESHOW_URL, deduplication: "normalized provider URL and sequence" }), capturedAt, capturedAt]));
  lines.push(insert("media_research_jobs", ["id", "collection_id", "source_id", "media_type", "status", "last_attempt_at", "result_count", "metadata_json", "created_at", "updated_at"], ["research-christian-dior-ss27-vogue-video", dior.collectionId, "source-vogue-video", "VIDEO", "SUCCEEDED", capturedAt, 1, JSON.stringify({ sourcePageUrl: DIOR_VOGUE_VIDEO_URL, classification: "RUNWAY_COVERAGE; completeness not established" }), capturedAt, capturedAt]));

  for (const image of images) {
    lines.push(insert("assets", ["id", "collection_id", "title", "source_page_url", "remote_url", "provider", "provider_asset_id", "thumbnail_url", "alternative_urls", "creator", "photographer", "credit_line", "source_ids", "rights_status", "download_policy", "display_mode", "canonical_status", "asset_kind", "coverage_type", "coverage_scope", "copyright_holder", "attribution_required", "embed_allowed", "remote_render_allowed", "rehost_allowed", "verified_at", "sequence_number", "look_number", "created_at", "updated_at"], [`asset-christian-dior-ss27-look-${String(image.sequenceNumber).padStart(2, "0")}`, dior.collectionId, `Christian Dior SS27 — look ${String(image.sequenceNumber).padStart(2, "0")}`, DIOR_VOGUE_SLIDESHOW_URL, image.remoteUrl, image.provider, image.providerAssetId, image.thumbnailUrl, image.alternativeUrls.join(","), image.creator, image.photographer, image.creditLine, "source-vogue-runway", "UNKNOWN", "DOWNLOAD_BLOCKED", "INLINE", "CANONICAL", "IMAGE", "RUNWAY", "LOOKBOOK_IMAGE", "Vogue Runway / Gorunway.com", 1, 0, 1, 0, capturedAt, image.sequenceNumber, image.sequenceNumber, capturedAt, capturedAt]));
    lines.push(insert("asset_sources", ["asset_id", "source_id", "contribution", "checked_at"], [`asset-christian-dior-ss27-look-${String(image.sequenceNumber).padStart(2, "0")}`, "source-vogue-runway", "ordered runway image, provider asset ID and Filippo Fior / Gorunway.com credit", capturedAt]));
  }
  lines.push(insert("assets", ["id", "collection_id", "title", "source_page_url", "provider", "provider_asset_id", "alternative_urls", "creator", "credit_line", "source_ids", "rights_status", "download_policy", "display_mode", "canonical_status", "asset_kind", "coverage_type", "coverage_scope", "copyright_holder", "attribution_required", "embed_allowed", "remote_render_allowed", "rehost_allowed", "verified_at", "canonical_url", "channel_name", "video_type", "completeness", "officiality", "orientation", "playback_mode", "availability_status", "metadata_json", "created_at", "updated_at"], ["asset-christian-dior-ss27-vogue-video", dior.collectionId, "Christian Dior Spring 2027 Ready-to-Wear — Vogue runway coverage", DIOR_VOGUE_VIDEO_URL, "website", "christian-dior-spring-2027-ready-to-wear", "", "Vogue", "Vogue editorial video page; complete-show coverage is not established.", "source-vogue-video", "UNKNOWN", "DOWNLOAD_BLOCKED", "LINK_ONLY", "CANONICAL", "VIDEO", "EDITORIAL", "EDITORIAL", "Vogue", 1, 0, 0, 0, capturedAt, DIOR_VOGUE_VIDEO_URL, "Vogue", "RUNWAY_COVERAGE", "PARTIAL", "PROFESSIONAL_VERIFIED", "UNKNOWN", "EXTERNAL_LINK", "AVAILABLE", JSON.stringify({ classificationReason: "The public page provides runway coverage and transcript, but does not establish a complete show recording." }), capturedAt, capturedAt]));
  lines.push(insert("asset_sources", ["asset_id", "source_id", "contribution", "checked_at"], ["asset-christian-dior-ss27-vogue-video", "source-vogue-video", "Vogue editorial video page and published date; not classified as FULL_SHOW", capturedAt]));
  lines.push("CREATE INDEX IF NOT EXISTS idx_pfw_ss27_schedule_date ON schedule_entries(start_time, event_id);");
  return `${lines.join("\n")}\n`;
}

function manifest(images: ReturnType<typeof extractVogueRunwayImages>) {
  const dior = idsFor("Christian Dior");
  const assets = images.map((image) => ({ id: `asset-christian-dior-ss27-look-${String(image.sequenceNumber).padStart(2, "0")}`, lookNumber: image.sequenceNumber, title: `Christian Dior SS27 — look ${String(image.sequenceNumber).padStart(2, "0")}`, assetKind: "IMAGE", coverageType: "RUNWAY", coverageScope: "LOOKBOOK_IMAGE", sourcePageUrl: DIOR_VOGUE_SLIDESHOW_URL, ...image, rightsStatus: "UNKNOWN", downloadPolicy: "DOWNLOAD_BLOCKED", displayMode: "INLINE", remoteRenderAllowed: true, rehostAllowed: false, canonicalStatus: "CANONICAL" }));
  return {
    schemaVersion: "1.1.0",
    collection: { id: dior.collectionId, slug: "christian-dior-womenswear-spring-summer-2027", name: "Christian Dior — Womenswear Spring/Summer 2027", maison: "Christian Dior", calendarYear: 2026, seasonYear: 2027, seasonCode: "SS27", seasonLabel: "Spring/Summer 2027", presentedOn: "2026-09-29", presentationFormat: "SHOW" },
    schedule: { title: "Christian Dior", localDate: "2026-09-29", localTime: "14:30", startTime: toParisIso("2026-09-29", "14:30"), timezone: "Europe/Paris", format: "SHOW", officialUrl: FHCM_PFW_CALENDAR_URL },
    sources: [
      { id: "source-fhcm", name: "Fédération de la Haute Couture et de la Mode (FHCM)", url: FHCM_PFW_CALENDAR_URL, authorityTier: "A", role: "official calendar" },
      { id: "source-vogue-runway", name: "Vogue Runway", url: DIOR_VOGUE_SLIDESHOW_URL, authorityTier: "B", role: "66-look slideshow and Filippo Fior / Gorunway.com credits" },
      { id: "source-vogue-video", name: "Vogue video", url: DIOR_VOGUE_VIDEO_URL, authorityTier: "B", role: "editorial runway coverage video" }
    ],
    imageGroups: [{ sourceId: "source-vogue-runway", coverageType: "RUNWAY", catalogedImageCount: assets.length, assets }],
    videos: [{ id: "asset-christian-dior-ss27-vogue-video", title: "Christian Dior Spring 2027 Ready-to-Wear — Vogue runway coverage", assetKind: "VIDEO", sourcePageUrl: DIOR_VOGUE_VIDEO_URL, provider: "website", providerAssetId: "christian-dior-spring-2027-ready-to-wear", canonicalUrl: DIOR_VOGUE_VIDEO_URL, channelName: "Vogue", videoType: "RUNWAY_COVERAGE", completeness: "PARTIAL", officiality: "PROFESSIONAL_VERIFIED", playbackMode: "EXTERNAL_LINK", orientation: "UNKNOWN", availabilityStatus: "AVAILABLE", downloadPolicy: "DOWNLOAD_BLOCKED", displayMode: "LINK_ONLY", sourceIds: ["source-vogue-video"], sourceId: "source-vogue-video", rightsStatus: "UNKNOWN", canonicalStatus: "CANONICAL" }],
    mediaStatus: { status: "PARTIAL", runwayImages: assets.length, backstageImages: 0, detailImages: 0, fullShowVideo: false, otherVideos: 1, officialSource: true, editorialSources: 2 },
    provenance: { catalogedImageCount: assets.length, remoteImageCount: assets.length, linkOnlyImageCount: 0, deduplication: { method: "normalized provider asset URL and sequence; alternate renditions remain on the same asset", inputCount: assets.length, distinctRemoteFiles: assets.length, duplicateFiles: [] } },
    videoEvidence: "Vogue publishes a runway coverage page with transcript. It is retained as RUNWAY_COVERAGE/PARTIAL and not FULL_SHOW because completeness is not established by the source."
  };
}

function firstThreeDaysScan(images: ReturnType<typeof extractVogueRunwayImages>) {
  const researched = new Map<string, { status: string; runwayImages: number; remoteImages: number; videos: number; fullShowVideo: boolean; sources: string[] }>([
    ["Julie Kegels", { status: "PARTIAL", runwayImages: 43, remoteImages: 43, videos: 1, fullShowVideo: false, sources: ["source-fhcm", "source-vogue-runway", "source-nss"] }],
    ["MAXHOSA AFRICA", { status: "PARTIAL", runwayImages: 51, remoteImages: 50, videos: 1, fullShowVideo: false, sources: ["source-fhcm", "source-vogue-runway", "source-maxhosa-youtube", "source-reuters-connect"] }],
    ["Christian Dior", { status: "PARTIAL", runwayImages: images.length, remoteImages: images.length, videos: 1, fullShowVideo: false, sources: ["source-fhcm", "source-vogue-runway", "source-vogue-video"] }]
  ]);
  const entries = PFW_SS27_28_30_SCHEDULE.map((entry) => {
    const media = researched.get(entry.title);
    return { ...entry, startTime: toParisIso(entry.localDate, entry.localTime), ...(media ? { media } : { media: { status: "NEEDS_RESEARCH", runwayImages: 0, remoteImages: 0, videos: 0, fullShowVideo: false, sources: ["source-fhcm"] } }) };
  });
  return {
    schemaVersion: "pfw-ss27-first-three-days-v1",
    capturedAt,
    window: { start: "2026-09-28", end: "2026-09-30", timezone: "Europe/Paris" },
    authority: { source: "FHCM", calendarUrl: FHCM_PFW_CALENDAR_URL, entries: entries.length },
    entries,
    summary: {
      totalProcessed: entries.length,
      byDate: Object.fromEntries(["2026-09-28", "2026-09-29", "2026-09-30"].map((date) => [date, entries.filter((entry) => entry.localDate === date).length])),
      runwayImageCollections: entries.filter((entry) => entry.media.runwayImages > 0).map((entry) => entry.title),
      backstageCollections: [],
      detailCollections: [],
      fullShowVideoCollections: entries.filter((entry) => entry.media.fullShowVideo).map((entry) => entry.title),
      youtubeVideos: entries.filter((entry) => entry.media.sources.some((source) => source.includes("youtube"))).map((entry) => entry.title),
      otherPlatformVideos: entries.filter((entry) => entry.media.videos > 0 && !entry.media.sources.some((source) => source.includes("youtube"))).map((entry) => entry.title),
      providers: ["Vogue Runway", "Instagram", "YouTube", "website"],
      orientations: ["UNKNOWN"],
      withoutVideo: entries.filter((entry) => entry.media.videos === 0).map((entry) => entry.title),
      withoutGallery: entries.filter((entry) => entry.media.runwayImages === 0).map((entry) => entry.title),
      errors: [],
      manualResearchRequired: entries.filter((entry) => entry.media.status === "NEEDS_RESEARCH").map((entry) => entry.title)
    },
    sourceRegistry: ["source-fhcm", "source-vogue-runway", "source-vogue-video", "source-nss", "source-maxhosa-youtube", "source-reuters-connect"]
  };
}

const html = await (await fetch(DIOR_VOGUE_SLIDESHOW_URL, { headers: { "user-agent": "NEX+ Fashion research connector" } })).text();
const images = extractVogueRunwayImages(html, diorFilename);
if (images.length === 0) throw new Error("No Vogue Runway images were found in the public slideshow HTML");
const root = resolve(process.cwd());
const migrationPath = resolve(root, "migrations/0010_pfw_ss27_first_three_days.sql");
const manifestPath = resolve(root, "docs/vertical-slice/christian-dior-ss27.json");
const scanPath = resolve(root, "docs/research/pfw-ss27-first-three-days.json");
mkdirSync(dirname(migrationPath), { recursive: true });
mkdirSync(dirname(manifestPath), { recursive: true });
mkdirSync(dirname(scanPath), { recursive: true });
writeFileSync(migrationPath, seedSql(images), "utf8");
writeFileSync(manifestPath, `${JSON.stringify(manifest(images), null, 2)}\n`, "utf8");
writeFileSync(scanPath, `${JSON.stringify(firstThreeDaysScan(images), null, 2)}\n`, "utf8");
console.log(JSON.stringify({ migrationPath, manifestPath, scanPath, scheduleEntries: PFW_SS27_28_30_SCHEDULE.length, diorImages: images.length }, null, 2));
