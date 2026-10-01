import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { extname, join, resolve } from "node:path";

interface ManifestImage {
  kind: "IMAGE";
  title: string;
  sourcePageUrl: string;
  remoteUrl: string;
  thumbnailUrl: string;
  alternativeUrls: string[];
  sequenceNumber: number;
  deduplicationKey: string;
}

interface ManifestCollection {
  name: string;
  localDate: string;
  images: ManifestImage[];
  coverAsset?: ManifestImage;
}

interface Manifest {
  schemaVersion: string;
  source: string;
  retrievedAt: string;
  collections: ManifestCollection[];
}

interface DownloadRecord {
  collection: string;
  localDate: string;
  title: string;
  sourcePageUrl: string;
  remoteUrl: string;
  alternativeUrls: string[];
  selectedUrl?: string;
  selectedKind?: "remote" | "alternative" | "thumbnail";
  localPath?: string;
  bytes?: number;
  sha256?: string;
  status: "DOWNLOADED" | "FAILED";
  error?: string;
}

function argument(name: string): string | undefined {
  const index = process.argv.indexOf(name);
  return index >= 0 ? process.argv[index + 1] : undefined;
}

function slug(value: string): string {
  return value.normalize("NFKD").replace(/[\u0300-\u036f]/g, "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
}

export function mediaCandidates(asset: Pick<ManifestImage, "remoteUrl" | "alternativeUrls" | "thumbnailUrl">): Array<{ url: string; kind: "remote" | "alternative" | "thumbnail" }> {
  const values = [
    { url: asset.remoteUrl, kind: "remote" as const },
    ...asset.alternativeUrls.map((url) => ({ url, kind: "alternative" as const })),
    { url: asset.thumbnailUrl, kind: "thumbnail" as const }
  ];
  const seen = new Set<string>();
  return values.filter((candidate) => {
    if (!candidate.url || seen.has(candidate.url)) return false;
    seen.add(candidate.url);
    return true;
  });
}

function extension(url: string, contentType: string | null): string {
  const fromType = contentType?.split("/")[1]?.split(";")[0]?.toLowerCase();
  if (fromType === "jpeg") return ".jpg";
  if (fromType && /^[a-z0-9]+$/.test(fromType)) return `.${fromType}`;
  const fromUrl = extname(new URL(url).pathname).toLowerCase();
  return fromUrl || ".bin";
}

async function downloadAsset(asset: ManifestImage, collection: ManifestCollection, outDir: string): Promise<DownloadRecord> {
  const record: DownloadRecord = {
    collection: collection.name,
    localDate: collection.localDate,
    title: asset.title,
    sourcePageUrl: asset.sourcePageUrl,
    remoteUrl: asset.remoteUrl,
    alternativeUrls: asset.alternativeUrls,
    status: "FAILED"
  };
  let lastError = "no candidate URL";
  for (const candidate of mediaCandidates(asset)) {
    try {
      const response = await fetch(candidate.url, { headers: { "user-agent": "NEX+ Fashion staging downloader/1.0" }, signal: AbortSignal.timeout(30000) });
      if (!response.ok) { lastError = `HTTP ${response.status}`; continue; }
      const contentType = response.headers.get("content-type");
      if (!contentType?.toLowerCase().startsWith("image/")) { lastError = `unexpected content-type ${contentType ?? "missing"}`; continue; }
      const bytes = Buffer.from(await response.arrayBuffer());
      const collectionDir = join(outDir, slug(collection.name));
      await mkdir(collectionDir, { recursive: true });
      const prefix = asset.sequenceNumber === 0 ? "cover" : String(asset.sequenceNumber).padStart(3, "0");
      const digest = createHash("sha256").update(asset.deduplicationKey).digest("hex").slice(0, 12);
      const filePath = join(collectionDir, `${prefix}-${digest}${extension(candidate.url, contentType)}`);
      await writeFile(filePath, bytes);
      record.selectedUrl = candidate.url;
      record.selectedKind = candidate.kind;
      record.localPath = filePath;
      record.bytes = bytes.byteLength;
      record.sha256 = createHash("sha256").update(bytes).digest("hex");
      record.status = "DOWNLOADED";
      return record;
    } catch (error) {
      lastError = error instanceof Error ? error.message : String(error);
    }
  }
  record.error = lastError;
  return record;
}

async function main(): Promise<void> {
  const manifestPath = argument("--manifest");
  const outputPath = argument("--out");
  if (!manifestPath || !outputPath || !process.argv.includes("--staging-only")) {
    throw new Error("Usage: tsx scripts/download-fhcm-media.ts --manifest <json> --out <ignored staging dir> --staging-only");
  }
  const manifest = JSON.parse(await readFile(resolve(manifestPath), "utf8")) as Manifest;
  const outDir = resolve(outputPath);
  await mkdir(outDir, { recursive: true });
  const assets = manifest.collections.flatMap((collection) => [
    ...collection.images.map((asset) => ({ collection, asset })),
    ...(collection.coverAsset ? [{ collection, asset: collection.coverAsset }] : [])
  ]);
  const records: DownloadRecord[] = [];
  for (let index = 0; index < assets.length; index += 12) {
    const batch = assets.slice(index, index + 12);
    records.push(...await Promise.all(batch.map(({ collection, asset }) => downloadAsset(asset, collection, outDir))));
    console.log(`downloaded ${Math.min(index + batch.length, assets.length)}/${assets.length}`);
  }
  const report = { schemaVersion: "pfw-ss27-fhcm-download-report-v1", stagingOnly: true, source: manifest.source, retrievedAt: manifest.retrievedAt, manifestPath: resolve(manifestPath), outputDirectory: outDir, total: records.length, downloaded: records.filter((record) => record.status === "DOWNLOADED").length, failed: records.filter((record) => record.status === "FAILED").length, records };
  await writeFile(join(outDir, "download-report.json"), `${JSON.stringify(report, null, 2)}\n`, "utf8");
  console.log(JSON.stringify({ total: report.total, downloaded: report.downloaded, failed: report.failed, outputDirectory: report.outputDirectory }));
}

if (process.argv[1]?.replace(/[\\/]+$/, "").endsWith("download-fhcm-media.ts")) await main();
