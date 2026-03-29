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
| **Current week** | Week 5 of 52 |
| **Phase focus** | ☑ Phase 0 · ☐ Phase 1 · ☐ Phase 2 · ☐ Phase 3 |
| **Last updated** | 2026-03-30 |
| **Owner / DRI** | |

---

## North star

**AgentRank = the search engine for agents.** People and machines use us to find, evaluate, and connect to the best agent for any task. We **crawl the open web** for A2A agent cards, **rank** them transparently, expose **MCP + A2A** so other agents search us programmatically, and give providers a **Console** to manage their presence — exactly like Google Search + Search Console, but for the agentic web.

**Market-ready means:**
1. Crawl discovers and refreshes agents autonomously (no manual seeding in steady state)
2. Search returns ranked, trust-annotated, explained results (not just BM25 keyword matches)
3. Other agents can search us via **MCP tool** and **A2A protocol** (machine-first)
4. Landing page explains how the engine works + lets humans discover agents + links to Console
5. Console lets providers claim domains, trigger crawls, see scores, verify identity
6. Community can submit agent URLs (hints API)

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

- [ ] **P0-01** Autonomous crawl of public agent card URLs (e.g. `/.well-known/agent.json` and variants in scope); **90%+** of a **defined pilot set** of known public agents discovered within **72h** of seed init (pilot list versioned in-repo or operator-owned)
- [ ] **P0-02** Registry feed ingestion — **≥5** distinct sources wired in production (not counting `builtin_demo_seed` alone); pipeline runs on a schedule
- [ ] **P0-03** Agent Card parse + validate + normalize (**99%+** valid cards in eval fixtures; invalid → evidence / quarantine, not indexed)
- [ ] **P0-04** Canonical registry + entity resolution — **95%+** duplicate-merge accuracy on labeled cross-source pairs (**Phase 1 gate:** Week 12 dedup + metrics; full 95% measured on maintained eval set)
- [ ] **P0-05** Hybrid search (lexical **+** semantic via embeddings + fusion) — **Phase 1 complete when:** NDCG@10 **≥ 0.65** on **≥ 50** judged golden queries (hybrid + AVERT ordering). **Opus stretch (Phase 2):** grow to **≥ 200** queries and NDCG@10 **≥ 0.7** with LTR / more labels
- [ ] **P0-06** AgentRank v1 transparent scoring + **explanation payload** on every result (top factors)
- [ ] **P0-07** Multi-level liveness — **Phase 1 includes L1 + L2 + L3:** **L1** TCP/TLS to endpoint; **L2** HTTP GET card URL with parse validation; **L3** minimal **non-destructive** A2A health ping (`message/send` with probe metadata / `do_not_bill` pattern per opus §15) where endpoint declares A2A; agents without A2A skip L3 gracefully. **SLAs:** status fresh within **15m** of material change where probes apply; **dead/delist** demotion within **1h** of sustained failure
- [ ] **P0-08** Trust tiers (Indexed → Established → Verified minimum set); **criteria documented** in repo; tiers visible in API + UI
- [ ] **P0-09** `POST /v1/search` — trust annotations + stable versioned schema; **p95 latency &lt; 50ms** on **synthetic / golden** load profile in staging (documented env + hardware); **two access modes:** (1) **anonymous** + IP rate limit (current behavior), (2) **`Authorization: Bearer &lt;API key&gt;`** for higher limits — keys issued for partners/internal (Week 10–11); OpenAPI updated
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
| P0-01 | Week 6 scheduled discover + expansion + feeds |
| P0-02 | Week 6 (≥5 sources) |
| P0-03 | Ongoing + Week 12 abuse overlap |
| P0-04 | Week 12 dedup |
| P0-05 | Week 6 hybrid + Week 8 AVERT ordering + Week 12 gate (≥50 judged, ≥0.65 NDCG) |
| P0-06 | Week 8 |
| P0-07 | Week 9 (L1–L3 + SLAs + probe policy) |
| P0-08 | Week 9 |
| P0-09 | Week 8 schema + Week 10–11 API keys + load profile; latency gate in Week 12 |
| P0-10 | Week 10 Console |
| P0-11 | Week 12 |
| P0-12 | Week 11 + GTM section |
| **Index freshness SLO** | Week 6–8 (upsert path + metric) |

