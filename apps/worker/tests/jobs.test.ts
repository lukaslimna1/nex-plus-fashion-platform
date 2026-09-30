import { describe, expect, it } from "vitest";
import type { AIProvider } from "@nex-plus/core";
import type { AIRequest } from "@nex-plus/types";
import { AIProviderRouter } from "../src/ai/router.js";
import { runAIEnrichmentBatch } from "../src/ai/jobs.js";

describe("AI enrichment jobs", () => {
  it("records a structured draft for review without publishing catalog data", async () => {
    const statements: Array<{ query: string; values: unknown[] }> = [];
    const db = {
      prepare(query: string) {
        const statement = { query, values: [] as unknown[] };
        statements.push(statement);
        return {
          bind(...values: unknown[]) { statement.values = values; return this; },
          async all() {
            return query.includes("FROM ai_enrichment_jobs") ? { results: [{ id: "job-1", entityType: "TERM", entityId: "term-1", task: "translateToPtBr", inputSourceIds: "source-getty", context: JSON.stringify({ entityType: "TERM", entityId: "term-1", canonicalName: "knitting", language: "en", sourceUrls: ["https://vocab.getty.edu/aat/300053634"], currentState: {}, allowedOutputSchema: { type: "object" } }), attempts: 0 }] } : { results: [] };
          },
          async first() { return null; },
          async run() { return { success: true }; }
        };
      }
    } as never;
    const provider: AIProvider = {
      name: "gemini", model: "test-model",
      async extractStructuredData<T>(request: AIRequest) { return { provider: "gemini", model: "test-model", generatedAt: "2026-09-30T00:00:00.000Z", timestamp: "2026-09-30T00:00:00.000Z", fallbackUsed: false, attempts: 1, inputSourceIds: request.inputSourceIds, schemaVersion: request.schemaVersion, status: "AI_DRAFT" as const, data: { ptBrName: "malharia" } as T }; },
      async translate() { throw new Error("not used"); }, async summarize() { throw new Error("not used"); }, async classify() { throw new Error("not used"); }
    };
    const summary = await runAIEnrichmentBatch(db, new AIProviderRouter({ gemini: provider }, { maxAttempts: 1 }), 1, new Date("2026-09-30T00:00:00.000Z"));
    expect(summary).toMatchObject({ claimed: 1, succeeded: 1, failed: 0, blocked: 0 });
    expect(statements.some((entry) => entry.query.includes("status = 'SUCCEEDED'") && entry.query.includes("review_status"))).toBe(true);
    expect(statements.some((entry) => entry.query.includes("INSERT INTO ai_runs"))).toBe(true);
    expect(statements.some((entry) => entry.query.includes("UPDATE terms"))).toBe(false);
  });
});
