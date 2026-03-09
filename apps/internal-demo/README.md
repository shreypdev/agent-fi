# A2A Travel Itinerary Demo

A demonstration of Google's [Agent-to-Agent (A2A) protocol](https://google.github.io/A2A/) using TypeScript. Five agents collaborate to plan an Italy travel itinerary — **no LLM or API keys required**.

## Architecture

```
You (CLI) ──▶ PersonalAgent ──▶ TravelPlannerAgent ──▶ ExpediaAgent     (flights)
              :3005              :3004                 ──▶ MarriottAgent    (hotels)
                                                       ──▶ RestaurantAgent  (dining)
```

| Agent | Port | Role |
|---|---|---|
| **Expedia Agent** | 3001 | Returns flight options for Italian cities |
| **Marriott Agent** | 3002 | Returns hotel options for Italian cities |
| **Restaurant Agent** | 3003 | Recommends restaurants in Italian cities |
| **Travel Planner Agent** | 3004 | Orchestrator — queries the 3 service agents via A2A and assembles a day-by-day itinerary |
| **Personal Agent** | 3005 | Your personal agent with a CLI. Sends your request to TravelPlannerAgent via A2A |

All agents communicate exclusively over the **A2A protocol** (JSON-RPC 2.0 over HTTP). Each agent exposes an Agent Card at `/.well-known/agent-card.json` describing its capabilities and skills.

## Prerequisites

- Node.js 18+ (tested on 24.x)
- npm

## Setup

From repo root (monorepo):

```bash
npm install
cd apps/internal-demo && npm install
```

Or from this package:

```bash
cd apps/internal-demo
npm install
```

## Running the Demo

**Option A — One command (recommended):** From repo root, run the playground. It starts all travel agents and the personal-agent CLI in one session; agent logs are prefixed with `[agents]`, you type at `You >`:

```bash
npm run playground
```

**Option B — Two terminals:** Start agents, then the CLI:

- **Terminal 1** (from root or this directory): `npm run services`
- **Terminal 2:** `cd apps/internal-demo && npm run cli`

Then type a travel planning request:

```
You > Plan a 5-day Italy trip visiting Rome, Florence, and Venice
```

Your PersonalAgent forwards the request to TravelPlannerAgent, which fans out A2A requests to Expedia, Marriott, and Restaurant agents, then assembles and returns a complete itinerary.

### Other commands

- `discover` — queries each agent's A2A Agent Card and prints their name, description, and skills
- `quit` — exit the CLI

## Supported Cities

Rome, Florence, Venice, Milan

## Running alongside the public agent

The **public agent** (echo agent on port 3010) is in `apps/public-agent`. From root: `npm run public:agent` to start it (e.g. for hosting or ngrok). You can run the playground in one terminal and the public agent in another.

See the root [README](../../README.md) and [TODO.md](../../TODO.md) for the full roadmap.

## What This Demonstrates

1. **A2A Agent Cards** — each agent publishes a JSON card describing its skills
2. **A2A Message Passing** — agents send/receive `message/send` JSON-RPC requests
3. **Multi-hop Communication** — PersonalAgent → TravelPlannerAgent → 3 service agents
4. **Agent as Both Server and Client** — TravelPlannerAgent serves incoming requests while also acting as an A2A client to other agents
5. **No LLM** — all logic is deterministic; A2A is the communication layer

## Project Structure

```
src/
├── config.ts                     Ports and shared constants
├── helpers.ts                    Shared A2A server/event utilities
├── data/
│   ├── flights.ts                Hardcoded flight catalog
│   ├── hotels.ts                 Hardcoded hotel catalog
│   └── restaurants.ts            Hardcoded restaurant catalog
├── agents/
│   ├── expedia-agent.ts          ExpediaAgent (port 3001)
│   ├── marriott-agent.ts         MarriottAgent (port 3002)
│   ├── restaurant-agent.ts       RestaurantAgent (port 3003)
│   └── travel-planner-agent.ts   TravelPlannerAgent orchestrator (port 3004)
├── personal-agent.ts             PersonalAgent CLI (port 3005)
└── start-services.ts             Launches all 4 service agents
```

## Tech Stack

- TypeScript + Node.js
- [`@a2a-js/sdk`](https://github.com/a2aproject/a2a-js) — official A2A JS/TS SDK
- Express — HTTP server
- tsx — run TypeScript directly without a build step
