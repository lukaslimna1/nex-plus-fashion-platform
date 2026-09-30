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

function detailDb() {
  return {
    prepare(query: string) {
      return {
        bind() { return this; },
        async first() { return { ok: 1 }; },
        async all() {
          if (query.includes("FROM collections")) return { results: [{ id: "c-1", maisonId: "m-1", editionId: "e-1", name: "Verified Collection", slug: "verified-collection", calendarYear: 2026, seasonYear: 2027, seasonCode: "SS27", seasonLabel: "Spring/Summer 2027", presentedOn: "2026-09-28", canonicalStatus: "CANONICAL", sourceIds: "source-1" }] };
          if (query.includes("FROM maisons")) return { results: [{ id: "m-1", name: "Verified Maison", slug: "verified-maison", websiteUrl: "https://example.test", officialSourceIds: "source-1" }] };
          if (query.includes("FROM editions")) return { results: [{ id: "e-1", eventId: "event-1", cityHubId: "city-1", calendarYear: 2026, seasonYear: 2027, seasonCode: "SS27", seasonLabel: "Spring/Summer 2027", status: "CANONICAL" }] };
          if (query.includes("FROM events")) return { results: [{ id: "event-1", name: "Event", slug: "event", kind: "FASHION_WEEK" }] };
          if (query.includes("FROM city_hubs")) return { results: [{ id: "city-1", name: "Paris", slug: "paris", countryId: "country-1", regionId: "region-1", timezone: "Europe/Paris" }] };
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
  it("returns the frontend home rail contract", async () => {
    const response = await worker.fetch(new Request("https://example.test/api/home"), { DB: fakeDb(), APP_ENV: "test" });
    const body = await response.json() as { data: { rails: Record<string, unknown> } };
    expect(response.status).toBe(200);
    expect(Object.keys(body.data.rails)).toEqual(["happeningNow", "upcoming", "recentCollections", "latestPresentations", "videos", "maisons", "reviews", "trends", "library"]);
  });
  it("exposes state-filtered schedule endpoints", async () => {
    const response = await worker.fetch(new Request("https://example.test/api/schedule/now"), { DB: fakeDb(), APP_ENV: "test" });
    expect(response.status).toBe(200);
    expect(await response.json()).toMatchObject({ data: [], meta: { count: 0, revision: 1 } });
    const upcomingResponse = await worker.fetch(new Request("https://example.test/api/schedule/upcoming"), { DB: fakeDb(), APP_ENV: "test" });
    expect(upcomingResponse.status).toBe(200);
    expect(await upcomingResponse.json()).toMatchObject({ data: [], meta: { count: 0, revision: 1 } });
  });
  it("returns typed collection and maison details by slug", async () => {
    const collectionResponse = await worker.fetch(new Request("https://example.test/api/collections/verified-collection"), { DB: detailDb(), APP_ENV: "test" });
    expect(collectionResponse.status).toBe(200);
    expect(await collectionResponse.json()).toMatchObject({ data: { collection: { slug: "verified-collection" }, maison: { slug: "verified-maison" }, edition: { id: "e-1" }, event: { id: "event-1" }, city: { slug: "paris" } } });
    const maisonResponse = await worker.fetch(new Request("https://example.test/api/maisons/verified-maison"), { DB: detailDb(), APP_ENV: "test" });
    expect(maisonResponse.status).toBe(200);
    expect(await maisonResponse.json()).toMatchObject({ data: { maison: { slug: "verified-maison" }, collections: [{ slug: "verified-collection" }] } });
  });

  it("passes route catalog filters without truncating the data source", async () => {
    const queries: string[] = [];
    const db = {
      prepare(query: string) {
        queries.push(query);
        return {
          bind(...values: unknown[]) { void values; return this; },
          async first() { return { ok: 1 }; },
          async all() {
            if (query.includes("FROM city_hubs")) return { results: [{ id: "city-paris", name: "Paris", slug: "paris", countryId: "country-france", regionId: "region-europe", countryName: "França", regionName: "Europa", aliases: "[]", relatedEventIds: "event-paris-fashion-week", complementarySourceIds: "", coverAssetKey: "cover-city-paris", coverUrl: "/assets/covers/cities/cover-city-paris.png", coverMatchStatus: "MATCHED", coverFallback: 0, researchStatus: "COMPLETE", importStatus: "UNCHANGED" }] };
            if (query.includes("FROM events")) return { results: [{ id: "event-paris-fashion-week", name: "Paris Fashion Week", slug: "paris-fashion-week", kind: "FASHION_WEEK", eventType: "Fashion Week", aliases: "[]", cityHubIds: "city-paris", socials: "{}", complementarySourceIds: "", coverAssetKey: "cover-event-paris-fashion-week", coverUrl: "/assets/covers/events/cover-event-paris-fashion-week.png", coverMatchStatus: "MATCHED", coverFallback: 0, researchStatus: "COMPLETE", importStatus: "UNCHANGED" }] };
            return { results: [] };
          },
          async run() { return { success: true }; }
        };
      }
    } as never;
    const cities = await worker.fetch(new Request("https://example.test/api/cities?region=europa&country=franca&status=COMPLETE&hasCover=true"), { DB: db, APP_ENV: "test" });
    expect(cities.status).toBe(200);
    expect(await cities.json()).toMatchObject({ data: [{ slug: "paris", cover: { assetKey: "cover-city-paris", fallback: false } }] });
    const events = await worker.fetch(new Request("https://example.test/api/events?status=COMPLETE&type=Fashion%20Week&city=paris&hasCover=true"), { DB: db, APP_ENV: "test" });
    expect(events.status).toBe(200);
    expect(await events.json()).toMatchObject({ data: [{ slug: "paris-fashion-week", cityHubIds: ["city-paris"], cover: { fallback: false } }] });
    expect(queries.some((query) => query.includes("r.slug = ?") && query.includes("h.cover_fallback = ?"))).toBe(true);
    expect(queries.some((query) => query.includes("EXISTS (SELECT 1 FROM event_locations") && query.includes("e.event_type = ?") && query.includes("e.cover_fallback = ?"))).toBe(true);
  });
});
