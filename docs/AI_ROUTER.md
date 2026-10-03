# Curator AI Router

The User catalog remains local-first and does not require AI. Curator/Admin uses
the router as an operational capability for discovery, interpretation,
extraction, translation, matching, classification and proposal generation.

The router is provider-neutral at the Core boundary:

    Source/Search/Adapter
      -> AI Router (allowedCost = ZERO)
      -> free-tier or local provider adapter
      -> AIExecution
      -> Candidate
      -> Curator Proposal
      -> human review
      -> optional Official promotion

Proposal kinds, capabilities, and structured payloads remain open strings/data.
The initial capability constants are defaults, not a closed allow-list. New
proposal types can be stored and reviewed without changing the migration.

## Authority and cost policy

The only hard authority boundary is promotion: a provider response never writes
Official data. `APPROVE` or `EDIT_APPROVE` is required. The Core currently has
promotion handlers for reviewed source discovery and review summaries; other
proposal types remain auditable until an explicit promotion handler exists.

The active financial policy is `allowedCost = ZERO`. The Router only selects a
provider/model when the active configuration explicitly marks the model and
capability as eligible for zero-cost use. A paid model, a missing eligibility
declaration, a non-zero provider cost, or a Workers Paid response is rejected.
Quota exhaustion, rate limits, unavailable models, and policy rejection move to
the next compatible zero-cost provider, then local execution, then `DEGRADED`.

## Active providers

- `gemini`: only explicitly configured Gemini models/capabilities eligible for
  the current free tier.
- `groq`: only explicitly configured Groq Free Plan models/capabilities.
- `cloudflare_workers_ai`: only models explicitly listed as eligible for the
  Workers Free allocation; paid-only models are denied before execution.
- `local`: a real OpenAI-compatible `llama.cpp` server, with no API cost.

## Runtime configuration

The router reads only these environment variable names. Values are never
persisted or logged. Cloud providers require all three policy declarations for
automatic selection: `ALLOWED_COST=ZERO`, the exact zero-cost model list, and
the zero-cost capability list.

- `NEX_AI_GEMINI_API_KEY`, `NEX_AI_GEMINI_MODEL`
- `NEX_AI_GEMINI_ALLOWED_COST`, `NEX_AI_GEMINI_ZERO_COST_MODELS`,
  `NEX_AI_GEMINI_ZERO_COST_CAPABILITIES`
- `NEX_AI_GROQ_API_KEY`, `NEX_AI_GROQ_MODEL`
- `NEX_AI_GROQ_ALLOWED_COST`, `NEX_AI_GROQ_ZERO_COST_MODELS`,
  `NEX_AI_GROQ_ZERO_COST_CAPABILITIES`
- `NEX_AI_CLOUDFLARE_WORKERS_AI_ACCOUNT_ID`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_API_TOKEN`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_MODEL`
- `NEX_AI_CLOUDFLARE_WORKERS_AI_ALLOWED_COST`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_ZERO_COST_MODELS`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_ZERO_COST_CAPABILITIES`
- `NEX_AI_LOCAL_ENDPOINT`, `NEX_AI_LOCAL_MODEL`, `NEX_AI_LOCAL_API_KEY`

The local adapter defaults to the `llama.cpp` OpenAI-compatible API shape at
`http://127.0.0.1:8080/v1/chat/completions`, but is selected only when an
endpoint is configured. Without a configured provider, Curator reports
`DEGRADED` and AI executions return `AI_NOT_CONFIGURED`. The User catalog,
Milano Pack, search and Personal remain available.

Runtime output is validated in Core because provider-side schema guarantees do
not establish editorial truth.
