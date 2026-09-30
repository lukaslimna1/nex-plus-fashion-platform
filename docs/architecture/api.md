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
- `GET /api/collections/:slug` returns `{ data: CollectionDetail, meta }`, with `collection`, `maison`, optional `edition`/`event`/`city`, `schedule`, the compatibility `assets` list, grouped `imageGroups`, independent `videos`, `sources`, `reviews` and `tags`.
- `GET /api/maisons/:slug` returns `{ data: MaisonDetail, meta }`, with `maison`, `collections`, `assets` and `sources`.
- `GET /api/cities` returns `CityHub[]`; `GET /api/events` returns `Event[]`.
- `GET /api/schedule/now` and `/api/schedule/upcoming` return `ScheduleEntry[]` with computed `state`.
- `GET /api/assets` accepts optional `collection` (slug or id), `source`, `coverageType` and `mediaType=IMAGE|VIDEO` filters. The response remains `{ data: Asset[], meta }`.

The shared contracts also cover `Edition`, `Asset`, `Source`, `ProfessionalReview`, `Term` and `Tag`. `Asset` keeps `sequenceNumber`, `lookNumber`, `remoteUrl`, `alternativeUrls`, `thumbnailUrl`, `sourcePageUrl`, `provider`, `providerAssetId`, `sourceId/sourceIds`, credit fields, rights/download policy, media-kind flags and `coverageScope`. Video assets additionally expose `canonicalUrl`, `channelName`, `durationSeconds`, `publishedAt`, `videoType`, `completeness` and `officiality`. A reel or film uses a non-full `completeness` value unless a source proves a complete show. Third-party media is URL/embed-only by default; no endpoint implies permission to download or rehost.

The frontend-ready collection shape is:

```ts
type ImageGroup = { sourceId: string; coverageType: CoverageType; source?: Source; assets: Asset[] };
type CollectionDetail = {
  collection: Collection; maison: Maison; edition?: Edition; event?: Event; city?: CityHub;
  schedule: ScheduleEntry[]; assets: Asset[]; imageGroups: ImageGroup[]; videos: Asset[];
  sources: Source[]; reviews: ProfessionalReview[]; tags: Tag[];
};
```

`GET /api/home` keeps the named rails (`happeningNow`, `upcoming`, `recentCollections`, `latestPresentations`, `videos`, `maisons`, `reviews`, `trends`, `library`). `GET /api/cities`, `/api/events`, `/api/schedule/now` and `/api/schedule/upcoming` remain list envelopes for `CityHub`, `Event` and `ScheduleEntry` respectively; the latter two compute `state` at request time.
