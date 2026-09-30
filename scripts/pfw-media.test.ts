import { describe, expect, it } from "vitest";
import { PFW_SS27_28_30_SCHEDULE, extractVogueRunwayImages, toParisIso } from "./pfw-media.js";

describe("Paris Fashion Week SS27 media ingestion", () => {
  it("keeps the official first three days as a source snapshot", () => {
    expect(PFW_SS27_28_30_SCHEDULE).toHaveLength(30);
    expect(PFW_SS27_28_30_SCHEDULE.filter((entry) => entry.localDate === "2026-09-28")).toHaveLength(8);
    expect(PFW_SS27_28_30_SCHEDULE.filter((entry) => entry.localDate === "2026-09-29")).toHaveLength(10);
    expect(PFW_SS27_28_30_SCHEDULE.filter((entry) => entry.localDate === "2026-09-30")).toHaveLength(12);
    expect(PFW_SS27_28_30_SCHEDULE.find((entry) => entry.title === "Christian Dior")).toMatchObject({ localDate: "2026-09-29", localTime: "14:30", format: "SHOW" });
  });

  it("deduplicates identical provider files without deduplicating different looks", () => {
    const html = [
      '<img src="https:\\/\\/assets.vogue.com\\/photos\\/photo-01\\/master\\/w_1600,c_limit\\/00001-christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg">',
      '<img src="https:\\/\\/assets.vogue.com\\/photos\\/photo-01\\/master\\/w_960,c_limit\\/00001-christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg">',
      '<img src="https://assets.vogue.com/photos/photo-02/master/w_1600,c_limit/00002-christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg">'
    ];
    const images = extractVogueRunwayImages(html.join(""), "christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg");
    expect(images).toHaveLength(2);
    expect(images.map((image) => image.sequenceNumber)).toEqual([1, 2]);
    expect(images[0]).toMatchObject({ providerAssetId: "photo-01/00001-christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg", photographer: "Filippo Fior" });
  });

  it("does not collapse different photos that share a look sequence", () => {
    const html = [
      '<img src="https://assets.vogue.com/photos/photo-a/master/w_1600,c_limit/00001-christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg">',
      '<img src="https://assets.vogue.com/photos/photo-b/master/w_1600,c_limit/00001-christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg">'
    ].join("");
    const images = extractVogueRunwayImages(html, "christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg");
    expect(images).toHaveLength(2);
    expect(images.map((image) => image.providerAssetId)).toEqual([
      "photo-a/00001-christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg",
      "photo-b/00001-christian-dior-spring-2027-ready-to-wear-credit-gorunway.jpg"
    ]);
  });

  it("preserves Paris civil time", () => {
    expect(toParisIso("2026-09-29", "14:30")).toBe("2026-09-29T12:30:00.000Z");
  });
});
