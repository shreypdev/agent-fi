# AgentRank — living execution todo

Companion to [`a2a-discovery-opus.md`](./a2a-discovery-opus.md) (architecture + [Milestones: Weekly build plan](./a2a-discovery-opus.md#milestones-weekly-build-plan)). **Edit this file every week:** check boxes, add notes, move spillover forward.

---

## How to maintain

1. **Start of week:** Set **Current week** below; copy this week’s unchecked items into GitHub Issues / Linear if you use a tracker.
2. **During the week:** Check off completed items; add `Note:` lines under any item (indent with two spaces) for decisions, PR links, or scope cuts.
3. **End of week:** Roll unfinished items to the next week or **Backlog** with a one-line reason; update **Last updated** date.
4. **Gates:** Do not mark a phase “complete” until the **Done when** criteria in the opus milestones are true (or explicitly waived in writing in **Decisions & waivers**).

---

## Meta (update when you touch this file)

| Field | Value |
|--------|--------|
| **Current week** | Week 5 of 52 |
| **Phase focus** | ☑ Phase 0 · ☑ Phase 1 (W5 slice) · ☐ Phase 2 · ☐ Phase 3 |
| **Last updated** | 2026-03-29 |
| **Owner / DRI** | |

---

## Ongoing (every week)

- [ ] Security: dependency / image triage; no known criticals unpatched in prod
- [ ] Data: backup restore spot check or automated proof artifact
- [ ] Quality: golden-query regression on release candidate
- [ ] Incidents: postmortem filed for any user-visible SLO miss

---

## Open discovery & crawler (north star)

**Goal:** **Open discovery** — find and refresh **public** agents that publish A2A-style metadata on the open web — **not** a closed directory only. This is **not** “crawl every HTML page like a generic search engine”; it **is** “politely crawl the **A2A discovery surface**” (well-known card URLs, and later protocol-native expansion below).

### What we are building (end-state mental model)

1. **Faucets (always refilling the frontier)**  
   Multiple sources normalize to **canonical card URLs** and **enqueue** into Redis (priority frontier): curated seeds, **registry JSON feeds**, **GitHub / code-host** patterns, **DNS TXT / discovery hints** (Week 6), partner lists, etc. **P0-02** (≥5 registry sources) and **Week 5–6** work live here.

2. **Protocol-native expansion (“search-engine-like,” but narrow)**  
   From fetched **Agent Cards** and (per opus) **`agent-sitemap.xml`** / related discovery docs, extract **more candidate card URLs** and enqueue them — same idea as **following links**, but the **link grammar is A2A discovery**, not arbitrary site graphs.

3. **Worker loop (AgentBot today; richer tomorrow)**  
   **Dequeue** → **`robots.txt`** (standard today; **`agent-robots.txt`** roadmap in opus) → **outbound URL policy** → **GET card** → **parse / validate** → **Postgres** + **`crawl_history`** → **re-enqueue** for freshness / backoff → **metrics**. Politeness (**per-host limits**, robots) stays non-negotiable as breadth grows.

4. **Downstream**  
   **Index** (Tantivy) must reflect DB changes (**rebuild / incremental** — operational today). **Dedupe, trust, abuse** (**P0-04**, **P0-08**, **P0-11**) prevent “open” from becoming low-signal noise.

### Where we are now vs P0-01

| Today (after Week 4–5) | Toward **P0-01** |
| ---------------------- | ---------------- |
| Frontier + `run-loop`, `robots.txt`, rate limits, SSRF policy, `discover` (builtin / HTTP JSON / file / GitHub) | More **autonomous** faucets + **scheduled** discover + **expansion** from sitemaps/cards + **recrawl** policy + scale tests against “known public” pilot set |

**P0-01** is intentionally the **autonomous-crawl gate**; Weeks 4–5 delivered the **engine** and **first faucets** — not the full breadth yet.

---

## Product requirements traceability (opus §4)

### P0 — must ship (MVP)

- [ ] **P0-01** Autonomous crawl of `/.well-known/agent.json` (90%+ of known public agents within 72h of seed init — define “known” for your pilot) — *see **Open discovery & crawler** above; this is open-web, protocol-bounded discovery, not a generic HTML spider.*
- [ ] **P0-02** Registry feed ingestion (≥5 sources: AgentVerse, PulseMCP, Moltbook, HuggingFace, CrewAI Hub — adjust names if APIs change)
- [ ] **P0-03** Agent Card parse + validate + normalize (99%+ valid formats; invalid → evidence, not index)
- [ ] **P0-04** Canonical registry + entity resolution (95%+ dup catch across sources)
- [ ] **P0-05** Hybrid search lexical + semantic (NDCG@10 ≥ 0.7 on ≥200 judged queries)
- [ ] **P0-06** AgentRank v1 transparent scoring + explanation payload on results
- [ ] **P0-07** Multi-level liveness (L1–L3 min); status fresh within 15m of change; dead demoted within 1h
- [ ] **P0-08** Trust tiers (Indexed → Established → Verified min); criteria documented
- [ ] **P0-09** `POST /v1/search` — trust annotations, p95 under 50ms, API key auth, stable schema
- [ ] **P0-10** Agent Search Console v1 — claim domain, crawl/index status, score visibility
- [ ] **P0-11** Basic abuse resistance (dup, squatting, stuffing, impersonation); FP under 1%, catch 90%+ synthetic spam tests
- [ ] **P0-12** Public web UI — search, results, detail pages, trust badges

### P1 — fast follow (target: 4–6 weeks after MVP)

- [ ] **P1-01** Direct connect — connection metadata on ≥95% of healthy results
- [ ] **P1-02** Brokered connect — 90%+ success compatible pairs (staging → prod)
- [ ] **P1-03** Outcome telemetry API — ingest, classify, store
- [ ] **P1-04** Domain verification — DNS TXT and/or `agent-proof.json`; within 5 minutes after proof
- [ ] **P1-05** Signed metadata — verify + trust boost + docs
- [ ] **P1-06** Graph retrieval — measurable NDCG lift vs lexical+semantic only
- [ ] **P1-07** Benchmark framework v1 — conformance + non-destructive capability signals
- [ ] **P1-08** A2A-native self-discovery — AgentRank at `/.well-known/agent.json`

### P2 — strategic (3–6 months)

- [ ] **P2-01** Enterprise private registries + tenant isolation (zero cross-tenant leakage)
- [ ] **P2-02** Federation pull + push — ≥2 live partners
- [ ] **P2-03** Policy-aware search (residency, compliance packs)
- [ ] **P2-04** Learning-to-rank — significant offline NDCG vs heuristic
- [ ] **P2-05** Outcome-driven ranking in production mix
- [ ] **P2-06** Agent Search Console v2 — benchmarking, query analytics, recommendations

---

## Phase 0 — Weeks 1–4 (vertical slice)

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
- [x] SSRF / fetch policy documented and tested (searchd does not fetch user URLs; **agentbot** ingest is the surface — see `apps/agentrank/README.md`, `docs/security-fetch-policy.md`)
- [x] searchd Prometheus `/metrics` + HTTP counters/histograms (Grafana/SLOs optional)
- [x] Agent Search Console v0.1: domain claim path, card inspector, crawl history (`agentrank-consoled`, `apps/console`)
- [x] Prometheus metrics per service; Grafana dashboards as code; alert narrative in `docs/grafana/README.md`
- [x] **Phase 0 gate:** script `apps/agentrank/scripts/phase0_gate.sh` + seed file (see **Decisions** for 100-URL curation waiver)

---

## Phase 1 — Weeks 5–12 (scale + hybrid + trust + connect)

### Week 5 — Seed explosion (A)

- [x] ≥3 registry connectors hardened + runbooks (`builtin_demo_seed`, `http_json_urls`, `static_json_file`; `docs/runbooks/registry-*.md`)
- [x] GitHub (or code host) discovery MVP for card URLs (`agentbot discover github`, wiremock tests)
- [x] Frontier metrics: URLs discovered / enqueued / dup rate (Prometheus + Grafana panels + `docs/grafana/README.md`)

### Week 6 — Seed explosion (B) + vectors

- [ ] DNS TXT / discovery hints MVP
- [ ] Qdrant (or chosen vector store) deployed; health + backup story
- [ ] Embedding pipeline: batch backfill + hook on new/updated agents
- [ ] Hybrid retrieval live behind same search API; small labeled set shows win vs BM25-only

### Week 7 — Ranking v1

- [ ] AVERT v1 signals computed and stored
- [ ] Search uses composite score; deterministic recomputation
- [ ] Explanation payload: top 3–5 factors per result

### Week 8 — Liveness

- [ ] Probe scheduler; L1–L3 probes defined
- [ ] Status state machine + persistence
- [ ] Ranking demotion rules for unhealthy agents

### Week 9 — Connect loop (A)

- [ ] Search response includes direct-connect fields (endpoint, auth profile, transport)
- [ ] Brokered connect v0.1: session create, handoff, stub policies
- [ ] Staging E2E: search → connect with test agents

### Week 10 — Trust + spam

- [ ] Domain verification: ≥2 methods implemented
- [ ] TLS metadata captured where applicable
- [ ] Spam v1 rules + quarantine/demotion; measure FP on fixtures

### Week 11 — Query understanding

- [ ] Intent classifier + frozen eval set
- [ ] Query expansion; optional structured filters (modality, auth family)

### Week 12 — Console + launch hardening

- [ ] Console v1.1: trust dashboard, impressions, liveness history
- [ ] Load test; error budget / SLO dashboard
- [ ] **Phase 1 gate:** 100K+ indexed; NDCG@10 above 0.70; search availability above 99.5% (launch window); comms checklist

---

## Phase 2 — Weeks 13–24 (LTR, enterprise, federation, revenue)

### Week 13 — LTR foundation

- [ ] Feature logging from search + outcomes (schema stable)
- [ ] Offline dataset builder; reproducible NDCG script
- [ ] CI job publishes NDCG artifact

### Week 14 — LTR model

- [ ] Train ranker (e.g. XGBoost); ONNX export; version pinning
- [ ] Shadow scoring path; offline gain vs AVERT-only (pre-agreed threshold)

### Week 15 — LTR production

- [ ] Gradual rollout + automatic fallback to heuristic
- [ ] Monitoring: drift, latency, error rate on model path

### Week 16 — Outcome ranking

- [ ] Outcome features documented for API clients
- [ ] Offline replay shows win vs week 15 slice

### Week 17 — Anti-abuse (A)

- [ ] Link farm / mutual citation detection v1
- [ ] Graph-based demotion or feature injection

### Week 18 — Anti-abuse (B) + benchmarks

- [ ] CTR / manipulation heuristics; behavioral anomaly flags
- [ ] Benchmark framework v1; 50+ conformance tests; scheduled reliability runs

### Week 19 — Enterprise (A)

- [ ] Tenant model + isolation strategy implemented
- [ ] API RBAC: two tenants cannot read each other’s data (test)

### Week 20 — Enterprise (B)

- [ ] Private ingest path for tenant-only agents
- [ ] mTLS or enterprise auth for tenant APIs (pilot config)

### Week 21 — Federation

- [ ] Pull connector ≥1 partner; dedup vs crawl; connector health metrics
- [ ] 50K+ federated agents in staging without dup explosion

### Week 22 — Monetization (A)

- [ ] Promoted slot design + policy (disclosure to users)
- [ ] Click / impression logging; demo auction path (manual ok)

### Week 23 — Monetization (B)

- [ ] Billing integration or invoice export
- [ ] Click fraud checks; reconciliation run

### Week 24 — Analytics + Phase 2 gate

- [ ] Premium analytics surfaces (Console or separate)
- [ ] Cost / unit economics dashboard
- [ ] **Phase 2 gate:** 1M+ agents; NDCG@10 above 0.75; enterprise pilot; MRR above $100K (or written waiver + new date)

---

## Phase 3 — Weeks 25–52 (scale + moat)

_Use one checkbox per week as the “theme done” tracker; expand sub-tasks in **Backlog** if needed._

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
- [ ] **W43** MCP discovery (A) — normalize into index
- [ ] **W44** MCP discovery (B) — tool-level index
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

- _(none yet)_

---

## Blockers & questions

| Date | Blocker | Owner | Status |
|------|---------|-------|--------|
| | | | |

---

## Decisions & waivers

_Log scope cuts and criterion waivers so the team does not “forget” a gate._

| Date | Decision | Rationale |
|------|-----------|-----------|
| 2026-03-28 | Phase 0 **100 real URL** seed list is **operator-curated**, not fully populated in-repo | Public A2A card URLs are sparse and rot; repo ships `tests/fixtures/phase0_seed_urls.txt` as a **vetted starter** plus `phase0_gate.sh`. Expand the file (or use `discover file` / HTTP feeds) before calling the gate “100-URL complete.” |
| 2026-03-28 | Grafana **alert** is documented as operator wiring (webhook not in git) | Matches plan: one rule narrative in `docs/grafana/README.md`; wire in Grafana Cloud / Alertmanager to your channel. |

---

## Quick links (fill in for your org)

- Repo: 
- Staging URL: 
- Grafana: 
- Runbooks: 
- Golden query set location: 
