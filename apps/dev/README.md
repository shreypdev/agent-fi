# Local dev stack (PostgreSQL + Redis)

Docker Compose file for AgentRank **Week 1** data plane. Matches the service layout used in CI (official Postgres + Redis images, healthchecks, `DATABASE_URL` / `REDIS_URL`).

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) with Compose v2

## Start

From repo root:

```bash
docker compose -f apps/dev/docker-compose.yml up -d
```

Wait until both services are **healthy**:

```bash
docker compose -f apps/dev/docker-compose.yml ps
```

## Ports and defaults (development only)

| Service    | Port | Notes                                      |
| ---------- | ---- | ------------------------------------------ |
| PostgreSQL | **5433** (host) → 5432 in container | User / password / DB: `agentrank` |
| Redis      | 6379 | No password                                |

Postgres is published on **5433** on the host so it does not collide with a system Postgres on **5432** (if `sqlx` or `healthd` reports `role "agentrank" does not exist`, you are usually hitting the wrong server—check `lsof -i :5432` and that `DATABASE_URL` uses port **5433**).

Connection strings for local tools (see root `.env.example`):

- `DATABASE_URL=postgresql://agentrank:agentrank@127.0.0.1:5433/agentrank`
- `REDIS_URL=redis://127.0.0.1:6379`

## Healthchecks

- **Postgres:** `pg_isready -U agentrank -d agentrank` — same idea as CI `pg_isready` on the server.
- **Redis:** `redis-cli ping` — expects `PONG`.

## Migrations

With Compose running:

```bash
cd apps/agentrank
export DATABASE_URL=postgresql://agentrank:agentrank@127.0.0.1:5433/agentrank
sqlx migrate run --source migrations
```

Install `sqlx-cli` once: `cargo install sqlx-cli --no-default-features --features rustls,postgres`.

## Health probe (`healthd`)

After migrations (optional — `healthd` only needs reachable PG and Redis):

```bash
cd apps/agentrank
export DATABASE_URL=postgresql://agentrank:agentrank@127.0.0.1:5433/agentrank
export REDIS_URL=redis://127.0.0.1:6379
cargo run -p agentrank-healthd --bin healthd
```

Exit code `0` means PostgreSQL answered `SELECT 1` and Redis answered `PING`.

## Stop / reset

```bash
docker compose -f apps/dev/docker-compose.yml down
```

Remove volumes (wipes DB data):

```bash
docker compose -f apps/dev/docker-compose.yml down -v
```

## `searchd` (Week 3)

Compose includes **`searchd`** on host port **8090**. The Tantivy index is a **bind mount** at `apps/agentrank/.local-search-index` → `/data/index` inside the container.

1. Start Postgres + Redis: `docker compose -f apps/dev/docker-compose.yml up -d postgres redis`
2. Migrate and build the index (from host, talking to Compose Postgres on **5433**):

   ```bash
   cd apps/agentrank
   export DATABASE_URL=postgresql://agentrank:agentrank@127.0.0.1:5433/agentrank
   sqlx migrate run --source migrations
   mkdir -p .local-search-index
   export SEARCH_INDEX_PATH="$PWD/.local-search-index"
   cargo run -p agentrank-search-index --bin agentrank-index -- rebuild --output "$SEARCH_INDEX_PATH"
   ```

3. Start searchd: `docker compose -f apps/dev/docker-compose.yml up -d searchd` (from repo root).

`GET http://127.0.0.1:8090/health` (liveness), `GET http://127.0.0.1:8090/ready` (readiness: DB + Redis + index), and `POST http://127.0.0.1:8090/v1/search` with JSON body `{"query":"demo","limit":5}` should work after the demo seed migration.

The **production** image ([`apps/agentrank/Dockerfile`](../agentrank/Dockerfile)) uses an **entrypoint** that runs `sqlx migrate`, index boot (`rebuild` or `reuse` via `SEARCHD_INDEX_BOOT`), then `searchd` (see [`docker/entrypoint-searchd.sh`](../agentrank/docker/entrypoint-searchd.sh)). Local Compose uses a plain `searchd` binary mount instead so you control migrate/rebuild from the host.

## CI parity

GitHub Actions runs PostgreSQL and Redis as **service containers** on **5432** / **6379** on the runner (no host conflict). Locally, Compose maps Postgres to host **5433** instead; credentials and DB name still match CI.
