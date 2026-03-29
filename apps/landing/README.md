# Landing page

React (Vite) landing for Pronox. Hero, connect link, roadmap, and "Try the public agent" / Connect page.

## Design system

For consistent visuals and motion, see **THEME.md**. Use the same colors, type scale, spacing, and motion tokens for all new components.

## Roadmap

The live roadmap on the site is driven by `src/data/roadmap.ts`. When you complete tasks in the root **TODO.md**, update `roadmap.ts` (set `done: true` for the corresponding items) so the landing page reflects progress. Optionally add a build-time script later to parse TODO.md and generate this file.

## Run locally

From repo root: `pnpm run landing` (or `npm run landing`). From here: `npm run dev`.

## Deploy (Railway)

Use a **separate Railway service** for this app from **searchd** and **Postgres** (see [Railway monorepo layout](../../docs/railway-architecture.md)). The landing is only the **UI**; search data comes from **`searchd`** over HTTPS.

**Recommended (Docker + pnpm workspace):**

1. **Root Directory:** repository root (`.`), not `apps/landing`.
2. **Config as code:** [`railway.toml`](./railway.toml) in this folder — forces **Dockerfile** build (avoids Railpack `npm ci` on the monorepo). In Railway, set the service config path to `apps/landing/railway.toml` if needed.
3. **Dockerfile** path **`apps/landing/Dockerfile`** (build context = repo root).
4. **Docker build arguments** (or Railway “Build” variables):  
   - `VITE_SEARCH_API_BASE_URL` — public **`searchd`** URL (no trailing slash).  
   - `VITE_PUBLIC_AGENT_URL` — optional; defaults in code if unset.

Local image:

```bash
docker build -f apps/landing/Dockerfile \
  --build-arg VITE_SEARCH_API_BASE_URL=https://your-searchd.up.railway.app \
  -t pronox-landing .
```

The runtime image serves static files with **nginx** (Alpine); Railway’s `PORT` is substituted into the generated config at container start.

The "Try the public agent" button defaults to `https://pronox-public-agent.up.railway.app` when `VITE_PUBLIC_AGENT_URL` is not set at build time.

### Agent search (`/search`, `/agents/:id`)

If **`VITE_SEARCH_API_BASE_URL`** is missing at **build** time, the search page shows a configuration notice. Changing the API URL requires a **rebuild** (Vite inlines env vars).

**Why not Railpack/npm?** This app is a **pnpm workspace** package; the lockfile is `pnpm-lock.yaml` at the repo root. Railpack + root `package-lock.json` caused **`npm ci`** lockfile drift failures. Use **Dockerfile + `railway.toml`** (`builder = DOCKERFILE`) so every deploy matches CI.

See [`apps/agentrank/RAILWAY.md`](../agentrank/RAILWAY.md) for **searchd**, Postgres, and Redis.

## Tests

```bash
pnpm test
```

Runs Vitest (e.g. `SearchPage` smoke).
