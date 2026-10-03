import assert from "node:assert/strict";
import test from "node:test";
import {
  BASELINE_MIGRATIONS,
  CORE_PROTOCOL_VERSION,
  AI_ALLOWED_COST,
  AI_CONTRACT_VERSION,
  AI_CAPABILITIES,
  AI_PROVIDERS,
  READ_CONTRACT_VERSION,
  isEntityKind,
} from "../dist/index.js";

test("baseline contract exposes the local core protocol", () => {
  assert.equal(CORE_PROTOCOL_VERSION, "0.1");
  assert.equal(AI_CONTRACT_VERSION, "1.0");
  assert.equal(AI_ALLOWED_COST, "ZERO");
  assert.deepEqual(Object.values(AI_PROVIDERS), [
    "gemini",
    "groq",
    "cloudflare_workers_ai",
    "mistral",
    "hugging_face",
    "local_llama_cpp",
  ]);
  assert.equal(AI_CAPABILITIES.localOnly, "LOCAL_ONLY");
  assert.equal(READ_CONTRACT_VERSION, "1.0");
  assert.deepEqual(BASELINE_MIGRATIONS, ["0001_core", "0002_fts5", "0003_milano_vertical", "0004_pack_runtime", "0005_personal_favorite", "0006_ai_curator"]);
});

test("entity-kind validation rejects unknown or non-scalar values", () => {
  assert.equal(isEntityKind("city"), true);
  assert.equal(isEntityKind("city_hub"), true);
  assert.equal(isEntityKind("milano_fact_mock"), false);
  assert.equal(isEntityKind({}), false);
});
