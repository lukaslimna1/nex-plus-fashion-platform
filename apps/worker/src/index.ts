import { D1CatalogRepository } from "@nex-plus/data";
import { getScheduleState } from "@nex-plus/core";
import type { APIError, APIListResponse, HealthResponse, ScheduleEntry } from "@nex-plus/types";
import type { WorkerEnv } from "./env.js";

const jsonHeaders = { "content-type": "application/json; charset=utf-8", "cache-control": "public, max-age=30, s-maxage=60", "access-control-allow-origin": "*", "access-control-allow-methods": "GET, OPTIONS", "access-control-allow-headers": "content-type" };
const securityHeaders = { "x-content-type-options": "nosniff", "x-frame-options": "DENY", "referrer-policy": "strict-origin-when-cross-origin", "permissions-policy": "camera=(), microphone=(), geolocation=()", "content-security-policy": "default-src 'self'; frame-src https://www.youtube.com https://player.vimeo.com; img-src 'self' https: data:; connect-src 'self' https:; style-src 'self' 'unsafe-inline'" };

function json<T>(body: T, init: ResponseInit = {}): Response {
  return new Response(JSON.stringify(body), { ...init, headers: { ...jsonHeaders, ...securityHeaders, ...(init.headers ?? {}) } });
}
function error(code: string, message: string, status = 400): Response { return json<APIError>({ error: { code, message } }, { status }); }
function listResponse<T>(data: T[]): Response { return json<APIListResponse<T>>({ data, meta: { count: data.length, generatedAt: new Date().toISOString(), revision: 1 } }); }

async function health(env: WorkerEnv): Promise<Response> {
  let reachable = false;
  try { await env.DB.prepare("SELECT 1 AS ok").first(); reachable = true; } catch { /* health is intentionally non-sensitive */ }
  const result: HealthResponse = { status: reachable ? "ok" : "degraded", environment: env.APP_ENV ?? "local", version: env.WORKER_VERSION ?? "dev", ...(env.GIT_COMMIT ? { commit: env.GIT_COMMIT } : {}), database: { reachable }, aiProviders: { geminiConfigured: Boolean(env.GEMINI_API_KEY), workersAIConfigured: Boolean(env.AI) } };
  return json(result, { status: reachable ? 200 : 503 });
}

export default {
  async fetch(request: Request, env: WorkerEnv): Promise<Response> {
    if (request.method === "OPTIONS") return new Response(null, { status: 204, headers: { ...jsonHeaders, ...securityHeaders } });
    const url = new URL(request.url);
    if (url.pathname === "/api/health") return health(env);
    if (!url.pathname.startsWith("/api/")) return env.ASSETS ? env.ASSETS.fetch(request) : error("STATIC_ASSETS_UNAVAILABLE", "Static Assets binding is not available", 404);
    if (request.method !== "GET") return error("METHOD_NOT_ALLOWED", "Only GET endpoints are public", 405);
    const repository = new D1CatalogRepository(env.DB);
    try {
      switch (url.pathname) {
        case "/api/regions": return listResponse(await repository.listRegions());
        case "/api/countries": return listResponse(await repository.listCountries());
        case "/api/cities": return listResponse(await repository.listCities());
        case "/api/events": return listResponse(await repository.listEvents());
        case "/api/editions": return listResponse(await repository.listEditions());
        case "/api/schedule": {
          const from = url.searchParams.get("from");
          const to = url.searchParams.get("to");
          const entries = await repository.listSchedule({ ...(from ? { from } : {}), ...(to ? { to } : {}) });
          const now = new Date();
          const data = entries.map((entry) => ({ ...entry, state: getScheduleState(entry, now) }));
          return listResponse(data);
        }
        case "/api/maisons": return listResponse(await repository.listMaisons());
        case "/api/collections": return listResponse(await repository.listCollections());
        case "/api/assets": return listResponse(await repository.listAssets());
        case "/api/sources": return listResponse(await repository.listSources());
        case "/api/search": {
          const query = url.searchParams.get("q")?.trim() ?? "";
          if (query.length < 2) return error("INVALID_QUERY", "q must contain at least two characters");
          return listResponse(await repository.search(query));
        }
        case "/api/reviews": return listResponse(await repository.listReviews());
        case "/api/terms": return listResponse(await repository.listTerms());
        case "/api/tags": return listResponse(await repository.listTags());
        default: return error("NOT_FOUND", "API endpoint not found", 404);
      }
    } catch (cause) {
      return error("DATABASE_ERROR", cause instanceof Error ? cause.message : "Database request failed", 503);
    }
  }
};

export type { ScheduleEntry };