### P2 — strategic (3–6 months)

- [ ] **P2-01** Enterprise private registries + tenant isolation (zero cross-tenant leakage)
- [ ] **P2-02** Federation pull + push — ≥2 live partners
- [ ] **P2-03** Policy-aware search (residency, compliance packs)
- [ ] **P2-04** Learning-to-rank — significant offline NDCG vs heuristic
- [ ] **P2-05** Outcome-driven ranking in production mix
- [ ] **P2-06** Agent Search Console v2 — benchmarking, query analytics, recommendations

---

## What exists today (inventory after Week 5)

| Layer | What's built | What's missing for market-ready |
|-------|-------------|--------------------------------|
| **Crawl engine** | Frontier, run-loop, robots.txt, SSRF, per-host rate limits, 4 discover sources (builtin, HTTP JSON, static file, GitHub) | Scheduled auto-discover (cron), recrawl/refresh policy, expansion from cards/sitemaps, ≥5 real registry feeds, community hints API |
| **Search API** | `POST /v1/search` (BM25 Tantivy), `GET /v1/agents/:id`, rate limit, CORS, OpenAPI | Hybrid (vectors), ranking (AVERT), trust annotations, explanation payloads, API key auth tiers, filters |
| **MCP/A2A on searchd** | **None** — only exists on `public-agent` (demo echo) | **Week 7 staged:** MCP tools first → structured A2A `message/send` → NL optional; see milestones A–C |
| **Landing page** | Home (marketing), `/search` (BM25), `/agents/:id` (JSON dump), `/connect` (demo agent) | "How it works" section, trust badges on results, rich agent detail page, link to Console, submit-an-agent CTA |
| **Console** | Domain claims (stub, no verify), crawl history (read-only), agent JSON inspector | Trigger crawls/enqueue, trust/score dashboard, domain verification flow, liveness history, "trigger re-index" |
| **Ranking** | None (results are unranked BM25) | AVERT v1, composite score, explanation payload, score decay, trust floor |
| **Trust & liveness** | `trust_records` table exists, `trust_tier` field on search results (always "indexed") | Probe scheduler, L1-L3 probes, status state machine, trust tier computation, demotion rules |
| **Vectors** | None | Qdrant, embedding pipeline, hybrid retrieval |
| **Dedup/entity resolution** | Cuckoo filter on frontier (URL-level only) | Cross-source entity resolution, canonical agent merging |

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

> **Goal:** By end of Week 12, a developer or agent can hit our API (REST, MCP, or A2A), get back ranked, trust-annotated, explained results from a continuously-refreshed index of real public agents. Providers use Console to manage their agents. The landing page tells the story and lets anyone discover agents.

### Week 5 — Seed explosion (A) ✅

- [x] ≥3 registry connectors hardened + runbooks (`builtin_demo_seed`, `http_json_urls`, `static_json_file`; `docs/runbooks/registry-*.md`)
- [x] GitHub (or code host) discovery MVP for card URLs (`agentbot discover github`, wiremock tests)
- [x] Frontier metrics: URLs discovered / enqueued / dup rate (Prometheus + Grafana panels)

### Week 6 — Autonomous crawl + vectors

**Theme: the crawler runs itself; first vector index.**

_Crawl autonomy (toward P0-01):_
- [ ] **Scheduled discover**: agentbot cron / timer that re-runs `discover` sources on interval (e.g. every 6h for GitHub, every 1h for HTTP feeds) — no manual BOOT_DISCOVER needed in steady state
- [ ] **Recrawl policy**: after successful ingest, re-enqueue the card URL at lower priority with adaptive interval based on change frequency (BLAKE3 hash diff from opus §9.6)
- [ ] **Card-link expansion**: when ingesting an Agent Card, extract any `url`, `provider.url`, related agent URLs → enqueue as new candidates (protocol-native expansion, opus §9)
- [ ] **≥2 more real registry feeds** (target: PulseMCP, mcp.so, or AgentVerse — bring total real sources to ≥5 for P0-02)
- [ ] **Community Hints API**: `POST /v1/hints` on searchd — accept `{url, source}`, validate URL, enqueue to frontier; rate limit by IP (5/day anon, 50/day keyed); responds with `hint_id` + `queued` status (opus §9)

