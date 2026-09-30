import type { SourceAdapter } from "@nex-plus/types";

export const sourceAdapters: SourceAdapter[] = [
  {
    id: "fhcm",
    provider: "fhcm",
    capabilities: { supportsImages: true, supportsVideo: true, supportsSchedule: true, supportsMetadata: true, supportsEmbed: true, supportsPagination: false }
  },
  {
    id: "vogue-runway",
    provider: "vogue-runway",
    capabilities: { supportsImages: true, supportsVideo: true, supportsSchedule: false, supportsMetadata: true, supportsEmbed: false, supportsPagination: true }
  },
  {
    id: "youtube",
    provider: "youtube",
    capabilities: { supportsImages: false, supportsVideo: true, supportsSchedule: false, supportsMetadata: true, supportsEmbed: true, supportsPagination: true }
  },
  {
    id: "website-gallery",
    provider: "website",
    capabilities: { supportsImages: true, supportsVideo: true, supportsSchedule: false, supportsMetadata: true, supportsEmbed: true, supportsPagination: true }
  }
];

export function getSourceAdapter(id: string): SourceAdapter | undefined {
  return sourceAdapters.find((adapter) => adapter.id === id);
}
