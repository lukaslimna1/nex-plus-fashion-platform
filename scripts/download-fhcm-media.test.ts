import { describe, expect, it } from "vitest";
import { mediaCandidates } from "./download-fhcm-media.js";

describe("FHCM staging downloader fallback order", () => {
  it("tries remote, alternatives, then thumbnail without duplicate requests", () => {
    expect(mediaCandidates({
      remoteUrl: "https://fhcm.test/primary.jpg",
      alternativeUrls: ["https://fhcm.test/alternate.jpg", "https://fhcm.test/primary.jpg"],
      thumbnailUrl: "https://fhcm.test/thumb.jpg"
    })).toEqual([
      { url: "https://fhcm.test/primary.jpg", kind: "remote" },
      { url: "https://fhcm.test/alternate.jpg", kind: "alternative" },
      { url: "https://fhcm.test/thumb.jpg", kind: "thumbnail" }
    ]);
  });
});
