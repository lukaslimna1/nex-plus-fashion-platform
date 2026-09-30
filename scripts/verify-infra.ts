import { execFile } from "node:child_process";
import { promisify } from "node:util";

const run = promisify(execFile);
try {
  const { stdout, stderr } = await run("npx", ["wrangler", "whoami"], { shell: true });
  const output = `${stdout}\n${stderr}`.replace(/\r?\n/g, " ").trim();
  if (/not authenticated|please run.*wrangler login/i.test(output)) throw new Error(output);
  console.log(JSON.stringify({ wrangler: "authenticated", output }));
} catch (error) {
  const message = error instanceof Error ? error.message : "unknown";
  console.log(JSON.stringify({ wrangler: "not_authenticated_or_unavailable", nextAction: "Run npx wrangler login in an interactive terminal, then rerun npm run verify:infra.", message: message.slice(0, 300) }));
  process.exitCode = 1;
}