_Vectors:_
- [ ] Qdrant deployed (Docker Compose + Railway); health check in `healthd`
- [ ] Embedding pipeline: on agent ingest/update, compute `BGE-base-en-v1.5` (768-dim) embedding of `name + description + skills`; store in Qdrant collection `agents`; batch backfill script for existing agents
- [ ] **Hybrid retrieval**: searchd runs parallel BM25 (Tantivy, top-500) + kNN (Qdrant, top-500), fuses with **RRF** (k=60, weights: lexical 0.4, semantic 0.6); small labeled set (≥30 queries) shows hybrid > BM25-only

_Index freshness (starts Week 6, hardened Week 7–8):_
- [ ] **Post-ingest upsert path:** after successful DB commit from ingest, invoke **incremental index update** (Tantivy doc upsert + Qdrant vector upsert) for that `agent_id` — no full-cluster rebuild required for single-agent freshness
- [ ] **Emit metric** `agentrank_index_lag_seconds` (histogram): `now() - agents.updated_at` at index write time; dashboard panel + alert if P95 &gt; **15 min** for 1h
- [ ] Document **fallback** when upsert fails: queue + retry; operator full rebuild unchanged

**Done when:** `agentbot run-loop` discovers new agents without operator intervention for 24h; hybrid search returns better results on labeled set; hints API accepts and enqueues a URL; **P95 index lag ≤ 15m** in staging on synthetic ingest burst.

### Week 7 — searchd becomes an agent (MCP + A2A) — **staged MVP cut**

**Theme:** ship **machine-native search** first; **structured A2A** second; **NL-in-A2A** last — avoids a single week that tries to ship Rust MCP + full A2A + NL parsing.

**Milestone A — MCP (ship first; Phase 1 gate depends on this):**
- [ ] **MCP tool `search_agents`** on searchd: StreamableHTTP transport at `/mcp`; tool accepts `{query, limit?, filters?}` (structured JSON args); calls internal search; returns `{results, total}` aligned with REST
- [ ] **MCP tool `get_agent_details`**: `{agent_id}` → full card + scores + trust when Week 8 lands (stub scores OK before AVERT merge)
- [ ] **MCP manifest** at `/.well-known/mcp.json` on searchd
- [ ] Landing `/connect`: add **"Add AgentRank search to your AI"** (Cursor / Claude / MCP URL) pointing at **searchd** public URL

**Milestone B — A2A structured (ship second; same week if capacity allows):**
- [ ] **Agent Card** at `/.well-known/agent-card.json` and **alias** `GET /.well-known/agent.json` (same JSON) for tools that expect either name
- [ ] **A2A JSON-RPC** `message/send`: accept **structured** `parts` / `data` (e.g. `{ "skill": "search_agents", "query": "...", "limit": 10 }`) — **no** free-form NL parsing required for this milestone
- [ ] **Optional:** register searchd’s public card URL in frontier for dogfood (low priority)

**Milestone C — NL in A2A (last; optional for Phase 1 gate — defer to Week 12 stretch or Phase 2):**
- [ ] **Natural-language** `message/send` body: map user text → search query via **rules + synonym table** first; **LLM-based** NL→query only if explicitly prioritized (see **LLM-assisted search** table above)
- [ ] If deferred: document `message/send` with structured payload only in `/docs`

**Milestone D — LLM (optional, not gate):**
- [ ] No generative LLM in Phase 1 gate path; embeddings-only for semantic search

**Done when (Phase 1 gate):** MCP tools work from Cursor; **A2A** `message/send` works with **structured** search payload; **NL** is optional and documented if skipped.

### Week 8 — Ranking v1 (AVERT)

**Theme: results are ranked, not just keyword-matched. Every result has a score and explanation.**

- [ ] **AVERT v1 compute** (opus §12): implement five sub-scores as stored fields on agents table:
  - **S_A (Availability, 0.25)**: uptime from crawl success rate, response time percentile
  - **S_V (Verification, 0.20)**: card completeness score (weighted field checklist from opus §12.4)
  - **S_E (Expertise, 0.20)**: skill depth, description quality, embedding distance from centroid
  - **S_R (Reputation, 0.20)**: initially based on provider sibling count + discovery source quality (PageRank deferred to W13+)
  - **S_T (Trust, 0.15)**: TLS presence, auth maturity, provenance signals
