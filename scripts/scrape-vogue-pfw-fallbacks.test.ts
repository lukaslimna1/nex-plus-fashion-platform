import { describe, expect, it } from "vitest";
import { buildVogueFallbackMigration, parseVogueRunwayFallbackImages, type VogueFallbackSpec } from "./scrape-vogue-pfw-fallbacks.js";

const spec: VogueFallbackSpec = {
  name: "HODAKOVA",
  localDate: "2026-09-28",
  slug: "hodakova",
  filenamePrefix: "hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg",
  creditSource: "GORUNWAY"
};

describe("Vogue Runway SS27 fallback scraper", () => {
  it("preserves ordered distinct looks and remote fallbacks without inventing photographer data", () => {
    const html = `
      <img src="https://assets.vogue.com/photos/photo-01/master/w_1600,c_limit/00001-hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg" />
      <img src="https://assets.vogue.com/photos/detail-01/master/w_1600,c_limit/00001-hodakova-spring-2027-ready-to-wear-detail-credit-gorunway.jpg" />
      <img src="https://assets.vogue.com/photos/photo-02/master/w_1600,c_limit/00002-hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg" />
      <img src="https://assets.vogue.com/photos/photo-01/master/w_1600,c_limit/00001-hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg" />
    `;
    const images = parseVogueRunwayFallbackImages(spec, html);
    expect(images).toHaveLength(3);
    expect(images.map((image) => image.sequenceNumber)).toEqual([1, 1, 2]);
    expect(images.map((image) => image.coverageScope)).toEqual(["LOOKBOOK_IMAGE", "DETAIL", "LOOKBOOK_IMAGE"]);
    expect(images[0]).toMatchObject({
      providerAssetId: "photo-01/00001-hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg",
      alternativeUrls: ["https://assets.vogue.com/photos/photo-01/master/w_960,c_limit/00001-hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg"],
      thumbnailUrl: "https://assets.vogue.com/photos/photo-01/master/w_360%2Cc_limit/00001-hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg",
      downloadPolicy: "DOWNLOAD_BLOCKED",
      rightsStatus: "UNKNOWN"
    });
    expect(images[0]).not.toHaveProperty("photographer");
  });

  it("writes idempotent collection-scoped SQL and preserves source provenance", () => {
    const migration = buildVogueFallbackMigration([{
      name: "HODAKOVA",
      localDate: "2026-09-28",
      collectionId: "collection-pfw-ss27-hodakova-2026",
      sourcePageUrl: "https://www.vogue.com/fashion-shows/spring-2027-ready-to-wear/hodakova",
      images: parseVogueRunwayFallbackImages(spec, "https://assets.vogue.com/photos/photo-01/master/w_1600,c_limit/00001-hodakova-spring-2027-ready-to-wear-credit-gorunway.jpg")
    }]);
    expect(migration).toContain("WHERE collection_id = 'collection-pfw-ss27-hodakova-2026' AND remote_url =");
    expect(migration).toContain("'source-vogue-runway'");
    expect(migration).toContain("DOWNLOAD_BLOCKED");
  });
});
