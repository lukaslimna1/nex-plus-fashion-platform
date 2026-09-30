# API contract

The Worker exposes GET-first JSON endpoints:

`/api/health`, `/api/regions`, `/api/countries`, `/api/cities`, `/api/events`, `/api/editions`, `/api/schedule`, `/api/schedule/now`, `/api/schedule/upcoming`, `/api/home`, `/api/maisons`, `/api/maisons/:slug`, `/api/collections`, `/api/collections/:slug`, `/api/assets`, `/api/sources`, `/api/reviews`, `/api/terms`, `/api/tags` and `/api/search?q=`.

List responses use:

```json
{ "data": [], "meta": { "count": 0, "generatedAt": "...", "revision": 1 } }
```

`/api/schedule` adds a deterministic `state`: `UPCOMING`, `SOON`, `NOW`, `ENDED` or `UNKNOWN`. It never delegates temporal state to AI. The public surface has no anonymous write endpoint; future ingestion/admin writes must require an authenticated server-side boundary.

## Frontend-ready shapes

All fields below are typed in `@nex-plus/types`. The collection, maison, event and schedule records preserve IDs and source IDs so the UI never has to infer provenance from labels.

- `GET /api/home` returns `{ data: HomeResponse, meta }`. `data.rails` contains `happeningNow`, `upcoming`, `recentCollections`, `latestPresentations`, `videos`, `maisons`, `reviews`, `trends` and `library`; each rail is `{ key, title, data }`.
- `GET /api/collections/:slug` returns `{ data: CollectionDetail, meta }`, with `collection`, `maison`, optional `edition`/`event`/`city`, `schedule`, `assets`, `sources`, `reviews` and `tags`.
- `GET /api/maisons/:slug` returns `{ data: MaisonDetail, meta }`, with `maison`, `collections`, `assets` and `sources`.
- `GET /api/cities` returns `CityHub[]`; `GET /api/events` returns `Event[]`.
- `GET /api/schedule/now` and `/api/schedule/upcoming` return `ScheduleEntry[]` with computed `state`.

The shared contracts also cover `Edition`, `Asset`, `Source`, `ProfessionalReview`, `Term` and `Tag`. `Asset` keeps `sequenceNumber`, `remoteUrl`, `alternativeUrls`, `thumbnailUrl`, `sourcePageUrl`, `provider`, `providerAssetId`, credit fields, rights/download policy, media-kind flags and `coverageScope`. A reel uses `RUNWAY_COVERAGE_REEL`; it must not be presented as a complete show unless a source proves that scope. Third-party media is URL/embed-only by default; no endpoint implies permission to download or rehost.