- [ ] **Composite AgentRank** `[0,100]` = weighted AVERT sum; stored in Postgres + Tantivy fast field
- [ ] **Score decay**: `score × e^(-λΔt)` with λ=0.001/h (~29-day half-life); recompute on crawl or daily batch
- [ ] **Explanation payload**: search results include `explain: {score, factors: [{name, value, weight, description}]}` — human-readable why
- [ ] **Search uses composite score**: after RRF fusion, multiply by decayed AgentRank; final ordering = `rrf_score * 0.6 + agent_rank_norm * 0.4`
- [ ] Update OpenAPI spec with `explain` field and score fields on search results
- [ ] Update landing `/search` results UI: show AgentRank score badge + trust tier + "Why this rank?" expandable

**Done when:** search results are ordered by composite score (not raw BM25); every result has `explain` payload; landing UI shows scores.

### Week 9 — Liveness + trust tiers

**Theme: dead agents drop; trusted agents rise. The index is alive.**

_Liveness probes (P0-07):_
- [ ] **L1 probe**: TCP+TLS connect to agent endpoint; record connect time, cert expiry; run every 15min for Hot tier, 1h for others
- [ ] **L2 probe**: HTTP GET agent card URL; validate JSON parses + required fields present; detect card changes (BLAKE3 hash)
- [ ] **L2/L3 fetch policy (same bar as agentbot):** probe HTTP clients **must** use the **same outbound URL policy** as ingest (`validate_fetch_url`, SSRF blocks, scheme allowlist, no internal IPs) — see `docs/security-fetch-policy.md`; **robots.txt** / **respect redirects** as for crawl; document in runbook **Probe fetch policy**
- [ ] **L3 probe:** minimal **non-destructive** A2A `message/send` (or documented health task) when endpoint exposes A2A; include `agentrank_probe` / `do_not_bill` metadata per opus §15; **skip** L3 if agent is MCP-only or no A2A URL — record `skipped_reason`
- [ ] **Probe scheduler**: background task in agentbot (or separate `probed` binary) that pulls agents due for probe from DB, executes L1/L2/L3, writes `probe_results` table
- [ ] **Status state machine** (opus §15.2): `HEALTHY → DEGRADED → UNHEALTHY → DEAD → DELISTED`; transitions based on consecutive failures; ranking penalties per state (0 / -10 / -30 / -50 / not searchable)
- [ ] **Freshness SLA**: status reflects reality within 15min; dead agent demoted within 1h

_Trust tiers (P0-08):_
- [ ] **Tier computation**: `Indexed` (default on first crawl), `Established` (card valid ≥7d + L2 pass rate >95% + ≥3 fields complete), `Verified` (domain claim verified + TLS valid + provider info present)
- [ ] **Trust floor at query time**: results with trust tier below query minimum get multiplicative penalty (0.5 for one tier below, 0.0 for two+)
- [ ] Store `trust_tier` and `liveness_status` in Tantivy fast fields; filter/boost in search
- [ ] Update search results: trust tier badge with explanation on landing UI; Console shows liveness timeline

**Done when:** probes run automatically; an agent that goes down transitions to UNHEALTHY within 15min and drops in search results; trust tiers compute correctly for ≥3 agents in staging; **L3** runs or **skips with reason** for each agent; **no probe** uses URLs outside **validate_fetch_url** policy.

### Week 10 — Console becomes useful

**Theme: providers can actually manage their agents through Console.**

_Console API (consoled):_
- [ ] **`POST /v1/console/enqueue`**: accept `{url, priority?}` → write to Redis frontier; requires auth; so operators/providers can trigger crawls from Console
- [ ] **`POST /v1/console/reindex`**: trigger Tantivy index rebuild (or incremental for specific agent); admin-only

_Search API auth (P0-09 — closes “API keys” for real users):_
- [ ] **`SEARCH_API_KEY`** (or shared **`AGENTRANK_API_KEY`**) on **searchd**: if `Authorization: Bearer` matches, apply **elevated** rate limit + optional metrics label `auth=api_key`; anonymous tier unchanged
- [ ] Document key issuance (env for pilot; future Stripe in Phase 2); OpenAPI security scheme `bearerAuth`
- [ ] **`GET /v1/console/agents`**: list agents for a claimed domain with scores, trust tier, liveness status, last crawl time
- [ ] **`GET /v1/console/agents/:id/scores`**: full AVERT breakdown + AgentRank + liveness history
- [ ] **Domain verification flow**: `POST /v1/console/domain-claims/:id/verify` — check DNS TXT record `_agentrank-verify=<token>` OR `/.well-known/agentrank-verify.json` containing token; transition claim to `verified`

