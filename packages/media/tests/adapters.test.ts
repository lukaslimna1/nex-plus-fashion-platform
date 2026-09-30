import { describe, expect, it } from "vitest";
import { getSourceAdapter, sourceAdapters } from "../src/adapters.js";

describe("source adapter registry", () => {
  it("keeps source capabilities separate from provider playback", () => {
    expect(getSourceAdapter("fhcm")).toMatchObject({ id: "fhcm", capabilities: { supportsSchedule: true, supportsImages: true } });
    expect(getSourceAdapter("youtube")).toMatchObject({ id: "youtube", provider: "youtube", capabilities: { supportsEmbed: true } });
    expect(sourceAdapters.some((adapter) => adapter.id === "vogue-runway" && adapter.capabilities.supportsPagination)).toBe(true);
  });
});
