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
| `GET /api/cities/:slug` | City detail | `city`, `country`, `region`, `events`, `editions`, `schedule`, `collections`, `assets`, `sources` | Detail relationship contract |
| `GET /api/events` | Event catalog | `data: Event[]`, `meta.count` | `200`, 59 records |
| `GET /api/events/:slug` | Event detail | `event`, `city`, `cities`, `editions`, `segments`, `schedule`, `collections`, `maisons`, `assets`, `sources` | Detail relationship contract |
| `GET /api/schedule/now` | Events happening now | `data: ScheduleEntry[]` | `200`, current count depends on request time |
| `GET /api/schedule/upcoming` | Upcoming schedule | `data: ScheduleEntry[]` | `200`, current count depends on request time |
| `GET /api/collections` | Collection catalog | `data: Collection[]`, `meta.count` | `200`, 30 records |
| `GET /api/collections/:slug` | Collection detail and media | `collection`, `maison`, `edition`, `event`, `city`, `schedule`, `assets`, `imageGroups`, `videos`, `sources`, `mediaStatus` | Julie Kegels, Christian Dior and MAXHOSA AFRICA verified |
| `GET /api/maisons/:slug` | Maison detail | `maison`, `creativeDirectorHistory`, `events`, `collections`, `assets`, `media`, `reviews`, `sources` | Independent Maison profile; Collection data is related, not substituted |
| `GET /api/assets` | Media catalog | `data: Asset[]`; filters `collection`, `source`, `coverageType`, `mediaType` | `200`, 163 records |
| `GET /api/terms` | Term/library catalog | `data: Term[]`; optional `search` and `category` | `200`, public read API exists |
| `GET /api/terms/:slug` | Term detail | `term`, `sources`, `relatedTerms`, `collections` | Detail relationship contract |
| `GET /api/trends` | Evidence-backed trend catalog | `Trend[]` with `scope`, `evidenceCount`, `status`, `provenance`, `sourceIds` | Model suggestions remain `PENDING_REVIEW` |
| `GET /api/trends/:slug` | Trend detail | `trend`, `collections`, `looks`, `sources`, `relatedTerms` | No general claim without scoped evidence |
| `GET /api/auth/session` | Google Identity session | `authenticated`, `provider`, optional profile/login URL | Google OIDC only; no password system |
| `GET /api/auth/login` | OIDC handoff status | error envelope until callback is configured | Credentials are never returned |
| `GET /api/auth/logout` | Session contract | unauthenticated `AuthSession` | Frontend clears its session after identity logout |
| `GET /api/favorites` | Personal favorites | `401 AUTH_REQUIRED` until an OIDC subject exists | Collection/Maison/City/Event/Term/Asset targets |
| `GET /api/preferences` | Personal preferences | `401 AUTH_REQUIRED` until an OIDC subject exists | language, timezone, autoplayPreview, reducedMotion, mediaPreference, editorialPreferences |
| `GET /api/sync/personal` | Personal sync contract | `GOOGLE_DRIVE_APP_DATA` scopes | Public catalog is explicitly excluded |

Additional public routes are documented in `docs/architecture/api.md`: regions, countries, editions, maisons, sources, reviews, tags and search.

## Media fields

`Asset` exposes `remoteUrl`, `alternativeUrls`, `thumbnailUrl`, `sourcePageUrl`, `provider`, `providerAssetId`, creator/photographer/credit fields, rights/download policy and provenance IDs. Video assets additionally expose provider, embed/playback, coverage, completeness and officiality fields. The API does not imply download or rehosting permission.

## CORS

Production serves the Web/PWA through the same Worker origin, so `CORS_ALLOWED_ORIGINS` is intentionally empty and no wildcard is emitted. Local and preview configuration allow the Vite origins `localhost:5173`, `127.0.0.1:5173`, `localhost:4173` and `127.0.0.1:4173`. If the frontend is later deployed at a separate origin, add that exact origin to the production Wrangler variable and redeploy.

## Library gap audit

`Term` is served by the repository and supports additive search/category/detail contracts:

```text
GET /api/terms?search=
GET /api/terms?category=
GET /api/terms/:slug
```

Term fields preserve `canonicalName`, `ptBrName`, `internationalName`,
`aliases`, `definition`, `context`, `category`, `externalUri`, `sourceIds`,
`relatedTermIds`, `examples` and `retrievedAt` when a verified source supplies
them. Schedule entries expose `sourcePageUrl` as the source-page alias of
`officialUrl`, plus optional `venueName` and `verifiedAt`.

The current remote Library contains 8 Terms (7 Getty AAT-sourced, including a
verified batch of jacquard, tweed, organza, chiffon and denim). Missing
Portuguese translations, definitions and aliases remain explicit gaps queued
for AI draft plus human review; they are not fabricated in seed data.

Detail routes use the same `{ data, meta }` envelope. The Worker resolves
Country/Region, City/Event, Edition, Schedule, Collection, Maison, Asset and
Source relationships so the frontend does not reconstruct them from labels.

## Canonical frontend shapes

`Maison` is an independent entity: `id`, `name`, `slug`, `websiteUrl`,
`logoUrl`, `foundedYear`, `foundedBy`, `country`, `city`, `headquarters`,
`currentCreativeDirector`, `artisticDirection`, `about`, `history`, `socials`,
`otherOfficialLinks`, `researchStatus`, `verifiedAt` and `officialSourceIds`.

`SearchResult` always includes `type`, `id`, `slug`, `title`, optional
`subtitle`/`thumbnail`, and `routeTarget`; it covers City, Event, Edition,
Collection, Maison, Term and Trend. `Trend` includes a JSON `scope`,
`evidenceCount`, `status`, `provenance` and `sourceIds`.

Schedule responses expose ISO timestamps and the calculated `state` (`NOW`,
`UPCOMING`, `ENDED` or `UNKNOWN`). `NOW` is emitted only when the entry has a
verified bounded interval containing the current instant; an open-ended entry
cannot be asserted as currently running.

The Worker runs a fifteen-minute scheduled AI batch. Jobs carry structured
entity context and an output schema, write to `ai_enrichment_jobs` and
`ai_runs`, and stop at `PENDING_REVIEW`; they never publish directly to
Maison, Collection or Term rows. Gemini is preferred, Workers AI is a limited
fallback for lightweight tasks, and unavailable providers are recorded as
`BLOCKED`.

Third-party media remains URL/embed/provenance metadata. Removed media is
retained for audit but omitted from playable collection `videos` and media
status. The Julie Kegels NSS Instagram reel is therefore `REMOVED`/`LINK_ONLY`
and must not be rendered as a player. The Dior Vogue item remains a
partial-coverage `LINK_ONLY` record until reproducible playback is verified.
