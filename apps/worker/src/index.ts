import { D1CatalogRepository } from "@nex-plus/data";
import type { AssetFilters, CityFilters, EventFilters } from "@nex-plus/data";
import { getHappeningNow, getScheduleState, getUpcoming } from "@nex-plus/core";
import type { APIError, APIListResponse, APISingleResponse, AuthSession, CityDetail, CollectionDetail, EventDetail, HealthResponse, HomeResponse, MaisonDetail, PersonalSyncContract, ScheduleEntry, TermDetail, TrendDetail } from "@nex-plus/types";
import type { WorkerEnv } from "./env.js";
import { GeminiAIProvider } from "./ai/gemini.js";
import { CloudflareAIProvider } from "./ai/cloudflare.js";
import { AIProviderRouter } from "./ai/router.js";
import { runAIEnrichmentBatch } from "./ai/jobs.js";

const jsonHeaders = { "content-type": "application/json; charset=utf-8", "cache-control": "public, max-age=30, s-maxage=60", "access-control-allow-methods": "GET, OPTIONS", "access-control-allow-headers": "content-type" };
const securityHeaders = { "x-content-type-options": "nosniff", "x-frame-options": "DENY", "referrer-policy": "strict-origin-when-cross-origin", "permissions-policy": "camera=(), microphone=(), geolocation=()", "content-security-policy": "default-src 'self'; frame-src https://www.youtube.com https://player.vimeo.com; img-src 'self' https: data:; connect-src 'self' https:; style-src 'self' 'unsafe-inline'" };

const localDevelopmentOrigins = new Set(["http://localhost:5173", "http://127.0.0.1:5173", "http://localhost:4173", "http://127.0.0.1:4173"]);

function isAllowedOrigin(origin: string, env: WorkerEnv): boolean {
  const configuredOrigins = (env.CORS_ALLOWED_ORIGINS ?? "").split(",").map((value) => value.trim()).filter(Boolean);
  return configuredOrigins.includes(origin) || (env.APP_ENV !== "production" && localDevelopmentOrigins.has(origin));
}

function withCors(response: Response, request: Request, env: WorkerEnv): Response {
  const headers = new Headers(response.headers);
  const origin = request.headers.get("Origin");
  if (origin && isAllowedOrigin(origin, env)) {
    headers.set("access-control-allow-origin", origin);
    headers.set("vary", headers.get("vary") ? `${headers.get("vary")}, Origin` : "Origin");
  } else {
    headers.delete("access-control-allow-origin");
  }
  return new Response(response.body, { status: response.status, statusText: response.statusText, headers });
}

function json<T>(body: T, init: ResponseInit = {}): Response {
  return new Response(JSON.stringify(body), { ...init, headers: { ...jsonHeaders, ...securityHeaders, ...(init.headers ?? {}) } });
}
function error(code: string, message: string, status = 400): Response { return json<APIError>({ error: { code, message } }, { status }); }
function listResponse<T>(data: T[]): Response { return json<APIListResponse<T>>({ data, meta: { count: data.length, generatedAt: new Date().toISOString(), revision: 1 } }); }
function singleResponse<T>(data: T): Response { return json<APISingleResponse<T>>({ data, meta: { generatedAt: new Date().toISOString(), revision: 1 } }); }

function homeResponse(input: Awaited<ReturnType<typeof buildHome>>): HomeResponse {
  const { schedule, collections, assets, maisons, reviews, trends, terms } = input;
  const now = new Date();
  const upcoming = getUpcoming(schedule, now);
  const rail = <T>(key: string, title: string, data: T[]) => ({ key, title, data });
  return {
    rails: {
      happeningNow: rail("happening-now", "Acontecendo agora", getHappeningNow(schedule, now)),
      upcoming: rail("upcoming", "Próximos eventos", upcoming),
      recentCollections: rail("recent-collections", "Collections recentes", collections.slice(0, 10)),
      latestPresentations: rail("latest-presentations", "Apresentações recentes", upcoming.filter((entry) => entry.format === "PRESENTATION").slice(0, 10)),
      videos: rail("videos", "Vídeos", assets.filter((asset) => asset.assetKind === "VIDEO" || Boolean(asset.embedUrl)).slice(0, 10)),
      maisons: rail("maisons", "Maisons", maisons.slice(0, 10)),
      reviews: rail("reviews", "Leituras profissionais", reviews.slice(0, 10)),
      trends: rail("trends", "Tendências com evidência", trends.slice(0, 10)),
      library: rail("library", "Biblioteca", terms.slice(0, 10))
    },
    generatedAt: new Date().toISOString(),
    revision: 1
  };
}

async function buildHome(repository: D1CatalogRepository) {
  const [schedule, collections, assets, maisons, reviews, trends, terms] = await Promise.all([
    repository.listSchedule(), repository.listCollections(), repository.listAssets(), repository.listMaisons(), repository.listReviews(), repository.listTrends(), repository.listTerms()
  ]);
  return { schedule, collections, assets, maisons, reviews, trends, terms };
}

