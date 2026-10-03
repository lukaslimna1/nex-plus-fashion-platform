# Source adapter framework · 2.0

## Boundary

The Rust Core owns jobs, state, SQLite and the only path to catalog data:

```text
Source
  -> acquisition strategy
  -> RawArtifact
  -> deterministic parser
  -> Observation
  -> normalizer
  -> Candidate
  -> Proposal
  -> Human Review
  -> explicit Official promotion
```

An adapter is an acquisition/normalization strategy, not the Source itself. A
Source may expose several strategies: API, JSON/JSON-LD, RSS/feed, static HTML,
media endpoint, sitemap, channel feed or browser automation. The registry
selects the lowest-cost deterministic strategy available; browser is a
capability boundary, never the default parser.

The current implementation stays Rust-only. No Cheerio, Crawlee, Playwright or
parallel Node backend was added. A future browser worker, if justified, must
communicate with the Rust Core through a structured, cancellable protocol and
must not write SQLite or Official records.

## Progressive acquisition

The intended escalation is:

```text
official API
  -> JSON / JSON-LD / RSS
  -> static HTML + deterministic parser
  -> bounded queue/crawler
  -> browser capability only when JS/interaction is required
  -> AI Router only when semantic interpretation adds value
  -> schema validation
  -> evidence/provenance
  -> Candidate / Proposal
  -> Human Review
```

The current Rust transport implements redirects, identifiable User-Agent,
MIME validation, maximum response bytes, timeout, conditional ETag and
Last-Modified requests, 304 handling, retry/backoff and per-source/host rate
limiting. The HTML proof uses deterministic metadata/JSON-LD extraction; it
does not call AI.

## RawArtifact

`RawArtifact` keeps the original URL, final URL, Source, adapter, acquisition
method, HTTP status, MIME type, fetched time, ETag, Last-Modified, content hash,
body reference, parent/referrer and pagination metadata. Bodies are stored once
in `raw_artifact_body` by SHA-256; RawArtifact rows reference the body hash.

Repeated acquisition of the same adapter/URL/content hash is idempotent. A
304 updates retrieval state and validators without downloading or reparsing the
body.

## Crawling limits

Every run has bounded `maxPages`, `maxDepth`, `maxItems`, `maxBytes` and
`maxDurationMs`. URL fragments are removed for canonicalization; queue entries
are deduplicated and checked against adapter domain scope. Pagination/cursors
and the last processed artifact hash are checkpointed after each item. A
`cancelKey` can cancel a running crawl through the Tauri Core command.

## AI Router boundary

Adapters never select Gemini, Groq, Cloudflare Workers AI, Mistral, Hugging
Face or llama.cpp. An adapter may optionally request a capability such as
`STRUCTURED_EXTRACTION`, `ENTITY_MATCHING` or `TRANSLATION_PT_BR`; the Core
passes that request to the canonical `AiRouter`. The Router owns provider
selection, ZERO-cost policy, schema validation, fallback and AI
Candidate/Proposal persistence. Deterministic paths leave AI unused.

## Proof adapters

### CNMI Milano calendar

The official CNMI adapter is the API/JSON proof. Its fixture represents the
verified Milano SS27 calendar, parses deterministically and emits 214 schedule
candidates. It records API acquisition metadata, evidence and provenance, but
never writes or changes Official schedule rows.

### Business of Fashion reviews

The BoF adapter is the second professional Source proof. It uses a static HTML
fixture, extracts public title/locale/JSON-LD metadata deterministically, keeps
the original URL and rights/access note, and emits a metadata-only review
candidate. Subscription-limited article content is not copied. Browser remains
an explicit fallback strategy and is not executed by this proof.

## Safety invariant

No acquisition adapter writes Official catalog rows. Unknown Sources remain
`SourceCandidate` records; acquisition results remain Candidate/Proposal data
until a human review path explicitly approves them.
