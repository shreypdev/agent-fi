# Landing page

React (Vite) landing for Pronox. Hero, connect link, roadmap, and "Try the public agent" / Connect page.

## Design system

For consistent visuals and motion, see **THEME.md**. Use the same colors, type scale, spacing, and motion tokens for all new components.

## Roadmap

The live roadmap on the site is driven by `src/data/roadmap.ts`. When you complete tasks in the root **TODO.md**, update `roadmap.ts` (set `done: true` for the corresponding items) so the landing page reflects progress. Optionally add a build-time script later to parse TODO.md and generate this file.

## Run locally

From repo root: `pnpm run landing` (or `npm run landing`). From here: `npm run dev`.

## Deploy (Railway)

The "Try the public agent" button defaults to `https://pronox-public-agent.up.railway.app`. To point it at a custom domain later, set **Variables** on the Landing service: `VITE_PUBLIC_AGENT_URL` = your public agent URL, then redeploy. The Connect page and "Try in Browser" widget use this URL.

### Agent search (`/search`, `/agents/:id`)

Set **`VITE_SEARCH_API_BASE_URL`** to your public **`searchd`** base URL (HTTPS, **no trailing slash**), e.g. `https://agentrank-searchd.up.railway.app`. Rebuild the static site after changing it (Vite bakes env at build time).

If unset, the search page explains that the API URL is missing (safe for landing-only deploys).

See [`apps/agentrank/RAILWAY.md`](../agentrank/RAILWAY.md) for Postgres + Redis + index + `searchd` on one Railway project.

## Tests

```bash
pnpm test
```

Runs Vitest (e.g. `SearchPage` smoke).
