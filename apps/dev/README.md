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

## CI parity

GitHub Actions runs PostgreSQL and Redis as **service containers** on **5432** / **6379** on the runner (no host conflict). Locally, Compose maps Postgres to host **5433** instead; credentials and DB name still match CI.
