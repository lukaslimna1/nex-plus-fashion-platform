import { GeminiAIProvider } from "../apps/worker/src/ai/gemini.js";
import { AIProviderRouter } from "../apps/worker/src/ai/router.js";
import { collectionExtractionSchema, scheduleExtractionSchema, validateAIOutput } from "../apps/worker/src/ai/schemas.js";

const apiKey = process.env.GEMINI_API_KEY;
if (!apiKey) {
  console.log(JSON.stringify({ provider: "gemini", status: "GEMINI_API_KEY_REQUIRED", schemaSuccess: false }));
  process.exitCode = 1;
} else {
  const started = performance.now();
  try {
    const provider = new GeminiAIProvider(apiKey, process.env.GEMINI_MODEL || "gemini-3.8-flash");
    const router = new AIProviderRouter({ gemini: provider }, { logger: (entry) => console.log(JSON.stringify({ audit: entry })) });
    const collectionResult = await router.extractStructuredData({
      task: "extractCollectionMetadata",
      input: "Return JSON only: {name:'Julie Kegels Womenswear Spring/Summer 2027', maisonName:'Julie Kegels', seasonCode:'SS27', seasonYear:2027, seasonLabel:'Spring/Summer 2027', presentedOn:'2026-09-28'}",
      inputSourceIds: ["source-fhcm"], schemaVersion: "1.0"
    });
    const parsedCollection = validateAIOutput(collectionExtractionSchema, collectionResult.data);
    const scheduleResult = await router.extractStructuredData({
      task: "extractScheduleEntries",
      input: "Return JSON only: {entries:[{title:'Julie Kegels',format:'SHOW',startTime:'2026-09-28T12:30:00.000Z',timezone:'Europe/Paris',officialUrl:'https://www.fhcm.paris/en/paris-fashion-week/calendar'}]}",
      inputSourceIds: ["source-fhcm"], schemaVersion: "1.0"
    });
    const parsedSchedule = validateAIOutput(scheduleExtractionSchema, scheduleResult.data);
    console.log(JSON.stringify({ provider: collectionResult.provider, model: collectionResult.model, status: "AUTH_OK_PROVIDER_AVAILABLE", latencyMs: Math.round(performance.now() - started), attempts: collectionResult.attempts + scheduleResult.attempts, flows: { extractCollectionMetadata: Boolean(parsedCollection), extractScheduleEntries: Boolean(parsedSchedule) }, schemaSuccess: Boolean(parsedCollection && parsedSchedule) }));
  } catch (error) {
    const raw = error instanceof Error ? error.message : "unknown";
    const message = raw.replaceAll(apiKey, "[REDACTED]").slice(0, 300);
    const status = /401|403|api key|unauthori[sz]ed|invalid credential/i.test(raw)
      ? "AUTH_FAILED"
      : /429|503|unavailable|high demand|overloaded|timeout/i.test(raw)
        ? "AUTH_OK_PROVIDER_UNAVAILABLE"
        : "PROVIDER_ERROR";
    console.log(JSON.stringify({ provider: "gemini", model: process.env.GEMINI_MODEL || "gemini-3.8-flash", status, latencyMs: Math.round(performance.now() - started), schemaSuccess: false, message }));
    process.exitCode = 1;
  }
}
