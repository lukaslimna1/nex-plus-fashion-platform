import { describe, expect, it } from "vitest";
import { collectForReview, deduplicateRemoteAssets } from "../src/ingestion.js";

describe("generic source ingestion boundary", () => {
  it("keeps provenance and stops at pending review before canonical upsert", async () => {
    const result = await collectForReview({
      source: { sourceId: "source-fhcm", authorityTier: "A", entityTypes: ["ScheduleEntry"], fetchStrategy: "HTML_CHEERIO", rightsBehavior: "REMOTE_ONLY" },
      async fetch() { return "official html"; },
      parse(input) { return { source: input }; },
      normalize(input) { return { ...input, verified: true }; },
      validate(input) { return input.verified ? [] : ["verified"]; }
    }, "2026-09-30T00:00:00.000Z");
    expect(result).toEqual({ status: "PENDING_REVIEW", sourceId: "source-fhcm", retrievedAt: "2026-09-30T00:00:00.000Z", candidate: { source: "official html", verified: true }, researchGaps: [] });
  });

  it("deduplicates only identical remote files, keeping different photos of one look", () => {
    const assets = deduplicateRemoteAssets([
      { remoteUrl: "https://example.test/photo-a.jpg", provider: "FHCM", providerAssetId: "a" },
      { remoteUrl: "https://example.test/photo-a.jpg", provider: "FHCM", providerAssetId: "a-copy" },
      { remoteUrl: "https://example.test/photo-b.jpg", provider: "FHCM", providerAssetId: "b" },
      { remoteUrl: "https://example.test/photo-c.jpg", provider: "Vogue Runway", providerAssetId: "c" }
    ]);
    expect(assets).toHaveLength(3);
    expect(assets.map((asset) => asset.remoteUrl)).toEqual(["https://example.test/photo-a.jpg", "https://example.test/photo-b.jpg", "https://example.test/photo-c.jpg"]);
  });
});
