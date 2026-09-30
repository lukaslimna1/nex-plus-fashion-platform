import { AI_POLICY } from "@nex-plus/core";
import type { AIProvider } from "@nex-plus/core";
import type { AIEnvelope, AIRequest, AITask } from "@nex-plus/types";

export interface AIProviderRouterOptions {
  timeoutMs?: number;
  maxAttempts?: number;
  baseDelayMs?: number;
  maxDelayMs?: number;
  jitterRatio?: number;
  sleep?: (ms: number) => Promise<void>;
  random?: () => number;
  logger?: (entry: { provider: string; model: string; attempt: number; status: string }) => void;
}

const lightweightFallbackTasks = new Set<AITask>(["normalizeCredits", "translateToPtBr", "suggestTags"]);

function statusOf(error: unknown): string {
  if (error && typeof error === "object") {
    const candidate = error as { status?: unknown; code?: unknown; response?: { status?: unknown } };
    const status = candidate.status ?? candidate.code ?? candidate.response?.status;
    if (typeof status === "number" || typeof status === "string") return String(status);
  }
  const message = error instanceof Error ? error.message : String(error);
  if (/429|resource[_ -]?exhausted|rate limit/i.test(message)) return "429";
  if (/503|unavailable|high demand|overloaded/i.test(message)) return "503";
  if (/timeout|timed out|aborted/i.test(message)) return "TIMEOUT";
  return "ERROR";
}

function retryable(error: unknown): boolean {
  return ["429", "503", "TIMEOUT"].includes(statusOf(error));
}

function timeout<T>(promise: Promise<T>, timeoutMs: number): Promise<T> {
  if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) return promise;
  return new Promise<T>((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("AI provider request timed out")), timeoutMs);
    promise.then((value) => { clearTimeout(timer); resolve(value); }, (error) => { clearTimeout(timer); reject(error); });
  });
}

export class AIProviderRouter {
  private readonly options: Required<AIProviderRouterOptions>;
  public constructor(private readonly providers: { gemini?: AIProvider; workersAI?: AIProvider }, options: AIProviderRouterOptions = {}) {
    this.options = {
      timeoutMs: options.timeoutMs ?? 20_000,
      maxAttempts: options.maxAttempts ?? 3,
      baseDelayMs: options.baseDelayMs ?? 250,
      maxDelayMs: options.maxDelayMs ?? 4_000,
      jitterRatio: options.jitterRatio ?? 0.2,
      sleep: options.sleep ?? ((ms) => new Promise((resolve) => setTimeout(resolve, ms))),
      random: options.random ?? Math.random,
      logger: options.logger ?? (() => undefined)
    };
  }

  async extractStructuredData<T>(request: AIRequest): Promise<AIEnvelope<T>> {
    const policy = AI_POLICY[request.task];
    if (policy.preferred === "deterministic") throw new Error(`Task ${request.task} must use deterministic logic first`);
    const candidates: Array<{ provider: AIProvider; fallbackUsed: boolean }> = [];
    const preferred = this.providers[policy.preferred === "gemini" ? "gemini" : "workersAI"];
    if (preferred) candidates.push({ provider: preferred, fallbackUsed: false });
    const fallback = policy.fallback ? this.providers[policy.fallback === "gemini" ? "gemini" : "workersAI"] : undefined;
    if (fallback && fallback !== preferred && lightweightFallbackTasks.has(request.task)) candidates.push({ provider: fallback, fallbackUsed: true });
    let lastError: unknown;
    for (const candidate of candidates) {
      try {
        const result = await this.callWithRetry<T>(candidate.provider, request);
        const timestamp = new Date().toISOString();
        return { ...result, timestamp, fallbackUsed: candidate.fallbackUsed, attempts: result.attempts };
      } catch (error) {
        lastError = error;
        if (candidate.fallbackUsed || !lightweightFallbackTasks.has(request.task)) break;
      }
    }
    throw lastError instanceof Error ? lastError : new Error("No AI provider is configured");
  }

  private async callWithRetry<T>(provider: AIProvider, request: AIRequest): Promise<AIEnvelope<T>> {
    let lastError: unknown;
    for (let attempt = 1; attempt <= this.options.maxAttempts; attempt += 1) {
      try {
        const result = await timeout(provider.extractStructuredData<T>(request), this.options.timeoutMs);
        return { ...result, attempts: attempt };
      } catch (error) {
        lastError = error;
        const status = statusOf(error);
        this.options.logger({ provider: provider.name, model: provider.model, attempt, status });
        if (!retryable(error) || attempt >= this.options.maxAttempts) break;
        const exponential = Math.min(this.options.maxDelayMs, this.options.baseDelayMs * 2 ** (attempt - 1));
        const jitter = exponential * this.options.jitterRatio * this.options.random();
        await this.options.sleep(exponential + jitter);
      }
    }
    throw lastError instanceof Error ? lastError : new Error("AI provider request failed");
  }
}
