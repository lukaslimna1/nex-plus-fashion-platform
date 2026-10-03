import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import type { CoreHealth } from "@nex-plus/contracts";

type LoadState =
  | { status: "loading" }
  | { status: "ready"; health: CoreHealth }
  | { status: "error"; message: string };

export default function App() {
  const [state, setState] = useState<LoadState>({ status: "loading" });

  useEffect(() => {
    invoke<CoreHealth>("core_health")
      .then((health) => setState({ status: "ready", health }))
      .catch((error: unknown) =>
        setState({
          status: "error",
          message: error instanceof Error ? error.message : String(error),
        }),
      );
  }, []);

  return (
    <main className="shell">
      <p className="eyebrow">NEX+ FASHION · DESKTOP CORE</p>
      <h1>Local-first baseline</h1>
      <p className="lede">
        Tauri 2 shell, typed IPC, and the first SQLite/FTS5 migration boundary.
      </p>
      <section className="status-card" aria-live="polite">
        <span className={`status-dot status-${state.status}`} />
        {state.status === "loading" && "Connecting to Rust Core…"}
        {state.status === "ready" && (
          <>
            Core online · protocol {state.health.protocolVersion} · {state.health.migrationCount} migrations
          </>
        )}
        {state.status === "error" && `Core unavailable: ${state.message}`}
      </section>
      <p className="boundary">
        Milano SS27 remains outside this baseline until startup and compilation are verified.
      </p>
    </main>
  );
}
