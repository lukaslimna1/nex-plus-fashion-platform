import { describe, expect, it } from "vitest";
import { parseFHCMCalendar, toParisIso } from "./scrape-fhcm.js";

describe("FHCM deterministic scraper", () => {
  it("extracts the target schedule row without AI", () => {
    const html = `<main><h1>Womenswear Spring/Summer 2027</h1><article>Julie Kegels <span>14:30</span> Show by invitation</article></main>`;
    expect(parseFHCMCalendar(html)).toMatchObject({ title: "Julie Kegels", localDate: "2026-09-28", localTime: "14:30" });
  });
  it("preserves Paris civil time as an instant", () => {
    expect(toParisIso("2026-09-28", "14:30")).toBe("2026-09-28T12:30:00.000Z");
  });
});
