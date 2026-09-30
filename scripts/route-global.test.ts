import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { buildRouteImportPlan, renderRouteImportSql, type RouteCoverManifest, type RouteNotionSnapshot } from "@nex-plus/data";

const snapshot = JSON.parse(readFileSync(resolve(process.cwd(), "docs/research/route-global-notion.snapshot.json"), "utf8")) as RouteNotionSnapshot;
const covers = JSON.parse(readFileSync(resolve(process.cwd(), "docs/research/route-global-covers.json"), "utf8")) as RouteCoverManifest;
const options = { coverManifest: covers, legacyRegionIds: { europe: "region-europe", europa: "region-europe", africa: "region-africa" }, legacyCountryIds: { franca: "country-france", "africa-do-sul": "country-south-africa" }, legacyCityIds: { paris: "city-paris" }, legacyEventIds: { "paris-fashion-week": "event-paris-fashion-week" } };

describe("Route Global Notion import", () => {
  it("imports every current source row without hardcoded catalog counts", () => {
    const plan = buildRouteImportPlan(snapshot, options);
    expect(plan.cities).toHaveLength(snapshot.cities.length);
    expect(plan.events).toHaveLength(snapshot.events.length);
    expect(plan.cities.every((city) => city.notionPageId && city.notionUrl && city.sourceHash && city.lastImportedAt)).toBe(true);
    expect(plan.events.every((event) => event.notionPageId && event.notionUrl && event.sourceHash && event.lastImportedAt)).toBe(true);
    expect(plan.locations.length).toBeGreaterThan(0);
    expect(new Set(plan.locations.map((location) => location.eventId)).size).toBeLessThanOrEqual(plan.events.length);
  });

  it("is idempotent and classifies unchanged records by source hash", () => {
    const first = buildRouteImportPlan(snapshot, options);
    const existing = {
      citiesByNotionPageId: Object.fromEntries(first.cities.map((city) => [city.notionPageId, { id: city.id, sourceHash: city.sourceHash, importStatus: city.importStatus }])),
      eventsByNotionPageId: Object.fromEntries(first.events.map((event) => [event.notionPageId, { id: event.id, sourceHash: event.sourceHash, importStatus: event.importStatus }]))
    };
    const second = buildRouteImportPlan(snapshot, { ...options, existing });
    expect(second.cities.map((city) => city.id)).toEqual(first.cities.map((city) => city.id));
    expect(second.events.map((event) => event.id)).toEqual(first.events.map((event) => event.id));
    expect(second.cities.filter((city) => city.importStatus === "UNCHANGED").length).toBeGreaterThan(0);
    expect(second.events.filter((event) => event.importStatus === "UNCHANGED").length).toBeGreaterThan(0);
  });

  it("classifies a changed page as UPDATED and preserves aliases and relationships", () => {
    const first = buildRouteImportPlan(snapshot, options);
    const paris = snapshot.cities.find((city) => city.Cidade === "Paris")!;
    const parisEvent = snapshot.events.find((event) => event.Evento === "Paris Fashion Week")!;
    const changed: RouteNotionSnapshot = { ...snapshot, cities: snapshot.cities.map((city) => city === paris ? { ...city, Observações: "Changed editorial note" } : city), events: snapshot.events.map((event) => event === parisEvent ? { ...event, Observações: "Changed event note" } : event) };
    const existing = {
      citiesByNotionPageId: { [first.cities.find((city) => city.name === "Paris")!.notionPageId]: { id: "city-paris", sourceHash: first.cities.find((city) => city.name === "Paris")!.sourceHash, importStatus: "NEW" as const } },
      eventsByNotionPageId: { [first.events.find((event) => event.name === "Paris Fashion Week")!.notionPageId]: { id: "event-paris-fashion-week", sourceHash: first.events.find((event) => event.name === "Paris Fashion Week")!.sourceHash, importStatus: "NEW" as const } }
    };
    const next = buildRouteImportPlan(changed, { ...options, existing });
    expect(next.cities.find((city) => city.name === "Paris")).toMatchObject({ id: "city-paris", importStatus: "UPDATED" });
    expect(next.events.find((event) => event.name === "Paris Fashion Week")).toMatchObject({ id: "event-paris-fashion-week", importStatus: "UPDATED" });
    expect(next.events.find((event) => event.name === "Paris Fashion Week")?.cityHubIds).toContain("city-paris");
    expect(next.cities.find((city) => city.name === "Odesa")?.aliases).toContain("Odessa");
  });

  it("keeps cover matching states and emits rerunnable SQL", () => {
    const plan = buildRouteImportPlan(snapshot, options);
    expect(plan.cities.find((city) => city.name === "Paris")?.cover).toMatchObject({ status: "MATCHED", fallback: false });
    expect(plan.cities.find((city) => city.name === "Sarajevo")?.cover).toMatchObject({ status: "ALIAS_MATCH", fallback: false });
    expect(plan.cities.find((city) => city.name === "Columbus")?.cover).toMatchObject({ status: "MISSING", fallback: true, assetKey: "cover-system-em-breve" });
    expect(renderRouteImportSql(plan)).toContain("ON CONFLICT(id) DO UPDATE");
    expect(renderRouteImportSql(plan)).toContain("DELETE FROM event_locations");
  });
});


