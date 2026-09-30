# Ingestion and provenance

The ingestion path is layered:

1. official API/JSON/RSS;
2. HTTP + Cheerio;
3. Crawlee only when source breadth requires it;
4. Playwright only when HTML cannot expose the source data;
5. Gemini extraction only when deterministic parsing is insufficient;
6. Zod/schema validation;
7. provenance and source-change metadata;
8. `PENDING_REVIEW` candidate;
9. human validation before canonical publication.

`scripts/scrape-fhcm.ts` is the first deterministic scraper. It reads the official FHCM calendar, preserves the Paris civil date/time and emits an ISO instant. The Julie Kegels vertical slice records only verified official data. Media is inline-first but third-party URLs remain remote, attributed and rights-specific; no third-party file is copied to the repository.

`source_checks` stores `etag`, `last_modified`, `content_hash`, `last_checked_at` and `last_changed_at`. An unchanged source stops before extraction; a changed source creates a candidate/diff for validation.

Source-specific capabilities live in `packages/media/src/adapters.ts` (`fhcm`, `vogue-runway`, `youtube` and `website-gallery`) rather than in one scraper. `scripts/pfw-media.ts` keeps the deterministic PFW SS27 first-three-days schedule snapshot and the Vogue Runway URL/sequence deduplicator. `scripts/generate-pfw-ss27-seed.ts` fetches public HTML only and writes versioned URL/provenance seed data; it never downloads media. The first scan is recorded in `docs/research/pfw-ss27-first-three-days.json`, while incomplete shows remain in `media_research_jobs` for later source-specific retries.
