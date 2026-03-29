# Railway — monorepo services (how pieces connect)

This repo is a **monorepo**. On Railway you run **separate services** (separate deployables) in **one project**. They do **not** share a process; they talk over the **public network** (HTTPS URLs) or **private networking** if you enable it.

## Mental model

| Service (Railway) | What it is | How others reach it |
| ----------------- | ---------- | ------------------- |
| **Landing** | React/Vite **static UI** (`apps/landing`) | Users’ browsers |
| **searchd** | Rust **search API** + Tantivy (`apps/agentrank` Docker) | Browser calls `VITE_SEARCH_API_BASE_URL`; other clients call `POST /v1/search` |
| **Public agent** | Your **demo A2A agent** (`apps/public-agent`) | `VITE_PUBLIC_AGENT_URL`, widgets, MCP clients |
| **Postgres** | Plugin | `DATABASE_URL` on **searchd** (and any job that migrates/ingests) |
| **Redis** | Plugin or Upstash | `REDIS_URL` on **searchd** |

Search **UI** lives in **Landing** because it is HTML/JS. Search **data and ranking** live in **searchd** + Postgres + index. That split is normal: **frontend service** vs **API service**.

## Landing service (pnpm, not npm)

- **Root Directory:** repository **root** (`.`). Do **not** set Root to `apps/landing` only — `pnpm-lock.yaml` and the workspace live at the root.
- **Config as code:** [`apps/landing/railway.toml`](../apps/landing/railway.toml) — sets `builder = DOCKERFILE` and `dockerfilePath = apps/landing/Dockerfile` so deploys do not use Railpack/`npm ci`.
- **Dockerfile path:** `apps/landing/Dockerfile` (same as in that config)
- **Build args / env (baked into the bundle):**
  - `VITE_SEARCH_API_BASE_URL` — public HTTPS URL of **searchd** (no trailing slash)
  - `VITE_PUBLIC_AGENT_URL` — optional; public agent URL for Connect / Try in browser

The repository root does **not** use `package-lock.json` for the workspace (pnpm is canonical). A root `package-lock.json` caused Railpack to run **`npm ci`** against an unsynced lockfile; removing it and pinning **Dockerfile** in `railway.toml` avoids that failure mode.

## searchd service

- **Root Directory:** repository **root**
- **Config as code:** [`apps/agentrank/railway.toml`](../apps/agentrank/railway.toml) — `builder = DOCKERFILE`, `dockerfilePath = apps/agentrank/Dockerfile`
- **Dockerfile path:** `apps/agentrank/Dockerfile`
- **Runtime env:** `DATABASE_URL`, `REDIS_URL`, `SEARCH_INDEX_PATH`, `PORT` (Railway sets `PORT`), `CORS_ORIGINS`, **`CORS_REQUIRE_ORIGINS=1`** (prod), optional **`TRUST_PROXY_HEADERS=0`** to disable trusting `X-Forwarded-For` on Railway, optional **`METRICS_BEARER_TOKEN`** to lock down `GET /metrics`, optional `SEARCHD_INDEX_BOOT=reuse` with a **volume** on `SEARCH_INDEX_PATH`
- **Probes:** `GET /health` = liveness; **`GET /ready`** = readiness (DB + Redis + index). Prefer `/ready` for “only route traffic when search works.”
- **Landing → API:** the browser calls **`VITE_SEARCH_API_BASE_URL`** (build-time on Landing) — must be the public **HTTPS** URL of searchd (no trailing slash).

## Single prod vs staging later

For **one production** project today: configure variables and build args as above. After go-live, when you add **staging**, duplicate services with separate URLs and secrets (second Railway project or environment); no repo change required beyond different `VITE_*` and `DATABASE_URL` values.

## Public agent service

Keep its own **Root Directory** (e.g. `apps/public-agent`) and existing build/start, or Docker if you add one — **independent** from Landing.

## Operations note

The searchd Docker **entrypoint** exits on migrate or index errors before starting the server (fail-fast). Monitor Railway deploy logs and set notifications so failed releases are visible.

## Scaling later

- **More traffic on search:** scale **searchd** replicas; use a **persistent volume** or external object store for the Tantivy directory if you stop rebuilding every deploy (`SEARCHD_INDEX_BOOT=reuse`).
- **More apps:** add services (e.g. admin, ingest workers) with their own Dockerfile or build; share **Postgres** via `DATABASE_URL` only where appropriate.
- **Preview environments:** branch deploys per service; point preview Landing **build args** at a preview **searchd** URL.

## References

- [apps/landing/README.md](../apps/landing/README.md) — Landing env and Docker build
- [apps/agentrank/RAILWAY.md](../apps/agentrank/RAILWAY.md) — searchd, Postgres, Redis
