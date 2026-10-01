import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const registry = readFileSync(new URL("../docs/research/resort-2027-image-sources-2026-09-30.json", import.meta.url), "utf8");
const migration = readFileSync(new URL("../migrations/0022_source_registry_resort_image_sources.sql", import.meta.url), "utf8");

describe("Resort 2027 image source registry", () => {
  it("keeps the requested WWD and Vogue source URLs explicit", () => {
    expect(registry).toContain("https://wwd.com/runway/resort-2027/paris/");
    expect(registry).toContain("https://www.vogue.com/fashion-shows/resort-2027");
    expect(registry).toContain('"downloadPolicy": "DOWNLOAD_BLOCKED"');
  });

  it("registers both sources idempotently with source checks", () => {
    expect(migration).toContain("INSERT OR IGNORE INTO sources");
    expect(migration).toContain("'source-wwd'");
    expect(migration).toContain("'source-vogue-resort'");
    expect(migration).toContain("INSERT OR IGNORE INTO source_checks");
  });
});
