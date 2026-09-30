import { describe, expect, it } from "vitest";
import { YOUTUBE_API_KEY_REQUIRED, YouTubeDataAdapter } from "../src/youtube.js";

describe("YouTube Data API adapter", () => {
  it("stops discovery explicitly when the API key is absent", async () => {
    await expect(new YouTubeDataAdapter(undefined).discover("Paris Fashion Week")).rejects.toThrow(YOUTUBE_API_KEY_REQUIRED);
  });

  it("maps Data API discovery to embed-ready metadata without scraping", async () => {
    const adapter = new YouTubeDataAdapter("test-key", async (url) => {
      expect(url).toContain("youtube/v3/search");
      expect(url).toContain("key=test-key");
      return new Response(JSON.stringify({ items: [{ id: { videoId: "abc123" }, snippet: { title: "Official coverage", channelTitle: "Official Channel", thumbnails: { high: { url: "https://i.ytimg.com/vi/abc123/hqdefault.jpg" } } } }] }), { status: 200 });
    });
    await expect(adapter.discover("Official coverage")).resolves.toEqual([{ provider: "youtube", providerAssetId: "abc123", title: "Official coverage", channelName: "Official Channel", sourcePageUrl: "https://www.youtube.com/watch?v=abc123", embedUrl: "https://www.youtube.com/embed/abc123", thumbnailUrl: "https://i.ytimg.com/vi/abc123/hqdefault.jpg" }]);
  });
});
