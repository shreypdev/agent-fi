# Railway — AgentRank (searchd, consoled, agentbot, healthd, console UI)

All **Rust** services share **one Docker image** built from [`Dockerfile`](./Dockerfile). Railway **duplicates the service** per role with the same **root directory** (repo `.`) and the same **config as code** path [`railway.toml`](./railway.toml); only **environment variables** differ (`AGENTRANK_PROCESS`). No custom build commands or alternate build folders.

**Console UI** is a **separate** Docker image ([`apps/console/Dockerfile`](../console/Dockerfile)) — same pattern as Landing (root + Dockerfile path in `railway.toml`).

See also: [docs/railway-architecture.md](../../docs/railway-architecture.md).

---

## Prerequisites (one project)

| Plugin / service | Purpose |
| ---------------- | ------- |
| **Postgres** | `DATABASE_URL` for searchd, consoled, agentbot, optional **healthd** |
| **Redis** | `REDIS_URL` for searchd + **agentbot frontier / rate limits** (+ **healthd** if used) |

---

## 1. `searchd` (search API + index)

1. **New service** → connect repo → **Settings → Config as code** → `apps/agentrank/railway.toml`.
2. **Root directory:** `.` (repository root).
3. **Variables (runtime):**
   - `DATABASE_URL` — reference Postgres.
   - `REDIS_URL` — reference Redis.
   - `SEARCH_INDEX_PATH` — e.g. `/tmp/agentrank-index` or a path on a **volume** (recommended for prod: attach volume, set `SEARCHD_INDEX_BOOT=reuse`).
   - **`AGENTRANK_PROCESS=searchd`** (recommended explicit; if omitted, the image defaults to `searchd`).
   - `CORS_ORIGINS` — include your **Landing** and **Console UI** public origins in prod.
   - `CORS_REQUIRE_ORIGINS=1` in prod.
   - Optional: `METRICS_BEARER_TOKEN`, `SEARCHD_INDEX_BOOT`, `TRUST_PROXY_HEADERS`, etc. (unchanged from before).
   - Optional: **`SEARCH_API_KEY`** — **searchd only.** If set, requests to **`POST /v1/hints`** with `Authorization: Bearer <this value>` get a **higher daily hint cap** (50 vs 5 for anonymous). Other services (**consoled**, **agentbot**, **healthd**) do **not** use this variable.
4. **Health check:** `GET /ready` (readiness).
5. Deploy → copy **public HTTPS URL** → use as **`VITE_SEARCH_API_BASE_URL`** on Landing.

---

## 2. `consoled` (operator Console API)

1. **New service** → same repo → **same** `apps/agentrank/railway.toml` and root **`.`**.
2. **Variables:**
   - **`AGENTRANK_PROCESS=consoled`**
   - `DATABASE_URL` — same Postgres as searchd.
   - **`CONSOLE_API_KEY`** — long random secret (operators enter it in the Console UI).
   - **`CONSOLE_CORS_ORIGIN`** — public origin of **Console UI** only, e.g. `https://your-console.up.railway.app` (no trailing slash).
   - `PORT` — leave unset; Railway injects it.
3. **Health check:** `GET /health` (no auth).
4. Deploy → copy **public HTTPS URL** → this is **`VITE_CONSOLE_API_BASE`** for the Console UI **build** (step 4).

**Note:** `consoled` runs `sqlx migrate` on startup; shared migrations include `console_domain_claims`. `searchd` also migrates on boot — safe and idempotent.

---

## 3. `agentbot` (crawl worker — scale replicas)

1. **New service** → same repo → same `apps/agentrank/railway.toml`, root **`.`**.
2. **Variables:**
   - **`AGENTRANK_PROCESS=agentbot`**
   - `DATABASE_URL`, `REDIS_URL` — same as searchd.
   - Optional: **`AGENTBOT_BOOT_DISCOVER=1`** — runs **`agentbot discover builtin` once** before `run-loop` (queues the built-in demo URL). Turn off when you use your own seeds only.
   - Optional: `GITHUB_TOKEN` for `discover github` (run via one-off / cron if you add a command override later).
   - Optional: `AGENTBOT_METRICS_BIND=0.0.0.0:9092` — expose **9092** on the service if you scrape Prometheus.
   - Politeness: `AGENTBOT_HOST_MAX_PER_SEC` (default `2`), `AGENTBOT_ROBOTS_TTL_*` — see `CrawlRunConfig::from_env` in code.
3. **Start command:** leave **empty** (image entrypoint runs `entrypoint-agentbot.sh` → `agentbot run-loop`).
4. **Scaling:** increase **replicas** in Railway; all workers share the Redis frontier (`ZPOPMAX` is atomic). Per-host rate limits are in Redis.

