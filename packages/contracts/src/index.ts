export const CORE_PROTOCOL_VERSION = "0.1" as const;

export const BASELINE_MIGRATIONS = ["0001_core", "0002_fts5"] as const;

export type EntityKind =
  | "city_hub"
  | "event"
  | "edition"
  | "schedule_entry"
  | "venue"
  | "maison"
  | "person"
  | "collection"
  | "source"
  | "media_asset"
  | "review"
  | "personal_note";

export interface CoreHealth {
  status: "ok";
  protocolVersion: typeof CORE_PROTOCOL_VERSION;
  storage: "sqlite";
  fts5: "ready";
  migrationCount: number;
}

export interface IpcError {
  code: string;
  message: string;
}

export interface IpcResponse<T> {
  ok: boolean;
  data?: T;
  error?: IpcError;
}

export function isEntityKind(value: unknown): value is EntityKind {
  return typeof value === "string" && [
    "city_hub",
    "event",
    "edition",
    "schedule_entry",
    "venue",
    "maison",
    "person",
    "collection",
    "source",
    "media_asset",
    "review",
    "personal_note",
  ].includes(value);
}
