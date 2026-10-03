# Curator AI Router

The User catalog remains local-first and does not require AI. Curator/Admin uses
the router as an operational capability for discovery, interpretation,
extraction, translation, matching, classification and proposal generation.

The router is provider-neutral at the Core boundary:

    Source/Search/Adapter
      -> AI Router
      -> Provider adapter
      -> AIExecution
      -> Candidate
      -> Curator Proposal
      -> human review
      -> optional Official promotion

proposalKind, capability, and the structured payload are open strings/data.
The initial capability constants are defaults, not a closed allow-list. New
proposal types can be stored and reviewed without changing the migration.

The only hard authority boundary is promotion: a provider response never writes
Official data. APPROVE or EDIT_APPROVE is required. The Core currently has
promotion handlers for reviewed source discovery and review summaries; other
proposal types remain auditable until an explicit promotion handler exists.

## Runtime configuration

The router reads only these environment variable names. Values are never
persisted or logged:

- NEX_AI_GEMINI_API_KEY, NEX_AI_GEMINI_MODEL
- NEX_AI_XAI_API_KEY, NEX_AI_XAI_MODEL
- NEX_AI_GROQ_API_KEY, NEX_AI_GROQ_MODEL
- NEX_AI_OPENROUTER_API_KEY, NEX_AI_OPENROUTER_MODEL
- NEX_AI_LOCAL_ENDPOINT, NEX_AI_LOCAL_MODEL, NEX_AI_LOCAL_API_KEY

Without a configured provider, Curator reports DEGRADED and AI executions
return AI_NOT_CONFIGURED. The User catalog, Milano Pack, search and Personal
remain available.

The current adapters use Gemini structured generation and the OpenAI-compatible
structured-output surface for xAI, Groq and OpenRouter. Runtime output is still
validated in Core because provider-side schema guarantees do not establish
editorial truth.
