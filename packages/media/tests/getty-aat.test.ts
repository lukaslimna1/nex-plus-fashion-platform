import { describe, expect, it } from "vitest";
import { GettyAATAdapter } from "../src/getty-aat.js";

describe("Getty AAT adapter", () => {
  it("uses the official SPARQL response and preserves URI, label, language and retrieval time", async () => {
    const adapter = new GettyAATAdapter({
      endpoint: "https://example.test/sparql",
      now: () => "2026-09-30T00:00:00.000Z",
      fetcher: async (url) => {
        expect(url).toContain("query=");
        expect(url).toContain("format=json");
        return new Response(JSON.stringify({ results: { bindings: [{ concept: { value: "http://vocab.getty.edu/aat/300053634" }, label: { value: "knitting (process)", "xml:lang": "en" } }] } }), { status: 200 });
      }
    });
    await expect(adapter.searchByLabel("knitting (process)")).resolves.toEqual([{
      externalUri: "http://vocab.getty.edu/aat/300053634",
      sourcePageUrl: "https://vocab.getty.edu/page/aat/300053634",
      originalLabel: "knitting (process)",
      language: "en",
      sourceId: "source-getty-aat",
      retrievedAt: "2026-09-30T00:00:00.000Z"
    }]);
  });
});
