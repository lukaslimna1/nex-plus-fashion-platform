import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { resolveMedia } from "../src/resolver.js";
import type { Asset } from "@nex-plus/types";

const manifest = JSON.parse(readFileSync(resolve(process.cwd(), "docs/vertical-slice/christian-dior-ss27.json"), "utf8")) as {
  imageGroups: Array<{ assets: Array<Record<string, unknown>> }>;
  videos: Array<Record<string, unknown>>;
};

describe("Christian Dior SS27 media slice", () => {
  it("keeps the 66 Vogue Runway looks ordered and unique", () => {
    const images = manifest.imageGroups[0]!.assets;
    expect(images).toHaveLength(66);
    expect(images.map((asset) => asset.sequenceNumber)).toEqual(Array.from({ length: 66 }, (_, index) => index + 1));
    expect(new Set(images.map((asset) => asset.remoteUrl)).size).toBe(66);
    expect(images.every((asset) => asset.downloadPolicy === "DOWNLOAD_BLOCKED")).toBe(true);
  });

  it("uses the alternate rendition when the primary remote URL fails", () => {
    const first = manifest.imageGroups[0]!.assets[0]!;
    const asset = { ...first, sourceIds: ["source-vogue-runway"], rightsStatus: "UNKNOWN", displayMode: "INLINE", canonicalStatus: "CANONICAL", assetKind: "IMAGE" } as Asset;
    const resolution = resolveMedia(asset, { placeholderUrl: "https://example.test/placeholder.svg", isAvailable: (url) => url !== asset.remoteUrl });
    expect(resolution.kind).toBe("alternative");
    expect(resolution.url).toBe((first.alternativeUrls as string[])[0]);
    expect(resolution.sourcePageUrl).toBe(first.sourcePageUrl);
  });

  it("does not promote Vogue coverage to a complete show", () => {
    expect(manifest.videos[0]).toMatchObject({ provider: "website", videoType: "RUNWAY_COVERAGE", completeness: "PARTIAL", playbackMode: "EXTERNAL_LINK" });
    expect(manifest.videos[0]!.videoType).not.toBe("FULL_SHOW");
  });
});
