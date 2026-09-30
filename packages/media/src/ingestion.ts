export type FetchStrategy = "OFFICIAL_API" | "JSON_RSS" | "HTML_CHEERIO" | "PDF" | "CRAWLEE_HTTP" | "PLAYWRIGHT";

export interface IngestionSourceMetadata {
  sourceId: string;
  authorityTier: "A" | "B" | "C" | "D";
  entityTypes: string[];
  fetchStrategy: FetchStrategy;
  rateLimitPerMinute?: number;
  rightsBehavior: "REMOTE_ONLY" | "EMBED_ONLY" | "LICENSE_REQUIRED" | "OPEN_LICENSE";
}

export interface IngestionAdapter<TFetched, TParsed, TNormalized> {
  source: IngestionSourceMetadata;
  fetch(): Promise<TFetched>;
  parse(input: TFetched): Promise<TParsed> | TParsed;
  normalize(input: TParsed): Promise<TNormalized> | TNormalized;
  validate(input: TNormalized): Promise<string[]> | string[];
}

export interface PendingReview<T> {
  status: "PENDING_REVIEW";
  sourceId: string;
  retrievedAt: string;
  candidate: T;
  researchGaps: string[];
}

export async function collectForReview<TFetched, TParsed, TNormalized>(adapter: IngestionAdapter<TFetched, TParsed, TNormalized>, retrievedAt = new Date().toISOString()): Promise<PendingReview<TNormalized>> {
  const fetched = await adapter.fetch();
  const parsed = await adapter.parse(fetched);
  const candidate = await adapter.normalize(parsed);
  const researchGaps = await adapter.validate(candidate);
  return { status: "PENDING_REVIEW", sourceId: adapter.source.sourceId, retrievedAt, candidate, researchGaps };
}

export function deduplicateRemoteAssets<T extends { remoteUrl?: string; provider?: string; providerAssetId?: string }>(assets: T[]): T[] {
  const seen = new Set<string>();
  return assets.filter((asset) => {
    const identity = asset.remoteUrl || (asset.provider && asset.providerAssetId ? `${asset.provider}:${asset.providerAssetId}` : undefined);
    if (!identity || seen.has(identity)) return !identity;
    seen.add(identity);
    return true;
  });
}
