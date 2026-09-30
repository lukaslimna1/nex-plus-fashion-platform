# API contract

The Worker exposes GET-first JSON endpoints:

`/api/health`, `/api/regions`, `/api/countries`, `/api/cities`, `/api/events`, `/api/editions`, `/api/schedule`, `/api/maisons`, `/api/collections`, `/api/assets`, `/api/sources`, `/api/reviews`, `/api/terms`, `/api/tags` and `/api/search?q=`.

List responses use:

```json
{ "data": [], "meta": { "count": 0, "generatedAt": "...", "revision": 1 } }
```

`/api/schedule` adds a deterministic `state`: `UPCOMING`, `SOON`, `NOW`, `ENDED` or `UNKNOWN`. It never delegates temporal state to AI. The public surface has no anonymous write endpoint; future ingestion/admin writes must require an authenticated server-side boundary.
