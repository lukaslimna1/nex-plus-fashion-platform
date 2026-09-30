import { describe, expect, it } from "vitest";
import { resolveMedia } from "../src/resolver.js";
import type { Asset } from "@nex-plus/types";

const asset: Asset = {
  id: "asset-1", sourcePageUrl: "https://example.com/page", alternativeUrls: ["https://example.com/alt.jpg"],
  remoteUrl: "https://example.com/main.jpg", sourceIds: ["source-1"], rightsStatus: "UNKNOWN",
  downloadPolicy: "DOWNLOAD_UNKNOWN", displayMode: "INLINE", canonicalStatus: "PENDING_REVIEW"
};

describe("inline-first media resolver", () => {
  it("prefers remote media before alternatives and placeholder", () => {
    expect(resolveMedia(asset, { placeholderUrl: "https://example.com/placeholder.svg" }).kind).toBe("remote");
  });
  it("rejects non-https media and keeps provenance policy out of the URL decision", () => {
    expect(resolveMedia({ ...asset, remoteUrl: "javascript:alert(1)", embedUrl: "https://example.com/embed" }, { placeholderUrl: "https://example.com/placeholder.svg" }).kind).toBe("alternative");
  });
});
