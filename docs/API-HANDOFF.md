# NEX+ Fashion API handoff

## Production

```text
API_BASE_URL=https://nex-plus-fashion.lucasmslima1.workers.dev
```

The production Worker and API use the remote D1 binding `DB`. JSON collection endpoints use `{ data, meta }`; `/api/health` returns its health object directly.

| Endpoint | Purpose | Main fields | Verified entity/state |
| --- | --- | --- | --- |
| `GET /api/health` | Runtime and dependency health | `status`, `environment`, `version`, `database`, `aiProviders` | `200`, production, remote DB reachable, Gemini configured |
| `GET /api/home` | Home rails | `data.rails.*`, `generatedAt`, `revision` | `200`, named rails including collections, videos and library |
| `GET /api/cities` | CityHub catalog | `data: CityHub[]`, `meta.count` | `200`, 136 records |
| `GET /api/events` | Event catalog | `data: Event[]`, `meta.count` | `200`, 59 records |
| `GET /api/schedule/now` | Events happening now | `data: ScheduleEntry[]` | `200`, current count depends on request time |
| `GET /api/schedule/upcoming` | Upcoming schedule | `data: ScheduleEntry[]` | `200`, current count depends on request time |
| `GET /api/collections` | Collection catalog | `data: Collection[]`, `meta.count` | `200`, 30 records |
| `GET /api/collections/:slug` | Collection detail and media | `collection`, `maison`, `edition`, `event`, `city`, `schedule`, `assets`, `imageGroups`, `videos`, `sources`, `mediaStatus` | Julie Kegels, Christian Dior and MAXHOSA AFRICA verified |
| `GET /api/maisons/:slug` | Maison detail | `maison`, `collections`, `assets`, `sources` | `200`, Julie Kegels, Christian Dior and MAXHOSA AFRICA verified |
| `GET /api/assets` | Media catalog | `data: Asset[]`; filters `collection`, `source`, `coverageType`, `mediaType` | `200`, 163 records |
| `GET /api/terms` | Existing term/library catalog | `data: Term[]` with `id`, `value`, `language`, `definition`, `sourceIds` | `200`, public read API exists |

Additional public routes are documented in `docs/architecture/api.md`: regions, countries, editions, maisons, sources, reviews, tags and search.

## Media fields

`Asset` exposes `remoteUrl`, `alternativeUrls`, `thumbnailUrl`, `sourcePageUrl`, `provider`, `providerAssetId`, creator/photographer/credit fields, rights/download policy and provenance IDs. Video assets additionally expose provider, embed/playback, coverage, completeness and officiality fields. The API does not imply download or rehosting permission.

## CORS

Production serves the Web/PWA through the same Worker origin, so `CORS_ALLOWED_ORIGINS` is intentionally empty and no wildcard is emitted. Local and preview configuration allow the Vite origins `localhost:5173`, `127.0.0.1:5173`, `localhost:4173` and `127.0.0.1:4173`. If the frontend is later deployed at a separate origin, add that exact origin to the production Wrangler variable and redeploy.

## Library gap audit

`Term` already has a schema, repository method and `GET /api/terms`. Search/category-specific term routes are not implemented and remain a future additive contract:

```text
GET /api/terms?search=
GET /api/terms?category=
GET /api/terms/:slug
```
