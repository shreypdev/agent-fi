# AgentFi

**The governance layer for the agent economy** — observability, identity, and trust for multi-agent systems.

See [PRD.md](PRD.md) and [MILESTONES.md](MILESTONES.md) for vision and roadmap. Track progress in [TODO.md](TODO.md).

## Monorepo structure

| Path | Purpose |
|------|--------|
| `apps/agentrank` | **AgentRank (Rust)** — data-plane crates, `healthd`, SQL migrations (`migrations/`). See [apps/agentrank/README.md](apps/agentrank/README.md). |
| `apps/ml-pipeline` | **Offline ML (Python)** — embeddings / LTR / eval (not on the hot path). See [apps/ml-pipeline/README.md](apps/ml-pipeline/README.md). |
| `apps/dev` | **Local data plane** — Docker Compose for PostgreSQL + Redis. See [apps/dev/README.md](apps/dev/README.md). |
| `apps/internal-demo` | Internal multi-agent testbed — 5 A2A travel agents (Expedia, Marriott, Restaurant, TravelPlanner, Personal). Used for SDK development and demos. |
| `apps/landing` | Marketing / landing site (Vite + React). |
| `apps/public-agent` | Public demo agent — echo skill, REST bridge, OpenAPI. Port 3010. Expose via ngrok for ChatGPT/Claude/Cursor. |
| `packages/` | (Week 2+) `common`, `trace-sdk`, `shield-sdk`, `collector`, `registry`, `dashboard`. |

## Prerequisites

- **Node.js 18+** (required by `@a2a-js/sdk` and tsx)
- **pnpm 9+** (declared in root `packageManager`; enables Turbo and workspace scripts — run `corepack enable` then `pnpm install` at repo root)
- **Rust** (stable, pin in [apps/agentrank/rust-toolchain.toml](apps/agentrank/rust-toolchain.toml)) for AgentRank services
- **Docker** (optional but recommended) for PostgreSQL + Redis via [apps/dev/docker-compose.yml](apps/dev/docker-compose.yml)

### Node version (automatic per repo)

This repo uses **Node 18**. Pick one approach:

| Tool | Automatic? | Setup |
|------|------------|--------|
| **[Volta](https://volta.sh)** | Yes — uses Node 18 whenever you run `node`/`npm` in this repo (no `cd` hook) | Install [volta.sh](https://volta.sh), then run any command; Volta reads `package.json` → `volta.node` |
| **fnm** | Yes — if you enable use-on-cd | `brew install fnm`, add `eval "$(fnm env --use-on-cd)"` to `~/.zshrc`, then `fnm install 18`. Uses [.nvmrc](.nvmrc) when you `cd` here |
| **nvm** | No — run once per shell | `nvm use` (reads [.nvmrc](.nvmrc)) |
| — | No | Download Node 18 LTS from [nodejs.org](https://nodejs.org/) |

**Recommended:** Volta for zero-config automatic switching. Otherwise fnm + `--use-on-cd` for auto-switch on `cd`.

Confirm: `node -v` → v18.x.x.

## Install

From repo root (workspaces install all apps):

```bash
corepack enable
pnpm install
```

Legacy per-app installs with npm still work for individual folders if needed, but CI and Turbo expect **pnpm** at the root.

After the first `pnpm install`, commit the generated `pnpm-lock.yaml` so CI caching and reproducible installs stay consistent.

## Run

### Playground (internal demo — one command)

Starts all travel agents and the personal-agent CLI in one interactive session. Agent activity is prefixed with `[agents]`; you chat via `You >`:

```bash
pnpm run playground
```

Then try: `Plan a 5-day Italy trip visiting Rome, Florence, and Venice`, or `discover`, or `quit`.

### Public agent (for hosting)

Starts the public demo agent on port 3010 (echo, REST bridge, OpenAPI). Use this when you want to expose the agent (e.g. via ngrok):

```bash
pnpm run public:agent
```

- A2A: Agent Card at `http://localhost:3010/.well-known/agent-card.json`
- REST: `curl "http://localhost:3010/api/chat?message=Hello"`
- OpenAPI: `http://localhost:3010/openapi.json`

### Optional: run agents and CLI separately

- **Terminal 1:** `pnpm run services` — travel agents only (3001–3004)
- **Terminal 2:** `cd apps/internal-demo && npm run cli` — personal agent CLI

See [apps/internal-demo/README.md](apps/internal-demo/README.md) and [apps/public-agent/README.md](apps/public-agent/README.md) for details and ngrok setup.

## Scripts (root)

| Script | Description |
|--------|-------------|
| `pnpm run playground` | Internal demo: travel agents + personal CLI in one interactive session |
| `pnpm run public:agent` | Public agent on port 3010 (for hosting) |
| `pnpm run services` | Travel agents only (no CLI) |
| `pnpm run build` | Turbo build (all packages that define `build`) |

## AgentRank Week 1 — data plane

- Copy [.env.example](.env.example) → `.env` for local URLs.
- Start Postgres + Redis: `docker compose -f apps/dev/docker-compose.yml up -d`
- Apply migrations: see [apps/agentrank/README.md](apps/agentrank/README.md)
- Health probe: `cargo run -p agentrank-healthd --bin healthd` (from `apps/agentrank`)

Architecture reference: [a2a-discovery-opus.md](a2a-discovery-opus.md); execution checklist: [a2a-discovery-todo.md](a2a-discovery-todo.md).

## Week 1 status (AgentFi demos)

- [x] Monorepo setup; `agent-demo` → `apps/internal-demo`; `apps/public-agent` scaffolded
- [x] 1.1 Public echo agent + Agent Card
- [x] 1.2 REST bridge + OpenAPI
- [x] 1.7 Internal test harness documented
- [x] AgentRank Rust workspace + schema v1 migrations + `healthd` + Compose + CI

See [TODO.md](TODO.md) for the full 16-week checklist.