**Do not** set `AGENTBOT_ALLOW_HTTP_LOCALHOST` / `AGENTBOT_ALLOW_LOOPBACK_HTTPS` in production unless you understand the SSRF tradeoff.

**Seeding:** besides `AGENTBOT_BOOT_DISCOVER`, enqueue from a trusted machine or a one-off job:  
`agentbot enqueue 'https://example.com/.well-known/agent.json' --priority 10`  
(same image: override start command to run `agentbot discover http-json 'https://feed/...'` once, then restore `agentbot` service to `run-loop` only.)

---

## 4. `healthd` (one-shot dependency probe — optional)

`healthd` is **not** an HTTP server. It connects to Postgres and Redis, optionally Qdrant, then **exits** with code **0** (ok) or **1** (failure). Use it for **Railway Cron** (scheduled smoke), CI, or manual “are my deps up?” checks.

1. **New service** → same repo → same `apps/agentrank/railway.toml`, root **`.`**.
2. **Variables:**
   - **`AGENTRANK_PROCESS=healthd`**
   - **`DATABASE_URL`** — reference Postgres (same as searchd).
   - **`REDIS_URL`** — reference Redis.
   - Optional: **`QDRANT_URL`** — e.g. `http://qdrant:6334` on private networking; if set and non-empty, the probe **requires** Qdrant to answer. Omit if you do not run Qdrant.
   - Optional: **`RUST_LOG=debug`** for verbose logs.
3. **Deploy / Cron:** use **Railway Cron** (or a scheduled job) so the container runs on an interval; success = exit **0**. Do **not** treat it like a long-running web service — there is no `PORT` to probe.
4. **No** `PORT`, `SEARCH_INDEX_PATH`, or CORS.

For API readiness (index + optional Qdrant), prefer **searchd** `GET /ready`.

---

## 5. Console UI (`apps/console`)

1. **New service** → same repo → **Config as code** → [`apps/console/railway.toml`](../console/railway.toml).
2. **Root directory:** `.`
3. **Build variable (Docker build arg):** add **`VITE_CONSOLE_API_BASE`** in Railway and mark it **available at build time** (same pattern as Landing’s `VITE_*`). Value = **public URL of consoled** (§2), **no trailing slash**.
4. **No runtime env required** for API calls if the build arg was correct (Vite bakes the base URL).
5. Deploy → operators open the Console URL → **Setup** → paste **`CONSOLE_API_KEY`**.

---

## 6. Search index freshness after crawls

`agentbot` **writes Postgres**; `searchd` serves **Tantivy** built from DB. New ingests **do not** appear in `POST /v1/search` until the index is rebuilt.

**Practical options:**

- **Simplest:** after a large crawl, **redeploy** the **searchd** service (entrypoint runs `agentrank-index rebuild` by default), or temporarily set `SEARCHD_INDEX_BOOT=full` and redeploy.
- **Slower cold starts avoided:** attach a **volume** to searchd, use `SEARCHD_INDEX_BOOT=reuse`, then run **manual redeploy** or a **scheduled redeploy** when you need freshness (Railway cron / external scheduler).
- **Future:** incremental `upsert` pipeline or shared index storage — not in this image yet.

---

## Local Docker smoke (optional)

```bash
# From repo root
docker build -f apps/agentrank/Dockerfile -t agentrank:local .

docker build -f apps/console/Dockerfile \
  --build-arg VITE_CONSOLE_API_BASE=https://consoled.example.com \
  -t agentrank-console:local .
```

---

## Config-as-code summary

| Railway service | Config file | `AGENTRANK_PROCESS` |
| --------------- | ----------- | ------------------- |
| searchd | `apps/agentrank/railway.toml` | `searchd` (or omit) |
| consoled | `apps/agentrank/railway.toml` | `consoled` |
| agentbot | `apps/agentrank/railway.toml` | `agentbot` |
| healthd (Cron / one-shot) | `apps/agentrank/railway.toml` | `healthd` |
| Console UI | `apps/console/railway.toml` | — |
| Landing | `apps/landing/railway.toml` | — |

## Cost / scale notes

- **Rust always-on:** searchd + consoled + agentbot (three allocations); agentbot replicas multiply cost linearly. **healthd** is usually **Cron-only** (short runs), not a fourth always-on server.
- **Console UI** + **Landing** are static/nginx — usually cheap.
- **Egress:** agentbot fetches the public web; keep `AGENTBOT_HOST_MAX_PER_SEC` conservative.

## GitHub → Railway

Connect the repo per service or use the Railway CLI / GitHub Action with `RAILWAY_TOKEN` for deploys.
