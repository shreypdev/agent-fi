# AgentRank — Rust workspace (data plane)

Week 1 **repo + data plane**: shared PostgreSQL/Redis helpers, `healthd` binary, and **sqlx** SQL migrations under `migrations/`.

## Toolchain

Uses [rust-toolchain.toml](./rust-toolchain.toml) (pinned stable). Install via [rustup](https://rustup.rs/).

```bash
cd apps/agentrank
cargo build --workspace
cargo test --workspace
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Local PostgreSQL and Redis

Start Docker services from repo root:

```bash
docker compose -f apps/dev/docker-compose.yml up -d
```

Copy root `.env.example` to `.env` and `source` it, or export:

```bash
export DATABASE_URL=postgresql://agentrank:agentrank@127.0.0.1:5433/agentrank
export REDIS_URL=redis://127.0.0.1:6379
```

## Migrations

Install CLI (once):

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

Apply:

```bash
cd apps/agentrank
sqlx migrate run --source migrations
```

Create database if needed:

```bash
sqlx database create
```

## `healthd`

Probes PostgreSQL (`SELECT 1`) and Redis (`PING`). Exits `0` only if both succeed.

```bash
cd apps/agentrank
export DATABASE_URL=...
export REDIS_URL=...
cargo run -p agentrank-healthd --bin healthd
```

Logs: set `RUST_LOG=debug` for more detail.

## Crates

| Crate                 | Role                                                |
| --------------------- | --------------------------------------------------- |
| `agentrank-data-plane` | `DATABASE_URL` / `REDIS_URL`, pools, health checks |
| `agentrank-healthd`    | Binary: operational smoke test                     |

## Schema v1

Tables: `providers`, `agents`, `crawl_history`, `trust_records`. See `migrations/20260328120000_initial_schema.sql`.

Default `trust_records.trust_tier` is `indexed`. Documented progression (app-enforced): `indexed` → `established` → `verified` → `trusted` → `authoritative`.
