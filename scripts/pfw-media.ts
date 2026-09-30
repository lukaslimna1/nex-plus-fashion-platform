export const FHCM_PFW_CALENDAR_URL = "https://www.fhcm.paris/en/paris-fashion-week/calendar";
export const DIOR_VOGUE_SLIDESHOW_URL = "https://www.vogue.com/fashion-shows/spring-2027-ready-to-wear/christian-dior/slideshow/collection";
export const DIOR_VOGUE_VIDEO_URL = "https://www.vogue.com/video/watch/christian-dior-spring-2027-ready-to-wear";

export interface PFWCalendarSnapshotEntry {
  title: string;
  localDate: `${number}-${number}-${number}`;
  localTime: string;
  format: "SHOW" | "PRESENTATION";
  officialUrl: string;
}

const schedule = [
  ["Julie Kegels", "2026-09-28", "14:30", "SHOW"], ["Fidan Novruzova", "2026-09-28", "15:00", "PRESENTATION"], ["KIMHEKIM", "2026-09-28", "15:30", "PRESENTATION"], ["Weinsanto", "2026-09-28", "16:00", "SHOW"], ["Vautrait", "2026-09-28", "16:30", "PRESENTATION"], ["MAXHOSA AFRICA", "2026-09-28", "17:00", "SHOW"], ["Hodakova", "2026-09-28", "18:00", "SHOW"], ["Matières Fécales", "2026-09-28", "19:00", "SHOW"],
  ["Ester Manas", "2026-09-29", "10:00", "SHOW"], ["Alexandra Golovanoff", "2026-09-29", "10:00", "PRESENTATION"], ["GANNI", "2026-09-29", "10:00", "PRESENTATION"], ["Marie Adam-Leenaerdt", "2026-09-29", "11:30", "SHOW"], ["Mame Kurogouchi", "2026-09-29", "13:00", "SHOW"], ["Christian Dior", "2026-09-29", "14:30", "SHOW"], ["Burc Akyol", "2026-09-29", "16:00", "SHOW"], ["Maison Margiela", "2026-09-29", "17:30", "SHOW"], ["Anrealage", "2026-09-29", "19:00", "SHOW"], ["Saint Laurent", "2026-09-29", "21:00", "SHOW"],
  ["Vaillant", "2026-09-30", "09:30", "PRESENTATION"], ["Loulou de Saison", "2026-09-30", "10:00", "PRESENTATION"], ["Courreges", "2026-09-30", "10:30", "SHOW"], ["The Row", "2026-09-30", "12:00", "SHOW"], ["Balmain", "2026-09-30", "13:30", "SHOW"], ["Ruohan", "2026-09-30", "14:00", "PRESENTATION"], ["Dries Van Noten", "2026-09-30", "15:00", "SHOW"], ["Maitrepierre", "2026-09-30", "15:30", "PRESENTATION"], ["Stella McCartney", "2026-09-30", "16:00", "SHOW"], ["Vaquera", "2026-09-30", "17:30", "SHOW"], ["Acne Studios", "2026-09-30", "18:30", "SHOW"], ["Tom Ford", "2026-09-30", "20:00", "SHOW"]
] as const;

export const PFW_SS27_28_30_SCHEDULE: PFWCalendarSnapshotEntry[] = schedule.map(([title, localDate, localTime, format]) => ({ title, localDate, localTime, format, officialUrl: FHCM_PFW_CALENDAR_URL }));

export interface VogueRunwayImage {
  sequenceNumber: number;
  providerAssetId: string;
  remoteUrl: string;
  alternativeUrls: string[];
  thumbnailUrl: string;
  provider: "Vogue Runway";
  creator: "Vogue Runway / Gorunway.com";
  photographer: string;
  creditLine: string;
}

function decodeHtml(value: string): string {
  return value.replaceAll("\\u002F", "/").replaceAll("\\/", "/").replaceAll("&quot;", "\"");
}

export function extractVogueRunwayImages(html: string, filenamePrefix: string, photographer = "Filippo Fior"): VogueRunwayImage[] {
  const decoded = decodeHtml(html);
  const pattern = /https:\/\/assets\.vogue\.com\/photos\/([^/]+)\/master\/w_1600,c_limit\/([^"\\\s]+\.jpg)/g;
  const byProviderAsset = new Map<string, VogueRunwayImage>();
  for (const match of decoded.matchAll(pattern)) {
    const photoId = match[1];
    const filename = match[2];
    if (!photoId || !filename) continue;
    const fileMatch = filename.match(/^(\d+)-(.+\.jpg)$/);
    if (!photoId || !fileMatch || fileMatch[2] !== filenamePrefix) continue;
    const sequenceNumber = Number(fileMatch[1]);
    if (!Number.isSafeInteger(sequenceNumber)) continue;
    const remoteUrl = `https://assets.vogue.com/photos/${photoId}/master/w_1600,c_limit/${filename}`;
    const thumbnailUrl = `https://assets.vogue.com/photos/${photoId}/master/w_360%2Cc_limit/${filename}`;
    const providerAssetId = `${photoId}/${filename}`;
    if (byProviderAsset.has(providerAssetId)) continue;
    byProviderAsset.set(providerAssetId, {
      sequenceNumber,
      providerAssetId,
      remoteUrl,
      alternativeUrls: [`https://assets.vogue.com/photos/${photoId}/master/w_960,c_limit/${filename}`],
      thumbnailUrl,
      provider: "Vogue Runway",
      creator: "Vogue Runway / Gorunway.com",
      photographer,
      creditLine: `${photographer} / Gorunway.com`
    });
  }
  return Array.from(byProviderAsset.values()).sort((left, right) => left.sequenceNumber - right.sequenceNumber || left.providerAssetId.localeCompare(right.providerAssetId));
}

export function toParisIso(localDate: string, localTime: string): string {
  return new Date(`${localDate}T${localTime}:00+02:00`).toISOString();
}
