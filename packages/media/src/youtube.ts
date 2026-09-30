export const YOUTUBE_API_KEY_REQUIRED = "YOUTUBE_API_KEY_REQUIRED";

export interface YouTubeDiscoveryCandidate {
  provider: "youtube";
  providerAssetId: string;
  title: string;
  channelName?: string;
  sourcePageUrl: string;
  embedUrl: string;
  thumbnailUrl?: string;
}

interface YouTubeSearchResponse { items?: Array<{ id?: { videoId?: string }; snippet?: { title?: string; channelTitle?: string; thumbnails?: { high?: { url?: string } } } }> }

export class YouTubeDataAdapter {
  public constructor(private readonly apiKey: string | undefined, private readonly fetcher: typeof fetch = fetch) {}

  public async discover(query: string, maxResults = 10): Promise<YouTubeDiscoveryCandidate[]> {
    if (!this.apiKey) throw new Error(YOUTUBE_API_KEY_REQUIRED);
    const url = new URL("https://www.googleapis.com/youtube/v3/search");
    url.searchParams.set("part", "snippet");
    url.searchParams.set("type", "video");
    url.searchParams.set("q", query);
    url.searchParams.set("maxResults", String(Math.max(1, Math.min(50, Math.trunc(maxResults)))));
    url.searchParams.set("key", this.apiKey);
    const response = await this.fetcher(url.toString());
    if (!response.ok) throw new Error(`YouTube Data API request failed with HTTP ${response.status}`);
    const body = await response.json() as YouTubeSearchResponse;
    return (body.items ?? []).flatMap((item) => {
      const id = item.id?.videoId;
      const title = item.snippet?.title;
      if (!id || !title) return [];
      return [{ provider: "youtube", providerAssetId: id, title, ...(item.snippet?.channelTitle ? { channelName: item.snippet.channelTitle } : {}), sourcePageUrl: `https://www.youtube.com/watch?v=${id}`, embedUrl: `https://www.youtube.com/embed/${id}`, ...(item.snippet?.thumbnails?.high?.url ? { thumbnailUrl: item.snippet.thumbnails.high.url } : {}) }];
    });
  }
}
