import type { AIProvider } from "@nex-plus/core";
import type { AIEnvelope, AIRequest } from "@nex-plus/types";

export interface WorkersAIBinding { run(model: string, input: Record<string, unknown>): Promise<unknown>; }

function extractText(value: unknown): string {
  if (typeof value === "string") return value;
  if (value && typeof value === "object" && "response" in value && typeof value.response === "string") return value.response;
  return JSON.stringify(value);
}

export class CloudflareAIProvider implements AIProvider {
  public readonly name = "workers-ai";
  public constructor(private readonly binding: WorkersAIBinding, public readonly model: string = "@cf/meta/llama-3.1-8b-instruct") {}
  async extractStructuredData<T>(request: AIRequest): Promise<AIEnvelope<T>> {
    const raw = await this.binding.run(this.model, { prompt: JSON.stringify({ instruction: request.input, context: request.context, allowedOutputSchema: request.outputSchema }), max_tokens: 512 });
    const text = extractText(raw).trim();
    let data: T;
    try { data = JSON.parse(text) as T; } catch { throw new Error("Workers AI returned non-JSON output"); }
    const timestamp = new Date().toISOString();
    return { provider: this.name, model: this.model, generatedAt: timestamp, timestamp, fallbackUsed: false, attempts: 1, inputSourceIds: request.inputSourceIds, schemaVersion: request.schemaVersion, status: "AI_DRAFT", data };
  }
  translate(request: AIRequest): Promise<AIEnvelope<{ text: string }>> { return this.extractStructuredData<{ text: string }>(request); }
  summarize(request: AIRequest): Promise<AIEnvelope<{ text: string }>> { return this.extractStructuredData<{ text: string }>(request); }
  classify(request: AIRequest): Promise<AIEnvelope<{ labels: string[] }>> { return this.extractStructuredData<{ labels: string[] }>(request); }
}
