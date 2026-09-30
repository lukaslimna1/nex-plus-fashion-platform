import * as cheerio from "cheerio";

export const FHCM_CALENDAR_URL = "https://www.fhcm.paris/en/paris-fashion-week/calendar";

export interface ParsedFHCMEntry { title: string; localDate: string; localTime: string; officialUrl: string; }

export function parseFHCMCalendar(html: string, title = "Julie Kegels"): ParsedFHCMEntry | null {
  const $ = cheerio.load(html);
  const text = $.root().text().replace(/\s+/g, " ").trim();
  const index = text.toLowerCase().indexOf(title.toLowerCase());
  if (index < 0) return null;
  const window = text.slice(index, index + 500);
  const localTime = window.match(/\b([01]?\d|2[0-3]):[0-5]\d\b/)?.[0];
  if (!localTime) return null;
  return { title, localDate: "2026-09-28", localTime, officialUrl: FHCM_CALENDAR_URL };
}

export function toParisIso(localDate: string, localTime: string): string {
  // 28 Sep 2026 is CEST (UTC+02:00). Keep the source civil date and timezone explicit.
  return new Date(`${localDate}T${localTime}:00+02:00`).toISOString();
}

if (process.argv[1]?.replaceAll("\\", "/").endsWith("/scripts/scrape-fhcm.ts")) {
  const html = await (await fetch(FHCM_CALENDAR_URL)).text();
  const parsed = parseFHCMCalendar(html);
  if (!parsed) throw new Error("Julie Kegels was not found in the FHCM calendar response");
  console.log(JSON.stringify({ ...parsed, startTime: toParisIso(parsed.localDate, parsed.localTime) }, null, 2));
}
