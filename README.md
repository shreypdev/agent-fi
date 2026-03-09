# AgentFi

**The governance layer for the agent economy** — observability, identity, and trust for multi-agent systems.

See [PRD.md](PRD.md) and [MILESTONES.md](MILESTONES.md) for vision and roadmap. Track progress in [TODO.md](TODO.md).

## Monorepo structure

| Path | Purpose |
|------|--------|
| `apps/internal-demo` | Internal multi-agent testbed — 5 A2A travel agents (Expedia, Marriott, Restaurant, TravelPlanner, Personal). Used for SDK development and demos. |
| `apps/public-agent` | Public demo agent — echo skill, REST bridge, OpenAPI. Port 3010. Expose via ngrok for ChatGPT/Claude/Cursor. |
| `packages/` | (Week 2+) `common`, `trace-sdk`, `shield-sdk`, `collector`, `registry`, `dashboard`. |

## Prerequisites

- **Node.js 18+** (required by `@a2a-js/sdk` and tsx)
- npm (or pnpm; see [pnpm-workspace.yaml](pnpm-workspace.yaml))

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

From repo root:

```bash
npm install
cd apps/internal-demo && npm install
cd ../public-agent && npm install
```

Or install in each app as needed.

## Run

### Playground (internal demo — one command)

Starts all travel agents and the personal-agent CLI in one interactive session. Agent activity is prefixed with `[agents]`; you chat via `You >`:

```bash
npm run playground
```

Then try: `Plan a 5-day Italy trip visiting Rome, Florence, and Venice`, or `discover`, or `quit`.

### Public agent (for hosting)

Starts the public demo agent on port 3010 (echo, REST bridge, OpenAPI). Use this when you want to expose the agent (e.g. via ngrok):

```bash
npm run public:agent
```

- A2A: Agent Card at `http://localhost:3010/.well-known/agent-card.json`
- REST: `curl "http://localhost:3010/api/chat?message=Hello"`
- OpenAPI: `http://localhost:3010/openapi.json`

### Optional: run agents and CLI separately

- **Terminal 1:** `npm run services` — travel agents only (3001–3004)
- **Terminal 2:** `cd apps/internal-demo && npm run cli` — personal agent CLI

See [apps/internal-demo/README.md](apps/internal-demo/README.md) and [apps/public-agent/README.md](apps/public-agent/README.md) for details and ngrok setup.

## Scripts (root)

| Script | Description |
|--------|-------------|
| `npm run playground` | Internal demo: travel agents + personal CLI in one interactive session |
| `npm run public:agent` | Public agent on port 3010 (for hosting) |
| `npm run services` | Travel agents only (no CLI) |
| `npm run build` | Turbo build (all packages that define `build`) |

## Week 1 status

- [x] Monorepo setup; `agent-demo` → `apps/internal-demo`; `apps/public-agent` scaffolded
- [x] 1.1 Public echo agent + Agent Card
- [x] 1.2 REST bridge + OpenAPI
- [x] 1.7 Internal test harness documented

See [TODO.md](TODO.md) for the full 16-week checklist.
