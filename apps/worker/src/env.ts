import type { D1DatabaseLike } from "@nex-plus/data";
import type { WorkersAIBinding } from "./ai/cloudflare.js";

export interface WorkerEnv {
  DB: D1DatabaseLike;
  AI?: WorkersAIBinding;
  ASSETS?: Fetcher;
  APP_ENV?: string;
  WORKER_VERSION?: string;
  GIT_COMMIT?: string;
  CORS_ALLOWED_ORIGINS?: string;
  GEMINI_API_KEY?: string;
  GEMINI_MODEL?: string;
  YOUTUBE_API_KEY?: string;
  GOOGLE_CLIENT_ID?: string;
  GOOGLE_CLIENT_SECRET?: string;
  GOOGLE_REDIRECT_URI?: string;
}
