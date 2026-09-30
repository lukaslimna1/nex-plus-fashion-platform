# Secrets and identity boundaries

Required values are listed by name only in `.env.example`: `GEMINI_API_KEY`, `CLOUDFLARE_ACCOUNT_ID`, optional `CLOUDFLARE_DATABASE_ID`, and environment metadata.

- Local Worker: put secrets in `.dev.vars`, which is ignored by Git; use `npx wrangler secret put GEMINI_API_KEY` for a remote environment.
- Production/preview: use Wrangler Secrets or the Cloudflare dashboard. Do not put secrets in `wrangler.jsonc`, a Vite public variable, the Web bundle, JSON fixtures or logs.
- `/api/health` exposes only booleans for provider configuration and D1 reachability.

Future identity is Google OIDC (`issuer` + private `subject`). Public reactions store only a server-derived HMAC pseudonymous identity key; a Google `sub` is never exposed or stored in community rows. Google Drive `appDataFolder` is represented by a future adapter contract for favorites, preferences, settings and reading state; OAuth consent and credentials are intentionally not implemented here.
