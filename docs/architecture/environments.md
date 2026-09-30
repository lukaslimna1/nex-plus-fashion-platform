# Environments

The same architecture is used in three environments:

- `LOCAL`: Wrangler local Worker, local D1, `.dev.vars` for local secrets.
- `PREVIEW`: named preview Worker/D1 resources, configured by `wrangler deploy --env preview` after account setup.
- `PRODUCTION`: the canonical Worker/D1 resources, currently published at the API base URL in `docs/API-HANDOFF.md` after explicit operator authorization.

The frontend reads its API base URL from environment-specific build configuration; it must not embed provider secrets. Public GET contracts remain the same in each environment.