_Console UI:_
- [ ] **Agents dashboard page** (`/agents`): table of claimed-domain agents with name, AgentRank score, trust tier, liveness status, last crawl, impressions (future); click → agent detail
- [ ] **Agent detail page** (`/agents/:id`): AVERT radar/bar chart, score history sparkline, liveness timeline, crawl history, card preview, "trigger re-crawl" button
- [ ] **Enqueue URL form**: simple "Submit agent URL" on Console home; calls enqueue endpoint
- [ ] **Domain verification wizard**: show verification token, DNS TXT instructions, "Verify now" button, status indicator
- [ ] **Polish**: move from `<pre>` JSON dumps to proper styled tables/cards; consistent nav; loading states

**Done when:** a provider can claim a domain, verify it via DNS TXT, see their agents with scores, and trigger a crawl — all from the Console UI.

### Week 11 — Landing page tells the story

**Theme: landing page becomes the front door. Explains the engine, lets people discover agents, links to Console.**

_Landing restructure:_
- [ ] **"How AgentRank works" section** on homepage: visual 4-step flow → (1) We crawl the web for agent cards, (2) We rank them with AVERT, (3) You search via web/API/MCP/A2A, (4) Scores update continuously
- [ ] **"For agent builders" CTA** on homepage: "List your agent" → links to Console or hints API; "Add AgentRank search to your agent" → MCP/A2A setup docs
- [ ] **Console link in NavBar**: "Console →" external link to Console URL (from env)
- [ ] **Submit agent URL** on landing: simple form on `/search` or dedicated `/submit` — calls hints API; "Know an agent? Help us discover it"

_Search experience upgrade:_
- [ ] **Rich search results**: each result card shows name, description snippet, AgentRank score (0-100 with color), trust tier badge (Indexed/Established/Verified), liveness indicator (green/yellow/red dot), protocol, endpoint domain
- [ ] **Agent detail page** (`/agents/:id`): real detail layout — header with score + trust + liveness; description; skills list; provider info; capabilities; "Connect" button/docs; "View raw card" toggle; link to Console for providers ("Claim this agent")
- [ ] **Search filters** (phase 1 of): trust tier minimum dropdown, protocol filter (A2A / MCP / any), sort by relevance / score / newest
- [ ] **Empty state**: when no results, show "Submit an agent" CTA + discovery tips

_Developer docs:_
- [ ] **`/docs` page or section** on landing: API usage (REST, MCP, A2A); code examples in Python/JS/curl; rate limits; authentication; link to OpenAPI spec
- [ ] **MCP integration guide**: step-by-step for Cursor, Claude, custom agent; copy-paste config

_GTM & legal (thin — “real users” bar; parallel with Week 11):_
- [ ] **Terms of Service** + **Privacy Policy** — published routes on landing (`/terms`, `/privacy`) or single hosted doc; linked from footer + signup flows
- [ ] **Crawl & index policy** — public page describing what AgentBot indexes, politeness, how to request **removal / opt-out** (email or form); link from footer and `/docs`
- [ ] **Abuse / copyright contact** — `abuse@` or form for DMCA-style takedowns (process can be manual v1)
- [ ] **Pricing / product** — single page (`/pricing` or section): free tier limits + how to get API key (even if “contact us” at first)
- [ ] **Status** — footer line “Status” linking to `status.*` stub, public RSS, or in-app **/status** “All systems operational” + last incident (manual ok for Phase 1)

**Done when:** a visitor landing on the homepage understands what AgentRank is in 10 seconds; can search and see ranked results with trust badges; can submit an agent URL; knows how to integrate via MCP/A2A; can navigate to Console; **legal links exist** so partners can sign up without embarrassment.

### Week 12 — Dedup + abuse + Phase 1 gate

**Theme: quality. The index is clean, spam is caught, and we can launch.**

