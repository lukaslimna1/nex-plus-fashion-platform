# Cloudflare architecture

## Decision

NEX+ Fashion uses one deployment: the Web/PWA build is published as Workers Static Assets and the same Worker owns the `/api/*` surface. This avoids a second Worker and keeps the API base URL stable for Web, PWA and the future Desktop shell. `wrangler.jsonc` is the canonical configuration; Workers Sites is not used.

The Web workstream owns the React/Vite entrypoint and must add `@cloudflare/vite-plugin` to its Vite configuration when it implements the UI. This Codex delivery does not edit `apps/web/**` beyond the workspace package shell.

## Bindings

- `DB`: D1 database named `nex-plus-fashion`, with migrations in `migrations/`.
- `AI`: Workers AI secondary provider binding.
- `ASSETS`: implicit Workers Static Assets fetcher for `apps/web/dist`.

The D1 `database_id` is intentionally absent until Wrangler authentication and account selection are verified. Never put account credentials or API keys in `wrangler.jsonc`.

## Commands

```powershell
npm install
npm run dev
npm run db:migrate:local
npm run build
npm run deploy:preview
```

`npm run deploy` is prepared but must only be run after explicit production authorization. A public deployment was not performed in this delivery.

## Native caching

The API returns short HTTP cache headers and the Worker can later add `caches.default` around immutable GET responses. KV and R2 are deliberately not introduced: third-party fashion media remains remote and rights-aware.
