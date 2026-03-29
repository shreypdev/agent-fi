# AgentRank — living execution todo

Companion to [`a2a-discovery-opus.md`](./a2a-discovery-opus.md) (architecture + [Milestones: Weekly build plan](./a2a-discovery-opus.md#milestones-weekly-build-plan)). **Edit this file every week:** check boxes, add notes, move spillover forward.

---

## How to maintain

1. **Start of week:** Set **Current week** below; copy this week's unchecked items into GitHub Issues / Linear if you use a tracker.
2. **During the week:** Check off completed items; add `Note:` lines under any item (indent with two spaces) for decisions, PR links, or scope cuts.
3. **End of week:** Roll unfinished items to the next week or **Backlog** with a one-line reason; update **Last updated** date.
4. **Gates:** Do not mark a phase "complete" until the **Done when** criteria in the opus milestones are true (or explicitly waived in writing in **Decisions & waivers**).

---

## Meta (update when you touch this file)

| Field | Value |
|--------|--------|
| **Current week** | Week 7 of 52 |
| **Phase focus** | ☑ Phase 0 · ☐ Phase 1 (Weeks 6–7 ✅; gate = Week 12) · ☐ Phase 2 · ☐ Phase 3 |
| **Last updated** | 2026-03-31 |
| **Owner / DRI** | |

---

## North star

**AgentRank = the search engine for agents.** People and machines use us to find, evaluate, and connect to the best agent for any task. We **crawl the open web** for A2A agent cards, **rank** them transparently, expose **MCP + A2A** so other agents search us programmatically, and give providers a **Console** to manage their presence — exactly like Google Search + Search Console, but for the agentic web.

**Market-ready means:**
1. **Crawler is best-in-class and fully autonomous** — discovers agents from the open internet via **≥8 vectors** (well-known + agent-sitemap, DNS, CT, registry/API catalogs, referral graph + hints, Common Crawl + HTML/JSON-LD, package/OCI registries, GitHub code search, RSS/feeds — see **§ Open-web crawler: complete requirement specification**) with **zero** manual intervention in steady state
2. Search returns ranked, trust-annotated, explained results (not just BM25 keyword matches)
3. Other agents can search us via **MCP tool** and **A2A protocol** (machine-first)
4. Landing page explains how the engine works + lets humans discover agents + links to Console
5. Console lets providers claim domains, trigger crawls, see scores, verify identity
6. Community can submit agent URLs (hints API)

**Crawl-first principle (2026-03-30):** The crawler is the moat. An empty index makes ranking, trust, console, and landing page irrelevant. Every week prioritizes coverage and autonomous discovery over downstream features. Nothing ships that doesn't feed or depend on the crawler until the crawler is world-class.

### LLM-assisted search (named scope — avoids “LLM” meaning only embeddings)

| Layer | What it is | Phase 1 plan |
|-------|------------|--------------|
| **Dense retrieval (not generative LLM)** | `BGE-*` embeddings + Qdrant kNN + RRF with BM25 | Week 6 — this is the primary “semantic” path |
| **Query understanding (rules first)** | Synonym map (A2A/MCP), optional lightweight classifier | Week 13+ in opus; **optional** Phase 1 stub: static expansion table in searchd |
| **LLM query rewrite (optional)** | Call an LLM API to rewrite or decompose user query before search | **Not required for Phase 1 gate.** Backlog or Phase 2 Week 13 if eval shows recall win |
| **Conversational search UI** | Chat UX that runs search + optional generative answer | **Post–Phase 1** unless explicitly prioritized; landing stays **query → ranked results** for MVP |

**Rule:** “Hybrid search” in P0 means **lexical + embeddings**, not “ChatGPT answers.” Generative LLM features are **named and gated** so the team does not confuse embeddings with chat.

### Index freshness SLO (searchable lag)

**SLO:** After a successful agent ingest (row committed in Postgres), the agent is **searchable** through `POST /v1/search` (Tantivy + Qdrant in sync) within **P95 ≤ 15 minutes**, **P99 ≤ 60 minutes** under normal load (Phase 1). Stricter targets (e.g. &lt;5 min) are a stretch.

**Implementation path (Phase 1):**
1. **On ingest / update:** agentbot (or shared lib) calls **`agentrank-index upsert`** (or equivalent) for that `agent_id` after DB commit — updates Tantivy + enqueues embedding job for Qdrant.
2. **Metric:** `agentrank_index_lag_seconds` histogram (time from `agents.updated_at` to “index write complete”); alert if P95 &gt; SLO for 1h.
3. **Fallback:** Console “Reindex” and deploy-time full rebuild remain for repair; they are not the steady-state path.
4. **Ready check:** `GET /ready` continues to reflect index readable; optional **index-lag** field in internal health JSON for operators.

---

## Ongoing (every week)

- [ ] Security: dependency / image triage; no known criticals unpatched in prod
- [ ] Data: backup restore spot check or automated proof artifact
- [ ] Quality: golden-query regression on release candidate
- [ ] Incidents: postmortem filed for any user-visible SLO miss

---

## Product requirements traceability (opus §4)

**P0 ↔ Phase 1 reconciliation:** The rows below are written so **Phase 1 (Weeks 6–12)** can close each item **without footnotes**. Where opus originally stated a higher bar (200 queries, 0.7 NDCG, L3, API keys everywhere), we either **ship the full criterion in Phase 1** or **split** into “Phase 1 done” vs “opus stretch (Phase 2)” in the same bullet.

### P0 — must ship (MVP)

- [ ] **P0-01** Autonomous crawl of public agent card URLs (e.g. `/.well-known/agent.json` and variants in scope). **Two bars:** (1) **Pilot:** **90%+** of a **defined pilot set** of known public agents discovered within **72h** of seed init (list versioned in-repo or operator-owned). (2) **Open web:** steady-state discovery from **≥8 vectors** in **§ Open-web crawler: complete requirement specification** without manual seeding — see Phase 1 gate and Weeks 8–10.
- [ ] **P0-02** Registry feed ingestion — **≥5** distinct sources wired in production (not counting `builtin_demo_seed` alone); pipeline runs on a schedule
- [ ] **P0-03** Agent Card parse + validate + normalize (**99%+** valid cards in eval fixtures; invalid → evidence / quarantine, not indexed)
- [ ] **P0-04** Canonical registry + entity resolution — **95%+** duplicate-merge accuracy on labeled cross-source pairs (**Phase 1 gate:** Week 12 dedup + metrics; full 95% measured on maintained eval set)
- [ ] **P0-05** Hybrid search (lexical **+** semantic via embeddings + fusion) — **Phase 1 complete when:** NDCG@10 **≥ 0.65** on **≥ 50** judged golden queries (hybrid + AVERT ordering). **Opus stretch (Phase 2):** grow to **≥ 200** queries and NDCG@10 **≥ 0.7** with LTR / more labels
- [ ] **P0-06** AgentRank v1 transparent scoring + **explanation payload** on every result (top factors)
- [ ] **P0-07** Multi-level liveness — **Phase 1 includes L1 + L2 + L3:** **L1** TCP/TLS to endpoint; **L2** HTTP GET card URL with parse validation; **L3** minimal **non-destructive** A2A health ping (`message/send` with probe metadata / `do_not_bill` pattern per opus §15) where endpoint declares A2A; agents without A2A skip L3 gracefully. **SLAs:** status fresh within **15m** of material change where probes apply; **dead/delist** demotion within **1h** of sustained failure
- [ ] **P0-08** Trust tiers (Indexed → Established → Verified minimum set); **criteria documented** in repo; tiers visible in API + UI
- [ ] **P0-09** `POST /v1/search` — trust annotations + stable versioned schema; **p95 latency &lt; 50ms** on **synthetic / golden** load profile in staging (documented env + hardware); **two access modes:** (1) **anonymous** + IP rate limit (current behavior), (2) **`Authorization: Bearer &lt;API key&gt;`** for higher limits — keys issued for partners/internal (**Week 11**); OpenAPI updated
- [ ] **P0-10** Agent Search Console v1 — domain claim, crawl/index **status**, **score** visibility, enqueue/reindex controls
- [ ] **P0-11** Basic abuse resistance — FP **&lt; 1%**, synthetic spam catch **≥ 90%** on fixed fixture suite (Week 12)
- [ ] **P0-12** Public web UI — search, results, detail pages, trust badges, **Console link**, **how-it-works** narrative

### P1 — fast follow (target: 4–6 weeks after MVP)

- [ ] **P1-01** Direct connect — connection metadata on ≥95% of healthy results
- [ ] **P1-02** Brokered connect — 90%+ success compatible pairs (staging → prod)
- [ ] **P1-03** Outcome telemetry API — ingest, classify, store
- [ ] **P1-04** Domain verification — DNS TXT and/or `agent-proof.json`; within 5 minutes after proof (**automated** polling — may move from Console manual verify)
- [ ] **P1-05** Signed metadata — verify + trust boost + docs
- [ ] **P1-06** Graph retrieval — measurable NDCG lift vs lexical+semantic only
- [ ] **P1-07** Benchmark framework v1 — conformance + non-destructive capability signals
- [ ] **P1-08** **A2A self-discovery (remainder):** core card + MCP + structured `message/send` ship in **Phase 1 Week 7**; **P1** adds OAuth scopes, richer skills, NL query path if deferred

### P0 → Phase 1 week mapping (single source of truth)

| ID | Closed in Phase 1 by… |
|----|------------------------|
| P0-01 | **Week 8–10 crawler v2/v3/v4** (sitemap, DNS, CT, domain probe, referral graph, Common Crawl, registries) — supersedes Week 6 basic discover |
| P0-02 | **Week 8** registry API connectors (paginated, auth) — ≥5 real sources |
| P0-03 | Ongoing + Week 12 abuse overlap |
| P0-04 | **Week 8** cross-source dedup at ingest + **Week 12** SimHash hardening |
| P0-05 | Week 6 hybrid + **Week 11** AVERT ordering + Week 12 gate (≥50 judged, ≥0.65 NDCG) |
| P0-06 | **Week 11** (AVERT v1 + explanation payload) |
| P0-07 | **Week 10** (L1–L3 probes + status state machine + fetch policy) |
| P0-08 | **Week 10** (trust tier computation + query-time trust floor) |
| P0-09 | **Week 11** API keys + load profile; latency gate in Week 12 |
| P0-10 | **Week 11** Console v2 (domain verify, agents dashboard, enqueue, scores) |
| P0-11 | Week 12 |
| P0-12 | **Week 12** Landing + GTM |
| **Index freshness SLO** | Week 6+ (upsert path + metric); crawler load increases through Week 10 — watch `agentrank_index_lag_seconds` |

### P2 — strategic (3–6 months)

- [ ] **P2-01** Enterprise private registries + tenant isolation (zero cross-tenant leakage)
- [ ] **P2-02** Federation pull + push — ≥2 live partners
- [ ] **P2-03** Policy-aware search (residency, compliance packs)
- [ ] **P2-04** Learning-to-rank — significant offline NDCG vs heuristic
- [ ] **P2-05** Outcome-driven ranking in production mix
- [ ] **P2-06** Agent Search Console v2 — benchmarking, query analytics, recommendations

---

## What exists today (inventory after Week 7)

| Layer | What's built | What's missing for market-ready |
|-------|-------------|--------------------------------|
| **Crawl engine** | Frontier, run-loop, robots.txt, SSRF, per-host rate limits, 5 discover sources (builtin, HTTP JSON, static file, GitHub, card-link expand), scheduled discover, recrawl policy, community hints API | **Full Phase 1 contract in § Open-web crawler: complete requirement specification** — agent-sitemap, DNS, CT, registries, domain probe (HEAD→GET), Common Crawl+HTML/JSON-LD, packages/OCI, RSS, provenance, crawl budgets, `agent-robots` |
| **Search API** | `POST /v1/search` (hybrid BM25+Qdrant RRF), `GET /v1/agents/:id`, rate limit, CORS, OpenAPI | Ranking (AVERT), trust annotations, explanation payloads, API key auth tiers, filters |
| **MCP/A2A on searchd** | MCP tools (`search_agents`, `get_agent_details`) at `POST /mcp`; A2A `message/send` structured at `POST /v1/a2a`; Agent Card at `/.well-known/agent-card.json` | NL query path (optional Phase 2) |
| **Landing page** | Home (marketing), `/search` (hybrid), `/agents/:id` (JSON dump), `/connect` (MCP/A2A setup) | "How it works" section, trust badges on results, rich agent detail page, link to Console, submit-an-agent CTA |
| **Console** | Domain claims (stub, no verify), crawl history (read-only), agent JSON inspector | Trigger crawls/enqueue, trust/score dashboard, domain verification flow, liveness history, "trigger re-index" |
| **Ranking** | None (results are unranked BM25+RRF) | AVERT v1, composite score, explanation payload, score decay, trust floor |
| **Trust & liveness** | `trust_records` table exists, `trust_tier` field on search results (always "indexed") | Probe scheduler, L1-L3 probes, status state machine, trust tier computation, demotion rules |
| **Vectors** | Qdrant deployed, embedding pipeline (hash/BGE), hybrid retrieval (RRF k=60) | Production BGE/ONNX embedder (hash is CI placeholder) |
| **Dedup/entity resolution** | Cuckoo filter on frontier (URL-level only) | Cross-source entity resolution, canonical agent merging |

---

## Open-web crawler: complete requirement specification (Phase 1)

**Purpose:** This block is the **contract** for a top-tier discovery system aligned with opus §9 (six discovery vectors). Weekly tasks (Weeks 8–10) implement subsets; nothing here is “optional nice-to-have” for a search engine that claims **open-web** coverage — scope is **not** thinned; execution may be **parallelized** (AI + humans). Items below are **acceptance criteria** unless marked *stretch*.

### A. Discovery vectors (must all be wired to the frontier by Phase 1 gate)

| # | Vector | Source | Output |
|---|--------|--------|--------|
| V1 | **Well-known + sitemap** | `/.well-known/agent.json`, `/.well-known/agent-sitemap.xml` (+ index shards), non-standard `/agent.json` | Card URLs + sibling agents |
| V2 | **DNS** | `_agentfi.*` TXT, `_a2a._https.*` SRV, optional `_mcp.*` | Sitemap URL or card URL candidates |
| V3 | **Certificate Transparency** | Major CT logs (poll / stream); pattern filter on CN+SAN | Domains → probe queue |
| V4 | **Registry / catalog APIs** | PulseMCP, mcp.so, AgentVerse, Smithery, Glama, OpenRouter, Hugging Face HOL, etc. | Normalized → card URL or endpoint→card probe |
| V5 | **Open web & artifacts** | Common Crawl WARC/CDX; **HTML** pages with links or JSON-LD; **RSS/Atom** items; **npm / PyPI / Cargo** (scoped search); **Docker Hub / GHCR / GCR public**; **GitHub** code search (already); **OCI labels** | Candidate URLs → validate with GET |
| V6 | **Community & graph** | `POST /v1/hints`; referral graph (`relatedAgents`, provider URL, auth-domain sitemap, URLs in text) | Enqueue + BFS expansion |

### B. Fetch & probe semantics (correctness on the real internet)

- **HEAD vs GET:** Many hosts misbehave on `HEAD` or omit `Content-Type`. **Requirement:** probe pipeline tries **HEAD** first with short timeout; on inconclusive (non-200, wrong type, empty) **fall back to GET** with **small byte cap** (e.g. first 8–64 KiB) and sniff JSON / parse partial card.
- **Redirects:** Follow **same-origin or explicit allowlist** redirects up to N hops; re-validate URL policy at each hop (SSRF, loopback, private IP blocks per `crawl-policy`).
- **Conditional requests:** Support `If-Modified-Since` / `If-None-Match` for sitemap and card URLs where stored.
- **Robots:** Respect **`robots.txt`** for the host; parse and honor **`/.well-known/agent-robots.txt`** (agent-specific allow/deny for card and sitemap paths) — *same policy layer as standard robots*.
- **X-Robots-Tag / noindex:** If response headers or HTML meta say **noindex** for the card URL, **do not index** (still may record crawl attempt for diagnostics).

### C. Politeness, budget, and safety (operable at scale)

- **Global crawl budget:** Per-registrable-host rate limits + **daily cap** per host; **global** max in-flight probes (config); backoff on 429/503 with jitter.
- **CT and DNS:** Separate **budget pools** so CT never starves user-facing ingest; configurable **max CT-derived probes per minute**.
- **Cost visibility:** Metrics for **HTTP bytes out**, **CT entries scanned**, **CC rows processed**, **DNS queries** — dashboards + optional alerts.
- **Abuse resistance:** Rate-limit **discovery-only** paths that could amplify DoS (e.g. wildcard subdomain enumeration); blocklist for **scanner traps** / known bad ASNs (*stretch*: shared threat feed).

### D. Data model & provenance

- Every enqueued URL carries **`discovery_source`** (enum: `sitemap`, `dns_txt`, `ct_log`, `registry:pulsemcp`, `common_crawl`, `github_search`, `hint`, `referral_graph`, …) and optional **`discovery_confidence`** for prioritization **before** AVERT exists.
- **Cross-source dedup** at ingest (endpoint + canonical URL); **`agent_aliases`** for multiple card URLs pointing at same logical agent.

### E. Recrawl policy (no circular dependency on AVERT)

- **Until Week 11 (AVERT):** adaptive intervals use **only** change streak, stable streak, crawl success, and optional **static** priority by `discovery_source` — **not** composite AgentRank score.
- **From Week 11:** may blend in AVERT / liveness for **priority** (hot vs cold) while keeping fairness caps.

### F. Phase 1 gate — crawler completeness

- **≥8 discovery sources** producing **non-zero** enqueue counts in staging over 30 days (metrics by `discovery_source`).
- **No manual seeding** required in steady state (operator may still use emergency `discover file` / enqueue).
- **Documented runbook** per major vector (`docs/runbooks/crawler-*.md`): env vars, rate limits, failure modes.

---

## Phase 0 — Weeks 1–4 (vertical slice) ✅

### Week 1 — Repo + data plane

- [x] Monorepo / workspace layout matches opus stack (Rust core, Python ML off hot path) — `apps/agentrank`, `apps/ml-pipeline`
- [x] CI: build, unit tests, lint, format check on PR — [.github/workflows/ci.yml](.github/workflows/ci.yml)
- [x] Docker Compose (or equivalent): app deps for local dev — [apps/dev/docker-compose.yml](apps/dev/docker-compose.yml)
- [x] PostgreSQL schema v1: `agents`, `providers`, `crawl_history`, `trust_records` (minimal) — [apps/agentrank/migrations](apps/agentrank/migrations)
- [x] Migrations applied cleanly in CI + documented — `sqlx migrate run` in CI + [apps/agentrank/README.md](apps/agentrank/README.md)
- [x] Redis up for frontier + cache; health check documented — Compose + `healthd` ([apps/agentrank/crates/healthd](apps/agentrank/crates/healthd))

### Week 2 — Ingest path

- [x] AgentBot v0.1: HTTP fetch → parse → persist one card URL — [`crates/agentbot`](apps/agentrank/crates/agentbot), `agentbot ingest <url>`
- [x] Parser: JSON schema validation, required fields, normalization rules — [`crates/card`](apps/agentrank/crates/card)
- [x] 50+ parser unit tests (valid / invalid / edge) — `cargo test -p agentrank-card`
- [x] Frontier v0.1: enqueue, priority dequeue, dedup (10K URL stress test) — [`crates/frontier`](apps/agentrank/crates/frontier)
- [x] Integration test: mock `agent.json` → expected DB row — `crates/agentbot/tests/ingest_integration.rs`

### Week 3 — Search + API + UI

- [x] Tantivy index: name, description, skills (minimum fields) — [`crates/search-index`](apps/agentrank/crates/search-index), `agentrank-index rebuild`
- [x] Index rebuild / incremental path defined (even if crude) — `rebuild` + `upsert`; `SEARCH_INDEX_PATH`; version file `AGENTRANK_INDEX_VERSION`
- [x] API gateway: `POST /v1/search` + `GET /v1/agents/:id`; anonymous tier + Redis rate limit — [`crates/searchd`](apps/agentrank/crates/searchd), OpenAPI [`openapi/search-v0.1.yaml`](apps/agentrank/openapi/search-v0.1.yaml)
- [x] Web UI: search, result list, agent detail page; responsive sanity check — [`apps/landing`](apps/landing) `/search`, `/agents/:id`, Vitest smoke
- [x] 1K fixtures path + keyword golden smoke + P99 script — [`scripts/gen_1k_agents_sql.py`](apps/agentrank/scripts/gen_1k_agents_sql.py), [`tests/search_golden.json`](apps/agentrank/tests/search_golden.json), [`scripts/search_p99.sh`](scripts/search_p99.sh)

### Week 4 — Crawl scale + console + observability

- [x] AgentBot v0.2: frontier consumer, per-host rate limits, `robots.txt` respect
- [x] SSRF / fetch policy documented and tested
- [x] searchd Prometheus `/metrics` + HTTP counters/histograms
- [x] Agent Search Console v0.1: domain claim path, card inspector, crawl history (`agentrank-consoled`, `apps/console`)
- [x] Prometheus metrics per service; Grafana dashboards as code; alert narrative in `docs/grafana/README.md`
- [x] **Phase 0 gate:** script `apps/agentrank/scripts/phase0_gate.sh` + seed file

---

## Phase 1 — Weeks 5–12 (the product becomes real)

> **Goal:** By end of Week 12, a developer or agent can hit our API (REST, MCP, or A2A), get back ranked, trust-annotated, explained results from a continuously-refreshed index of real public agents. **Discovery** meets **§ Open-web crawler: complete requirement specification** (open-web coverage, not registry-only). Providers use Console to manage their agents. The landing page tells the story and lets anyone discover agents.

### Week 5 — Seed explosion (A) ✅

- [x] ≥3 registry connectors hardened + runbooks (`builtin_demo_seed`, `http_json_urls`, `static_json_file`; `docs/runbooks/registry-*.md`)
- [x] GitHub (or code host) discovery MVP for card URLs (`agentbot discover github`, wiremock tests)
- [x] Frontier metrics: URLs discovered / enqueued / dup rate (Prometheus + Grafana panels)

### Week 6 — Autonomous crawl + vectors

**Theme: the crawler runs itself; first vector index.**

_Crawl autonomy (toward P0-01):_
- [x] **Scheduled discover**: agentbot cron / timer that re-runs `discover` sources on interval (e.g. every 6h for GitHub, every 1h for HTTP feeds) — no manual BOOT_DISCOVER needed in steady state
  - Note: `discover_scheduler` + `AGENTBOT_DISCOVER_INTERVAL_SECS`, `AGENTBOT_DISCOVER_HTTP_JSONS`, `PULSEMCP_FEED_URL` / `MCPSO_FEED_URL`; `docs/runbooks/agentbot-scheduler.md`
- [x] **Recrawl policy**: after successful ingest, re-enqueue the card URL at lower priority with adaptive interval based on change frequency (BLAKE3 hash diff from opus §9.6)
  - Note: `frontier_url_state` table + `recrawl::schedule_recrawl`; fixed 6h horizon in MVP (expand adaptive intervals later)
- [x] **Card-link expansion**: when ingesting an Agent Card, extract any `url`, `provider.url`, related agent URLs → enqueue as new candidates (protocol-native expansion, opus §9)
  - Note: `card_expand::enqueue_card_links` walks `card_json` for `https://` strings + `validate_outbound_url`
- [x] **≥2 more real registry feeds** (target: PulseMCP, mcp.so, or AgentVerse — bring total real sources to ≥5 for P0-02)
  - Note: `PULSEMCP_FEED_URL` / `MCPSO_FEED_URL` + wiremock tests; P0-02 “≥5 total” — verify connector mix in staging
- [x] **Community Hints API**: `POST /v1/hints` on searchd — accept `{url, source}`, validate URL, enqueue to frontier; rate limit by IP (5/day anon, 50/day keyed); responds with `hint_id` + `queued` status (opus §9)
  - Note: `SEARCH_API_KEY` Bearer → 50/day; OpenAPI + `live_search` integration tests

_Vectors:_
- [x] Qdrant deployed (Docker Compose + Railway); health check in `healthd`
  - Note: `QDRANT_URL` gRPC **6334**; use **private** `http(s)://qdrant…:6334` on Railway (not public `*.up.railway.app` for gRPC); see `apps/agentrank/RAILWAY.md`
- [x] Embedding pipeline: on agent ingest/update, compute `BGE-base-en-v1.5` (768-dim) embedding of `name + description + skills`; store in Qdrant collection `agents`; batch backfill script for existing agents
  - Note: **768-dim** path shipped with deterministic **`AGENTRANK_EMBEDDER=hash`** (Blake3 unit vectors) for CI/plumbing; swap to BGE/ONNX or sidecar for production semantic quality. `index_pipeline` after ingest; `agentbot index-backfill` for repair; `index_jobs` on failure
- [x] **Hybrid retrieval**: searchd runs parallel BM25 (Tantivy, top-500) + kNN (Qdrant, top-500), fuses with **RRF** (k=60, weights: lexical 0.4, semantic 0.6); small labeled set (≥30 queries) shows hybrid > BM25-only
  - Note: `fusion.rs` + `hybrid_qdrant_e2e` in CI with Qdrant service; `scripts/hybrid_eval.sh` + `tests/hybrid_eval/` for manual eval; **NDCG ≥0.65 / ≥50 queries** remains Week 12 gate

_Index freshness (starts Week 6, hardened Week 7–8):_
- [x] **Post-ingest upsert path:** after successful DB commit from ingest, invoke **incremental index update** (Tantivy doc upsert + Qdrant vector upsert) for that `agent_id` — no full-cluster rebuild required for single-agent freshness
- [x] **Emit metric** `agentrank_index_lag_seconds` (histogram): `now() - agents.updated_at` at index write time; dashboard panel + alert if P95 &gt; **15 min** for 1h
  - Note: `docs/grafana/README.md` narrative; **staging P95 burst** measurement still operator-validated
- [x] Document **fallback** when upsert fails: queue + retry; operator full rebuild unchanged
  - Note: Postgres `index_jobs` + metric `agentrank_index_upsert_failures_total`

**Done when:** `agentbot run-loop` discovers new agents without operator intervention for 24h; hybrid search returns better results on labeled set; hints API accepts and enqueues a URL; **P95 index lag ≤ 15m** in staging on synthetic ingest burst.
  - Note: **24h unattended** + **P95 lag** — validate in staging with real cadence; core plumbing is in place.

### Week 7 — searchd becomes an agent (MCP + A2A) — **staged MVP cut**

**Theme:** ship **machine-native search** first; **structured A2A** second; **NL-in-A2A** last — avoids a single week that tries to ship Rust MCP + full A2A + NL parsing.

**Milestone A — MCP (ship first; Phase 1 gate depends on this):**
- [x] **MCP tool `search_agents`** on searchd: StreamableHTTP transport at `/mcp`; tool accepts `{query, limit?, filters?}` (structured JSON args); calls internal search; returns `{results, total}` aligned with REST
  - Note: **`POST /mcp`** JSON-RPC (`initialize`, `tools/list`, `tools/call`) — **stateless POST**; not full Streamable HTTP + SSE session; see `docs/api/mcp-a2a-searchd.md`
- [x] **MCP tool `get_agent_details`**: `{agent_id}` → full card + scores + trust when Week 8 lands (stub scores OK before AVERT merge)
- [x] **MCP manifest** at `/.well-known/mcp.json` on searchd
  - Note: `AGENTRANK_PUBLIC_URL` for absolute URLs in manifest
- [x] Landing `/connect`: add **"Add AgentRank search to your AI"** (Cursor / Claude / MCP URL) pointing at **searchd** public URL
  - Note: `ConnectPage` + `VITE_SEARCH_API_BASE_URL` / `searchApi.ts` MCP helpers

**Milestone B — A2A structured (ship second; same week if capacity allows):**
- [x] **Agent Card** at `/.well-known/agent-card.json` and **alias** `GET /.well-known/agent.json` (same JSON) for tools that expect either name
- [x] **A2A JSON-RPC** `message/send`: accept **structured** `parts` / `data` (e.g. `{ "skill": "search_agents", "query": "...", "limit": 10 }`) — **no** free-form NL parsing required for this milestone
  - Note: **`POST /v1/a2a`** minimal structured body (`skill`, `query`, `agent_id`, …) — not full JSON-RPC transport; golden tests in `live_search`
- [ ] **Optional:** register searchd’s public card URL in frontier for dogfood (low priority)

**Milestone C — NL in A2A (last; optional for Phase 1 gate — defer to Week 12 stretch or Phase 2):**
- [ ] **Natural-language** `message/send` body: map user text → search query via **rules + synonym table** first; **LLM-based** NL→query only if explicitly prioritized (see **LLM-assisted search** table above)
- [x] If deferred: document `message/send` with structured payload only in `/docs`
  - Note: `nl_not_supported` on `text` field; `docs/api/mcp-a2a-searchd.md`

**Milestone D — LLM (optional, not gate):**
- [x] No generative LLM in Phase 1 gate path; embeddings-only for semantic search
  - Note: `apps/agentrank/README.md` / embeddings = dense retrieval only

**Done when (Phase 1 gate):** MCP tools work from Cursor; **A2A** `message/send` works with **structured** search payload; **NL** is optional and documented if skipped.
  - Note: validate **Cursor** against **public searchd** `POST /mcp`; structured A2A path covered by tests.

### Week 8 — Crawler v2: agent-sitemap + domain probing + registry API connectors

**Theme: the crawler leaves the nest. From "GitHub + JSON feeds" to "probe any domain on the internet for agents." This is the Googlebot sprint.**

_Agent-sitemap.xml (opus §8.1.1 — highest-yield single vector for multi-agent providers):_
- [ ] **Sitemap parser crate** (`agentrank-sitemap`): parse `agent-sitemap.xml` and `agentsitemapindex` (index files pointing to child sitemaps); extract `<card_url>`, `<lastmod>`, `<status>`, inline `<skills>` summary; validate against schema; handle gzip
- [ ] **Sitemap discover path**: for every **known provider domain** (from `agents.provider_url` in DB), probe `https://{domain}/.well-known/agent-sitemap.xml`; if found, parse → enqueue all `<card_url>` entries to frontier at HIGH priority
- [ ] **Sitemap in crawl loop**: after successful card ingest, extract `provider.url` domain → check if sitemap already fetched (Redis key `sitemap:{domain}` with TTL 24h) → if not, fetch + parse + enqueue siblings
- [ ] **Sitemap conditional fetch**: use `<lastmod>` to skip re-fetching individual cards whose sitemap lastmod hasn't changed since last crawl (ETag/If-Modified-Since where supported)
- [ ] **Metrics**: `agentrank_sitemap_fetch_total{status}`, `agentrank_sitemap_cards_discovered_total`

_Domain probing — well-known path discovery (opus §9 Vector 1; see **§ Open-web crawler** §B):_
- [ ] **Domain prober module** (`discover_domain_probe`): for each candidate host, **HEAD** first (5s timeout) against: `/.well-known/agent.json`, `/.well-known/agent-sitemap.xml`, `/agent.json`, `/api/agent-card`; on **inconclusive** (non-200, missing/wrong `Content-Type`, empty body) → **GET** with **byte cap** (8–64 KiB) and JSON sniff / partial parse
- [ ] **Subdomain expansion**: for each probed domain, also try `agents.{domain}`, `agent.{domain}`, `api.{domain}`, `a2a.{domain}`, `mcp.{domain}` — DNS resolve first (no HTTP if NXDOMAIN)
- [ ] **Seed domain list v1**: curate initial domain list from (a) all unique provider domains already in DB, (b) cloud platform patterns from opus (`.fly.dev`, `.railway.app`, `.run.app`, `.vercel.app`, `.up.railway.app`), (c) known agent-hosting domains from community/GitHub
- [ ] **Probe scheduler**: run domain prober on interval (daily for known domains, weekly sweep for cloud platform subdomains); enqueue discoveries to frontier; dedup via Redis bloom filter
- [ ] **Robots + agent-robots**: extend `crawl-policy` / robots cache to fetch and apply **`/.well-known/agent-robots.txt`** (agent-specific allow/deny) in addition to site `robots.txt`; document interaction when both exist (most restrictive path wins)
- [ ] **Politeness**: User-Agent `AgentBot`; per-host budget (see **§C**); backoff on 429/503; respect redirect limits and URL policy each hop
- [ ] **Metrics**: `agentrank_domain_probe_total{result=found|miss|error}`, `agentrank_domain_probe_latency_seconds`, `agentrank_probe_bytes_total`

_Registry API connectors — paginated, authenticated (opus §9 Vector 4):_
- [ ] **Registry connector trait v2**: extend `RegistrySource` with `async fn discover_paginated(&self, client: &Client, cursor: Option<String>) -> Result<(Vec<DiscoveredUrl>, Option<String>)>` — cursor pagination + API key auth + per-registry rate limiting
- [ ] **PulseMCP connector**: PulseMCP Sub-Registry API (`/api/v0.1/servers`); auth via `X-Tenant-Id` + `X-API-Key`; parse server entries → construct card URL probe candidates; paginate with `updated_since`
- [ ] **mcp.so connector**: scraper or API connector for mcp.so catalog; extract server entries → probe for agent cards
- [ ] **AgentVerse (Fetch.ai) connector**: AgentVerse REST API; API key auth; paginate; map entries to card URL candidates
- [ ] **Generic registry adapter**: config-driven connector: `{api_url, auth_header, page_param, entries_jsonpath, card_url_template}` — covers Smithery.ai, Glama.ai, OpenRouter without per-registry code
- [ ] **Registry sync state**: Postgres table `registry_sync_state(registry_name, last_cursor, last_sync_at, total_synced, errors)` for pagination resume
- [ ] **Env wiring**: `PULSEMCP_API_KEY`, `PULSEMCP_TENANT_ID`, `AGENTVERSE_API_KEY` etc.; update `.env.example` + `docs/runbooks/agentbot-scheduler.md`
- [ ] **≥5 real registry feeds live** in staging with real API keys — closes **P0-02** for real

_Dedup hardening (crawler will find same agent from 3+ sources now):_
- [ ] **Cross-source dedup at ingest**: check `endpoint_url` or normalized `canonical_url` already in DB → merge (highest-trust version, union skills, all source URLs as `agent_aliases`)
- [ ] Dedup rate metric: `agentrank_dedup_merges_total`

_Provenance (feeds AVERT S_R and ops dashboards):_
- [ ] **Extend frontier enqueue path** so every URL carries **`discovery_source`** (and optional **confidence**) from sitemap probe, domain probe, and registry connectors — see **§D** (Week 10 extends same to CC/RSS/package jobs)

**Done when:** agentbot discovers agents from sitemaps, domain probing, **`agent-robots.txt` honored**, and ≥3 real registry APIs (paginated, auth) without manual intervention. New agents from providers with sitemaps found within one crawl cycle. Domain probing finds agents on `.fly.dev` / `.railway.app` / `.run.app` without seeds. ≥5 registry sources produce real URLs in staging. **§ Open-web crawler §B** (HEAD→GET) satisfied for probes.

### Week 9 — Crawler v3: DNS discovery + CT log mining + referral graph

**Theme: the crawler sees the whole internet. Every new HTTPS certificate, every DNS record, every link between agents becomes a discovery signal.**

_DNS TXT/SRV discovery (opus §9 Vector 1 — fast path):_
- [ ] **DNS resolver module** (`discover_dns`): for a domain, resolve `_agentfi.{domain}` TXT → extract `sitemap=` URL and `agents=` count; resolve `_a2a._https.{domain}` SRV → construct card URLs from SRV targets
- [ ] **DNS batch prober**: given a domain list, batch-resolve TXT+SRV via `trust-dns` / `hickory-dns` async resolver; filter hits → enqueue to frontier
- [ ] **Cloud platform subdomain enumeration**: for each cloud pattern (`*.fly.dev`, `*.railway.app`, `*.run.app`, etc.), enumerate known subdomains via (a) CT-log-derived lists, (b) permutation of agent keywords (`agent-*`, `a2a-*`, `mcp-*`, `*-bot`); DNS resolve → if exists → HEAD probe
- [ ] **Daily DNS sweep**: cron/scheduled task; processes domain list in batches; resume-safe (Redis cursor per sweep run)
- [ ] **Metrics**: `agentrank_dns_probe_total{type=txt|srv, result=found|miss}`, new domains discovered per sweep

_Certificate Transparency log mining (opus §9 Vector 2 — near-real-time):_
- [ ] **CT log monitor** (`discover_ct`): connect to CT log feeds (Google Argon, Let's Encrypt Oak, Cloudflare Nimbus) via polling; stream new certificate entries
- [ ] **Domain pattern filter**: from each cert CN + SANs, match high-confidence patterns: `agents?.*.*, a2a.*.*, mcp.*.*`; medium: cloud subdomains with agent keywords; low: `*.ai.*` (configurable)
- [ ] **Pipeline**: CT entry → pattern match → DNS pre-check (domain resolves?) → enqueue **probe task** to frontier (same HEAD→GET fallback as **§B**) at priority by confidence
- [ ] **Backfill**: on first deploy, process recent CT logs (past 30 days) for domains issued before monitor started
- [ ] **Rate control**: CT logs produce ~10K entries/sec; after filtering ~50-200 probe candidates/sec; respect probe rate limits
- [ ] **Binary or background task**: `agentbot discover ct-stream` (long-running) or spawned task in `run-loop` alongside `discover_scheduler`
- [ ] **Metrics**: `agentrank_ct_entries_processed_total`, `agentrank_ct_candidates_total`, `agentrank_ct_confirmed_total`

_Referral graph walking (opus §9 Vector 3 — amplifies every discovery):_
- [ ] **Graph walker module** (`discover_referral_graph`): for each ingested agent, extract references: `provider.url` → sitemap probe, `relatedAgents[]` → direct enqueue, shared auth domain → sitemap probe, URL mentions in skill descriptions → candidate probe
- [ ] **Depth-limited BFS**: walk referral graph up to depth=3; confidence decays 0.7^depth; stop below 0.3 threshold
- [ ] **Scheduled graph walk**: after each crawl batch (or daily), select recently-discovered agents → run graph walk → enqueue new candidates
- [ ] Upgrade existing `card_expand` to be the **depth-1** case of graph walker (add provider-domain sitemap probe + subdomain expansion beyond current `https://` string extraction)
- [ ] **Metrics**: `agentrank_graph_walk_depth`, `agentrank_graph_walk_discoveries_total`

_Adaptive recrawl (upgrade from Week 6 fixed interval; see **§E** — no AVERT dependency until Week 11):_
- [ ] **Change-frequency tracker**: use existing `change_streak` / `stable_streak` in `frontier_url_state`; compute adaptive interval: `base_interval * 2^stable_streak` (max 7d) or `base_interval / 2^change_streak` (min 1h)
- [ ] **Priority until Week 11**: use **crawl-derived signals only** — change rate, time since last successful ingest, `discovery_source` static weight — **not** composite AgentRank (scores ship Week 11)
- [ ] **Post-Week 11 (optional blend)**: may increase frequency for high AVERT + healthy liveness; keep per-host caps
- [ ] Replace Week 6 fixed 6h recrawl horizon with adaptive scheduler

**Done when:** CT pipeline produces candidate domains within **≤120s p95** from cert visibility to enqueue (staging; **60s** is stretch — depends on CT poll interval and filter load). DNS sweep finds agents advertising via `_agentfi` TXT records. Referral graph expands from 1 known agent to 5+ unknown agents at same provider. Adaptive recrawl adjusts interval from **change/stability** data without AVERT.


### Week 10 — Crawler v4: Common Crawl + package registries + liveness

**Theme: mine the entire archived web. Ship liveness probes so the index stays alive as it grows. The crawler is now world-class.**

_Common Crawl batch processing (opus §9 Vector 5 — monthly ~3B pages):_
- [ ] **Common Crawl processor**: download WARC index / CDX for latest crawl month; filter for: (a) URLs ending `/.well-known/agent.json` or `agent.json` on allowlisted paths, (b) **HTML** responses containing `/.well-known/agent.json`, `agent-card`, `a2a`, `application/json` links to card-like URLs, (c) **JSON-LD** `application/ld+json` blocks with `SoftwareApplication` / `WebAPI` / custom `@type` referencing agent endpoints (*extract URLs*)
- [ ] **HTML link extraction (bounded):** for matched HTML records, **regex + lightweight parser** (no full browser) to pull `href`/`src` pointing at card URLs or `*.json` on same registrable domain; **cap** links per page and **total bytes** processed per job
- [ ] **Implementation**: Python or Spark job (AWS Athena on CC index, or local `cdx-toolkit`) → output `urls.json` + `discovery_source=common_crawl` → `agentbot discover file` or direct Redis enqueue; documented in `docs/runbooks/common-crawl.md`
- [ ] **Schedule**: monthly after each Common Crawl release; CI job or manual trigger; track yield per month
- [ ] **Metrics**: `agentrank_common_crawl_candidates_total`, `agentrank_common_crawl_confirmed_total`, `agentrank_common_crawl_bytes_processed_total`

_Package & OCI registry mining (opus §9 Vector 4 — npm, PyPI, Docker Hub, GHCR):_
- [ ] **npm discovery**: search npm for `@a2a/*`, `@mcp/*`, `a2a-*`, `mcp-*`; extract `repository.url` → GitHub probe → card URL candidates
- [ ] **PyPI discovery**: search PyPI for `a2a-*`, `mcp-*`; extract `project_urls.Homepage` / `Repository` → probe for agent cards
- [ ] **Cargo / crates.io** (*stretch*): search `a2a-*`, `mcp-*` crates; repo URL → probe
- [ ] **Docker Hub discovery**: search for `*/a2a-*`, `*/mcp-*` images; extract labels (`org.opencontainers.image.*`) + README for deployment URLs → probe
- [ ] **GHCR / GCR public** (*stretch*): same label + README strategy for public registries with API access
- [ ] **Scheduled**: weekly; paginated; dedup against frontier; results → probe queue
- [ ] **Metrics**: `agentrank_pkg_registry_candidates_total{registry=npm|pypi|docker|ghcr}`

_Feeds & docs (opus §9 Vector 5 — adjunct to CC):_
- [ ] **RSS/Atom poller**: maintain list of **engineering blogs / product changelogs** (config) + discover feeds from ingested `provider.url` domains; **fetch feed** → extract URLs matching agent-card patterns → enqueue (`discovery_source=rss`)
- [ ] **Sitemap.xml (HTML site):** optional **low-priority** fetch of `https://{domain}/sitemap.xml` for provider domains already in DB; grep for URLs containing `agent` / `well-known` (*do not* crawl full web graph — seed only)

_Provenance (all new enqueue paths):_
- [ ] **DB or Redis**: store `discovery_source` + optional `confidence` on frontier enqueue (or parallel `frontier_discover_meta` table) for **§D** traceability and AVERT **S_R** later

_Liveness probes (P0-07 — keeps the growing index honest):_
- [ ] **L1 probe**: TCP+TLS connect; record connect time, cert expiry; 15min for Hot tier, 1h others
- [ ] **L2 probe**: HTTP GET card URL; validate JSON + required fields; detect changes (BLAKE3 hash)
- [ ] **L2/L3 fetch policy**: same outbound URL policy as ingest (`validate_fetch_url`, SSRF blocks, scheme allowlist); robots.txt respected; documented in `docs/security-fetch-policy.md`
- [ ] **L3 probe**: minimal non-destructive A2A `message/send` where endpoint declares A2A; `agentrank_probe` / `do_not_bill` metadata; skip for MCP-only — record `skipped_reason`
- [ ] **Probe scheduler**: background task pulling agents due for probe; L1/L2/L3 → `probe_results` table
- [ ] **Status state machine** (opus §15.2): `HEALTHY → DEGRADED → UNHEALTHY → DEAD → DELISTED`; consecutive failures → transitions; ranking penalties (0 / -10 / -30 / -50 / not searchable)
- [ ] **Freshness SLA**: status reflects reality within 15min; dead agent demoted within 1h

_Trust tiers (P0-08):_
- [ ] **Tier computation**: `Indexed` (default), `Established` (card valid ≥7d + L2 pass >95% + ≥3 fields), `Verified` (domain verified + TLS + provider info)
- [ ] **Trust floor at query time**: penalty multiplier (0.5 one tier below, 0.0 two+)
- [ ] Store `trust_tier` and `liveness_status` in Tantivy fast fields; filter/boost in search

**Done when:** Common Crawl monthly batch yields ≥1000 candidate URLs (or documented yield for first month). Package + OCI registry discovery finds packages on npm/PyPI (and Docker Hub / GHCR if in scope). **RSS/feed** path produces ≥1 batch of candidate URLs in staging. **Provenance** (`discovery_source`) recorded for enqueue paths. Liveness probes run automatically; dead agents demote within 1h. Trust tiers compute and affect ordering. **The crawler meets § Open-web crawler sections A–F and operates across 8+ vectors with zero manual intervention in steady state.**

### Week 11 — Ranking v1 (AVERT) + Console + API keys

**Theme: now that the index is full and alive, rank it. Give providers tools to manage their agents.**

_AVERT v1 (P0-06 — ranking becomes real):_
- [ ] **AVERT v1 compute** (opus §12): five sub-scores as stored fields on agents table:
  - **S_A (Availability, 0.25)**: uptime from crawl success rate + L1/L2 probe pass rate, response time percentile
  - **S_V (Verification, 0.20)**: card completeness score (weighted field checklist from opus §12.4)
  - **S_E (Expertise, 0.20)**: skill depth, description quality, embedding distance from centroid
  - **S_R (Reputation, 0.20)**: provider sibling count + discovery source quality + referral graph in-degree
  - **S_T (Trust, 0.15)**: TLS presence, auth maturity, provenance signals, trust tier
- [ ] **Composite AgentRank** `[0,100]` = weighted AVERT sum; stored in Postgres + Tantivy fast field
- [ ] **Score decay**: `score × e^(-λΔt)` with λ=0.001/h (~29-day half-life); recompute on crawl or daily batch
- [ ] **Explanation payload**: `explain: {score, factors: [{name, value, weight, description}]}`
- [ ] **Search uses composite score**: `rrf_score * 0.6 + agent_rank_norm * 0.4`
- [ ] Update OpenAPI spec with `explain` field and score fields

_Console v2 (P0-10 — providers manage agents):_
- [ ] **`POST /v1/console/enqueue`**: accept `{url, priority?}` → Redis frontier; requires auth
- [ ] **`POST /v1/console/reindex`**: trigger index rebuild; admin-only
- [ ] **`GET /v1/console/agents`**: list agents for a claimed domain with scores, trust tier, liveness, last crawl
- [ ] **`GET /v1/console/agents/:id/scores`**: full AVERT breakdown + liveness history
- [ ] **Domain verification flow**: `POST /v1/console/domain-claims/:id/verify` — DNS TXT `_agentrank-verify=<token>` OR `/.well-known/agentrank-verify.json`; transition claim to `verified`
- [ ] Console UI: agents dashboard, agent detail with AVERT chart, enqueue form, domain verification wizard, polish from JSON dumps to styled components

_Search API auth (P0-09):_
- [ ] **API key on searchd**: `Authorization: Bearer` → elevated rate limit + metrics label; anonymous tier unchanged
- [ ] Document key issuance; OpenAPI security scheme `bearerAuth`

**Done when:** search results ordered by composite AVERT score; every result has `explain` payload; Console domain verification works E2E; API key access live for partners.

### Week 12 — Landing + GTM + abuse + Phase 1 gate

**Theme: present the crawler's work to the world. Clean the index. Launch.**

_Landing page (P0-12):_
- [ ] **"How AgentRank works" section**: visual 4-step flow → (1) We crawl the web for agent cards via 8+ discovery vectors, (2) We rank them with AVERT, (3) Search via web/API/MCP/A2A, (4) Scores update continuously
- [ ] **"For agent builders" CTA**: Console link + hints API; "Add AgentRank search to your agent" → MCP/A2A setup
- [ ] **Submit agent URL** on landing: form calling hints API
- [ ] **Rich search results**: score badge + trust tier + liveness indicator + protocol + domain
- [ ] **Agent detail page**: real layout with score, trust, liveness, skills, provider, connect docs
- [ ] **Search filters**: trust tier min, protocol filter, sort by relevance/score/newest
- [ ] **Developer docs**: `/docs` with API usage, code examples, MCP integration guide

_GTM & legal:_
- [ ] Terms of Service + Privacy Policy (`/terms`, `/privacy`)
- [ ] Crawl & index policy — what AgentBot indexes, politeness, opt-out (link from footer)
- [ ] Abuse contact; Pricing page (free tier + "contact us" for API key)

_Abuse resistance (P0-11):_
- [ ] **Spam scoring**: keyword stuffing, ALL-CAPS, excessive emojis, skill count outlier
- [ ] **Impersonation check**: Levenshtein distance vs existing Verified agents; auto-quarantine
- [ ] **Quarantine status**: `visibility=quarantined`; operators review or auto-release after 7d
- [ ] Synthetic spam test suite: catch rate ≥90%, FP <1%

_Entity resolution hardening (P0-04):_
- [ ] **SimHash fingerprint** for near-duplicate detection: >90% similar descriptions at same endpoint → auto-merge
- [ ] Measure dedup accuracy on labeled cross-source pairs

_Phase 1 gate:_
- [ ] **500+ real public agents indexed** (from autonomous multi-vector crawl, not just seeds) — **raised from 100** because the crawler is now world-class
- [ ] **NDCG@10 ≥ 0.65** on **≥ 50** judged golden queries (hybrid + AVERT ordering)
- [ ] **≥ 200** judged queries and **NDCG@10 ≥ 0.7** — Phase 2 stretch (do not block)
- [ ] **Search availability >99.5%** over 7-day window; **p95 < 50ms**
- [ ] **Index freshness:** P95 lag ≤ 15 min over 7 days
- [ ] **8+ discovery vectors operational** (non-zero staging traffic each; see **§A**): e.g. agent-sitemap + well-known probe, DNS TXT/SRV, CT, GitHub code search, registry/API catalogs, domain probe, referral graph + hints, Common Crawl (+ HTML/JSON-LD extraction), package/OCI registries, RSS/feed poller — **≥8** distinct `discovery_source` values with proof in metrics
- [ ] **MCP Milestone A** + **A2A Milestone B** complete; **NL** shipped or documented as deferred
- [ ] **L1–L3** probes live; demotion SLAs met
- [ ] **API key** path live; Console domain verification works
- [ ] **Landing + GTM**: homepage → search → results → detail E2E; legal links in footer
- [ ] Comms checklist: README, changelog, social announcement draft

---

## Phase 2 — Weeks 13–24 (intelligence + enterprise + revenue)

> **Goal:** By end of Week 24, ranking uses ML (LTR), outcomes feed back into scores, enterprise tenants can use private registries, and revenue streams are live.

### Week 13 — LTR foundation + query understanding

- [ ] **Feature logging**: every search logs query + results + positions + scores to ClickHouse `query_log`
- [ ] **Click/impression tracking**: landing search tracks impressions (result seen) and clicks (result clicked); log to ClickHouse
- [ ] **Offline dataset builder**: script that joins query_log + clicks → pairwise training data with position bias correction
- [ ] **Intent classifier v0**: rule-based + simple model that classifies query intent (capability search, name lookup, protocol filter, comparison); used to select retrieval strategy
- [ ] **Query expansion**: synonym table (a2a → agent-to-agent, mcp → model-context-protocol) + skill taxonomy expansion

### Week 14 — LTR model + outcome API

- [ ] **Train LambdaMART** (XGBoost rank:ndcg): ≥15 features (RRF score, AgentRank, trust_tier, S_A through S_T, freshness, skill_match, name_match, description_quality, provider_agent_count); ONNX export
- [ ] **Shadow scoring**: run LTR model alongside AVERT heuristic on 100% traffic; log both; compare NDCG offline
- [ ] **Outcome telemetry API**: `POST /v1/outcomes` on searchd — accept `{session_id, caller_agent?, target_agent_id, outcome: connected|completed|timed_out|error, quality?, latency_ms?, tokens?}`; store in `connection_outcomes`; rate limit
- [ ] CI job: weekly NDCG artifact comparing heuristic vs LTR on held-out set

### Week 15 — LTR production + advanced search

- [ ] **LTR rollout**: 10% traffic → 50% → 100% with automatic fallback to heuristic if NDCG drops >5% or P95 latency >100ms
- [ ] **Structured filters on search API**: `protocols`, `trust_tier_min`, `auth_schemes`, `modalities`, `max_latency_ms`; reflected in OpenAPI and landing UI
- [ ] **Pagination**: opaque cursor-based (base64 JSON: query hash, last score, last agent_id); `next_cursor` in response; landing UI infinite scroll or "Load more"
- [ ] **Sort options**: relevance (default), AgentRank score, newest, most connections (future)

### Week 16 — Direct connect + outcomes in ranking

- [ ] **Direct connect metadata** (P1-01): search results include `connection` object: `{endpoint, protocol, auth_schemes, transport}` for ≥95% of healthy agents
- [ ] **Outcome features in ranking**: `outcome_success_rate_30d`, `total_outcomes_30d` as LTR features; agents with positive outcomes rank higher
- [ ] **Offline replay**: show NDCG lift from outcome features vs Week 15 model

### Week 17 — Anti-abuse (A) + graph foundations

- [ ] **Link farm detection**: agents that mutually reference each other in provider/delegation links → flag + penalize in S_R
- [ ] **CTR manipulation detection**: statistical model for expected CTR by position; agents with CTR >3σ above expected → flag
- [ ] **Graph store** (Neo4j or Postgres recursive CTE): `PROVIDER_OF`, `REFERENCES`, `SAME_ENDPOINT` edges; populate from crawl data
- [ ] **Graph signal in ranking**: provider sibling count, reference in-degree as features (not full PageRank yet)

### Week 18 — Benchmarks + graph ranking

- [ ] **Benchmark framework v1** (P1-07): L4-style conformance tests — send A2A `message/send` with test prompts to opted-in agents; score task completion, format, safety; store results in `benchmark_runs`
- [ ] **50+ conformance test cases**: response format, timeout handling, auth flow, error reporting
- [ ] **PageRank v0**: power iteration on agent graph (d=0.85); daily recompute; inject into S_R
- [ ] **Graph retrieval** (P1-06): for top-10 RRF seeds, expand 1-hop on graph → additional candidates; measure NDCG lift

### Week 19 — Enterprise (A) + domain verification v2

- [ ] **Tenant model**: `tenant_id` on agents table; `visibility_scope` enum (public, tenant_private); JWT-based tenant context on API
- [ ] **Tenant isolation test**: two tenants, each with private agents; neither can see the other's
- [ ] **Domain verification v2** (P1-04): automated DNS TXT check on schedule (every 5min for pending claims); `/.well-known/agentrank-verify.json` as alternative; Console shows real-time verification status
- [ ] **Verified trust tier**: agents on verified domains auto-upgrade to Verified tier

### Week 20 — Enterprise (B) + signed metadata

- [ ] **Private ingest**: tenant can submit agent cards via API that are only visible to their org
- [ ] **Signed metadata** (P1-05): validate Ed25519 signatures on agent cards that include `agentRankExtensions.proof`; trust boost for signed cards
- [ ] **mTLS option**: enterprise API endpoints support mTLS for tenant auth (or OAuth2 client credentials)

### Week 21 — Federation + WebSocket

- [ ] **Federation pull**: connector that fetches agent lists from partner registries; dedup vs existing crawl data; health metrics per partner
- [ ] **≥50K agents in staging** from federated + crawled sources without dup explosion
- [ ] **WebSocket events** on searchd: `agent_discovered`, `agent_rank_changed`, `agent_died`, `agent_revived`; for integrators building real-time UIs

### Week 22 — Monetization (A)

- [ ] **API key tiers**: Free (120 req/min), Pro ($49/mo, 1000 req/min, outcome API, bulk), Enterprise (custom)
- [ ] **Promoted slots design**: max 2 per query; labeled "Sponsored"; Verified+ only; bid × quality_score auction
- [ ] **Click/impression logging** for billing: extend ClickHouse pipeline

### Week 23 — Monetization (B)

- [ ] **Billing integration**: Stripe or invoice export for Pro/Enterprise tiers
- [ ] **Agent Analytics** ($19/agent/mo): impressions, clicks, CTR, rank history, recommendations — visible in Console
- [ ] **Click fraud checks**: IP/session dedup; suspicious pattern detection; reconciliation

### Week 24 — Analytics + Phase 2 gate

- [ ] Premium analytics surfaces in Console: query analytics, competitor comparison, optimization recommendations
- [ ] Cost / unit economics dashboard (internal)
- [ ] **Phase 2 gate:** ≥1M agents indexed (crawled + federated); NDCG@10 ≥ 0.75; enterprise pilot active; MRR above $100K (or written waiver + date)

---

## Phase 3 — Weeks 25–52 (scale + moat + dominance)

_One checkbox per week as "theme done" tracker._

- [ ] **W25** Push federation — contract + stub receiver
- [ ] **W26** Push federation — first partner live
- [ ] **W27** Partner portal v0 — keys, health, support path
- [ ] **W28** Multi-region — second region + failover drill
- [ ] **W29** Data residency — policy tags + query enforcement
- [ ] **W30** 5+ federation partners green on dashboard
- [ ] **W31** Compositional / multi-step queries MVP
- [ ] **W32** Workflow search — graph expansion recall win
- [ ] **W33** Embedding / reranker experiment slot + A/B
- [ ] **W34** Security benchmarks — opt-in scan → trust payload
- [ ] **W35** Security UX — user-tested trust surfacing
- [ ] **W36** Ranking polish — merge 31–35 without SLO regression
- [ ] **W37** Recommendations v0 — history + similar agents
- [ ] **W38** Recommendations v1 — contextual at search time
- [ ] **W39** Data licensing v0 — aggregate API / export + review
- [ ] **W40** Data licensing — first external customer
- [ ] **W41** On-chain attestation PoC
- [ ] **W42** Attestation provider UX + support playbook
- [ ] **W43** MCP deep discovery — index MCP servers alongside A2A agents; tool-level search
- [ ] **W44** MCP tool-level index — search returns individual tools, not just servers
- [ ] **W45** Cross-protocol ranking fairness vs A2A baseline
- [ ] **W46** Cost optimization pass — documented savings/capacity
- [ ] **W47** Game day / chaos — Kafka, DB, broker
- [ ] **W48** Developer flywheel — docs, SDK stubs, examples
- [ ] **W49** Ecosystem growth — content + partner pipeline metric
- [ ] **W50** 10K+ QPS peak drill (or documented path)
- [ ] **W51** Quality week — golden set + spam + manual eval
- [ ] **W52** Year review — roadmap v2, 5M+ path, moat metrics readout

---

## Backlog (unscheduled)

_Items **promoted** from backlog into weeks are struck through here. Add bullets as `- [ ] …`; promote when ready._

- ~~Agent-sitemap.xml support~~ → **Week 8** (Crawler v2)
- ~~DNS TXT discovery~~ → **Week 9** (Crawler v3)
- ~~Certificate Transparency log scanning~~ → **Week 9** (Crawler v3)
- ~~L3 probe~~ → **Week 10** (Crawler v4 liveness)
- ~~agent-robots.txt~~ → **Week 8** (Crawler v2 — `robots` + `/.well-known/agent-robots.txt`; see **§ Open-web crawler** §B)
- [ ] GraphQL API for Console and integrators (opus §16.8)
- [ ] L4 probe: capability verification with generated test prompts (opus §15.3)
- [ ] Brokered connect v1 (P1-02): session broker, policy engine, mediation
- [ ] Cross-encoder reranker stage (opus §11.5): `ms-marco-MiniLM-L-6-v2` between RRF and LTR
- [ ] Agent-proof.json verification: Ed25519 signature + BLAKE3 card hash (opus §8)
- [ ] ClickHouse for analytics (replace/supplement Prometheus for long-term query/connection data)
- [ ] Kafka/Redpanda event bus for crawl→index→rank pipeline (replace direct DB polling)
- [ ] Neo4j for graph retrieval (replace Postgres recursive CTE if scale demands)
- [ ] Python ML pipeline: offline embedding retraining, LTR hyperparameter search, eval harness
- [ ] Internationalization: multi-language agent cards, query understanding
- [ ] "Agent of the day" / featured agents on landing homepage
- [ ] API SDK: `@agentrank/sdk` (JS/TS) and `agentrank-py` (Python) client libraries
- [ ] Status page at `status.agentrank.dev`
- [ ] **Generative LLM query rewrite** (call external LLM to expand user query before hybrid retrieval) — only if offline eval shows recall lift; default remains rules + synonyms (Phase 2 Week 13 unless promoted)

---

## Blockers & questions

| Date | Blocker | Owner | Status |
|------|---------|-------|--------|
| | | | |

---

## Decisions & waivers

_Log scope cuts and criterion waivers so the team does not "forget" a gate._

| Date | Decision | Rationale |
|------|-----------|-----------|
| 2026-03-28 | Phase 0 **100 real URL** seed list is **operator-curated**, not fully populated in-repo | Public A2A card URLs are sparse and rot; repo ships `tests/fixtures/phase0_seed_urls.txt` as a **vetted starter** plus `phase0_gate.sh`. Expand the file (or use `discover file` / HTTP feeds) before calling the gate "100-URL complete." |
| 2026-03-28 | Grafana **alert** is documented as operator wiring (webhook not in git) | Matches plan: one rule narrative in `docs/grafana/README.md`; wire in Grafana Cloud / Alertmanager to your channel. |
| 2026-03-29 | **MCP + A2A on searchd** elevated to Week 7 (was P1-08, post-MVP) | Machine-native search is the killer feature. If other agents can search us via MCP/A2A, we get adoption from the agent ecosystem immediately. This is what makes us "the search engine" not just "a website with a search box." |
| 2026-03-30 | **P0-05 split:** Phase 1 = **≥50** judged queries + **NDCG@10 ≥ 0.65**; opus **≥200** + **≥0.7** = Phase 2 LTR / labeling scale | Removes contradiction between P0 table and Phase 1 gate; **P0 row text** now states both tiers explicitly. |
| 2026-03-30 | **P0-07:** Phase 1 ships **L1 + L2 + L3** (L3 skipped gracefully for non-A2A agents) | Matches opus multi-level liveness without leaving L3 only in backlog. |
| 2026-03-30 | **P0-09:** Anonymous + **optional API key** on searchd (not keys-only) | Preserves public search while satisfying "API key auth" as a **mode** for partners. |
| 2026-03-30 | **Week 7 staged:** MCP → structured A2A → NL last | De-risks shipping Rust MCP + A2A in one sprint; NL/LLM optional for gate. |
| 2026-03-29 | **Qdrant chosen over Milvus/Pinecone** for vector store | Self-hosted, Rust-native, good k8s/Docker support, free, gRPC + REST. Aligns with opus §19 stack choice. |
| 2026-03-30 | **Crawler-first reorder:** Weeks 8–10 dedicated to crawler (sitemap, DNS, CT, Common Crawl, registry APIs, domain probing, referral graph, package registries); AVERT ranking moved to Week 11; Console/Landing/GTM compressed to Weeks 11–12 | The crawler is the moat. An empty or tiny index makes ranking, trust tiers, Console, and landing page irrelevant. Weeks 8–10 implement **all 6 opus discovery vectors** (DNS/subdomain, CT logs, referral graph, registry federation, open web/Common Crawl, community hints) plus liveness probes. AVERT ranking only becomes meaningful once the index is large enough to differentiate agents. Phase 1 gate raised to **500+ agents** (from 100) and **8+ discovery vectors** operational. |
| 2026-03-31 | **Full crawler requirement spec (no scope thin):** Added **§ Open-web crawler: complete requirement specification** — vectors A–F, HEAD/GET semantics, `agent-robots`, crawl budget, provenance, recrawl without AVERT circular dependency. **AI-assisted implementation** is acceptable; the **contract** is completeness for open-web discovery, not a minimal MVP. | Aligns doc with “top tier” bar; CT **60s** relaxed to **120s p95** with **60s stretch**; adaptive recrawl uses crawl signals only until Week 11. |

---

## Quick links (fill in for your org)

- Repo: 
- Staging URL: 
- Grafana: 
- Runbooks: 
- Golden query set location: 
