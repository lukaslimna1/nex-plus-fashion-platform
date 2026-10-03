# NEX+ Fashion

Clean local-first desktop baseline for the new NEX+ architecture.

The current baseline is intentionally small: Tauri 2, React/TypeScript, Rust Core, SQLite/FTS5 migrations, and a typed IPC contract. Milano Fashion Week SS27 data is not seeded until this baseline has passed its startup and compilation gate.

## Commands

```text
npm install
npm run build
npm test
npm run tauri:dev
```

The Rust checks require a local Rust toolchain (`cargo`, `rustc`, and the Tauri prerequisites).
