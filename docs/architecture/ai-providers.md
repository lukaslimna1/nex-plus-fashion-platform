# AI providers

The domain only sees `AIProvider` and task names. `GeminiAIProvider` uses the server-side `@google/genai` SDK; `CloudflareAIProvider` uses the Worker `AI` binding as a fallback. `AIProviderRouter` applies the task policy, tries the preferred provider, then the fallback, and never writes directly to canonical tables.

Deterministic parsing is preferred for names, seasons and source-change checks when it can solve the task. The prepared task contracts cover calendar/schedule, collection/maison/event metadata, normalization, translation, summaries, source changes, tags/terms/trend evidence and image/video metadata. The first controlled flows are `extractScheduleEntries` and `extractCollectionMetadata`.

Every AI result is wrapped with `provider`, `model`, `generatedAt`, `inputSourceIds`, `schemaVersion` and `status` (`AI_DRAFT`, `PENDING_REVIEW`, `VALIDATED` or `REJECTED`). It must pass schema validation and deterministic checks before review or canonical persistence.

Run the non-secret verification command with `npm run verify:ai`. Without `GEMINI_API_KEY` it reports `not_configured`; with a local secret it performs one small structured-output request and logs only provider, model, status, latency and schema success. Workers AI verification requires a live Worker binding and is not silently simulated.
