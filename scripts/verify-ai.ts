import { GeminiAIProvider } from "../apps/worker/src/ai/gemini.js";
import { collectionExtractionSchema, scheduleExtractionSchema, validateAIOutput } from "../apps/worker/src/ai/schemas.js";

const apiKey = process.env.GEMINI_API_KEY;
if (!apiKey) {
  console.log(JSON.stringify({ provider: "gemini", status: "not_configured", schemaSuccess: false }));
  process.exit(0);
}

const started = performance.now();
try {
  const provider = new GeminiAIProvider(apiKey, process.env.GEMINI_MODEL || "gemini-3.8-flash");
  const collectionResult = await provider.extractStructuredData({
    task: "extractCollectionMetadata",
    input: "Return JSON only: {name:'Julie Kegels Womenswear Spring/Summer 2027', maisonName:'Julie Kegels', seasonCode:'SS27', seasonYear:2027, seasonLabel:'Spring/Summer 2027', presentedOn:'2026-09-28'}",
    inputSourceIds: ["source-fhcm"], schemaVersion: "1.0"
  });
  const parsedCollection = validateAIOutput(collectionExtractionSchema, collectionResult.data);
  const scheduleResult = await provider.extractStructuredData({
    task: "extractScheduleEntries",
    input: "Return JSON only: {entries:[{title:'Julie Kegels',format:'SHOW',startTime:'2026-09-28T12:30:00.000Z',timezone:'Europe/Paris',officialUrl:'https://www.fhcm.paris/en/paris-fashion-week/calendar'}]}",
    inputSourceIds: ["source-fhcm"], schemaVersion: "1.0"
  });
  const parsedSchedule = validateAIOutput(scheduleExtractionSchema, scheduleResult.data);
  console.log(JSON.stringify({ provider: collectionResult.provider, model: collectionResult.model, status: "ok", latencyMs: Math.round(performance.now() - started), flows: { extractCollectionMetadata: Boolean(parsedCollection), extractScheduleEntries: Boolean(parsedSchedule) }, schemaSuccess: Boolean(parsedCollection && parsedSchedule) }));
} catch (error) {
  console.log(JSON.stringify({ provider: "gemini", status: "error", latencyMs: Math.round(performance.now() - started), schemaSuccess: false, message: error instanceof Error ? error.message : "unknown" }));
  process.exitCode = 1;
}
