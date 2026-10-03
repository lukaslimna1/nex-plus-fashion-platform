import assert from "node:assert/strict";
import test from "node:test";
import { BASELINE_MIGRATIONS, CORE_PROTOCOL_VERSION, isEntityKind } from "../dist/index.js";

test("baseline contract exposes the local core protocol", () => {
  assert.equal(CORE_PROTOCOL_VERSION, "0.1");
  assert.deepEqual(BASELINE_MIGRATIONS, ["0001_core", "0002_fts5", "0003_milano_vertical", "0004_pack_runtime"]);
});

test("entity-kind validation rejects unknown or non-scalar values", () => {
  assert.equal(isEntityKind("city_hub"), true);
  assert.equal(isEntityKind("milano_fact_mock"), false);
  assert.equal(isEntityKind({}), false);
});