function aiRouter(env: WorkerEnv): AIProviderRouter {
  return new AIProviderRouter({
    ...(env.GEMINI_API_KEY ? { gemini: new GeminiAIProvider(env.GEMINI_API_KEY, env.GEMINI_MODEL) } : {}),
    ...(env.AI ? { workersAI: new CloudflareAIProvider(env.AI) } : {})
  });
}

async function health(env: WorkerEnv): Promise<Response> {
  let reachable = false;
  try { await env.DB.prepare("SELECT 1 AS ok").first(); reachable = true; } catch { /* health is intentionally non-sensitive */ }
  const result: HealthResponse = { status: reachable ? "ok" : "degraded", environment: env.APP_ENV ?? "local", version: env.WORKER_VERSION ?? "dev", ...(env.GIT_COMMIT ? { commit: env.GIT_COMMIT } : {}), database: { reachable }, aiProviders: { geminiConfigured: Boolean(env.GEMINI_API_KEY), workersAIConfigured: Boolean(env.AI) } };
  return json(result, { status: reachable ? 200 : 503 });
}

export default {
  async fetch(request: Request, env: WorkerEnv): Promise<Response> {
    const respond = (response: Response) => withCors(response, request, env);
    if (request.method === "OPTIONS") return respond(new Response(null, { status: 204, headers: { ...jsonHeaders, ...securityHeaders } }));
    const url = new URL(request.url);
    if (url.pathname === "/api/health") return respond(await health(env));
    if (!url.pathname.startsWith("/api/")) return env.ASSETS ? env.ASSETS.fetch(request) : respond(error("STATIC_ASSETS_UNAVAILABLE", "Static Assets binding is not available", 404));
    if (request.method !== "GET") return respond(error("METHOD_NOT_ALLOWED", "Only GET endpoints are public", 405));
    const repository = new D1CatalogRepository(env.DB);
    try {
      const collectionMatch = url.pathname.match(/^\/api\/collections\/([^/]+)$/);
      if (collectionMatch) {
        const data = await repository.getCollectionBySlug(decodeURIComponent(collectionMatch[1]!));
        return respond(data ? singleResponse<CollectionDetail>(data) : error("NOT_FOUND", "Collection not found", 404));
      }
      const maisonMatch = url.pathname.match(/^\/api\/maisons\/([^/]+)$/);
      if (maisonMatch) {
        const data = await repository.getMaisonBySlug(decodeURIComponent(maisonMatch[1]!));
        return respond(data ? singleResponse<MaisonDetail>(data) : error("NOT_FOUND", "Maison not found", 404));
      }
      const cityMatch = url.pathname.match(/^\/api\/cities\/([^/]+)$/);
      if (cityMatch) {
        const data = await repository.getCityBySlug(decodeURIComponent(cityMatch[1]!));
        return respond(data ? singleResponse<CityDetail>(data) : error("NOT_FOUND", "City not found", 404));
      }
      const eventMatch = url.pathname.match(/^\/api\/events\/([^/]+)$/);
      if (eventMatch) {
        const data = await repository.getEventBySlug(decodeURIComponent(eventMatch[1]!));
        return respond(data ? singleResponse<EventDetail>(data) : error("NOT_FOUND", "Event not found", 404));
      }
      const termMatch = url.pathname.match(/^\/api\/terms\/([^/]+)$/);
      if (termMatch) {
        const data = await repository.getTermBySlug(decodeURIComponent(termMatch[1]!));
        return respond(data ? singleResponse<TermDetail>(data) : error("NOT_FOUND", "Term not found", 404));
      }
      const trendMatch = url.pathname.match(/^\/api\/trends\/([^/]+)$/);
      if (trendMatch) {
        const data = await repository.getTrendBySlug(decodeURIComponent(trendMatch[1]!));
        return respond(data ? singleResponse<TrendDetail>(data) : error("NOT_FOUND", "Trend not found", 404));
      }
      switch (url.pathname) {
        case "/api/home": return respond(singleResponse<HomeResponse>(homeResponse(await buildHome(repository))));
        case "/api/regions": return respond(listResponse(await repository.listRegions()));
        case "/api/countries": return respond(listResponse(await repository.listCountries()));
        case "/api/cities": {
          const hasCover = url.searchParams.get("hasCover");
          const params: CityFilters = {
            ...(url.searchParams.get("region") ? { region: url.searchParams.get("region")! } : {}),
            ...(url.searchParams.get("country") ? { country: url.searchParams.get("country")! } : {}),
            ...(url.searchParams.get("status") ? { status: url.searchParams.get("status")! } : {}),
            ...(hasCover === "true" || hasCover === "1" ? { hasCover: true } : hasCover === "false" || hasCover === "0" ? { hasCover: false } : {})
          };
          return respond(listResponse(await repository.listCities(params)));
        }
        case "/api/events": {
          const hasCover = url.searchParams.get("hasCover");
          const params: EventFilters = {
            ...(url.searchParams.get("status") ? { status: url.searchParams.get("status")! } : {}),
            ...(url.searchParams.get("type") ? { type: url.searchParams.get("type")! } : {}),
            ...(url.searchParams.get("city") ? { city: url.searchParams.get("city")! } : {}),
            ...(hasCover === "true" || hasCover === "1" ? { hasCover: true } : hasCover === "false" || hasCover === "0" ? { hasCover: false } : {})
          };
          return respond(listResponse(await repository.listEvents(params)));
        }
        case "/api/editions": return respond(listResponse(await repository.listEditions()));
        case "/api/schedule": {
          const from = url.searchParams.get("from");
          const to = url.searchParams.get("to");
          const entries = await repository.listSchedule({ ...(from ? { from } : {}), ...(to ? { to } : {}) });
          const now = new Date();
          const data = entries.map((entry) => ({ ...entry, state: getScheduleState(entry, now) }));
          return respond(listResponse(data));
        }
        case "/api/schedule/now": {
          const entries = await repository.listSchedule();
          return respond(listResponse(getHappeningNow(entries, new Date())));
        }
        case "/api/schedule/upcoming": {
          const entries = await repository.listSchedule();
          return respond(listResponse(getUpcoming(entries, new Date())));
        }
        case "/api/maisons": return respond(listResponse(await repository.listMaisons()));
        case "/api/collections": return respond(listResponse(await repository.listCollections()));
        case "/api/assets": {
          const collection = url.searchParams.get("collection") ?? undefined;
          const source = url.searchParams.get("source") ?? undefined;
          const coverageType = url.searchParams.get("coverageType") as AssetFilters["coverageType"] | undefined;
          const mediaType = url.searchParams.get("mediaType") as AssetFilters["mediaType"] | undefined;
          return respond(listResponse(await repository.listAssets({ ...(collection ? { collection } : {}), ...(source ? { source } : {}), ...(coverageType ? { coverageType } : {}), ...(mediaType ? { mediaType } : {}) })));
        }
        case "/api/sources": return respond(listResponse(await repository.listSources()));
        case "/api/search": {
          const query = url.searchParams.get("q")?.trim() ?? "";
          if (query.length < 2) return respond(error("INVALID_QUERY", "q must contain at least two characters"));
          return respond(listResponse(await repository.search(query)));
        }
        case "/api/reviews": return respond(listResponse(await repository.listReviews()));
        case "/api/trends": return respond(listResponse(await repository.listTrends()));
        case "/api/terms": {
          const search = url.searchParams.get("search")?.trim() || undefined;
          const category = url.searchParams.get("category")?.trim() || undefined;
          return respond(listResponse(await repository.listTerms({ ...(search ? { search } : {}), ...(category ? { category } : {}) })));
        }
        case "/api/auth/session": {
          const session: AuthSession = { authenticated: false, provider: "GOOGLE_OIDC", ...(env.GOOGLE_CLIENT_ID ? { loginUrl: "/api/auth/login" } : {}) };
          return respond(singleResponse(session));
        }
        case "/api/auth/logout": return respond(singleResponse<AuthSession>({ authenticated: false, provider: "GOOGLE_OIDC" }));
        case "/api/auth/login": return respond(env.GOOGLE_CLIENT_ID && env.GOOGLE_REDIRECT_URI ? json({ error: { code: "OIDC_HANDOFF_REQUIRED", message: "Google OIDC authorization handoff is configured but must be completed by the frontend callback." } }, { status: 501 }) : error("AUTH_NOT_CONFIGURED", "Google Identity credentials are not configured", 503));
        case "/api/favorites": return respond(error("AUTH_REQUIRED", "Google Identity is required for personal favorites", 401));
        case "/api/preferences": return respond(error("AUTH_REQUIRED", "Google Identity is required for personal preferences", 401));
        case "/api/sync/personal": {
          const sync: PersonalSyncContract = { provider: "GOOGLE_DRIVE_APP_DATA", scopes: ["favorites", "preferences", "settings", "reading-state"], publicCatalogExcluded: true, consentRequired: true };
          return respond(singleResponse(sync));
        }
        case "/api/tags": return respond(listResponse(await repository.listTags()));
        default: return respond(error("NOT_FOUND", "API endpoint not found", 404));
      }
    } catch (cause) {
      return respond(error("DATABASE_ERROR", cause instanceof Error ? cause.message : "Database request failed", 503));
    }
  },
  async scheduled(_controller: ScheduledController, env: WorkerEnv, ctx: ExecutionContext): Promise<void> {
    ctx.waitUntil(runAIEnrichmentBatch(env.DB, aiRouter(env), 3));
  }
};

export type { ScheduleEntry };
