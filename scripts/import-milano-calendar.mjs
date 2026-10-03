import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { runInNewContext } from "node:vm";

const calendarUrl = "https://milanofashionweek.cameramoda.it/en/calendar";
const officialBaseUrl = "https://milanofashionweek.cameramoda.it/";
const officialBrandUrls = {
  PRADA: "https://milanofashionweek.cameramoda.it/en/brand/13550",
};

const slugify = (value) => value
  .normalize("NFD")
  .replace(/[\u0300-\u036f]/g, "")
  .toLowerCase()
  .replace(/[^a-z0-9]+/g, "-")
  .replace(/^-|-$/g, "");

const timeFromMinutes = (minutes) => {
  if (minutes === null || minutes === undefined) return null;
  return `${String(Math.floor(minutes / 60)).padStart(2, "0")}:${String(minutes % 60).padStart(2, "0")}`;
};

const formatMap = {
  EVENTO: "event",
  PRESENTAZIONE: "presentation",
  APPUNTAMENTO: "presentation_by_appointment",
  SFILATA: "fashion_show",
};

const locationStatus = (address) => {
  if (!address || address === "DIGITAL") return "not_published";
  if (address.toLowerCase().startsWith("location")) return "invitation";
  return "published";
};

const page = await fetch(calendarUrl).then((response) => {
  if (!response.ok) throw new Error(`CNMI calendar returned HTTP ${response.status}`);
  return response.text();
});
const payloadStart = page.indexOf("<script>window.__NUXT__=");
const payloadEnd = page.indexOf("</script>", payloadStart);
if (payloadStart < 0 || payloadEnd < 0) throw new Error("CNMI Nuxt payload was not found");

const sandbox = { window: {} };
runInNewContext(page.slice(payloadStart + "<script>".length, payloadEnd), sandbox);
const entries = Object.values(sandbox.window.__NUXT__.data)
  .find((value) => Array.isArray(value) && value.length > 0 && value.every((item) => item?.eventName));
if (!entries) throw new Error("CNMI calendar entries were not found");

const retrievedAt = new Date().toISOString();
const sourceId = "source:cnmi";
const cityId = "geo:city:milano";
const editionId = "edition:milano-fashion-week:ss27:2026";
const normalizedEntries = entries.map((entry) => {
  const address = entry.indirizzo ?? null;
  const venueId = address && !["DIGITAL", "Location In The Invitation"].includes(address)
    ? `venue:milano:${slugify(address)}`
    : null;
  const format = formatMap[entry.tipo];
  if (!format) throw new Error(`Unsupported CNMI format ${entry.tipo}`);
  const sourceExternalId = String(entry.id);
  const participantId = `participant:milano:${slugify(entry.eventName)}`;
  const normalized = {
    id: `schedule:cnmi:${sourceExternalId}`,
    sourceExternalId,
    participantId,
    participantNameRaw: entry.eventName,
    maisonId: entry.eventName === "PRADA" ? "maison:prada" : null,
    localDate: entry.dataInizio,
    startTimeLocal: timeFromMinutes(entry.oraInizio),
    endTimeLocal: null,
    timeZone: "Europe/Rome",
    format,
    scheduleStatus: "completed",
    locationStatus: locationStatus(address),
    deliveryMode: address === "DIGITAL" ? "digital" : "physical",
    venueId,
    venueLabel: address,
    officialStreamUrl: null,
    officialEntryUrl: calendarUrl,
    officialNote: entry.liveStreaming ? "LIVE" : address === "DIGITAL" ? "DIGITAL" : null,
    sourceHash: createHash("sha256").update(JSON.stringify(entry)).digest("hex"),
  };
  return normalized;
});

const participantMap = new Map();
for (const entry of normalizedEntries) {
  participantMap.set(entry.participantId, {
    id: entry.participantId,
    displayName: entry.participantNameRaw,
    canonicalKind: entry.maisonId ? "maison" : null,
    reconciliationStatus: entry.maisonId ? "reconciled" : "unreconciled",
    maisonId: entry.maisonId,
  });
}

const addressMap = new Map();
for (const entry of normalizedEntries) {
  if (entry.venueId) addressMap.set(entry.venueId, {
    id: entry.venueId,
    name: null,
    address: entry.venueLabel,
  });
}

