/**
 * Single-command playground: starts all travel agents and the personal-agent CLI.
 * Agent logs are prefixed with [agents]; you interact via the CLI (You >).
 */
import { spawn } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.join(__dirname, "..");

function prefixLines(prefix: string, data: Buffer): void {
  const lines = data.toString().split("\n").filter(Boolean);
  for (const line of lines) {
    process.stdout.write(`\x1b[36m${prefix}\x1b[0m ${line}\n`);
  }
}

const agents = spawn("npx", ["tsx", "src/start-services.ts"], {
  cwd: root,
  stdio: ["ignore", "pipe", "pipe"],
  shell: process.platform === "win32",
});

agents.stdout?.on("data", (data: Buffer) => prefixLines("[agents]", data));
agents.stderr?.on("data", (data: Buffer) => prefixLines("[agents]", data));
agents.on("error", (err) => {
  process.stderr.write(`Failed to start agents: ${err.message}\n`);
  process.exit(1);
});
agents.on("exit", (code) => {
  if (code != null && code !== 0 && code !== 143) {
    process.stderr.write(`Agents exited with code ${code}\n`);
  }
});

const cli = spawn("npx", ["tsx", "src/personal-agent.ts"], {
  cwd: root,
  stdio: ["pipe", "pipe", "pipe"],
  shell: process.platform === "win32",
});

cli.stdout?.pipe(process.stdout);
cli.stderr?.pipe(process.stderr);
process.stdin.pipe(cli.stdin!);

cli.on("exit", (code) => {
  agents.kill("SIGTERM");
  process.exit(code ?? 0);
});

agents.on("exit", () => {
  try {
    cli.kill("SIGTERM");
  } catch {
    /* already exited */
  }
});

process.on("SIGINT", () => {
  cli.kill("SIGINT");
});
