# Railway — unified demo (Postgres + Redis + searchd + landing)

Goal: one Railway **project** with **multiple services** (see [docs/railway-architecture.md](../../docs/railway-architecture.md)): catalog in Postgres, **Tantivy** on disk, **`searchd`** for `POST /v1/search`, and a **separate Landing** service that calls search over HTTPS via `VITE_SEARCH_API_BASE_URL` (build-time).

## Config as code (recommended)

Point each service at the matching file so builds do **not** fall through to Railpack/npm:

| Service  | Config file | Root directory |
| -------- | ----------- | -------------- |
| Landing  | [`apps/landing/railway.toml`](../landing/railway.toml) | Repository **root** `.` |
| searchd  | [`railway.toml`](./railway.toml) (this app) | Repository **root** `.` |

In Railway: **Settings → Config as code** → path above (if not auto-detected). The repo root no longer ships a root `package-lock.json` (pnpm is canonical); Landing must use **Dockerfile** + pnpm in the image.

## Services (recommended)

| Service    | Template        | Purpose |
| ---------- | --------------- | ------- |
| Postgres   | Railway Postgres | Source of truth; run `sqlx migrate` |
| Redis      | Railway Redis or Upstash | Rate limits for `searchd` |
| searchd    | **Docker** from this repo (`apps/agentrank/Dockerfile`) | Search API |
| Landing    | **Docker** from `apps/landing/Dockerfile` | Set `VITE_SEARCH_API_BASE_URL` to searchd public URL |

## Environment variables

**searchd (container):**

- `DATABASE_URL` — reference Postgres plugin.
- `REDIS_URL` — reference Redis / Upstash.
- `SEARCH_INDEX_PATH` — default `/tmp/agentrank-index` in the Docker image (ephemeral unless you attach a **volume**; see below).
- `PORT` — Railway injects automatically; `searchd` listens on `PORT`.
- **`TRUST_PROXY_HEADERS`** — optional override. On Railway, `X-Forwarded-For` / `X-Real-IP` are **trusted by default** when `RAILWAY_ENVIRONMENT` or `RAILWAY_PROJECT_ID` is set (typical deploy). Set **`TRUST_PROXY_HEADERS=0`** to use only the TCP peer (rare). Set **`1`** to force trust when not on Railway.
- `CORS_ORIGINS` — comma-separated allowed browser origins (e.g. your landing `https://….up.railway.app`). Empty allows any origin (demo only).
- **`CORS_REQUIRE_ORIGINS=1`** — production: fail startup if `CORS_ORIGINS` is empty or invalid (prevents accidental wide-open CORS).
- Optional: `SEARCH_RATE_LIMIT_PER_MINUTE` (default `120`).
- Optional: `SEARCHD_BOOT_MIGRATE=0` — skip `sqlx migrate` on start (if you run migrations in a Release Command).
- Optional: `SEARCHD_BOOT_REBUILD_INDEX=0` — skip index rebuild on start (only if the index is pre-provisioned elsewhere).
- Optional: **`SEARCHD_INDEX_BOOT=reuse`** — when rebuild is enabled, skip `rebuild` if `agentrank-index probe` succeeds (pair with a **persistent volume** mounted at `SEARCH_INDEX_PATH`, e.g. `/data/agentrank-index`). Default `full` rebuilds every start.

**Health checks:** `GET /health` is **liveness** (process only). **`GET /ready`** is **readiness** (Postgres + Redis + Tantivy index on disk). Point Railway health checks at `/ready` if you only want traffic when search can succeed.

**Metrics:** `GET /metrics` — Prometheus text. Set **`METRICS_BEARER_TOKEN`** (non-empty) to require `Authorization: Bearer <token>`; leave unset for open access (local/demo only).

**Index volume (recommended for prod):** Mount a Railway volume on `SEARCH_INDEX_PATH`, set `SEARCHD_INDEX_BOOT=reuse`, and run a full rebuild after changing [`INDEX_VERSION`](../crates/search-index/src/schema.rs) or when you need a guaranteed full reindex. **Multi-replica:** each instance can use its own volume + same boot policy, or share a read-only index from a single writer job (no concurrent writers to the same Tantivy directory).

**Operations:** The entrypoint is **fail-fast** — if migrate or rebuild fails, the container exits before `searchd` starts. Watch Railway deploy logs and alerts; a bad migration should not go unnoticed.

The [`Dockerfile`](./Dockerfile) **entrypoint** runs `sqlx migrate`, index boot (`rebuild` or `reuse` probe), then `searchd`, so a single Railway Docker service can boot end-to-end without a separate release script.

**Landing (build):**

- `VITE_SEARCH_API_BASE_URL` — public HTTPS URL of `searchd`.

## Release / deploy commands

**Default (Docker image):** nothing extra — the container entrypoint runs migrations and rebuilds the index before `searchd` starts.

**Optional split:** set `SEARCHD_BOOT_MIGRATE=0` and `SEARCHD_BOOT_REBUILD_INDEX=0`, then use a Railway **Release Command** or one-off:

```bash
sqlx migrate run --source migrations
agentrank-index rebuild --output "$SEARCH_INDEX_PATH"
```

**Demo data:** migration `20260328200000_demo_seed_agents.sql` adds two agents idempotently; the entrypoint rebuilds the index so search returns them.

**Ingest real cards:** one-off `agentbot ingest <url>` (see `apps/agentrank/README.md`), then `agentrank-index upsert` or full `rebuild`.

## Cost notes

If Railway Postgres/Redis are too costly at idle, use **Neon** + **Upstash** and keep only `searchd` + landing on Railway; connection strings swap straight in.

## GitHub → Railway

Either connect each Railway service to the repo (push to `master`), or add a GitHub Actions workflow with `RAILWAY_TOKEN` and the Railway CLI for explicit deploy logs.
