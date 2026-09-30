import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { buildRouteImportPlan, coverManifestStatus, renderRouteImportSql, type RouteCoverManifest, type RouteImportPlan, type RouteNotionSnapshot } from "@nex-plus/data";

function argument(name: string, fallback: string): string {
  const index = process.argv.indexOf(name);
  return index >= 0 && process.argv[index + 1] ? process.argv[index + 1]! : fallback;
}
function readJson<T>(path: string): T { return JSON.parse(readFileSync(resolve(path), "utf8")) as T; }
function report(plan: RouteImportPlan) {
  const regionDistribution = plan.cities.reduce<Record<string, number>>((result, city) => { result[city.regionName] = (result[city.regionName] ?? 0) + 1; return result; }, {});
  const research = (items: Array<{ researchStatus: string }>) => items.reduce<Record<string, number>>((result, item) => { result[item.researchStatus] = (result[item.researchStatus] ?? 0) + 1; return result; }, {});
  const incompleteCities = plan.cities.filter((city) => city.researchStatus !== "COMPLETE").map((city) => ({ id: city.id, name: city.name, researchStatus: city.researchStatus, importStatus: city.importStatus, reason: city.notes }));
  const incompleteEvents = plan.events.filter((event) => event.researchStatus !== "COMPLETE").map((event) => ({ id: event.id, name: event.name, researchStatus: event.researchStatus, importStatus: event.importStatus, reason: event.notes }));
  const missingCovers = [...plan.cities, ...plan.events].filter((item) => item.cover.fallback).map((item) => ({ id: item.id, name: item.name, coverStatus: item.cover.status }));
  return {
    schemaVersion: "route-global-gaps-v1", generatedAt: plan.importedAt, authority: "Notion",
    totals: { cities: plan.cities.length, events: plan.events.length, regions: plan.regions.length, countries: plan.countries.length, sources: plan.sources.length, eventCityRelations: plan.locations.length },
    regionDistribution, research: { cities: research(plan.cities), events: research(plan.events) },
    incomplete: { cities: incompleteCities, events: incompleteEvents },
    covers: { catalogedEntities: plan.cities.length + plan.events.length, matched: [...plan.cities, ...plan.events].filter((item) => !item.cover.fallback).length, fallback: missingCovers.length, byStatus: coverManifestStatus(plan), missing: missingCovers },
    warnings: plan.warnings
  };
}

const inputPath = argument("--input", "docs/research/route-global-notion.snapshot.json");
const coversPath = argument("--covers", "docs/research/route-global-covers.json");
const reportPath = argument("--report-out", "docs/research/route-global-gaps.json");
const sqlPath = process.argv.includes("--sql-out") ? argument("--sql-out", "tmp/route-global-import.sql") : undefined;
const snapshot = readJson<RouteNotionSnapshot>(inputPath);
const covers = readJson<RouteCoverManifest>(coversPath);
const plan = buildRouteImportPlan(snapshot, {
  coverManifest: covers,
  legacyRegionIds: { europe: "region-europe", europa: "region-europe", africa: "region-africa" },
  legacyCountryIds: { france: "country-france", franca: "country-france", "south-africa": "country-south-africa", "africa-do-sul": "country-south-africa" },
  legacyCityIds: { paris: "city-paris" },
  legacyEventIds: { "paris-fashion-week": "event-paris-fashion-week" }
});
const output = report(plan);
mkdirSync(dirname(resolve(reportPath)), { recursive: true });
writeFileSync(resolve(reportPath), `${JSON.stringify(output, null, 2)}\n`, "utf8");
if (sqlPath) { mkdirSync(dirname(resolve(sqlPath)), { recursive: true }); writeFileSync(resolve(sqlPath), renderRouteImportSql(plan), "utf8"); }
console.log(JSON.stringify({ inputPath, reportPath, sqlPath: sqlPath ?? null, totals: output.totals, covers: output.covers, warnings: plan.warnings.length }, null, 2));
