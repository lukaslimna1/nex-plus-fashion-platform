# Source Adapter Framework

## Boundary

The Curator integration boundary is:

    Source
      -> Adapter Registry
      -> discovery
      -> raw artifact
      -> observation
      -> normalization
      -> ingestion candidate
      -> ingestion proposal
      -> human review
      -> optional Official promotion

An adapter is an acquisition strategy, not the Source itself. A Source can
have several integrations and adapters. Adapter capabilities are strings and
are not a closed enum, so a new capability can be registered without changing
the storage model.

## Contract

AdapterSpec declares:

- adapter and version identifiers;
- related Source and Integration identifiers;
- extensible capabilities;
- discovery, fetch, parse and normalization strategies;
- supported content types;
- pagination/cursor/checkpoint behavior;
- rate-limit and retry policy;
- provenance support;
- produced entity kinds;
- fixture and test support.

Adapter health is separate from editorial Source status. The supported states
are HEALTHY, DEGRADED, RATE_LIMITED, AUTH_REQUIRED, BLOCKED, CHANGED,
UNAVAILABLE and NOT_CONFIGURED.

## Persistence

Migration 0007_adapter_framework adds:

- adapter_registry and integration_registry;
- source_health;
- raw_artifact and source_observation;
- adapter_run and ingestion_checkpoint;
- ingestion_candidate and ingestion_proposal;
- source_candidate, adapter_candidate and integration_proposal.

Raw artifacts retain content hash, canonical URL, media type, retrieval time
and either inline content or an external content reference. Repeated
acquisition of the same adapter/URL/hash is an upsert. Candidates use stable
adapter keys and proposals remain pending until human review.

Unknown Sources are stored as SourceCandidate. Their evidence, observed
capabilities and provenance remain available for an AdapterCandidate and an
IntegrationProposal; nothing is promoted to source_registry automatically.

## Proof adapters

### CNMI Milano calendar

adapter:cnmi:milano-calendar is the official CNMI proof adapter. It discovers
the official Milano calendar URL, acquires the existing verified SS27 calendar
payload through the fixture-backed transport seam, parses it deterministically
and emits 214 schedule_entry_observation candidates.

Each candidate preserves the CNMI URL and Source/adapter provenance and is
compared with the already materialized schedule by stable ID and source hash.
The adapter never writes schedule_entry or other Official rows.

### Business of Fashion reviews

adapter:bof:fashion-week-reviews proves a non-official professional Source
path. It uses the documented Fashion Week Reviews URL as a small metadata-only
fixture. Subscription-limited article content is not copied; the candidate
retains the original URL, title, language, access state, rights note and
provenance. The adapter is intentionally DEGRADED until a permitted live
acquisition path is configured.

## IPC

The Core exposes adapter registry/health, adapter runs, SourceCandidates,
AdapterCandidates and IntegrationProposals through typed Tauri commands. The
User catalog contracts remain separate; ingestion details are Curator-facing.

AI may propose an unknown Source or acquisition strategy, but only the
candidate/proposal path can carry it forward. Deterministic JSON is parsed
without an LLM; the AI Router remains an optional assist for ambiguous
interpretation, matching and enrichment.
