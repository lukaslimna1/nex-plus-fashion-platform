import { describe, expect, it } from "vitest";
import { D1CatalogRepository, IdempotencyLedger } from "../src/repository.js";

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

describe("asset query contract", () => {
  it("passes collection, source, coverage and media filters to D1", async () => {
    let query = "";
    let values: unknown[] = [];
    const db = {
      prepare(input: string) {
        query = input;
        return {
          bind(...inputValues: unknown[]) { values = inputValues; return this; },
          async all() { return { results: [] }; },
          async first() { return null; },
          async run() { return { success: true }; }
        };
      }
    } as never;
    await new D1CatalogRepository(db).listAssets({ collection: "maxhosa-africa-womenswear-spring-summer-2027", source: "source-vogue-runway", coverageType: "RUNWAY", mediaType: "IMAGE" });
    expect(query).toContain("c.slug = ?");
    expect(query).toContain("a.source_ids");
    expect(query).toContain("COALESCE(a.coverage_type_label, a.coverage_type) = ?");
    expect(query).toContain("a.asset_kind = ?");
    expect(values).toEqual(["maxhosa-africa-womenswear-spring-summer-2027", "maxhosa-africa-womenswear-spring-summer-2027", "%,source-vogue-runway,%", "RUNWAY", "IMAGE"]);
  });
});

describe("catalog relationship contract", () => {
  it("loads only the explicitly related schedule entry for a Collection", async () => {
    let query = "";
    const db = {
      prepare(input: string) {
        query = input;
        return {
          bind() { return this; },
          async all() { return { results: [] }; },
          async first() { return null; },
          async run() { return { success: true }; }
        };
      }
    } as never;
    await new D1CatalogRepository(db).listSchedule();
    expect(query).toContain("collection_schedule_entries");
    expect(query).toContain("cse.schedule_entry_id = s.id");
  });
});
