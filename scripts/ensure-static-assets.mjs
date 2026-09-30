import { mkdir } from "node:fs/promises";
await mkdir(new URL("../apps/web/dist/", import.meta.url), { recursive: true });
