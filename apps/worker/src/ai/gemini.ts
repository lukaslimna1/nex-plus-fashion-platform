import { GoogleGenAI } from "@google/genai";
import type { AIEnvelope, AIRequest } from "@nex-plus/types";
import type { AIProvider } from "@nex-plus/core";

export class GeminiAIProvider implements AIProvider {
  public readonly name = "gemini";
  public constructor(private readonly apiKey: string, public readonly model: string = "gemini-2.5-flash") {}

  async extractStructuredData<T>(request: AIRequest): Promise<AIEnvelope<T>> {
    const client = new GoogleGenAI({ apiKey: this.apiKey });
    const response = await client.models.generateContent({
      model: this.model,
      contents: request.input,
      config: { responseMimeType: "application/json" }
    });
    const text = response.text?.trim() ?? "";
    if (!text) throw new Error("Gemini returned an empty response");
    const data = JSON.parse(text) as T;
    return { provider: this.name, model: this.model, generatedAt: new Date().toISOString(), inputSourceIds: request.inputSourceIds, schemaVersion: request.schemaVersion, status: "AI_DRAFT", data };
  }
  translate(request: AIRequest): Promise<AIEnvelope<{ text: string }>> { return this.extractStructuredData<{ text: string }>(request); }
  summarize(request: AIRequest): Promise<AIEnvelope<{ text: string }>> { return this.extractStructuredData<{ text: string }>(request); }
  classify(request: AIRequest): Promise<AIEnvelope<{ labels: string[] }>> { return this.extractStructuredData<{ labels: string[] }>(request); }
}
