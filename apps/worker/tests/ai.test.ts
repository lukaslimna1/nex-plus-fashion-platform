import { describe, expect, it } from "vitest";
import { AIProviderRouter } from "../src/ai/router.js";
import type { AIProvider } from "@nex-plus/core";
import type { AIRequest } from "@nex-plus/types";

function provider(name: string, fail = false): AIProvider {
  return {
    name, model: "test-model",
    async extractStructuredData<T>(request: AIRequest) { if (fail) throw new Error("provider failed"); return { provider: name, model: "test-model", generatedAt: "2026-09-30T00:00:00.000Z", inputSourceIds: request.inputSourceIds, schemaVersion: request.schemaVersion, status: "AI_DRAFT" as const, data: { ok: true } as T }; },
    async translate() { throw new Error("not used"); }, async summarize() { throw new Error("not used"); }, async classify() { throw new Error("not used"); }
  };
}

describe("AI provider router", () => {
  it("uses the preferred provider and falls back without exposing it to domain code", async () => {
    const router = new AIProviderRouter({ gemini: provider("gemini", true), workersAI: provider("workers-ai") });
    const result = await router.extractStructuredData({ task: "extractCollectionMetadata", input: "{}", inputSourceIds: ["s-1"], schemaVersion: "1.0" });
    expect(result.provider).toBe("workers-ai");
  });
  it("rejects deterministic tasks at the AI boundary", async () => {
    const router = new AIProviderRouter({ gemini: provider("gemini") });
    await expect(router.extractStructuredData({ task: "normalizeNames", input: "{}", inputSourceIds: [], schemaVersion: "1.0" })).rejects.toThrow("deterministic");
  });
});
