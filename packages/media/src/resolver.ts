import type { Asset, DisplayMode } from "@nex-plus/types";

export type MediaResolutionKind = "local" | "cache" | "remote" | "alternative" | "embed" | "thumbnail" | "placeholder";
export interface MediaResolution { url: string; kind: MediaResolutionKind; displayMode: DisplayMode; assetId?: string; }

function usable(value: string | undefined): value is string {
  if (!value) return false;
  try { const url = new URL(value); return url.protocol === "https:"; } catch { return false; }
}

export function resolveMedia(asset: Asset, options: { placeholderUrl: string }): MediaResolution {
  if (usable(asset.localPath)) return { url: asset.localPath, kind: "local", displayMode: asset.displayMode, assetId: asset.id };
  if (usable(asset.cacheUrl)) return { url: asset.cacheUrl, kind: "cache", displayMode: asset.displayMode, assetId: asset.id };
  if (usable(asset.remoteUrl)) return { url: asset.remoteUrl, kind: "remote", displayMode: asset.displayMode, assetId: asset.id };
  const alternative = asset.alternativeUrls.find(usable);
  if (alternative) return { url: alternative, kind: "alternative", displayMode: asset.displayMode, assetId: asset.id };
  if (usable(asset.embedUrl)) return { url: asset.embedUrl, kind: "embed", displayMode: "EMBED", assetId: asset.id };
  if (usable(asset.thumbnailUrl)) return { url: asset.thumbnailUrl, kind: "thumbnail", displayMode: "THUMBNAIL_ONLY", assetId: asset.id };
  return { url: options.placeholderUrl, kind: "placeholder", displayMode: "PLACEHOLDER", assetId: asset.id };
}
