import type { AIRequest, AIEnvelope, AITask } from "@nex-plus/types";

export interface AIProvider {
  readonly name: string;
  readonly model: string;
  extractStructuredData<T>(request: AIRequest): Promise<AIEnvelope<T>>;
  translate(input: AIRequest): Promise<AIEnvelope<{ text: string }>>;
  summarize(input: AIRequest): Promise<AIEnvelope<{ text: string }>>;
  classify(input: AIRequest): Promise<AIEnvelope<{ labels: string[] }>>;
}

export interface ProviderPolicy { preferred: "gemini" | "workers-ai" | "deterministic"; fallback?: "gemini" | "workers-ai"; }

export const AI_POLICY: Record<AITask, ProviderPolicy> = {
  extractCalendar: { preferred: "gemini", fallback: "workers-ai" }, extractScheduleEntries: { preferred: "gemini", fallback: "workers-ai" },
  extractCollectionMetadata: { preferred: "gemini", fallback: "workers-ai" }, extractMaisonMetadata: { preferred: "gemini", fallback: "workers-ai" }, extractEventMetadata: { preferred: "gemini", fallback: "workers-ai" },
  normalizeNames: { preferred: "deterministic", fallback: "gemini" }, normalizeSeason: { preferred: "deterministic", fallback: "gemini" }, normalizeCredits: { preferred: "gemini", fallback: "workers-ai" },
  translateToPtBr: { preferred: "gemini", fallback: "workers-ai" }, summarizePressRelease: { preferred: "gemini", fallback: "workers-ai" }, summarizeProfessionalReview: { preferred: "gemini", fallback: "workers-ai" },
  detectSourceChanges: { preferred: "deterministic", fallback: "gemini" }, suggestTags: { preferred: "gemini", fallback: "workers-ai" }, suggestTerms: { preferred: "gemini", fallback: "workers-ai" }, suggestTrendEvidence: { preferred: "gemini", fallback: "workers-ai" },
  extractImageMetadata: { preferred: "gemini", fallback: "workers-ai" }, extractVideoMetadata: { preferred: "gemini", fallback: "workers-ai" }
};
