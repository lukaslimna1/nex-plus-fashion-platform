import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const seed = JSON.parse(readFileSync(resolve(root, "src-tauri/seed/milano_ss27.json"), "utf8"));
const canonicalize = (value) => {
  if (Array.isArray(value)) return value.map(canonicalize);
  if (value && typeof value === "object") {
    return Object.fromEntries(Object.keys(value).sort().map((key) => [key, canonicalize(value[key])]));
  }
  return value;
};
const payload = canonicalize(seed);
const payloadJson = JSON.stringify(payload);
const contentHash = createHash("sha256").update(payloadJson, "utf8").digest("hex");
const manifest = {
  format: "nexpack",
  formatVersion: 1,
  packId: seed.packId,
  version: seed.version,
  family: "base_catalog",
  title: seed.edition.displayName,
  scope: {
    cityHubId: seed.cityHub.id,
    eventId: seed.event.id,
    editionId: seed.edition.id,
  },
  schemaVersion: "catalog.milano.v1",
  appCompatibility: {
    minCoreProtocol: "0.1",
    minAppVersion: "0.1.0",
  },
  contentHash,
  sizeBytes: Buffer.byteLength(payloadJson, "utf8"),
  origin: {
    sourceIds: [seed.source.id],
    evidenceUrls: [seed.edition.officialPageUrl, seed.edition.officialCalendarUrl],
    retrievedAt: seed.retrievedAt,
  },
  integrity: {
    algorithm: "sha256",
    payloadEncoding: "utf8-json",
    payloadHash: contentHash,
  },
  entityCounts: {
    city_hub: 1,
    event: 1,
    edition: 1,
    schedule_entry: seed.entries.length,
    participant: seed.participants.length,
    venue: seed.venues.length,
    maison: seed.maisons.length,
    source: 1,
  },
};

const pack = {
  format: "nexpack",
  formatVersion: 1,
  manifest,
  payload,
};
const output = resolve(root, "packs/milano-ss27-2026.10.03.nexpack");
mkdirSync(dirname(output), { recursive: true });
writeFileSync(output, `${JSON.stringify(pack, null, 2)}\n`, "utf8");
console.log(`Built ${output} (${Buffer.byteLength(JSON.stringify(pack), "utf8")} bytes)`);
