import { describe, expect, it } from "vitest";
import { getHappeningNow, getNextScheduleEntries, getScheduleState } from "../src/schedule.js";
import type { ScheduleEntry } from "@nex-plus/types";

const base: ScheduleEntry = {
  id: "schedule-1", editionId: "edition-1", eventId: "pfw", cityHubId: "paris", title: "Julie Kegels",
  format: "SHOW", startTime: "2026-09-28T12:30:00.000Z", endTime: "2026-09-28T13:30:00.000Z", timezone: "Europe/Paris",
  verificationStatus: "VERIFIED", sourceIds: ["fhcm"]
};

describe("deterministic schedule state", () => {
  it("keeps calendar year and season year independent", () => {
    expect(getScheduleState(base, new Date("2026-09-28T12:00:00Z"))).toBe("SOON");
  });
  it("derives now and ended from the schedule boundaries", () => {
    expect(getScheduleState(base, new Date("2026-09-28T12:45:00Z"))).toBe("NOW");
    expect(getScheduleState(base, new Date("2026-09-28T14:00:00Z"))).toBe("ENDED");
    expect(getHappeningNow([base], new Date("2026-09-28T12:45:00Z"))).toHaveLength(1);
  });
  it("orders only upcoming entries", () => {
    const next = { ...base, id: "schedule-2", startTime: "2026-09-28T15:00:00Z" };
    expect(getNextScheduleEntries([next, base], 2, new Date("2026-09-28T12:00:00Z")).map((x) => x.id)).toEqual(["schedule-1", "schedule-2"]);
  });
  it("does not invent an ended time when the source omitted an end", () => {
    const openEnded = { ...base, endTime: undefined };
    expect(getScheduleState(openEnded, new Date("2026-09-28T14:00:00Z"))).toBe("UNKNOWN");
  });
});
