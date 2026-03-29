# Railway — unified demo (Postgres + Redis + searchd + landing)

Goal: one Railway **project** with catalog data in Postgres, a **Tantivy** index on disk, **`searchd`** serving `POST /v1/search`, and the **landing** static app calling that API via `VITE_SEARCH_API_BASE_URL`.

## Services (recommended)

| Service    | Template        | Purpose |
| ---------- | --------------- | ------- |
| Postgres   | Railway Postgres | Source of truth; run `sqlx migrate` |
| Redis      | Railway Redis or Upstash | Rate limits for `searchd` |
| searchd    | **Docker** from this repo (`apps/agentrank/Dockerfile`) | Search API |
| Landing    | Static (existing) | Set `VITE_SEARCH_API_BASE_URL` to searchd public URL |

## Environment variables

**searchd (container):**

- `DATABASE_URL` — reference Postgres plugin.
- `REDIS_URL` — reference Redis / Upstash.
- `SEARCH_INDEX_PATH` — default `/tmp/agentrank-index` in the Docker image (ephemeral; rebuilt each deploy via entrypoint).
- `PORT` — Railway injects automatically; `searchd` listens on `PORT`.
- Optional: `CORS_ORIGINS` (comma-separated); if empty, CORS allows any origin (demo only).
- Optional: `SEARCH_RATE_LIMIT_PER_MINUTE` (default `120`).
- Optional: `SEARCHD_BOOT_MIGRATE=0` — skip `sqlx migrate` on start (if you run migrations in a Release Command).
- Optional: `SEARCHD_BOOT_REBUILD_INDEX=0` — skip index rebuild on start (only if the index is pre-provisioned elsewhere).

The [`Dockerfile`](./Dockerfile) **entrypoint** runs `sqlx migrate`, `agentrank-index rebuild`, then `searchd`, so a single Railway Docker service can boot end-to-end without a separate release script.

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