_Entity resolution (P0-04):_
- [ ] **Cross-source dedup**: when ingesting an agent, check if `endpoint_url` or `canonical_url` already exists in DB → merge (keep highest-trust version, union skills, record all source URLs as aliases)
- [ ] **SimHash or content fingerprint** for near-duplicate detection: agents with >90% similar descriptions at same endpoint → flag for review or auto-merge
- [ ] Dedup rate metric: `agentrank_dedup_merges_total`

_Abuse resistance (P0-11):_
- [ ] **Spam scoring**: description keyword stuffing (>5 repetitions of same skill word = flag), ALL-CAPS, excessive emojis, contact info in description, skill count outlier (>50)
- [ ] **Impersonation check**: Levenshtein distance between new agent name and existing Verified agents; auto-quarantine if too similar + unverified
- [ ] **Quarantine status**: flagged agents are `visibility=quarantined` (not in search index); operators review in Console or auto-release after 7d if probe passes
- [ ] Synthetic spam test suite: 50+ fake agents with known spam patterns; measure catch rate ≥90%, FP <1%

_Phase 1 gate (aligned with **P0** — no waiver needed for these numbers):_
- [ ] **100+ real public agents indexed** (from autonomous crawl, not just seeds)
- [ ] **NDCG@10 ≥ 0.65** on **≥ 50** judged golden queries (hybrid + AVERT ordering) — matches **P0-05** Phase 1 clause
- [ ] **≥ 200** judged queries and **NDCG@10 ≥ 0.7** — **Phase 2** stretch (do not block Phase 1 launch)
- [ ] **Search availability >99.5%** over 7-day measurement window; **p95 search latency < 50ms** on documented staging load profile (matches **P0-09**)
- [ ] **Index freshness:** P95 lag from ingest to searchable **≤ 15 min** over 7 days (see **Index freshness SLO**)
- [ ] **MCP Milestone A** + **A2A Milestone B** complete (structured `message/send`); **Milestone C (NL)** explicitly shipped **or** documented as deferred
- [ ] **L1–L3** probes live with **probe fetch policy** enforced; demotion SLAs met in spot checks
- [ ] **API key** path live for `POST /v1/search` (partner pilot) — matches **P0-09**
- [ ] **Console**: at least one domain claimed + verified in production
- [ ] **Landing + GTM**: homepage → search → results → detail E2E; **Terms + Privacy + crawl policy** linked in footer
- [ ] Comms checklist: README updated, changelog, social announcement draft, **one** case study or “how we index” blog optional

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

### Week 19 — Enterprise (A) + domain verification

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

_Add bullets as `- [ ] …`; promote into a week when ready._

- [ ] Agent-sitemap.xml support: fetch `/agent-sitemap.xml` from domains with verified agents → extract card URLs → enqueue
- [ ] agent-robots.txt: parse agent-specific crawl directives alongside standard robots.txt
- [ ] GraphQL API for Console and integrators (opus §16.8)
- [ ] L3 probe: functional test (send simple A2A message, verify response)
- [ ] L4 probe: capability verification with generated test prompts (opus §15.3)
- [ ] Brokered connect v1 (P1-02): session broker, policy engine, mediation
- [ ] Cross-encoder reranker stage (opus §11.5): `ms-marco-MiniLM-L-6-v2` between RRF and LTR
- [ ] DNS TXT discovery: `_a2a._https` SRV records → card URL extraction
- [ ] Agent-proof.json verification: Ed25519 signature + BLAKE3 card hash (opus §8)
- [ ] Certificate Transparency log scanning for agent-related domains
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
| 2026-03-30 | **P0-09:** Anonymous + **optional API key** on searchd (not keys-only) | Preserves public search while satisfying “API key auth” as a **mode** for partners. |
| 2026-03-30 | **Week 7 staged:** MCP → structured A2A → NL last | De-risks shipping Rust MCP + A2A in one sprint; NL/LLM optional for gate. |
| 2026-03-29 | **Qdrant chosen over Milvus/Pinecone** for vector store | Self-hosted, Rust-native, good k8s/Docker support, free, gRPC + REST. Aligns with opus §19 stack choice. |

---

## Quick links (fill in for your org)

- Repo: 
- Staging URL: 
- Grafana: 
- Runbooks: 
- Golden query set location: 
