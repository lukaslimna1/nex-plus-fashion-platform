import { describe, expect, it } from "vitest";
import worker from "../src/index.js";

function fakeDb() {
  return {
    prepare(query: string) {
      return {
        bind() { return this; },
        async first() { return { ok: 1 }; },
        async all() {
          if (query.includes("FROM collections")) return { results: [{ id: "c-1", maisonId: "m-1", editionId: "e-1", name: "Verified Collection", slug: "verified-collection", calendarYear: 2026, seasonYear: 2027, seasonCode: "SS27", seasonLabel: "Spring/Summer 2027", canonicalStatus: "CANONICAL", sourceIds: "source-1" }] };
          return { results: [] };
        },
        async run() { return { success: true }; }
      };
    }
  } as never;
}

describe("Worker API contract", () => {
  it("returns non-sensitive health data", async () => {
    const response = await worker.fetch(new Request("https://example.test/api/health"), { DB: fakeDb(), APP_ENV: "test", WORKER_VERSION: "test", GEMINI_API_KEY: "secret-is-not-returned" });
    const body = await response.json() as Record<string, unknown>;
    expect(response.status).toBe(200);
    expect(body).not.toHaveProperty("GEMINI_API_KEY");
    expect(body).toMatchObject({ status: "ok", environment: "test", database: { reachable: true }, aiProviders: { geminiConfigured: true } });
  });
  it("returns frontend-ready list envelopes", async () => {
    const response = await worker.fetch(new Request("https://example.test/api/collections"), { DB: fakeDb(), APP_ENV: "test" });
    expect(await response.json()).toMatchObject({ data: [{ id: "c-1", seasonCode: "SS27" }], meta: { count: 1, revision: 1 } });
  });
});
