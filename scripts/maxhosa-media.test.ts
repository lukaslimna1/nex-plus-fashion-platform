import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { resolveMedia } from "@nex-plus/media";
import type { Asset } from "@nex-plus/types";

type Manifest = {
  imageGroups: Array<{ sourceId: string; coverageType: string; assets: Array<Record<string, unknown>> }>;
  videos: Array<Record<string, unknown>>;
  reviews: Array<Record<string, unknown>>;
  provenance: { catalogedImageCount: number; remoteImageCount: number; linkOnlyImageCount: number; deduplication: { inputCount: number; distinctRemoteFiles: number; duplicateFiles: string[] } };
};

describe("MAXHOSA AFRICA SS27 media manifest", () => {
  const manifest = JSON.parse(readFileSync(resolve(process.cwd(), "docs/vertical-slice/maxhosa-africa-ss27.json"), "utf8")) as Manifest;
  const images = manifest.imageGroups.flatMap((group) => group.assets);
  const remoteRunwayImages = manifest.imageGroups.find((group) => group.sourceId === "source-vogue-runway")!.assets;

  it("keeps the 50 Vogue Runway images in published order without look-based deduplication", () => {
    expect(remoteRunwayImages).toHaveLength(50);
    expect(remoteRunwayImages.map((asset) => asset.sequenceNumber)).toEqual(Array.from({ length: 50 }, (_, index) => index + 1));
    expect(new Set(remoteRunwayImages.map((asset) => asset.remoteUrl)).size).toBe(50);
    for (const asset of remoteRunwayImages) {
      expect(asset.sourcePageUrl).toContain("vogue.com/fashion-shows/spring-2027-ready-to-wear/maxhosa/slideshow/collection");
      expect(asset.remoteUrl).toMatch(/^https:\/\/assets\.vogue\.com\/photos\/.+\/master\/w_1600,c_limit\/.+\.jpg$/);
      expect(asset.alternativeUrls).toHaveLength(1);
      expect(asset.thumbnailUrl).toContain("w_360%2Cc_limit");
      expect(asset.photographer).toBe("Filippo Fior");
      expect(asset.creditLine).toBe("Filippo Fior / Gorunway.com");
      expect(asset.downloadPolicy).toBe("DOWNLOAD_BLOCKED");
    }
  });

  it("keeps the second source as a licensed link-only image", () => {
    const reuters = images.find((asset) => asset.id === "asset-maxhosa-africa-ss27-reuters-context");
    expect(reuters).toMatchObject({ provider: "Reuters Connect", photographer: "Sarah Meyssonnier", displayMode: "LINK_ONLY", rightsStatus: "RESTRICTED", downloadPolicy: "DOWNLOAD_BLOCKED" });
    expect(reuters?.remoteUrl).toBeUndefined();
    expect(manifest.provenance).toMatchObject({ catalogedImageCount: 51, remoteImageCount: 50, linkOnlyImageCount: 1, deduplication: { inputCount: 51, distinctRemoteFiles: 50, duplicateFiles: [] } });
  });

  it("uses the alternate remote rendition when the primary image URL fails", () => {
    const first = remoteRunwayImages[0]!;
    const asset = { ...first, sourceIds: ["source-vogue-runway"], rightsStatus: "UNKNOWN", downloadPolicy: "DOWNLOAD_BLOCKED", displayMode: "INLINE", canonicalStatus: "CANONICAL", assetKind: "IMAGE" } as Asset;
    const resolution = resolveMedia(asset, { placeholderUrl: "https://example.test/placeholder.svg", isAvailable: (url) => url !== asset.remoteUrl });
    expect(resolution.kind).toBe("alternative");
    expect(resolution.url).toBe((first.alternativeUrls as string[])[0]);
    expect(resolution.sourcePageUrl).toBe(first.sourcePageUrl);
  });

  it("models the official YouTube film and repost alternative without claiming a full show", () => {
    expect(manifest.videos).toHaveLength(1);
    expect(manifest.videos[0]).toMatchObject({ provider: "YouTube", providerAssetId: "yFus8VAaRME", videoType: "OFFICIAL_FILM", completeness: "UNKNOWN", officiality: "OFFICIAL", embedUrl: "https://www.youtube.com/embed/yFus8VAaRME" });
    expect(manifest.videos[0]!.alternativeUrls).toEqual(["https://www.youtube.com/watch?v=Kl3OGBtMNl4"]);
    expect(manifest.videos[0]!.videoType).not.toBe("FULL_SHOW");
  });

  it("keeps the professional review metadata without full review text", () => {
    expect(manifest.reviews).toContainEqual(expect.objectContaining({ publication: "Vogue Runway", author: "Tina Isaac-Goizé", title: "Maxhosa Spring 2027 Ready-to-Wear Collection" }));
    expect(manifest.reviews[0]).not.toHaveProperty("body");
  });
});
