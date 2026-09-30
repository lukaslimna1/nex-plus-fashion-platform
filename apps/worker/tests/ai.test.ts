import { describe, expect, it } from "vitest";
import { AIProviderRouter } from "../src/ai/router.js";
import type { AIProvider } from "@nex-plus/core";
import type { AIRequest } from "@nex-plus/types";

function provider(name: string, fail = false): AIProvider {
  return {
    name, model: "test-model",
    async extractStructuredData<T>(request: AIRequest) { if (fail) throw new Error("provider failed"); return { provider: name, model: "test-model", generatedAt: "2026-09-30T00:00:00.000Z", timestamp: "2026-09-30T00:00:00.000Z", fallbackUsed: false, attempts: 1, inputSourceIds: request.inputSourceIds, schemaVersion: request.schemaVersion, status: "AI_DRAFT" as const, data: { ok: true } as T }; },
    async translate() { throw new Error("not used"); }, async summarize() { throw new Error("not used"); }, async classify() { throw new Error("not used"); }
  };
}

const context = { entityType: "TERM" as const, entityId: "term-1", canonicalName: "Knitwear", language: "en", sourceUrls: ["https://example.test/source"], currentState: {}, allowedOutputSchema: { type: "object" } };
const requestOptions = { context, outputSchema: { type: "object" } };

describe("AI provider router", () => {
  it("uses the preferred provider and falls back without exposing it to domain code", async () => {
    const router = new AIProviderRouter({ gemini: provider("gemini", true), workersAI: provider("workers-ai") }, { maxAttempts: 1 });
    const result = await router.extractStructuredData({ task: "translateToPtBr", input: "{}", inputSourceIds: ["s-1"], schemaVersion: "1.0", ...requestOptions });
    expect(result.provider).toBe("workers-ai");
    expect(result.fallbackUsed).toBe(true);
  });
  it("rejects deterministic tasks at the AI boundary", async () => {
    const router = new AIProviderRouter({ gemini: provider("gemini") });
    await expect(router.extractStructuredData({ task: "normalizeNames", input: "{}", inputSourceIds: [], schemaVersion: "1.0", ...requestOptions })).rejects.toThrow("deterministic");
  });
  it("does not silently fallback complex editorial extraction", async () => {
    const router = new AIProviderRouter({ gemini: provider("gemini", true), workersAI: provider("workers-ai") }, { maxAttempts: 1 });
    await expect(router.extractStructuredData({ task: "extractCollectionMetadata", input: "{}", inputSourceIds: [], schemaVersion: "1.0", ...requestOptions })).rejects.toThrow("provider failed");
  });
  it("retries 503 with exponential scheduling and returns audit metadata", async () => {
    let attempts = 0;
    const flaky: AIProvider = {
      name: "gemini", model: "test-model",
      async extractStructuredData<T>(request: AIRequest) {
        attempts += 1;
        if (attempts < 3) throw Object.assign(new Error("503 UNAVAILABLE"), { status: 503 });
        return { provider: "gemini", model: "test-model", generatedAt: "2026-09-30T00:00:00.000Z", timestamp: "2026-09-30T00:00:00.000Z", fallbackUsed: false, attempts: 1, inputSourceIds: request.inputSourceIds, schemaVersion: request.schemaVersion, status: "AI_DRAFT" as const, data: { ok: true } as T };
      },
      async translate() { throw new Error("not used"); }, async summarize() { throw new Error("not used"); }, async classify() { throw new Error("not used"); }
    };
    const statuses: string[] = [];
    const router = new AIProviderRouter({ gemini: flaky }, { baseDelayMs: 0, maxDelayMs: 0, jitterRatio: 0, sleep: async () => undefined, logger: (entry) => statuses.push(entry.status) });
    const result = await router.extractStructuredData({ task: "extractCollectionMetadata", input: "{}", inputSourceIds: [], schemaVersion: "1.0", ...requestOptions });
    expect(attempts).toBe(3);
    expect(statuses).toEqual(["503", "503"]);
    expect(result).toMatchObject({ provider: "gemini", model: "test-model", fallbackUsed: false, attempts: 3 });
    expect(result.timestamp).toEqual(expect.any(String));
  });
});
