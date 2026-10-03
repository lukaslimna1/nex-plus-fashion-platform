# Curator AI Router

The User catalog remains local-first and does not require AI. Curator/Admin uses
the Router as an operational capability for discovery, interpretation,
extraction, translation, matching, classification and proposal generation.

The provider matrix is exactly five online providers plus one local provider:

1. Gemini
2. Groq
3. Cloudflare Workers AI
4. Mistral
5. Hugging Face Inference Providers
6. Local `llama.cpp`

The Core boundary remains provider-neutral:

    Source/Search/Adapter
      -> AI Router (allowedCost = ZERO)
      -> zero-cost-eligible provider adapter
      -> AIExecution
      -> Candidate
      -> Curator Proposal
      -> human review
      -> optional Official promotion

Proposal kinds, capabilities and structured payloads remain open strings/data.
The initial capability constants are defaults, not a closed allow-list.

## Authority and cost policy

Provider output never writes Official data. `APPROVE` or `EDIT_APPROVE` is
required. Source discovery and review-summary promotion are implemented; other
proposal types remain auditable until an explicit promotion handler exists.

Every concrete provider/model/capability route must be classified as:

- `ZERO_COST_ELIGIBLE`: may receive a call;
- `ZERO_COST_INELIGIBLE`: never selected;
- `ZERO_COST_UNKNOWN`: fail closed and never called.

The active financial policy is `allowedCost = ZERO`. Quota exhaustion, rate
limits, paid-tier responses, unavailable models and missing budget guarantees
move to the next compatible zero-cost provider, then local execution, then
`DEGRADED`.

## Capability matrix

The current adapters expose these task capabilities only when the route policy
also explicitly lists them as zero-cost eligible:

| Provider | Text / structured extraction | Translation / summary / matching / tags / validation | Vision | URL/web context | Embeddings |
| --- | --- | --- | --- | --- | --- |
| Gemini | Yes; structured JSON schema | Yes, via text task | Yes when Gemini route is configured for it | No adapter-side web context | No |
| Groq | Yes; structured JSON schema | Yes, via text task | No | No | No |
| Cloudflare Workers AI | Yes for configured text model | Yes, via text task | No | No | No |
| Mistral | Yes; JSON mode validated by Core | Yes, via text task | Not claimed by this adapter | No | No |
| Hugging Face | Yes; JSON schema for configured route/model | Yes, via text task | Not claimed by this adapter | No | No |
| Local llama.cpp | Yes; configured server must support JSON schema request shape | Yes, via local model | Not claimed by this adapter | No | No |

The matrix does not infer web search, vision or embeddings from a provider's
general catalog. Those capabilities need a dedicated adapter and explicit
route policy before selection.

## Provider-specific safety

### Mistral

Mistral calls use `https://api.mistral.ai/v1/chat/completions`. The model and
capabilities must be explicitly declared as Mistral Free-mode routes. Paid
responses, billing-required responses and exhausted Free-mode quota are denied
and never retried through pay-as-you-go.

### Hugging Face Inference Providers

Hugging Face calls use `https://router.huggingface.co/v1/chat/completions` and
the routed Hugging Face token only. This provider is fallback/diversity
oriented, not primary load. It is fail-closed: model, capability, remaining
free budget and a conservative maximum call bound must all be configured. The
Router reserves that bound before the call and stops when the bound cannot be
guaranteed. No custom paid provider key or PAYG activation is used.

## Runtime configuration

Values are never persisted or logged. Names and empty documentation-only
entries are kept in `.env.example`; the local `.env` remains private.

- Gemini: `NEX_AI_GEMINI_API_KEY`, `NEX_AI_GEMINI_MODEL`,
  `NEX_AI_GEMINI_ALLOWED_COST`, `NEX_AI_GEMINI_ZERO_COST_MODELS`,
  `NEX_AI_GEMINI_ZERO_COST_CAPABILITIES`
- Groq: `NEX_AI_GROQ_API_KEY`, `NEX_AI_GROQ_MODEL`,
  `NEX_AI_GROQ_ALLOWED_COST`, `NEX_AI_GROQ_ZERO_COST_MODELS`,
  `NEX_AI_GROQ_ZERO_COST_CAPABILITIES`
- Cloudflare: `NEX_AI_CLOUDFLARE_WORKERS_AI_ACCOUNT_ID`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_API_TOKEN`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_MODEL`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_ALLOWED_COST`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_ZERO_COST_MODELS`,
  `NEX_AI_CLOUDFLARE_WORKERS_AI_ZERO_COST_CAPABILITIES`
- Mistral: `MISTRAL_API_KEY`, `MISTRAL_MODEL`, `MISTRAL_ALLOWED_COST`,
  `MISTRAL_ZERO_COST_MODELS`, `MISTRAL_ZERO_COST_CAPABILITIES`
- Hugging Face: `HUGGINGFACE_API_TOKEN`, `HUGGINGFACE_MODEL`,
  `HUGGINGFACE_INFERENCE_PROVIDER`, `HUGGINGFACE_ALLOWED_COST`,
  `HUGGINGFACE_ZERO_COST_MODELS`, `HUGGINGFACE_ZERO_COST_CAPABILITIES`,
  `HUGGINGFACE_FREE_BUDGET_USD`, `HUGGINGFACE_MAX_CALL_COST_USD`
- Local: `NEX_AI_LOCAL_ENDPOINT`, `NEX_AI_LOCAL_MODEL`, `NEX_AI_LOCAL_API_KEY`

Without a configured route, Curator reports `DEGRADED` while the User catalog,
Milano Pack, search and Personal remain available.
