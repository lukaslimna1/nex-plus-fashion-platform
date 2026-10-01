import type { Asset, DisplayMode } from "@nex-plus/types";

export type MediaResolutionKind = "local" | "cache" | "remote" | "alternative" | "embed" | "thumbnail" | "placeholder";
export interface MediaResolution { url: string; kind: MediaResolutionKind; displayMode: DisplayMode; assetId?: string; sourcePageUrl: string; }

function usable(value: string | undefined): value is string {
  if (!value) return false;
  try { const url = new URL(value); return url.protocol === "https:"; } catch { return false; }
}

function isMetaProvider(asset: Asset): boolean {
  const provider = asset.provider?.toLowerCase();
  return provider === "instagram" || provider === "facebook" || asset.playbackMode === "INSTAGRAM_EMBED" || asset.playbackMode === "FACEBOOK_EMBED";
}

function isMetaUrl(value: string): boolean {
  try {
    const host = new URL(value).hostname.toLowerCase();
    return host === "instagram.com" || host.endsWith(".instagram.com") || host === "facebook.com" || host.endsWith(".facebook.com") || host === "fb.watch";
  } catch { return false; }
}

export function resolveMedia(asset: Asset, options: { placeholderUrl: string; isAvailable?: (url: string) => boolean }): MediaResolution {
  const available = options.isAvailable ?? (() => true);
  const metaProvider = isMetaProvider(asset);
  const choose = (url: string | undefined, kind: MediaResolutionKind, displayMode: DisplayMode): MediaResolution | undefined => {
    if (!usable(url) || isMetaUrl(url) || !available(url)) return undefined;
    return { url, kind, displayMode, assetId: asset.id, sourcePageUrl: asset.sourcePageUrl };
  };
  return choose(asset.localPath, "local", asset.displayMode)
    ?? choose(asset.cacheUrl, "cache", asset.displayMode)
    ?? (!metaProvider ? choose(asset.remoteUrl, "remote", asset.displayMode) : undefined)
    ?? asset.alternativeUrls.map((url) => choose(url, "alternative", asset.displayMode)).find((result): result is MediaResolution => Boolean(result))
    ?? (!metaProvider ? choose(asset.embedUrl, "embed", "EMBED") : undefined)
    ?? choose(asset.thumbnailUrl, "thumbnail", "THUMBNAIL_ONLY")
    ?? { url: options.placeholderUrl, kind: "placeholder", displayMode: "PLACEHOLDER", assetId: asset.id, sourcePageUrl: asset.sourcePageUrl };
}
