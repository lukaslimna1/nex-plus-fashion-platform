import type { D1DatabaseLike } from "@nex-plus/data";
import type { AIEntityContext, AIOutputStatus, AITask } from "@nex-plus/types";
import { AIProviderRouter } from "./router.js";

type JobRow = {
  id: string; entityType: AIEntityContext["entityType"]; entityId: string; task: AITask;
  inputSourceIds: string; context: string; attempts: number;
};

export interface AIJobBatchResult {
  claimed: number;
  succeeded: number;
  failed: number;
  blocked: number;
  fallbackUsed: number;
}

function ids(value: string): string[] { return value ? value.split(",").map((item) => item.trim()).filter(Boolean) : []; }
function isProviderUnavailable(error: unknown): boolean {
  const text = error instanceof Error ? error.message : String(error);
  return /429|503|unavailable|high demand|overloaded|timeout|timed out|aborted/i.test(text);
}

/**
 * Claims a small batch and records only AI drafts. It deliberately never updates
 * editorial tables; promotion remains a human-review operation.
 */
export async function runAIEnrichmentBatch(db: D1DatabaseLike, router: AIProviderRouter, limit = 3, now = new Date()): Promise<AIJobBatchResult> {
  const current = now.toISOString();
  const { results } = await db.prepare("SELECT id, entity_type AS entityType, entity_id AS entityId, task, input_source_ids AS inputSourceIds, context_json AS context, attempts FROM ai_enrichment_jobs WHERE status = 'PENDING' AND scheduled_at <= ? ORDER BY scheduled_at, created_at LIMIT ?").bind(current, limit).all<JobRow>();
  const summary: AIJobBatchResult = { claimed: results.length, succeeded: 0, failed: 0, blocked: 0, fallbackUsed: 0 };
  for (const row of results) {
    await db.prepare("UPDATE ai_enrichment_jobs SET status = 'RUNNING', attempts = attempts + 1, updated_at = ? WHERE id = ? AND status = 'PENDING'").bind(current, row.id).run();
    try {
      const context = JSON.parse(row.context) as AIEntityContext;
      const result = await router.extractStructuredData({
        task: row.task,
        input: JSON.stringify({ entityType: context.entityType, entityId: context.entityId, canonicalName: context.canonicalName, sourceUrls: context.sourceUrls, retrievedText: context.retrievedText ?? "", currentState: context.currentState }),
        inputSourceIds: ids(row.inputSourceIds),
        schemaVersion: "1.0",
        context,
        outputSchema: context.allowedOutputSchema
      });
      const resultJson = JSON.stringify(result.data);
      const reviewStatus: AIOutputStatus = "PENDING_REVIEW";
      await db.prepare("UPDATE ai_enrichment_jobs SET status = 'SUCCEEDED', result_json = ?, review_status = ?, provider = ?, model = ?, completed_at = ?, error = NULL, updated_at = ? WHERE id = ?").bind(resultJson, reviewStatus, result.provider, result.model, current, current, row.id).run();
      await db.prepare("INSERT INTO ai_runs (id, task, provider, model, generated_at, input_source_ids, confidence, schema_version, status, raw_output, validated_output) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)").bind(`ai-run-${row.id}-${Date.now()}`, row.task, result.provider, result.model, result.generatedAt, row.inputSourceIds, result.confidence ?? null, result.schemaVersion, result.status, resultJson, resultJson).run();
      summary.succeeded += 1;
      if (result.fallbackUsed) summary.fallbackUsed += 1;
    } catch (error) {
      const blocked = isProviderUnavailable(error);
      const status = blocked ? "BLOCKED" : "FAILED";
      const message = (error instanceof Error ? error.message : String(error)).slice(0, 500);
      await db.prepare("UPDATE ai_enrichment_jobs SET status = ?, error = ?, updated_at = ? WHERE id = ?").bind(status, message, current, row.id).run();
      if (blocked) summary.blocked += 1; else summary.failed += 1;
    }
  }
  return summary;
}

