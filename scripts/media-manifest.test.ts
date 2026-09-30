import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

interface ManifestAsset {
  id: string;
  sequenceNumber?: number;
  assetKind: "IMAGE" | "VIDEO";
  coverageScope?: string;
  sourcePageUrl: string;
  remoteUrl?: string;
  alternativeUrls: string[];
  thumbnailUrl?: string;
  provider?: string;
  providerAssetId?: string;
  creator?: string;
  photographer?: string;
  creditLine?: string;
  downloadPolicy: string;
  embedUrl?: string;
}

describe("Julie Kegels SS27 media manifest", () => {
  const manifest = JSON.parse(readFileSync(resolve(process.cwd(), "docs/vertical-slice/julie-kegels-ss27.json"), "utf8")) as { media: { catalogedImageCount: number; deduplication: { method: string; inputCount: number; distinctRemoteFiles: number; duplicateHeaderGroups: number; duplicateFiles: string[] }; assets: ManifestAsset[] } };

  it("keeps all 43 distinct FHCM lookbook images in source order", () => {
    const images = manifest.media.assets.filter((asset) => asset.assetKind === "IMAGE");
    expect(manifest.media.catalogedImageCount).toBe(43);
    expect(manifest.media.deduplication).toEqual({ method: "remoteUrl plus ETag plus Content-Length header identity", inputCount: 43, distinctRemoteFiles: 43, duplicateHeaderGroups: 0, duplicateFiles: [] });
    expect(images).toHaveLength(43);
    expect(images.map((asset) => asset.sequenceNumber)).toEqual(Array.from({ length: 43 }, (_, index) => index + 1));
    expect(new Set(images.map((asset) => asset.remoteUrl)).size).toBe(43);
    for (const image of images) {
      expect(image.sourcePageUrl).toContain("fhcm.paris/en/collection/julie-kegels");
      expect(image.remoteUrl).toMatch(/^https:\/\/www\.fhcm\.paris\/sites\/default\/files\/styles\/lkt\/public\/lme\/.+\.jpg\?itok=/);
      expect(image.thumbnailUrl).toBe(image.remoteUrl);
      expect(image.provider).toBe("FHCM");
      expect(image.creator).toBe("FHCM / Launchmetrics");
      expect(image.photographer).toBe("Filippo Fior");
      expect(image.creditLine).toContain("Gorunway.com");
      expect(image.downloadPolicy).toBe("DOWNLOAD_BLOCKED");
    }
  });

  it("keeps the NSS reel classified as coverage, not a complete show", () => {
    const reel = manifest.media.assets.find((asset) => asset.assetKind === "VIDEO");
    expect(reel).toMatchObject({ provider: "Instagram", providerAssetId: "Dd1bejCMjsx", coverageScope: "RUNWAY_COVERAGE_REEL", downloadPolicy: "DOWNLOAD_BLOCKED" });
    expect(reel?.sourcePageUrl).toContain("nssmag.com");
    expect(reel?.embedUrl).toBe("https://www.instagram.com/reel/Dd1bejCMjsx/embed/");
  });
});
