import type { ScheduleEntry, ScheduleState } from "@nex-plus/types";

export const SOON_WINDOW_MS = 30 * 60 * 1000;

function validDate(value: string | undefined): number | undefined {
  if (!value) return undefined;
  const time = Date.parse(value);
  return Number.isFinite(time) ? time : undefined;
}

export function getScheduleState(entry: ScheduleEntry, now: Date = new Date()): ScheduleState {
  if (entry.verificationStatus === "CANCELLED") return "UNKNOWN";
  const start = validDate(entry.startTime);
  const end = validDate(entry.endTime);
  if (start === undefined || (entry.endTime != null && end === undefined)) return "UNKNOWN";
  const current = now.getTime();
  if (entry.endTime == null) return current < start ? (start - current <= SOON_WINDOW_MS ? "SOON" : "UPCOMING") : "UNKNOWN";
  if (end !== undefined && current >= end) return "ENDED";
  if (current >= start) return "NOW";
  return start - current <= SOON_WINDOW_MS ? "SOON" : "UPCOMING";
}

export function withScheduleState(entry: ScheduleEntry, now?: Date): ScheduleEntry {
  return { ...entry, state: getScheduleState(entry, now) };
}

export function getHappeningNow(entries: ScheduleEntry[], now?: Date): ScheduleEntry[] {
  return entries.map((entry) => withScheduleState(entry, now)).filter((entry) => entry.state === "NOW");
}

export function getUpcoming(entries: ScheduleEntry[], now?: Date): ScheduleEntry[] {
  return entries.map((entry) => withScheduleState(entry, now)).filter((entry) => entry.state === "UPCOMING" || entry.state === "SOON");
}

export function getNextScheduleEntries(entries: ScheduleEntry[], limit = 5, now: Date = new Date()): ScheduleEntry[] {
  return entries
    .map((entry) => withScheduleState(entry, now))
    .filter((entry) => entry.state === "UPCOMING" || entry.state === "SOON")
    .sort((a, b) => Date.parse(a.startTime) - Date.parse(b.startTime))
    .slice(0, limit);
}
