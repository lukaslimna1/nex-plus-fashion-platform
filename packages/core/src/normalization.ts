import type { CivilDate } from "@nex-plus/types";

export function normalizeName(value: string): string {
  return value.trim().replace(/\s+/g, " ");
}

export function normalizeSeasonCode(value: string): string {
  return normalizeName(value).toUpperCase().replace(/[\s-]/g, "");
}

export function assertSeasonIntegrity(calendarYear: number, seasonYear: number, seasonCode: string): void {
  if (!Number.isInteger(calendarYear) || !Number.isInteger(seasonYear)) throw new Error("season years must be integers");
  if (!/^(SS|FW)\d{2,4}$/.test(normalizeSeasonCode(seasonCode))) throw new Error("invalid season code");
  if (seasonYear < calendarYear - 1 || seasonYear > calendarYear + 2) throw new Error("season year outside expected range");
}

export function isCivilDate(value: string): value is CivilDate {
  return /^\d{4}-\d{2}-\d{2}$/.test(value) && !Number.isNaN(Date.parse(`${value}T00:00:00Z`));
}

export function assertSafeUrl(value: string, allowedProtocols = ["https:"]): string {
  const url = new URL(value);
  if (!allowedProtocols.includes(url.protocol)) throw new Error("unsupported URL protocol");
  return url.toString();
}
