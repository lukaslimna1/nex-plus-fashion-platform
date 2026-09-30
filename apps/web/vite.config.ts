import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@nex-plus/ui/styles.css": path.resolve(__dirname, "../../packages/ui/src/styles/index.css"),
      "@nex-plus/ui": path.resolve(__dirname, "../../packages/ui/src/index.ts"),
      "@nex-plus/types": path.resolve(__dirname, "../../packages/types/src/index.ts"),
    },
  },
  server: {
    port: 3000,
  },
});
