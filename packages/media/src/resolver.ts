import type { Asset, DisplayMode } from "@nex-plus/types";

export type MediaResolutionKind = "local" | "cache" | "remote" | "alternative" | "embed" | "thumbnail" | "placeholder";
export interface MediaResolution { url: string; kind: MediaResolutionKind; displayMode: DisplayMode; assetId?: string; sourcePageUrl: string; }

function usable(value: string | undefined): value is string {
  if (!value) return false;
  try { const url = new URL(value); return url.protocol === "https:"; } catch { return false; }
}

export function resolveMedia(asset: Asset, options: { placeholderUrl: string; isAvailable?: (url: string) => boolean }): MediaResolution {
  const available = options.isAvailable ?? (() => true);
  const choose = (url: string | undefined, kind: MediaResolutionKind, displayMode: DisplayMode): MediaResolution | undefined => {
    if (!usable(url) || !available(url)) return undefined;
    return { url, kind, displayMode, assetId: asset.id, sourcePageUrl: asset.sourcePageUrl };
  };
  return choose(asset.localPath, "local", asset.displayMode)
    ?? choose(asset.cacheUrl, "cache", asset.displayMode)
    ?? choose(asset.remoteUrl, "remote", asset.displayMode)
    ?? asset.alternativeUrls.map((url) => choose(url, "alternative", asset.displayMode)).find((result): result is MediaResolution => Boolean(result))
    ?? choose(asset.embedUrl, "embed", "EMBED")
    ?? choose(asset.thumbnailUrl, "thumbnail", "THUMBNAIL_ONLY")
    ?? { url: options.placeholderUrl, kind: "placeholder", displayMode: "PLACEHOLDER", assetId: asset.id, sourcePageUrl: asset.sourcePageUrl };
}