const gaps = [
  {
    id: "gap:geo:city:milano:subregion_id",
    entityType: "geo_city",
    entityId: cityId,
    fieldName: "subregion_id",
    reason: "The canonical CityHub record does not provide a sub-region; the relation remains null.",
  },
  {
    id: "gap:geo:city:milano:admin_division_id",
    entityType: "geo_city",
    entityId: cityId,
    fieldName: "admin_division_id",
    reason: "The canonical CityHub record does not provide an administrative division; the relation remains null.",
  },
  {
    id: `gap:${editionId}:collections_reconciliation`,
    entityType: "edition",
    entityId: editionId,
    fieldName: "collections_reconciliation",
    reason: "The official calendar establishes schedule entries; Collection records require a separate verified collection source and are not inferred from every entry.",
  },
  {
    id: "gap:maison:prada:creative_credits",
    entityType: "maison",
    entityId: "maison:prada",
    fieldName: "creative_credits",
    reason: "No creative-credit relation is included until an official or professional source is reconciled.",
  },
  {
    id: `gap:${editionId}:media_occurrences`,
    entityType: "edition",
    entityId: editionId,
    fieldName: "media_occurrences",
    reason: "Media contracts and provenance tables are ready; no media occurrence is asserted by the calendar-only ingest.",
  },
  {
    id: `gap:${editionId}:reviews`,
    entityType: "edition",
    entityId: editionId,
    fieldName: "reviews",
    reason: "Review contracts are ready; no review is asserted by the calendar-only ingest.",
  },
  ...[...participantMap.values()]
    .filter((participant) => participant.reconciliationStatus !== "reconciled")
    .map((participant) => ({
      id: `gap:${participant.id}:maison_id`,
      entityType: "participant",
      entityId: participant.id,
      fieldName: "maison_id",
      reason: "Official calendar participant retained as raw name until an independent Maison reconciliation source is verified.",
    })),
];

const payload = {
  packId: "nex.fashion.milano.ss27",
  version: "2026.10.03",
  contentHash: null,
  retrievedAt,
  source: {
    id: sourceId,
    name: "Camera Nazionale della Moda Italiana (CNMI)",
    sourceKind: "event_organizer",
    authorityTier: "A",
    baseUrl: officialBaseUrl,
    accessMode: "remote_render",
    status: "active",
    endpoint: {
      id: "endpoint:cnmi:calendar",
      endpointType: "calendar",
      baseUrl: calendarUrl,
      accessMethod: "html",
      capabilities: ["calendar", "lineup", "entity_identity", "video_livestream", "search"],
      adapterId: "adapter:cnmi:milano-calendar:v1",
    },
  },
  region: {
    id: "geo:region:europe",
    name: "Europa",
    m49Code: "150",
  },
  country: {
    id: "geo:country:it",
    regionId: "geo:region:europe",
    name: "Itália",
    isoAlpha2: "IT",
    isoAlpha3: "ITA",
    m49Code: "380",
  },
  city: {
    id: cityId,
    countryId: "geo:country:it",
    name: "Milano",
    aliases: ["Milan", "Milano"],
  },
  cityHub: {
    id: "cityhub:milano",
    cityId,
    officialName: "Milano",
    displayName: "Milano",
    slug: "milano",
    hubType: "fashion_city",
    status: "active",
  },
  event: {
    id: "event:milano-fashion-week",
    cityHubId: "cityhub:milano",
    officialName: "Milano Fashion Week",
    displayName: "Milano Fashion Week",
    shortName: "MFW",
    eventType: "fashion_week",
    status: "active",
    startYear: 1958,
    officialWebsite: "https://milanofashionweek.cameramoda.it/",
  },
  segment: {
    id: "segment:womenswear",
    name: "Womenswear",
    code: "womenswear",
    description: "Main womenswear segment used by this vertical slice.",
  },
  edition: {
    id: editionId,
    eventId: "event:milano-fashion-week",
    segmentId: "segment:womenswear",
    displayName: "Milano Fashion Week SS27",
    season: "Spring/Summer",
    seasonCode: "SS27",
    seasonYear: 2027,
    calendarYear: 2026,
    startDate: "2026-09-22",
    endDate: "2026-09-28",
    timeZone: "Europe/Rome",
    editionStatus: "finished",
    officialPageUrl: "https://milanofashionweek.cameramoda.it/en/",
    officialCalendarUrl: calendarUrl,
  },
  venues: [...addressMap.values()].map((venue) => ({
    ...venue,
    cityId,
    venueType: "calendar_location",
  })),
  participants: [...participantMap.values()],
  maisons: [{
    id: "maison:prada",
    officialName: "PRADA",
    displayName: "Prada",
    slug: "prada",
    status: "active",
    officialWebsite: officialBrandUrls.PRADA,
  }],
  entries: normalizedEntries,
  gaps,
};

const hashPayload = { ...payload, contentHash: null };
payload.contentHash = createHash("sha256").update(JSON.stringify(hashPayload)).digest("hex");

const output = resolve(dirname(fileURLToPath(import.meta.url)), "../src-tauri/seed/milano_ss27.json");
mkdirSync(dirname(output), { recursive: true });
writeFileSync(output, `${JSON.stringify(payload, null, 2)}\n`, "utf8");
console.log(`Imported ${normalizedEntries.length} official CNMI entries to ${output}`);
