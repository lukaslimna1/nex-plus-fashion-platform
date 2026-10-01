import { describe, expect, it } from "vitest";
import { resolveMedia } from "../src/resolver.js";
import type { Asset } from "@nex-plus/types";

const asset: Asset = {
  id: "asset-julie-kegels-ss27-look-01", sourcePageUrl: "https://www.fhcm.paris/en/collection/julie-kegels-womenswear-springsummer-2027", alternativeUrls: ["https://www.fhcm.paris/sites/default/files/styles/lkt/public/lme/686f17c1d66fe1c55c1c2a3f153456302ed26a2c/FIO00051.jpg?itok=_ebopqbb"],
  remoteUrl: "https://www.fhcm.paris/sites/default/files/styles/lkt/public/lme/686f17c1d66fe1c55c1c2a3f153456302ed26a2c/FIO00038.jpg?itok=289PnrE8", thumbnailUrl: "https://www.fhcm.paris/sites/default/files/styles/lkt/public/lme/686f17c1d66fe1c55c1c2a3f153456302ed26a2c/FIO00038.jpg?itok=289PnrE8", sourceIds: ["source-fhcm", "source-vogue-runway"], rightsStatus: "UNKNOWN",
  downloadPolicy: "DOWNLOAD_BLOCKED", displayMode: "INLINE", canonicalStatus: "CANONICAL", assetKind: "IMAGE", coverageType: "RUNWAY"
};

describe("inline-first media resolver", () => {
  it("prefers remote media before alternatives and placeholder", () => {
    const resolution = resolveMedia(asset, { placeholderUrl: "https://example.com/placeholder.svg" });
    expect(resolution.kind).toBe("remote");
    expect(resolution.sourcePageUrl).toBe(asset.sourcePageUrl);
  });
  it("falls back to the alternative when the real primary URL is unavailable", () => {
    const resolution = resolveMedia(asset, { placeholderUrl: "https://example.com/placeholder.svg", isAvailable: (url) => url !== asset.remoteUrl });
    expect(resolution.kind).toBe("alternative");
    expect(resolution.url).toBe(asset.alternativeUrls[0]);
  });
  it("falls back to thumbnail and then placeholder while preserving sourcePageUrl", () => {
    const thumbnailAsset = { ...asset, thumbnailUrl: "https://example.com/thumbnail.jpg" };
    const unavailable = new Set([thumbnailAsset.remoteUrl, ...thumbnailAsset.alternativeUrls]);
    const thumbnail = resolveMedia(thumbnailAsset, { placeholderUrl: "https://example.com/placeholder.svg", isAvailable: (url) => !unavailable.has(url) });
    expect(thumbnail.kind).toBe("thumbnail");
    const { thumbnailUrl: _thumbnailUrl, ...assetWithoutThumbnail } = asset;
    const placeholder = resolveMedia(assetWithoutThumbnail, { placeholderUrl: "https://example.com/placeholder.svg", isAvailable: () => false });
    expect(placeholder.kind).toBe("placeholder");
    expect(placeholder.sourcePageUrl).toBe(asset.sourcePageUrl);
  });
  it("rejects non-https media before selecting the next fallback", () => {
    expect(resolveMedia({ ...asset, remoteUrl: "javascript:alert(1)", alternativeUrls: [] }, { placeholderUrl: "https://example.com/placeholder.svg" }).kind).toBe("thumbnail");
  });
  it("never promotes Instagram or Facebook to primary playback", () => {
    const metaAsset = { ...asset, assetKind: "VIDEO" as const, provider: "instagram", providerAssetId: "Dd1bejCMjsx", remoteUrl: "https://www.instagram.com/reel/Dd1bejCMjsx/", embedUrl: "https://www.instagram.com/reel/Dd1bejCMjsx/embed/", thumbnailUrl: "https://example.com/meta-thumbnail.jpg", alternativeUrls: [] };
    const resolution = resolveMedia(metaAsset, { placeholderUrl: "https://example.com/placeholder.svg" });
    expect(resolution.kind).toBe("thumbnail");
    expect(resolution.url).not.toContain("instagram");
  });
  it("uses a verified stable alternative when the original provider is Meta", () => {
    const metaAsset = { ...asset, assetKind: "VIDEO" as const, provider: "facebook", remoteUrl: "https://www.facebook.com/reel/123", alternativeUrls: ["https://www.youtube.com/watch?v=verified"] };
    const resolution = resolveMedia(metaAsset, { placeholderUrl: "https://example.com/placeholder.svg" });
    expect(resolution.kind).toBe("alternative");
    expect(resolution.url).toBe(metaAsset.alternativeUrls[0]);
  });
});
