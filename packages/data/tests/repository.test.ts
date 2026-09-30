import { describe, expect, it } from "vitest";
import { IdempotencyLedger } from "../src/repository.js";

describe("sync idempotency", () => {
  it("accepts only a strictly newer revision", () => {
    const ledger = new IdempotencyLedger();
    const mutation = { entityType: "collection", entityId: "c-1", revision: 2, updatedAt: "2026-09-30T00:00:00Z" };
    expect(ledger.accept(mutation)).toBe(true);
    expect(ledger.accept(mutation)).toBe(false);
    expect(ledger.accept({ ...mutation, revision: 1 })).toBe(false);
    expect(ledger.accept({ ...mutation, revision: 3 })).toBe(true);
  });
});
