import { AI_POLICY } from "@nex-plus/core";
import type { AIProvider } from "@nex-plus/core";
import type { AIEnvelope, AIRequest } from "@nex-plus/types";

export class AIProviderRouter {
  public constructor(private readonly providers: { gemini?: AIProvider; workersAI?: AIProvider }) {}

  async extractStructuredData<T>(request: AIRequest): Promise<AIEnvelope<T>> {
    const policy = AI_POLICY[request.task];
    if (policy.preferred === "deterministic") throw new Error(`Task ${request.task} must use deterministic logic first`);
    const candidates = [this.providers[policy.preferred === "gemini" ? "gemini" : "workersAI"]];
    if (policy.fallback) candidates.push(this.providers[policy.fallback === "gemini" ? "gemini" : "workersAI"]);
    let lastError: unknown;
    for (const provider of candidates) {
      if (!provider) continue;
      try { return await provider.extractStructuredData<T>(request); } catch (error) { lastError = error; }
    }
    throw lastError instanceof Error ? lastError : new Error("No AI provider is configured");
  }
}
