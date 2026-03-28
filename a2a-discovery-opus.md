# AgentRank: The Definitive Search Engine for AI Agents

## A Complete Technical Architecture for Autonomous Discovery, Ranking, Connection, and Trust at Internet Scale

**Version:** 0.1.0-draft
**Date:** March 2026
**Classification:** Internal — Founding Technical Document
**Status:** Master synthesis — combines and resolves three prior architecture proposals into a single canonical reference

---

## Table of Contents

### Part I: Strategy and Product

1. [Executive Summary](#1-executive-summary)
2. [Market Analysis and Problem Statement](#2-market-analysis-and-problem-statement)
   - 2.1 Current Landscape
   - 2.2 The Five Fundamental Problems
   - 2.3 Competitive Landscape Analysis
   - 2.4 Why Now
   - 2.5 The Opportunity
3. [Product Vision and Principles](#3-product-vision-and-principles)
   - 3.1 Vision Statement
   - 3.2 Product Positioning
   - 3.3 Design Principles
   - 3.4 Web Search Analogy Table
   - 3.5 Market Thesis
4. [Product Requirements](#4-product-requirements)
   - 4.1 Core User Personas
   - 4.2 P0 Requirements (Must Ship)
   - 4.3 P1 Requirements (Fast Follow)
   - 4.4 P2 Requirements (Strategic)
   - 4.5 Non-Functional Requirements
5. [Scope and Non-Goals](#5-scope-and-non-goals)

**[Milestones — Weekly build plan](#milestones-weekly-build-plan)** — week-by-week execution checklist (Year 1).

### Part II: Discovery and Crawling

6. Protocol Foundation and Discovery Compatibility Strategy
   - 6.1 The Agent Card as Discovery Root
   - 6.2 Spec and Convention Drift Resolution
   - 6.3 Compatibility Matrix and Canonicalization Rules
   - 6.4 Discovery Extensions to Standardize
7. Discovery Model
   - 7.1 Discovery Source Taxonomy
   - 7.2 Discovery Leads vs Registration
   - 7.3 Ten-Stage Discovery Pipeline
   - 7.4 Strategic Discovery Insights
8. Crawl Architecture
   - 8.1 Frontier Goals and Partitioning
   - 8.2 Frontier Priority Scoring Function
   - 8.3 Fetch Rules and Politeness
   - 8.4 Failure Handling State Machine
   - 8.5 Crawler Service Decomposition
   - 8.6 Scale Targets

### Part III: Registry and Entity Resolution

9. Parsing, Validation, and Normalization
   - 9.1 Four-Layer Validation Stack
   - 9.2 Normalization Rules
   - 9.3 Extraction Outputs
10. Canonical Registry
    - 10.1 Registry Purpose and Core Entities
    - 10.2 Entity Resolution Strategy
    - 10.3 Evidence Model
    - 10.4 Visibility Model

### Part IV: Search, Ranking, and Trust

11. Search and Retrieval Architecture
    - 11.1 Multi-Stage Retrieval Pipeline
    - 11.2 Query Understanding
    - 11.3 Candidate Generation Modes
    - 11.4 Result Fusion
    - 11.5 Re-Ranking Roadmap
    - 11.6 Explainability
12. Ranking System: AgentRank
    - 12.1 AgentRank Composite Score
    - 12.2 Search Ranking Score vs Discoverability Score
    - 12.3 The AVERT Ranking Framework
    - 12.4 Signal Families and Feature Dictionary
    - 12.5 Transparent v1 Formula
    - 12.6 Abuse Resistance
    - 12.7 Learning-to-Rank Roadmap
13. Trust, Verification, and Abuse Resistance
    - 13.1 Threat Model
    - 13.2 Trust Layers
    - 13.3 Trust Tiers
    - 13.4 Verification Methods
    - 13.5 Abuse Controls

### Part V: Liveness, Connection, and Outcomes

14. Liveness, Benchmarking, and Outcome Quality
    - 14.1 Multi-Level Liveness Probing
    - 14.2 Status State Machine
    - 14.3 Capability Verification
    - 14.4 Benchmark Types
    - 14.5 Outcome Telemetry and Signal Trust
15. Connection and Orchestration Plane
    - 15.1 Why Connection Matters
    - 15.2 Direct Connect vs Brokered Connect
    - 15.3 Connect Decision Flow
    - 15.4 Outcome Capture Loop

### Part VI: Enterprise and Provider Experience

16. Enterprise, Federation, and Private Discovery
    - 16.1 Federation Modes
    - 16.2 Enterprise Requirements
    - 16.3 Public vs Private Ranking Isolation
    - 16.4 Access Control Model
17. Provider Experience: Agent Search Console
    - 17.1 Core Features
    - 17.2 Discoverability Score Breakdown
    - 17.3 Ecosystem Flywheel

### Part VII: APIs and Data Platform

18. Public APIs and Protocol Contracts
    - 18.1 Core API Surface
    - 18.2 Search API Contract
    - 18.3 Connect API Contract
    - 18.4 Outcome API Contract
    - 18.5 A2A-Native Self-Discovery
19. Data Platform and Storage Architecture
    - 19.1 Storage Role Assignment
    - 19.2 Graph Computation Strategy
    - 19.3 Data Lineage Requirements

### Part VIII: Infrastructure

20. Service Architecture and Language Decisions
    - 20.1 Polyglot Architecture with Sharp Boundaries
    - 20.2 Language Justifications
    - 20.3 Service Decomposition
21. Security Architecture
    - 21.1 Threat Model
    - 21.2 Security Controls
    - 21.3 Auth and Access Tiers
    - 21.4 Auditability
22. Privacy, Governance, and Compliance
    - 22.1 Data Classification
    - 22.2 Privacy Principles
    - 22.3 Retention Policy
    - 22.4 Governance Bodies

### Part IX: Operations and Growth

23. Observability, Evaluation, and Experimentation
    - 23.1 Core Metrics by Surface
    - 23.2 Evaluation Framework
    - 23.3 Experimentation Rules
24. Monetization and Business Model
    - 24.1 Revenue Streams
    - 24.2 Anti-Patterns to Avoid
    - 24.3 Strategic Position
25. Rollout Plan and Phasing
    - 25.1 Phase 0: Foundations
    - 25.2 Phase 1: Search Quality
    - 25.3 Phase 2: Trust and Connect
    - 25.4 Phase 3: Enterprise Federation
    - 25.5 Phase 4: Learning Network Effects
26. Team Topology
27. Key Risks and Mitigations
28. Category Strategy and Long-Term Moat
    - 28.1 The Compounding Loop
    - 28.2 Moat Components
    - 28.3 What Competitors Will Miss

### Appendices

29. Appendix A: Canonical API Shapes
30. Appendix B: Canonical Schema Guidance
31. Appendix C: Discovery Extensions to Standardize
32. Appendix D: AgentRank Feature Dictionary
33. Appendix E: Web Search Analogy — Full Mapping
34. Appendix F: Glossary

---

## 1. Executive Summary

The agent ecosystem is at an inflection point. As of March 2026, over **104,000 publicly listed AI agents** are scattered across **15+ independent registries** with **zero interoperability** between them. There is no unified way to discover an agent, no way to evaluate whether it is trustworthy, no way to know whether it is alive, and no programmatic way for one agent to find another.

### The Landscape Today

| Registry | Listed Agents | Market Share |
|----------|--------------|-------------|
| AgentVerse | 36,338 | 34.8% |
| ERC-8004 (on-chain) | 18,344 | 17.6% |
| PulseMCP | 16,510 | 15.8% |
| Moltbook | 14,436 | 13.8% |
| x402 Bazaar | 7,606 | 7.3% |
| Others (10+ registries) | 10,766 | 10.4% |
| **Total** | **104,000+** | **100%** |

Every one of these registries operates as a manual submission directory. Agents are listed because a human filled out a form. Metadata is self-reported and unverified. There is no quality signal, no liveness guarantee, no semantic search, no ranking. Most registries are flat lists sorted by submission date or alphabetical order.

This is the equivalent of the early web before Google — millions of pages, a handful of manually curated directories (Yahoo, DMOZ), and no scalable way to find anything.

### The Core Thesis

**Manual registration does not scale. Crawling does.**

The web did not organize itself through Yahoo submission forms. It organized itself when Googlebot crawled the web, built an index, and ranked pages by quality. The agent ecosystem will follow the same trajectory. The question is not whether a search-engine-style discovery layer will exist — the question is who will build it first.

### What AgentRank Is

AgentRank is not a registry. It is not a directory. It is not a search box. It is the **DNS + Google + connection broker for the agentic web** — an integrated platform with six pillars:

1. **Discovery Engine** — Continuously discovers agents across the public internet and federated private networks without requiring manual registration. Crawls well-known URIs, registry feeds, DNS records, certificate transparency logs, code repositories, and the agent interaction graph itself.

2. **Canonical Registry** — Builds a versioned, evidence-backed, deduplicated source of truth that merges fragmented records from multiple sources into a single canonical identity per agent. Every field is traceable to its source evidence.

3. **Ranking Engine (AgentRank)** — Scores agents across nine dimensions: task relevance, compatibility, trust, operational quality, outcome success, authority, freshness, economic efficiency, and documentation quality. The ranking is transparent and explainable, not a black box.

4. **Trust and Verification System** — Measures identity provenance, metadata integrity, operational reliability, protocol conformance, and ecosystem reputation. Assigns trust tiers (Indexed → Established → Verified → Trusted → Authoritative) that gate ranking influence and UI treatment.

5. **Connection Broker** — Mediates the path from discovery to successful connection: resolves endpoints, validates compatibility, negotiates auth, enforces policy, and captures outcome telemetry. Supports both direct connect (caller goes straight to the agent) and brokered connect (platform mediates setup).

6. **Agent Search Console** — A provider-facing optimization platform, analogous to Google Search Console, where agent operators can verify domain ownership, inspect crawl/index status, view their AgentRank scores, diagnose discoverability issues, and improve metadata quality.

### The Closed Loop

The critical strategic insight — and the primary differentiator against every competitor — is the **closed outcome loop**:

```
discovery → ranking → selection → connection → outcome → trust → better ranking
```

Search alone is not enough. The market winner will own the full lifecycle from discovering an agent to confirming that the agent successfully completed the task. Each completed interaction makes the ranking smarter, the trust signals stronger, and the connection faster. This loop is the moat.

### Timing

There is a **12–18 month window** before the ecosystem fragments irreversibly or a hyperscaler locks down discovery within its own agent stack. A2A v1.0 shipped in March 2026 with standardized Agent Cards and the `/.well-known/agent.json` convention, making agents crawlable by design for the first time. Over 50 enterprise partners (Atlassian, Salesforce, ServiceNow, Deloitte) have committed to A2A. The Agentic AI Foundation (146 member companies) has produced no DNS-equivalent for agents. The window is open.

### Technology Stack

| Layer | Technology | Justification |
|-------|-----------|---------------|
| Core services | **Rust** | Memory safety, zero-cost abstractions, predictable latency for the online path. Crawler, search API, connect broker, and registry all benefit from Rust's performance guarantees at the concurrency levels required (50K+ fetches/sec, 10K QPS search). |
| Search index | **Tantivy** | Rust-native full-text search engine. Eliminates JVM operational overhead of Lucene-based alternatives. Embeddable, fast, and the team controls the binary. |
| Vector search | **Qdrant** | Rust-native vector database for semantic retrieval. Supports filtering, payload indexing, and hybrid search. Avoids the operational weight of Vespa while covering the embedding retrieval use case. |
| Event streaming | **Kafka** | Battle-tested for high-throughput event pipelines. Crawl events, parse events, health signals, benchmark results, connect outcomes, and feature updates all flow through Kafka topics. |
| Relational store | **PostgreSQL** | Canonical registry, workflow state, tenant configuration, policy metadata, and control-plane state. The most reliable relational database in existence. |
| Analytics | **ClickHouse** | Columnar analytics for telemetry, ranking feature computation, dashboards, and experiment analysis. Handles the append-heavy, read-heavy workload that ranking feature pipelines produce. |
| Object storage | **S3 / R2 / GCS** | Raw crawled artifacts, validation snapshots, benchmark logs, signature proofs, and audit exports. Immutable evidence archive. |
| ML and evaluation | **Python** | Offline ML pipelines, embedding generation, labeling tools, LTR training, and evaluation harnesses. Python stays off the hot path entirely. |
| Provider console | **TypeScript / React** | Agent Search Console and internal admin UIs. Standard web frontend stack. |

**The operating principle:** Rust for anything on the hot path or requiring correctness guarantees. Python for anything that runs in batch. TypeScript for anything a human looks at.

---

## 2. Market Analysis and Problem Statement

### 2.1 Current Landscape

The agent registry market as of March 2026 is fragmented, shallow, and growing fast. No registry provides search. No registry provides ranking. No registry guarantees that listed agents are alive.

#### Registry Census

| Registry | Agents | Type | Discovery Model | Search | Ranking | Liveness | Auth |
|----------|--------|------|----------------|--------|---------|----------|------|
| **AgentVerse** | 36,338 | General marketplace | Manual submission | Tag filter | None (recency sort) | None | API key |
| **ERC-8004** | 18,344 | On-chain registry | Smart contract registration | None | None | None | Wallet signature |
| **PulseMCP** | 16,510 | MCP-focused directory | Manual + GitHub PR | Keyword | None | None | None |
| **Moltbook** | 14,436 | Agent directory | Manual submission | Tag filter | Star rating (self-reported) | None | None |
| **x402 Bazaar** | 7,606 | Pay-per-use marketplace | Manual listing | Category browse | Price sort | None | x402 payment header |
| **Hugging Face Agents** | 3,200+ | Model-linked agents | Model card extension | HF search | Downloads sort | None | HF token |
| **A2A Registry (a2a-registry.org)** | ~800 | A2A-specific directory | Manual submission | None | None | None | None |
| **DUADP Network** | ~36 | DNS-based discovery | DNS TXT + WebFinger + DIDs | None | None | Partial (DNS TTL) | DID-based |
| **MCP DNS Registry** | ~150 | DNS-based MCP directory | DNS TXT records | None | None | None | None |
| **CrewAI Hub** | 1,200+ | Framework-specific | Manual + CLI publish | Keyword | Usage count | None | CrewAI auth |
| **LangChain Hub** | 800+ | Framework-specific | Manual + CLI publish | Keyword | Downloads | None | LangSmith auth |
| **Others (5+ registries)** | ~5,580 | Various | Various | Various | None | None | Various |
| **TOTAL** | **104,000+** | | | | | | |

**Key observations:**

- **Zero registries provide semantic search.** Every registry that has search uses keyword matching or tag filtering. None understand natural-language intent like "find an agent that can review Terraform plans for security compliance."
- **Zero registries provide quality ranking.** Sorting by recency, downloads, or self-reported star ratings is not ranking. There is no equivalent of PageRank, no evidence-based quality signal, no way to distinguish a production-grade agent from a weekend experiment.
- **Zero registries guarantee liveness.** When an agent goes offline, its listing persists indefinitely. Registries are graveyards of stale endpoints.
- **Zero registries support programmatic agent-to-agent discovery.** Every registry is designed for humans browsing a web UI. No registry exposes a structured API that an agent can call at runtime to find another agent by capability.

### 2.2 The Five Fundamental Problems

#### Problem 1: Manual Registration Does Not Scale

The web did not organize itself through the Yahoo directory submission form. By the time Yahoo had 1 million manually curated listings, the web had 100 million pages. The gap was unbridgeable by human curation.

The agent ecosystem is on the same trajectory. Manual submission registries will capture a fraction of total agents. The fraction will shrink as agents proliferate. Coverage will be biased toward agents backed by well-resourced teams that know about and bother to submit to registries. The long tail — which on the web contained the most valuable specialized content — will be invisible.

**The solution is crawling.** A2A's `/.well-known/agent.json` convention makes agents crawlable by design. Any domain hosting an A2A-compatible agent can be discovered without the operator submitting a form. This is the Googlebot moment for agents.

#### Problem 2: No Quality Signal

Every existing registry presents a flat list. There is no equivalent of PageRank — no signal that distinguishes a highly reliable, well-documented, production-grade agent from a broken prototype someone deployed once and forgot about.

The absence of quality signal has three consequences:

1. **Callers cannot evaluate options.** A search for "code review agent" returns dozens of results with no way to assess which ones are worth trying.
2. **Good agents are drowned by noise.** A high-quality agent from a reputable provider appears alongside abandoned experiments and outright spam.
3. **There is no incentive to improve.** Without a visible quality score, providers have no signal telling them what to fix. There is no "Agent SEO" because there is no search engine to optimize for.

**The solution is multi-signal ranking.** AgentRank combines task relevance, trust, operational quality, outcome success, authority, freshness, compatibility, economic efficiency, and documentation quality into a single composite score. This score is transparent: providers can see what factors contributed to their ranking and what they need to improve.

#### Problem 3: No Semantic Understanding

Keyword registries cannot understand intent. When a calling agent needs "an agent that can analyze AWS CloudFormation templates for cost optimization," a keyword search for those terms will miss agents described as "infrastructure cost analyzer" or "cloud spend optimization service" even though they are perfect matches.

The gap between how callers describe their needs and how providers describe their agents is the vocabulary mismatch problem — the same problem that led the web search industry from keyword matching to semantic retrieval.

**The solution is hybrid retrieval.** AgentRank combines lexical matching (BM25 over agent names, descriptions, skills, and documentation), semantic matching (dense embeddings over agent descriptions, skill summaries, and example queries), graph matching (provider relationships, invocation co-occurrence, workflow adjacency), and structured constraint matching (auth, modality, region, compliance, SLA). These four retrieval modes run in parallel and their results are fused using Reciprocal Rank Fusion in v1, with learned fusion planned for v2.

#### Problem 4: No Liveness Guarantee

Registries are graveyards. An agent listed in AgentVerse six months ago may be long dead — the endpoint returning 404, the domain expired, the maintainer moved on. But the listing persists with no indication that the agent is unreachable.

This is not a minor inconvenience. For an autonomous agent making a runtime decision about which agent to delegate a subtask to, connecting to a dead endpoint means wasted time, failed fallbacks, and degraded user experience.

**The solution is continuous liveness probing.** AgentRank maintains a multi-level health system:

- **L1:** TCP/TLS reachability — is the endpoint up?
- **L2:** Metadata fetch — does the agent card still validate?
- **L3:** Protocol handshake — can we establish an A2A session?
- **L4:** Safe capability probe — does the agent respond correctly to a benign test?

Each agent has a health status (Healthy → Degraded → Unhealthy → Dead → Delisted) that directly influences its ranking. Dead agents are demoted. Chronically unhealthy agents are delisted. Health history is a ranking signal.

#### Problem 5: Agents Cannot Search

No existing registry provides a programmatic, A2A-native discovery interface. Every registry is designed for humans clicking through web pages. If an agent at runtime needs to find the best agent for a subtask, it has no protocol-native way to query a discovery service, receive ranked results with trust and compatibility information, and initiate a connection — all within the A2A interaction model.

**The solution is agent-first API design.** AgentRank's primary interface is `POST /v1/search` — a structured API designed for machines. The platform itself is an A2A-discoverable agent, meaning agents can find AgentRank the same way they find any other agent. The search API returns ranked results with trust tiers, compatibility checks, health status, and connection metadata — everything a calling agent needs to make an autonomous delegation decision.

### 2.3 Competitive Landscape Analysis

Four categories of competitors exist today. None address the full problem space.

#### A2A Registry (a2a-registry.org)

**What it is:** A community-maintained directory of A2A-compatible agents. Approximately 800 agents listed as of March 2026.

**Strengths:**
- A2A-native (understands Agent Card format)
- Low barrier to submission
- Community goodwill as "the official registry"

**Weaknesses:**
- Manual submission only — no crawling
- No search at all — browse by category or scroll the list
- No ranking — agents are listed in submission order
- No liveness verification — dead agents persist
- No semantic understanding — no natural-language query support
- No trust system — every listing is equally "trusted"
- Centralized and fragile — single maintainer, GitHub-based, PR-merge workflow
- No API for programmatic discovery

**Strategic assessment:** A2A Registry is the Yahoo of agents. It will serve the ecosystem at current scale (hundreds of agents) but will not survive the transition to thousands. Its community legitimacy is a risk if it becomes "good enough" by default, but its architecture fundamentally cannot scale beyond manual curation.

#### DUADP (Decentralized Universal Agent Discovery Protocol)

**What it is:** A DNS-based, decentralized discovery protocol combining DNS TXT records, WebFinger, DIDs, and a gossip-based peer network.

**Strengths:**
- Decentralized — no single point of failure
- DNS-native — leverages existing internet infrastructure
- DID-based identity — strong cryptographic provenance
- Gossip protocol for peer discovery
- Standards-aligned philosophy

**Weaknesses:**
- Extremely complex — requires DNS TXT configuration, WebFinger server, DID resolver, and gossip node
- No ranking whatsoever — discovery returns unranked results
- No semantic search — resolution is by exact DID or DNS lookup
- Only 36 resources in the network as of March 2026
- No quality signal, no liveness probing, no outcome tracking
- No agent-specific understanding — treats agents as generic resources
- Operator burden is prohibitively high for adoption

**Strategic assessment:** DUADP solves the decentralization problem but ignores the ranking problem. In web search terms, it is DNS without Google — it can resolve names but cannot tell you which result is best. Its complexity severely limits adoption. We should support DNS-based discovery hints as a seed source without adopting the full DUADP stack.

#### MCP DNS Registry

**What it is:** A simple DNS-based registry for MCP servers, using TXT records at `_mcp._tcp.<domain>` to advertise MCP endpoint metadata.

**Strengths:**
- Simple — a single DNS TXT record per server
- Zero registration friction
- DNS infrastructure is universally available

**Weaknesses:**
- MCP-only — no A2A understanding
- No search — resolution is by domain name only
- No ranking — all results are equal
- No liveness — DNS TTL is the only freshness signal
- No semantic understanding
- No trust system
- No multi-agent discovery — you must know the domain to look up

**Strategic assessment:** MCP DNS Registry solves one narrow problem (looking up a known MCP server by domain) and does it elegantly. But it is not a discovery system — you must already know what you are looking for. We should ingest DNS TXT records as a seed source for our crawler.

#### Gap Analysis Matrix

| Dimension | A2A Registry | DUADP | MCP DNS | AgentRank |
|-----------|-------------|-------|---------|-----------|
| **Autonomous discovery** (crawling, no manual submission) | ❌ Manual only | ⚠️ DNS-based but requires operator config | ⚠️ DNS-based but requires operator config | ✅ Full web-scale crawling |
| **Semantic search** (natural-language query) | ❌ None | ❌ None | ❌ None | ✅ Hybrid lexical + semantic + graph |
| **Quality ranking** (evidence-based, multi-signal) | ❌ None | ❌ None | ❌ None | ✅ 9-dimension AgentRank |
| **Liveness verification** (continuous health probing) | ❌ None | ❌ None | ❌ None | ✅ 4-level health system |
| **Trust and verification** (identity, provenance, abuse) | ❌ None | ⚠️ DID-based identity only | ❌ None | ✅ Full trust tier system |
| **Connection brokering** (auth, policy, outcome) | ❌ None | ❌ None | ❌ None | ✅ Direct + brokered connect |
| **Agent-native API** (programmatic A2A-native search) | ❌ None | ⚠️ Gossip-based resolution | ❌ None | ✅ A2A-native search API |
| **Scale capacity** | ~1K (manual ceiling) | ~100 (complexity ceiling) | ~1K (DNS management ceiling) | 10M+ (crawling architecture) |
| **Provider optimization** | ❌ None | ❌ None | ❌ None | ✅ Agent Search Console |

**The gap is total.** No competitor addresses more than two of these seven dimensions. AgentRank addresses all seven by design.

### 2.4 Why Now

Four forces have converged to create a window of exactly the right width:

**1. A2A v1.0 shipped with standardized Agent Cards (March 2026).**
The `/.well-known/agent.json` convention is the `robots.txt` + `sitemap.xml` moment for agents. For the first time, agent metadata is published at predictable URIs on the open web, making agents crawlable by design. Before this convention, there was nothing to crawl. Now there is.

**2. Enterprise adoption is accelerating.**
Over 50 enterprise partners — including Atlassian, Salesforce, ServiceNow, Deloitte, SAP, Box, Intuit, and MongoDB — have committed to A2A. These companies will deploy hundreds of internal agents and need a discovery layer that works across organizational boundaries. The enterprise demand creates a monetization path that pure-directory competitors cannot serve.

**3. The Agentic AI Foundation (146 companies) has produced no DNS-equivalent.**
The Foundation was formed to coordinate agent interoperability standards. After months of work, it has produced protocol specifications but no discovery infrastructure. This is a coordination failure that creates a startup opportunity: the Foundation defined the socket, but nobody built the switchboard.

**4. Hyperscaler bundling risk creates urgency.**
There is a 12–18 month window before:
- A hyperscaler (Google, Microsoft, AWS, Anthropic) bundles discovery into its agent platform, locking it to a single ecosystem
- A standards body ships a "good enough" default that reduces greenfield opportunity
- Fragmented directories accumulate enough inertia to be sticky

The product must move with the urgency of a category-defining infrastructure build. If AgentRank is not the default discovery layer within 18 months, someone else will be.

### 2.5 The Opportunity

**Whoever builds the discovery layer becomes the control plane of the agentic internet.**

This is not an incremental product opportunity. This is a platform infrastructure opportunity — the kind that produces network effects, data moats, and winner-take-most dynamics. The web search analogy is precise:

- Google did not just build a search box. It built the index, the crawler, the ranking system, the trust system (SafeSearch, spam detection, site verification), the provider optimization platform (Search Console), the analytics layer (Analytics, Search Console metrics), and the connection infrastructure (AMP, cached results, knowledge panels).
- AgentRank must follow the same trajectory: not just search, but the full discovery-to-outcome lifecycle.

The market size is bounded only by the number of agents. If the agent ecosystem reaches 10 million agents (a conservative estimate for 2028 given current growth rates), the discovery layer becomes as critical as DNS is to the web.

---

## 3. Product Vision and Principles

### 3.1 Vision Statement

> **Build the default discovery and trust layer for the agent internet: the place where agents go to find the best agent for a task, understand whether it is safe and compatible, and connect to it in a few milliseconds.**

This vision deliberately prioritizes agents as the primary consumer over humans. Humans will use AgentRank through the Agent Search Console and exploratory web UI. But the critical adoption path is agent-to-agent: an orchestrator at runtime calling `POST /v1/search` to find the best agent for a subtask. If AgentRank becomes the default runtime dependency for agent composition, it becomes infrastructure.

### 3.2 Product Positioning

**This product is NOT:**
- Just a registry — registries are static and stale
- Just a search box — search without trust and connection is incomplete
- Just a protocol gateway — protocol translation is a commodity
- Just a benchmark site — benchmarks without ranking integration are academic
- Just a directory — directories do not crawl, rank, verify, or connect

**This product IS:**

| Pillar | Function | Web Analogy |
|--------|----------|-------------|
| **Discovery Engine** | Finds agents automatically by crawling the web, ingesting registry feeds, monitoring DNS, and following the agent interaction graph | Googlebot |
| **Canonical Registry** | Merges fragmented records into a single deduplicated, evidence-backed source of truth per agent | Google's web index |
| **Ranking Engine** | Scores agents by relevance, trust, quality, and outcomes using a transparent, multi-signal formula | PageRank + search ranking |
| **Trust System** | Verifies identity, measures operational quality, detects abuse, and assigns trust tiers | Google Safe Browsing + site verification |
| **Connection Broker** | Mediates auth, validates compatibility, enforces policy, and captures outcome telemetry | Google AMP / cached results + click tracking |
| **Agent Search Console** | Gives providers visibility into crawl status, indexing, ranking factors, and optimization opportunities | Google Search Console |

### 3.3 Design Principles

These ten principles are synthesized from three prior architecture proposals. They are ordered by priority. When principles conflict, higher-numbered principles yield to lower-numbered ones.

#### Principle 1: Crawl, Don't Register

**Statement:** Zero manual registration is required for an agent to be discoverable.

**Justification:** The web did not scale through Yahoo submission forms. Manual registration creates coverage bias (well-resourced agents are overrepresented), staleness (submitted data decays), and operator burden (providers must learn about and submit to each registry). Autonomous crawling eliminates all three failure modes.

**Implication:** The system must support optional claiming, verification, and optimization by providers — but an agent that has never been "registered" must still appear in search results if it is crawlable, parseable, and passes validation.

#### Principle 2: Protocol-Native First

**Statement:** A2A Agent Cards are the primary discovery root. The platform must understand A2A semantics natively.

**Justification:** A2A is the emerging standard for agent interoperability. Its Agent Card convention (`/.well-known/agent.json`) provides the structured metadata that makes crawling possible: name, description, skills, auth schemes, endpoint URL. Building protocol-native means the platform understands agent-specific concepts (skills, modalities, auth profiles) rather than treating agents as generic web resources.

**Implication:** The crawler, parser, validator, registry schema, search index, and ranking system are all designed around agent-specific concepts from day one. Non-A2A agents (MCP servers, raw HTTP services) are supported through compatibility adapters, not as first-class citizens.

#### Principle 3: Evidence Over Claims

**Statement:** Observed behavior outweighs self-reported metadata in every ranking and trust decision.

**Justification:** Providers have incentives to overclaim capabilities, inflate descriptions, and misrepresent quality. Self-reported metadata is a starting signal, not a ground truth. The platform's value is precisely that it independently verifies claims through liveness probing, capability benchmarking, outcome tracking, and ecosystem reputation.

**Implication:** The ranking formula weights evidence-derived signals (health probes, benchmark results, connect success rates, outcome telemetry) more heavily than self-reported signals (agent card descriptions, claimed skills, self-assigned tags).

#### Principle 4: Search Engine, Not Static Catalog

**Statement:** The architecture must support crawling, freshness, ranking, anti-spam, and continuous evaluation — not just storage and lookup.

**Justification:** A static catalog decays. A search engine maintains freshness through re-crawling, maintains quality through ranking, and maintains trust through anti-spam. The operational complexity of a search engine is higher, but the value is categorically different.

**Implication:** Every architectural decision should be evaluated against the question: "Does this make the system more like a search engine or more like a database?" The answer should always be "search engine."

#### Principle 5: Trust-Aware by Default

**Statement:** Relevance without trust is spam. Every search result must carry trust context.

**Justification:** As the agent ecosystem grows, spam, impersonation, and capability inflation will follow. Without trust as a first-class ranking dimension, the search results will be dominated by agents that are optimized for appearing relevant without being trustworthy.

**Implication:** Trust is not a filter applied after ranking. It is a dimension of the ranking formula itself. Low-trust agents are suppressed in ranking by a trust floor function, even if they are otherwise highly relevant. Trust tier is exposed in every search result.

#### Principle 6: Agents Are First-Class Searchers

**Statement:** The product is agent-first, human-second. The primary user is a calling agent at runtime, not a developer browsing a web UI.

**Justification:** The dominant use case is runtime composition: an orchestrator needs to find the best agent for a subtask and connect to it within the latency budget of a user-facing workflow. This requires a structured API with sub-50ms p95 latency, not a web page.

**Implication:** The search API (`POST /v1/search`) is the primary product surface. The web UI is important for providers and human explorers but is not the moat. The platform itself must be an A2A-discoverable agent.

#### Principle 7: Connection Quality Matters

**Statement:** Discovery is incomplete without connection. The platform must close the loop from search to successful task completion.

**Justification:** Competitors will stop at search. This leaves three categories of value on the table: (a) connection friction (auth negotiation, policy enforcement, compatibility checking), (b) outcome intelligence (which agents actually succeed at which tasks), and (c) data moat (the outcome graph that makes ranking smarter over time).

**Implication:** The connect broker and outcome telemetry system are not optional future features. They are core platform components that ship in Phase 2.

#### Principle 8: Public and Private by Design

**Statement:** One architecture must serve internet-scale public discovery and enterprise-private agent networks.

**Justification:** If the platform only works on the public internet, it is a useful search engine but not a control plane. Enterprise buyers need tenant-private registries, scoped visibility, policy-aware search, and ranking signal isolation. Supporting this from the start avoids an expensive architectural retrofit later.

**Implication:** Every entity in the registry has a visibility scope (Public, Tenant Private, Partner Restricted, Internal). Every search query resolves a caller identity and tenant context. Ranking features are partitioned to prevent cross-tenant signal leakage.

#### Principle 9: Open Protocol, Proprietary Ranking

**Statement:** The discovery protocol, API contracts, and data formats should be open. The ranking algorithm and its training data are the proprietary moat.

**Justification:** Open protocols drive adoption. Proprietary ranking drives defensibility. This is the Google model: anyone can build a crawler, but Google's ranking is the product. AgentRank's APIs and schema should be documented and stable. AgentRank's ranking weights, signal features, and training data should be proprietary.

**Implication:** Publish API specs, schema documentation, and discovery extension proposals. Do not publish ranking weights, feature dictionaries, or evaluation datasets.

#### Principle 10: Optimize for Long-Term Moat

**Statement:** Every product surface should increase at least one of: crawl coverage, registry quality, trust graph density, outcome graph depth, or provider optimization engagement.

**Justification:** The moat is not any single feature. It is the combination of four compounding graphs:
- **Agent graph** — which agents exist, how they relate
- **Interaction graph** — which agents are called together, in what workflows
- **Outcome graph** — which agents succeed at which tasks, under what conditions
- **Trust graph** — which providers are verified, which agents are reliable

Each product surface should contribute to at least one of these graphs.

**Implication:** Features that do not contribute to any graph should be deprioritized. Features that contribute to multiple graphs simultaneously (e.g., the connect broker contributes to both interaction and outcome graphs) should be prioritized.

### 3.4 Web Search Analogy Table

The following table maps every core concept from web search to its AgentRank equivalent, including the AVERT framework (Authority, Verification, Evidence, Ranking, Trust) mapping where applicable.

| Web Search Concept | AgentRank Equivalent | AVERT Dimension | Notes |
|-------------------|---------------------|-----------------|-------|
| Web page | Agent (with versioned Agent Card) | — | The atomic unit of discovery |
| URL | Agent Card URI (`/.well-known/agent.json`) | — | The canonical address |
| Domain | Provider domain | Authority | Groups agents by operator |
| `robots.txt` | `agent-robots.txt` | — | Crawl policy for agent search |
| `sitemap.xml` | `agent-sitemap.json` | — | Provider-declared agent inventory |
| Googlebot | AgentRank Crawler | — | Autonomous discovery engine |
| Web index | Canonical Registry | — | Deduplicated, evidence-backed store |
| PageRank | AgentRank Authority Score | Authority | Graph-based authority signal |
| Search ranking | AgentRank Search Score | Ranking | Multi-signal composite score |
| Organic results | Ranked agent results | Ranking | Evidence-based ordering |
| Snippet | Result explanation payload | Ranking | Why this result was returned |
| Google Safe Browsing | Trust tier system | Trust | Safety and abuse classification |
| HTTPS indicator | Verification badges (Verified, Trusted) | Verification | Visual trust markers |
| Google Search Console | Agent Search Console | Evidence | Provider optimization platform |
| Structured data markup | Agent Card fields + extensions | Evidence | Machine-readable metadata |
| Core Web Vitals | Operational Quality Score | Evidence | Health, latency, reliability |
| Click-through rate | Connect success rate | Ranking | Post-search engagement signal |
| Search quality rater | Benchmark and evaluation suite | Evidence | Human + automated quality labels |
| Knowledge Graph | Agent relationship graph | Authority | Cross-agent connections |
| Google Analytics | Outcome telemetry | Evidence | Post-connection success data |
| AMP / Cached results | Brokered Connect | — | Optimized connection path |
| SEO | Agent Search Optimization (ASO) | — | Provider-driven discoverability improvement |
| Spam detection | Abuse resistance system | Trust | Sybil, spam, impersonation detection |
| Site verification | Domain verification (DNS TXT, `agent-proof.json`) | Verification | Provider identity binding |
| Crawl budget | Frontier priority score | — | Per-domain crawl allocation |
| Index coverage | Registry coverage metrics | — | What percentage of agents are indexed |

### 3.5 Market Thesis

The agent ecosystem will fragment along five axes:

1. **Frameworks** — LangGraph, CrewAI, AutoGen, Semantic Kernel, custom — each with its own deployment patterns and metadata conventions
2. **Hosting** — Cloud functions, Kubernetes, edge runtimes, local machines, on-chain — each with different discovery characteristics
3. **Identity** — API keys, OAuth, DIDs, wallet signatures, mTLS, custom — no convergence in sight
4. **Quality** — Production-grade enterprise agents alongside weekend experiments and abandoned prototypes — no quality differentiation exists
5. **Registries** — 15+ today, likely 50+ within a year — each with partial coverage and no interoperability

This fragmentation is the opportunity. The winner does for agents what Google did for web pages: builds a system that crawls across all these fragments, normalizes the metadata, ranks by quality, verifies trust, and presents a unified discovery interface.

**The market thesis in one sentence:** The fragmentation is the feature. The more fragmented the ecosystem becomes, the more valuable a unified discovery layer becomes.

---

## 4. Product Requirements

### 4.1 Core User Personas

#### Persona 1: Calling Agent

**Who:** An autonomous agent (or orchestrator) that needs to find another agent at runtime to complete a subtask.

**Primary job:** "Find the best agent for this task that I can trust, connect to, and rely on — in under 50 milliseconds."

**Key needs:**
- Low-latency structured search API (p95 < 50ms)
- Natural-language capability matching ("find an agent that can review Terraform plans for security")
- Structured constraint filtering (auth scheme, modality, region, compliance, SLA)
- Compatibility validation (can I actually connect to this agent given my auth context?)
- Trust signals (is this agent verified? what is its success rate?)
- Connection metadata (endpoint, auth profile, connect mode)
- Fallback suggestions (if the primary result is unhealthy, what is the next best option?)

**Success metric:** Time from search query to successful task completion.

#### Persona 2: Agent Provider

**Who:** A team or individual that operates one or more A2A-compatible agents and wants them to be discovered, used, and trusted.

**Primary job:** "Make my agents discoverable, understand how they rank, and improve their visibility and trust."

**Key needs:**
- Automatic discovery without manual registration (my agent should appear because it is crawlable, not because I submitted a form)
- Crawl and index visibility (has my agent been found? is it indexed? when was it last crawled?)
- Discoverability score with actionable diagnostics (why am I ranked low? what should I fix?)
- Verification tools (domain claiming, signed metadata, proof files)
- Benchmark results (how does my agent perform on standard tests?)
- Trust tier visibility (what tier am I in? what do I need to reach the next tier?)
- Fair ranking (I should be rewarded for quality, not punished for not paying)

**Success metric:** Improvement in discoverability score and inbound connect requests over time.

#### Persona 3: Enterprise Platform Team

**Who:** A team operating a fleet of agents across business units, with requirements for access control, compliance, and audit.

**Primary job:** "Give our teams a governed discovery experience for internal and external agents with policy enforcement and audit trails."

**Key needs:**
- Tenant-private agent indexing (our internal agents are visible only within our organization)
- Scoped visibility (different business units see different agent inventories)
- mTLS and enterprise auth integration (SSO, SAML, OIDC)
- Policy-aware search (only show agents that comply with our data residency and security policies)
- Audit trails (who searched for what, who connected to what, what outcomes resulted)
- Region-aware indexing (agents in eu-west-1 should be preferred for EU queries)
- Ranking signal isolation (our private usage data does not leak into public rankings)

**Success metric:** Reduction in time-to-discover for internal agents; audit coverage percentage.

#### Persona 4: Ecosystem Integrator

**Who:** A cloud platform, marketplace, framework provider, or workflow vendor that wants to embed agent discovery into its own product.

**Primary job:** "Give my users access to the best agent discovery without building it myself."

**Key needs:**
- Embeddable search API with white-label support
- Partner federation feeds (ingest agents from my ecosystem, contribute my agents to the public index)
- Ranking and trust data via API (show trust badges and ranking context in my own UI)
- Analytics and reporting (how are agents in my ecosystem performing in discovery?)
- Co-branding and attribution (my agents carry my ecosystem badge in results)

**Success metric:** API call volume and partner-originated connect success rate.

### 4.2 P0 Requirements (Must Ship)

These requirements define the minimum viable product. The product does not ship without every P0 delivered.

| ID | Requirement | Acceptance Criteria |
|----|------------|-------------------|
| **P0-01** | **Autonomous web crawling of A2A agents.** The system discovers agents by crawling `/.well-known/agent.json` endpoints without manual registration. | Crawler discovers 90%+ of known public A2A agents within 72 hours of seed list initialization. |
| **P0-02** | **Registry feed ingestion.** The system ingests agents from at least 5 third-party registries (AgentVerse, PulseMCP, Moltbook, HuggingFace, CrewAI Hub) as seed sources. | Feed ingestion pipeline processes all 5 sources. Agents enter crawl validation — no direct listing. |
| **P0-03** | **Agent Card parsing and validation.** The system parses A2A Agent Cards, validates schema conformance, and normalizes metadata. | Parser handles 99%+ of valid Agent Card formats. Invalid cards are stored as evidence but not indexed. |
| **P0-04** | **Canonical registry with entity resolution.** The system deduplicates agents discovered from multiple sources into a single canonical identity per agent. | Duplicate detection catches 95%+ of same-agent records from different sources. |
| **P0-05** | **Hybrid search: lexical + semantic.** The search API supports both keyword and natural-language queries, returning ranked results. | Offline evaluation: NDCG@10 ≥ 0.7 on a golden query set of 200+ judged queries. |
| **P0-06** | **AgentRank v1 transparent scoring.** Results are ranked by a published, transparent weighted formula across at least 6 dimensions. | Every result includes an explanation payload showing top ranking factors. |
| **P0-07** | **Multi-level liveness probing.** The system continuously probes agent endpoints for health (L1–L3 minimum). | Health status updates within 15 minutes of a state change. Dead agents are demoted within 1 hour. |
| **P0-08** | **Trust tier assignment.** Every indexed agent receives a trust tier (Indexed → Established → Verified) based on evidence. | Trust tier criteria are documented and deterministic. |
| **P0-09** | **Search API for agents.** `POST /v1/search` returns ranked, explained, trust-annotated results in under 50ms p95. | API is publicly accessible with API key auth. Response schema is documented and stable. |
| **P0-10** | **Agent Search Console v1.** Providers can claim a domain, see crawl/index status, and view their AgentRank scores. | Console is functional for domain verification and basic crawl/index diagnostics. |
| **P0-11** | **Basic abuse resistance.** The system detects and suppresses obvious spam: duplicate content, domain squatting, keyword stuffing, and endpoint impersonation. | False positive rate < 1% on legitimate agents. Spam suppression catches 90%+ of synthetic test spam. |
| **P0-12** | **Public web UI.** A searchable web interface for human exploration of the agent index. | UI supports search, result browsing, agent detail pages, and trust badge display. |

### 4.3 P1 Requirements (Fast Follow)

These ship within 4–6 weeks of MVP launch.

| ID | Requirement | Acceptance Criteria |
|----|------------|-------------------|
| **P1-01** | **Direct Connect flow.** Search results include connection metadata (endpoint, auth profile, transport). Callers can connect directly using the returned metadata. | Connect metadata is present in 95%+ of healthy search results. |
| **P1-02** | **Brokered Connect flow.** The platform mediates connection setup: auth negotiation, policy validation, compatibility checking. | Brokered connect succeeds for 90%+ of compatible agent pairs. |
| **P1-03** | **Outcome telemetry API.** Callers can report connection outcomes (success, failure, timeout, auth error). | Outcome events are accepted, classified by trust level, and stored. |
| **P1-04** | **Domain verification.** Providers can verify domain ownership via DNS TXT record or `agent-proof.json` file. | Verification completes within 5 minutes of proof publication. |
| **P1-05** | **Signed metadata support.** The system recognizes and rewards cryptographically signed Agent Cards. | Signed cards receive a trust score boost. Signature verification is documented. |
| **P1-06** | **Graph retrieval.** Search includes provider relationship and invocation co-occurrence signals. | Graph signals improve NDCG@10 by measurable margin over lexical+semantic baseline. |
| **P1-07** | **Benchmark framework v1.** The system runs non-destructive capability tests against agents and publishes results. | Benchmarks cover protocol conformance and basic capability verification. |
| **P1-08** | **A2A-native self-discovery.** AgentRank itself is discoverable as an A2A agent at `/.well-known/agent.json`. | External agents can discover and call AgentRank's search API using standard A2A flows. |

### 4.4 P2 Requirements (Strategic)

These ship within 3–6 months and define the strategic moat.

| ID | Requirement | Acceptance Criteria |
|----|------------|-------------------|
| **P2-01** | **Enterprise tenant-private registries.** Enterprises can index internal agents visible only within their tenant. | Tenant isolation is cryptographically enforced. Cross-tenant signal leakage is zero. |
| **P2-02** | **Federation connectors.** The platform supports pull and push federation with partner registries. | At least 2 federation partners are live and contributing agents. |
| **P2-03** | **Policy-aware search.** Search results respect enterprise policy packs (data residency, security level, compliance requirements). | Policy filters are applied at query time. Non-compliant agents are excluded, not just demoted. |
| **P2-04** | **Learning-to-rank.** The ranking system transitions from transparent heuristics to a trained model using outcome data and judged query sets. | Offline evaluation shows statistically significant NDCG improvement over heuristic baseline. |
| **P2-05** | **Outcome-driven ranking.** Outcome telemetry (connect success, task completion, repeat usage) feeds back into ranking scores. | Agents with high outcome success rates rank measurably higher for relevant queries. |
| **P2-06** | **Agent Search Console v2.** Console includes competitive benchmarking, query analytics (which searches lead to my agent), and optimization recommendations. | Providers report actionable insights from console data. |

### 4.5 Non-Functional Requirements

| Metric | Target | Measurement |
|--------|--------|------------|
| **Search latency p50** | < 20ms | End-to-end from API gateway to response, measured at application boundary |
| **Search latency p95** | < 50ms | Same measurement point |
| **Search latency p99** | < 100ms | Same measurement point |
| **Crawl-to-index latency** | < 6 hours | Time from first successful fetch of a new agent card to appearance in search results |
| **High-priority crawl-to-index** | < 15 minutes | For agents from known high-confidence seed sources |
| **Search QPS (sustained)** | 10,000 | Under normal operating conditions |
| **Search QPS (burst)** | 50,000 | Peak burst capacity with graceful degradation |
| **Uptime** | 99.95% | Monthly, measured per-API-surface |
| **Registry capacity** | 10M+ agents | Long-term design target for canonical agent records |
| **Evidence objects** | 100M+ | Versions, snapshots, proofs, benchmarks, outcomes |
| **Crawler throughput** | 50K+ fetches/sec | Design envelope for peak crawl capacity |
| **Health probe freshness** | < 15 minutes | Maximum age of health status before re-probe |
| **Zero-result rate** | < 5% | Percentage of non-empty queries returning no results |
| **Duplicate suppression** | > 95% | Percentage of duplicate agent records correctly merged |
| **Trust tier accuracy** | > 99% | Percentage of agents in the correct trust tier (measured by manual audit) |
| **Data durability** | 99.999999999% (11 nines) | For evidence archive in object storage |

---

## 5. Scope and Non-Goals

### In Scope

The following capabilities are within the scope of AgentRank's architecture and roadmap:

| Capability | Phase | Justification |
|------------|-------|---------------|
| Autonomous web crawling of A2A agents | Phase 0 | Core discovery mechanism |
| Third-party registry feed ingestion | Phase 0 | Bootstraps coverage before crawling reaches critical mass |
| Agent Card parsing, validation, and normalization | Phase 0 | Foundation for registry quality |
| Canonical registry with entity resolution | Phase 0 | Backbone of the product |
| Hybrid search (lexical + semantic) | Phase 0 | Core search quality |
| AgentRank transparent scoring (v1 heuristic) | Phase 0 | Core ranking |
| Multi-level liveness probing (L1–L4) | Phase 0 | Differentiator over static registries |
| Trust tier assignment and badge display | Phase 0 | Trust-aware ranking |
| Search API (`POST /v1/search`) | Phase 0 | Primary product surface |
| Agent Search Console v1 | Phase 0 | Provider flywheel |
| Basic abuse resistance (duplicate, spam, impersonation) | Phase 0 | Quality protection |
| Public web UI for human exploration | Phase 0 | Market presence |
| Direct Connect and Brokered Connect flows | Phase 1 | Closes the outcome loop |
| Outcome telemetry API | Phase 1 | Feeds ranking and trust |
| Domain verification (DNS TXT, `agent-proof.json`) | Phase 1 | Trust infrastructure |
| Signed metadata recognition | Phase 1 | Trust infrastructure |
| Graph retrieval signals | Phase 1 | Search quality improvement |
| Benchmark framework | Phase 1 | Evidence-based quality |
| A2A-native self-discovery | Phase 1 | Protocol legitimacy |
| Enterprise tenant-private registries | Phase 2 | Enterprise revenue |
| Federation connectors (pull + push) | Phase 2 | Ecosystem integration |
| Policy-aware search | Phase 2 | Enterprise requirement |
| Learning-to-rank transition | Phase 2 | Ranking moat |
| Outcome-driven ranking | Phase 2 | Data moat |
| Agent Search Console v2 | Phase 2 | Provider moat |
| DNS discovery hints (`_a2a._tcp`) | Phase 1 | Seed source expansion |
| `agent-sitemap.json` convention | Phase 1 | Provider coverage |
| `agent-robots.txt` convention | Phase 1 | Crawl policy |
| MCP server discovery (via `_mcp._tcp` DNS TXT) | Phase 1 | Cross-protocol coverage |
| Open web discovery (GitHub repos, docs, package metadata) | Phase 1 | Coverage expansion |
| Infrastructure discovery (CT logs, passive DNS) | Phase 2 | Advanced coverage |

### Out of Scope

The following capabilities are explicitly **not** part of AgentRank. Declaring non-goals prevents scope creep and keeps the team focused on the discovery, ranking, and trust problem.

| Non-Goal | Why It's Out | Risk If Included |
|----------|-------------|-----------------|
| **Agent execution or hosting** | AgentRank discovers and connects to agents. It does not run them. The platform is a search engine, not a cloud provider. | Massive operational complexity, competitive conflict with every cloud provider. |
| **Agent development framework** | AgentRank does not provide SDKs for building agents. It discovers agents built with any framework. | Framework lock-in, competitive conflict with LangGraph/CrewAI/AutoGen. |
| **Full traffic proxy** | The connect broker mediates setup, not ongoing traffic. Direct connect is the default. Brokered connect handles setup and steps out of path. | Latency penalty, scale bottleneck, provider resistance to mandatory proxying. |
| **Payment processing** | AgentRank does not process payments between agents. Economic efficiency is a ranking signal, not a transaction rail. | Regulatory burden, competitive conflict with x402/Stripe/payment providers. |
| **Agent training or fine-tuning** | AgentRank benchmarks agents as they are. It does not improve them. | Scope explosion, competitive conflict with model providers. |
| **General-purpose web search** | AgentRank searches for agents, not web pages. The crawler fetches agent metadata, not arbitrary web content. | Existential scope creep. |
| **Replacing A2A or MCP protocols** | AgentRank works with existing protocols. It proposes extensions (agent-sitemap, agent-robots, agent-proof) but does not define a new agent communication protocol. | Standards-body conflict, adoption friction. |
| **Real-time agent orchestration** | AgentRank helps callers find agents. The caller orchestrates the workflow. The connect broker mediates setup, not ongoing coordination. | Competitive conflict with orchestration platforms. |
| **Data storage or processing for agents** | AgentRank does not store or process data on behalf of discovered agents. | Liability, security surface expansion. |
| **Identity provider** | AgentRank verifies identities. It does not issue long-lived identities. Domain verification proves existing identity, not creates new identity. | Competitive conflict with DID providers, complexity. |
| **Marketplace transaction mediation** | AgentRank is a search engine with connection brokering, not a marketplace with transaction guarantees, escrow, or dispute resolution. | Regulatory burden, operational complexity, wrong business model. |
| **Agent-to-agent communication protocol** | AgentRank does not define how agents talk to each other. A2A defines that. AgentRank discovers, ranks, and connects — it does not relay messages. | Protocol-level conflict with A2A. |

### Boundary Principle

When a proposed feature falls on the boundary between in-scope and out-of-scope, apply this test:

> **Does this feature make AgentRank a better search engine, a better trust system, or a better connection broker?** If yes, it's in scope. **Does this feature make AgentRank an agent runtime, a marketplace, or a protocol?** If yes, it's out of scope.

---

## Milestones: Weekly build plan

This section is the **operating cadence** for the team: one primary focus per week, concrete builds, and a clear “done when.” It aligns with Phase 0–3 elsewhere in this document (foundation → public launch scale → intelligence and revenue → dominance). Treat each **Done when** as the gate before pulling the next week’s work into the main line.

**How to use it:** At the start of the week, copy the row into your issue tracker; mid-week, only protect scope if something is on fire. If you slip, finish the **Done when** before adopting the next week’s theme—otherwise the crawl/index/search loop breaks.

### Weeks 1–4 — Phase 0: Foundation (vertical slice)

| Week | Theme | Build this week | Done when |
|------|--------|-----------------|-----------|
| **1** | Repo + data plane | Monorepo/workspace, CI (build, test, lint, fmt), Docker Compose dev env; PostgreSQL schema v1 (`agents`, `providers`, `crawl_history`, `trust_records` minimal); Redis for frontier + cache | `cargo build`/equivalent green on CI; `sqlx migrate` (or chosen tool) applies cleanly; Redis cluster/single mode healthy |
| **2** | Ingest path | AgentBot v0.1: fetch one URL → parse Agent Card → persist; Card parser v0.1 (schema validation, required fields, normalization); URL frontier v0.1 (Redis sorted set, dedup, basic priority) | Integration test: mock `agent.json` URL → row in DB; 50+ parser unit tests; enqueue/dequeue 10K URLs in priority order with no dupes |
| **3** | Search + API + UI | Tantivy v0.1 index (name, description, skills); API gateway `/search` v0.1 (anon tier, rate limits); Web UI v0.1 (search box, results list, agent detail) | 1K test agents indexed; keyword search returns sensible order; P99 under 200ms in dev; UI shows results on desktop + mobile |
| **4** | Crawl scale + console + observability | AgentBot v0.2: frontier-driven crawl, rate limits, `robots.txt`; Agent Search Console v0.1 (domain verification stub or manual claim, card inspector, crawl history view); Prometheus + Grafana + alerts | **Phase 0 integration test:** seed URLs → crawl → parse → index → `POST /search` → results in UI; 100 real card URLs crawled without policy violations |

### Weeks 5–12 — Phase 1: Autonomous discovery + hybrid search + trust + connect

| Week | Theme | Build this week | Done when |
|------|--------|-----------------|-----------|
| **5** | Seed explosion (A) | Registry feed ingestion hardening for ≥3 sources; GitHub discovery MVP (search/API for `agent.json` links or well-known patterns); expand seed lists into frontier | New sources enqueue thousands of unique card URLs; documented runbook for each feed |
| **6** | Seed explosion (B) + vectors | DNS TXT / hint scanning MVP; Qdrant online; embedding pipeline (batch + incremental path); **hybrid** retrieval (BM25 + vector) behind same `/search` | Hybrid beats BM25-only on a small labeled dev set; vectors backfill for all indexed agents |
| **7** | Ranking v1 | AgentRank **AVERT** v1 in scoring path; composite score persisted; explanation payload on each result (top factors) | Recompute stable across runs; every search result carries score + explain snippet |
| **8** | Liveness | Scheduler + L1–L3 probes; status state machine; demotion rules for dead endpoints | Stale status under 15 min after state change; dead agents demoted in ranking within SLA you documented |
| **9** | Connect loop (A) | **Direct connect** metadata in API responses; **Brokered connect** v0.1 (session create, handoff, policy placeholders) | E2E: search → pick agent → connect flow succeeds in staging with test agents |
| **10** | Trust + spam | Domain verification (≥2 methods, e.g. DNS TXT + hosted file); TLS metadata capture; spam v1 (dup content, obvious stuffing) | ≥100 agents verified in staging; spam fixtures demoted or quarantined; FP rate within agreed threshold |
| **11** | Query understanding | Intent classifier + basic query expansion; optional filters wired (e.g. modality, auth family) | Over 85% intent accuracy on your frozen dev set; recall lift on expansion set |
| **12** | Console + launch hardening | Agent Search Console v1.1 (trust dashboard, query impressions, liveness history); load test; SLO dashboards; **Phase 1 launch** checklist | **100K+** agents indexed; **NDCG@10 above 0.70** on golden set; **availability above 99.5%** for search week; launch comms ready |

### Weeks 13–24 — Phase 2: LTR, outcomes, enterprise, federation, monetization

| Week | Theme | Build this week | Done when |
|------|--------|-----------------|-----------|
| **13** | LTR foundation | Feature logging from search + outcomes; training dataset pipeline; baseline reproducible offline eval | Train/score script runs on CI; NDCG report artifact per build |
| **14** | LTR model | XGBoost (or chosen) trainer; ONNX export; shadow scoring in search path | Offline NDCG gain vs AVERT-only on held-out set (pre-agreed bar) |
| **15** | LTR production | Gradual rollout + guardrails; fallback to AVERT if model unhealthy | Production traffic slice using LTR with automatic rollback tested |
| **16** | Outcome ranking | Outcome signals as features; documentation for callers on reporting outcomes | Outcome-augmented ranker beats prior slice in offline replay |
| **17** | Anti-abuse (A) | Link farm / mutual-citation detection v1; graph features in ranking or demotion | Simulated farm detected in staging |
| **18** | Anti-abuse (B) + benchmarks | CTR / manipulation heuristics; behavioral anomalies; **Benchmark framework v1** (conformance + reliability) | 50+ conformance tests green against fixtures; reliability job runs on schedule |
| **19** | Enterprise (A) | Tenant model; schema or DB isolation strategy; RBAC for APIs | Two mock tenants cannot read each other’s agents |
| **20** | Enterprise (B) | Private registry ingest path; mTLS or enterprise auth for tenant APIs | Pilot tenant can index internal-only agents end-to-end |
| **21** | Federation | Pull federation from ≥1 partner; dedup vs crawl; connector health metrics | ≥50K agents via federation in staging without dup explosion |
| **22** | Monetization (A) | Promoted slots design; auction/billing hooks (can be manual ops first); click tracking | End-to-end demo: bid → promoted impression → click logged |
| **23** | Monetization (B) | Billing integration or invoice export; fraud checks on clicks | First synthetic billing run reconciles with click log |
| **24** | Analytics + Phase 2 gate | Agent Analytics / Console premium views; review SLOs; cost dashboards | **Phase 2 launch:** **1M+** agents, **NDCG@10 above 0.75**, enterprise pilot live, **MRR above $100K** (or documented exception with new target date) |

### Weeks 25–52 — Phase 3: Scale, federation depth, advanced search, moat

| Week | Theme | Build this week | Done when |
|------|--------|-----------------|-----------|
| **25** | Push federation design | Partner webhook/event design; auth; idempotency; replay | Contract doc + stub receiver in staging |
| **26** | Push federation MVP | First partner on push updates; hybrid pull/push reconciliation | Real-time updates visible in index within SLA |
| **27** | Partner portal v0 | Self-serve keys, feed health, dispute contact | One partner can onboard without engineering |
| **28** | Multi-region (A) | Second region deploy; traffic routing; failover drill | DR runbook executed once |
| **29** | Data residency | Policy tags on agents; query-time enforcement | EU-only query returns only EU-resident agents in test |
| **30** | Federation scale | 5+ partners integrated (push and/or pull) | Partner dashboard shows green sync |
| **31** | Compositional queries | Multi-skill / multi-step query parsing MVP | Golden queries for “A then B” return plausible agent sets |
| **32** | Workflow search | Package adjacent agents; graph expansion in retrieval | Measurable recall win on workflow benchmark |
| **33** | Semantic upgrades | Better embeddings or reranker experiment slot | A/B shows non-regression + win on slice |
| **34** | Security benchmarks | Opt-in security scan job; results in trust payload | Report shows in Console for test agent |
| **35** | Trust + security UX | Surface security tier without alarm fatigue | User testing sign-off |
| **36** | Ranking polish | Merge learnings from 31–35 into prod ranker | SLOs still met |
| **37** | Recommendations v0 | Caller history feature; “similar agents” | Click-through on recs measured |
| **38** | Recommendations v1 | Contextual suggestions at search time | Offline policy review passed |
| **39** | Data licensing v0 | Anonymized aggregate API / export | Internal customer zero review complete |
| **40** | Data licensing pilot | First external consumer of trend data | Contract + data checklist signed |
| **41** | On-chain attestations PoC | Publish trust summary to chain (e.g. ERC-8004 experiment) | Demo tx + docs |
| **42** | Attestation UX | Explain to providers what is published | Support playbook |
| **43** | MCP discovery (A) | `_mcp._tcp` or equivalent discovery → normalized records | MCP servers appear in index smoke test |
| **44** | MCP discovery (B) | Tool-level metadata indexing | Search by tool name works on fixtures |
| **45** | Cross-protocol ranking | Fair blending vs A2A-only baseline | No collapse in A2A relevance metrics |
| **46** | Cost optimization | Crawl + search unit economics review | % cost reduction or capacity gain documented |
| **47** | Reliability push | Game days; chaos tests on Kafka/DB | Postmortem template used once |
| **48** | Developer flywheel | Docs, SDK stubs, public examples refreshed | N new integrations from partners (target you set) |
| **49** | Ecosystem growth | Conference / blog / sample agents | Inbound partner pipeline metric |
| **50** | Performance | **10K+ QPS** peak drill (or path to it) | Bottlenecks documented + fixes merged |
| **51** | Quality week | Golden set expansion; spam review; manual eval | NDCG and zero-result rate within targets |
| **52** | Year review | Roadmap v2; **5M+** agents path; moat metrics dashboard | Exec readout + next-year Phase 4 themes |

### Ongoing every week (non-negotiable)

| Activity | Minimum bar |
|----------|-------------|
| **Security** | Dependency patch triage; no known criticals unpatched in prod |
| **Data** | Backup/restore spot check or automated proof |
| **Quality** | Golden-query regression run on candidate builds |
| **Incidents** | Postmortem for any user-visible SLO miss |

---

<!-- Parts 2-5 follow below -->
## 8. Protocol Foundation: A2A as the Discovery Root

### Overview

The Google Agent-to-Agent (A2A) protocol introduces the **Agent Card** as the canonical first-party metadata object — the machine-readable self-description that every compliant agent publishes at its well-known endpoint. The Agent Card is the **origin document**: the authoritative declaration by an agent of its own identity, capabilities, interaction modalities, and access requirements. It is the seed from which all downstream discovery, indexing, ranking, and orchestration derive.

But an Agent Card is an **origin document**, not the **full truth**. Just as an HTML page's `<meta>` tags declare what the page claims to be about, the Agent Card declares what the agent claims it can do. The actual truth — whether the agent delivers on those claims, whether it's trustworthy, whether it performs well, whether it's compatible with a given consumer — requires an entire evidence-collection and scoring apparatus built on top of the origin document.

This section establishes the Agent Card as the protocol foundation, examines what it provides and where it falls short, and proposes a suite of discovery extensions that bridge the gap between self-declaration and verifiable truth.

### Agent Card: The Canonical First-Party Metadata Object

Every A2A-compliant agent MUST publish an Agent Card at `/.well-known/agent.json`. This JSON document serves as the agent's primary self-description and is the first thing any discovery system, orchestrator, or peer agent retrieves when encountering an agent endpoint.

**What Agent Cards provide:**

| Field Category | Fields | Purpose |
|---|---|---|
| **Identity** | `name`, `description`, `version`, `provider` | Human-readable identification and provenance |
| **Endpoint** | `url` | The canonical URL where the agent accepts A2A requests |
| **Authentication** | `authentication.schemes[]` | Supported auth mechanisms (OAuth2, API key, bearer, mTLS) |
| **Capabilities** | `capabilities.streaming`, `capabilities.pushNotifications`, `capabilities.stateTransitionHistory` | Protocol-level feature support flags |
| **Skills** | `skills[].id`, `skills[].name`, `skills[].description`, `skills[].tags`, `skills[].examples` | Declared functional abilities with natural-language descriptions |
| **Interaction Features** | `defaultInputModes[]`, `defaultOutputModes[]` | Supported MIME types for input and output (text, image, audio, video) |

**Example Agent Card (abbreviated):**

```json
{
  "name": "FinancialAnalysisAgent",
  "description": "Performs portfolio analysis, risk assessment, and market research using real-time data feeds.",
  "version": "2.4.1",
  "url": "https://agents.acmecorp.com/financial-analysis",
  "provider": {
    "organization": "AcmeCorp Financial Services",
    "url": "https://acmecorp.com",
    "contactEmail": "agents@acmecorp.com"
  },
  "authentication": {
    "schemes": [
      {
        "scheme": "OAuth2",
        "authorizationUrl": "https://auth.acmecorp.com/oauth2/authorize",
        "tokenUrl": "https://auth.acmecorp.com/oauth2/token",
        "scopes": ["financial:read", "portfolio:analyze"]
      },
      {
        "scheme": "Bearer",
        "headerName": "Authorization"
      }
    ]
  },
  "capabilities": {
    "streaming": true,
    "pushNotifications": true,
    "stateTransitionHistory": true
  },
  "skills": [
    {
      "id": "portfolio-analysis",
      "name": "Portfolio Risk Analysis",
      "description": "Analyzes investment portfolio composition, calculates VaR, Sharpe ratio, and provides rebalancing recommendations.",
      "tags": ["finance", "portfolio", "risk", "analysis"],
      "examples": [
        "Analyze my portfolio for concentration risk",
        "Calculate the Sharpe ratio for these holdings",
        "Recommend rebalancing given a 60/40 target allocation"
      ]
    },
    {
      "id": "market-research",
      "name": "Market Research",
      "description": "Provides real-time market data, sector analysis, and earnings summaries.",
      "tags": ["finance", "market", "research", "equities"],
      "examples": [
        "What are the top movers in the S&P 500 today?",
        "Summarize AAPL's last earnings call",
        "Compare tech sector performance YTD vs historical"
      ]
    }
  ],
  "defaultInputModes": ["text/plain", "application/json"],
  "defaultOutputModes": ["text/plain", "application/json", "text/markdown"]
}
```

### Why Agent Cards Are Necessary but Insufficient

The Agent Card is the **indispensable starting point** — without it, there is no standardized way to discover what an agent is or does. But the Agent Card alone cannot answer the questions that matter most for high-quality discovery:

| Question | Why the Agent Card Can't Answer It |
|---|---|
| **Does the agent actually work?** | Cards are self-declared; no liveness or correctness verification |
| **How well does it perform?** | No latency, throughput, or success-rate metrics in the spec |
| **Is it trustworthy?** | No cryptographic proof of identity, no attestation chain |
| **Is it compatible with my agent?** | Auth scheme overlap and I/O mode intersection require cross-referencing |
| **How does it compare to alternatives?** | No ranking signals; all cards are equally "valid" |
| **Is it still alive?** | Cards are static snapshots; no freshness guarantee |
| **What do other agents think of it?** | No reputation, review, or endorsement mechanism |
| **Does it handle edge cases?** | Example queries don't capture failure modes or limitations |

To bridge this gap, a discovery system must layer **six additional capabilities** on top of the Agent Card:

```
┌─────────────────────────────────────────────────────────────┐
│                    DISCOVERY SYSTEM                          │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  6. Outcome-Based Ranking                           │    │
│  │     Live probes, A/B task routing, success tracking  │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  5. Compatibility Assessment                        │    │
│  │     Auth intersection, I/O mode matching, protocol  │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  4. Quality Scoring                                 │    │
│  │     Latency, reliability, skill depth, freshness    │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  3. Trust Signal Collection                         │    │
│  │     DNS verification, TLS chain, domain reputation  │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  2. Canonicalization & Deduplication                 │    │
│  │     Entity resolution across registries, versions   │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  1. Evidence Collection                             │    │
│  │     Crawl history, uptime logs, change tracking     │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  0. Agent Card (Origin Document)                    │    │
│  │     Self-declared identity, skills, capabilities    │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### The Origin Document Analogy

The relationship between Agent Cards and the discovery system mirrors the relationship between HTML pages and web search engines:

| Web Search Analogy | Agent Discovery Equivalent | Role |
|---|---|---|
| HTML Page | Agent Card | Origin document: self-declared content |
| `<meta>` tags, `<title>` | `name`, `description`, `skills[]` | Structured self-description metadata |
| `robots.txt` | `agent-robots.txt` (proposed) | Crawl directives and access policies |
| `sitemap.xml` | `agent-sitemap.xml` (proposed) | Discovery manifest for multi-agent providers |
| Googlebot | AgentBot | Automated crawler that fetches and indexes origin docs |
| PageRank | AgentRank (§12) | Authority scoring based on graph structure and signals |
| Google Search Index | Aggregated Registry (§10) | Canonical, deduplicated, enriched record store |
| Google Search Results | Discovery API (§11) | Ranked, filtered results served to consumers |
| Chrome UX Report (CrUX) | Evidence Store | Real-world performance and reliability data |
| Google Safe Browsing | Trust & Safety Layer (§14) | Abuse detection and harmful-agent filtering |
| Schema.org markup | Capability Taxonomy (proposed) | Standardized vocabulary for agent skills |
| HTTPS / EV Certificates | `agent-proof.json` (proposed) | Cryptographic identity verification |
| DNS / WHOIS | DNS-based Discovery Hints (proposed) | Infrastructure-level discovery signals |
| Lighthouse Scores | Quality Score (§12) | Multi-signal quality assessment |

This analogy is not superficial — it reflects a deep structural isomorphism. Every challenge that web search solved over 25 years (spam, stale content, duplicate pages, trust, relevance) will manifest in agent discovery. The difference is that we can design for these challenges from day one rather than retrofitting solutions.

---

### 8.1 Proposed Discovery Extensions

The base A2A specification provides the Agent Card and the well-known endpoint. To enable robust, scalable discovery, we propose five protocol extensions that agents and providers MAY implement. Each extension is designed to be incrementally adoptable — agents that don't implement them still work; agents that do provide richer discovery signals.

#### 8.1.1 `agent-sitemap.xml` — Multi-Agent Discovery Manifest

**Purpose:** Allows providers hosting multiple agents to declare a complete manifest in a single, crawlable document. Analogous to `sitemap.xml` for web pages.

**Location:** `https://{provider-domain}/.well-known/agent-sitemap.xml`

**Specification:**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<agentsitemap xmlns="https://schemas.agentfi.dev/sitemap/1.0"
              xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
              xsi:schemaLocation="https://schemas.agentfi.dev/sitemap/1.0
                                  https://schemas.agentfi.dev/sitemap/1.0/agent-sitemap.xsd">

  <meta>
    <provider>
      <name>AcmeCorp AI Services</name>
      <url>https://acmecorp.com</url>
      <contact>agents@acmecorp.com</contact>
    </provider>
    <generated>2026-03-23T14:30:00Z</generated>
    <version>1.0</version>
    <total_agents>7</total_agents>
  </meta>

  <agent>
    <name>FinancialAnalysisAgent</name>
    <card_url>https://agents.acmecorp.com/financial-analysis/.well-known/agent.json</card_url>
    <endpoint>https://agents.acmecorp.com/financial-analysis</endpoint>
    <lastmod>2026-03-20T09:15:00Z</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
    <status>active</status>
    <version>2.4.1</version>
    <skills>
      <skill id="portfolio-analysis">
        <name>Portfolio Risk Analysis</name>
        <tags>finance,portfolio,risk,analysis</tags>
      </skill>
      <skill id="market-research">
        <name>Market Research</name>
        <tags>finance,market,research,equities</tags>
      </skill>
    </skills>
    <capabilities>
      <streaming>true</streaming>
      <push_notifications>true</push_notifications>
    </capabilities>
    <authentication>
      <scheme>OAuth2</scheme>
      <scheme>Bearer</scheme>
    </authentication>
  </agent>

  <agent>
    <name>DocumentSummarizerAgent</name>
    <card_url>https://agents.acmecorp.com/doc-summarizer/.well-known/agent.json</card_url>
    <endpoint>https://agents.acmecorp.com/doc-summarizer</endpoint>
    <lastmod>2026-03-18T11:00:00Z</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
    <status>active</status>
    <version>1.2.0</version>
    <skills>
      <skill id="summarize-pdf">
        <name>PDF Summarization</name>
        <tags>document,summarization,pdf,nlp</tags>
      </skill>
      <skill id="summarize-legal">
        <name>Legal Document Analysis</name>
        <tags>legal,document,analysis,compliance</tags>
      </skill>
      <skill id="extract-entities">
        <name>Named Entity Extraction</name>
        <tags>ner,entities,extraction,nlp</tags>
      </skill>
    </skills>
    <capabilities>
      <streaming>true</streaming>
      <push_notifications>false</push_notifications>
    </capabilities>
    <authentication>
      <scheme>APIKey</scheme>
    </authentication>
  </agent>

  <agent>
    <name>CodeReviewAgent</name>
    <card_url>https://agents.acmecorp.com/code-review/.well-known/agent.json</card_url>
    <endpoint>https://agents.acmecorp.com/code-review</endpoint>
    <lastmod>2026-03-22T16:45:00Z</lastmod>
    <changefreq>daily</changefreq>
    <priority>0.9</priority>
    <status>active</status>
    <version>3.1.0</version>
    <skills>
      <skill id="security-review">
        <name>Security Vulnerability Scan</name>
        <tags>security,code-review,vulnerability,sast</tags>
      </skill>
      <skill id="perf-review">
        <name>Performance Analysis</name>
        <tags>performance,optimization,profiling</tags>
      </skill>
    </skills>
    <capabilities>
      <streaming>true</streaming>
      <push_notifications>true</push_notifications>
    </capabilities>
    <authentication>
      <scheme>OAuth2</scheme>
    </authentication>
  </agent>

  <agent>
    <name>TranslationAgent</name>
    <card_url>https://agents.acmecorp.com/translate/.well-known/agent.json</card_url>
    <endpoint>https://agents.acmecorp.com/translate</endpoint>
    <lastmod>2026-02-15T08:00:00Z</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.6</priority>
    <status>active</status>
    <version>1.0.3</version>
    <skills>
      <skill id="translate-text">
        <name>Text Translation</name>
        <tags>translation,i18n,multilingual,nlp</tags>
      </skill>
    </skills>
    <capabilities>
      <streaming>true</streaming>
      <push_notifications>false</push_notifications>
    </capabilities>
    <authentication>
      <scheme>Bearer</scheme>
    </authentication>
  </agent>

  <agent>
    <name>DataPipelineAgent</name>
    <card_url>https://agents.acmecorp.com/data-pipeline/.well-known/agent.json</card_url>
    <endpoint>https://agents.acmecorp.com/data-pipeline</endpoint>
    <lastmod>2026-03-21T13:20:00Z</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.7</priority>
    <status>beta</status>
    <version>0.9.2</version>
    <skills>
      <skill id="etl-orchestration">
        <name>ETL Pipeline Orchestration</name>
        <tags>data,etl,pipeline,orchestration</tags>
      </skill>
      <skill id="data-quality">
        <name>Data Quality Assessment</name>
        <tags>data,quality,validation,profiling</tags>
      </skill>
    </skills>
    <capabilities>
      <streaming>false</streaming>
      <push_notifications>true</push_notifications>
    </capabilities>
    <authentication>
      <scheme>mTLS</scheme>
      <scheme>OAuth2</scheme>
    </authentication>
  </agent>

  <agent>
    <name>DeprecatedLegacyBot</name>
    <card_url>https://agents.acmecorp.com/legacy-bot/.well-known/agent.json</card_url>
    <endpoint>https://agents.acmecorp.com/legacy-bot</endpoint>
    <lastmod>2025-06-01T00:00:00Z</lastmod>
    <changefreq>never</changefreq>
    <priority>0.1</priority>
    <status>deprecated</status>
    <version>0.1.0</version>
    <deprecation>
      <reason>Replaced by FinancialAnalysisAgent</reason>
      <successor_card>https://agents.acmecorp.com/financial-analysis/.well-known/agent.json</successor_card>
      <sunset_date>2026-06-01</sunset_date>
    </deprecation>
    <skills>
      <skill id="basic-calc">
        <name>Basic Calculations</name>
        <tags>finance,calculator</tags>
      </skill>
    </skills>
    <capabilities>
      <streaming>false</streaming>
      <push_notifications>false</push_notifications>
    </capabilities>
    <authentication>
      <scheme>APIKey</scheme>
    </authentication>
  </agent>

  <agent>
    <name>InternalOnlyAgent</name>
    <card_url>https://agents.acmecorp.com/internal/.well-known/agent.json</card_url>
    <endpoint>https://agents.acmecorp.com/internal</endpoint>
    <lastmod>2026-03-23T10:00:00Z</lastmod>
    <changefreq>daily</changefreq>
    <priority>0.0</priority>
    <status>active</status>
    <visibility>private</visibility>
    <version>4.0.0</version>
    <skills>
      <skill id="internal-ops">
        <name>Internal Operations</name>
        <tags>internal,operations</tags>
      </skill>
    </skills>
    <capabilities>
      <streaming>true</streaming>
      <push_notifications>true</push_notifications>
    </capabilities>
    <authentication>
      <scheme>mTLS</scheme>
    </authentication>
  </agent>

</agentsitemap>
```

**Key Design Decisions:**

1. **`<status>` field:** Explicit lifecycle states (`active`, `beta`, `deprecated`, `private`) allow crawlers to adjust behavior without fetching each card.
2. **`<deprecation>` block:** Enables graceful migration — crawlers can redirect consumers to successors and index sunset timelines.
3. **`<priority>` field:** Provider-declared importance hint (0.0–1.0). Crawlers MAY use this to prioritize fetch order but MUST NOT treat it as a ranking signal (to prevent gaming).
4. **`<visibility>` field:** Providers can signal that certain agents should not be publicly indexed. Default is `public`.
5. **Inline `<skills>` summary:** Allows crawlers to build a partial index from the sitemap alone, deferring full card fetches for detailed indexing.

**Crawl Behavior:**

- AgentBot SHOULD fetch `agent-sitemap.xml` before probing individual `/.well-known/agent.json` endpoints.
- If a sitemap exists, all agents listed SHOULD be fetched; agents NOT listed MAY still be discovered via other vectors.
- `<lastmod>` enables conditional fetching — if the sitemap's `lastmod` for an agent hasn't changed since last crawl, the individual card fetch can be skipped.
- Sitemaps MUST NOT exceed 50MB uncompressed or 50,000 agent entries. Providers with more agents SHOULD use sitemap index files.

**Sitemap Index (for large providers):**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<agentsitemapindex xmlns="https://schemas.agentfi.dev/sitemap/1.0">
  <sitemap>
    <loc>https://agents.acmecorp.com/.well-known/agent-sitemap-finance.xml</loc>
    <lastmod>2026-03-23T14:30:00Z</lastmod>
  </sitemap>
  <sitemap>
    <loc>https://agents.acmecorp.com/.well-known/agent-sitemap-dev-tools.xml</loc>
    <lastmod>2026-03-22T09:00:00Z</lastmod>
  </sitemap>
  <sitemap>
    <loc>https://agents.acmecorp.com/.well-known/agent-sitemap-data.xml</loc>
    <lastmod>2026-03-21T16:00:00Z</lastmod>
  </sitemap>
</agentsitemapindex>
```

---

#### 8.1.2 `agent-robots.txt` — Crawl Directives for Agent Discovery

**Purpose:** Gives agent providers explicit control over how discovery crawlers interact with their agent infrastructure. Extends the semantics of `robots.txt` for the agent domain.

**Location:** `https://{provider-domain}/.well-known/agent-robots.txt`

**Full Specification Example:**

```
# =================================================================
# Agent Discovery Crawl Directives for AcmeCorp
# https://acmecorp.com/.well-known/agent-robots.txt
#
# This file controls how agent discovery crawlers (AgentBot and
# compatible crawlers) interact with our agent infrastructure.
#
# Last updated: 2026-03-23
# Contact: agents@acmecorp.com
# =================================================================

# --- Global Rules (apply to all agent crawlers) ---

User-agent: *
Allow: /.well-known/agent.json
Allow: /.well-known/agent-sitemap.xml
Allow: /.well-known/agent-proof.json
Disallow: /internal/
Disallow: /staging/
Disallow: /canary/
Disallow: /debug/
Disallow: /admin/
Crawl-delay: 2

# --- AgentFi Discovery Engine (Primary) ---

User-agent: AgentBot/1.0
Allow: /.well-known/agent.json
Allow: /.well-known/agent-sitemap.xml
Allow: /.well-known/agent-proof.json
Allow: /agents/*/health
Allow: /agents/*/capabilities
Disallow: /agents/internal-*
Disallow: /agents/*/sessions
Disallow: /agents/*/tasks
Disallow: /agents/*/artifacts
Crawl-delay: 1

# Liveness probing (AgentBot can health-check public agents)
Allow-probe: /agents/*/health
Probe-interval: 300
Probe-method: GET
Probe-accept: application/json

# Capability discovery depth
# shallow = card only; deep = card + skill enumeration + example tasks
Discovery-depth: deep

# Rate limit override: max 10 requests per minute to agent endpoints
Rate-limit: 10/minute

# Preferred crawl window (UTC)
Crawl-window: 02:00-06:00

# --- Third-Party Agent Crawlers ---

User-agent: AgentVerseBot/1.0
Allow: /.well-known/agent.json
Disallow: /agents/*/health
Disallow: /.well-known/agent-proof.json
Crawl-delay: 5
Discovery-depth: shallow

User-agent: MCPIndexer/2.0
Allow: /.well-known/agent.json
Allow: /.well-known/agent-sitemap.xml
Disallow: /agents/*/health
Crawl-delay: 3
Discovery-depth: shallow

# --- Block Known Bad Actors ---

User-agent: SpamCrawler
Disallow: /

User-agent: UnauthorizedBot
Disallow: /

# --- Metadata ---

# Sitemap location hint
Sitemap: https://agents.acmecorp.com/.well-known/agent-sitemap.xml

# Agent proof location
Agent-proof: https://agents.acmecorp.com/.well-known/agent-proof.json

# Provider verification
Verification-dns: _agentfi-verify.acmecorp.com
Verification-method: DNS-TXT

# Opt-out of specific discovery features
Opt-out: quality-probing
Opt-out: task-sampling

# GDPR/Privacy compliance
Data-retention: 90d
Data-processing-basis: legitimate-interest
Privacy-contact: privacy@acmecorp.com
```

**Extended Directives (beyond standard robots.txt):**

| Directive | Type | Description |
|---|---|---|
| `Allow-probe` | Path pattern | Endpoints that crawlers may health-check |
| `Probe-interval` | Seconds | Minimum interval between health probes |
| `Probe-method` | HTTP method | Allowed probe method (GET, HEAD, OPTIONS) |
| `Probe-accept` | MIME type | Expected response content type for probes |
| `Discovery-depth` | `shallow` / `deep` | How much detail the crawler may collect |
| `Rate-limit` | `N/period` | Maximum request rate (overrides Crawl-delay) |
| `Crawl-window` | `HH:MM-HH:MM` | Preferred UTC time window for crawling |
| `Agent-proof` | URL | Location of the agent-proof.json file |
| `Verification-dns` | Hostname | DNS TXT record for domain verification |
| `Verification-method` | Method name | Verification approach (DNS-TXT, meta-tag, file) |
| `Opt-out` | Feature name | Specific discovery features the provider declines |
| `Data-retention` | Duration | Maximum time crawled data should be retained |
| `Data-processing-basis` | Legal basis | GDPR data processing justification |
| `Privacy-contact` | Email | Privacy-related contact address |

**Crawl Behavior:**

- AgentBot MUST fetch and respect `agent-robots.txt` before any other requests to the domain.
- If `agent-robots.txt` is absent, AgentBot falls back to standard `robots.txt` for the domain.
- If both are absent, AgentBot applies default politeness policies (§9.10).
- `Crawl-delay` values MUST be respected as minimum inter-request intervals per domain.
- `Disallow` paths MUST NOT be fetched, even if discovered through other vectors (e.g., sitemap references).
- `Opt-out: quality-probing` means the crawler MUST NOT send synthetic tasks to the agent for quality assessment.

---

#### 8.1.3 `agent-proof.json` — Cryptographic Identity Verification

**Purpose:** Provides cryptographic proof that an Agent Card was published by the entity that controls the domain. Prevents impersonation, man-in-the-middle card injection, and registry-level spoofing.

**Location:** `https://{provider-domain}/.well-known/agent-proof.json`

**Full Specification:**

```json
{
  "version": "1.0",
  "provider": {
    "domain": "acmecorp.com",
    "organization": "AcmeCorp Financial Services",
    "organization_id": "urn:lei:5493001KJTIIGC8Y1R12",
    "contact": "security@acmecorp.com"
  },
  "signing_key": {
    "algorithm": "Ed25519",
    "public_key": "MCowBQYDK2VwAyEA2K3GYXE7aFDFnBgMFsBpZ2gM9URz0iBDBxkGx1y7Wgs=",
    "key_id": "acmecorp-agent-signing-2026-q1",
    "created": "2026-01-01T00:00:00Z",
    "expires": "2027-01-01T00:00:00Z",
    "revocation_url": "https://acmecorp.com/.well-known/agent-key-revocation.json"
  },
  "previous_keys": [
    {
      "key_id": "acmecorp-agent-signing-2025-q3",
      "public_key": "MCowBQYDK2VwAyEAf7bGN2gKp5rT4wEvZ1sJ3oN9kQ5vY8xC2hLm0dR6uAk=",
      "created": "2025-07-01T00:00:00Z",
      "expired": "2026-01-01T00:00:00Z",
      "rotation_reason": "scheduled-rotation"
    }
  ],
  "agents": [
    {
      "agent_name": "FinancialAnalysisAgent",
      "card_url": "https://agents.acmecorp.com/financial-analysis/.well-known/agent.json",
      "card_hash": {
        "algorithm": "blake3",
        "digest": "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
      },
      "signature": "7KjR3vF9p2mN5xQ8bT4wL6cY1dA0hS3jE9uI2oP5rG8kW7nM4xV6zA1fH3bJ5tD2yU9iC0eR7wQ4sK6nL8pO1mX3gB5vF2jA9dT7hN4cS6wE0rY8uI1oP3qG5kW7n",
      "signed_at": "2026-03-20T09:15:00Z",
      "valid_until": "2026-04-20T09:15:00Z"
    },
    {
      "agent_name": "DocumentSummarizerAgent",
      "card_url": "https://agents.acmecorp.com/doc-summarizer/.well-known/agent.json",
      "card_hash": {
        "algorithm": "blake3",
        "digest": "b3e2f1d4c5a6b7e8f9d0c1a2b3e4f5d6c7a8b9e0f1d2c3a4b5e6f7d8c9a0b1"
      },
      "signature": "9mN5xQ8bT4wL6cY1dA0hS3jE9uI2oP5rG8kW7nM4xV6zA1fH3bJ5tD2yU9iC0eR7wQ4sK6nL8pO1mX3gB5vF2jA9dT7hN4cS6wE0rY8uI1oP3qG5kW7nR3vF9p2",
      "signed_at": "2026-03-18T11:00:00Z",
      "valid_until": "2026-04-18T11:00:00Z"
    },
    {
      "agent_name": "CodeReviewAgent",
      "card_url": "https://agents.acmecorp.com/code-review/.well-known/agent.json",
      "card_hash": {
        "algorithm": "blake3",
        "digest": "c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4"
      },
      "signature": "4wL6cY1dA0hS3jE9uI2oP5rG8kW7nM4xV6zA1fH3bJ5tD2yU9iC0eR7wQ4sK6nL8pO1mX3gB5vF2jA9dT7hN4cS6wE0rY8uI1oP3qG5kW7nR3vF9p2mN5xQ8bT",
      "signed_at": "2026-03-22T16:45:00Z",
      "valid_until": "2026-04-22T16:45:00Z"
    }
  ],
  "dns_verification": {
    "method": "TXT",
    "record_name": "_agentfi-verify.acmecorp.com",
    "expected_value": "agentfi-verify=acmecorp-agent-signing-2026-q1:blake3:d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3",
    "verified_at": "2026-03-23T00:00:00Z",
    "ttl": 86400
  },
  "certificate_transparency": {
    "domain": "agents.acmecorp.com",
    "ct_log_inclusion": [
      {
        "log_name": "Google Argon 2026",
        "log_id": "Y/Lbzeg7zCzPC3KEJ1drM6SNYXePvXWmOLHHaFRL2I0=",
        "timestamp": "2026-01-15T10:30:00Z",
        "sct_version": 1
      }
    ]
  },
  "attestations": [
    {
      "type": "domain-control",
      "method": "dns-txt",
      "verified_at": "2026-03-23T00:00:00Z",
      "evidence": "_agentfi-verify.acmecorp.com TXT record matches"
    },
    {
      "type": "organization-identity",
      "method": "lei-verification",
      "verified_at": "2026-02-01T00:00:00Z",
      "evidence": "LEI 5493001KJTIIGC8Y1R12 verified via GLEIF API"
    },
    {
      "type": "code-signing",
      "method": "sigstore-rekor",
      "verified_at": "2026-03-20T09:15:00Z",
      "evidence": "Agent binary signed and logged in Rekor transparency log",
      "rekor_entry": "https://rekor.sigstore.dev/api/v1/log/entries/24296fb24b8ad77a..."
    }
  ],
  "metadata": {
    "generated_at": "2026-03-23T14:30:00Z",
    "generator": "agentfi-proof-gen/1.2.0",
    "schema_url": "https://schemas.agentfi.dev/proof/1.0/agent-proof.schema.json"
  }
}
```

**Verification Algorithm:**

```
VERIFY-AGENT-PROOF(proof_json, agent_card):
  1. Parse proof_json, extract signing_key
  2. Verify signing_key.expires > NOW()
  3. Check revocation: GET signing_key.revocation_url → ensure key_id not listed
  4. Find agent entry in proof.agents[] matching agent_card.url
  5. Compute BLAKE3(canonical_json(agent_card)) → computed_hash
  6. Assert computed_hash == agent_entry.card_hash.digest
  7. Verify Ed25519(signing_key.public_key, agent_entry.signature, computed_hash)
  8. Assert agent_entry.valid_until > NOW()
  9. (Optional) DNS verification:
     a. Resolve TXT record at proof.dns_verification.record_name
     b. Assert TXT value matches proof.dns_verification.expected_value
  10. Return VERIFIED with confidence level based on attestation count
```

**Confidence Levels:**

| Attestations Present | Confidence Level | Trust Score Modifier |
|---|---|---|
| Domain control only | Basic | +0.1 |
| Domain + organization identity | Standard | +0.2 |
| Domain + organization + code signing | Strong | +0.35 |
| All above + CT log inclusion | Maximum | +0.5 |

---

#### 8.1.4 DNS-Based Discovery Hints

**Purpose:** Enables infrastructure-level agent discovery without HTTP requests. Allows DNS resolvers and network-layer tools to discover agent endpoints before establishing HTTP connections.

**SRV Records — Endpoint Discovery:**

```
; Service discovery for A2A agents
; Format: _service._proto.name TTL class SRV priority weight port target

_a2a._https.acmecorp.com.       300  IN  SRV  10 100 443 agents.acmecorp.com.
_a2a._https.acmecorp.com.       300  IN  SRV  20  50 443 agents-backup.acmecorp.com.

; Per-agent SRV records (optional, for direct agent addressing)
_a2a._https.financial-analysis.agents.acmecorp.com.  300  IN  SRV  10 100 443 agents.acmecorp.com.
_a2a._https.code-review.agents.acmecorp.com.         300  IN  SRV  10 100 443 agents.acmecorp.com.

; MCP endpoint discovery (for agents also exposing MCP)
_mcp._https.acmecorp.com.       300  IN  SRV  10 100 443 mcp.acmecorp.com.
```

**TXT Records — Metadata and Verification:**

```
; Agent discovery metadata
_agentfi.acmecorp.com.           3600  IN  TXT  "v=agentfi1; agents=7; sitemap=https://agents.acmecorp.com/.well-known/agent-sitemap.xml; proof=https://agents.acmecorp.com/.well-known/agent-proof.json"

; Domain verification (used by agent-proof.json)
_agentfi-verify.acmecorp.com.    3600  IN  TXT  "agentfi-verify=acmecorp-agent-signing-2026-q1:blake3:d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3"

; Agent capability hints (compact format)
_agentfi-caps.acmecorp.com.      3600  IN  TXT  "v=1; caps=streaming,push,history; auth=oauth2,bearer,mtls; skills=finance,code-review,summarization,translation"

; Protocol version support
_agentfi-proto.acmecorp.com.     3600  IN  TXT  "v=1; a2a=1.0; mcp=1.1; protocols=a2a,mcp"

; Rate limit advertisement
_agentfi-ratelimit.acmecorp.com. 3600  IN  TXT  "v=1; rpm=60; rpd=10000; burst=20; window=60s"
```

**TXT Record Field Definitions:**

| Record | Field | Type | Description |
|---|---|---|---|
| `_agentfi` | `v` | String | Protocol version (`agentfi1`) |
| `_agentfi` | `agents` | Integer | Total number of public agents |
| `_agentfi` | `sitemap` | URL | Location of agent-sitemap.xml |
| `_agentfi` | `proof` | URL | Location of agent-proof.json |
| `_agentfi-caps` | `caps` | CSV | Supported A2A capabilities |
| `_agentfi-caps` | `auth` | CSV | Supported auth schemes |
| `_agentfi-caps` | `skills` | CSV | Top-level skill categories |
| `_agentfi-proto` | `a2a` | Semver | A2A protocol version |
| `_agentfi-proto` | `mcp` | Semver | MCP protocol version (if supported) |
| `_agentfi-proto` | `protocols` | CSV | All supported protocols |
| `_agentfi-ratelimit` | `rpm` | Integer | Requests per minute limit |
| `_agentfi-ratelimit` | `rpd` | Integer | Requests per day limit |
| `_agentfi-ratelimit` | `burst` | Integer | Burst request allowance |
| `_agentfi-ratelimit` | `window` | Duration | Rate limit window |

**Crawl Behavior:**

1. AgentBot performs DNS lookup for `_agentfi.{domain}` TXT record FIRST.
2. If present, extract `sitemap` URL and `agents` count for pre-planning.
3. Resolve `_a2a._https.{domain}` SRV records for endpoint discovery.
4. Use `_agentfi-ratelimit` to pre-configure rate limiting before first HTTP request.
5. DNS results are cached per TTL and refresh asynchronously.

**Benefits of DNS-Based Discovery:**

- **Zero HTTP overhead:** Discovery metadata available without TLS handshake.
- **CDN/proxy transparent:** DNS records are not affected by reverse proxies or CDNs that might block well-known paths.
- **Bulk scanning friendly:** DNS queries are lightweight and can be parallelized across millions of domains.
- **Failover signaling:** SRV records naturally express priority and weight for load balancing.
- **Verification anchor:** DNS TXT records provide an independent verification channel separate from the HTTP-served content.

---

#### 8.1.5 Capability Taxonomy Documents

**Purpose:** Provides a standardized, hierarchical vocabulary for describing agent skills and capabilities. Analogous to Schema.org for web content, this taxonomy enables precise matching between consumer needs and agent capabilities.

**Location:** Published centrally at `https://schemas.agentfi.dev/taxonomy/v1/capabilities.json` with providers optionally publishing domain-specific extensions.

**Core Taxonomy Structure:**

```json
{
  "version": "1.0.0",
  "generated": "2026-03-23T00:00:00Z",
  "taxonomy_id": "agentfi-capability-taxonomy",
  "description": "Standardized hierarchical vocabulary for agent skill and capability classification",
  "namespaces": {
    "core": "https://schemas.agentfi.dev/taxonomy/v1/core",
    "finance": "https://schemas.agentfi.dev/taxonomy/v1/domains/finance",
    "engineering": "https://schemas.agentfi.dev/taxonomy/v1/domains/engineering",
    "content": "https://schemas.agentfi.dev/taxonomy/v1/domains/content",
    "data": "https://schemas.agentfi.dev/taxonomy/v1/domains/data"
  },
  "capability_tree": {
    "core:analysis": {
      "display_name": "Analysis & Reasoning",
      "description": "Capabilities related to analyzing information, reasoning over data, and producing insights",
      "children": {
        "core:analysis:quantitative": {
          "display_name": "Quantitative Analysis",
          "description": "Numerical and statistical analysis capabilities",
          "children": {
            "finance:analysis:portfolio": {
              "display_name": "Portfolio Analysis",
              "description": "Investment portfolio composition, risk, and performance analysis",
              "synonyms": ["portfolio review", "investment analysis", "holdings analysis"],
              "related": ["finance:risk:var", "finance:optimization:rebalancing"],
              "input_types": ["application/json", "text/csv"],
              "output_types": ["application/json", "text/markdown", "image/png"],
              "example_queries": [
                "Analyze my portfolio for concentration risk",
                "What is the sector breakdown of these holdings?"
              ]
            },
            "finance:analysis:valuation": {
              "display_name": "Valuation Analysis",
              "description": "Company and asset valuation using DCF, comps, and other methodologies",
              "synonyms": ["DCF analysis", "comparable analysis", "fair value"],
              "related": ["finance:analysis:portfolio", "finance:research:fundamentals"]
            },
            "data:analysis:statistical": {
              "display_name": "Statistical Analysis",
              "description": "Hypothesis testing, regression, correlation, and descriptive statistics",
              "synonyms": ["stats", "regression analysis", "hypothesis testing"],
              "related": ["data:ml:prediction", "data:visualization:charts"]
            }
          }
        },
        "core:analysis:qualitative": {
          "display_name": "Qualitative Analysis",
          "description": "Text, sentiment, and subjective analysis capabilities",
          "children": {
            "content:analysis:sentiment": {
              "display_name": "Sentiment Analysis",
              "description": "Detecting and classifying sentiment in text content",
              "synonyms": ["opinion mining", "mood detection", "tone analysis"]
            },
            "content:analysis:summarization": {
              "display_name": "Content Summarization",
              "description": "Condensing documents, articles, or conversations into key points",
              "synonyms": ["summary", "TLDR", "digest", "abstract"]
            },
            "content:analysis:classification": {
              "display_name": "Content Classification",
              "description": "Categorizing and labeling text content by topic, type, or intent",
              "synonyms": ["categorization", "labeling", "tagging"]
            }
          }
        }
      }
    },
    "core:generation": {
      "display_name": "Content Generation",
      "description": "Capabilities related to creating new content",
      "children": {
        "core:generation:text": {
          "display_name": "Text Generation",
          "description": "Creating written content in various formats",
          "children": {
            "content:generation:code": {
              "display_name": "Code Generation",
              "description": "Writing, completing, and generating source code",
              "synonyms": ["coding", "programming", "code writing"],
              "metadata": {
                "languages": ["python", "javascript", "typescript", "rust", "go", "java", "c++"],
                "frameworks": ["react", "django", "fastapi", "spring"]
              }
            },
            "content:generation:prose": {
              "display_name": "Prose Generation",
              "description": "Writing articles, blog posts, documentation, and other long-form text",
              "synonyms": ["writing", "copywriting", "content creation"]
            },
            "content:generation:translation": {
              "display_name": "Language Translation",
              "description": "Translating text between natural languages",
              "synonyms": ["translate", "localization", "i18n"],
              "metadata": {
                "language_pairs": "dynamic"
              }
            }
          }
        },
        "core:generation:media": {
          "display_name": "Media Generation",
          "description": "Creating images, audio, video, and other media",
          "children": {
            "content:generation:image": {
              "display_name": "Image Generation",
              "output_types": ["image/png", "image/jpeg", "image/webp", "image/svg+xml"]
            },
            "content:generation:audio": {
              "display_name": "Audio Generation",
              "output_types": ["audio/mpeg", "audio/wav", "audio/ogg"]
            }
          }
        }
      }
    },
    "core:action": {
      "display_name": "Actions & Operations",
      "description": "Capabilities related to performing operations and side effects",
      "children": {
        "core:action:api": {
          "display_name": "API Operations",
          "description": "Interacting with external APIs and services",
          "children": {
            "engineering:action:deployment": {
              "display_name": "Deployment & DevOps",
              "description": "Deploying applications, managing infrastructure, CI/CD",
              "synonyms": ["deploy", "CI/CD", "infrastructure", "DevOps"]
            },
            "data:action:etl": {
              "display_name": "ETL & Data Pipeline",
              "description": "Extract, transform, load operations for data processing",
              "synonyms": ["data pipeline", "data ingestion", "data transformation"]
            }
          }
        },
        "core:action:communication": {
          "display_name": "Communication",
          "description": "Sending messages, notifications, and alerts",
          "children": {
            "core:action:email": {
              "display_name": "Email Operations",
              "synonyms": ["send email", "email draft", "email management"]
            },
            "core:action:notification": {
              "display_name": "Notification & Alerting",
              "synonyms": ["alert", "notify", "push notification"]
            }
          }
        }
      }
    },
    "core:review": {
      "display_name": "Review & Validation",
      "description": "Capabilities related to reviewing, validating, and quality-checking",
      "children": {
        "engineering:review:code": {
          "display_name": "Code Review",
          "description": "Reviewing source code for bugs, style, performance, and security",
          "children": {
            "engineering:review:security": {
              "display_name": "Security Review",
              "description": "Identifying security vulnerabilities, OWASP compliance, SAST",
              "synonyms": ["security audit", "vulnerability scan", "SAST", "pentest"]
            },
            "engineering:review:performance": {
              "display_name": "Performance Review",
              "description": "Identifying performance bottlenecks and optimization opportunities",
              "synonyms": ["profiling", "optimization review", "perf audit"]
            }
          }
        },
        "data:review:quality": {
          "display_name": "Data Quality",
          "description": "Validating data completeness, accuracy, and consistency",
          "synonyms": ["data validation", "data profiling", "quality check"]
        }
      }
    }
  },
  "matching_rules": {
    "exact_match_weight": 1.0,
    "parent_match_weight": 0.6,
    "sibling_match_weight": 0.4,
    "synonym_match_weight": 0.85,
    "related_match_weight": 0.5,
    "max_traversal_depth": 3
  }
}
```

**Provider-Specific Extension Example:**

```json
{
  "version": "1.0.0",
  "extends": "https://schemas.agentfi.dev/taxonomy/v1/capabilities.json",
  "provider": "acmecorp.com",
  "extensions": {
    "finance:analysis:portfolio:acme-proprietary-risk": {
      "display_name": "AcmeCorp Proprietary Risk Model",
      "description": "Risk analysis using AcmeCorp's proprietary multi-factor model",
      "parent": "finance:analysis:portfolio",
      "proprietary": true,
      "requires_auth": true,
      "certification": "SOC2-Type-II"
    }
  }
}
```

**Taxonomy Usage in Agent Cards:**

Agents reference taxonomy IDs in their skill declarations, enabling precise matching:

```json
{
  "skills": [
    {
      "id": "portfolio-analysis",
      "name": "Portfolio Risk Analysis",
      "taxonomy_ids": [
        "finance:analysis:portfolio",
        "finance:risk:var"
      ],
      "taxonomy_version": "1.0.0"
    }
  ]
}
```

---

## 9. Discovery Engine: The Crawler (AgentBot)

### Overview

AgentBot is the autonomous discovery engine that systematically finds, fetches, validates, normalizes, and tracks Agent Cards across the open internet. It is the equivalent of Googlebot for the agent ecosystem — a distributed, polite, high-throughput crawler purpose-built for the A2A protocol.

AgentBot operates on the principle of **evidence-driven discovery**: every claim an agent makes in its Agent Card is treated as a hypothesis to be verified, not a fact to be indexed verbatim. The crawler collects multiple independent signals — HTTP metadata, DNS records, TLS certificate chains, liveness probes, content hashes, and cross-references from other registries — to build a comprehensive evidence profile for each agent.

```
┌─────────────────────────────────────────────────────────────────────┐
│                        AgentBot Architecture                        │
│                                                                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│  │ Discovery │  │  Crawl   │  │  Crawl   │  │ Registry │           │
│  │ Vectors   │──│ Frontier │──│ Pipeline │──│ Emitter  │           │
│  │  (§9.1)   │  │  (§9.5)  │  │  (§9.3)  │  │          │           │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘           │
│       │              │              │              │                 │
│       ▼              ▼              ▼              ▼                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│  │  URL     │  │ Schedule │  │ Validate │  │Aggregated│           │
│  │  Seeds   │  │ Manager  │  │ & Diff   │  │ Registry │           │
│  │          │  │  (§9.6)  │  │  (§9.4)  │  │  (§10)   │           │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘           │
│                                                                     │
│  Infrastructure: Rust + Tokio + reqwest + Redis + ScyllaDB          │
└─────────────────────────────────────────────────────────────────────┘
```

---

### 9.1 Discovery Vectors

AgentBot discovers new agent endpoints through six complementary vectors, each contributing different types of candidate URLs. The vectors are designed to maximize coverage while minimizing false positives.

#### Vector 1: DNS / Subdomain Probing

**Purpose:** Systematically probe known hosting patterns to discover agents at predictable well-known paths.

**Seed Sources:**

| Source | Coverage | Update Frequency | Est. Domains |
|---|---|---|---|
| Tranco Top 10M | Global web traffic leaders | Monthly | 10,000,000 |
| Cloud platform subdomains (`*.fly.dev`) | Fly.io deployments | Continuous | ~500,000 |
| Cloud platform subdomains (`*.railway.app`) | Railway deployments | Continuous | ~200,000 |
| Cloud platform subdomains (`*.vercel.app`) | Vercel deployments | Continuous | ~2,000,000 |
| Cloud platform subdomains (`*.netlify.app`) | Netlify deployments | Continuous | ~1,500,000 |
| Cloud platform subdomains (`*.render.com`) | Render deployments | Continuous | ~300,000 |
| Cloud platform subdomains (`*.ondigitalocean.app`) | DigitalOcean App Platform | Continuous | ~200,000 |
| Cloud platform subdomains (`*.azurewebsites.net`) | Azure App Service | Continuous | ~5,000,000 |
| Cloud platform subdomains (`*.herokuapp.com`) | Heroku | Continuous | ~3,000,000 |
| Cloud platform subdomains (`*.run.app`) | Google Cloud Run | Continuous | ~1,000,000 |
| Cloud platform subdomains (`*.lambda-url.*.on.aws`) | AWS Lambda URLs | Continuous | ~2,000,000 |
| DNS zone transfers (where permitted) | Cooperative registrars | On-demand | Variable |
| Certificate Transparency logs | All HTTPS domains | Real-time | ~500,000,000 |
| Passive DNS feeds (Farsight DNSDB, etc.) | Historical DNS resolutions | Continuous | ~1,000,000,000 |

**Probing Algorithm:**

```
FUNCTION dns_subdomain_probe(domain_list: Vec<String>) -> Vec<CandidateURL>:
    candidates = []
    
    FOR EACH domain IN domain_list:
        // Phase 1: DNS pre-check (no HTTP overhead)
        dns_hints = resolve_txt("_agentfi." + domain)
        IF dns_hints.is_valid():
            // Fast path: domain explicitly declares agents
            sitemap_url = dns_hints.extract("sitemap")
            IF sitemap_url:
                candidates.push(CandidateURL {
                    url: sitemap_url,
                    source: "dns-txt-sitemap",
                    confidence: 0.95,
                    priority: HIGH
                })
            CONTINUE  // Skip generic probing for this domain
        
        srv_records = resolve_srv("_a2a._https." + domain)
        IF srv_records.is_not_empty():
            FOR EACH srv IN srv_records:
                candidates.push(CandidateURL {
                    url: format!("https://{}/.well-known/agent.json", srv.target),
                    source: "dns-srv",
                    confidence: 0.90,
                    priority: HIGH
                })
            CONTINUE
        
        // Phase 2: HTTP probing (HEAD requests first)
        well_known_paths = [
            "/.well-known/agent.json",
            "/.well-known/agent-sitemap.xml",
            "/agent.json",                    // Non-standard but common
            "/api/agent-card",                // Alternative pattern
        ]
        
        FOR EACH path IN well_known_paths:
            url = format!("https://{}{}", domain, path)
            response = http_head(url, timeout=5s)
            
            IF response.status == 200
               AND response.content_type.contains("application/json"):
                candidates.push(CandidateURL {
                    url: url,
                    source: "dns-subdomain-probe",
                    confidence: 0.70,
                    priority: MEDIUM
                })
                BREAK  // Found an agent endpoint, stop probing paths
            
            IF response.status == 301 OR response.status == 302:
                redirect_url = response.headers["Location"]
                candidates.push(CandidateURL {
                    url: redirect_url,
                    source: "dns-subdomain-probe-redirect",
                    confidence: 0.65,
                    priority: MEDIUM
                })
                BREAK
        
        // Phase 3: Common agent subdomain patterns
        agent_subdomains = [
            "agents." + domain,
            "agent." + domain,
            "api." + domain,
            "a2a." + domain,
            "mcp." + domain,
        ]
        
        FOR EACH subdomain IN agent_subdomains:
            IF dns_resolves(subdomain):
                candidates.push(CandidateURL {
                    url: format!("https://{}/.well-known/agent.json", subdomain),
                    source: "dns-subdomain-pattern",
                    confidence: 0.50,
                    priority: LOW
                })
    
    RETURN candidates
```

**Scale Targets for DNS Probing:**

- Phase 1: 1M domains/day probed via DNS, 100K HTTP HEAD requests/day
- Phase 2: 10M domains/day probed via DNS, 1M HTTP HEAD requests/day
- Phase 3: 50M domains/day probed via DNS, 5M HTTP HEAD requests/day

---

#### Vector 2: Certificate Transparency Log Mining

**Purpose:** Discover new agent-hosting domains in near-real-time by monitoring Certificate Transparency (CT) logs for certificate issuances that match agent-hosting patterns.

**Target CT Logs:**

| Log | Operator | Estimated Volume | Monitoring Priority |
|---|---|---|---|
| Google Argon | Google | ~5B certs | Critical |
| Google Xenon | Google | ~3B certs | Critical |
| Cloudflare Nimbus | Cloudflare | ~2B certs | High |
| DigiCert Yeti | DigiCert | ~1B certs | High |
| Let's Encrypt Oak | ISRG | ~4B certs | Critical |
| Sectigo Sabre | Sectigo | ~500M certs | Medium |

**Processing Pipeline:**

```
CT Log Stream          Filtering            DNS Pre-check         HTTP Probe
┌──────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│           │     │              │     │              │     │              │
│  Websocket│     │ Domain       │     │ _agentfi TXT │     │ HEAD request │
│  / polling│────▶│ pattern      │────▶│ record check │────▶│ to well-known│
│  from CT  │     │ matching     │     │              │     │ path         │
│  logs     │     │              │     │              │     │              │
│           │     │ *.agents.*   │     │ If TXT found │     │ If 200 + JSON│
│  ~10K     │     │ *.a2a.*     │     │ → HIGH conf  │     │ → enqueue    │
│  certs/sec│     │ *.ai.*      │     │ If not found │     │ for full     │
│           │     │ *.bot.*     │     │ → MEDIUM conf│     │ crawl        │
│           │     │ Cloud PLT   │     │              │     │              │
│           │     │ patterns    │     │              │     │              │
│           │     │              │     │              │     │              │
│           │     │ ~500         │     │ ~200         │     │ ~50          │
│           │     │ matches/sec  │     │ lookups/sec  │     │ probes/sec   │
└──────────┘     └──────────────┘     └──────────────┘     └──────────────┘
                                                                    │
                                                                    ▼
                                                           ┌──────────────┐
                                                           │   URL        │
                                                           │   Frontier   │
                                                           │   (§9.5)     │
                                                           └──────────────┘
```

**Filtering Heuristics:**

```
FUNCTION ct_domain_filter(cert_entry: CTLogEntry) -> Option<CandidateURL>:
    domain = cert_entry.subject_common_name
    sans = cert_entry.subject_alternative_names
    
    all_domains = [domain] + sans
    
    FOR EACH d IN all_domains:
        // High-confidence patterns
        IF d.matches_regex(r"^agents?\..+\..+$"):        // agents.example.com
            RETURN Some(candidate(d, confidence=0.80))
        IF d.matches_regex(r"^a2a\..+\..+$"):            // a2a.example.com
            RETURN Some(candidate(d, confidence=0.85))
        IF d.matches_regex(r"^mcp\..+\..+$"):            // mcp.example.com
            RETURN Some(candidate(d, confidence=0.70))
        
        // Medium-confidence: cloud platform patterns
        IF d.ends_with(".fly.dev") OR d.ends_with(".railway.app")
           OR d.ends_with(".run.app") OR d.ends_with(".vercel.app"):
            // Only if subdomain contains agent-related keywords
            subdomain = d.split('.')[0]
            IF subdomain.contains_any(["agent", "a2a", "mcp", "bot", "ai-"]):
                RETURN Some(candidate(d, confidence=0.60))
        
        // Low-confidence: wildcard AI/ML domains
        IF d.matches_regex(r".*\b(ai|ml|llm|gpt|claude)\b.*"):
            RETURN Some(candidate(d, confidence=0.30))
    
    RETURN None

FUNCTION candidate(domain: String, confidence: f64) -> CandidateURL:
    RETURN CandidateURL {
        url: format!("https://{}/.well-known/agent.json", domain),
        source: "ct-log-mining",
        confidence: confidence,
        discovered_at: now(),
        ct_log_entry_id: cert_entry.log_id,
        priority: if confidence > 0.7 { HIGH } else { MEDIUM }
    }
```

**Target Latency:** < 60 seconds from certificate issuance to candidate URL in the frontier. This enables near-real-time discovery of newly deployed agents.

**Volume Estimates:**

- CT log ingestion: ~10,000 certificate entries per second across all monitored logs
- After domain pattern filtering: ~500 candidate domains per second
- After DNS pre-check: ~200 domains with potential agent endpoints per second
- After HTTP probing: ~50 confirmed agent card URLs per second
- Daily yield: ~100,000–500,000 new candidate URLs

---

#### Vector 3: Referral Graph Walking

**Purpose:** Discover new agents by following references from already-known agents. Agent Cards may reference other agents (e.g., in skill descriptions, provider metadata, or through a proposed `relatedAgents` field). Walking this referral graph mirrors how web crawlers follow hyperlinks.

**Reference Sources:**

| Source | Signal Type | Confidence |
|---|---|---|
| Agent Card `provider.url` → other agent cards at same provider | Co-location | 0.85 |
| Agent Card `skills[].description` mentions of other agents | Semantic reference | 0.50 |
| Agent-sitemap.xml entries | Explicit declaration | 0.95 |
| A2A task responses referencing other agent endpoints | Runtime reference | 0.75 |
| Proposed `relatedAgents[]` field in Agent Card | Explicit link | 0.90 |
| Agent delegation chains (agent A delegates to agent B) | Behavioral reference | 0.80 |
| Shared authentication domains | Infrastructure affinity | 0.60 |

**Graph Walking Algorithm:**

```
FUNCTION referral_graph_walk(
    seed_agents: Vec<AgentCardRecord>,
    max_depth: u32,
    max_candidates_per_round: usize
) -> Vec<CandidateURL>:
    
    visited: HashSet<Url> = seed_agents.map(|a| a.source_url).collect()
    frontier: PriorityQueue<(Url, f64, u32)> = PriorityQueue::new()  // (url, confidence, depth)
    all_candidates: Vec<CandidateURL> = []
    
    // Initialize frontier with references from seed agents
    FOR EACH agent IN seed_agents:
        refs = extract_references(agent)
        FOR EACH (ref_url, ref_confidence) IN refs:
            IF NOT visited.contains(ref_url):
                frontier.push((ref_url, ref_confidence, 1))
    
    WHILE NOT frontier.is_empty() AND all_candidates.len() < max_candidates_per_round:
        (url, confidence, depth) = frontier.pop_max()  // highest confidence first
        
        IF visited.contains(url) OR depth > max_depth:
            CONTINUE
        
        visited.insert(url)
        
        // Attempt to fetch and validate the candidate
        result = fetch_and_validate(url)
        
        IF result.is_valid_agent_card():
            all_candidates.push(CandidateURL {
                url: url,
                source: "referral-graph",
                confidence: confidence * depth_decay(depth),
                referrer: result.referrer,
                depth: depth,
                priority: confidence_to_priority(confidence * depth_decay(depth))
            })
            
            // Extract further references from this newly discovered agent
            IF depth < max_depth:
                new_refs = extract_references(result.agent_card_record)
                FOR EACH (ref_url, ref_confidence) IN new_refs:
                    IF NOT visited.contains(ref_url):
                        // Decay confidence with depth to prevent unbounded exploration
                        decayed_confidence = ref_confidence * depth_decay(depth + 1)
                        IF decayed_confidence > MIN_CONFIDENCE_THRESHOLD:
                            frontier.push((ref_url, decayed_confidence, depth + 1))
    
    RETURN all_candidates

FUNCTION depth_decay(depth: u32) -> f64:
    // Exponential decay: 1.0 at depth 1, 0.7 at depth 2, 0.49 at depth 3
    RETURN 0.7_f64.pow((depth - 1) as f64)

FUNCTION extract_references(record: AgentCardRecord) -> Vec<(Url, f64)>:
    refs = []
    
    // 1. Same-provider discovery
    provider_domain = record.card.provider.url.domain()
    refs.push((
        format!("https://{}/.well-known/agent-sitemap.xml", provider_domain),
        0.90
    ))
    
    // 2. Explicit related agents (proposed field)
    IF record.card.related_agents.is_some():
        FOR EACH related IN record.card.related_agents:
            refs.push((related.card_url, 0.90))
    
    // 3. Shared auth domain analysis
    FOR EACH scheme IN record.card.authentication.schemes:
        IF scheme.authorization_url.is_some():
            auth_domain = scheme.authorization_url.domain()
            // Look for other agents using the same auth provider
            refs.push((
                format!("https://{}/.well-known/agent-sitemap.xml", auth_domain),
                0.50
            ))
    
    // 4. Semantic reference extraction from descriptions
    all_text = record.card.description + record.card.skills.map(|s| s.description).join(" ")
    extracted_urls = extract_urls_from_text(all_text)
    FOR EACH url IN extracted_urls:
        IF url.path().contains("agent") OR url.path().contains("a2a"):
            refs.push((url, 0.40))
    
    RETURN refs
```

---

#### Vector 4: Registry Federation Ingestion

**Purpose:** Ingest agent metadata from existing public registries, directories, and catalogs. These third-party sources provide pre-curated lists of agents that complement organic discovery.

**Target Registries:**

| Registry | Type | API Format | Update Frequency | Est. Agents |
|---|---|---|---|---|
| AgentVerse (Fetch.ai) | A2A/Agent registry | REST JSON | Real-time | ~5,000 |
| PulseMCP | MCP server directory | REST JSON | Hourly | ~3,000 |
| mcp.so | MCP server catalog | REST JSON | Daily | ~2,000 |
| Hugging Face Online Labs (HOL) | AI model/agent hub | REST JSON / GraphQL | Hourly | ~10,000 |
| GitHub Topics (`a2a-agent`, `mcp-server`) | Code repositories | GraphQL | Daily | ~15,000 |
| npm registry (`@a2a/*`, `@mcp/*`) | Package registry | REST JSON | Continuous | ~5,000 |
| PyPI (`a2a-*`, `mcp-*`) | Package registry | REST JSON | Continuous | ~3,000 |
| Docker Hub (`*/a2a-*`, `*/mcp-*`) | Container registry | REST JSON | Daily | ~8,000 |
| Smithery.ai | MCP marketplace | REST JSON | Hourly | ~1,000 |
| Glama.ai | AI tool directory | REST JSON | Daily | ~2,000 |
| OpenRouter | LLM/Agent directory | REST JSON | Hourly | ~500 |

**Registry Ingestion Function:**

```
FUNCTION ingest_registry(
    registry: RegistryConfig,
    last_sync_cursor: Option<String>
) -> RegistryIngestionResult:
    
    result = RegistryIngestionResult::new()
    
    // 1. Fetch registry listing (paginated)
    cursor = last_sync_cursor.unwrap_or(registry.initial_cursor)
    
    LOOP:
        page = http_get(
            url: registry.api_endpoint,
            params: {
                "cursor": cursor,
                "limit": registry.page_size,
                "updated_since": last_sync_cursor.map(|c| c.timestamp),
                "sort": "updated_desc"
            },
            headers: {
                "Authorization": registry.api_key,
                "Accept": "application/json"
            },
            timeout: 30s
        )
        
        IF page.status != 200:
            result.errors.push(RegistryError {
                registry: registry.name,
                status: page.status,
                message: page.body
            })
            BREAK
        
        entries = registry.parser.parse(page.body)
        
        FOR EACH entry IN entries:
            // 2. Normalize registry-specific format to canonical
            normalized = normalize_registry_entry(registry.format, entry)
            
            // 3. Attempt to resolve to an A2A Agent Card URL
            card_url = resolve_to_agent_card(normalized)
            
            IF card_url.is_some():
                result.candidates.push(CandidateURL {
                    url: card_url.unwrap(),
                    source: format!("registry:{}", registry.name),
                    confidence: registry.base_confidence,
                    registry_metadata: RegistryMetadata {
                        registry_name: registry.name,
                        registry_id: entry.id,
                        registry_url: entry.url,
                        stars: entry.stars,
                        downloads: entry.downloads,
                        last_updated: entry.updated_at,
                        verified: entry.verified,
                        categories: entry.categories,
                    },
                    priority: HIGH
                })
            ELSE:
                // No direct Agent Card URL; store as unresolved for later probing
                result.unresolved.push(UnresolvedEntry {
                    registry: registry.name,
                    entry_id: entry.id,
                    entry_url: entry.url,
                    metadata: normalized,
                    probe_urls: generate_probe_urls(normalized)
                })
        
        cursor = page.next_cursor
        IF cursor.is_none():
            BREAK
    
    result.sync_cursor = cursor
    RETURN result

FUNCTION resolve_to_agent_card(entry: NormalizedRegistryEntry) -> Option<Url>:
    // Strategy 1: Direct Agent Card URL in metadata
    IF entry.agent_card_url.is_some():
        RETURN Some(entry.agent_card_url)
    
    // Strategy 2: Construct from endpoint URL
    IF entry.endpoint_url.is_some():
        candidate = entry.endpoint_url + "/.well-known/agent.json"
        IF http_head(candidate).status == 200:
            RETURN Some(candidate)
    
    // Strategy 3: Construct from homepage/docs URL
    IF entry.homepage_url.is_some():
        domain = entry.homepage_url.domain()
        FOR EACH pattern IN ["agents.{}", "api.{}", "{}"]:
            candidate = format!("https://{}/.well-known/agent.json", pattern.replace("{}", domain))
            IF http_head(candidate).status == 200:
                RETURN Some(candidate)
    
    // Strategy 4: GitHub repo → inspect for deployment URL
    IF entry.source_repo.is_some() AND entry.source_repo.contains("github.com"):
        readme = fetch_github_readme(entry.source_repo)
        urls = extract_deployment_urls(readme)
        FOR EACH url IN urls:
            candidate = url + "/.well-known/agent.json"
            IF http_head(candidate).status == 200:
                RETURN Some(candidate)
    
    RETURN None
```

---

#### Vector 5: Open Web Discovery

**Purpose:** Discover agents by mining the broader web for references, documentation, and deployment artifacts that indicate agent existence.

**Target Sources:**

| Source Type | Specific Sources | Discovery Method |
|---|---|---|
| **Provider Pages** | Official agent landing pages, pricing pages, API docs | URL pattern matching, structured data extraction |
| **Documentation Sites** | ReadTheDocs, GitBook, Notion, Mintlify, Docusaurus | Keyword search + endpoint extraction |
| **GitHub Repositories** | `*.well-known/agent.json` in repos, deployment configs | GitHub Code Search API, file pattern matching |
| **Package Manifests** | `package.json`, `pyproject.toml`, `Cargo.toml` with A2A deps | Package registry search + repo inspection |
| **OCI/Container Registries** | Docker Hub, GitHub Container Registry, AWS ECR Public | Image labels, Dockerfile inspection |
| **Common Crawl** | Monthly web crawl archive (~3B pages) | MapReduce over WARC files, agent URL extraction |
| **API Documentation** | OpenAPI/Swagger specs mentioning A2A | Schema field matching |
| **Blog Posts / Announcements** | Company engineering blogs, product announcements | NER + URL extraction |
| **Conference Talks / Papers** | Slides, papers mentioning agent deployments | Citation/URL extraction |

**Common Crawl Processing (monthly batch):**

```
FUNCTION process_common_crawl(crawl_month: String) -> Vec<CandidateURL>:
    candidates = []
    
    // Common Crawl stores data as WARC (Web ARChive) files
    // Each monthly crawl is ~60-80TB, ~3B pages
    warc_paths = list_common_crawl_warcs(crawl_month)  // ~90,000 WARC files
    
    // Distributed processing via Apache Spark / AWS EMR
    FOR EACH warc_path IN warc_paths PARALLEL:
        warc_stream = open_warc(warc_path)
        
        FOR EACH record IN warc_stream:
            IF record.content_type != "text/html" AND record.content_type != "application/json":
                CONTINUE
            
            url = record.target_uri
            body = record.payload
            
            // Strategy 1: Direct agent card captures
            IF url.ends_with("/.well-known/agent.json"):
                candidates.push(CandidateURL {
                    url: url,
                    source: "common-crawl-direct",
                    confidence: 0.95,
                    priority: HIGH
                })
                CONTINUE
            
            // Strategy 2: HTML pages referencing agent endpoints
            IF record.content_type == "text/html":
                agent_urls = extract_agent_references(body)
                FOR EACH agent_url IN agent_urls:
                    candidates.push(CandidateURL {
                        url: agent_url,
                        source: "common-crawl-reference",
                        confidence: 0.55,
                        referrer: url,
                        priority: MEDIUM
                    })
            
            // Strategy 3: JSON-LD / structured data with agent schema
            structured_data = extract_json_ld(body)
            IF structured_data.contains_agent_schema():
                candidates.push(CandidateURL {
                    url: structured_data.agent_endpoint,
                    source: "common-crawl-structured",
                    confidence: 0.70,
                    priority: MEDIUM
                })
    
    // Deduplicate
    candidates = deduplicate_by_url(candidates)
    
    RETURN candidates
```

---

#### Vector 6: Community Hint API

**Purpose:** Accept user-submitted hints about agent endpoints. This enables a human-in-the-loop discovery channel where developers, users, and agent operators can manually register agents for discovery.

**API Endpoint:** `POST /api/v1/hints`

**Request:**

```json
{
  "hints": [
    {
      "url": "https://agents.newstartup.io/.well-known/agent.json",
      "hint_type": "agent_card",
      "source": "developer_submission",
      "metadata": {
        "submitter_email": "dev@newstartup.io",
        "submitter_verified": false,
        "description": "Our new code review agent, just launched on Fly.io",
        "tags": ["code-review", "security", "python"],
        "estimated_skills": ["security-review", "code-quality"],
        "github_repo": "https://github.com/newstartup/code-review-agent"
      },
      "priority_request": "normal",
      "notes": "We just launched and would like to be discoverable ASAP"
    },
    {
      "url": "https://api.bigcorp.com/agents/",
      "hint_type": "provider_domain",
      "source": "community_report",
      "metadata": {
        "submitter_email": "user@example.com",
        "submitter_verified": true,
        "description": "BigCorp appears to have launched several A2A agents",
        "evidence_urls": [
          "https://engineering.bigcorp.com/blog/introducing-a2a-agents",
          "https://twitter.com/bigcorp/status/123456789"
        ]
      }
    }
  ],
  "callback_url": "https://newstartup.io/webhook/discovery-status",
  "idempotency_key": "hint-2026-03-23-newstartup-001"
}
```

**Response:**

```json
{
  "status": "accepted",
  "hint_results": [
    {
      "url": "https://agents.newstartup.io/.well-known/agent.json",
      "hint_id": "hint_01HXYZ123ABC456DEF789",
      "status": "queued",
      "estimated_crawl_time": "2026-03-23T15:30:00Z",
      "queue_position": 42,
      "message": "Hint accepted. Agent card URL will be probed within the next crawl cycle."
    },
    {
      "url": "https://api.bigcorp.com/agents/",
      "hint_id": "hint_01HXYZ123ABC456DEF790",
      "status": "queued",
      "estimated_crawl_time": "2026-03-23T16:00:00Z",
      "queue_position": 108,
      "message": "Provider domain hint accepted. Will probe for agent-sitemap.xml and well-known paths."
    }
  ],
  "rate_limit": {
    "remaining": 48,
    "limit": 50,
    "reset_at": "2026-03-24T00:00:00Z"
  },
  "callback_registered": true
}
```

**Rate Limits:**

| Submitter Type | Hints/Day | Priority Processing |
|---|---|---|
| Anonymous | 5 | No |
| Verified email | 50 | No |
| Verified domain owner | 500 | Yes |
| Partner/registry | 10,000 | Yes |
| Internal | Unlimited | Yes |

**Anti-Abuse Measures:**

- URL validation: must be HTTPS, must resolve, must not be on blocklist
- Duplicate detection: Cuckoo filter check before queueing
- Reputation scoring: repeat submitters with high-quality hints get priority
- Content moderation: submitted descriptions scanned for spam/abuse
- IP-based rate limiting in addition to account-based limits

---

### 9.2 Crawl Architecture (Rust + Tokio)

The crawler is implemented in Rust for maximum throughput, memory safety, and predictable latency. The async runtime is Tokio, with the HTTP client stack built on `reqwest` (which uses `hyper` under the hood).

**Architecture Diagram:**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          Tokio Runtime (multi-threaded)                      │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                        Worker Pool (N = num_cpus * 2)               │    │
│  │                                                                     │    │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐     │    │
│  │  │Worker 0 │ │Worker 1 │ │Worker 2 │ │Worker 3 │ │Worker N │     │    │
│  │  │         │ │         │ │         │ │         │ │         │     │    │
│  │  │Crawl    │ │Crawl    │ │Crawl    │ │Crawl    │ │Crawl    │     │    │
│  │  │Tasks    │ │Tasks    │ │Tasks    │ │Tasks    │ │Tasks    │     │    │
│  │  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘     │    │
│  │       │           │           │           │           │           │    │
│  └───────┼───────────┼───────────┼───────────┼───────────┼───────────┘    │
│          │           │           │           │           │                  │
│          ▼           ▼           ▼           ▼           ▼                  │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    Connection Pool (reqwest + hyper)                 │    │
│  │                                                                     │    │
│  │  ┌─────────────────────────────────────────────────────────────┐   │    │
│  │  │  Per-Domain Connection Limits                               │   │    │
│  │  │                                                             │   │    │
│  │  │  domain_a.com: [conn1, conn2] (max: 2)                     │   │    │
│  │  │  domain_b.io:  [conn1, conn2, conn3, conn4] (max: 6)       │   │    │
│  │  │  domain_c.dev: [conn1] (max: 2)                            │   │    │
│  │  │  ...                                                        │   │    │
│  │  │  Total active connections: ≤10,000                          │   │    │
│  │  └─────────────────────────────────────────────────────────────┘   │    │
│  │                                                                     │    │
│  │  ┌─────────────────────────────────────────────────────────────┐   │    │
│  │  │  TLS Session Reuse (rustls)                                 │   │    │
│  │  │                                                             │   │    │
│  │  │  Session cache: LRU, 100K entries                           │   │    │
│  │  │  TLS 1.3 0-RTT where supported                             │   │    │
│  │  │  Certificate pinning for known high-value providers         │   │    │
│  │  └─────────────────────────────────────────────────────────────┘   │    │
│  │                                                                     │    │
│  │  ┌─────────────────────────────────────────────────────────────┐   │    │
│  │  │  HTTP/2 Multiplexing                                        │   │    │
│  │  │                                                             │   │    │
│  │  │  Max concurrent streams per connection: 100                 │   │    │
│  │  │  HPACK header compression enabled                           │   │    │
│  │  │  Server push: disabled (not useful for crawling)            │   │    │
│  │  └─────────────────────────────────────────────────────────────┘   │    │
│  │                                                                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    Politeness Controller                             │    │
│  │                                                                     │    │
│  │  ┌─────────────────────────────────────────────────────────────┐   │    │
│  │  │  robots.txt / agent-robots.txt Cache                        │   │    │
│  │  │                                                             │   │    │
│  │  │  In-memory LRU: 500K domains                                │   │    │
│  │  │  Redis fallback for cache misses                            │   │    │
│  │  │  TTL: min(robots.txt Cache-Control, 24h)                    │   │    │
│  │  │  Negative cache (no robots.txt): 72h TTL                    │   │    │
│  │  └─────────────────────────────────────────────────────────────┘   │    │
│  │                                                                     │    │
│  │  ┌─────────────────────────────────────────────────────────────┐   │    │
│  │  │  Token Bucket Rate Limiter (per-domain)                     │   │    │
│  │  │                                                             │   │    │
│  │  │  Default: 1 request/second/domain                           │   │    │
│  │  │  Respects Crawl-delay directive (min 1s, max 60s)           │   │    │
│  │  │  429 backoff: exponential, 2^n seconds, max 3600s           │   │    │
│  │  │  Burst allowance: 3 requests (for sitemap + robots + card)  │   │    │
│  │  │  Implementation: dashmap + atomic token counters             │   │    │
│  │  └─────────────────────────────────────────────────────────────┘   │    │
│  │                                                                     │    │
│  │  ┌─────────────────────────────────────────────────────────────┐   │    │
│  │  │  Global Rate Governor                                       │   │    │
│  │  │                                                             │   │    │
│  │  │  Max global request rate: 50,000 req/sec                    │   │    │
│  │  │  Circuit breaker: trips at 20% error rate, resets after 60s │   │    │
│  │  │  Backpressure: slows frontier dequeue when >80% capacity    │   │    │
│  │  └─────────────────────────────────────────────────────────────┘   │    │
│  │                                                                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Core Data Structures:**

```rust
use chrono::{DateTime, Utc};
use url::Url;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentCardRecord {
    card: AgentCard,
    source_url: Url,
    fetch_timestamp: DateTime<Utc>,
    http_metadata: HttpMetadata,
    validation_result: ValidationResult,
    content_hash: Blake3Hash,
    previous_hash: Option<Blake3Hash>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HttpMetadata {
    status_code: u16,
    headers: Vec<(String, String)>,
    tls_version: Option<String>,
    tls_cipher_suite: Option<String>,
    tls_certificate_chain: Vec<CertificateInfo>,
    server_header: Option<String>,
    content_type: String,
    content_length: Option<u64>,
    cache_control: Option<String>,
    etag: Option<String>,
    last_modified: Option<DateTime<Utc>>,
    response_time_ms: u64,
    dns_resolve_time_ms: u64,
    tls_handshake_time_ms: u64,
    ttfb_ms: u64,
    total_transfer_time_ms: u64,
    ip_address: std::net::IpAddr,
    http_version: HttpVersion,
    redirect_chain: Vec<RedirectHop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CertificateInfo {
    subject: String,
    issuer: String,
    serial_number: String,
    not_before: DateTime<Utc>,
    not_after: DateTime<Utc>,
    subject_alternative_names: Vec<String>,
    fingerprint_sha256: String,
    is_ev: bool,
    ct_sct_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RedirectHop {
    url: Url,
    status_code: u16,
    location_header: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum HttpVersion {
    Http10,
    Http11,
    Http2,
    Http3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValidationResult {
    is_valid: bool,
    schema_version: String,
    errors: Vec<ValidationError>,
    warnings: Vec<ValidationWarning>,
    required_fields_present: RequiredFieldsCheck,
    skill_count: usize,
    capability_flags: CapabilityFlags,
    auth_schemes_valid: bool,
    url_consistency: UrlConsistencyCheck,
    proof_verification: Option<ProofVerificationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValidationError {
    field: String,
    error_type: ValidationErrorType,
    message: String,
    severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ValidationErrorType {
    MissingRequiredField,
    InvalidType,
    InvalidFormat,
    InvalidUrl,
    SchemaViolation,
    ConstraintViolation,
    AuthSchemeInvalid,
    SkillSchemaInvalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RequiredFieldsCheck {
    name: bool,
    description: bool,
    url: bool,
    version: bool,
    skills: bool,
    provider: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CapabilityFlags {
    streaming: bool,
    push_notifications: bool,
    state_transition_history: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UrlConsistencyCheck {
    card_url_matches_source: bool,
    provider_url_resolves: bool,
    auth_urls_resolve: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProofVerificationResult {
    proof_found: bool,
    signature_valid: bool,
    hash_matches: bool,
    dns_verified: bool,
    confidence_level: ProofConfidenceLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ProofConfidenceLevel {
    None,
    Basic,
    Standard,
    Strong,
    Maximum,
}

type Blake3Hash = [u8; 32];
```

---

### 9.3 Crawl Pipeline

The crawl pipeline processes each URL through ten sequential stages. Each stage is implemented as an async Tokio task that reads from an input channel and writes to an output channel, enabling backpressure propagation.

**Pipeline Diagram:**

```
 ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
 │          │   │          │   │          │   │          │   │          │
 │ DEQUEUE  │──▶│POLITENESS│──▶│   DNS    │──▶│   TLS    │──▶│  HTTP    │
 │          │   │  GATE    │   │ RESOLVE  │   │ CONNECT  │   │  FETCH   │
 │          │   │          │   │          │   │          │   │          │
 │ Pull URL │   │ Check    │   │ Resolve  │   │ TLS 1.3  │   │ GET with │
 │ from     │   │ robots,  │   │ A/AAAA   │   │ handshake│   │ headers, │
 │ frontier │   │ rate     │   │ records, │   │ cert     │   │ timeout, │
 │ (Redis)  │   │ limits,  │   │ cache    │   │ chain    │   │ redirect │
 │          │   │ opt-out  │   │ results  │   │ capture  │   │ follow   │
 └──────────┘   └──────────┘   └──────────┘   └──────────┘   └──────────┘
                                                                    │
 ┌──────────────────────────────────────────────────────────────────┘
 │
 ▼
 ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
 │          │   │          │   │          │   │          │   │          │
 │ VALIDATE │──▶│NORMALIZE │──▶│   DIFF   │──▶│   EMIT   │──▶│ SCHEDULE │
 │          │   │          │   │          │   │          │   │          │
 │ JSON     │   │ Canonical│   │ Compare  │   │ Kafka    │   │ Compute  │
 │ schema   │   │ form,    │   │ against  │   │ event to │   │ next     │
 │ check,   │   │ field    │   │ previous │   │ registry │   │ crawl    │
 │ required │   │ cleanup, │   │ version, │   │ pipeline,│   │ time,    │
 │ fields,  │   │ URL      │   │ detect   │   │ metrics, │   │ re-enque │
 │ auth     │   │ resolve  │   │ changes  │   │ alerts   │   │ URL in   │
 │ check    │   │          │   │          │   │          │   │ frontier │
 └──────────┘   └──────────┘   └──────────┘   └──────────┘   └──────────┘
```

**Stage Descriptions:**

**Stage 1: DEQUEUE**
Pull the next URL from the Redis-backed URL frontier. The frontier is a priority queue (Redis Sorted Set) ordered by a composite priority score. Dequeue respects domain-level concurrency limits: no more than N URLs for the same domain can be in-flight simultaneously (default N=2).

**Stage 2: POLITENESS_GATE**
Check the domain's `agent-robots.txt` (falling back to `robots.txt`) for crawl directives. Verify the URL path is allowed. Acquire a rate-limit token from the per-domain token bucket. If the token is not available, the URL is re-enqueued with a delay. Check opt-out flags and respect `Crawl-window` directives.

**Stage 3: DNS_RESOLVE**
Resolve the hostname to IP addresses. Use the local DNS cache (TTL-aware) first, falling back to recursive resolution. Capture both A and AAAA records. Check for DNSSEC validation where available. Store the resolved IP for TLS SNI and for geographic affinity scoring.

**Stage 4: TLS_CONNECT**
Establish a TLS 1.3 connection (falling back to TLS 1.2 if necessary). Capture the full certificate chain for evidence storage. Verify the certificate is valid, not expired, and matches the hostname. Check for Certificate Transparency SCTs. Record the TLS handshake latency for performance scoring.

**Stage 5: HTTP_FETCH**
Send the HTTP GET request with appropriate headers (`User-Agent: AgentBot/1.0`, `Accept: application/json`). Follow redirects (max 5 hops). Capture all response headers. Enforce a maximum response body size (1MB for agent cards). Record TTFB and total transfer time. Use conditional fetching (If-None-Match/If-Modified-Since) when ETags or Last-Modified headers are available from previous crawls.

**Stage 6: VALIDATE**
Parse the response body as JSON. Validate against the A2A Agent Card JSON schema. Check all required fields are present and well-formed. Validate URLs within the card (provider URL, auth URLs). Verify skill definitions conform to expected structure. Run the `validate_agent_card` function (§9.4).

**Stage 7: NORMALIZE**
Convert the validated Agent Card into canonical form. Normalize URLs (lowercase scheme/host, remove default ports, resolve relative paths). Trim whitespace from string fields. Normalize Unicode (NFC normalization). Sort arrays for deterministic hashing. Apply the capability taxonomy mapping where possible.

**Stage 8: DIFF**
Compare the normalized card against the previously stored version (if any). Compute the content diff to determine what changed. Classify the change type: `new` (first time seen), `updated` (fields changed), `unchanged` (content hash matches), `degraded` (fields removed or broken), `recovered` (previously broken, now fixed). The diff uses Blake3 content hashing for efficient change detection.

**Stage 9: EMIT**
If the card is new or changed, emit a `CardDiscovered` or `CardUpdated` event to the Kafka event bus. The event includes the full `AgentCardRecord` with all evidence. Unchanged cards emit a `CardConfirmed` heartbeat event (lower priority topic). Broken cards emit `CardDegraded` events. Update Prometheus metrics counters.

**Stage 10: SCHEDULE**
Compute the next crawl time based on the adaptive scheduling algorithm (§9.6). Re-enqueue the URL in the frontier with the computed priority score and scheduled crawl time. Update the URL's crawl history metadata in Redis.

---

### 9.4 Agent Card Validation

```rust
async fn validate_agent_card(
    raw_json: &[u8],
    source_url: &Url,
    http_metadata: &HttpMetadata,
) -> ValidationResult {
    let mut errors: Vec<ValidationError> = Vec::new();
    let mut warnings: Vec<ValidationWarning> = Vec::new();
    
    // Stage 1: JSON parsing
    let parsed: serde_json::Value = match serde_json::from_slice(raw_json) {
        Ok(v) => v,
        Err(e) => {
            errors.push(ValidationError {
                field: "$root".to_string(),
                error_type: ValidationErrorType::InvalidFormat,
                message: format!("Failed to parse JSON: {}", e),
                severity: Severity::Error,
            });
            return ValidationResult {
                is_valid: false,
                schema_version: "unknown".to_string(),
                errors,
                warnings,
                required_fields_present: RequiredFieldsCheck::all_false(),
                skill_count: 0,
                capability_flags: CapabilityFlags::default(),
                auth_schemes_valid: false,
                url_consistency: UrlConsistencyCheck::all_false(),
                proof_verification: None,
            };
        }
    };
    
    // Stage 2: JSON Schema validation
    let schema = load_a2a_agent_card_schema();
    let schema_errors = jsonschema::validate(&schema, &parsed);
    for schema_error in schema_errors {
        errors.push(ValidationError {
            field: schema_error.instance_path.to_string(),
            error_type: ValidationErrorType::SchemaViolation,
            message: schema_error.to_string(),
            severity: Severity::Error,
        });
    }
    
    // Stage 3: Required field presence checks
    let required_fields = RequiredFieldsCheck {
        name: parsed.get("name").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false),
        description: parsed.get("description").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false),
        url: parsed.get("url").and_then(|v| v.as_str()).map(|s| Url::parse(s).is_ok()).unwrap_or(false),
        version: parsed.get("version").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false),
        skills: parsed.get("skills").and_then(|v| v.as_array()).map(|a| !a.is_empty()).unwrap_or(false),
        provider: parsed.get("provider").is_some(),
    };
    
    if !required_fields.name {
        errors.push(validation_error("name", MissingRequiredField, "Agent name is required"));
    }
    if !required_fields.description {
        errors.push(validation_error("description", MissingRequiredField, "Agent description is required"));
    }
    if !required_fields.url {
        errors.push(validation_error("url", MissingRequiredField, "Agent endpoint URL is required and must be valid"));
    }
    if !required_fields.skills {
        warnings.push(validation_warning("skills", "No skills declared; agent will have limited discoverability"));
    }
    
    // Stage 4: URL consistency checks
    let card_url = parsed.get("url").and_then(|v| v.as_str()).unwrap_or("");
    let url_consistency = UrlConsistencyCheck {
        card_url_matches_source: {
            if let Ok(card_parsed) = Url::parse(card_url) {
                card_parsed.host() == source_url.host()
            } else {
                false
            }
        },
        provider_url_resolves: {
            if let Some(provider_url) = parsed.pointer("/provider/url").and_then(|v| v.as_str()) {
                dns_resolves(provider_url).await
            } else {
                false
            }
        },
        auth_urls_resolve: {
            let mut all_resolve = true;
            if let Some(schemes) = parsed.pointer("/authentication/schemes").and_then(|v| v.as_array()) {
                for scheme in schemes {
                    if let Some(auth_url) = scheme.get("authorizationUrl").and_then(|v| v.as_str()) {
                        if !dns_resolves(auth_url).await {
                            all_resolve = false;
                            warnings.push(validation_warning(
                                "authentication.schemes[].authorizationUrl",
                                &format!("Auth URL does not resolve: {}", auth_url)
                            ));
                        }
                    }
                }
            }
            all_resolve
        },
    };
    
    if !url_consistency.card_url_matches_source {
        warnings.push(validation_warning(
            "url",
            &format!(
                "Agent card URL host ({}) does not match source URL host ({}). Possible misconfiguration or CDN.",
                card_url, source_url
            )
        ));
    }
    
    // Stage 5: Skill validation
    let skill_count = parsed.get("skills")
        .and_then(|v| v.as_array())
        .map(|skills| {
            for (i, skill) in skills.iter().enumerate() {
                let path = format!("skills[{}]", i);
                
                if skill.get("id").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    errors.push(validation_error(&format!("{}.id", path), SkillSchemaInvalid, "Skill ID is required"));
                }
                if skill.get("name").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    errors.push(validation_error(&format!("{}.name", path), SkillSchemaInvalid, "Skill name is required"));
                }
                if skill.get("description").and_then(|v| v.as_str()).map(|s| s.len() < 10).unwrap_or(true) {
                    warnings.push(validation_warning(
                        &format!("{}.description", path),
                        "Skill description is missing or too short; will reduce discoverability"
                    ));
                }
                
                // Check for suspiciously many tags (potential spam signal)
                if let Some(tags) = skill.get("tags").and_then(|v| v.as_array()) {
                    if tags.len() > 20 {
                        warnings.push(validation_warning(
                            &format!("{}.tags", path),
                            &format!("Skill has {} tags; excessive tags may be a spam signal", tags.len())
                        ));
                    }
                }
            }
            skills.len()
        })
        .unwrap_or(0);
    
    // Stage 6: Authentication scheme validation
    let auth_schemes_valid = parsed.pointer("/authentication/schemes")
        .and_then(|v| v.as_array())
        .map(|schemes| {
            let mut valid = true;
            for scheme in schemes {
                let scheme_type = scheme.get("scheme").and_then(|v| v.as_str()).unwrap_or("");
                match scheme_type {
                    "OAuth2" => {
                        if scheme.get("authorizationUrl").is_none() || scheme.get("tokenUrl").is_none() {
                            errors.push(validation_error(
                                "authentication.schemes[].OAuth2",
                                AuthSchemeInvalid,
                                "OAuth2 requires authorizationUrl and tokenUrl"
                            ));
                            valid = false;
                        }
                    },
                    "Bearer" | "APIKey" => { /* minimal validation */ },
                    "mTLS" => { /* check for cert references */ },
                    "" => {
                        errors.push(validation_error(
                            "authentication.schemes[].scheme",
                            AuthSchemeInvalid,
                            "Auth scheme type is required"
                        ));
                        valid = false;
                    },
                    other => {
                        warnings.push(validation_warning(
                            "authentication.schemes[].scheme",
                            &format!("Unknown auth scheme: '{}'; may not be supported by consumers", other)
                        ));
                    }
                }
            }
            valid
        })
        .unwrap_or(true);
    
    // Stage 7: Capability flags
    let capability_flags = CapabilityFlags {
        streaming: parsed.pointer("/capabilities/streaming")
            .and_then(|v| v.as_bool()).unwrap_or(false),
        push_notifications: parsed.pointer("/capabilities/pushNotifications")
            .and_then(|v| v.as_bool()).unwrap_or(false),
        state_transition_history: parsed.pointer("/capabilities/stateTransitionHistory")
            .and_then(|v| v.as_bool()).unwrap_or(false),
    };
    
    // Stage 8: Proof verification (if available)
    let proof_verification = verify_agent_proof(source_url, &parsed).await;
    
    // Stage 9: Content quality heuristics
    if let Some(desc) = parsed.get("description").and_then(|v| v.as_str()) {
        if desc.len() < 20 {
            warnings.push(validation_warning("description", "Description is very short; consider adding more detail"));
        }
        if desc.len() > 5000 {
            warnings.push(validation_warning("description", "Description exceeds 5000 chars; may be truncated in UIs"));
        }
        if desc.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
            warnings.push(validation_warning("description", "Description appears to be ALL CAPS; potential spam signal"));
        }
    }
    
    let is_valid = errors.iter().all(|e| e.severity != Severity::Error) 
                   || errors.is_empty();
    
    ValidationResult {
        is_valid,
        schema_version: "a2a-1.0".to_string(),
        errors,
        warnings,
        required_fields_present: required_fields,
        skill_count,
        capability_flags,
        auth_schemes_valid,
        url_consistency,
        proof_verification,
    }
}
```

---

### 9.5 URL Frontier

The URL frontier is the priority-ordered queue of URLs to be crawled. It determines what gets crawled next and ensures fair, efficient resource allocation across domains.

**Implementation: Redis Sorted Sets**

Each URL is stored as a member in a Redis Sorted Set, with the score representing the composite priority. This provides O(log N) insertion and O(log N) extraction of the highest-priority URL.

```
Redis Key Structure:
  frontier:{partition_id}    → Sorted Set (url → priority_score)
  frontier:metadata:{url}    → Hash (last_crawl, crawl_count, tier, etc.)
  frontier:domain:{domain}   → Set (active urls for domain)
  frontier:inflight          → Hash (url → lease_expiry)
```

**Priority Scoring Formula:**

```
priority(url) = Σ(weight_i × signal_i) for i in [1..6]

Components:
  1. source_confidence      (weight: 0.30)  —  Confidence from discovery vector
  2. expected_change_rate   (weight: 0.20)  —  How likely the card has changed
  3. popularity_demand      (weight: 0.15)  —  How often consumers request this agent
  4. trust_priority         (weight: 0.15)  —  Higher trust = more frequent verification
  5. recrawl_deadline_urgency (weight: 0.10) — Urgency of approaching recrawl deadline
  6. cheap_fetch_probability  (weight: 0.10) — Likelihood of 304 Not Modified
```

**Detailed Signal Computation:**

```
source_confidence(url):
    // Based on how the URL was discovered
    MATCH url.discovery_source:
        "dns-txt-sitemap"       → 0.95
        "agent-sitemap"         → 0.95
        "dns-srv"               → 0.90
        "referral-graph"        → 0.85  × depth_decay(url.referral_depth)
        "registry:agentverse"   → 0.85
        "registry:pulsemcp"     → 0.80
        "registry:github"       → 0.75
        "ct-log-mining"         → 0.70
        "dns-subdomain-probe"   → 0.65
        "common-crawl"          → 0.55
        "community-hint"        → 0.50  × submitter_reputation(url.submitter)
        _                       → 0.40

expected_change_rate(url):
    // Based on historical change frequency
    IF url.crawl_history.len() < 2:
        RETURN 0.50  // Unknown, assume moderate
    
    changes = url.crawl_history.count_changes()
    crawls = url.crawl_history.len()
    change_ratio = changes / crawls
    
    // Weighted by recency: recent changes matter more
    recent_changes = url.crawl_history.last(5).count_changes()
    recent_ratio = recent_changes / min(5, crawls)
    
    RETURN 0.3 × change_ratio + 0.7 × recent_ratio

popularity_demand(url):
    // Based on how often the agent is requested via Discovery API
    requests_last_24h = api_request_counter.get(url.agent_id, "24h")
    requests_last_7d = api_request_counter.get(url.agent_id, "7d")
    
    // Log-scaled to prevent top agents from monopolizing crawl budget
    RETURN min(1.0, log10(1 + requests_last_24h) / 4.0) × 0.6
         + min(1.0, log10(1 + requests_last_7d) / 5.0) × 0.4

trust_priority(url):
    // Higher trust scores should be verified more frequently
    trust_score = registry.get_trust_score(url.agent_id)
    IF trust_score > 0.8:
        RETURN 0.9  // High-trust agents: verify often to maintain confidence
    ELIF trust_score > 0.5:
        RETURN 0.5  // Medium trust: standard frequency
    ELIF trust_score > 0.2:
        RETURN 0.3  // Low trust: less priority
    ELSE:
        RETURN 0.1  // Very low trust: minimal crawl investment

recrawl_deadline_urgency(url):
    time_since_last = now() - url.last_crawl_time
    scheduled_interval = url.scheduled_interval
    
    ratio = time_since_last / scheduled_interval
    IF ratio >= 1.0:
        RETURN 1.0  // Overdue
    ELIF ratio >= 0.8:
        RETURN 0.8  // Approaching deadline
    ELIF ratio >= 0.5:
        RETURN 0.4
    ELSE:
        RETURN 0.1  // Plenty of time

cheap_fetch_probability(url):
    // If the previous crawl returned an ETag or Last-Modified,
    // a conditional GET is likely to return 304, which is cheap.
    IF url.has_etag OR url.has_last_modified:
        RETURN 0.8
    ELSE:
        RETURN 0.2
```

**Cuckoo Filter vs Bloom Filter for URL Deduplication:**

| Property | Bloom Filter | Cuckoo Filter | Winner |
|---|---|---|---|
| **Membership query** | Yes (probabilistic) | Yes (probabilistic) | Tie |
| **Deletion support** | No | Yes | Cuckoo |
| **False positive rate** | ~1% at 9.6 bits/item | ~1% at 8.5 bits/item | Cuckoo |
| **Space efficiency** | Good | Better at same FP rate | Cuckoo |
| **Insertion speed** | O(k) hash operations | O(1) amortized, O(n) worst | Bloom (marginally) |
| **Lookup speed** | O(k) hash operations | O(1) expected | Cuckoo |
| **Load factor** | N/A (append-only) | Up to 95.5% (4-way) | Bloom |
| **Concurrent access** | Lock-free reads | Needs locking for writes | Bloom |
| **Use case fit** | Immutable URL sets | Evolving URL sets (URLs can be removed) | Cuckoo |

**Decision:** Use **Cuckoo filters** for URL deduplication. The ability to delete URLs (when agents are delisted or domains expire) is critical for a discovery system that must maintain a clean frontier over time.

**Memory Budget Breakdown (target: 100M URLs):**

| Component | Per-URL Bytes | Total (100M URLs) | Notes |
|---|---|---|---|
| Cuckoo filter entries | 12 | 1.2 GB | 8-byte fingerprint + 4-byte bucket overhead |
| Redis Sorted Set members | 200 | 20 GB | URL string + score + Redis overhead |
| Metadata hashes | 300 | 30 GB | Last crawl time, tier, ETags, counters |
| Domain rate-limit state | ~50/domain | ~500 MB | ~10M unique domains |
| **Total** | | **~51.7 GB** | Fits in a 64GB Redis instance |

**Optimization Notes:**

- URL compression: Store URLs as 16-byte BLAKE3 hashes in the sorted set, with full URLs in a separate lookup table.
- Redis Cluster: Partition frontier by consistent hash of domain name across 8 Redis shards.
- Hot/cold split: Keep only Hot and Active tier URLs in Redis; Cold and Zombie tiers in ScyllaDB with on-demand promotion.

---

### 9.6 Crawl Scheduling

Crawl scheduling determines how frequently each URL is re-crawled. The system uses an adaptive, tier-based approach that balances freshness against crawl budget.

**Crawl Tier Definitions:**

| Tier | Base Interval | Criteria | Population (est.) | Budget Share |
|---|---|---|---|---|
| **Hot** | 1 hour | High demand, frequent changes, top trust | ~5% of index | 40% |
| **Active** | 6 hours | Moderate demand, periodic changes | ~15% of index | 30% |
| **Stable** | 24 hours | Low demand, infrequent changes | ~40% of index | 20% |
| **Cold** | 72 hours | Minimal demand, no changes detected | ~30% of index | 8% |
| **Zombie** | 168 hours (7 days) → delist | Multiple consecutive failures, no demand | ~10% of index | 2% |

**Tier Assignment Rules:**

```
FUNCTION assign_tier(agent: AgentCrawlProfile) -> CrawlTier:
    // Zombie detection (highest priority)
    IF agent.consecutive_failures >= 5:
        IF agent.consecutive_failures >= 10:
            RETURN Zombie { action: Delist }
        RETURN Zombie { action: ProbeOnly }
    
    // Hot tier qualification
    IF agent.api_requests_24h > 100
       OR agent.change_rate_7d > 0.5
       OR (agent.trust_score > 0.9 AND agent.skill_count > 5):
        RETURN Hot
    
    // Active tier qualification
    IF agent.api_requests_7d > 50
       OR agent.change_rate_30d > 0.2
       OR agent.trust_score > 0.7:
        RETURN Active
    
    // Cold tier detection
    IF agent.api_requests_30d == 0
       AND agent.change_rate_90d == 0.0
       AND agent.days_since_discovery > 30:
        RETURN Cold
    
    // Default: Stable
    RETURN Stable
```

**Adaptive Interval Formula:**

Within each tier, the actual crawl interval is dynamically adjusted based on observed behavior:

```
adaptive_interval(url) = base_interval(tier)
    × change_factor(url)
    × cache_factor(url)
    × error_factor(url)
    × demand_factor(url)

WHERE:
    change_factor(url):
        // Agents that change frequently get shorter intervals
        recent_change_ratio = changes_in_last_5_crawls / 5
        IF recent_change_ratio > 0.8:
            RETURN 0.5   // Halve the interval (crawl twice as often)
        ELIF recent_change_ratio > 0.4:
            RETURN 0.75
        ELIF recent_change_ratio > 0.1:
            RETURN 1.0   // No adjustment
        ELSE:
            RETURN 1.5   // Lengthen interval (crawl less often)

    cache_factor(url):
        // If the agent supports conditional fetching, we can crawl more aggressively
        // since most requests will be cheap 304 responses
        IF url.supports_etag OR url.supports_last_modified:
            RETURN 0.7   // Crawl more often when it's cheap
        ELSE:
            RETURN 1.0

    error_factor(url):
        // Back off on errors
        IF url.last_crawl_error:
            RETURN 2.0 ^ min(url.consecutive_errors, 5)  // Exponential backoff, max 32x
        ELSE:
            RETURN 1.0

    demand_factor(url):
        // Crawl more often when consumers are actively requesting this agent
        requests_24h = api_request_counter.get(url.agent_id, "24h")
        IF requests_24h > 1000:
            RETURN 0.5
        ELIF requests_24h > 100:
            RETURN 0.75
        ELIF requests_24h > 10:
            RETURN 0.9
        ELSE:
            RETURN 1.0

BOUNDS:
    min_interval = 5 minutes   // Hard floor: never crawl more often than every 5 min
    max_interval = 168 hours   // Hard ceiling: at least once a week or delist
    
    final_interval = clamp(adaptive_interval, min_interval, max_interval)
```

**Blake3 Content Hashing for Change Detection:**

```
FUNCTION compute_content_hash(agent_card_json: &[u8]) -> Blake3Hash:
    // 1. Parse JSON
    parsed = serde_json::from_slice(agent_card_json)
    
    // 2. Canonicalize (deterministic serialization)
    // - Sort all object keys alphabetically
    // - Normalize Unicode to NFC
    // - Remove insignificant whitespace
    // - Normalize URLs (lowercase scheme/host)
    canonical = canonicalize_json(parsed)
    
    // 3. Compute BLAKE3 hash
    hash = blake3::hash(canonical.as_bytes())
    
    RETURN hash

FUNCTION detect_change(current_hash: Blake3Hash, previous_hash: Option<Blake3Hash>) -> ChangeType:
    MATCH previous_hash:
        None => ChangeType::New,
        Some(prev) if prev == current_hash => ChangeType::Unchanged,
        Some(prev) => {
            // Detailed diff for analytics
            diff = compute_field_diff(current_card, previous_card)
            IF diff.has_breaking_changes():
                ChangeType::Breaking(diff)
            ELIF diff.has_additions():
                ChangeType::Updated(diff)
            ELIF diff.has_removals():
                ChangeType::Degraded(diff)
            ELSE:
                ChangeType::CosmeticUpdate(diff)
        }
```

---

### 9.7 Failure Handling

The crawler encounters diverse failure modes. Each failure type requires specific handling to balance retry effort against crawl budget.

**Failure Classification Matrix:**

| HTTP Status / Error | Classification | Action | Retry? | Max Retries | Backoff | Delist After |
|---|---|---|---|---|---|---|
| **400 Bad Request** | Client error | Log, check request format | No | 0 | — | 3 consecutive across crawl cycles |
| **401 Unauthorized** | Auth required | Record auth requirement, skip body | No | 0 | — | Never (valid signal) |
| **403 Forbidden** | Access denied | Respect, check robots.txt change | No | 0 | — | 10 consecutive |
| **404 Not Found** | Card removed | Mark as potentially delisted | Yes | 2 | 24h, 72h | 3 consecutive (with 72h gap) |
| **410 Gone** | Permanently removed | Delist immediately | No | 0 | — | Immediate |
| **429 Too Many Requests** | Rate limited | Back off, respect Retry-After | Yes | 5 | Retry-After header or 2^n × 30s | Never |
| **500 Internal Server Error** | Server error | Retry with backoff | Yes | 3 | 1m, 5m, 30m | 10 consecutive |
| **502 Bad Gateway** | Proxy error | Retry with backoff | Yes | 3 | 30s, 2m, 10m | 10 consecutive |
| **503 Service Unavailable** | Temporary down | Retry with backoff, respect Retry-After | Yes | 5 | Retry-After or 1m, 5m, 30m, 2h, 12h | 20 consecutive |
| **504 Gateway Timeout** | Timeout upstream | Retry with backoff | Yes | 3 | 1m, 5m, 30m | 10 consecutive |
| **Connection timeout** | Network issue | Retry with backoff | Yes | 3 | 30s, 2m, 10m | 10 consecutive |
| **TLS handshake error** | TLS config issue | Retry once, then flag | Yes | 1 | 5m | 5 consecutive |
| **DNS resolution failure** | Domain issue | Retry with backoff, check domain status | Yes | 3 | 5m, 30m, 6h | 5 consecutive |
| **Malformed JSON** | Content error | Log, attempt partial parse | No | 0 | — | 5 consecutive |
| **Schema validation fail** | Content error | Index with warnings | No | 0 | — | Never (partial index) |
| **Response too large** | Abuse/misconfiguration | Truncate and validate | No | 0 | — | 3 consecutive |
| **SSL certificate expired** | TLS issue | Log, attempt without verification (NO) | No | 0 | — | 3 consecutive |
| **Connection reset** | Network issue | Retry with backoff | Yes | 3 | 10s, 60s, 5m | 10 consecutive |

**Exponential Backoff Implementation:**

```
FUNCTION compute_backoff(
    attempt: u32,
    base_delay_ms: u64,
    max_delay_ms: u64,
    jitter: bool
) -> Duration:
    // Base exponential: base_delay × 2^attempt
    delay_ms = base_delay_ms × (2_u64.pow(attempt))
    
    // Cap at maximum
    delay_ms = min(delay_ms, max_delay_ms)
    
    // Add jitter to prevent thundering herd
    IF jitter:
        jitter_range = delay_ms / 4  // ±25% jitter
        jitter_offset = random_range(-jitter_range, jitter_range)
        delay_ms = max(base_delay_ms, delay_ms + jitter_offset)
    
    RETURN Duration::from_millis(delay_ms)

// Defaults by failure type:
BACKOFF_CONFIGS = {
    "429":     { base: 30_000,  max: 3_600_000, jitter: true },   // 30s → 1h
    "500":     { base: 60_000,  max: 1_800_000, jitter: true },   // 1m → 30m
    "502/503": { base: 30_000,  max: 43_200_000, jitter: true },  // 30s → 12h
    "timeout": { base: 30_000,  max: 600_000,   jitter: true },   // 30s → 10m
    "dns":     { base: 300_000, max: 21_600_000, jitter: false },  // 5m → 6h
    "tls":     { base: 300_000, max: 3_600_000, jitter: false },   // 5m → 1h
}
```

**Delist Protocol:**

When an agent meets the delist criteria (consecutive failures exceeding threshold), the following process executes:

1. Agent moved to Zombie tier with `ProbeOnly` status
2. Zombie probe: lightweight HEAD request every 168 hours
3. If Zombie probe succeeds: re-promote to Cold tier, reset failure counter
4. If Zombie probe fails 3 consecutive times: emit `AgentDelisted` event
5. Delisted agents removed from active index but retained in historical archive for 1 year
6. Consumer queries against delisted agents return `410 Gone` with `last_seen` timestamp

---

### 9.8 Distributed Crawling

At scale, a single crawler instance cannot meet throughput requirements. The system distributes crawl work across multiple regional crawler nodes.

**Architecture:**

```
                        ┌─────────────────────┐
                        │                     │
                        │  Central Scheduler  │
                        │                     │
                        │  • URL Frontier     │
                        │    (Redis Cluster)  │
                        │                     │
                        │  • Work Assignment  │
                        │                     │
                        │  • Global Rate      │
                        │    Governor         │
                        │                     │
                        │  • Metrics          │
                        │    Aggregation      │
                        │                     │
                        └──────┬──┬──┬────────┘
                               │  │  │
                 ┌─────────────┘  │  └─────────────┐
                 │                │                 │
                 ▼                ▼                 ▼
        ┌────────────┐  ┌────────────┐    ┌────────────┐
        │            │  │            │    │            │
        │ us-east-1  │  │ eu-west-1  │    │ap-south-1  │
        │            │  │            │    │            │
        │┌──────────┐│  │┌──────────┐│    │┌──────────┐│
        ││Crawler   ││  ││Crawler   ││    ││Crawler   ││
        ││Pod 1..N  ││  ││Pod 1..N  ││    ││Pod 1..N  ││
        │├──────────┤│  │├──────────┤│    │├──────────┤│
        ││Local DNS ││  ││Local DNS ││    ││Local DNS ││
        ││Cache     ││  ││Cache     ││    ││Cache     ││
        │├──────────┤│  │├──────────┤│    │├──────────┤│
        ││Local     ││  ││Local     ││    ││Local     ││
        ││Rate      ││  ││Rate      ││    ││Rate      ││
        ││Limiter   ││  ││Limiter   ││    ││Limiter   ││
        │├──────────┤│  │├──────────┤│    │├──────────┤│
        ││Kafka     ││  ││Kafka     ││    ││Kafka     ││
        ││Producer  ││  ││Producer  ││    ││Producer  ││
        │└──────────┘│  │└──────────┘│    │└──────────┘│
        │            │  │            │    │            │
        └────────────┘  └────────────┘    └────────────┘
```

**Frontier Partitioning by Consistent Hashing:**

URLs are assigned to crawler regions using consistent hashing on the domain name:

```
FUNCTION assign_region(url: &Url) -> Region:
    domain = url.domain()
    ring_position = consistent_hash(domain, HASH_RING_VNODES)
    
    // Prefer geographic affinity (US domain → US crawler, etc.)
    preferred_region = geographic_affinity(domain)
    
    IF preferred_region.is_available() AND preferred_region.has_capacity():
        RETURN preferred_region
    
    // Fallback to consistent hash assignment
    RETURN ring_position.assigned_region()
```

**Lease-Based Work Claiming:**

Crawlers claim work from the frontier using time-limited leases to prevent duplicate work:

```
FUNCTION claim_work(crawler_id: CrawlerId, batch_size: u32) -> Vec<CrawlWork>:
    // Atomically pop highest-priority URLs and set lease
    work = REDIS.ZPOPMIN("frontier:{partition}", batch_size)
    
    FOR EACH url IN work:
        lease_key = format!("lease:{}", url)
        REDIS.SET(lease_key, crawler_id, EX=300)  // 5-minute lease
    
    RETURN work

FUNCTION complete_work(crawler_id: CrawlerId, url: &Url, result: CrawlResult):
    lease_key = format!("lease:{}", url)
    
    // Only release if we still hold the lease
    IF REDIS.GET(lease_key) == crawler_id:
        REDIS.DEL(lease_key)
    
    // Re-enqueue with updated priority and scheduled time
    new_priority = compute_priority(url, result)
    new_scheduled = compute_next_crawl(url, result)
    REDIS.ZADD("frontier:{partition}", new_priority, url, NX=false)
    REDIS.HSET("frontier:metadata:{url}", "next_crawl", new_scheduled)

FUNCTION handle_expired_leases():
    // Periodic job: reclaim work from crashed/slow crawlers
    expired = REDIS.SCAN("lease:*", match_expired=true)
    FOR EACH lease IN expired:
        url = lease.url
        REDIS.DEL(lease.key)
        // Re-enqueue with boosted priority (overdue)
        REDIS.ZADD("frontier:{partition}", HIGH_PRIORITY, url)
        metrics.increment("lease_expired_reclaims")
```

**Geographic Affinity Table:**

| Domain Pattern | Preferred Region | Rationale |
|---|---|---|
| `*.us`, `*.com` (US registrar), US cloud platforms | us-east-1 | Minimize latency, respect data locality |
| `*.eu`, `*.de`, `*.fr`, `*.uk`, EU cloud platforms | eu-west-1 | GDPR compliance, latency |
| `*.in`, `*.jp`, `*.cn`, `*.au`, `*.sg` | ap-south-1 | APAC latency optimization |
| `*.io`, `*.dev`, `*.ai` (generic TLDs) | Region with lowest RTT | No geographic signal from TLD |
| Cloud platforms (`*.fly.dev`, `*.vercel.app`) | Region closest to cloud region | Infer from DNS CNAME chain |
| Unknown / ambiguous | Round-robin | Distribute load evenly |

---

### 9.9 Crawler Scale Targets

| Metric | Phase 1 (0-6 months) | Phase 2 (6-18 months) | Phase 3 (18-36 months) |
|---|---|---|---|
| **Agents indexed** | 100,000 | 500,000 | 5,000,000 |
| **URLs probed/day** | 10,000,000 | 50,000,000 | 500,000,000 |
| **Concurrent fetchers** | 10,000 | 50,000 | 200,000 |
| **Crawl throughput** | 500 cards/sec | 2,500 cards/sec | 25,000 cards/sec |
| **Frontier size** | 10M URLs | 50M URLs | 500M URLs |
| **Bandwidth** | 10 Gbps | 50 Gbps | 200 Gbps |
| **Crawler instances** | 10 (1 region) | 50 (3 regions) | 200 (6 regions) |
| **Redis cluster** | 3 nodes, 96GB | 8 nodes, 256GB | 24 nodes, 768GB |
| **CT log ingestion** | 1,000 certs/sec | 5,000 certs/sec | 10,000 certs/sec |
| **Registry sync frequency** | Daily | Hourly | Real-time (webhook) |
| **Discovery latency (new agent)** | < 24 hours | < 4 hours | < 15 minutes |
| **Content freshness (p95)** | 24 hours | 6 hours | 1 hour |

---

### 9.10 Politeness Policy

AgentBot adheres to strict politeness policies to be a good citizen of the agent ecosystem.

**User-Agent String:**

```
AgentBot/1.0 (+https://agentfi.dev/agentbot; agentbot@agentfi.dev)
```

**Opt-Out Mechanisms:**

| Mechanism | Scope | How to Implement | Response Time |
|---|---|---|---|
| `agent-robots.txt` `Disallow: /` | Per-domain | Publish at `/.well-known/agent-robots.txt` | Immediate (next crawl) |
| `robots.txt` `User-agent: AgentBot` `Disallow: /` | Per-domain | Standard robots.txt | Immediate (next crawl) |
| HTTP response header `X-AgentBot: noindex` | Per-endpoint | Add header to agent card response | Immediate |
| DNS TXT record `_agentfi.domain.com TXT "opt-out=all"` | Per-domain | Add DNS TXT record | Next DNS refresh (~1h) |
| Email opt-out | Per-domain or per-agent | Email `agentbot@agentfi.dev` with domain | < 24 hours (manual processing) |
| API opt-out | Per-domain or per-agent | `DELETE /api/v1/opt-out/{domain}` with domain verification | Immediate |
| Agent Card field `"discoverable": false` | Per-agent | Add field to Agent Card JSON | Immediate (next crawl) |

**Rate Limiting Defaults:**

| Scope | Default | Configurable Via |
|---|---|---|
| Per-domain request rate | 1 req/sec | `agent-robots.txt` `Crawl-delay` |
| Per-domain max concurrent | 2 connections | `agent-robots.txt` `Rate-limit` |
| Per-domain daily maximum | 1,000 requests | `agent-robots.txt` `Rate-limit` |
| Global burst rate | 50,000 req/sec | Internal config only |
| Retry-After respect | Always honored | N/A (mandatory) |
| Crawl window | 24/7 | `agent-robots.txt` `Crawl-window` |
| Maximum response size | 1 MB | Internal config only |
| Connection timeout | 10 seconds | Internal config only |
| Total request timeout | 30 seconds | Internal config only |
| Maximum redirects | 5 hops | Internal config only |

---

## 10. Aggregated Registry Design

### Overview

The Aggregated Registry is the canonical, deduplicated, evidence-enriched store of all discovered agents. It is the single source of truth for the discovery system — every query served by the Discovery API (§11), every ranking computed by AgentRank (§12), and every trust assessment made by the Trust & Safety layer (§14) reads from this registry.

The registry is **not** a user-edited database. It is a **computed artifact** derived entirely from automated discovery, crawling, validation, and canonicalization. Human intervention is limited to review queues for edge cases (ambiguous merges, abuse reports) and never involves manual data entry.

---

### 10.1 Registry Purpose

The Aggregated Registry serves three critical functions:

1. **Canonical Identity:** Resolve multiple references to the same agent (across registries, versions, and URLs) into a single canonical entity.
2. **Evidence Accumulation:** Collect and maintain the full evidence trail for every agent — crawl history, validation results, trust signals, performance metrics.
3. **Query-Optimized Storage:** Provide fast, complex queries (semantic search, faceted filtering, multi-signal ranking) with millisecond latency.

**Entity-Relationship Model:**

```
┌─────────────┐       ┌─────────────┐       ┌────────────────┐
│             │       │             │       │                │
│  PROVIDER   │──1:N──│   AGENT     │──1:N──│ AGENT_VERSION  │
│             │       │             │       │                │
│ id          │       │ id          │       │ id             │
│ domain      │       │ provider_id │       │ agent_id       │
│ org_name    │       │ canonical_  │       │ version_string │
│ trust_score │       │   name      │       │ card_hash      │
│ verified    │       │ status      │       │ discovered_at  │
│ created_at  │       │ visibility  │       │ card_snapshot  │
│             │       │ agent_rank  │       │ changes[]      │
└─────────────┘       │ quality_    │       └────────────────┘
                      │   score     │              │
                      └─────────────┘              │
                            │                      │
                            │                      │
                     ┌──────┴──────┐         ┌─────┴──────────┐
                     │             │         │                │
                1:N──│  SKILL      │         │   EVIDENCE     │──N:1
                     │             │         │                │
                     │ id          │         │ id             │
                     │ agent_id    │         │ entity_type    │
                     │ taxonomy_id │         │ entity_id      │
                     │ name        │         │ evidence_type  │
                     │ description │         │ source         │
                     │ tags[]      │         │ payload        │
                     │ examples[]  │         │ collected_at   │
                     └─────────────┘         │ confidence     │
                            │                │ expires_at     │
                            │                └────────────────┘
                     ┌──────┴──────┐
                     │             │
                1:N──│ CAPABILITY  │
                     │             │
                     │ id          │
                     │ agent_id    │
                     │ type        │
                     │ value       │
                     │ verified    │
                     └─────────────┘

Additional relationships:
  AGENT ──N:M── AGENT         (referral_graph edges)
  AGENT ──1:N── CRAWL_RECORD  (crawl history)
  AGENT ──1:N── LIVENESS_CHECK (health probe results)
  PROVIDER ──1:N── DNS_RECORD  (DNS verification records)
```

---

### 10.2 Canonical Entity Model

#### Entity: Provider

```sql
CREATE TABLE providers (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    domain              TEXT NOT NULL UNIQUE,
    organization_name   TEXT,
    organization_url    TEXT,
    contact_email       TEXT,
    lei_code            TEXT,
    
    -- Trust signals
    trust_score         FLOAT NOT NULL DEFAULT 0.0 CHECK (trust_score BETWEEN 0.0 AND 1.0),
    domain_verified     BOOLEAN NOT NULL DEFAULT false,
    org_verified        BOOLEAN NOT NULL DEFAULT false,
    proof_verified      BOOLEAN NOT NULL DEFAULT false,
    verification_level  TEXT NOT NULL DEFAULT 'none' 
                        CHECK (verification_level IN ('none', 'domain', 'organization', 'extended')),
    
    -- Metadata
    agent_count         INTEGER NOT NULL DEFAULT 0,
    first_seen_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_verified_at    TIMESTAMPTZ,
    
    -- DNS evidence
    dns_a_records       JSONB,
    dns_txt_records     JSONB,
    dns_srv_records     JSONB,
    nameservers         TEXT[],
    
    -- TLS evidence
    tls_certificate     JSONB,
    ct_log_entries      JSONB,
    
    -- Registry cross-references
    external_ids        JSONB NOT NULL DEFAULT '{}',
    
    -- Audit
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    -- Indexes
    CONSTRAINT providers_domain_idx UNIQUE (domain)
);

CREATE INDEX idx_providers_trust_score ON providers (trust_score DESC);
CREATE INDEX idx_providers_verification ON providers (verification_level);
CREATE INDEX idx_providers_last_seen ON providers (last_seen_at DESC);
CREATE INDEX idx_providers_org_name_trgm ON providers USING gin (organization_name gin_trgm_ops);
```

#### Entity: Agent

```sql
CREATE TABLE agents (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider_id         UUID NOT NULL REFERENCES providers(id) ON DELETE CASCADE,
    
    -- Identity
    canonical_name      TEXT NOT NULL,
    display_name        TEXT NOT NULL,
    description         TEXT NOT NULL,
    short_description   TEXT,
    
    -- Endpoint
    canonical_url       TEXT NOT NULL,
    card_url            TEXT NOT NULL,
    
    -- Current state
    current_version_id  UUID REFERENCES agent_versions(id),
    current_card_hash   BYTEA NOT NULL,
    
    -- Classification
    primary_category    TEXT,
    taxonomy_ids        TEXT[] NOT NULL DEFAULT '{}',
    tags                TEXT[] NOT NULL DEFAULT '{}',
    
    -- Scoring
    agent_rank          FLOAT NOT NULL DEFAULT 0.0 CHECK (agent_rank BETWEEN 0.0 AND 1.0),
    quality_score       FLOAT NOT NULL DEFAULT 0.0 CHECK (quality_score BETWEEN 0.0 AND 1.0),
    trust_score         FLOAT NOT NULL DEFAULT 0.0 CHECK (trust_score BETWEEN 0.0 AND 1.0),
    popularity_score    FLOAT NOT NULL DEFAULT 0.0 CHECK (popularity_score BETWEEN 0.0 AND 1.0),
    freshness_score     FLOAT NOT NULL DEFAULT 0.0 CHECK (freshness_score BETWEEN 0.0 AND 1.0),
    
    -- Capabilities
    supports_streaming          BOOLEAN NOT NULL DEFAULT false,
    supports_push_notifications BOOLEAN NOT NULL DEFAULT false,
    supports_state_history      BOOLEAN NOT NULL DEFAULT false,
    supported_input_modes       TEXT[] NOT NULL DEFAULT '{"text/plain"}',
    supported_output_modes      TEXT[] NOT NULL DEFAULT '{"text/plain"}',
    
    -- Authentication
    auth_schemes        JSONB NOT NULL DEFAULT '[]',
    requires_auth       BOOLEAN NOT NULL DEFAULT false,
    
    -- Status
    status              TEXT NOT NULL DEFAULT 'active'
                        CHECK (status IN ('active', 'beta', 'deprecated', 'delisted', 'suspended')),
    visibility          TEXT NOT NULL DEFAULT 'public'
                        CHECK (visibility IN ('public', 'private', 'restricted')),
    
    -- Lifecycle
    first_discovered_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_crawled_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_changed_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_liveness_at    TIMESTAMPTZ,
    liveness_status     TEXT DEFAULT 'unknown'
                        CHECK (liveness_status IN ('healthy', 'degraded', 'down', 'unknown')),
    
    -- Crawl metadata
    crawl_tier          TEXT NOT NULL DEFAULT 'stable'
                        CHECK (crawl_tier IN ('hot', 'active', 'stable', 'cold', 'zombie')),
    consecutive_failures INTEGER NOT NULL DEFAULT 0,
    total_crawl_count   INTEGER NOT NULL DEFAULT 0,
    total_change_count  INTEGER NOT NULL DEFAULT 0,
    
    -- Canonicalization
    canonical_cluster_id UUID,
    merge_confidence    FLOAT,
    
    -- Full-text search
    search_vector       TSVECTOR,
    
    -- Embedding for semantic search (pgvector)
    embedding           VECTOR(1536),
    
    -- Audit
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT agents_canonical_url_unique UNIQUE (canonical_url),
    CONSTRAINT agents_card_url_unique UNIQUE (card_url)
);

CREATE INDEX idx_agents_provider ON agents (provider_id);
CREATE INDEX idx_agents_status ON agents (status) WHERE status = 'active';
CREATE INDEX idx_agents_visibility ON agents (visibility);
CREATE INDEX idx_agents_agent_rank ON agents (agent_rank DESC) WHERE status = 'active';
CREATE INDEX idx_agents_quality ON agents (quality_score DESC) WHERE status = 'active';
CREATE INDEX idx_agents_trust ON agents (trust_score DESC);
CREATE INDEX idx_agents_taxonomy ON agents USING gin (taxonomy_ids);
CREATE INDEX idx_agents_tags ON agents USING gin (tags);
CREATE INDEX idx_agents_search ON agents USING gin (search_vector);
CREATE INDEX idx_agents_embedding ON agents USING ivfflat (embedding vector_cosine_ops) WITH (lists = 1000);
CREATE INDEX idx_agents_crawl_tier ON agents (crawl_tier);
CREATE INDEX idx_agents_last_crawled ON agents (last_crawled_at ASC) WHERE status = 'active';
CREATE INDEX idx_agents_auth_schemes ON agents USING gin (auth_schemes);
CREATE INDEX idx_agents_input_modes ON agents USING gin (supported_input_modes);
CREATE INDEX idx_agents_output_modes ON agents USING gin (supported_output_modes);
CREATE INDEX idx_agents_canonical_cluster ON agents (canonical_cluster_id);

CREATE TRIGGER agents_search_vector_update
    BEFORE INSERT OR UPDATE ON agents
    FOR EACH ROW EXECUTE FUNCTION
    tsvector_update_trigger(
        search_vector, 'pg_catalog.english',
        canonical_name, display_name, description, short_description
    );
```

#### Entity: AgentVersion

```sql
CREATE TABLE agent_versions (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id            UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    
    -- Version identity
    version_string      TEXT NOT NULL,
    version_major       INTEGER,
    version_minor       INTEGER,
    version_patch       INTEGER,
    
    -- Content
    card_snapshot       JSONB NOT NULL,
    card_hash           BYTEA NOT NULL,
    canonical_json      JSONB NOT NULL,
    
    -- Diff from previous
    previous_version_id UUID REFERENCES agent_versions(id),
    diff_summary        JSONB,
    change_type         TEXT NOT NULL DEFAULT 'unknown'
                        CHECK (change_type IN ('new', 'updated', 'degraded', 'breaking', 'cosmetic', 'unknown')),
    
    -- Fields changed
    fields_added        TEXT[] NOT NULL DEFAULT '{}',
    fields_removed      TEXT[] NOT NULL DEFAULT '{}',
    fields_modified     TEXT[] NOT NULL DEFAULT '{}',
    skills_added        TEXT[] NOT NULL DEFAULT '{}',
    skills_removed      TEXT[] NOT NULL DEFAULT '{}',
    
    -- Discovery metadata
    discovered_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    source_url          TEXT NOT NULL,
    discovery_vector    TEXT NOT NULL,
    
    -- HTTP evidence at discovery time
    http_status         INTEGER,
    http_headers        JSONB,
    response_time_ms    INTEGER,
    tls_info            JSONB,
    
    -- Validation at discovery time
    validation_result   JSONB NOT NULL,
    is_valid            BOOLEAN NOT NULL DEFAULT false,
    
    -- Audit
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT agent_versions_unique UNIQUE (agent_id, card_hash)
);

CREATE INDEX idx_versions_agent ON agent_versions (agent_id, discovered_at DESC);
CREATE INDEX idx_versions_hash ON agent_versions (card_hash);
CREATE INDEX idx_versions_change_type ON agent_versions (change_type);
CREATE INDEX idx_versions_discovered ON agent_versions (discovered_at DESC);
```

#### Entity: Evidence

```sql
CREATE TABLE evidence (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Polymorphic reference
    entity_type         TEXT NOT NULL CHECK (entity_type IN ('provider', 'agent', 'agent_version')),
    entity_id           UUID NOT NULL,
    
    -- Evidence classification
    evidence_type       TEXT NOT NULL CHECK (evidence_type IN (
        'crawl_result',
        'dns_verification',
        'tls_certificate',
        'ct_log_entry',
        'proof_verification',
        'liveness_probe',
        'registry_cross_reference',
        'community_report',
        'abuse_report',
        'quality_probe',
        'referral_edge',
        'semantic_analysis',
        'performance_measurement'
    )),
    
    -- Source
    source              TEXT NOT NULL,
    source_url          TEXT,
    source_confidence   FLOAT NOT NULL DEFAULT 0.5 CHECK (source_confidence BETWEEN 0.0 AND 1.0),
    
    -- Payload
    payload             JSONB NOT NULL,
    payload_hash        BYTEA NOT NULL,
    
    -- Validity
    collected_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    valid_from          TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at          TIMESTAMPTZ,
    is_current          BOOLEAN NOT NULL DEFAULT true,
    
    -- Confidence
    confidence          FLOAT NOT NULL DEFAULT 0.5 CHECK (confidence BETWEEN 0.0 AND 1.0),
    
    -- Audit
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_evidence_entity ON evidence (entity_type, entity_id);
CREATE INDEX idx_evidence_type ON evidence (evidence_type);
CREATE INDEX idx_evidence_current ON evidence (is_current) WHERE is_current = true;
CREATE INDEX idx_evidence_collected ON evidence (collected_at DESC);
CREATE INDEX idx_evidence_expires ON evidence (expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX idx_evidence_confidence ON evidence (confidence DESC);

CREATE OR REPLACE FUNCTION expire_old_evidence()
RETURNS void AS $$
BEGIN
    UPDATE evidence
    SET is_current = false
    WHERE expires_at < now() AND is_current = true;
END;
$$ LANGUAGE plpgsql;
```

**Additional Supporting Entities:**

```sql
CREATE TABLE skills (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id            UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    
    skill_external_id   TEXT NOT NULL,
    name                TEXT NOT NULL,
    description         TEXT,
    taxonomy_ids        TEXT[] NOT NULL DEFAULT '{}',
    tags                TEXT[] NOT NULL DEFAULT '{}',
    examples            TEXT[] NOT NULL DEFAULT '{}',
    input_modes         TEXT[] NOT NULL DEFAULT '{}',
    output_modes        TEXT[] NOT NULL DEFAULT '{}',
    
    -- Scoring
    relevance_score     FLOAT NOT NULL DEFAULT 0.0,
    
    -- Search
    search_vector       TSVECTOR,
    embedding           VECTOR(1536),
    
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT skills_agent_external_unique UNIQUE (agent_id, skill_external_id)
);

CREATE INDEX idx_skills_agent ON skills (agent_id);
CREATE INDEX idx_skills_taxonomy ON skills USING gin (taxonomy_ids);
CREATE INDEX idx_skills_tags ON skills USING gin (tags);
CREATE INDEX idx_skills_search ON skills USING gin (search_vector);
CREATE INDEX idx_skills_embedding ON skills USING ivfflat (embedding vector_cosine_ops) WITH (lists = 500);

CREATE TABLE capabilities (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id            UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    
    capability_type     TEXT NOT NULL,
    capability_value    JSONB NOT NULL,
    verified            BOOLEAN NOT NULL DEFAULT false,
    verified_at         TIMESTAMPTZ,
    verification_method TEXT,
    
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT capabilities_agent_type_unique UNIQUE (agent_id, capability_type)
);

CREATE INDEX idx_capabilities_agent ON capabilities (agent_id);
CREATE INDEX idx_capabilities_type ON capabilities (capability_type);
CREATE INDEX idx_capabilities_verified ON capabilities (verified) WHERE verified = true;

CREATE TABLE referral_edges (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_agent_id     UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    target_agent_id     UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    
    edge_type           TEXT NOT NULL CHECK (edge_type IN (
        'explicit_reference', 'same_provider', 'shared_auth',
        'delegation', 'semantic_similarity', 'co_mention'
    )),
    weight              FLOAT NOT NULL DEFAULT 1.0,
    confidence          FLOAT NOT NULL DEFAULT 0.5,
    
    evidence_id         UUID REFERENCES evidence(id),
    
    first_seen_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT referral_edges_unique UNIQUE (source_agent_id, target_agent_id, edge_type)
);

CREATE INDEX idx_referral_source ON referral_edges (source_agent_id);
CREATE INDEX idx_referral_target ON referral_edges (target_agent_id);
CREATE INDEX idx_referral_type ON referral_edges (edge_type);

CREATE TABLE crawl_records (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id            UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    
    crawl_url           TEXT NOT NULL,
    crawl_timestamp     TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    http_status         INTEGER,
    response_time_ms    INTEGER,
    content_hash        BYTEA,
    content_changed     BOOLEAN NOT NULL DEFAULT false,
    
    error_type          TEXT,
    error_message       TEXT,
    
    validation_passed   BOOLEAN NOT NULL DEFAULT false,
    
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_crawl_agent ON crawl_records (agent_id, crawl_timestamp DESC);
CREATE INDEX idx_crawl_timestamp ON crawl_records (crawl_timestamp DESC);
CREATE INDEX idx_crawl_errors ON crawl_records (error_type) WHERE error_type IS NOT NULL;
```

---

### 10.3 Canonicalization Strategy

When the same agent appears across multiple sources (its own Agent Card, AgentVerse listing, GitHub repository, npm package), the registry must merge these into a single canonical entity. This is the **entity resolution** problem.

**Match Signal Classification:**

**Strong Signals (high confidence, deterministic matching):**

| Signal | Confidence | Example |
|---|---|---|
| Identical Agent Card URL | 0.99 | Two sources both point to `https://agents.acme.com/fin/.well-known/agent.json` |
| Identical content hash | 0.98 | Same BLAKE3 hash of canonical JSON across two crawl sources |
| Same domain + same agent name | 0.95 | `FinancialAnalysisAgent` at `agents.acme.com` from two registries |
| DNS-verified same domain | 0.95 | Both endpoints resolve to same IP with verified domain ownership |
| Shared `agent-proof.json` signature | 0.97 | Both agent cards signed by same Ed25519 key |
| Explicit `relatedAgents` cross-reference | 0.90 | Agent A's card lists Agent B's URL; Agent B is already canonical |

**Medium Signals (moderate confidence, fuzzy matching):**

| Signal | Confidence | Example |
|---|---|---|
| Same provider domain + similar name (Levenshtein ≤ 3) | 0.75 | `FinAnalysisAgent` vs `FinancialAnalysisAgent` at same domain |
| Same provider + overlapping skills (Jaccard > 0.7) | 0.70 | Both agents declare 5+ identical skill IDs |
| Same GitHub org + similar repo name | 0.65 | `acme/fin-agent` and `acme/financial-analysis-agent` |
| Overlapping description embedding (cosine > 0.92) | 0.60 | Semantic similarity of descriptions |
| Same auth provider + similar endpoint path | 0.55 | Both use `auth.acme.com` OAuth2 with similar path prefixes |

**Weak Signals (low confidence, require human review):**

| Signal | Confidence | Example |
|---|---|---|
| Similar name across different domains | 0.30 | `FinancialAnalysisAgent` at `acme.com` vs `bigcorp.com` |
| Similar description (cosine 0.80–0.92) | 0.25 | Descriptions discuss same capabilities but different wording |
| Same skill tags but different implementation | 0.20 | Both tagged `finance, portfolio` but different providers |
| Same npm package name | 0.40 | Both reference `@a2a/financial-analysis` package |

**Three-Stage Canonicalization Pipeline:**

```
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│                  │     │                  │     │                  │
│   Stage 1:       │     │   Stage 2:       │     │   Stage 3:       │
│   Deterministic  │────▶│   ML-Assisted    │────▶│   Human Review   │
│   Matching       │     │   Matching       │     │   Queue          │
│                  │     │                  │     │                  │
│  • Exact URL     │     │  • Embedding     │     │  • Ambiguous     │
│  • Content hash  │     │    similarity    │     │    merges        │
│  • Domain + name │     │  • Skill overlap │     │  • Cross-domain  │
│  • Proof sig     │     │  • Description   │     │    candidates    │
│                  │     │    matching      │     │  • Disputed      │
│  Confidence:     │     │  • Link analysis │     │    ownership     │
│  > 0.90          │     │                  │     │                  │
│                  │     │  Confidence:     │     │  Confidence:     │
│  Auto-merge      │     │  0.60 – 0.90    │     │  < 0.60          │
│                  │     │                  │     │                  │
│  Volume: ~80%    │     │  Auto-merge with │     │  Manual decision │
│  of candidates   │     │  review flag     │     │                  │
│                  │     │                  │     │  Volume: ~2%     │
│                  │     │  Volume: ~18%    │     │  of candidates   │
│                  │     │  of candidates   │     │                  │
└──────────────────┘     └──────────────────┘     └──────────────────┘
```

**Merge Operation Pseudocode:**

```
FUNCTION merge_agent_records(
    canonical: AgentRecord,
    incoming: AgentRecord,
    match_confidence: f64,
    match_signals: Vec<MatchSignal>
) -> MergeResult:
    
    // 1. Validate merge eligibility
    IF match_confidence < MINIMUM_MERGE_THRESHOLD:
        RETURN MergeResult::Rejected { reason: "Confidence below threshold" }
    
    IF canonical.provider_id != incoming.provider_id AND match_confidence < 0.95:
        RETURN MergeResult::NeedsReview { 
            reason: "Cross-provider merge requires high confidence",
            review_queue: "cross-provider"
        }
    
    // 2. Determine field precedence
    // Rule: First-party Agent Card > agent-sitemap > third-party registry > inferred
    precedence = [
        ("agent-card", 1.0),    // Highest precedence
        ("agent-sitemap", 0.9),
        ("agent-proof", 0.95),
        ("registry:agentverse", 0.7),
        ("registry:github", 0.6),
        ("registry:npm", 0.5),
        ("ct-log", 0.4),
        ("inferred", 0.3),      // Lowest precedence
    ]
    
    // 3. Merge fields with precedence
    merged = AgentRecord::new()
    
    // Identity: prefer canonical (already established)
    merged.canonical_name = canonical.canonical_name
    merged.canonical_url = canonical.canonical_url
    
    // Description: prefer the longer, more detailed one from highest-precedence source
    IF incoming.description.len() > canonical.description.len() 
       AND source_precedence(incoming) >= source_precedence(canonical):
        merged.description = incoming.description
    ELSE:
        merged.description = canonical.description
    
    // Skills: union of skills, deduplicated by skill ID
    merged.skills = union_skills(canonical.skills, incoming.skills)
    
    // Capabilities: OR of all capability flags (if any source says true, it's true)
    merged.supports_streaming = canonical.supports_streaming OR incoming.supports_streaming
    merged.supports_push = canonical.supports_push OR incoming.supports_push
    merged.supports_history = canonical.supports_history OR incoming.supports_history
    
    // Auth schemes: union
    merged.auth_schemes = union_auth_schemes(canonical.auth_schemes, incoming.auth_schemes)
    
    // Tags: union, deduplicated
    merged.tags = canonical.tags.union(incoming.tags).deduplicate()
    
    // Scores: keep canonical scores (recalculated separately by AgentRank)
    merged.agent_rank = canonical.agent_rank
    merged.quality_score = canonical.quality_score
    merged.trust_score = canonical.trust_score
    
    // 4. Record the merge as evidence
    evidence = Evidence {
        entity_type: "agent",
        entity_id: canonical.id,
        evidence_type: "canonicalization_merge",
        source: incoming.discovery_source,
        payload: {
            "incoming_id": incoming.id,
            "match_confidence": match_confidence,
            "match_signals": match_signals,
            "fields_updated": diff(canonical, merged),
            "merge_strategy": "precedence-based"
        },
        confidence: match_confidence,
        collected_at: now(),
    }
    
    // 5. Update cross-reference map
    cross_ref = CrossReference {
        canonical_id: canonical.id,
        source_id: incoming.id,
        source_registry: incoming.discovery_source,
        source_url: incoming.source_url,
        confidence: match_confidence,
    }
    
    RETURN MergeResult::Success {
        merged_record: merged,
        evidence: evidence,
        cross_reference: cross_ref,
        fields_updated: diff(canonical, merged).field_names(),
        needs_reindex: diff(canonical, merged).has_searchable_changes(),
    }
```

---

### 10.4 Visibility Model

The registry supports three visibility tiers that control how agents appear in search results and API responses.

**Visibility Tiers:**

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Visibility Model                             │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                                                             │    │
│  │  Tier 1: PUBLIC                                             │    │
│  │                                                             │    │
│  │  • Visible to all API consumers                             │    │
│  │  • Indexed in search results                                │    │
│  │  • Included in rankings and recommendations                 │    │
│  │  • Agent Card served from public well-known endpoint        │    │
│  │  • No access restrictions on discovery metadata             │    │
│  │                                                             │    │
│  │  Access: Any authenticated or anonymous API consumer        │    │
│  │  Default for: agents without explicit visibility setting    │    │
│  │                                                             │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                                                             │    │
│  │  Tier 2: PRIVATE (Tenant-Scoped)                            │    │
│  │                                                             │    │
│  │  • Visible only to the owning organization/tenant           │    │
│  │  • NOT indexed in public search results                     │    │
│  │  • NOT included in public rankings                          │    │
│  │  • Accessible via tenant-scoped API with auth               │    │
│  │  • May appear in organization-internal discovery             │    │
│  │                                                             │    │
│  │  Access: Authenticated users belonging to the same          │    │
│  │          organization/tenant as the agent's provider        │    │
│  │  Use case: Internal corporate agents, staging agents,       │    │
│  │            pre-release agents                                │    │
│  │                                                             │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                                                             │    │
│  │  Tier 3: RESTRICTED (Partner/Federated)                     │    │
│  │                                                             │    │
│  │  • Visible only to approved partners/federations            │    │
│  │  • NOT indexed in public search results                     │    │
│  │  • Available through partner-specific API endpoints         │    │
│  │  • May appear in federated discovery across partner orgs    │    │
│  │  • Requires explicit access grant (ACL-based)               │    │
│  │                                                             │    │
│  │  Access: Authenticated users with explicit partner access   │    │
│  │          grant from the agent's provider                     │    │
│  │  Use case: B2B agents, compliance-restricted agents,        │    │
│  │            agents under NDA, beta partner programs           │    │
│  │                                                             │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Visibility Transition Rules:**

| From → To | Allowed? | Trigger | Side Effects |
|---|---|---|---|
| Public → Private | Yes | Provider sets `"visibility": "private"` in Agent Card, or sets `"discoverable": false` | Removed from public index within 1 crawl cycle; historical data retained |
| Public → Restricted | Yes | Provider requests via API or sets visibility in card | Removed from public index; added to partner access list |
| Private → Public | Yes | Provider removes visibility restriction from Agent Card | Agent re-enters public indexing pipeline; fresh crawl initiated |
| Private → Restricted | Yes | Provider grants partner access | Added to partner access lists |
| Restricted → Public | Yes | Provider removes restrictions | Agent re-enters public indexing pipeline |
| Restricted → Private | Yes | Provider revokes partner access, keeps private | Removed from partner access lists |
| Any → Suspended | Yes (admin only) | Trust & Safety action (abuse, spam, fraud) | Removed from all indexes; card returns 451 status in API |
| Suspended → Any | Yes (admin review) | Appeal reviewed and approved | Restored to previous visibility tier |
| Any → Delisted | Yes (system) | Agent endpoint unreachable for extended period | Removed from active index; moved to archive |
| Delisted → Active | Yes (system) | Agent endpoint becomes reachable again | Re-enters crawl pipeline; promoted from Zombie tier |

**Implementation Notes:**

- Visibility is enforced at the **query layer**, not the storage layer. All agents exist in the same database; visibility filtering happens in SQL queries and API middleware.
- Visibility changes propagate through the system within one crawl cycle (worst case: the agent's crawl tier interval).
- Private agents are still **crawled** (to track changes and maintain evidence), but their data is filtered from public API responses.
- Restricted agents require an explicit `GRANT` record in an `access_grants` table linking the agent to authorized consumer organizations.

```sql
CREATE TABLE access_grants (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id            UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    grantee_org_id      UUID NOT NULL,
    granted_by          UUID NOT NULL,
    
    grant_type          TEXT NOT NULL CHECK (grant_type IN ('partner', 'federation', 'explicit')),
    permissions         TEXT[] NOT NULL DEFAULT '{"read"}',
    
    valid_from          TIMESTAMPTZ NOT NULL DEFAULT now(),
    valid_until         TIMESTAMPTZ,
    is_active           BOOLEAN NOT NULL DEFAULT true,
    
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at          TIMESTAMPTZ,
    revocation_reason   TEXT,
    
    CONSTRAINT access_grants_unique UNIQUE (agent_id, grantee_org_id, grant_type)
);

CREATE INDEX idx_grants_agent ON access_grants (agent_id) WHERE is_active = true;
CREATE INDEX idx_grants_grantee ON access_grants (grantee_org_id) WHERE is_active = true;
CREATE INDEX idx_grants_valid ON access_grants (valid_until) WHERE valid_until IS NOT NULL;
```

**Query-Level Visibility Enforcement:**

```sql
CREATE OR REPLACE FUNCTION visible_agents(
    p_consumer_org_id UUID DEFAULT NULL,
    p_include_private BOOLEAN DEFAULT false,
    p_include_restricted BOOLEAN DEFAULT false
)
RETURNS TABLE (agent_id UUID) AS $$
BEGIN
    RETURN QUERY
    SELECT a.id
    FROM agents a
    WHERE a.status = 'active'
      AND (
          -- Public agents: always visible
          a.visibility = 'public'
          
          -- Private agents: only visible to the owning organization
          OR (p_include_private AND a.visibility = 'private' AND EXISTS (
              SELECT 1 FROM providers p
              WHERE p.id = a.provider_id
                AND p.id IN (
                    SELECT org_provider_id 
                    FROM organization_members 
                    WHERE org_id = p_consumer_org_id
                )
          ))
          
          -- Restricted agents: visible to orgs with explicit grants
          OR (p_include_restricted AND a.visibility = 'restricted' AND EXISTS (
              SELECT 1 FROM access_grants ag
              WHERE ag.agent_id = a.id
                AND ag.grantee_org_id = p_consumer_org_id
                AND ag.is_active = true
                AND (ag.valid_until IS NULL OR ag.valid_until > now())
          ))
      );
END;
$$ LANGUAGE plpgsql STABLE;
```

---

*End of Part 2: Protocol Foundation, Discovery Engine, and Aggregated Registry*
# Part IV: Search, Ranking, and Trust — The Intelligence Layer

---

## 11. Search and Ranking Architecture

Search is the product surface that users touch. Ranking is the product surface that users _feel_. Together, they determine whether AgentRank is perceived as a smart recommendation engine or a dumb list.

This section specifies the complete retrieval architecture: from raw query bytes arriving at the API gateway to the final ranked, diversified, explained result set returned to the caller. The design borrows heavily from mature information retrieval systems (Google, Bing, Spotify, Airbnb, LinkedIn) but adapts every stage to the unique characteristics of agent discovery: small corpus relative to the web, high dimensionality per document, strong trust requirements, and the need for both programmatic and human-facing result quality.

### 11.1 Retrieval Philosophy: The 6-Stage Pipeline

The retrieval pipeline is a narrowing funnel. Each stage trades breadth for precision, operating on a progressively smaller candidate set with progressively more expensive scoring functions. The key insight is that no single retrieval method is sufficient — lexical retrieval misses semantic matches, semantic retrieval misses exact keyword matches, graph retrieval misses agents with no connections — so we run multiple retrieval modes in parallel and fuse their results before scoring.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     THE 6-STAGE RETRIEVAL PIPELINE                          │
│                                                                             │
│   ┌─────────────────────┐                                                   │
│   │  Stage 1: Candidate │   ~1,000 candidates                              │
│   │  Generation          │   3 parallel modes: Lexical + Semantic + Graph   │
│   │  (BM25 + kNN + BFS) │   Budget: 8ms per mode (parallel)                │
│   └─────────┬───────────┘                                                   │
│             │                                                               │
│             ▼  1000 → 400                                                   │
│   ┌─────────────────────┐                                                   │
│   │  Stage 2: Eligibility│   Hard filters: protocol, auth, region,          │
│   │  Filtering           │   modality, tenant ACLs, policy, trust,          │
│   │  (Boolean Masks)     │   price ceiling, latency SLA                     │
│   │                      │   Budget: 1ms (bit-vector intersection)          │
│   └─────────┬───────────┘                                                   │
│             │                                                               │
│             ▼  400 → 100                                                    │
│   ┌─────────────────────┐                                                   │
│   │  Stage 3: Lightweight│   RRF fusion + static quality signals            │
│   │  Scoring             │   AgentRank precomputed, trust floor,            │
│   │  (RRF + Heuristics)  │   freshness decay                               │
│   │                      │   Budget: 2ms                                    │
│   └─────────┬───────────┘                                                   │
│             │                                                               │
│             ▼  100 → 50                                                     │
│   ┌─────────────────────┐                                                   │
│   │  Stage 4: Heavy      │   Cross-encoder re-ranking                       │
│   │  Re-Ranking          │   (query, agent_text) → relevance score          │
│   │  (Cross-Encoder)     │   GPU-batched inference                          │
│   │                      │   Budget: 15ms (batched)                         │
│   └─────────┬───────────┘                                                   │
│             │                                                               │
│             ▼  50 → 20                                                      │
│   ┌─────────────────────┐                                                   │
│   │  Stage 5: Diversity  │   Provider dedup, taxonomy spread,               │
│   │  & Policy Enforcement│   MMR for result diversity,                      │
│   │  (MMR + Rules)       │   enterprise policy enforcement                  │
│   │                      │   Budget: 2ms                                    │
│   └─────────┬───────────┘                                                   │
│             │                                                               │
│             ▼  20 → 10                                                      │
│   ┌─────────────────────┐                                                   │
│   │  Stage 6: Connection │   Auth hint resolution,                          │
│   │  Hints & Explanation │   endpoint health snapshot,                      │
│   │  (Enrichment)        │   ranking explanation payload,                   │
│   │                      │   connection recommendation                      │
│   │                      │   Budget: 5ms (parallel enrichment)              │
│   └─────────────────────┘                                                   │
│                                                                             │
│   Total latency budget: 33ms (P50), 50ms (P95), 100ms (P99)                │
│   Candidate flow: 1000 → 400 → 100 → 50 → 20 → 10                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Why these specific counts?**

| Stage | Input | Output | Reduction | Rationale |
|-------|-------|--------|-----------|-----------|
| Candidate Generation | Full index (~10M) | ~1,000 | 10,000:1 | ANN and BM25 top-k limits; diminishing marginal recall beyond 1K |
| Eligibility Filtering | ~1,000 | ~400 | 2.5:1 | Hard constraint removal; varies by query restrictiveness |
| Lightweight Scoring | ~400 | ~100 | 4:1 | RRF fusion + static quality; cheap enough for 400 candidates |
| Heavy Re-ranking | ~100 | ~50 | 2:1 | Cross-encoder inference is O(n) GPU; 100 is the latency ceiling |
| Diversity & Policy | ~50 | ~20 | 2.5:1 | Provider dedup and taxonomy spread |
| Connection Hints | ~20 | ~10 | 2:1 | Final page size; enrichment cost per result is high |

**Why 6 stages and not fewer?**

Collapsing stages creates two failure modes:

1. **Accuracy degradation**: Running a cross-encoder on 1,000 candidates would produce better relevance scores but would blow the latency budget (1,000 × 3ms = 3,000ms). The staged approach ensures each scoring function operates within its computational budget.

2. **Policy leakage**: Applying diversity rules before eligibility filtering could promote an ineligible agent that then gets removed, leaving a diversity gap. The strict ordering ensures correctness.

**Why not 7 or 8 stages?**

Every stage adds latency overhead (context switching, memory allocation, intermediate serialization). Six stages represent the minimum viable decomposition that separates all orthogonal concerns. In the LTR phase (Section 12.11), stages 3-5 may be partially collapsed into a single learned ranker, but the logical separation remains.

---

### 11.2 Candidate Generation (3 Parallel Modes)

Candidate generation is the only stage that touches the full index. It must be fast (sub-10ms), high-recall (miss nothing obviously relevant), and fault-tolerant (degrade gracefully if one mode fails). The three modes run in parallel and their results are fused via Reciprocal Rank Fusion (Section 11.4).

```
┌───────────────────────────────────────────────────────────────────┐
│                  CANDIDATE GENERATION (PARALLEL)                   │
│                                                                    │
│   Query: "kubernetes deployment agent with OAuth support"          │
│                                                                    │
│   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│   │   Lexical     │  │   Semantic    │  │   Graph      │            │
│   │   (Tantivy)   │  │   (Qdrant)    │  │   (Neo4j)    │            │
│   │              │  │              │  │              │            │
│   │  BM25 over   │  │  Multi-vector │  │  BFS from    │            │
│   │  9 fields    │  │  kNN over 3   │  │  seed nodes  │            │
│   │  with boosts │  │  embedding    │  │  with 5 edge │            │
│   │              │  │  spaces       │  │  types       │            │
│   │  top-500     │  │  top-500      │  │  top-100     │            │
│   └──────┬───────┘  └──────┬───────┘  └──────┬───────┘            │
│          │                 │                 │                     │
│          └────────────┬────┴─────────────────┘                     │
│                       │                                            │
│                       ▼                                            │
│            ┌──────────────────┐                                     │
│            │   RRF Fusion     │                                     │
│            │   k = 60         │                                     │
│            │   Output: ~1,000 │                                     │
│            │   unique agents  │                                     │
│            └──────────────────┘                                     │
│                                                                    │
│   Fallback: If any mode times out (>12ms), proceed with            │
│   available results. Minimum: 1 mode must succeed.                 │
└───────────────────────────────────────────────────────────────────┘
```

#### 11.2.1 Lexical Retrieval (BM25 via Tantivy)

**Why Tantivy?**

Tantivy is a Rust-native full-text search library modeled after Apache Lucene. It provides:

- Sub-millisecond BM25 scoring on millions of documents
- Custom tokenizers and analyzers without JVM overhead
- Thread-safe concurrent readers with near-lock-free segment architecture
- Memory-mapped I/O for large indices with minimal heap pressure
- First-class integration with Rust services (the search service is Rust)

Elasticsearch/OpenSearch were considered and rejected: the operational overhead of a distributed JVM cluster is not justified at our initial scale (sub-10M agents). Tantivy runs as an embedded library within the search service process, eliminating network hops for lexical retrieval.

**Index Schema**

Every agent in the canonical registry is indexed with the following fields:

| Field | Tantivy Type | Boost | Source | Notes |
|-------|-------------|-------|--------|-------|
| `agent_id` | `STRING` (stored, indexed) | — | Registry primary key | UUID v7, used for dedup and joins |
| `name` | `TEXT` (stored, indexed) | **3.0×** | `agent_card.name` | Agent's display name; highest boost because exact name matches are almost always relevant |
| `skills_names` | `TEXT` (stored, indexed) | **2.5×** | `agent_card.skills[].name` | Concatenated skill names; high boost because skill names are the primary capability signal |
| `skills_tags` | `TEXT` (stored, indexed) | **2.0×** | `agent_card.skills[].tags[]` | Flattened tag array; moderate boost for taxonomy-aligned queries |
| `description` | `TEXT` (stored, indexed) | **1.5×** | `agent_card.description` | Free-text agent description; moderate boost for narrative matches |
| `skills_descriptions` | `TEXT` (stored, indexed) | **1.0×** | `agent_card.skills[].description` | Concatenated skill descriptions; base boost for detailed capability text |
| `examples` | `TEXT` (stored, indexed) | **1.0×** | `agent_card.skills[].examples[]` | Example queries and use cases; base boost for intent matching |
| `provider_org` | `TEXT` (stored, indexed) | **1.5×** | `registry.provider.organization` | Provider organization name; moderate boost for brand/company queries |
| `taxonomy_labels` | `TEXT` (stored, indexed) | **2.0×** | `registry.taxonomy_labels[]` | Platform-assigned taxonomy categories; moderate boost for structured classification queries |
| `endpoint_url` | `STRING` (stored, not full-text indexed) | — | `agent_card.url` | Stored for result enrichment, not searched |
| `agent_rank` | `F64` (stored, fast-field) | — | Precomputed AgentRank score | Used in Stage 3 lightweight scoring |
| `trust_tier` | `U64` (stored, fast-field) | — | Registry trust tier ordinal | Used in eligibility filtering |
| `last_seen_healthy` | `DATE` (stored, fast-field) | — | Liveness system timestamp | Used in freshness calculations |
| `protocols` | `BYTES` (stored, fast-field) | — | Bit-packed protocol support flags | Used in eligibility filtering |
| `auth_schemes` | `BYTES` (stored, fast-field) | — | Bit-packed auth scheme flags | Used in eligibility filtering |
| `regions` | `BYTES` (stored, fast-field) | — | Bit-packed region flags | Used in eligibility filtering |
| `visibility` | `U64` (stored, fast-field) | — | Visibility scope enum | Used in tenant-scoped queries |
| `tenant_ids` | `BYTES` (stored, fast-field) | — | Bit-set of authorized tenant IDs | Used in enterprise access control |

**Boost Rationale**

The boost values follow a principle: **the more specific a field is to what the agent _does_, the higher its boost.** Agent name (3.0×) is the strongest signal because if a user types a name, they want that exact agent. Skill names (2.5×) and tags (2.0×) are the primary capability identifiers. Description (1.5×) is a weaker signal because descriptions are often marketing copy with high term overlap. Examples (1.0×) are useful for intent matching but often contain noise.

These boosts are tunable via A/B experiments (Section 26 in the full document) and will likely evolve as we collect click-through data.

**Text Analysis Pipeline (7 Stages)**

Every text field passes through the following analysis pipeline before indexing and at query time:

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    TEXT ANALYSIS PIPELINE (7 STAGES)                      │
│                                                                          │
│   Input: "Kubernetes   Déployment Agent — supports OAuth2.0 & mTLS"     │
│                                                                          │
│   ┌──────────────┐                                                       │
│   │ 1. Unicode    │  NFC normalization                                   │
│   │    Normalize  │  "Kubernetes   Déployment Agent — supports ..."      │
│   │    (NFC)      │  → canonical Unicode form, decomposed then composed  │
│   └──────┬───────┘                                                       │
│          │                                                               │
│          ▼                                                               │
│   ┌──────────────┐                                                       │
│   │ 2. Lowercase  │  ASCII + Unicode-aware lowering                      │
│   │              │  → "kubernetes   déployment agent — supports ..."     │
│   └──────┬───────┘                                                       │
│          │                                                               │
│          ▼                                                               │
│   ┌──────────────┐                                                       │
│   │ 3. ICU        │  ICU word-break rules (UAX #29)                      │
│   │    Tokenize   │  → ["kubernetes", "déployment", "agent",             │
│   │              │     "supports", "oauth2.0", "mtls"]                  │
│   │              │  Note: preserves "oauth2.0" as single token,         │
│   │              │  strips punctuation-only tokens                       │
│   └──────┬───────┘                                                       │
│          │                                                               │
│          ▼                                                               │
│   ┌──────────────┐                                                       │
│   │ 4. Stop Word  │  English stop words (NLTK list, ~179 words)          │
│   │    Removal    │  + domain-specific keeps: "agent", "api", "model"   │
│   │              │  → ["kubernetes", "déployment", "agent",             │
│   │              │     "supports", "oauth2.0", "mtls"]                  │
│   │              │  ("agent" kept because it's domain-significant)       │
│   └──────┬───────┘                                                       │
│          │                                                               │
│          ▼                                                               │
│   ┌──────────────┐                                                       │
│   │ 5. Snowball   │  English Snowball stemmer                            │
│   │    Stemming   │  → ["kubernetes", "deploy", "agent",                │
│   │              │     "support", "oauth2.0", "mtls"]                   │
│   │              │  Note: "déployment" → stripped accent → "deploy"     │
│   └──────┬───────┘                                                       │
│          │                                                               │
│          ▼                                                               │
│   ┌──────────────┐                                                       │
│   │ 6. Synonym    │  Expansion from curated synonym map                  │
│   │    Expansion  │  → "kubernetes" also indexes as "k8s"               │
│   │              │  → "oauth2.0" also indexes as "oauth", "oauth2"     │
│   │              │  → "mtls" also indexes as                            │
│   │              │     "mutual tls", "mutual_tls"                       │
│   └──────┬───────┘                                                       │
│          │                                                               │
│          ▼                                                               │
│   ┌──────────────┐                                                       │
│   │ 7. n-gram     │  Character n-grams (2-4) for fuzzy matching          │
│   │    (2-4)      │  → "ku", "kub", "kube", "ub", "ube", "uber", ...   │
│   │              │  Applied to a separate n-gram field for fallback     │
│   │              │  matching, not the primary BM25 fields               │
│   └──────────────┘                                                       │
│                                                                          │
│   Final indexed terms (primary): ["kubernetes", "k8s", "deploy",         │
│     "agent", "support", "oauth2.0", "oauth", "oauth2", "mtls",          │
│     "mutual tls", "mutual_tls"]                                         │
└──────────────────────────────────────────────────────────────────────────┘
```

**Synonym Map (Curated)**

The synonym map is critical for agent discovery because the agent ecosystem uses abbreviations, brand names, and technical jargon interchangeably. A manually curated synonym map is maintained and expanded weekly.

| Canonical Term | Synonyms |
|---------------|----------|
| `kubernetes` | `k8s`, `kube` |
| `large language model` | `llm`, `LLM` |
| `natural language processing` | `nlp`, `NLP` |
| `machine learning` | `ml`, `ML` |
| `artificial intelligence` | `ai`, `AI` |
| `oauth` | `oauth2`, `oauth2.0`, `openid connect`, `oidc` |
| `mutual tls` | `mtls`, `mTLS`, `mutual_tls` |
| `model context protocol` | `mcp`, `MCP` |
| `agent to agent` | `a2a`, `A2A` |
| `retrieval augmented generation` | `rag`, `RAG` |
| `continuous integration` | `ci`, `CI`, `ci/cd`, `CI/CD` |
| `continuous deployment` | `cd`, `CD` |
| `amazon web services` | `aws`, `AWS` |
| `google cloud platform` | `gcp`, `GCP` |
| `microsoft azure` | `azure` |
| `application programming interface` | `api`, `API`, `rest`, `REST`, `graphql`, `GraphQL` |
| `database` | `db`, `DB`, `datastore` |
| `postgresql` | `postgres`, `pg` |
| `elasticsearch` | `elastic`, `es`, `opensearch` |
| `tensorflow` | `tf` |
| `pytorch` | `torch` |
| `optical character recognition` | `ocr`, `OCR` |
| `text to speech` | `tts`, `TTS` |
| `speech to text` | `stt`, `STT`, `asr`, `ASR` |

Synonyms are applied at **index time** (multi-term expansion into the posting list) rather than **query time** (query rewriting) to avoid query latency overhead. The trade-off is larger index size (~15% increase) and the need to re-index when the synonym map changes.

**BM25 Scoring Formula**

BM25 (Best Matching 25) is the lexical relevance scoring function. The formula for a query `Q` containing terms `q₁, q₂, ..., qₙ` against document `D`:

```
BM25(D, Q) = Σᵢ IDF(qᵢ) × [ f(qᵢ, D) × (k₁ + 1) ] / [ f(qᵢ, D) + k₁ × (1 - b + b × |D|/avgdl) ]

Where:
  IDF(qᵢ) = ln( (N - n(qᵢ) + 0.5) / (n(qᵢ) + 0.5) + 1 )
  f(qᵢ, D)  = term frequency of qᵢ in document D
  |D|        = document length (in tokens)
  avgdl      = average document length across the index
  N          = total number of documents in the index
  n(qᵢ)     = number of documents containing term qᵢ
  k₁         = 1.2   (term frequency saturation parameter)
  b          = 0.75  (document length normalization parameter)
```

**Parameter choices:**

- **k₁ = 1.2**: Standard value. Controls how quickly term frequency saturates. At k₁=1.2, a term appearing 3 times in a document scores only ~2.3× a term appearing once (not 3×). This prevents keyword-stuffed agent descriptions from dominating. Values tested: k₁ ∈ {0.8, 1.0, 1.2, 1.5, 2.0}. k₁=1.2 produced the best NDCG@10 on our internal evaluation set.

- **b = 0.75**: Standard value. Controls document length normalization. At b=0.75, a document twice the average length needs proportionally more term occurrences to achieve the same score. This prevents verbose agent descriptions from dominating over concise, focused ones. Without length normalization (b=0), agents with long marketing descriptions would unfairly outrank agents with precise, technical descriptions.

**Multi-field BM25 with Boosts**

The final lexical score is a weighted sum across all text fields:

```
BM25_total(D, Q) = Σⱼ boost(fieldⱼ) × BM25(D.fieldⱼ, Q)

                 = 3.0 × BM25(name, Q)
                 + 2.5 × BM25(skills_names, Q)
                 + 2.0 × BM25(skills_tags, Q)
                 + 2.0 × BM25(taxonomy_labels, Q)
                 + 1.5 × BM25(description, Q)
                 + 1.5 × BM25(provider_org, Q)
                 + 1.0 × BM25(skills_descriptions, Q)
                 + 1.0 × BM25(examples, Q)
```

The Tantivy query is constructed as a `BooleanQuery` with `SHOULD` clauses for each field, with per-field boosts applied via Tantivy's `BoostQuery` wrapper. The top-500 results are returned by BM25 score.

**Tantivy Index Configuration (Rust)**

```rust
use tantivy::schema::*;
use tantivy::tokenizer::*;
use tantivy::{Index, IndexWriter};

fn build_agent_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    let text_options = TextOptions::default()
        .set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("agent_tokenizer")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        )
        .set_stored();

    schema_builder.add_text_field("agent_id", STRING | STORED);
    schema_builder.add_text_field("name", text_options.clone());
    schema_builder.add_text_field("skills_names", text_options.clone());
    schema_builder.add_text_field("skills_tags", text_options.clone());
    schema_builder.add_text_field("description", text_options.clone());
    schema_builder.add_text_field("skills_descriptions", text_options.clone());
    schema_builder.add_text_field("examples", text_options.clone());
    schema_builder.add_text_field("provider_org", text_options.clone());
    schema_builder.add_text_field("taxonomy_labels", text_options.clone());
    schema_builder.add_text_field("endpoint_url", STRING | STORED);

    schema_builder.add_f64_field("agent_rank", FAST | STORED);
    schema_builder.add_u64_field("trust_tier", FAST | STORED);
    schema_builder.add_date_field("last_seen_healthy", FAST | STORED);
    schema_builder.add_bytes_field("protocols", FAST | STORED);
    schema_builder.add_bytes_field("auth_schemes", FAST | STORED);
    schema_builder.add_bytes_field("regions", FAST | STORED);
    schema_builder.add_u64_field("visibility", FAST | STORED);
    schema_builder.add_bytes_field("tenant_ids", FAST | STORED);

    schema_builder.build()
}

fn register_agent_tokenizer(index: &Index) {
    let tokenizer = TextAnalyzer::builder(ICUTokenizer)
        .filter(LowerCaser)
        .filter(StopWordFilter::remove(stop_words::ENGLISH))
        .filter(Stemmer::new(Language::English))
        .filter(SynonymFilter::new(load_synonym_map()))
        .build();

    index
        .tokenizers()
        .register("agent_tokenizer", tokenizer);
}
```

---

#### 11.2.2 Semantic Retrieval (Multi-Vector via Qdrant)

**Why Multi-Vector?**

A single embedding per agent collapses all semantic dimensions (capability, domain, interaction style) into one vector. This creates a representation bottleneck: an agent that is highly relevant for its _capabilities_ but operates in a different _domain_ than the query implies may be unfairly suppressed because the single vector averages out the capability match.

Multi-vector retrieval solves this by embedding different semantic facets of an agent into separate vector spaces, then combining the similarity scores with learned weights.

**Why Qdrant?**

Qdrant is a Rust-native vector database purpose-built for production similarity search. It provides:

- Named vectors (multiple vectors per point, each with its own HNSW index)
- Int8 scalar quantization (4× memory reduction with <1% recall loss)
- Payload filtering integrated into the HNSW search path (no post-filter recall loss)
- Snapshot-based backup and recovery
- gRPC API with streaming support
- Single-binary deployment (no JVM, no external dependencies)

Alternatives considered:

| System | Rejected Because |
|--------|-----------------|
| Pinecone | Managed-only; no self-hosted option; vendor lock-in risk |
| Weaviate | Go-based; higher memory overhead; less mature quantization |
| Milvus | Complex multi-component deployment; Java/Go hybrid; operational overhead |
| pgvector | Performance ceiling at scale; no native multi-vector; limited HNSW tuning |
| Vespa | Good candidate for Phase 2 consolidation but overkill for initial deployment |

**Three Embedding Vectors Per Agent**

Each agent is represented by three named vectors in Qdrant, each capturing a different semantic facet:

| Vector Name | Dimensions | Model | Source Fields | Semantic Purpose |
|-------------|-----------|-------|---------------|-----------------|
| `capability_embedding` | 768 | `BGE-large-en-v1.5` | `skills[].name` + `skills[].description` | What the agent _can do_. The primary matching signal for task-oriented queries. |
| `domain_embedding` | 768 | `BGE-large-en-v1.5` | `description` + `provider.organization` + `skills[].tags[]` | What _domain_ the agent operates in. Captures industry, platform, and ecosystem context. |
| `interaction_embedding` | 384 | `BGE-small-en-v1.5` | `capabilities` + I/O modes + auth schemes + protocol features | _How_ to interact with the agent. Captures protocol compatibility and integration style. |

**Why BGE (BAAI General Embedding)?**

BGE-large-en-v1.5 is the best open-source English embedding model on the MTEB leaderboard at our deployment size. Key properties:

- 768 dimensions (good balance of expressiveness and storage cost)
- Trained on massive retrieval datasets including technical documentation
- Apache 2.0 license (no usage restrictions)
- Efficient batch inference on GPU (~5,000 embeddings/sec on A10G)
- Supports instruction-prefix for asymmetric retrieval (short query → long document)

The `interaction_embedding` uses BGE-small (384d) because interaction metadata is lower-dimensional and the smaller model reduces storage by 50% per agent for this vector.

**Embedding Generation**

```python
from sentence_transformers import SentenceTransformer
import numpy as np
from typing import TypedDict

class AgentCard(TypedDict):
    name: str
    description: str
    skills: list[dict]
    provider: dict
    capabilities: dict
    auth_schemes: list[str]
    protocol_version: str

CAPABILITY_MODEL = SentenceTransformer("BAAI/bge-large-en-v1.5", device="cuda")
DOMAIN_MODEL = CAPABILITY_MODEL  # same model, different input
INTERACTION_MODEL = SentenceTransformer("BAAI/bge-small-en-v1.5", device="cuda")

CAPABILITY_INSTRUCTION = "Represent this agent's capabilities for retrieval: "
DOMAIN_INSTRUCTION = "Represent this agent's domain and context for retrieval: "
INTERACTION_INSTRUCTION = "Represent this agent's interaction profile for retrieval: "

def build_capability_text(card: AgentCard) -> str:
    """Concatenate skill names and descriptions into a single capability string."""
    parts = []
    for skill in card.get("skills", []):
        name = skill.get("name", "")
        desc = skill.get("description", "")
        if name and desc:
            parts.append(f"{name}: {desc}")
        elif name:
            parts.append(name)
    return " | ".join(parts)

def build_domain_text(card: AgentCard) -> str:
    """Concatenate description, provider, and tags into a domain context string."""
    parts = [card.get("description", "")]
    provider = card.get("provider", {})
    if org := provider.get("organization"):
        parts.append(f"Provider: {org}")
    for skill in card.get("skills", []):
        for tag in skill.get("tags", []):
            parts.append(tag)
    return " | ".join(filter(None, parts))

def build_interaction_text(card: AgentCard) -> str:
    """Concatenate capabilities, I/O modes, auth, and protocol into interaction string."""
    parts = []
    caps = card.get("capabilities", {})
    if streaming := caps.get("streaming"):
        parts.append(f"streaming={streaming}")
    if push_notifications := caps.get("pushNotifications"):
        parts.append(f"push_notifications={push_notifications}")
    if state_transition := caps.get("stateTransitionHistory"):
        parts.append(f"state_history={state_transition}")
    for scheme in card.get("auth_schemes", []):
        parts.append(f"auth:{scheme}")
    if proto := card.get("protocol_version"):
        parts.append(f"protocol:{proto}")
    input_modes = set()
    output_modes = set()
    for skill in card.get("skills", []):
        for m in skill.get("inputModes", []):
            input_modes.add(m)
        for m in skill.get("outputModes", []):
            output_modes.add(m)
    if input_modes:
        parts.append(f"input:[{','.join(sorted(input_modes))}]")
    if output_modes:
        parts.append(f"output:[{','.join(sorted(output_modes))}]")
    return " | ".join(parts)

def generate_embeddings(card: AgentCard) -> dict[str, np.ndarray]:
    """Generate all three embedding vectors for an agent card."""
    cap_text = build_capability_text(card)
    dom_text = build_domain_text(card)
    int_text = build_interaction_text(card)

    cap_emb = CAPABILITY_MODEL.encode(
        CAPABILITY_INSTRUCTION + cap_text,
        normalize_embeddings=True,
    )
    dom_emb = DOMAIN_MODEL.encode(
        DOMAIN_INSTRUCTION + dom_text,
        normalize_embeddings=True,
    )
    int_emb = INTERACTION_MODEL.encode(
        INTERACTION_INSTRUCTION + int_text,
        normalize_embeddings=True,
    )
    return {
        "capability_embedding": cap_emb,
        "domain_embedding": dom_emb,
        "interaction_embedding": int_emb,
    }

def batch_generate_embeddings(
    cards: list[AgentCard], batch_size: int = 256
) -> list[dict[str, np.ndarray]]:
    """Batch-generate embeddings for throughput efficiency."""
    all_cap_texts = [CAPABILITY_INSTRUCTION + build_capability_text(c) for c in cards]
    all_dom_texts = [DOMAIN_INSTRUCTION + build_domain_text(c) for c in cards]
    all_int_texts = [INTERACTION_INSTRUCTION + build_interaction_text(c) for c in cards]

    cap_embs = CAPABILITY_MODEL.encode(
        all_cap_texts, batch_size=batch_size, normalize_embeddings=True, show_progress_bar=True
    )
    dom_embs = DOMAIN_MODEL.encode(
        all_dom_texts, batch_size=batch_size, normalize_embeddings=True, show_progress_bar=True
    )
    int_embs = INTERACTION_MODEL.encode(
        all_int_texts, batch_size=batch_size, normalize_embeddings=True, show_progress_bar=True
    )

    return [
        {
            "capability_embedding": cap_embs[i],
            "domain_embedding": dom_embs[i],
            "interaction_embedding": int_embs[i],
        }
        for i in range(len(cards))
    ]
```

**Qdrant Collection Configuration**

```json
{
  "collection_name": "agents",
  "vectors": {
    "capability_embedding": {
      "size": 768,
      "distance": "Cosine",
      "hnsw_config": {
        "m": 16,
        "ef_construct": 200,
        "full_scan_threshold": 10000,
        "max_indexing_threads": 0,
        "on_disk": false
      },
      "quantization_config": {
        "scalar": {
          "type": "int8",
          "quantile": 0.99,
          "always_ram": true
        }
      }
    },
    "domain_embedding": {
      "size": 768,
      "distance": "Cosine",
      "hnsw_config": {
        "m": 16,
        "ef_construct": 200,
        "full_scan_threshold": 10000,
        "max_indexing_threads": 0,
        "on_disk": false
      },
      "quantization_config": {
        "scalar": {
          "type": "int8",
          "quantile": 0.99,
          "always_ram": true
        }
      }
    },
    "interaction_embedding": {
      "size": 384,
      "distance": "Cosine",
      "hnsw_config": {
        "m": 16,
        "ef_construct": 128,
        "full_scan_threshold": 10000,
        "max_indexing_threads": 0,
        "on_disk": false
      },
      "quantization_config": {
        "scalar": {
          "type": "int8",
          "quantile": 0.99,
          "always_ram": true
        }
      }
    }
  },
  "optimizers_config": {
    "default_segment_number": 4,
    "max_segment_size": 200000,
    "memmap_threshold": 50000,
    "indexing_threshold": 20000,
    "flush_interval_sec": 5,
    "max_optimization_threads": 4
  },
  "wal_config": {
    "wal_capacity_mb": 256,
    "wal_segments_ahead": 2
  },
  "replication_factor": 2,
  "shard_number": 4
}
```

**HNSW Parameter Rationale:**

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| `m` = 16 | 16 bidirectional connections per node | Standard value. m=16 gives ~95% recall at ef_search=128. Higher m (32, 64) increases memory linearly with diminishing recall improvement. |
| `ef_construct` = 200 | Build-time search width | Higher than default (100) to improve index quality at the cost of slower index builds. Index builds are offline batch operations, so build time is not latency-sensitive. |
| `ef_construct` = 128 (interaction) | Lower for smaller vector | 384d vectors need fewer connections for equivalent recall. Saves ~20% index build time. |
| Int8 quantization | 4× memory reduction | At quantile=0.99, the 1% most extreme values are clipped. Empirically causes <0.5% recall@100 degradation on our evaluation set. |
| `always_ram` = true | Quantized vectors stay in RAM | Eliminates disk I/O for the hot path. Full vectors on disk for re-ranking if needed. |

**Query-Time Multi-Vector Search**

At query time, the user's query is embedded into the same three vector spaces. The similarity scores from each space are combined using a weighted max-sim aggregation:

```
semantic_score(q, a) = w_cap × cos(q_cap, a_cap) 
                     + w_dom × cos(q_dom, a_dom) 
                     + w_int × cos(q_int, a_int)

Where:
  w_cap  = 0.50   (capability match is the primary signal)
  w_dom  = 0.35   (domain context is the secondary signal)
  w_int  = 0.15   (interaction compatibility is a tiebreaker)
  
  Σ weights = 1.00
```

**Weight rationale:**

- **Capability (0.50)**: When a user searches "translate text from English to French", the capability vector should dominate. The agent's translation capability is the primary relevance signal.
- **Domain (0.35)**: When a user searches "healthcare compliance agent", the domain vector should contribute significantly. An agent in the healthcare domain that does compliance checking is more relevant than a generic compliance agent.
- **Interaction (0.15)**: When a user searches "streaming agent with OAuth", the interaction vector adds signal. But interaction compatibility is typically handled by eligibility filters (Stage 2), so the vector weight is lower.

**Qdrant Query Implementation**

```python
from qdrant_client import QdrantClient, models

client = QdrantClient(host="qdrant.internal", port=6334, prefer_grpc=True)

def semantic_search(
    query_text: str,
    limit: int = 500,
    tenant_filter: str | None = None,
) -> list[tuple[str, float]]:
    """Multi-vector semantic search with weighted score combination."""
    q_cap = CAPABILITY_MODEL.encode(
        CAPABILITY_INSTRUCTION + query_text, normalize_embeddings=True
    )
    q_dom = DOMAIN_MODEL.encode(
        DOMAIN_INSTRUCTION + query_text, normalize_embeddings=True
    )
    q_int = INTERACTION_MODEL.encode(
        INTERACTION_INSTRUCTION + query_text, normalize_embeddings=True
    )

    query_filter = None
    if tenant_filter:
        query_filter = models.Filter(
            must=[
                models.FieldCondition(
                    key="tenant_ids",
                    match=models.MatchAny(any=[tenant_filter]),
                )
            ]
        )

    prefetch_cap = models.Prefetch(
        query=q_cap.tolist(),
        using="capability_embedding",
        limit=limit,
        filter=query_filter,
    )
    prefetch_dom = models.Prefetch(
        query=q_dom.tolist(),
        using="domain_embedding",
        limit=limit,
        filter=query_filter,
    )
    prefetch_int = models.Prefetch(
        query=q_int.tolist(),
        using="interaction_embedding",
        limit=limit,
        filter=query_filter,
    )

    results = client.query_points(
        collection_name="agents",
        prefetch=[prefetch_cap, prefetch_dom, prefetch_int],
        query=models.FusionQuery(fusion=models.Fusion.RRF),
        limit=limit,
    )

    return [(point.id, point.score) for point in results.points]
```

> **Implementation note:** Qdrant's native RRF fusion across prefetch queries is used here for simplicity. In the production pipeline, we may switch to application-level weighted fusion (Section 11.4) for finer control over the combination weights, since Qdrant's built-in RRF uses uniform weights across prefetch groups.

---

#### 11.2.3 Graph Retrieval

Graph retrieval captures relationships that neither lexical nor semantic search can model: "agents published by the same provider as this trusted agent", "agents commonly invoked together in workflows", "agents that benchmark similarly to a known good result".

**Why Neo4j?**

Neo4j is the standard graph database for relationship-heavy queries. The agent relationship graph has the following characteristics that favor a purpose-built graph database over SQL recursive CTEs or ad-hoc graph traversals:

- Variable-depth traversals (2-4 hops) with heterogeneous edge types
- Edge-weight-dependent scoring during traversal
- Frequent pattern queries ("find all agents that share a provider with X and are taxonomy siblings of Y")
- Subgraph extraction for explanation payloads

For Phase 1, Neo4j is used exclusively for graph-based candidate retrieval. In Phase 2, we may evaluate Apache AGE (graph extension for PostgreSQL) to reduce operational complexity.

**Edge Types**

The agent relationship graph contains five directed edge types:

| Edge Type | Direction | Source Signal | Weight | Description | Example |
|-----------|-----------|--------------|--------|-------------|---------|
| `PROVIDER_OF` | `Provider → Agent` | Registry | 1.0 | Provider publishes agent | `Acme Corp → acme-translator` |
| `INVOKES` | `Agent → Agent` | Outcome telemetry | 0.8 | Agent A delegates tasks to Agent B | `orchestrator-v2 → acme-translator` |
| `BENCHMARK_SIMILAR` | `Agent ↔ Agent` | Benchmark system | 0.6 | Similar benchmark profiles (cosine > 0.85 on benchmark feature vector) | `translator-a ↔ translator-b` |
| `TAXONOMY_SIBLING` | `Agent ↔ Agent` | Taxonomy system | 0.4 | Share ≥2 taxonomy categories | `translator-a ↔ translator-c` |
| `CITES` | `Agent → Agent` | Agent Card metadata | 0.3 | Agent Card references another agent (e.g., in description, examples, or linked agents) | `acme-pipeline → acme-translator` |

**Graph Schema (Cypher)**

```cypher
// Node types
CREATE CONSTRAINT agent_id_unique IF NOT EXISTS
  FOR (a:Agent) REQUIRE a.agent_id IS UNIQUE;

CREATE CONSTRAINT provider_id_unique IF NOT EXISTS
  FOR (p:Provider) REQUIRE p.provider_id IS UNIQUE;

CREATE CONSTRAINT taxonomy_id_unique IF NOT EXISTS
  FOR (t:TaxonomyNode) REQUIRE t.taxonomy_id IS UNIQUE;

// Node properties
// Agent: {agent_id, name, agent_rank, trust_tier, status, domain_embedding[768]}
// Provider: {provider_id, name, domain, verified, trust_tier}
// TaxonomyNode: {taxonomy_id, label, parent_id, depth}

// Edge types with properties
// (:Provider)-[:PROVIDER_OF {since: datetime}]->(:Agent)
// (:Agent)-[:INVOKES {count: int, last_seen: datetime, success_rate: float}]->(:Agent)
// (:Agent)-[:BENCHMARK_SIMILAR {cosine: float, benchmark_type: string}]-(:Agent)
// (:Agent)-[:TAXONOMY_SIBLING {shared_categories: list, jaccard: float}]-(:Agent)
// (:Agent)-[:CITES {context: string, citation_type: string}]->(:Agent)

// Indexes for traversal performance
CREATE INDEX agent_rank_idx IF NOT EXISTS FOR (a:Agent) ON (a.agent_rank);
CREATE INDEX agent_trust_idx IF NOT EXISTS FOR (a:Agent) ON (a.trust_tier);
CREATE INDEX agent_status_idx IF NOT EXISTS FOR (a:Agent) ON (a.status);
```

**BFS Traversal with Decay**

Graph retrieval starts from "seed nodes" — agents that scored highly in lexical or semantic retrieval — and traverses outward along the five edge types, applying an exponential decay factor at each hop to ensure that close neighbors are scored higher than distant ones.

```
graph_score(seed, candidate, path) = seed_score × Πᵢ (edge_weight(eᵢ) × decay^hopᵢ)

Where:
  seed_score   = the candidate generation score of the seed agent
  edge_weight  = the base weight of edge type (see table above)
  decay        = 0.6 (exponential decay per hop)
  hopᵢ         = the hop number (1-indexed) for edge eᵢ in the path
  
Example (2-hop path):
  seed_score = 0.85
  path: seed --INVOKES--> A --TAXONOMY_SIBLING--> candidate
  graph_score = 0.85 × (0.8 × 0.6¹) × (0.4 × 0.6²)
              = 0.85 × 0.48 × 0.144
              = 0.0588
```

**Traversal Algorithm**

```python
from dataclasses import dataclass
from collections import defaultdict
from neo4j import GraphDatabase

EDGE_WEIGHTS = {
    "PROVIDER_OF": 1.0,
    "INVOKES": 0.8,
    "BENCHMARK_SIMILAR": 0.6,
    "TAXONOMY_SIBLING": 0.4,
    "CITES": 0.3,
}
DECAY = 0.6
MAX_HOPS = 3
MAX_CANDIDATES = 100

@dataclass
class GraphCandidate:
    agent_id: str
    score: float
    path: list[str]
    hops: int

def graph_retrieval(
    seed_agents: list[tuple[str, float]],
    max_hops: int = MAX_HOPS,
    max_candidates: int = MAX_CANDIDATES,
) -> list[GraphCandidate]:
    """BFS-based graph retrieval from seed agents with exponential decay.
    
    Args:
        seed_agents: List of (agent_id, seed_score) from lexical/semantic retrieval.
                     Typically top-20 from each mode.
        max_hops: Maximum traversal depth (default 3).
        max_candidates: Maximum candidates to return (default 100).
    """
    driver = GraphDatabase.driver("bolt://neo4j.internal:7687")
    candidate_scores: dict[str, GraphCandidate] = {}
    seed_ids = {agent_id for agent_id, _ in seed_agents}
    
    with driver.session() as session:
        for seed_id, seed_score in seed_agents:
            query = """
            MATCH path = (seed:Agent {agent_id: $seed_id})-[rels*1..""" + str(max_hops) + """]->(candidate:Agent)
            WHERE candidate.status = 'healthy'
              AND candidate.agent_id <> $seed_id
            WITH candidate, rels, 
                 [r IN rels | type(r)] AS edge_types,
                 length(path) AS hops
            RETURN candidate.agent_id AS agent_id,
                   edge_types,
                   hops
            LIMIT 500
            """
            
            results = session.run(query, seed_id=seed_id)
            
            for record in results:
                agent_id = record["agent_id"]
                edge_types = record["edge_types"]
                hops = record["hops"]
                
                score = seed_score
                for hop_idx, edge_type in enumerate(edge_types, start=1):
                    weight = EDGE_WEIGHTS.get(edge_type, 0.1)
                    score *= weight * (DECAY ** hop_idx)
                
                if agent_id in candidate_scores:
                    existing = candidate_scores[agent_id]
                    if score > existing.score:
                        candidate_scores[agent_id] = GraphCandidate(
                            agent_id=agent_id,
                            score=score,
                            path=[seed_id] + edge_types,
                            hops=hops,
                        )
                    else:
                        candidate_scores[agent_id] = GraphCandidate(
                            agent_id=existing.agent_id,
                            score=existing.score + score * 0.1,
                            path=existing.path,
                            hops=existing.hops,
                        )
                else:
                    candidate_scores[agent_id] = GraphCandidate(
                        agent_id=agent_id,
                        score=score,
                        path=[seed_id] + edge_types,
                        hops=hops,
                    )
    
    candidates = sorted(
        candidate_scores.values(), key=lambda c: c.score, reverse=True
    )
    return candidates[:max_candidates]
```

**Graph Retrieval Characteristics**

| Property | Value | Notes |
|----------|-------|-------|
| Seed agents | Top-20 from lexical + top-20 from semantic (deduplicated) | ~30-35 unique seeds after dedup |
| Max hops | 3 | Beyond 3 hops, decay reduces scores below noise floor |
| Max candidates returned | 100 | Graph retrieval is the smallest contributor to the candidate pool |
| Typical latency | 5-8ms | Neo4j traversal with warm caches |
| Cold start behavior | Returns empty when no edges exist for seed agents | Gracefully handled by RRF fusion (lexical + semantic still contribute) |

---

### 11.3 Eligibility Filters

Eligibility filters are hard boolean constraints that remove candidates from consideration. Unlike scoring (which is relative), eligibility is absolute: an agent either passes all filters or is removed. Filters are evaluated using bit-vector intersection for performance.

**The 9 Hard Filters**

| # | Filter | Field | Semantics | Typical Cardinality | Example |
|---|--------|-------|-----------|--------------------|---------| 
| 1 | **Protocol Compatibility** | `protocols` | Agent supports at least one protocol the caller can speak | ~5 protocol bits | Caller requires A2A v1.0; agent only supports MCP → filtered out |
| 2 | **Auth Scheme Compatibility** | `auth_schemes` | Agent supports at least one auth scheme the caller can provide | ~8 auth scheme bits | Caller can provide OAuth2; agent requires mTLS only → filtered out |
| 3 | **Region Constraint** | `regions` | Agent operates in at least one region the caller accepts | ~20 region bits | Caller restricted to EU; agent only deployed in us-east-1 → filtered out |
| 4 | **Modality Match** | `input_modes`, `output_modes` | Agent supports the required I/O modalities | ~10 modality bits | Caller needs image output; agent only produces text → filtered out |
| 5 | **Tenant ACL** | `tenant_ids`, `visibility` | Agent is visible to the caller's tenant | ~1000 tenant bits (compressed) | Private agent owned by Tenant A; caller is Tenant B → filtered out |
| 6 | **Policy Compliance** | `policy_tags` | Agent passes the caller's policy posture (e.g., no-PII, SOC2-required) | ~30 policy bits | Caller requires SOC2; agent has no compliance attestation → filtered out |
| 7 | **Trust Floor** | `trust_tier` | Agent meets the minimum trust tier for the search context | 6 trust tiers (ordinal) | Search context requires `Verified` minimum; agent is `Indexed` only → filtered out |
| 8 | **Price Ceiling** | `price_tier` | Agent's pricing tier is within the caller's budget constraint | ~5 price tiers | Caller sets max price tier = `standard`; agent is `enterprise` → filtered out |
| 9 | **Latency SLA** | `p95_latency_ms` | Agent's observed P95 latency is within the caller's latency budget | Continuous (bucketed to 8 ranges) | Caller requires P95 < 200ms; agent's observed P95 is 800ms → filtered out |

**Bit-Vector Implementation**

Filters 1-6 use pre-computed bit vectors stored as Tantivy fast fields. At query time, the caller's constraints are compiled into a bit mask, and eligibility is checked via bitwise AND:

```
eligible = (agent.protocols & caller.required_protocols) != 0
         ∧ (agent.auth_schemes & caller.available_auth) != 0
         ∧ (agent.regions & caller.accepted_regions) != 0
         ∧ (agent.modalities & caller.required_modalities) == caller.required_modalities
         ∧ (agent.tenant_visibility & caller.tenant_mask) != 0
         ∧ (agent.policy_tags & caller.required_policies) == caller.required_policies
```

Filters 7-9 use range comparisons on fast fields:

```
eligible = eligible
         ∧ agent.trust_tier >= caller.min_trust_tier
         ∧ agent.price_tier <= caller.max_price_tier
         ∧ agent.p95_latency_ms <= caller.max_latency_ms
```

**Performance:** Bit-vector intersection on 1,000 candidates completes in <1ms on a single core. The filters are applied in the order listed (protocol first, latency last) because early filters have the highest selectivity, reducing the candidate set before more expensive range comparisons.

**Filter Cascade Statistics (Expected)**

| After Filter | Remaining (typical) | Selectivity |
|-------------|-------------------|--------------| 
| 1. Protocol | ~900 | 0.90 (most agents support A2A) |
| 2. Auth | ~800 | 0.89 (most agents support common auth) |
| 3. Region | ~600 | 0.75 (region constraints are common in enterprise) |
| 4. Modality | ~550 | 0.92 (most queries are text-only) |
| 5. Tenant ACL | ~500 | 0.91 (public queries see most agents) |
| 6. Policy | ~450 | 0.90 (policy filters are opt-in) |
| 7. Trust Floor | ~420 | 0.93 (default trust floor is low) |
| 8. Price | ~410 | 0.98 (most callers don't set price limits) |
| 9. Latency SLA | ~400 | 0.98 (most callers don't set latency limits) |

---

### 11.4 Result Fusion (Reciprocal Rank Fusion — RRF)

The three candidate generation modes produce three ranked lists. Reciprocal Rank Fusion (RRF) combines them into a single ranked list without requiring score normalization across heterogeneous scoring functions.

**Why RRF?**

| Method | Pros | Cons | Why Not Chosen (or Why Chosen) |
|--------|------|------|-------------------------------|
| **Weighted Linear Combination** | Simple, interpretable | Requires score normalization across modes; BM25 scores (0-30 range) vs cosine similarity (0-1 range) are not directly comparable; normalization is query-dependent | Score distribution differences make normalization unreliable |
| **CombSUM** | Simple addition | Same normalization problem as weighted linear | — |
| **CombMNZ** | Rewards documents found by multiple modes | Still requires comparable scores | — |
| **Reciprocal Rank Fusion (RRF)** | No score normalization needed; only uses rank positions; robust to score distribution differences; proven in production at scale (Microsoft, Qdrant, Vespa) | Ignores score magnitude (a document ranked #1 with score 0.99 and one ranked #1 with score 0.51 are treated identically) | **Chosen.** Simplicity and robustness outweigh the score magnitude loss. |
| **Learned fusion (LTR)** | Optimal given training data | Requires labeled data that we don't have in Phase 1 | Phase 3 roadmap item |

**RRF Formula**

```
RRF_score(d) = Σᵢ 1 / (k + rankᵢ(d))

Where:
  d       = a candidate agent
  i       = retrieval mode index (lexical=1, semantic=2, graph=3)
  rankᵢ(d) = the rank of d in mode i's result list (1-indexed)
            = ∞ if d was not retrieved by mode i (contributes 0)
  k       = 60 (smoothing constant)
```

**Why k = 60?**

The constant `k` controls how much top ranks dominate the fused score. At k=60:

| Rank | Contribution: 1/(60 + rank) | Cumulative insight |
|------|----------------------------|-------------------|
| 1 | 1/61 = 0.01639 | Rank 1 contributes ~1.6% of the theoretical maximum |
| 10 | 1/70 = 0.01429 | Rank 10 contributes 87% of rank 1's contribution — high ranks matter but don't dominate |
| 50 | 1/110 = 0.00909 | Rank 50 still contributes 55% of rank 1's contribution — deep results are not wasted |
| 100 | 1/160 = 0.00625 | Rank 100 contributes 38% of rank 1's contribution — useful for rare matches |
| 500 | 1/560 = 0.00179 | Rank 500 contributes 11% of rank 1's contribution — diminishing but nonzero |

**Alternative k values and their behavior:**

- **k=1**: Rank 1 contributes 0.5, rank 2 contributes 0.33 — top ranks dominate too aggressively. A document ranked #1 by one mode and not found by others would beat a document ranked #5 in all three modes.
- **k=20**: More balanced than k=1 but still top-heavy. Rank 1 contributes 0.0476, rank 10 contributes 0.0333 (70% of rank 1).
- **k=60**: The standard choice (originally proposed by Cormack et al., 2009). Rank 1 contributes 0.0164, rank 10 contributes 0.0143 (87% of rank 1). This flattens the contribution curve, rewarding documents that appear in multiple lists over documents that appear high in a single list.
- **k=100**: Even flatter — approaches uniform weighting. Loses the benefit of rank position.

We use k=60 because it maximizes the "found by multiple modes" signal while still preferring higher-ranked documents within each mode.

**RRF Computation Example**

Query: "kubernetes deployment agent with OAuth"

| Agent | Lexical Rank | Semantic Rank | Graph Rank | RRF Score |
|-------|-------------|---------------|------------|-----------|
| `k8s-deploy-pro` | 1 | 3 | — | 1/(60+1) + 1/(60+3) + 0 = 0.01639 + 0.01587 = **0.03226** |
| `kube-orchestrator` | 5 | 1 | 8 | 1/(60+5) + 1/(60+1) + 1/(60+8) = 0.01538 + 0.01639 + 0.01471 = **0.04648** |
| `deploy-agent-v2` | 2 | 12 | — | 1/(60+2) + 1/(60+12) + 0 = 0.01613 + 0.01389 = **0.03002** |
| `acme-k8s-helper` | — | 7 | 2 | 0 + 1/(60+7) + 1/(60+2) = 0.01493 + 0.01613 = **0.03106** |

Result: `kube-orchestrator` ranks first because it was found by all three modes. `k8s-deploy-pro` ranks second despite being #1 in lexical because it was not found by graph retrieval.

**Python RRF Implementation**

```python
from collections import defaultdict
from dataclasses import dataclass, field

@dataclass
class FusedResult:
    agent_id: str
    rrf_score: float
    mode_ranks: dict[str, int] = field(default_factory=dict)
    mode_count: int = 0

def reciprocal_rank_fusion(
    ranked_lists: dict[str, list[str]],
    k: int = 60,
    top_n: int = 1000,
) -> list[FusedResult]:
    """Fuse multiple ranked lists using Reciprocal Rank Fusion.
    
    Args:
        ranked_lists: Dict mapping mode name to ordered list of agent IDs.
                      e.g., {"lexical": ["a1", "a2", ...], "semantic": [...], "graph": [...]}
        k: RRF smoothing constant (default 60).
        top_n: Maximum number of fused results to return.
    
    Returns:
        List of FusedResult objects sorted by descending RRF score.
    """
    scores: dict[str, float] = defaultdict(float)
    mode_ranks: dict[str, dict[str, int]] = defaultdict(dict)
    mode_counts: dict[str, int] = defaultdict(int)
    
    for mode_name, ranked_list in ranked_lists.items():
        for rank_0indexed, agent_id in enumerate(ranked_list):
            rank = rank_0indexed + 1  # RRF uses 1-indexed ranks
            contribution = 1.0 / (k + rank)
            scores[agent_id] += contribution
            mode_ranks[agent_id][mode_name] = rank
            if mode_name not in mode_ranks[agent_id] or True:
                mode_counts[agent_id] = len(mode_ranks[agent_id])
    
    results = [
        FusedResult(
            agent_id=agent_id,
            rrf_score=score,
            mode_ranks=mode_ranks[agent_id],
            mode_count=len(mode_ranks[agent_id]),
        )
        for agent_id, score in scores.items()
    ]
    
    results.sort(key=lambda r: r.rrf_score, reverse=True)
    return results[:top_n]

def weighted_rrf(
    ranked_lists: dict[str, list[str]],
    mode_weights: dict[str, float],
    k: int = 60,
    top_n: int = 1000,
) -> list[FusedResult]:
    """Weighted RRF variant where each mode has a configurable weight.
    
    Default weights:
        lexical = 0.35
        semantic = 0.45
        graph = 0.20
    """
    scores: dict[str, float] = defaultdict(float)
    mode_ranks: dict[str, dict[str, int]] = defaultdict(dict)
    
    for mode_name, ranked_list in ranked_lists.items():
        weight = mode_weights.get(mode_name, 1.0)
        for rank_0indexed, agent_id in enumerate(ranked_list):
            rank = rank_0indexed + 1
            contribution = weight / (k + rank)
            scores[agent_id] += contribution
            mode_ranks[agent_id][mode_name] = rank
    
    results = [
        FusedResult(
            agent_id=agent_id,
            rrf_score=score,
            mode_ranks=mode_ranks[agent_id],
            mode_count=len(mode_ranks[agent_id]),
        )
        for agent_id, score in scores.items()
    ]
    
    results.sort(key=lambda r: r.rrf_score, reverse=True)
    return results[:top_n]
```

---

### 11.5 Re-Ranking (Two-Stage)

After RRF fusion and lightweight scoring (Stage 3) produce the top-100 candidates, we apply two re-ranking stages that use progressively more expensive models. The key design principle: **cheap models on many candidates, expensive models on few candidates.**

```
┌────────────────────────────────────────────────────────────────────────────┐
│                      TWO-STAGE RE-RANKING PIPELINE                         │
│                                                                            │
│   Input: 100 candidates from RRF + lightweight scoring                     │
│                                                                            │
│   ┌─────────────────────────────────────────┐                              │
│   │  Stage 1: Cross-Encoder Re-ranking       │                              │
│   │  Model: cross-encoder/ms-marco-MiniLM-L-6-v2                          │
│   │  Input: (query_text, agent_text) pairs   │                              │
│   │  Output: relevance score ∈ [-10, 10]     │                              │
│   │  Candidates: 100 → 50                    │                              │
│   │  Latency: ~15ms (batched GPU inference)  │                              │
│   └────────────┬────────────────────────────┘                              │
│                │                                                            │
│                ▼                                                            │
│   ┌─────────────────────────────────────────┐                              │
│   │  Stage 2: LambdaMART (XGBoost LTR)      │                              │
│   │  Model: XGBoost rank:ndcg               │                              │
│   │  Input: 25-feature vector per candidate  │                              │
│   │  Output: final relevance score           │                              │
│   │  Candidates: 50 → final ranking          │                              │
│   │  Latency: ~2ms (CPU inference)           │                              │
│   └─────────────────────────────────────────┘                              │
│                                                                            │
│   Total re-ranking latency: ~17ms                                          │
└────────────────────────────────────────────────────────────────────────────┘
```

#### Stage 1: Cross-Encoder (top-100 → top-50)

**Why a cross-encoder?**

Bi-encoders (used in Stage 1 candidate generation) encode the query and document independently, then compute similarity. This is fast but loses cross-attention: the model cannot attend to query terms when encoding the document. Cross-encoders jointly encode the (query, document) pair, allowing full cross-attention. This produces significantly better relevance scores but at O(n) cost per candidate (each pair requires a full forward pass).

**Model Selection: `cross-encoder/ms-marco-MiniLM-L-6-v2`**

| Property | Value |
|----------|-------|
| Architecture | 6-layer MiniLM (distilled from BERT) |
| Parameters | 22.7M |
| Max sequence length | 512 tokens |
| Training data | MS MARCO passage ranking (530K training pairs) |
| NDCG@10 on MS MARCO | 0.390 (competitive with 12-layer models at 2× speed) |
| Inference latency | ~0.15ms per pair on A10G GPU |
| Batch throughput | 100 pairs in ~15ms (batched) |

**Why MiniLM-L-6 and not a larger model?**

The 6-layer model is chosen for latency. A 12-layer cross-encoder (e.g., `ms-marco-MiniLM-L-12-v2`) improves NDCG@10 by ~2% but doubles inference time. At our latency budget (15ms for 100 pairs), the 6-layer model is the best accuracy/latency trade-off. If the latency budget increases (e.g., for offline re-ranking), we would upgrade to the 12-layer variant.

**Agent Text Construction for Cross-Encoder**

The cross-encoder input is a (query, agent_text) pair. The agent text is constructed to fit within the 512-token limit while maximizing information density:

```python
def build_agent_text_for_reranking(agent: dict, max_tokens: int = 400) -> str:
    """Build a compact text representation for cross-encoder re-ranking.
    
    Priority order (most informative first):
    1. Agent name
    2. First skill name + description
    3. Agent description (truncated)
    4. Additional skill names
    5. Tags
    """
    parts = []
    
    parts.append(f"Agent: {agent['name']}")
    
    if agent.get("skills"):
        first_skill = agent["skills"][0]
        parts.append(f"Primary skill: {first_skill['name']}")
        if desc := first_skill.get("description"):
            parts.append(desc[:200])
    
    if desc := agent.get("description"):
        parts.append(f"Description: {desc[:200]}")
    
    if agent.get("skills") and len(agent["skills"]) > 1:
        other_skills = [s["name"] for s in agent["skills"][1:5]]
        parts.append(f"Other skills: {', '.join(other_skills)}")
    
    all_tags = set()
    for skill in agent.get("skills", []):
        for tag in skill.get("tags", []):
            all_tags.add(tag)
    if all_tags:
        parts.append(f"Tags: {', '.join(sorted(all_tags)[:10])}")
    
    text = " | ".join(parts)
    return text[:max_tokens * 4]  # rough char-to-token estimate
```

**Cross-Encoder Reranker Implementation**

```python
import torch
import numpy as np
from transformers import AutoModelForSequenceClassification, AutoTokenizer

class CrossEncoderReranker:
    """Cross-encoder re-ranker for Stage 1 re-ranking (100 → 50)."""
    
    def __init__(
        self,
        model_name: str = "cross-encoder/ms-marco-MiniLM-L-6-v2",
        device: str = "cuda",
        max_length: int = 512,
        batch_size: int = 32,
    ):
        self.device = torch.device(device if torch.cuda.is_available() else "cpu")
        self.tokenizer = AutoTokenizer.from_pretrained(model_name)
        self.model = AutoModelForSequenceClassification.from_pretrained(model_name)
        self.model.to(self.device)
        self.model.eval()
        self.max_length = max_length
        self.batch_size = batch_size
        
        self._score_cache: dict[tuple[str, str], float] = {}
        self._cache_max = 50_000
    
    def _get_cache_key(self, query: str, agent_text: str) -> tuple[str, str]:
        return (query[:100], agent_text[:100])
    
    @torch.no_grad()
    def rerank(
        self,
        query: str,
        candidates: list[dict],
        top_k: int = 50,
    ) -> list[tuple[dict, float]]:
        """Re-rank candidates using cross-encoder scores.
        
        Args:
            query: The search query text.
            candidates: List of agent dicts with at minimum 'agent_id', 'name', 'skills', etc.
            top_k: Number of top results to return after re-ranking.
            
        Returns:
            List of (agent_dict, score) tuples sorted by descending relevance.
        """
        pairs = []
        agent_texts = []
        cached_scores = {}
        uncached_indices = []
        
        for i, candidate in enumerate(candidates):
            agent_text = build_agent_text_for_reranking(candidate)
            agent_texts.append(agent_text)
            cache_key = self._get_cache_key(query, agent_text)
            
            if cache_key in self._score_cache:
                cached_scores[i] = self._score_cache[cache_key]
            else:
                pairs.append((query, agent_text))
                uncached_indices.append(i)
        
        new_scores = {}
        if pairs:
            all_scores = []
            for batch_start in range(0, len(pairs), self.batch_size):
                batch = pairs[batch_start:batch_start + self.batch_size]
                encoded = self.tokenizer(
                    [p[0] for p in batch],
                    [p[1] for p in batch],
                    padding=True,
                    truncation=True,
                    max_length=self.max_length,
                    return_tensors="pt",
                ).to(self.device)
                
                outputs = self.model(**encoded)
                scores = outputs.logits.squeeze(-1).cpu().numpy()
                all_scores.extend(scores.tolist() if isinstance(scores, np.ndarray) else [scores])
            
            for idx, score in zip(uncached_indices, all_scores):
                new_scores[idx] = float(score)
                cache_key = self._get_cache_key(query, agent_texts[idx])
                self._score_cache[cache_key] = float(score)
                
                if len(self._score_cache) > self._cache_max:
                    oldest_key = next(iter(self._score_cache))
                    del self._score_cache[oldest_key]
        
        all_results = []
        for i, candidate in enumerate(candidates):
            score = cached_scores.get(i) or new_scores.get(i, -10.0)
            all_results.append((candidate, score))
        
        all_results.sort(key=lambda x: x[1], reverse=True)
        return all_results[:top_k]
```

**Latency Management**

| Technique | Impact | Details |
|-----------|--------|---------|
| **Batch inference** | 100 pairs → 4 batches of 25 → ~15ms total | GPU parallelism amortizes per-pair overhead |
| **LRU cache** | ~30% cache hit rate for popular queries | Same query text against same agent text returns cached score |
| **Async overlap** | Re-ranking overlaps with Stage 2 feature computation | While cross-encoder runs, LambdaMART features are precomputed for the top candidates |
| **Model quantization** | INT8 quantization reduces inference by ~30% | Applied via ONNX Runtime with INT8 calibration |
| **Token truncation** | Max 512 tokens per pair | Prevents long agent descriptions from inflating inference time |

---

#### Stage 2: LambdaMART (XGBoost) — top-50 → final ranking

LambdaMART is a gradient-boosted decision tree algorithm optimized for ranking. It directly optimizes NDCG (Normalized Discounted Cumulative Gain), making it ideal for learning-to-rank applications where the goal is to put the best results at the top.

**Why XGBoost for LambdaMART?**

- Native `rank:ndcg` objective function
- CPU inference in ~2ms for 50 candidates with 25 features
- Interpretable feature importances
- Robust to missing features (common in agent metadata)
- Battle-tested in production ranking systems (Airbnb, Booking.com, LinkedIn)

**The 25-Feature Vector**

Every candidate in the top-50 is described by a 25-feature vector, organized into five categories:

| # | Feature | Category | Type | Range | Source | Description |
|---|---------|----------|------|-------|--------|-------------|
| 1 | `bm25_score` | Retrieval | float | [0, ∞) | Tantivy | Raw BM25 score from lexical retrieval |
| 2 | `semantic_score_cap` | Retrieval | float | [-1, 1] | Qdrant | Cosine similarity on capability embedding |
| 3 | `semantic_score_dom` | Retrieval | float | [-1, 1] | Qdrant | Cosine similarity on domain embedding |
| 4 | `semantic_score_int` | Retrieval | float | [-1, 1] | Qdrant | Cosine similarity on interaction embedding |
| 5 | `rrf_score` | Retrieval | float | [0, ~0.05] | RRF fusion | Reciprocal Rank Fusion score |
| 6 | `cross_encoder_score` | Retrieval | float | [-10, 10] | Stage 1 | Cross-encoder relevance score |
| 7 | `retrieval_mode_count` | Retrieval | int | [1, 3] | RRF fusion | Number of retrieval modes that found this candidate |
| 8 | `agent_rank` | Agent Quality | float | [0, 100] | AgentRank | Precomputed AgentRank composite score |
| 9 | `trust_tier` | Agent Quality | int | [0, 5] | Registry | Trust tier ordinal (0=Suspicious, 5=Authoritative) |
| 10 | `uptime_30d` | Agent Quality | float | [0, 1] | Liveness | 30-day uptime ratio |
| 11 | `p95_latency_norm` | Agent Quality | float | [0, 1] | Liveness | Normalized P95 latency (0=fast, 1=slow) |
| 12 | `benchmark_score` | Agent Quality | float | [0, 1] | Benchmark | Best benchmark score for relevant capability |
| 13 | `skill_count` | Agent Quality | int | [1, ∞) | Registry | Number of declared skills |
| 14 | `query_name_match` | Query-Agent Match | float | [0, 1] | Computed | Jaccard similarity between query tokens and agent name tokens |
| 15 | `query_skill_match` | Query-Agent Match | float | [0, 1] | Computed | Max Jaccard similarity between query and any skill name |
| 16 | `query_tag_overlap` | Query-Agent Match | float | [0, 1] | Computed | Fraction of query entity tags that appear in agent tags |
| 17 | `query_domain_match` | Query-Agent Match | float | [0, 1] | Computed | Whether agent's taxonomy labels match query-inferred domain |
| 18 | `description_length` | Query-Agent Match | int | [0, ∞) | Registry | Character length of agent description (proxy for metadata quality) |
| 19 | `hours_since_last_crawl` | Freshness | float | [0, ∞) | Crawler | Hours since last successful crawl |
| 20 | `hours_since_last_healthy` | Freshness | float | [0, ∞) | Liveness | Hours since last healthy probe |
| 21 | `card_version_count` | Freshness | int | [1, ∞) | Registry | Number of card versions (proxy for active maintenance) |
| 22 | `ctr_7d` | Behavioral | float | [0, 1] | Analytics | 7-day click-through rate from search results |
| 23 | `connect_success_rate_30d` | Behavioral | float | [0, 1] | Outcome | 30-day connection success rate |
| 24 | `delegation_in_degree` | Behavioral | int | [0, ∞) | Graph | Number of agents that delegate to this agent |
| 25 | `provider_agent_count` | Behavioral | int | [1, ∞) | Registry | Number of agents from the same provider (provider diversity signal) |

**Feature Category Visualization**

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    25-FEATURE VECTOR ORGANIZATION                         │
│                                                                          │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  ┌──────────┐    │
│  │  RETRIEVAL   │  │ AGENT QUALITY │  │ QUERY-AGENT   │  │ FRESHNESS│    │
│  │  (7 features)│  │ (6 features)  │  │ MATCH         │  │ (3 feat.)│    │
│  │             │  │              │  │ (5 features)  │  │          │    │
│  │  bm25       │  │  agent_rank  │  │  name_match   │  │  crawl   │    │
│  │  sem_cap    │  │  trust_tier  │  │  skill_match  │  │  healthy │    │
│  │  sem_dom    │  │  uptime_30d  │  │  tag_overlap  │  │  versions│    │
│  │  sem_int    │  │  p95_latency │  │  domain_match │  │          │    │
│  │  rrf        │  │  benchmark   │  │  desc_length  │  │          │    │
│  │  cross_enc  │  │  skill_count │  │               │  │          │    │
│  │  mode_count │  │              │  │               │  │          │    │
│  └─────────────┘  └──────────────┘  └───────────────┘  └──────────┘    │
│                                                                          │
│  ┌────────────────────────────┐                                          │
│  │  BEHAVIORAL (4 features)   │                                          │
│  │                            │                                          │
│  │  ctr_7d                    │                                          │
│  │  connect_success_rate_30d  │                                          │
│  │  delegation_in_degree      │                                          │
│  │  provider_agent_count      │                                          │
│  └────────────────────────────┘                                          │
│                                                                          │
│  Total: 25 features × 50 candidates = 1,250 feature values per query    │
└──────────────────────────────────────────────────────────────────────────┘
```

**Training Pipeline**

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    LAMBDAMART TRAINING PIPELINE                           │
│                                                                          │
│  ┌─────────────┐     ┌──────────────┐     ┌──────────────┐              │
│  │  DATA        │     │  DATA         │     │  TRAINING     │              │
│  │  COLLECTION  │────▶│  PREPARATION  │────▶│               │              │
│  │             │     │              │     │  XGBoost      │              │
│  │ 3 sources:  │     │ Feature      │     │  rank:ndcg    │              │
│  │ • implicit  │     │ extraction   │     │               │              │
│  │   CTR data  │     │ Label        │     │  5-fold CV    │              │
│  │ • explicit  │     │ assignment   │     │               │              │
│  │   human     │     │ Query group  │     │  Hyperparameter│             │
│  │   ratings   │     │ formation    │     │  tuning       │              │
│  │ • synthetic │     │ Train/val    │     │               │              │
│  │   LLM       │     │ split        │     │               │              │
│  │   judgments  │     │              │     │               │              │
│  └─────────────┘     └──────────────┘     └──────┬───────┘              │
│                                                   │                      │
│                                                   ▼                      │
│  ┌──────────────┐     ┌──────────────┐                                   │
│  │  DEPLOYMENT   │◀────│  EVALUATION   │                                   │
│  │              │     │              │                                   │
│  │ A/B test     │     │ NDCG@5, @10  │                                   │
│  │ Shadow mode  │     │ MAP@10       │                                   │
│  │ Gradual      │     │ MRR          │                                   │
│  │ rollout      │     │ Precision@1  │                                   │
│  │              │     │ Feature imp. │                                   │
│  └──────────────┘     └──────────────┘                                   │
│                                                                          │
│  Cadence: Weekly retraining, daily feature refresh                       │
└──────────────────────────────────────────────────────────────────────────┘
```

**Training Data Sources**

| Source | Label Type | Volume (weekly) | Trust Level | Weight |
|--------|-----------|-----------------|-------------|--------|
| **Implicit CTR** | Binary (clicked=1, shown-not-clicked=0) | ~100K pairs | Medium (noisy — click ≠ relevance) | 0.30 |
| **Explicit Human Ratings** | 4-point scale (irrelevant=0, marginally=1, relevant=2, highly=3) | ~2K pairs | High (gold standard) | 0.50 |
| **Synthetic LLM Judgments** | 4-point scale (same as human) | ~20K pairs | Medium-Low (calibrated against human ratings) | 0.20 |

**Label Assignment Rules:**

- Implicit CTR: `clicked AND connected_successfully` = relevant (1), `shown_not_clicked` = irrelevant (0)
- Human ratings: Trained raters assess (query, agent) pairs on a 4-point scale
- Synthetic LLM: GPT-4 assesses (query, agent_card) pairs with calibrated prompts; outputs are de-biased against human rating distribution

**XGBoost Training Configuration**

```python
import xgboost as xgb
import numpy as np

params = {
    "objective": "rank:ndcg",
    "eval_metric": ["ndcg@5", "ndcg@10", "map@10"],
    "eta": 0.05,
    "max_depth": 6,
    "min_child_weight": 10,
    "subsample": 0.8,
    "colsample_bytree": 0.8,
    "lambda": 1.0,           # L2 regularization
    "alpha": 0.1,            # L1 regularization
    "gamma": 0.1,            # min loss reduction for split
    "max_bin": 256,
    "tree_method": "hist",
    "nthread": 8,
    "seed": 42,
}

def train_lambdamart(
    X_train: np.ndarray,
    y_train: np.ndarray,
    group_train: list[int],
    X_val: np.ndarray,
    y_val: np.ndarray,
    group_val: list[int],
    num_rounds: int = 2000,
    early_stopping: int = 50,
) -> xgb.Booster:
    """Train a LambdaMART model using XGBoost.
    
    Args:
        X_train: Feature matrix (n_samples, 25).
        y_train: Relevance labels (n_samples,).
        group_train: Query group sizes (n_queries,). Sum must equal n_samples.
        X_val: Validation feature matrix.
        y_val: Validation labels.
        group_val: Validation query group sizes.
        num_rounds: Maximum boosting rounds.
        early_stopping: Stop if val metric doesn't improve for this many rounds.
    """
    dtrain = xgb.DMatrix(X_train, label=y_train)
    dtrain.set_group(group_train)
    
    dval = xgb.DMatrix(X_val, label=y_val)
    dval.set_group(group_val)
    
    model = xgb.train(
        params,
        dtrain,
        num_boost_round=num_rounds,
        evals=[(dtrain, "train"), (dval, "val")],
        early_stopping_rounds=early_stopping,
        verbose_eval=100,
    )
    
    return model

def predict_scores(
    model: xgb.Booster,
    X: np.ndarray,
) -> np.ndarray:
    """Predict ranking scores for candidates."""
    dmatrix = xgb.DMatrix(X)
    return model.predict(dmatrix)
```

**XGBoost Parameter Rationale**

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| `eta` = 0.05 | Low learning rate | Prevents overfitting on noisy CTR labels; combined with 2000 rounds and early stopping |
| `max_depth` = 6 | Moderate tree depth | Allows complex feature interactions (e.g., BM25 × trust_tier) without memorizing training noise |
| `min_child_weight` = 10 | Minimum leaf weight | Prevents splits on small query groups; stabilizes predictions for rare query types |
| `subsample` = 0.8 | Row sampling | Standard bagging ratio; reduces overfitting on training set |
| `colsample_bytree` = 0.8 | Feature sampling | Forces trees to use diverse feature subsets; prevents over-reliance on BM25 or cross-encoder score |
| `lambda` = 1.0 | L2 regularization | Standard regularization to prevent extreme leaf weights |
| `gamma` = 0.1 | Min split gain | Prevents splits that provide negligible ranking improvement |

**Retraining Cadence**

| Task | Frequency | Duration | Infrastructure |
|------|-----------|----------|----------------|
| Feature refresh | Daily | ~30 min | Batch Spark job reading from ClickHouse |
| Model retraining | Weekly (Sunday 02:00 UTC) | ~2 hours | 8-core CPU instance, ~50GB training data |
| Human rating collection | Continuous (target: 500 ratings/week) | — | Rating platform (internal tool) |
| Synthetic label generation | Daily | ~1 hour | GPT-4 API calls for 3K (query, agent) pairs |
| A/B evaluation | Continuous | — | 5% traffic to new model, 95% to current model |
| Full model deployment | Weekly (after A/B validation) | ~10 min | Blue/green deployment of XGBoost model file |

---

## 12. AgentRank: The Scoring Algorithm (AVERT Framework)

AgentRank is the precomputed quality score assigned to every agent in the registry. It is computed offline (not at query time), stored in PostgreSQL, Tantivy, and ClickHouse, and used as one of the 25 features in the LambdaMART ranking model.

AgentRank is designed to answer a single question: **"How good is this agent, independent of any specific query?"**

Query-dependent relevance scoring happens at search time (Sections 11.4-11.5). AgentRank captures query-independent quality: is the agent well-maintained, trustworthy, properly documented, reliably available, and respected by the ecosystem?

### 12.1 Framework Overview: AVERT

AVERT stands for **Availability, Verification, Expertise, Reputation, Trust**. These five dimensions capture the orthogonal quality signals for an agent:

```
AgentRank(a) = 100 × (  0.25 × S_A(a)        ← Availability
                       + 0.20 × S_V(a)        ← Verification
                       + 0.20 × S_E(a)        ← Expertise
                       + 0.20 × S_R(a)        ← Reputation
                       + 0.15 × S_T(a)  )     ← Trust

Where:
  S_A, S_V, S_E, S_R, S_T ∈ [0, 1]    (each sub-score is normalized to [0,1])
  AgentRank ∈ [0, 100]                  (final score is scaled to [0,100])
  
  Weights sum to: 0.25 + 0.20 + 0.20 + 0.20 + 0.15 = 1.00
```

```
┌──────────────────────────────────────────────────────────────────────────┐
│                     AVERT FRAMEWORK OVERVIEW                             │
│                                                                          │
│                        ┌───────────────┐                                 │
│                        │  AgentRank    │                                 │
│                        │  [0 — 100]    │                                 │
│                        └───────┬───────┘                                 │
│                                │                                         │
│         ┌──────────┬──────────┼──────────┬──────────┐                   │
│         │          │          │          │          │                   │
│         ▼          ▼          ▼          ▼          ▼                   │
│  ┌────────────┐┌────────────┐┌────────────┐┌────────────┐┌──────────┐  │
│  │ AVAILABILITY││VERIFICATION││ EXPERTISE  ││ REPUTATION ││  TRUST   │  │
│  │   S_A       ││   S_V       ││   S_E      ││   S_R      ││   S_T    │  │
│  │  (0.25)    ││  (0.20)    ││  (0.20)    ││  (0.20)    ││  (0.15)  │  │
│  │            ││            ││            ││            ││          │  │
│  │ • Uptime   ││ • Card     ││ • Skill    ││ • PageRank ││ • TLS    │  │
│  │ • Latency  ││   complete ││   depth    ││ • Selection││ • Auth   │  │
│  │ • Reliabil.││ • Capability││ • Descript.││   quality  ││ • Provena│  │
│  │ • Consist. ││   verified ││ • Domain   ││ • Delegat. ││ • History│  │
│  │            ││ • Response ││   specific.││   frequency││ • Stabil.│  │
│  │            ││   valid    ││ • Examples ││ • Provider ││          │  │
│  │            ││ • Schema   ││ • Tags     ││   reputat. ││          │  │
│  │            ││   conform  ││            ││            ││          │  │
│  └────────────┘└────────────┘└────────────┘└────────────┘└──────────┘  │
│                                                                          │
│   Weight rationale:                                                      │
│   • Availability (0.25) highest: an agent that's down is useless         │
│   • Verification, Expertise, Reputation (0.20 each): equally important   │
│     but secondary to being available                                     │
│   • Trust (0.15) lowest in composite because trust also acts as a        │
│     multiplicative floor at query time (Section 12.9)                    │
└──────────────────────────────────────────────────────────────────────────┘
```

**Weight Rationale**

- **Availability (0.25)**: Highest weight because an agent that is frequently down, slow, or unreliable provides negative value regardless of how well-documented or trusted it is. The worst user experience is selecting a highly-ranked agent that fails at connection time.
- **Verification (0.20)**: Metadata completeness and capability verification are strong signals of a well-maintained agent. Incomplete agent cards correlate strongly with abandoned or experimental agents.
- **Expertise (0.20)**: Measures how deep and specific the agent's capabilities are. Specialists should rank higher than generalists for domain-specific queries.
- **Reputation (0.20)**: The PageRank-inspired component. Agents referenced and delegated to by other trusted agents are likely higher quality — this is the "wisdom of the ecosystem" signal.
- **Trust (0.15)**: Lowest in the composite score because trust also acts as a **multiplicative floor** at query time (Section 12.9). Giving trust a high composite weight AND a multiplicative floor would double-penalize low-trust agents. The 0.15 composite weight provides a gentle boost for highly trusted agents without excessive penalty.

---

### 12.2 Availability Score (S_A)

The Availability Score measures whether an agent can be reached, responds within SLA, and does so consistently.

```
S_A(a) = 0.40 × U₃₀(a) + 0.25 × (1 - L_p95_norm(a)) + 0.20 × R₇(a) + 0.15 × C(a)
```

**Variable Definitions:**

| Variable | Name | Formula | Range | Description |
|----------|------|---------|-------|-------------|
| `U₃₀` | 30-day Uptime Ratio | `successful_probes₃₀ / total_probes₃₀` | [0, 1] | Fraction of liveness probes that returned a healthy response in the last 30 days. A probe is "successful" if it completes at the appropriate liveness level (L1-L4) within the timeout window. |
| `L_p95_norm` | Normalized P95 Latency | `min(p95_latency_ms / 5000, 1.0)` | [0, 1] | The agent's 95th-percentile response latency over the last 30 days, normalized to [0,1] using a 5-second ceiling. Agents with P95 > 5s are capped at 1.0 (worst possible). The score uses `(1 - L_p95_norm)` so lower latency = higher score. |
| `R₇` | 7-day Reliability Ratio | `successful_connections₇ / total_connection_attempts₇` | [0, 1] | Fraction of connection attempts (via the connect broker or direct) that succeeded in the last 7 days. More recent and more granular than uptime. Uses a shorter window to be more responsive to degradation. Falls back to `U₃₀` if no connection data exists. |
| `C` | Consistency Score | `1 - (σ(response_times₃₀) / μ(response_times₃₀))` if μ > 0 else 0 | [0, 1] | Coefficient of variation of response times over 30 days, inverted so that consistent agents score higher. An agent with μ=100ms and σ=10ms scores `1 - 0.1 = 0.9`. An agent with μ=100ms and σ=90ms scores `1 - 0.9 = 0.1`. Clamped to [0, 1]. |

**Example Computation:**

Agent: `acme-translator`

| Metric | Raw Value | Computed Variable | Weighted Contribution |
|--------|-----------|-------------------|-----------------------|
| Successful probes (30d) | 8,547 of 8,640 | U₃₀ = 8547/8640 = 0.9892 | 0.40 × 0.9892 = **0.3957** |
| P95 latency (30d) | 320ms | L_p95_norm = 320/5000 = 0.064; (1-0.064) = 0.936 | 0.25 × 0.936 = **0.2340** |
| Connect success (7d) | 412 of 430 | R₇ = 412/430 = 0.9581 | 0.20 × 0.9581 = **0.1916** |
| Response time μ=250ms, σ=45ms | CV = 0.18 | C = 1 - 0.18 = 0.82 | 0.15 × 0.82 = **0.1230** |
| | | **S_A** | **0.9443** |

---

### 12.3 Verification Score (S_V)

The Verification Score measures how well the agent's metadata is filled out, whether its capabilities have been independently verified, and whether its responses conform to declared schemas.

```
S_V(a) = 0.30 × V_card(a) + 0.25 × V_cap(a) + 0.25 × V_resp(a) + 0.20 × V_schema(a)
```

**Variable Definitions:**

| Variable | Name | Formula | Range | Description |
|----------|------|---------|-------|-------------|
| `V_card` | Card Completeness | Weighted sum of filled fields | [0, 1] | Measures what fraction of agent card fields are populated, with fields weighted by importance |
| `V_cap` | Capability Verification | `verified_skills / total_skills` | [0, 1] | Fraction of declared skills that have been independently verified by the benchmark system |
| `V_resp` | Response Validity | `valid_responses / total_probed_responses` | [0, 1] | Fraction of probed responses that passed A2A protocol validation (correct JSON-RPC, valid task states, proper error codes) |
| `V_schema` | Schema Conformance | `schema_valid_cards / total_card_fetches` | [0, 1] | Fraction of fetched agent cards that pass JSON Schema validation without errors or warnings |

**Card Completeness Field Weights**

`V_card` is not a simple "count of non-null fields". Fields are weighted by their contribution to discoverability and caller utility:

| Field | Weight | Rationale |
|-------|--------|-----------|
| `name` | 0.15 | Essential for identification; always expected |
| `description` | 0.12 | Primary narrative signal for search |
| `url` (endpoint) | 0.12 | Required for connection; useless without it |
| `skills` (≥1 skill) | 0.15 | Core capability declaration |
| `skills[].name` | 0.08 | Skill-level identification |
| `skills[].description` | 0.06 | Skill-level detail |
| `skills[].tags` (≥1 tag) | 0.05 | Taxonomy alignment |
| `skills[].examples` (≥1) | 0.04 | Intent matching support |
| `skills[].inputModes` | 0.03 | I/O compatibility signal |
| `skills[].outputModes` | 0.03 | I/O compatibility signal |
| `provider.organization` | 0.06 | Provider identification |
| `provider.url` | 0.03 | Provider verification support |
| `authentication` (≥1 scheme) | 0.05 | Auth compatibility signal |
| `capabilities` object | 0.03 | Feature flag signal (streaming, push, etc.) |
| **Total** | **1.00** | |

```
V_card(a) = Σ weight(field) × present(field)

Where present(field) = 1 if the field is non-null and non-empty, 0 otherwise.
```

**Capability Verification Process (Phase 2)**

In Phase 1, `V_cap` defaults to 0 for all agents (no capabilities have been verified). In Phase 2, the benchmark system will verify capabilities through safe, non-destructive probes:

```
┌──────────────────────────────────────────────────────────────────┐
│               CAPABILITY VERIFICATION FLOW                        │
│                                                                   │
│  For each declared skill:                                         │
│                                                                   │
│  1. Select test case from skill-specific test suite               │
│     (e.g., for "text-translation": send known EN→FR pair)        │
│                                                                   │
│  2. Send A2A task request with test input                         │
│     - Timeout: 30s                                                │
│     - Retry: 1 attempt                                            │
│     - Flag: benchmark=true (allows providers to detect)           │
│                                                                   │
│  3. Evaluate response:                                            │
│     - Did the agent respond within timeout? (binary)              │
│     - Did the response match expected output format? (binary)     │
│     - Did the response content pass quality threshold? (0-1)      │
│     - Did the agent handle the task state lifecycle? (binary)     │
│                                                                   │
│  4. Assign verification status:                                   │
│     - VERIFIED: all checks pass                                   │
│     - PARTIAL: some checks pass                                   │
│     - UNVERIFIED: not yet tested or all checks fail               │
│                                                                   │
│  V_cap = count(VERIFIED) / count(all_skills)                      │
│  Partial skills contribute 0.5 each.                              │
└──────────────────────────────────────────────────────────────────┘
```

---

### 12.4 Expertise Score (S_E)

The Expertise Score measures how deep, specific, and well-documented an agent's capabilities are. The key design insight: **specialists should rank higher than generalists for domain-specific queries, and the expertise score captures this preference offline.**

```
S_E(a) = 0.30 × E_depth(a) + 0.25 × E_desc(a) + 0.20 × E_domain(a) + 0.15 × E_examples(a) + 0.10 × E_tags(a)
```

**Variable Definitions:**

| Variable | Name | Formula | Range | Description |
|----------|------|---------|-------|-------------|
| `E_depth` | Skill Depth | `min(avg_skill_description_tokens / 100, 1.0)` | [0, 1] | Average token count of skill descriptions, normalized. Agents with detailed skill descriptions (>100 tokens on average) score 1.0. Measures the depth of capability documentation. |
| `E_desc` | Description Quality | `min(description_tokens / 200, 1.0) × coherence_score` | [0, 1] | Product of description length (normalized to 200 tokens) and a coherence score (0-1) computed by a small classifier that detects keyword stuffing, repetition, and incoherent text. |
| `E_domain` | Domain Specificity | `1 - cos(embed(a), centroid_all)` | [0, 1] | **See key insight below.** Cosine distance from the agent's capability embedding to the centroid of all agent embeddings. Specialists are far from the centroid; generalists are near it. |
| `E_examples` | Examples Quality | `min(total_examples / 5, 1.0) × avg_example_length_norm` | [0, 1] | Combination of example count (capped at 5) and average example length (normalized). Having 5+ well-written examples maxes out this score. |
| `E_tags` | Tag Coverage | `min(total_unique_tags / 8, 1.0)` | [0, 1] | Number of unique tags across all skills, normalized. Having 8+ unique tags maxes out this score. |

**KEY INSIGHT: Domain Specificity via Embedding Distance**

```
E_domain(a) = 1 - cos(embed(a), centroid_all)

Where:
  embed(a)      = the capability_embedding (768d) of agent a
  centroid_all  = the mean embedding vector across all agents in the registry
  cos(u, v)     = cosine similarity = (u · v) / (||u|| × ||v||)
```

**Why this works:** The centroid of all agent embeddings represents the "average agent" — a bland, generic point in embedding space. Agents that are close to the centroid are generalists whose capability descriptions overlap with many other agents. Agents that are far from the centroid are specialists with unique, domain-specific language.

**Example:**

- Agent `generic-assistant` (description: "A general-purpose AI assistant that can help with many tasks") → embedding is close to centroid → `cos = 0.92` → `E_domain = 0.08` (low expertise)
- Agent `cardiology-ecg-analyzer` (description: "Analyzes 12-lead ECG recordings to detect arrhythmias, ST-segment changes, and conduction abnormalities") → embedding is far from centroid → `cos = 0.35` → `E_domain = 0.65` (high expertise)

**Why specialists should rank higher:**

1. Specialists are more useful for specific tasks. A user searching for "ECG analysis" wants the cardiology agent, not a generic assistant.
2. Specialists are harder to find (there are fewer of them), so surfacing them is higher value.
3. Generalists compete on other dimensions (availability, trust, reputation), so the expertise dimension should differentiate specialists.

**Centroid Recomputation Schedule:**

The centroid is recomputed daily as new agents enter the index. The recomputation is a single-pass mean over all capability embeddings (~10M × 768 floats = ~30GB) and completes in ~5 minutes on a single machine.

---

### 12.5 Reputation Score (S_R) — PageRank Component

The Reputation Score is the most structurally complex component. It combines a PageRank-inspired graph authority signal with behavioral signals from the ecosystem.

```
S_R(a) = 0.35 × G_auth(a) + 0.25 × Q_sel(a) + 0.20 × D_freq(a) + 0.20 × P_rep(a)
```

**Variable Definitions:**

| Variable | Name | Formula | Range | Description |
|----------|------|---------|-------|-------------|
| `G_auth` | Graph Authority (PageRank) | PageRank algorithm on agent graph | [0, 1] | Agent's authority score from the PageRank algorithm applied to the agent interaction graph. Normalized to [0,1] by dividing by max PageRank in the graph. |
| `Q_sel` | Selection Quality | `selected_and_succeeded / total_selected` | [0, 1] | Of the times this agent was selected from search results, how often did the subsequent connection succeed and the outcome was positive? Measures post-selection quality. |
| `D_freq` | Delegation Frequency | `min(delegation_in_degree / 50, 1.0)` | [0, 1] | How often other agents delegate tasks to this agent, normalized. Being delegated to by many agents is a strong ecosystem trust signal. |
| `P_rep` | Provider Reputation | `avg(AgentRank(siblings))` where siblings share the same provider | [0, 1] | Average AgentRank of other agents from the same provider, normalized to [0,1]. A provider with consistently high-quality agents lifts all their agents. |

**PageRank Formula for G_auth**

The PageRank algorithm is applied to the agent interaction graph, where edges represent delegation, reference, and invocation relationships.

```
PR(a) = (1 - d) / N + d × Σ(b→a) [ w(b,a) × PR(b) / out_degree(b) ]

Where:
  d           = 0.85  (damping factor — standard value)
  N           = total number of agents in the graph
  (b → a)     = edge from agent b to agent a
  w(b, a)     = edge weight (see below)
  out_degree(b) = weighted out-degree of agent b
  
  Edge weights:
  | Edge Type           | Weight |
  |---------------------|--------|
  | delegation          | 1.0    |
  | reference (citation)| 0.5    |
  | same_provider       | 0.2    |
  | benchmark_similar   | 0.1    |
  | taxonomy_sibling    | 0.05   |
```

**PageRank Computation:**

- **Algorithm**: Power iteration with convergence threshold ε = 10⁻⁶
- **Iterations**: Typically converges in 40-60 iterations for graphs under 10M nodes
- **Schedule**: Recomputed daily (batch job, ~15 minutes for 10M agents)
- **Storage**: Stored in PostgreSQL (`agents.pagerank_score`) and synced to Tantivy fast field

**Bootstrap Problem and Solutions**

When the platform is new, the interaction graph is sparse. Most agents have no incoming edges, so PageRank gives nearly uniform scores (≈ 1/N). This is the "cold start" problem for graph authority.

| Solution | When Applied | Effect |
|----------|-------------|--------|
| **Provider seed trust** | Phase 1 | Agents from verified providers (e.g., Google, Salesforce) receive a seed PageRank boost proportional to provider reputation. Implemented as a personalization vector in the PageRank computation. |
| **Crawl-link initialization** | Phase 1 | Agents that are referenced in other agents' documentation or link to each other via `cites` edges receive initial authority. These edges are cheapest to detect (no runtime data needed). |
| **Synthetic delegation edges** | Phase 1 | The benchmark system creates synthetic "delegation" edges when one agent successfully processes a task that another agent references. Low weight (0.3× normal delegation) to prevent bootstrapping from dominating. |
| **Behavioral signal fallback** | Phase 1 | When `G_auth` is near-uniform (std < 0.01), the reputation score falls back to a weighted average of `Q_sel`, `D_freq`, and `P_rep` with increased weights. |

**Personalized PageRank for Bootstrap**

```
PR(a) = (1 - d) × v(a) + d × Σ(b→a) [ w(b,a) × PR(b) / out_degree(b) ]

Where v(a) is the personalization vector:
  v(a) = provider_trust(a) / Σ provider_trust(all agents)
  
  provider_trust(a) = {
    1.0   if provider is Authoritative tier
    0.5   if provider is Verified tier
    0.2   if provider is Established tier
    0.05  if provider is Indexed tier (default)
  }
```

This ensures that even in a sparse graph, agents from trusted providers receive disproportionate authority, which then propagates through the graph as edges form.

---

### 12.6 Trust Score (S_T)

The Trust Score measures the security posture and trustworthiness of an agent's infrastructure, authentication, provenance, and operational history.

```
S_T(a) = 0.25 × T_tls(a) + 0.25 × T_auth(a) + 0.20 × T_prov(a) + 0.15 × T_hist(a) + 0.15 × T_stab(a)
```

**Variable Definitions:**

| Variable | Name | Formula | Range | Description |
|----------|------|---------|-------|-------------|
| `T_tls` | TLS Security | See scale below | [0, 1] | Quality of TLS configuration: certificate validity, protocol version, cipher strength, HSTS presence |
| `T_auth` | Auth Maturity | See scale below | [0, 1] | Sophistication of the agent's authentication scheme |
| `T_prov` | Provenance Attestation | See scale below | [0, 1] | Strength of the agent's identity and provenance claims |
| `T_hist` | Historical Trust | `days_since_first_healthy / max(365, days_since_first_healthy)` × `(1 - incident_rate)` | [0, 1] | Combination of how long the agent has been healthy and its trust incident rate |
| `T_stab` | Stability Score | `1 - (config_changes_30d / max(30, config_changes_30d))` | [0, 1] | How stable the agent's configuration is. Frequently changing agents (card updates, endpoint changes, auth rotations) score lower because instability correlates with unreliability. |

**TLS Security Scale (T_tls)**

| Configuration | Score | Criteria |
|--------------|-------|----------|
| TLS 1.3 + valid cert + HSTS + strong cipher suite | 1.0 | Best-practice TLS configuration |
| TLS 1.3 + valid cert + no HSTS | 0.8 | Good but missing HSTS |
| TLS 1.2 + valid cert | 0.6 | Acceptable but using older protocol |
| TLS 1.2 + expired/self-signed cert | 0.3 | Security concerns |
| TLS 1.0/1.1 or no TLS | 0.0 | Unacceptable; agent should be flagged |

**Auth Maturity Scale (T_auth)**

| Auth Scheme | Score | Rationale |
|------------|-------|-----------|
| `mTLS` (mutual TLS) | **1.0** | Strongest auth: both client and server authenticated at transport level |
| `OAuth 2.0` with PKCE | **0.85** | Strong auth with industry-standard token flow |
| `OAuth 2.0` (basic) | **0.70** | Good auth but may be vulnerable to token theft without PKCE |
| `API Key` (in header) | **0.30** | Weak but ubiquitous; susceptible to key leakage |
| `API Key` (in query param) | **0.15** | Very weak; keys visible in logs and referrer headers |
| `None` (unauthenticated) | **0.00** | No authentication; anyone can call the agent |

**Provenance Attestation Scale (T_prov)**

| Provenance Level | Score | Criteria |
|-----------------|-------|----------|
| Signed agent card + DNS TXT verification + code repo linkage | 1.0 | Maximum provenance: cryptographic proof of identity + verifiable deployment source |
| Signed agent card + DNS TXT verification | 0.8 | Strong provenance: cryptographic proof of identity |
| DNS TXT verification only | 0.6 | Moderate provenance: domain control verified |
| Well-known endpoint serves valid card | 0.4 | Basic provenance: agent is at expected location |
| Third-party registry reference only | 0.2 | Weak provenance: someone else claims this agent exists |
| No verifiable provenance | 0.0 | No provenance: treat all claims with skepticism |

---

### 12.7 Score Computation Pipeline

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                    AGENTRANK COMPUTATION PIPELINE                              │
│                                                                               │
│  ┌───────────────┐                                                            │
│  │  DATA SOURCES  │                                                            │
│  │               │                                                            │
│  │ • Crawl data  │     ┌───────────────────────────────────────────┐          │
│  │   (card meta) │────▶│  5 PARALLEL SCORERS                      │          │
│  │ • Liveness    │     │                                           │          │
│  │   probes      │     │  ┌──────────┐ ┌──────────┐ ┌──────────┐ │          │
│  │ • Benchmark   │     │  │ Avail.   │ │ Verif.   │ │ Expert.  │ │          │
│  │   results     │     │  │ Scorer   │ │ Scorer   │ │ Scorer   │ │          │
│  │ • Outcome     │     │  │ (S_A)    │ │ (S_V)    │ │ (S_E)    │ │          │
│  │   telemetry   │     │  └────┬─────┘ └────┬─────┘ └────┬─────┘ │          │
│  │ • Graph data  │     │       │             │             │       │          │
│  │ • Registry    │     │  ┌────┴─────┐ ┌────┴─────┐              │          │
│  │   metadata    │     │  │ Reput.   │ │ Trust    │              │          │
│  │ • TLS/cert    │     │  │ Scorer   │ │ Scorer   │              │          │
│  │   data        │     │  │ (S_R)    │ │ (S_T)    │              │          │
│  └───────────────┘     │  └────┬─────┘ └────┬─────┘              │          │
│                        │       │             │                    │          │
│                        └───────┼─────────────┼────────────────────┘          │
│                                │             │                               │
│                                ▼             ▼                               │
│                        ┌──────────────────────┐                              │
│                        │   AVERT COMBINER      │                              │
│                        │                       │                              │
│                        │   AgentRank =         │                              │
│                        │     100 × (0.25×S_A   │                              │
│                        │          + 0.20×S_V   │                              │
│                        │          + 0.20×S_E   │                              │
│                        │          + 0.20×S_R   │                              │
│                        │          + 0.15×S_T)  │                              │
│                        │                       │                              │
│                        │   Output: [0 — 100]   │                              │
│                        └──────────┬────────────┘                              │
│                                   │                                           │
│                                   ▼                                           │
│                        ┌──────────────────────┐                              │
│                        │   SCORE SINKS         │                              │
│                        │                       │                              │
│                        │   • PostgreSQL        │                              │
│                        │     (agents table)    │                              │
│                        │   • Tantivy           │                              │
│                        │     (fast field)      │                              │
│                        │   • ClickHouse        │                              │
│                        │     (historical)      │                              │
│                        └──────────────────────┘                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Computation Schedule**

| Component | Schedule | Duration | Dependencies |
|-----------|----------|----------|-------------|
| S_A (Availability) | Every 4 hours | ~10 min for 10M agents | Liveness probe data (ClickHouse) |
| S_V (Verification) | Every 12 hours | ~20 min | Card metadata (PostgreSQL), benchmark results |
| S_E (Expertise) | Daily | ~30 min | Embeddings (Qdrant), card metadata |
| S_R (Reputation) | Daily | ~15 min (PageRank) + ~5 min (behavioral) | Graph (Neo4j), outcome telemetry (ClickHouse) |
| S_T (Trust) | Every 12 hours | ~15 min | TLS scan data, auth probes, provenance checks |
| AVERT Combiner | After any sub-scorer completes | ~5 min | All 5 sub-scores |
| Score Sink Sync | After combiner | ~10 min | Combined AgentRank score |

---

### 12.8 Score Decay

Agent scores should decay over time if the agent is not re-crawled, not probed, or not interacted with. This prevents stale data from persisting in rankings.

```
decayed_score(a, t) = current_score(a) × e^(-λ × Δt)

Where:
  λ  = 0.001 (decay constant, in hours⁻¹)
  Δt = hours since the score was last computed
  
  Half-life: t½ = ln(2) / λ = 0.693 / 0.001 ≈ 693 hours ≈ 28.9 days
```

**Decay Schedule**

| Time Since Last Score | Δt (hours) | Decay Factor: e^(-0.001 × Δt) | Effective Score (if base=85.0) |
|-----------------------|-----------|-------------------------------|-------------------------------|
| 0 hours (fresh) | 0 | 1.000 | 85.0 |
| 24 hours (1 day) | 24 | 0.976 | 83.0 |
| 168 hours (1 week) | 168 | 0.846 | 71.9 |
| 720 hours (30 days) | 720 | 0.487 | 41.4 |
| 1,440 hours (60 days) | 1440 | 0.237 | 20.1 |
| 2,160 hours (90 days) | 2160 | 0.115 | 9.8 |

**Design Rationale:**

- **28.9-day half-life**: Chosen so that agents re-crawled within their normal cadence (weekly for high-priority, bi-weekly for medium) experience minimal decay (<15%). Agents not crawled for 30+ days lose ~50% of their score, creating strong pressure to maintain healthy, reachable endpoints.
- **Exponential decay (not linear)**: Exponential decay is smoother and does not create cliff edges. A linear decay with a 30-day cutoff would cause agents to jump from full score to zero, which produces ranking instability.
- **Score floor**: Decayed scores are clamped at 1.0 to prevent division-by-zero issues downstream. An agent with a decayed score of 1.0 is effectively de-ranked but not delisted.

---

### 12.9 Query-Time Score Combination

At query time, the precomputed AgentRank is combined with query-dependent signals to produce the final ranking score. This happens in Stage 3 (Lightweight Scoring) and is refined by the LambdaMART model in Stage 5.

**Stage 3 Formula:**

```
final_score(q, a) = eligibility_gate(a, q) 
                  × trust_floor(a, q) 
                  × weighted_sum(q, a)
```

**Eligibility Gate:**

```
eligibility_gate(a, q) = {
  1.0  if agent a passes all 9 eligibility filters for query q
  0.0  otherwise (agent is removed from results)
}
```

**Trust Floor:**

```
trust_floor(a, q) = {
  1.0                        if trust_tier(a) ≥ min_trust_tier(q)
  0.5                        if trust_tier(a) = min_trust_tier(q) - 1
  0.0                        if trust_tier(a) < min_trust_tier(q) - 1
}
```

The trust floor is a **multiplicative penalty**, not a filter. An agent one tier below the minimum trust threshold is not removed (unlike eligibility filters) but receives a 50% score penalty. This ensures that low-trust agents are not completely invisible but are strongly deprioritized.

**Weighted Sum Components:**

```
weighted_sum(q, a) = Σⱼ wⱼ × componentⱼ(q, a)
```

| # | Component | Weight | Source | Description |
|---|-----------|--------|--------|-------------|
| 1 | `rrf_score` | 0.25 | Stage 2 RRF fusion | Combined lexical + semantic + graph retrieval score |
| 2 | `agent_rank_decayed` | 0.20 | AgentRank with decay | Precomputed quality score, time-decayed |
| 3 | `cross_encoder_score` (normalized) | 0.15 | Stage 4 re-ranking | Cross-encoder relevance (normalized to [0,1]) |
| 4 | `name_match_bonus` | 0.08 | Query processing | Exact/fuzzy name match bonus (1.0 for exact, 0.5 for fuzzy, 0 otherwise) |
| 5 | `skill_match_score` | 0.08 | Query processing | Best skill name match against query entities |
| 6 | `domain_match_score` | 0.07 | Query processing | Taxonomy domain match score |
| 7 | `freshness_score` | 0.05 | Registry | Recency of last successful crawl (exponential decay) |
| 8 | `popularity_score` | 0.07 | Analytics | 7-day connect attempt volume (log-normalized) |
| 9 | `provider_diversity_penalty` | 0.05 | Diversity | Penalty for multiple agents from same provider in result set |
| | **Total** | **1.00** | | |

---

### 12.10 Discoverability Score (Provider-Facing)

The Discoverability Score is a provider-facing metric exposed in the Agent Search Console. It tells providers how well their agents are optimized for discovery and what they can do to improve.

**This is the "Agent SEO" mechanism.** Just as Google's Search Console shows webmasters how to improve their site's search performance, the Agent Search Console shows agent providers how to improve their agents' discoverability.

```
Discoverability(a) = 0.25 × metadata_completeness(a)
                   + 0.20 × skill_specificity(a)
                   + 0.15 × examples_quality(a)
                   + 0.10 × crawlability(a)
                   + 0.10 × canonicalization(a)
                   + 0.10 × signed_provenance(a)
                   + 0.10 × docs_quality(a)
```

| Component | Weight | Definition | How to Improve |
|-----------|--------|------------|----------------|
| `metadata_completeness` | 0.25 | Same as V_card (Section 12.3) | Fill in all agent card fields, especially skills, tags, and examples |
| `skill_specificity` | 0.20 | Average E_domain across skills (Section 12.4). Higher = more specific. | Write specific, technical skill descriptions; avoid generic language |
| `examples_quality` | 0.15 | Number and quality of example queries/use cases per skill | Add 3-5 diverse, realistic example queries per skill |
| `crawlability` | 0.10 | Whether the agent card is at a well-known URL, responds quickly, and has no crawl errors | Serve agent card at `/.well-known/agent.json` with proper CORS headers |
| `canonicalization` | 0.10 | Whether the agent has a single canonical URL, proper redirect handling, and version consistency | Use canonical URLs, implement proper redirects, maintain version consistency |
| `signed_provenance` | 0.10 | Whether the agent card is cryptographically signed and the domain is verified | Sign the agent card with a verified key; complete DNS TXT verification |
| `docs_quality` | 0.10 | Quality and accessibility of linked documentation | Link to comprehensive, accessible documentation from the agent card |

**Agent Search Console Mock**

```
┌──────────────────────────────────────────────────────────────────────────┐
│  Agent Search Console — acme-translator                                   │
│  Provider: Acme Corp (Verified ✓)                                         │
│                                                                          │
│  ┌────────────────────────────────────────────────────────────────────┐   │
│  │  DISCOVERABILITY SCORE                              78 / 100       │   │
│  │  ████████████████████████████████████████████░░░░░░░░░░░░░░░░░░░  │   │
│  └────────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  Component Breakdown:                                                    │
│                                                                          │
│  Metadata Completeness    ██████████████████████████████░░░  92%         │
│  Skill Specificity        ████████████████████████░░░░░░░░░  72%         │
│  Examples Quality         ████████████████░░░░░░░░░░░░░░░░░  55%  ⚠     │
│  Crawlability             ██████████████████████████████████  100%        │
│  Canonicalization         ████████████████████████████░░░░░░  85%         │
│  Signed Provenance        ████████████████████░░░░░░░░░░░░░  60%  ⚠     │
│  Documentation Quality    ██████████████████████████░░░░░░░  78%         │
│                                                                          │
│  ⚠ Recommendations:                                                      │
│  1. Add more example queries to "text-translation" skill (currently 2,   │
│     recommend 5+)                                                        │
│  2. Sign your agent card with a verifiable key to improve provenance     │
│  3. Add inputModes and outputModes to all skills                         │
│                                                                          │
│  AgentRank: 85.2 / 100   |   Trust Tier: Verified                       │
│  Search Impressions (7d): 1,247  |  Clicks: 312  |  CTR: 25.0%          │
│  Connection Attempts (7d): 189   |  Success Rate: 95.8%                  │
│                                                                          │
│  Ranking Factors (debug view):                                           │
│  S_A = 0.944  S_V = 0.871  S_E = 0.812  S_R = 0.756  S_T = 0.823      │
│                                                                          │
│  Recent Crawl History:                                                   │
│  2026-03-23 04:12 UTC  ✓ Crawled successfully (234ms)                   │
│  2026-03-22 04:15 UTC  ✓ Crawled successfully (198ms)                   │
│  2026-03-21 04:11 UTC  ✓ Crawled successfully (245ms)                   │
│  2026-03-20 04:14 UTC  ⚠ Card validation warning: missing outputModes   │
│  2026-03-19 04:12 UTC  ✓ Crawled successfully (201ms)                   │
└──────────────────────────────────────────────────────────────────────────┘
```

---

### 12.11 Learning-to-Rank Roadmap

The AVERT framework is designed as the **Phase 1 transparent heuristic ranker**. The roadmap anticipates a gradual transition to learned ranking as training data accumulates.

**Phase 1: Transparent Heuristic (Launch — Month 0-6)**

- AVERT formula with fixed, hand-tuned weights
- All weights published in the Agent Search Console (providers can see why they rank where they do)
- Feature importances are deterministic and explainable
- No machine learning in the ranking path (deterministic scoring)
- **Principle**: If the team cannot explain why a result ranked where it did, the system is not ready for black-box ranking

**Phase 2: Offline LTR (Month 6-12)**

- LambdaMART model trained on accumulated data:
  - Human-judged query sets (~10K rated pairs)
  - Click-through and connection data (~500K implicit pairs)
  - Benchmark labels (~5K verified capabilities)
  - Synthetic LLM judgments (~50K labeled pairs)
- AVERT score becomes one feature among 25 in the LTR model
- A/B testing framework validates that LTR outperforms heuristic on key metrics:
  - NDCG@10 (primary metric)
  - Connection success rate (secondary metric)
  - Provider diversity (guardrail metric — must not regress)
- Rollback plan: instant switch back to heuristic if LTR degrades any guardrail metric

**Phase 3: Contextual Ranking (Month 12+)**

- Ranking incorporates caller context:
  - Tenant preferences (enterprise customers configure vertical biases)
  - Calling agent context (the orchestrator's declared capabilities influence what "complementary" means)
  - Vertical priors (healthcare queries get healthcare-optimized ranking)
  - Policy posture (high-security callers see security-optimized rankings)
- Model architecture evolves from pointwise LambdaMART to listwise neural ranking
- Explore/exploit framework: 5% of traffic serves randomized results to collect unbiased training data
- Online learning: model weights update hourly based on streaming outcome data

---

## 13. Query Engine

The Query Engine is the front door of the search system. It receives raw queries from callers (agents, orchestrators, humans), transforms them into structured search plans, executes those plans against the retrieval backends, and assembles the final result set.

### 13.1 Query Understanding Pipeline

Raw queries from agents and humans are messy: they contain abbreviations, implicit intent, ambiguous entities, and unstated constraints. The Query Understanding Pipeline transforms raw queries into structured, enriched search plans that maximize retrieval quality.

```
┌──────────────────────────────────────────────────────────────────────────┐
│                   QUERY UNDERSTANDING PIPELINE                            │
│                                                                          │
│   Input: "I need a k8s agent that can deploy containers, supports        │
│           OAuth, and works in EU regions"                                │
│                                                                          │
│   ┌────────────────┐                                                     │
│   │ 1. INTENT       │  Classify query intent                             │
│   │    CLASSIFIER    │  → capability_search (0.92 confidence)            │
│   │    (Transformer) │  → also: constraint_search (0.78)                 │
│   └────────┬───────┘                                                     │
│            │                                                             │
│            ▼                                                             │
│   ┌────────────────┐                                                     │
│   │ 2. ENTITY       │  Extract structured entities                       │
│   │    EXTRACTOR     │  → capabilities: ["kubernetes deployment",        │
│   │    (NER + Rules) │     "container deployment"]                       │
│   │                  │  → auth_requirement: "oauth"                      │
│   │                  │  → region_constraint: "EU"                        │
│   │                  │  → agent_name: null                               │
│   └────────┬───────┘                                                     │
│            │                                                             │
│            ▼                                                             │
│   ┌────────────────┐                                                     │
│   │ 3. QUERY        │  Expand abbreviations and synonyms                 │
│   │    EXPANDER      │  → "k8s" expanded to ["k8s", "kubernetes"]       │
│   │    (Synonym Map  │  → "OAuth" expanded to ["oauth", "oauth2",       │
│   │     + Acronyms)  │     "oauth2.0"]                                  │
│   │                  │  → "EU" expanded to ["eu-west-1", "eu-central-1",│
│   │                  │     "eu-north-1", "europe"]                      │
│   └────────┬───────┘                                                     │
│            │                                                             │
│            ▼                                                             │
│   ┌────────────────┐                                                     │
│   │ 4. EXECUTION     │  Build search plan                                │
│   │    PLANNER       │  → lexical_query: "kubernetes deployment          │
│   │                  │     container oauth"                              │
│   │                  │  → semantic_query: "kubernetes agent that can     │
│   │                  │     deploy containers"                            │
│   │                  │  → filters: {auth: ["oauth*"], region: ["eu-*"]} │
│   │                  │  → boost_hints: {skill_tags: "kubernetes"}       │
│   └────────────────┘                                                     │
│                                                                          │
│   Output: SearchPlan {                                                   │
│     intent: "capability_search",                                         │
│     lexical_query: "kubernetes deployment container oauth",              │
│     semantic_query: "kubernetes agent that can deploy containers",       │
│     entities: {capabilities: [...], auth: [...], region: [...]},         │
│     filters: {auth_schemes: ["oauth*"], regions: ["eu-*"]},             │
│     boost_hints: {skill_tags: ["kubernetes", "container"]},              │
│     confidence: 0.92                                                     │
│   }                                                                      │
└──────────────────────────────────────────────────────────────────────────┘
```

**Intent Types**

The Intent Classifier categorizes queries into five types that influence how the search plan is constructed:

| Intent Type | Description | Example Query | Search Plan Effect |
|-------------|------------|---------------|-------------------|
| `capability_search` | User wants an agent that can do something specific | "translate documents from English to French" | Boost skill name/description fields; emphasize capability embedding |
| `name_lookup` | User knows the agent's name and wants to find it | "acme-translator" | Boost name field 5×; fall back to exact match on agent_id |
| `provider_search` | User wants agents from a specific provider | "Google Cloud agents" | Boost provider_org field; filter by provider domain |
| `constraint_search` | User specifies technical constraints (auth, region, etc.) | "agents with mTLS in us-east-1" | Extract constraints into eligibility filters; reduce semantic weight |
| `exploratory_browse` | User is exploring what's available in a domain | "what AI agents exist for healthcare?" | Broaden semantic search; increase diversity weight; reduce precision requirements |

**Entity Extraction Example**

Input: `"I need a k8s agent that can deploy containers, supports OAuth, and works in EU regions"`

| Entity Type | Extracted Value | Extraction Method |
|-------------|----------------|-------------------|
| `capability` | `["kubernetes deployment", "container deployment"]` | NER model + noun phrase chunking |
| `auth_requirement` | `"oauth"` | Keyword matching against auth vocabulary |
| `region_constraint` | `"EU"` | Keyword matching against region vocabulary |
| `agent_name` | `null` | No agent name detected |
| `provider` | `null` | No provider name detected |
| `modality` | `null` | No modality constraint detected |
| `price_constraint` | `null` | No price constraint detected |
| `domain` | `"infrastructure"` | Inferred from "k8s" and "deploy" → infrastructure taxonomy |

**Synonym and Acronym Expansion**

The Query Expander uses the same synonym map as the text analysis pipeline (Section 11.2.1) plus an additional acronym expansion layer for region names, protocol abbreviations, and technology nicknames:

| Category | Original | Expanded Forms |
|----------|----------|---------------|
| Technology | `k8s` | `kubernetes`, `k8s` |
| Auth | `OAuth` | `oauth`, `oauth2`, `oauth2.0`, `openid connect` |
| Region | `EU` | `eu-west-1`, `eu-west-2`, `eu-central-1`, `eu-north-1`, `europe` |
| Protocol | `A2A` | `agent-to-agent`, `a2a`, `agent2agent` |
| Protocol | `MCP` | `model-context-protocol`, `mcp` |
| AI/ML | `RAG` | `retrieval-augmented-generation`, `rag` |
| Cloud | `GCP` | `google-cloud-platform`, `google cloud`, `gcp` |
| Cloud | `AWS` | `amazon-web-services`, `amazon web services`, `aws` |

---

### 13.2 Hybrid Search Execution

The Query Engine orchestrates the full 6-stage pipeline (Section 11.1) with precise latency management. The execution is designed for maximum parallelism: independent operations run concurrently, and dependent operations are pipelined.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                      HYBRID SEARCH EXECUTION FLOW                               │
│                                                                                 │
│   T=0ms  Query Understanding (Intent + Entity + Expansion)                      │
│          ┌──────────────────────────────────────────┐                           │
│          │  Intent Classifier  (2ms)                 │                           │
│          │  Entity Extractor   (1ms)                 │                           │
│          │  Query Expander     (1ms)                 │                           │
│          └──────────────────────────────────────────┘                           │
│                            │                                                    │
│          T=4ms             ▼                                                    │
│          ┌─────────────────┬─────────────────┬──────────────────┐               │
│          │   PARALLEL       │   PARALLEL       │   PARALLEL       │               │
│          │                 │                 │                 │               │
│          │  ┌───────────┐  │  ┌───────────┐  │  ┌───────────┐  │               │
│          │  │ Tantivy    │  │  │ Qdrant     │  │  │ PostgreSQL │  │               │
│          │  │ BM25       │  │  │ Multi-Vec  │  │  │ PG Filter  │  │               │
│          │  │ Search     │  │  │ kNN Search │  │  │ Precompute │  │               │
│          │  │ (top-500)  │  │  │ (top-500)  │  │  │ (bitmasks) │  │               │
│          │  │            │  │  │            │  │  │            │  │               │
│          │  │ ~6ms       │  │  │ ~8ms       │  │  │ ~3ms       │  │               │
│          │  └─────┬─────┘  │  └─────┬─────┘  │  └─────┬─────┘  │               │
│          │        │        │        │        │        │        │               │
│          └────────┼────────┴────────┼────────┴────────┼────────┘               │
│                   │                 │                 │                         │
│          T=12ms   ▼                 ▼                 ▼                         │
│          ┌──────────────────────────────────────────────────────┐               │
│          │              RRF FUSION (k=60)                        │               │
│          │  Lexical results + Semantic results → merged list    │               │
│          │  + Graph retrieval (from top-20 seeds) → merged      │               │
│          │  Output: ~1,000 fused candidates                     │               │
│          │  ~2ms                                                │               │
│          └──────────────────────────────────┬───────────────────┘               │
│                                             │                                   │
│          T=14ms                             ▼                                   │
│          ┌──────────────────────────────────────────────────────┐               │
│          │         ELIGIBILITY FILTERING (bit-vector AND)        │               │
│          │  1,000 → ~400 candidates                             │               │
│          │  ~1ms                                                │               │
│          └──────────────────────────────────┬───────────────────┘               │
│                                             │                                   │
│          T=15ms                             ▼                                   │
│          ┌──────────────────────────────────────────────────────┐               │
│          │      LIGHTWEIGHT SCORING (RRF + AgentRank + Trust)   │               │
│          │  400 → top-100 candidates                            │               │
│          │  ~2ms                                                │               │
│          └────────────────┬─────────────────────────────────────┘               │
│                           │                                                     │
│          T=17ms           ▼                                                     │
│          ┌────────────────┬──────────────────────────────┐                      │
│          │  PARALLEL       │  PARALLEL                    │                      │
│          │                │                              │                      │
│          │  ┌───────────┐ │  ┌────────────────────────┐  │                      │
│          │  │ Cross-     │ │  │ LambdaMART Feature     │  │                      │
│          │  │ Encoder    │ │  │ Pre-computation        │  │                      │
│          │  │ Re-rank    │ │  │ (for top-100)          │  │                      │
│          │  │ 100→50     │ │  │                        │  │                      │
│          │  │ ~15ms      │ │  │ ~10ms                  │  │                      │
│          │  └─────┬─────┘ │  └────────────┬───────────┘  │                      │
│          │        │       │               │              │                      │
│          └────────┼───────┴───────────────┼──────────────┘                      │
│                   │                       │                                      │
│          T=32ms   ▼                       ▼                                      │
│          ┌──────────────────────────────────────────────────────┐               │
│          │           LAMBDAMART RE-RANKING                       │               │
│          │  50 candidates × 25 features → final scores          │               │
│          │  ~2ms                                                │               │
│          └──────────────────────────────────┬───────────────────┘               │
│                                             │                                   │
│          T=34ms                             ▼                                   │
│          ┌──────────────────────────────────────────────────────┐               │
│          │    DIVERSITY & POLICY ENFORCEMENT (MMR + Rules)       │               │
│          │  50 → 20 candidates                                  │               │
│          │  - Max 3 agents per provider                          │               │
│          │  - Min 3 taxonomy categories in top-10                │               │
│          │  - Enterprise policy enforcement                      │               │
│          │  ~2ms                                                │               │
│          └──────────────────────────────────┬───────────────────┘               │
│                                             │                                   │
│          T=36ms                             ▼                                   │
│          ┌──────────────────────────────────────────────────────┐               │
│          │        CONNECTION HINTS & EXPLANATION                  │               │
│          │  20 → 10 results (final page)                        │               │
│          │  - Auth hint resolution                               │               │
│          │  - Endpoint health snapshot                            │               │
│          │  - Ranking explanation payload                         │               │
│          │  - Connection recommendation (direct vs brokered)     │               │
│          │  ~5ms (parallel enrichment)                           │               │
│          └──────────────────────────────────────────────────────┘               │
│                                                                                 │
│          T=41ms  Response serialized and returned                               │
│                                                                                 │
│   ═══════════════════════════════════════════════════════════════                │
│   TOTAL LATENCY: 41ms (P50 target: 20ms, P95 target: 50ms)                     │
│   The P50 is lower because most queries hit warm caches at                      │
│   multiple stages, reducing actual execution time.                              │
└────────────────────────────────────────────────────────────────────────────────┘
```

**Latency Waterfall Diagram**

```
T(ms) 0    5    10   15   20   25   30   35   40   45   50
      |    |    |    |    |    |    |    |    |    |    |
      ├────┤                                            Query Understanding (4ms)
      |    ├─────────┤                                  Tantivy BM25 (6ms)
      |    ├───────────┤                                Qdrant kNN (8ms)
      |    ├──────┤                                     PG Filter Precompute (3ms)
      |              ├──┤                               RRF Fusion (2ms)
      |                 ├┤                              Eligibility Filter (1ms)
      |                  ├──┤                           Lightweight Score (2ms)
      |                     ├──────────────────┤        Cross-Encoder (15ms)
      |                     ├────────────┤              Feature Precompute (10ms)
      |                                       ├──┤     LambdaMART (2ms)
      |                                          ├──┤  Diversity+Policy (2ms)
      |                                             ├──────┤ Enrichment (5ms)
      |    |    |    |    |    |    |    |    |    |    |
      0    5    10   15   20   25   30   35   40   45   50

      ████ = on critical path
      ░░░░ = parallel (not on critical path)

Critical path: QU(4) → Qdrant(8) → RRF(2) → Filter(1) → LightScore(2) → 
               CrossEncoder(15) → LambdaMART(2) → Diversity(2) → Enrich(5) = 41ms

P50 target: 20ms (assumes cache hits on Qdrant + CrossEncoder for popular queries)
P95 target: 50ms (allows for cold cache + full computation)
P99 target: 100ms (allows for backend slowness + retry)
```

---

### 13.3 Query Performance Targets

**Latency Targets**

| Percentile | Target | Rationale |
|-----------|--------|-----------|
| **P50** | < 20ms | Agent-to-agent runtime search must fit within a 100ms orchestrator latency budget. At P50, the search should be fast enough to be imperceptible. |
| **P95** | < 50ms | Allows for cache misses, cold vector search, and full re-ranking without breaching orchestrator budgets. |
| **P99** | < 100ms | Worst-case scenario: cold caches, slow backend, retry. Still within acceptable bounds for real-time agent composition. |
| **P99.9** | < 500ms | Extreme tail; likely involves backend failover or degraded mode. Acceptable for non-latency-critical callers. |

**Throughput Targets**

| Metric | Target | Infrastructure |
|--------|--------|----------------|
| **Peak QPS** | > 10,000 | Horizontally scaled search service behind load balancer |
| **Sustained QPS** | > 5,000 | Normal operating capacity |
| **Burst QPS** | > 20,000 (30s burst) | Auto-scaling with pre-warmed capacity |

**Degradation Strategy**

When the system is under load or a backend is unhealthy, the query engine progressively degrades the pipeline to maintain latency targets:

| Degradation Level | Trigger | Action | Impact |
|-------------------|---------|--------|--------|
| **L0 — Normal** | All systems healthy, load < 70% | Full 6-stage pipeline | None — optimal quality |
| **L1 — Reduced Re-ranking** | Cross-encoder latency > 25ms OR load > 80% | Skip cross-encoder; use lightweight scores + LambdaMART only | ~3% NDCG regression; saves 15ms |
| **L2 — Reduced Retrieval** | Any retrieval backend latency > 15ms OR load > 90% | Disable slowest retrieval mode (usually graph); proceed with 2 modes | ~5% recall regression for graph-reachable agents; saves 5-8ms |
| **L3 — Cache-Only Semantic** | Qdrant latency > 20ms | Use pre-cached semantic results for popular queries; fall back to lexical-only for cache misses | ~10% NDCG regression for novel queries; saves 8ms |
| **L4 — Emergency Lexical** | Multiple backends unhealthy OR load > 95% | Tantivy BM25 only + cached AgentRank scores; no re-ranking | ~20% NDCG regression; guarantees sub-15ms response |
| **L5 — Static Fallback** | All backends unhealthy | Return pre-computed "top agents by category" from CDN cache | No personalization; ensures availability |

**Degradation Decision Logic**

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    DEGRADATION DECISION TREE                              │
│                                                                          │
│   On every query:                                                        │
│                                                                          │
│   1. Check circuit breakers for each backend                             │
│      (Tantivy, Qdrant, Neo4j, Cross-Encoder GPU, PG)                   │
│                                                                          │
│   2. Check current system load (QPS / capacity)                         │
│                                                                          │
│   3. Select degradation level:                                           │
│                                                                          │
│      if all_healthy AND load < 0.70:                                     │
│          level = L0                                                      │
│      elif cross_encoder_slow OR load > 0.80:                            │
│          level = L1                                                      │
│      elif any_retrieval_slow OR load > 0.90:                            │
│          level = L2                                                      │
│      elif qdrant_slow:                                                   │
│          level = L3                                                      │
│      elif multiple_backends_down OR load > 0.95:                        │
│          level = L4                                                      │
│      else:  # catastrophic                                               │
│          level = L5                                                      │
│                                                                          │
│   4. Execute pipeline at selected level                                  │
│                                                                          │
│   5. Tag response with degradation_level for observability               │
│                                                                          │
│   Circuit breaker settings:                                              │
│   - Failure threshold: 5 consecutive failures                            │
│   - Recovery probe: every 5 seconds                                      │
│   - Half-open: allow 1 request after probe success                       │
│   - Full open: allow all requests after 3 consecutive probe successes    │
└──────────────────────────────────────────────────────────────────────────┘
```

**Caching Strategy**

| Cache Layer | Scope | TTL | Size | Hit Rate (expected) |
|------------|-------|-----|------|---------------------|
| **Query Plan Cache** | Parsed + expanded query plan | 5 min | 10K entries | ~40% (popular queries repeat) |
| **Embedding Cache** | Query text → embedding vector | 15 min | 50K entries | ~30% (same queries get same embeddings) |
| **Tantivy Segment Cache** | OS page cache for index segments | Indefinite (LRU) | Available RAM | ~95% (hot index fits in memory) |
| **Qdrant HNSW Cache** | HNSW graph nodes in RAM | Indefinite | Dedicated RAM | ~99% (HNSW always in RAM) |
| **Cross-Encoder Cache** | (query_hash, agent_id) → score | 30 min | 100K entries | ~25% (same query-agent pairs) |
| **Result Cache** | Full result page for exact query + filters | 2 min | 5K entries | ~15% (exact query repeats are rare) |
| **AgentRank Cache** | agent_id → precomputed AgentRank | 1 hour | All agents | ~100% (precomputed, always warm) |

**Request Lifecycle (Complete)**

```
┌──────────────────────────────────────────────────────────────────────────┐
│                       QUERY REQUEST LIFECYCLE                             │
│                                                                          │
│   1. API Gateway receives POST /v1/search                                │
│      → Rate limit check (token bucket per caller)                       │
│      → Authentication (JWT validation)                                   │
│      → Request ID assignment (UUID v7)                                   │
│      → Tenant context resolution                                         │
│                                                                          │
│   2. Query Understanding Service                                         │
│      → Check query plan cache (LRU, 5min TTL)                           │
│      → If miss: parse → classify intent → extract entities → expand      │
│      → Build SearchPlan                                                  │
│                                                                          │
│   3. Search Coordinator                                                  │
│      → Assess degradation level                                          │
│      → Fan out to retrieval backends (parallel)                         │
│      → Set per-backend timeouts (backend_timeout = budget - elapsed)     │
│      → Collect results with deadline propagation                         │
│                                                                          │
│   4. Fusion + Filtering + Scoring                                        │
│      → RRF fusion of available results                                   │
│      → Eligibility filtering (bit-vector AND)                           │
│      → Lightweight scoring (AgentRank + RRF + trust floor)              │
│                                                                          │
│   5. Re-ranking                                                          │
│      → Cross-encoder (if L0 degradation, GPU available)                 │
│      → LambdaMART (CPU, always available)                               │
│                                                                          │
│   6. Post-processing                                                     │
│      → Diversity enforcement (MMR, provider dedup)                      │
│      → Policy enforcement (enterprise rules)                             │
│      → Connection hint resolution                                        │
│      → Explanation payload construction                                  │
│                                                                          │
│   7. Response Assembly                                                   │
│      → Serialize results (protobuf or JSON based on Accept header)      │
│      → Attach metadata: request_id, degradation_level, latency_ms,     │
│        cache_hit_rate, result_count, has_more                           │
│                                                                          │
│   8. Telemetry (async, non-blocking)                                     │
│      → Emit search event to Kafka (query, results, latency, level)     │
│      → Update rate limit counters                                        │
│      → Update latency histograms (Prometheus)                           │
└──────────────────────────────────────────────────────────────────────────┘
```

**Search Result Schema**

Every search response includes both the results and metadata that enables the caller to understand, trust, and act on the results:

```json
{
  "request_id": "01905a2b-7c3d-7e4f-8a1b-2c3d4e5f6a7b",
  "query": {
    "original": "kubernetes deployment agent with OAuth in EU",
    "parsed_intent": "capability_search",
    "expanded_terms": ["kubernetes", "k8s", "deployment", "oauth", "oauth2"],
    "applied_filters": {
      "auth_schemes": ["oauth*"],
      "regions": ["eu-*"]
    }
  },
  "results": [
    {
      "rank": 1,
      "agent_id": "01905a2b-aaaa-bbbb-cccc-ddddeeeeefff",
      "name": "kube-orchestrator",
      "provider": {
        "organization": "CloudNative Inc.",
        "domain": "cloudnative.io",
        "verified": true
      },
      "description": "Production-grade Kubernetes deployment and management agent...",
      "skills_matched": ["kubernetes-deploy", "helm-chart-management"],
      "scores": {
        "final_score": 0.892,
        "agent_rank": 87.3,
        "relevance_score": 0.91,
        "trust_tier": "verified",
        "trust_tier_ordinal": 3
      },
      "explanation": {
        "primary_match": "Capability match on 'kubernetes deployment'",
        "secondary_signals": [
          "High AgentRank (87.3/100)",
          "Verified provider",
          "95.2% connection success rate",
          "P95 latency: 180ms"
        ],
        "retrieval_modes": ["lexical", "semantic", "graph"]
      },
      "connection_hint": {
        "recommended_mode": "direct",
        "endpoint": "https://api.cloudnative.io/a2a/v1",
        "auth_schemes": ["oauth2"],
        "health_status": "healthy",
        "last_probe": "2026-03-23T10:45:00Z",
        "estimated_latency_ms": 180
      },
      "metadata": {
        "protocols": ["a2a-v1.0"],
        "input_modes": ["application/json", "text/plain"],
        "output_modes": ["application/json"],
        "regions": ["eu-west-1", "eu-central-1"],
        "last_crawled": "2026-03-23T04:12:00Z",
        "card_version": "2.1.0"
      }
    }
  ],
  "metadata": {
    "total_candidates": 1247,
    "after_filtering": 412,
    "results_returned": 10,
    "has_more": true,
    "next_cursor": "eyJvZmZzZXQiOjEwfQ==",
    "latency_ms": 38,
    "degradation_level": "L0",
    "cache_utilization": {
      "query_plan": "hit",
      "embedding": "miss",
      "cross_encoder": "partial",
      "result": "miss"
    }
  }
}
```

**Observability Integration**

Every query emits the following metrics:

| Metric | Type | Labels | Purpose |
|--------|------|--------|---------|
| `query_latency_ms` | Histogram | `intent`, `degradation_level` | Track latency by intent and degradation |
| `query_throughput` | Counter | `intent`, `status_code` | Track QPS and error rates |
| `candidates_per_stage` | Histogram | `stage` (1-6) | Monitor funnel health |
| `retrieval_mode_latency_ms` | Histogram | `mode` (lexical, semantic, graph) | Per-backend latency tracking |
| `cross_encoder_batch_size` | Histogram | — | Monitor GPU utilization |
| `cache_hit_rate` | Gauge | `cache_layer` | Per-cache hit rate tracking |
| `degradation_activations` | Counter | `level` | Track degradation frequency |
| `rrf_mode_contribution` | Histogram | `mode` | Track which modes contribute most to final results |
| `diversity_removals` | Counter | `reason` (provider_dedup, taxonomy, policy) | Monitor diversity impact |
| `zero_result_rate` | Gauge | `intent` | Detect coverage gaps |

---

### 13.4 Explain API

Every search result includes an explanation payload that answers "why was this agent returned, and why at this rank?" This is critical for both human users (building trust in the search system) and agent callers (making informed selection decisions).

**Explanation Structure**

```
┌──────────────────────────────────────────────────────────────────────────┐
│  EXPLAIN: kube-orchestrator at rank #1                                    │
│                                                                          │
│  WHY THIS AGENT?                                                         │
│  ┌────────────────────────────────────────────────────────────────┐      │
│  │ Primary: Capability match on "kubernetes deployment"           │      │
│  │ - Skill "kubernetes-deploy" matched query with BM25 score 24.3│      │
│  │ - Capability embedding similarity: 0.94                       │      │
│  └────────────────────────────────────────────────────────────────┘      │
│                                                                          │
│  WHY THIS RANK?                                                          │
│  ┌────────────────────────────────────────────────────────────────┐      │
│  │ Score breakdown (final_score = 0.892):                        │      │
│  │                                                               │      │
│  │ RRF fusion score:           0.0465  (weight: 0.25)  ████████  │      │
│  │ AgentRank (decayed):        87.3    (weight: 0.20)  ███████   │      │
│  │ Cross-encoder relevance:    0.91    (weight: 0.15)  ██████    │      │
│  │ Name match:                 0.0     (weight: 0.08)            │      │
│  │ Skill match:                0.95    (weight: 0.08)  ████      │      │
│  │ Domain match:               0.88    (weight: 0.07)  ███       │      │
│  │ Freshness:                  0.98    (weight: 0.05)  ██        │      │
│  │ Popularity:                 0.72    (weight: 0.07)  ███       │      │
│  │ Provider diversity:         1.0     (weight: 0.05)  ██        │      │
│  └────────────────────────────────────────────────────────────────┘      │
│                                                                          │
│  TRUST CONTEXT:                                                          │
│  ┌────────────────────────────────────────────────────────────────┐      │
│  │ Trust tier: Verified (3/5)                                    │      │
│  │ Provider: CloudNative Inc. — domain verified ✓                │      │
│  │ TLS: 1.3 + HSTS ✓                                            │      │
│  │ Auth: OAuth 2.0 with PKCE ✓                                  │      │
│  │ Uptime (30d): 99.2%                                          │      │
│  │ Connect success (7d): 95.8%                                   │      │
│  └────────────────────────────────────────────────────────────────┘      │
│                                                                          │
│  RETRIEVAL PATH:                                                         │
│  ┌────────────────────────────────────────────────────────────────┐      │
│  │ Found by: Lexical (rank #5), Semantic (rank #1), Graph (#8)   │      │
│  │ Survived filters: protocol ✓, auth ✓, region ✓, trust ✓      │      │
│  │ Cross-encoder rank: #2 → LambdaMART rank: #1                 │      │
│  └────────────────────────────────────────────────────────────────┘      │
└──────────────────────────────────────────────────────────────────────────┘
```

**Why Explain Matters**

1. **Builder trust**: If developers cannot understand why results are ranked as they are, they will not trust the search system and will build their own.
2. **Debugging**: When search results are wrong, the explain payload is the primary debugging tool.
3. **Provider optimization**: Providers in the Agent Search Console use explanation data to understand how to improve their agents' rankings.
4. **Compliance**: Enterprise customers may require explanations for agent selection decisions for audit purposes.
5. **Anti-gaming detection**: Explain data reveals when agents are artificially boosted by specific signals, helping the trust team identify gaming attempts.

---

### 13.5 Multi-Tenant Query Isolation

Enterprise queries must be strictly isolated from public queries and from other tenants. The query engine enforces this at multiple levels:

```
┌──────────────────────────────────────────────────────────────────────────┐
│                   MULTI-TENANT QUERY ISOLATION                            │
│                                                                          │
│   Layer 1: Authentication                                                │
│   - JWT token contains tenant_id and scopes                             │
│   - API Gateway validates token before query reaches search service     │
│                                                                          │
│   Layer 2: Query Rewriting                                               │
│   - Query planner injects tenant_id filter into all retrieval queries   │
│   - Tantivy: filter on tenant_ids fast field                            │
│   - Qdrant: payload filter on tenant_ids                                │
│   - Neo4j: WHERE clause on agent visibility                             │
│                                                                          │
│   Layer 3: Result Filtering                                              │
│   - Post-retrieval verification: every result is checked against        │
│     tenant ACL before inclusion in response                             │
│   - Defense-in-depth: even if a retrieval backend leaks a result,       │
│     the result filter catches it                                        │
│                                                                          │
│   Layer 4: Ranking Signal Isolation                                      │
│   - Behavioral features (CTR, connect rate) are computed per-tenant     │
│   - Cross-tenant signal leakage is prevented by maintaining separate    │
│     feature stores per tenant                                           │
│   - Exception: public AgentRank scores are shared (they are computed    │
│     from public data only)                                              │
│                                                                          │
│   Layer 5: Audit Logging                                                 │
│   - Every query is logged with tenant_id for audit compliance           │
│   - Cross-tenant access attempts are logged as security events          │
└──────────────────────────────────────────────────────────────────────────┘
```

---

### 13.6 Query API Contract

The canonical search endpoint accepts structured queries with optional constraints, filters, and configuration:

```
POST /v1/search
Content-Type: application/json
Authorization: Bearer <jwt>

{
  "query": "kubernetes deployment agent with OAuth support",
  
  "filters": {
    "protocols": ["a2a-v1.0"],
    "auth_schemes": ["oauth2"],
    "regions": ["eu-*"],
    "trust_tier_min": "established",
    "modalities": {
      "input": ["application/json"],
      "output": ["application/json"]
    },
    "price_tier_max": "standard",
    "max_latency_ms": 500
  },
  
  "options": {
    "limit": 10,
    "offset": 0,
    "include_explanation": true,
    "include_connection_hints": true,
    "diversity": {
      "max_per_provider": 3,
      "min_taxonomy_categories": 3
    }
  },
  
  "context": {
    "caller_agent_id": "01905a2b-caller-agent-id",
    "task_description": "Deploy a containerized microservice to production",
    "urgency": "normal"
  }
}
```

**Response: 200 OK** — see result schema in Section 13.2.

**Error Responses:**

| Status | Code | Description |
|--------|------|-------------|
| 400 | `invalid_query` | Malformed query or invalid filter values |
| 401 | `unauthorized` | Missing or invalid JWT |
| 403 | `forbidden` | Caller lacks search permission for requested tenant scope |
| 429 | `rate_limited` | Caller exceeded rate limit (include `Retry-After` header) |
| 500 | `internal_error` | Unexpected failure (include `request_id` for debugging) |
| 503 | `service_degraded` | Search available but at degraded quality (include `degradation_level`) |

---

### 13.7 Pagination and Cursor Strategy

Search pagination uses opaque cursors (not page numbers) to ensure result stability across pages even as the underlying index changes.

**Why Cursors Over Offsets?**

| Approach | Pros | Cons |
|----------|------|------|
| **Offset-based** (`?page=2&size=10`) | Simple; stateless | Results shift between pages if index changes between requests; performance degrades at deep offsets (Tantivy must scan and discard N results) |
| **Cursor-based** (opaque token) | Stable results across pages; constant performance regardless of depth; no skipped/duplicated results | Requires server-side state or encoded cursor; cannot jump to arbitrary page |

We use **cursor-based pagination** because:
1. Agent-to-agent callers iterate sequentially (they don't jump to page 5)
2. Index updates between pages would cause result instability with offsets
3. Deep pagination performance matters for callers that need to enumerate many results

**Cursor Encoding**

The cursor is a base64-encoded JSON object containing enough state to resume the search:

```json
{
  "v": 1,
  "query_hash": "sha256:abc123...",
  "last_score": 0.7832,
  "last_agent_id": "01905a2b-aaaa-bbbb-cccc-ddddeeeeefff",
  "degradation_level": "L0",
  "timestamp": "2026-03-23T10:45:00Z",
  "ttl": 300
}
```

**Cursor TTL:** Cursors expire after 5 minutes (300 seconds). After expiry, the caller must re-execute the query from the beginning. This prevents stale cursors from serving increasingly outdated results.

**Search-After Implementation:** Under the hood, cursor-based pagination uses Tantivy's "search after" capability: given the last result's sort key (score + agent_id for tiebreaking), the next page starts immediately after that key without scanning preceding results.

---

### 13.8 Query Telemetry and Feedback Loop

Every search interaction generates telemetry that feeds back into the ranking system. This closed loop is the primary mechanism for ranking improvement over time.

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    SEARCH TELEMETRY FEEDBACK LOOP                         │
│                                                                          │
│   ┌──────────────┐     ┌──────────────┐     ┌──────────────┐            │
│   │  Search       │────▶│  Kafka        │────▶│  ClickHouse  │            │
│   │  Service      │     │  Events       │     │  Analytics   │            │
│   │  (emits       │     │  Topic:       │     │  (stores     │            │
│   │   events)     │     │  search.*     │     │   30d of     │            │
│   └──────────────┘     └──────────────┘     │   events)    │            │
│                                              └──────┬───────┘            │
│                                                     │                    │
│                                                     ▼                    │
│   ┌──────────────────────────────────────────────────────────┐          │
│   │  SIGNALS DERIVED FROM TELEMETRY                           │          │
│   │                                                           │          │
│   │  • CTR per (query, agent) pair → LambdaMART feature #22  │          │
│   │  • Connect success rate → LambdaMART feature #23          │          │
│   │  • Zero-result queries → query expansion improvements     │          │
│   │  • Degradation frequency → capacity planning              │          │
│   │  • Per-mode contribution → retrieval weight tuning        │          │
│   │  • Latency distributions → performance regression alerts  │          │
│   └──────────────────────────────────────────────────────────┘          │
│                                                                          │
│   EVENTS EMITTED:                                                        │
│                                                                          │
│   search.query_executed                                                  │
│     {request_id, query, intent, entities, filters, degradation_level,   │
│      latency_ms, result_count, cache_utilization, tenant_id}            │
│                                                                          │
│   search.result_shown                                                    │
│     {request_id, agent_id, rank, final_score, retrieval_modes}          │
│                                                                          │
│   search.result_clicked                                                  │
│     {request_id, agent_id, rank, dwell_time_ms}                         │
│                                                                          │
│   search.connect_attempted                                               │
│     {request_id, agent_id, auth_scheme, connect_mode}                   │
│                                                                          │
│   search.connect_outcome                                                 │
│     {request_id, agent_id, success, failure_reason, latency_ms}         │
│                                                                          │
│   search.task_outcome                                                    │
│     {request_id, agent_id, task_status, quality_signal, duration_ms}    │
└──────────────────────────────────────────────────────────────────────────┘
```

**Click Model for Implicit Relevance**

Raw click-through data is noisy: users click on results for many reasons besides relevance (position bias, curiosity, familiarity). We apply a position-bias-corrected click model to extract relevance signal from CTR data:

```
relevance_probability(a, q) = P(click | a, q, rank) / P(examine | rank)

P(examine | rank) is estimated from position-specific CTR across all queries:
  rank 1: 0.85
  rank 2: 0.72
  rank 3: 0.61
  rank 4: 0.50
  rank 5: 0.42
  rank 6: 0.35
  rank 7: 0.29
  rank 8: 0.24
  rank 9: 0.20
  rank 10: 0.17
```

This is a simplified cascade click model. In Phase 3, we will implement a full Bayesian click model (e.g., Dynamic Bayesian Network model) for more accurate position debiasing.

---

### 13.9 Evaluation Framework

The search system is evaluated continuously using both offline and online metrics:

**Offline Evaluation (Weekly)**

| Metric | Definition | Target | Current |
|--------|-----------|--------|---------|
| **NDCG@5** | Normalized Discounted Cumulative Gain at rank 5 | > 0.75 | — |
| **NDCG@10** | NDCG at rank 10 (primary metric) | > 0.70 | — |
| **MAP@10** | Mean Average Precision at rank 10 | > 0.65 | — |
| **MRR** | Mean Reciprocal Rank (rank of first relevant result) | > 0.85 | — |
| **Precision@1** | Fraction of queries where rank-1 result is relevant | > 0.80 | — |
| **Recall@100** | Fraction of relevant agents found in top-100 candidates | > 0.95 | — |
| **Zero-result rate** | Fraction of queries returning 0 results | < 2% | — |

**Online Evaluation (Continuous)**

| Metric | Definition | Target | Alerting Threshold |
|--------|-----------|--------|-------------------|
| **CTR@1** | Click-through rate on rank-1 result | > 30% | < 20% |
| **Connect rate** | Fraction of shown results that lead to connection attempt | > 15% | < 8% |
| **Connect success** | Fraction of connection attempts that succeed | > 90% | < 80% |
| **Session success** | Fraction of search sessions leading to positive task outcome | > 60% | < 40% |
| **Abandonment rate** | Fraction of queries followed by a reformulated query | < 15% | > 25% |
| **Latency P50/P95** | Query latency at 50th and 95th percentile | 20ms / 50ms | 30ms / 80ms |

**Evaluation Data Sources**

| Source | Method | Volume | Update |
|--------|--------|--------|--------|
| Human-judged query set | Trained raters on 4-point scale | 10K (query, agent, label) triples | +500/week |
| Synthetic judgments | GPT-4 with calibrated prompts | 50K triples | Daily (3K/day) |
| Click data | Position-debiased CTR | 100K+ search sessions/week | Real-time |
| Connect outcomes | Platform-observed connection telemetry | 20K+ connect attempts/week | Real-time |
| Task outcomes | Partner-signed or platform-observed | 5K+ task completions/week | Real-time |

---

### 13.10 Search Quality Guardrails

The search system includes automated guardrails that detect and prevent ranking regressions:

**Pre-deployment Guardrails (CI/CD)**

| Guardrail | Condition | Action |
|-----------|-----------|--------|
| NDCG regression | NDCG@10 on eval set drops > 2% vs current production model | Block deployment; alert search team |
| Zero-result regression | Zero-result rate increases > 1% absolute | Block deployment; review query expansion |
| Latency regression | P95 latency increases > 10ms on benchmark query set | Block deployment; profile performance |
| Diversity regression | Average number of unique providers in top-10 drops > 0.5 | Warning; review diversity parameters |

**Runtime Guardrails (Production)**

| Guardrail | Condition | Action |
|-----------|-----------|--------|
| Anomalous CTR drop | CTR@1 drops > 20% relative in 1-hour window | Alert; auto-rollback to previous model if in A/B test |
| Spike in zero-results | Zero-result rate > 5% in 5-minute window | Alert; check backend health; activate L2 degradation |
| Single-provider dominance | One provider occupies > 50% of top-10 results across all queries | Alert; review diversity enforcement; check for gaming |
| Trust tier anomaly | Agent with `Suspicious` tier appears in top-10 | Alert; immediate review; potential filter bypass |

---

*End of Part 3: Sections 11-13*

---

**Cross-references to other parts:**

- Section 8 (Crawl Architecture) provides the data pipeline that feeds Section 11 (candidate generation indexing)
- Section 10 (Canonical Registry) provides the entity model that Section 12 (AgentRank) scores
- Section 14 (Liveness and Benchmarking) provides the operational signals consumed by S_A (Availability) and S_V (Verification)
- Section 16 (Trust and Verification) provides the trust tier system consumed by S_T (Trust Score) and eligibility filters
- Section 18 (Connection Plane) provides the outcome telemetry consumed by S_R (Reputation) and LambdaMART behavioral features
- Section 20 (Agent Search Console) consumes the Discoverability Score (Section 12.10) and explanation payloads (Section 13.4)
- Section 26 (Experimentation) provides the A/B testing framework referenced in the LTR roadmap (Section 12.11)
# Part V: Connection, Liveness, APIs, and Data Platform

## Sections 14–18: Connection Architecture, Liveness & Verification, API & Protocol Layer, Data Model & Schema, Canonical Schema Guidance

---

## 14. Connection Architecture

### 14.1 Why Connection Matters Strategically

Discovery alone is a library catalog. Connection is the checkout desk, the reading room, and the librarian who confirms the book was useful. Every competitor in the agent discovery space stops at search — returning a list of agent endpoints and leaving the caller to handle authentication, compatibility, policy, retries, and outcome evaluation on their own. This is the equivalent of Google returning URLs but never loading the page.

**Connection is where the data moat compounds.**

Every successful connection generates three categories of signal that no discovery-only system can produce:

1. **Interaction signals** — which agents are actually being called, by whom, for what tasks, in what composition patterns. This builds the interaction graph.
2. **Outcome signals** — whether the connection succeeded, whether the task completed, what the quality of the result was. This builds the outcome graph.
3. **Trust signals** — whether the agent behaved as advertised, whether auth negotiation was clean, whether latency matched expectations. This builds the trust graph.

These three signal categories feed back into ranking, creating a flywheel that accelerates with every connection:

```
┌─────────────────────────────────────────────────────────────────────┐
│                   THE AGENTRANK FLYWHEEL                            │
│                                                                     │
│    ┌──────────┐     ┌──────────┐     ┌────────────┐                │
│    │          │     │          │     │            │                │
│    │ DISCOVERY├────►│ RANKING  ├────►│ CONNECTION │                │
│    │          │     │          │     │            │                │
│    └──────────┘     └─────▲────┘     └──────┬─────┘                │
│                           │                 │                       │
│                           │                 ▼                       │
│                    ┌──────┴────┐     ┌────────────┐                │
│                    │           │     │            │                │
│                    │   TRUST   │◄────┤  OUTCOME   │                │
│                    │           │     │            │                │
│                    └───────────┘     └────────────┘                │
│                                                                     │
│  Each revolution:                                                   │
│    • Discovery finds more agents (+coverage)                        │
│    • Ranking surfaces better agents (+precision)                    │
│    • Connection validates quality (+evidence)                       │
│    • Outcomes prove value (+signal)                                 │
│    • Trust filters noise (+safety)                                  │
│    • Better ranking attracts more callers (+volume)                 │
│    • More callers generate more outcomes (+data moat)               │
└─────────────────────────────────────────────────────────────────────┘
```

Without the connection layer, the flywheel is broken at the third stage. You discover agents, rank them, and then... the caller leaves your platform, connects independently, and you never learn whether the connection succeeded. You never improve. You are a static directory forever.

**Competitive comparison: Directory-Only vs AgentRank (Connection Loop)**

| Dimension | Directory-Only Model | AgentRank (Connection Loop) |
|-----------|---------------------|----------------------------|
| **Ranking signal source** | Self-reported metadata, manual reviews, download counts | Observed outcomes, connection success rates, latency measurements, task completion evidence |
| **Signal freshness** | Static until provider updates listing | Continuous — every connection generates fresh signal |
| **Trust model** | Binary (listed / not listed) or manual verification | Multi-tier, evidence-based, continuously updated from behavioral signals |
| **Network effects** | Linear — each new listing adds constant value | Superlinear — each new connection makes all rankings smarter |
| **Moat depth** | Shallow — any competitor can scrape and replicate listings | Deep — outcome graph is proprietary and compounds over time |
| **Monetization surface** | Listing fees, banner ads, featured placement | Connection-based pricing, premium outcome analytics, SLA guarantees, priority routing |
| **Data asset** | Static catalog of metadata | Four compounding graphs (agent, interaction, outcome, trust) |
| **Ranking accuracy over time** | Degrades (metadata rots, no feedback) | Improves (more data → better ranking → more users → more data) |
| **Failure mode** | Stale results, dead endpoints, no quality signal | Self-correcting — dead agents detected, low-quality agents suppressed |
| **Strategic position** | Replaceable commodity | Increasing-returns infrastructure |

The connection layer transforms AgentRank from a search engine into a platform with compounding network effects. This is the strategic insight that every competitor will miss for 12–18 months.

### 14.2 Connection Modes

AgentRank supports two fundamental connection modes, each optimized for different trust levels, latency requirements, and observability needs.

#### Direct Connect

In Direct Connect mode, AgentRank returns the resolved endpoint and the caller connects to the target agent directly via A2A protocol. The platform is not in the data path after the initial resolution.

```
┌────────────┐     ┌────────────┐     ┌────────────────┐
│            │     │            │     │                │
│   Caller   ├────►│  AgentRank ├────►│  Resolution    │
│   Agent    │  1  │  Search    │  2  │  Response with │
│            │◄────┤  API       │◄────┤  endpoint +    │
│            │     │            │     │  auth hint     │
│            │     └────────────┘     └────────────────┘
│            │
│            │  3  ┌────────────────┐
│            ├────►│                │
│            │     │  Target Agent  │    (Direct A2A connection)
│            │◄────┤  (resolved     │
│            │  4  │   endpoint)    │
└────────────┘     └────────────────┘

         5  ┌────────────────┐
   Caller ──►│  AgentRank     │    (Async outcome report)
             │  Outcome API   │
             └────────────────┘
```

**Characteristics:**
- Lowest latency — no proxy overhead after resolution
- Caller manages auth, retries, and error handling
- Outcome reporting is voluntary (but incentivized through ranking benefits)
- Best for: public agents, simple auth schemes (API key, bearer token), latency-sensitive workflows
- Limitation: platform has no visibility into connection quality without voluntary reporting

#### Brokered Connect

In Brokered Connect mode, AgentRank mediates the connection setup and optionally proxies the session. The platform handles authentication exchange, compatibility validation, policy enforcement, and captures outcome telemetry automatically.

```
┌────────────┐     ┌────────────────────────────────────────────────┐
│            │     │              AgentRank Connect Broker           │
│            │     │                                                 │
│   Caller   │     │  ┌─────────┐  ┌─────────┐  ┌──────────────┐  │
│   Agent    ├────►│  │ Resolve ├─►│Validate ├─►│  Negotiate   │  │
│            │  1  │  │Endpoint │  │ Compat  │  │  Auth        │  │
│            │     │  └─────────┘  └─────────┘  └──────┬───────┘  │
│            │     │                                     │          │
│            │     │  ┌─────────┐  ┌─────────┐  ┌──────▼───────┐  │
│            │◄────┤  │  Emit   │◄─┤  Route  │◄─┤  Apply       │  │
│            │  6  │  │Telemetry│  │ Traffic │  │  Policy      │  │
│            │     │  └─────────┘  └─────────┘  └──────────────┘  │
│            │     │                                                 │
│            │     └──────────────────────┬─────────────────────────┘
│            │                            │
│            │     ┌──────────────────────▼─────────────────────────┐
│            │◄───►│           Target Agent                         │
│            │  7  │           (proxied or direct after handshake)  │
│            │     └────────────────────────────────────────────────┘
└────────────┘
```

**Characteristics:**
- Higher latency — mediation adds overhead
- Platform manages auth exchange, policy enforcement, and retries
- Outcome telemetry is captured automatically (platform sees the session)
- Best for: enterprise agents, complex auth (OAuth exchange, mutual TLS), policy-governed workflows, agents requiring SLA guarantees
- Advantage: full observability, automatic outcome capture, retry intelligence

#### Mediation Depth Spectrum

Brokered Connect is not a single mode — it is a spectrum of mediation depths. The platform selects the appropriate depth based on policy, auth requirements, and caller preferences.

| Mediation Depth | Latency Overhead | Description | Platform Visibility | Use Case |
|----------------|-----------------|-------------|-------------------|----------|
| **Handshake-only** | 50–80ms | Platform resolves endpoint, validates compatibility, and returns a connection token. Caller connects directly using the token. | Session start only | Low-friction auth, caller wants direct path but needs platform-validated token |
| **Proxy-first** | 80–150ms | Platform proxies the initial handshake (auth exchange, capability negotiation) then hands off. Subsequent messages go direct. | Handshake + first exchange | OAuth token exchange, capability negotiation, SLA validation |
| **Full-proxy** | 100–300ms | All traffic flows through the platform for the duration of the session. | Full session visibility | Enterprise policy enforcement, audit logging, content inspection, rate limiting |
| **Managed-session** | 100–500ms | Platform manages the entire session lifecycle: connection pooling, retry on failure, failover to alternate agents, timeout management. | Full session + lifecycle | Mission-critical workflows, multi-agent orchestration, SLA-guaranteed connections |

**Selection heuristic:**

```
IF caller.trust_tier >= VERIFIED AND target.auth == API_KEY:
    → handshake-only (minimize latency)
ELIF target.auth IN [OAUTH2, SAML]:
    → proxy-first (platform handles token exchange)
ELIF caller.policy.requires_audit OR target.policy.requires_content_inspection:
    → full-proxy (platform sees all traffic)
ELIF caller.requires_sla OR caller.requires_failover:
    → managed-session (platform manages lifecycle)
ELSE:
    → proxy-first (safe default)
```

### 14.3 Connection Broker Responsibilities

The Connection Broker is the service that implements brokered connections. It is a stateful, policy-aware pipeline that transforms a connection request into a live session.

#### Seven-Step Pipeline

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                        CONNECTION BROKER PIPELINE                               │
│                                                                                 │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐    │
│  │    1.    │   │    2.    │   │    3.    │   │    4.    │   │    5.    │    │
│  │ RESOLVE  │──►│ VALIDATE │──►│NEGOTIATE │──►│  APPLY   │──►│ CREATE   │    │
│  │ENDPOINT  │   │ COMPAT   │   │  AUTH    │   │ POLICY   │   │ SESSION  │    │
│  │          │   │          │   │          │   │          │   │          │    │
│  │ Registry │   │ Protocol │   │ Auth     │   │ Tenant   │   │ Session  │    │
│  │ lookup,  │   │ version, │   │ exchange │   │ rules,   │   │ token,   │    │
│  │ health   │   │ skill    │   │ token    │   │ rate     │   │ timeout, │    │
│  │ check,   │   │ match,   │   │ mint,    │   │ limits,  │   │ routing  │    │
│  │ endpoint │   │ format   │   │ mTLS     │   │ content  │   │ config   │    │
│  │ select   │   │ check    │   │ setup    │   │ filter   │   │          │    │
│  └────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘    │
│       │              │              │              │              │            │
│       ▼              ▼              ▼              ▼              ▼            │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐    │
│  │ Fail:    │   │ Fail:    │   │ Fail:    │   │ Fail:    │   │ Fail:    │    │
│  │ AGENT_   │   │ INCOMPAT │   │ AUTH_    │   │ POLICY_  │   │ SESSION_ │    │
│  │ NOT_FOUND│   │ _IBLE    │   │ FAILED   │   │ DENIED   │   │ ERROR    │    │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘   └──────────┘    │
│                                                                                 │
│       ┌──────────┐        ┌──────────┐                                         │
│       │    6.    │        │    7.    │                                         │
│  ────►│  ROUTE   │───────►│   EMIT   │──────► Session Active                  │
│       │ TRAFFIC  │        │TELEMETRY │                                         │
│       │          │        │          │                                         │
│       │ Forward  │        │ Latency, │                                         │
│       │ to target│        │ status,  │                                         │
│       │ via mode │        │ outcome  │                                         │
│       └──────────┘        └──────────┘                                         │
└─────────────────────────────────────────────────────────────────────────────────┘
```

#### Step 1: Resolve Endpoint

The broker resolves the target agent's current endpoint from the canonical registry, checks its health status, and selects the best endpoint if the agent has multiple replicas.

```rust
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ResolvedEndpoint {
    pub agent_id: Uuid,
    pub endpoint_url: String,
    pub protocol_version: String,
    pub health_status: AgentHealthStatus,
    pub latency_p50_ms: Option<f64>,
    pub latency_p99_ms: Option<f64>,
    pub region: Option<String>,
    pub last_healthy_at: Option<chrono::DateTime<chrono::Utc>>,
    pub auth_schemes: Vec<AuthScheme>,
    pub capabilities_hash: String,
    pub trust_tier: TrustTier,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AgentHealthStatus {
    Healthy,
    Degraded { reason: String },
    Unhealthy { since: chrono::DateTime<chrono::Utc> },
    Unknown,
}

#[derive(Debug, Clone)]
pub enum AuthScheme {
    None,
    ApiKey { header_name: String },
    BearerToken,
    OAuth2 {
        token_url: String,
        scopes: Vec<String>,
    },
    MutualTls {
        ca_cert_url: Option<String>,
    },
    Custom {
        scheme_name: String,
        metadata: serde_json::Value,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustTier {
    Indexed = 0,
    Established = 1,
    Verified = 2,
    Trusted = 3,
    Authoritative = 4,
}

pub struct EndpointResolver {
    registry: Arc<RegistryClient>,
    health_cache: Arc<HealthCache>,
    geo_resolver: Arc<GeoResolver>,
}

impl EndpointResolver {
    pub async fn resolve(
        &self,
        agent_id: Uuid,
        caller_region: Option<&str>,
    ) -> Result<ResolvedEndpoint, ConnectError> {
        let agent = self.registry
            .get_agent(agent_id)
            .await
            .map_err(|_| ConnectError::AgentNotFound { agent_id })?;

        if agent.delisted {
            return Err(ConnectError::AgentDelisted {
                agent_id,
                reason: agent.delist_reason.unwrap_or_default(),
            });
        }

        let health = self.health_cache
            .get_status(agent_id)
            .await
            .unwrap_or(AgentHealthStatus::Unknown);

        match &health {
            AgentHealthStatus::Unhealthy { since } => {
                let unhealthy_duration = chrono::Utc::now() - *since;
                if unhealthy_duration > chrono::Duration::hours(24) {
                    return Err(ConnectError::AgentUnhealthy {
                        agent_id,
                        since: *since,
                    });
                }
                // Allow connection attempt if unhealthy < 24h — might be transient
            }
            _ => {}
        }

        let endpoints = agent.endpoints.clone();
        let selected = self.select_best_endpoint(
            &endpoints,
            caller_region,
            &health,
        )?;

        Ok(ResolvedEndpoint {
            agent_id,
            endpoint_url: selected.url.clone(),
            protocol_version: agent.protocol_version.clone(),
            health_status: health,
            latency_p50_ms: selected.latency_p50_ms,
            latency_p99_ms: selected.latency_p99_ms,
            region: selected.region.clone(),
            last_healthy_at: agent.last_healthy_at,
            auth_schemes: agent.auth_schemes.clone(),
            capabilities_hash: agent.capabilities_hash.clone(),
            trust_tier: agent.trust_tier,
        })
    }

    fn select_best_endpoint(
        &self,
        endpoints: &[AgentEndpoint],
        caller_region: Option<&str>,
        health: &AgentHealthStatus,
    ) -> Result<&AgentEndpoint, ConnectError> {
        if endpoints.is_empty() {
            return Err(ConnectError::NoEndpoints);
        }

        if endpoints.len() == 1 {
            return Ok(&endpoints[0]);
        }

        // Multi-endpoint selection: prefer same-region, healthy, lowest latency
        let mut scored: Vec<(usize, f64)> = endpoints
            .iter()
            .enumerate()
            .filter(|(_, ep)| ep.healthy)
            .map(|(i, ep)| {
                let mut score = 0.0;

                // Region affinity: +100 for same region, +50 for same continent
                if let (Some(caller_r), Some(ep_r)) = (caller_region, &ep.region) {
                    if caller_r == ep_r {
                        score += 100.0;
                    } else if self.geo_resolver.same_continent(caller_r, ep_r) {
                        score += 50.0;
                    }
                }

                // Latency: lower is better, normalize to 0-50 range
                if let Some(p50) = ep.latency_p50_ms {
                    score += 50.0 * (1.0 - (p50 / 1000.0).min(1.0));
                }

                // Success rate: higher is better, 0-30 range
                if let Some(rate) = ep.success_rate_7d {
                    score += 30.0 * rate;
                }

                (i, score)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scored
            .first()
            .map(|(i, _)| &endpoints[*i])
            .ok_or(ConnectError::NoHealthyEndpoints)
    }
}
```

#### Step 2: Validate Compatibility

Before attempting a connection, the broker validates that the caller and target are protocol-compatible and that the requested capability actually exists on the target.

```rust
#[derive(Debug)]
pub struct CompatibilityResult {
    pub compatible: bool,
    pub protocol_match: ProtocolMatch,
    pub skill_match: Option<SkillMatch>,
    pub format_issues: Vec<FormatIssue>,
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub enum ProtocolMatch {
    Exact,
    BackwardCompatible { target_version: String, caller_version: String },
    ForwardCompatible { target_version: String, caller_version: String },
    Incompatible { target_version: String, caller_version: String, reason: String },
}

#[derive(Debug)]
pub struct SkillMatch {
    pub skill_id: String,
    pub skill_name: String,
    pub confidence: f64,
    pub input_compatible: bool,
    pub output_compatible: bool,
}

#[derive(Debug)]
pub struct FormatIssue {
    pub field: String,
    pub expected: String,
    pub actual: String,
    pub severity: IssueSeverity,
}

#[derive(Debug)]
pub enum IssueSeverity {
    Warning,
    Error,
    Fatal,
}

pub fn validate_compatibility(
    caller: &CallerProfile,
    target: &ResolvedEndpoint,
    requested_skill: Option<&str>,
    registry: &RegistryClient,
) -> Result<CompatibilityResult, ConnectError> {
    let mut warnings = Vec::new();
    let mut format_issues = Vec::new();

    // Protocol version compatibility check
    let protocol_match = check_protocol_version(
        &caller.protocol_version,
        &target.protocol_version,
    );

    if let ProtocolMatch::Incompatible { ref reason, .. } = protocol_match {
        return Ok(CompatibilityResult {
            compatible: false,
            protocol_match,
            skill_match: None,
            format_issues: vec![FormatIssue {
                field: "protocol_version".into(),
                expected: caller.protocol_version.clone(),
                actual: target.protocol_version.clone(),
                severity: IssueSeverity::Fatal,
            }],
            warnings,
        });
    }

    if matches!(protocol_match, ProtocolMatch::BackwardCompatible { .. }) {
        warnings.push(format!(
            "Target uses older protocol version {}; some features may be unavailable",
            target.protocol_version
        ));
    }

    // Skill match validation
    let skill_match = if let Some(skill_id) = requested_skill {
        let agent_skills = registry.get_agent_skills_sync(target.agent_id)?;
        let matched = agent_skills.iter().find(|s| {
            s.id == skill_id || s.name.to_lowercase() == skill_id.to_lowercase()
        });

        match matched {
            Some(skill) => {
                let input_ok = validate_input_format(caller, skill);
                let output_ok = validate_output_format(caller, skill);

                if !input_ok {
                    format_issues.push(FormatIssue {
                        field: "input_format".into(),
                        expected: format!("{:?}", caller.expected_input_modes),
                        actual: format!("{:?}", skill.input_modes),
                        severity: IssueSeverity::Error,
                    });
                }

                if !output_ok {
                    format_issues.push(FormatIssue {
                        field: "output_format".into(),
                        expected: format!("{:?}", caller.expected_output_modes),
                        actual: format!("{:?}", skill.output_modes),
                        severity: IssueSeverity::Warning,
                    });
                }

                Some(SkillMatch {
                    skill_id: skill.id.clone(),
                    skill_name: skill.name.clone(),
                    confidence: 1.0,
                    input_compatible: input_ok,
                    output_compatible: output_ok,
                })
            }
            None => {
                return Ok(CompatibilityResult {
                    compatible: false,
                    protocol_match,
                    skill_match: None,
                    format_issues: vec![FormatIssue {
                        field: "skill".into(),
                        expected: skill_id.to_string(),
                        actual: "not_found".into(),
                        severity: IssueSeverity::Fatal,
                    }],
                    warnings,
                });
            }
        }
    } else {
        None
    };

    let has_fatal = format_issues.iter().any(|i| matches!(i.severity, IssueSeverity::Fatal));
    let has_error = format_issues.iter().any(|i| matches!(i.severity, IssueSeverity::Error));

    Ok(CompatibilityResult {
        compatible: !has_fatal && !has_error,
        protocol_match,
        skill_match,
        format_issues,
        warnings,
    })
}

fn check_protocol_version(caller_ver: &str, target_ver: &str) -> ProtocolMatch {
    let caller_parts: Vec<u32> = caller_ver.split('.').filter_map(|s| s.parse().ok()).collect();
    let target_parts: Vec<u32> = target_ver.split('.').filter_map(|s| s.parse().ok()).collect();

    if caller_parts.is_empty() || target_parts.is_empty() {
        return ProtocolMatch::Incompatible {
            target_version: target_ver.to_string(),
            caller_version: caller_ver.to_string(),
            reason: "Unparseable version string".into(),
        };
    }

    // Same major version = compatible
    if caller_parts[0] == target_parts[0] {
        if caller_ver == target_ver {
            ProtocolMatch::Exact
        } else if caller_parts > target_parts {
            ProtocolMatch::BackwardCompatible {
                target_version: target_ver.to_string(),
                caller_version: caller_ver.to_string(),
            }
        } else {
            ProtocolMatch::ForwardCompatible {
                target_version: target_ver.to_string(),
                caller_version: caller_ver.to_string(),
            }
        }
    } else {
        ProtocolMatch::Incompatible {
            target_version: target_ver.to_string(),
            caller_version: caller_ver.to_string(),
            reason: format!(
                "Major version mismatch: caller={}, target={}",
                caller_parts[0], target_parts[0]
            ),
        }
    }
}
```

#### Step 3: Negotiate Auth

Auth negotiation is the most complex step in the pipeline. The broker supports four auth strategies depending on the caller's credentials, the target's requirements, and the trust relationship between them.

**Auth Strategies:**

| Strategy | Description | When Used | Latency Impact | Security Level |
|----------|-------------|-----------|---------------|---------------|
| **Passthrough** | Caller already has valid credentials for the target. Broker forwards them unmodified. | Caller has pre-registered API key or bearer token for the target agent. | +0ms (no exchange needed) | Medium — credentials traverse the broker |
| **Exchange** | Caller authenticates with the broker; broker exchanges for target-specific credentials via OAuth2 / SAML / OIDC. | Target requires OAuth2 and caller does not have a token. Broker is registered as an OAuth client with the target's auth provider. | +50–200ms (token exchange round-trip) | High — broker manages token lifecycle |
| **Broker-minted** | Broker mints a short-lived, scoped connection token signed with the broker's key. Target trusts broker-signed tokens. | Target has opted into AgentRank's trust federation. No per-caller credential management needed. | +5–20ms (local token mint) | Very high — target delegates trust to broker |
| **Mutual TLS** | Broker establishes mTLS connection with target using platform-managed certificates. | Enterprise targets requiring certificate-based auth. | +30–100ms (TLS handshake with client cert) | Very high — cryptographic identity on both sides |

```rust
#[derive(Debug, Clone)]
pub enum AuthStrategy {
    Passthrough {
        credentials: EncryptedCredentials,
    },
    Exchange {
        token_url: String,
        client_id: String,
        client_secret: EncryptedString,
        scopes: Vec<String>,
        grant_type: OAuthGrantType,
    },
    BrokerMinted {
        issuer: String,
        audience: String,
        ttl: Duration,
        scopes: Vec<String>,
        signing_key_id: String,
    },
    MutualTls {
        client_cert_path: String,
        client_key_path: String,
        ca_cert_path: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum OAuthGrantType {
    ClientCredentials,
    TokenExchange { subject_token_type: String },
    JwtBearer,
}

#[derive(Debug)]
pub struct NegotiatedAuth {
    pub strategy_used: AuthStrategyType,
    pub credentials: EncryptedCredentials,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub scopes_granted: Vec<String>,
    pub negotiation_latency_ms: f64,
}

pub async fn negotiate_auth(
    caller: &CallerProfile,
    target: &ResolvedEndpoint,
    strategy_config: &AuthConfig,
) -> Result<NegotiatedAuth, ConnectError> {
    let start = Instant::now();

    // Strategy selection priority
    let strategies_to_try = select_auth_strategies(caller, target, strategy_config);

    let mut last_error = None;

    for strategy in &strategies_to_try {
        match execute_auth_strategy(strategy, caller, target).await {
            Ok(creds) => {
                let elapsed = start.elapsed();
                return Ok(NegotiatedAuth {
                    strategy_used: strategy.strategy_type(),
                    credentials: creds.credentials,
                    expires_at: creds.expires_at,
                    scopes_granted: creds.scopes,
                    negotiation_latency_ms: elapsed.as_secs_f64() * 1000.0,
                });
            }
            Err(e) => {
                tracing::warn!(
                    strategy = ?strategy.strategy_type(),
                    error = %e,
                    "Auth strategy failed, trying next"
                );
                last_error = Some(e);
            }
        }
    }

    Err(ConnectError::AuthFailed {
        agent_id: target.agent_id,
        strategies_tried: strategies_to_try.iter().map(|s| s.strategy_type()).collect(),
        last_error: last_error.map(|e| e.to_string()).unwrap_or_default(),
    })
}

fn select_auth_strategies(
    caller: &CallerProfile,
    target: &ResolvedEndpoint,
    config: &AuthConfig,
) -> Vec<AuthStrategy> {
    let mut strategies = Vec::new();

    // If caller has pre-registered credentials for this agent, try passthrough first
    if let Some(creds) = caller.stored_credentials.get(&target.agent_id) {
        strategies.push(AuthStrategy::Passthrough {
            credentials: creds.clone(),
        });
    }

    // If target trusts broker-minted tokens, try that next (lowest latency)
    if target.auth_schemes.iter().any(|s| matches!(s, AuthScheme::BearerToken))
        && config.broker_mint_enabled_for(target.agent_id)
    {
        strategies.push(AuthStrategy::BrokerMinted {
            issuer: config.broker_issuer.clone(),
            audience: target.endpoint_url.clone(),
            ttl: Duration::from_secs(300),
            scopes: vec!["agent:connect".into()],
            signing_key_id: config.active_signing_key.clone(),
        });
    }

    // If target has OAuth2 config and broker is registered as client
    for scheme in &target.auth_schemes {
        if let AuthScheme::OAuth2 { token_url, scopes } = scheme {
            if let Some(client_config) = config.oauth_clients.get(token_url) {
                strategies.push(AuthStrategy::Exchange {
                    token_url: token_url.clone(),
                    client_id: client_config.client_id.clone(),
                    client_secret: client_config.client_secret.clone(),
                    scopes: scopes.clone(),
                    grant_type: OAuthGrantType::ClientCredentials,
                });
            }
        }
    }

    // mTLS as final fallback for enterprise targets
    for scheme in &target.auth_schemes {
        if let AuthScheme::MutualTls { .. } = scheme {
            if let Some(cert_config) = config.mtls_certs.get(&target.agent_id) {
                strategies.push(AuthStrategy::MutualTls {
                    client_cert_path: cert_config.cert_path.clone(),
                    client_key_path: cert_config.key_path.clone(),
                    ca_cert_path: cert_config.ca_path.clone(),
                });
            }
        }
    }

    strategies
}
```

#### Step 4: Apply Policy

Policy enforcement gates whether a connection is allowed, and under what constraints. Policies are defined at the tenant, caller, and target level.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub rules_evaluated: Vec<EvaluatedRule>,
    pub constraints: Vec<ConnectionConstraint>,
    pub denied_reason: Option<String>,
    pub evaluation_time_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatedRule {
    pub rule_id: String,
    pub rule_type: PolicyRuleType,
    pub matched: bool,
    pub action: PolicyAction,
    pub source: PolicySource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyRuleType {
    RateLimit {
        window: Duration,
        max_requests: u64,
        current_count: u64,
    },
    TrustFloor {
        minimum_tier: TrustTier,
        actual_tier: TrustTier,
    },
    AllowList {
        allowed_agents: Vec<Uuid>,
    },
    DenyList {
        denied_agents: Vec<Uuid>,
        denied_providers: Vec<String>,
    },
    GeoRestriction {
        allowed_regions: Vec<String>,
        caller_region: String,
    },
    ContentFilter {
        blocked_categories: Vec<String>,
        task_category: Option<String>,
    },
    CostLimit {
        max_cost_per_request: f64,
        estimated_cost: f64,
        currency: String,
    },
    TimeWindow {
        allowed_hours_utc: (u8, u8),
        current_hour_utc: u8,
    },
    ProtocolConstraint {
        required_version_min: String,
        actual_version: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    Allow,
    Deny { reason: String },
    AllowWithConstraints { constraints: Vec<ConnectionConstraint> },
    Audit { allow: bool, audit_reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicySource {
    Platform,
    Tenant { tenant_id: Uuid },
    CallerPolicy { caller_id: Uuid },
    TargetPolicy { target_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionConstraint {
    MaxDuration(Duration),
    MaxMessages(u64),
    MaxPayloadBytes(u64),
    RequireEncryption,
    AuditLog,
    NoDataRetention,
    SandboxMode,
}

pub struct PolicyEngine {
    rules_store: Arc<RulesStore>,
    rate_limiter: Arc<RateLimiter>,
    metrics: Arc<PolicyMetrics>,
}

impl PolicyEngine {
    pub async fn evaluate(
        &self,
        caller: &CallerProfile,
        target: &ResolvedEndpoint,
        request: &ConnectRequest,
    ) -> PolicyDecision {
        let start = Instant::now();
        let mut evaluated_rules = Vec::new();
        let mut constraints = Vec::new();
        let mut denied = false;
        let mut deny_reason = None;

        // Load rules from all sources, ordered by priority
        let rules = self.rules_store.load_rules(
            caller.tenant_id,
            caller.caller_id,
            target.agent_id,
        ).await;

        for rule in &rules {
            let result = self.evaluate_rule(rule, caller, target, request).await;
            evaluated_rules.push(result.clone());

            match &result.action {
                PolicyAction::Deny { reason } => {
                    denied = true;
                    deny_reason = Some(reason.clone());
                    break; // Short-circuit on first deny
                }
                PolicyAction::AllowWithConstraints { constraints: c } => {
                    constraints.extend(c.clone());
                }
                PolicyAction::Audit { allow, audit_reason } => {
                    if !allow {
                        denied = true;
                        deny_reason = Some(audit_reason.clone());
                    }
                    self.metrics.record_audit_event(rule, caller, target).await;
                }
                PolicyAction::Allow => {}
            }
        }

        let elapsed = start.elapsed();
        self.metrics.record_evaluation(elapsed, denied);

        PolicyDecision {
            allowed: !denied,
            rules_evaluated: evaluated_rules,
            constraints,
            denied_reason: deny_reason,
            evaluation_time_us: elapsed.as_micros() as u64,
        }
    }
}
```

### 14.4 Connection Decision Flow

The full connection decision flow integrates all pipeline steps with fallback behavior. When the primary connection path fails, the broker attempts intelligent recovery before returning an error.

```
┌──────────────────────────────────────────────────────────────────────┐
│                   CONNECTION DECISION FLOW                           │
│                                                                      │
│  1. Receive ConnectRequest                                           │
│     │                                                                │
│     ▼                                                                │
│  2. Resolve endpoint from registry                                   │
│     │                                                                │
│     ├── FAIL: Agent not found ──────────────► Return NOT_FOUND       │
│     ├── FAIL: Agent delisted ───────────────► Return DELISTED        │
│     │                                                                │
│     ▼                                                                │
│  3. Check agent health status                                        │
│     │                                                                │
│     ├── UNHEALTHY (>24h) ───────────────────► Return UNHEALTHY       │
│     ├── DEGRADED ───────────────────────────► Continue with warning   │
│     ├── UNKNOWN ────────────────────────────► Attempt fresh probe     │
│     │   └── Probe fails ────────────────────► Return UNHEALTHY       │
│     │                                                                │
│     ▼                                                                │
│  4. Validate compatibility                                           │
│     │                                                                │
│     ├── Protocol incompatible ──────────────► Return INCOMPATIBLE    │
│     ├── Skill not found ────────────────────► Return SKILL_NOT_FOUND │
│     ├── Format issues (warnings only) ──────► Continue with warnings │
│     │                                                                │
│     ▼                                                                │
│  5. Apply policy rules                                               │
│     │                                                                │
│     ├── Denied by rate limit ───────────────► Return RATE_LIMITED    │
│     ├── Denied by trust floor ──────────────► Return TRUST_TOO_LOW   │
│     ├── Denied by deny list ────────────────► Return POLICY_DENIED   │
│     ├── Denied by geo restriction ──────────► Return GEO_RESTRICTED  │
│     │                                                                │
│     ▼                                                                │
│  6. Negotiate authentication                                         │
│     │                                                                │
│     ├── All strategies failed ──────────────► Return AUTH_FAILED     │
│     │                                                                │
│     ▼                                                                │
│  7. Establish connection (based on mediation depth)                   │
│     │                                                                │
│     ├── Connection refused ─────────────────► Retry with backoff     │
│     │   ├── Retry 1 (100ms delay) ──────────► Attempt 2              │
│     │   ├── Retry 2 (500ms delay) ──────────► Attempt 3              │
│     │   └── Retry 3 (2000ms delay) ─────────► Return CONNECT_FAILED │
│     │                                                                │
│     ├── Timeout ────────────────────────────► Try alternate endpoint  │
│     │   └── No alternates ──────────────────► Return TIMEOUT         │
│     │                                                                │
│     ▼                                                                │
│  8. Return session token + connection metadata                       │
│     │                                                                │
│     ▼                                                                │
│  9. Emit telemetry (async, non-blocking)                             │
│     └── connection_time_ms, auth_strategy, policy_constraints,       │
│         endpoint_region, protocol_version, trust_tier                │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

```rust
pub struct ConnectionResult {
    pub session_id: Uuid,
    pub session_token: String,
    pub endpoint_url: String,
    pub mediation_mode: MediationMode,
    pub auth_strategy_used: AuthStrategyType,
    pub constraints: Vec<ConnectionConstraint>,
    pub warnings: Vec<String>,
    pub timing: ConnectionTiming,
}

pub struct ConnectionTiming {
    pub total_ms: f64,
    pub resolve_ms: f64,
    pub validate_ms: f64,
    pub auth_ms: f64,
    pub policy_ms: f64,
    pub connect_ms: f64,
    pub retries: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum MediationMode {
    HandshakeOnly,
    ProxyFirst,
    FullProxy,
    ManagedSession,
}

pub async fn connect_with_fallback(
    broker: &ConnectionBroker,
    request: ConnectRequest,
) -> Result<ConnectionResult, ConnectError> {
    let start = Instant::now();

    // Step 1: Resolve endpoint
    let resolve_start = Instant::now();
    let resolved = broker.resolver.resolve(
        request.target_agent_id,
        request.caller_region.as_deref(),
    ).await?;
    let resolve_ms = resolve_start.elapsed().as_secs_f64() * 1000.0;

    // Step 2: Validate compatibility
    let validate_start = Instant::now();
    let compat = validate_compatibility(
        &request.caller_profile,
        &resolved,
        request.requested_skill.as_deref(),
        &broker.registry,
    )?;
    let validate_ms = validate_start.elapsed().as_secs_f64() * 1000.0;

    if !compat.compatible {
        return Err(ConnectError::Incompatible {
            agent_id: request.target_agent_id,
            issues: compat.format_issues,
        });
    }

    // Step 3: Apply policy
    let policy_start = Instant::now();
    let policy = broker.policy_engine.evaluate(
        &request.caller_profile,
        &resolved,
        &request,
    ).await;
    let policy_ms = policy_start.elapsed().as_secs_f64() * 1000.0;

    if !policy.allowed {
        return Err(ConnectError::PolicyDenied {
            agent_id: request.target_agent_id,
            reason: policy.denied_reason.unwrap_or_default(),
        });
    }

    // Step 4: Negotiate auth
    let auth_start = Instant::now();
    let auth = negotiate_auth(
        &request.caller_profile,
        &resolved,
        &broker.auth_config,
    ).await?;
    let auth_ms = auth_start.elapsed().as_secs_f64() * 1000.0;

    // Step 5: Determine mediation mode
    let mediation = broker.select_mediation_mode(&request, &resolved, &policy);

    // Step 6: Establish connection with retry
    let connect_start = Instant::now();
    let mut retries = 0u32;
    let max_retries = 3;
    let backoff_ms = [100, 500, 2000];

    let session = loop {
        match broker.establish_session(
            &resolved,
            &auth,
            mediation,
            &policy.constraints,
        ).await {
            Ok(session) => break session,
            Err(e) if retries < max_retries && e.is_retryable() => {
                retries += 1;
                let delay = Duration::from_millis(backoff_ms[retries as usize - 1]);
                tokio::time::sleep(delay).await;

                tracing::warn!(
                    retry = retries,
                    delay_ms = delay.as_millis(),
                    error = %e,
                    "Connection attempt failed, retrying"
                );
                continue;
            }
            Err(e) => return Err(e),
        }
    };
    let connect_ms = connect_start.elapsed().as_secs_f64() * 1000.0;

    let total_ms = start.elapsed().as_secs_f64() * 1000.0;

    let result = ConnectionResult {
        session_id: session.id,
        session_token: session.token,
        endpoint_url: resolved.endpoint_url.clone(),
        mediation_mode: mediation,
        auth_strategy_used: auth.strategy_used,
        constraints: policy.constraints,
        warnings: compat.warnings,
        timing: ConnectionTiming {
            total_ms,
            resolve_ms,
            validate_ms,
            auth_ms,
            policy_ms,
            connect_ms,
            retries,
        },
    };

    // Step 7: Emit telemetry (non-blocking)
    let telemetry = ConnectionTelemetry {
        session_id: session.id,
        caller_id: request.caller_profile.caller_id,
        target_id: request.target_agent_id,
        timing: result.timing.clone(),
        mediation_mode: mediation,
        auth_strategy: auth.strategy_used,
        trust_tier: resolved.trust_tier,
        endpoint_region: resolved.region.clone(),
    };
    tokio::spawn(async move {
        broker.telemetry.emit(telemetry).await;
    });

    Ok(result)
}
```

### 14.5 Connect API

#### `POST /v1/connect`

The Connect API is the primary interface for establishing brokered connections between a caller and a target agent.

**Request:**

```json
{
  "target_agent_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "requested_skill": "code-review",
  "mode": "brokered",
  "mediation_preference": "auto",
  "caller_context": {
    "caller_agent_id": "98765432-1abc-def0-1234-567890abcdef",
    "protocol_version": "1.0.0",
    "expected_input_modes": ["text"],
    "expected_output_modes": ["text", "data"],
    "caller_region": "us-east-1",
    "tenant_id": "tenant-acme-corp",
    "session_purpose": "Automated PR review pipeline",
    "max_latency_ms": 500,
    "max_cost_usd": 0.10
  },
  "auth": {
    "strategy_preference": ["broker_minted", "exchange", "passthrough"],
    "stored_credentials": null,
    "oauth_context": {
      "subject_token": "eyJhbGciOiJSUzI1NiIs...",
      "subject_token_type": "urn:ietf:params:oauth:token-type:jwt"
    }
  },
  "policy_overrides": {
    "require_audit_log": true,
    "max_session_duration_seconds": 300,
    "sandbox_mode": false
  },
  "idempotency_key": "connect-req-20260323-abc123"
}
```

**Success Response (200 OK):**

```json
{
  "session_id": "sess-f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "session_token": "agr_sess_v1.eyJzZXNzaW9uX2lkIjoiZjQ3YWMxMGItNThjYy00Mzcy...",
  "endpoint": {
    "url": "https://code-review-agent.example.com/a2a",
    "protocol_version": "1.0.0",
    "region": "us-east-1"
  },
  "mediation": {
    "mode": "proxy_first",
    "proxy_url": "wss://connect.agentrank.dev/proxy/sess-f47ac10b",
    "direct_after_handshake": true
  },
  "auth": {
    "strategy_used": "broker_minted",
    "token_expires_at": "2026-03-23T15:05:00Z",
    "scopes_granted": ["agent:connect", "skill:code-review"]
  },
  "compatibility": {
    "protocol_match": "exact",
    "skill_match": {
      "skill_id": "code-review",
      "skill_name": "Code Review Analysis",
      "confidence": 1.0,
      "input_compatible": true,
      "output_compatible": true
    },
    "warnings": []
  },
  "constraints": [
    {
      "type": "max_duration",
      "value_seconds": 300
    },
    {
      "type": "audit_log",
      "enabled": true
    }
  ],
  "target_agent": {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "CodeReview Pro",
    "provider": "DevTools Inc.",
    "trust_tier": "verified",
    "health_status": "healthy",
    "agentrank_score": 0.87
  },
  "timing": {
    "total_ms": 142.3,
    "resolve_ms": 12.1,
    "validate_ms": 3.4,
    "auth_ms": 18.7,
    "policy_ms": 2.1,
    "connect_ms": 105.2,
    "retries": 0
  },
  "request_id": "req-20260323-xyz789"
}
```

**Failure Response (422 Unprocessable Entity — Compatibility Failure):**

```json
{
  "error": {
    "code": "INCOMPATIBLE",
    "message": "Target agent is not compatible with the requested connection",
    "details": {
      "protocol_match": {
        "status": "incompatible",
        "caller_version": "2.0.0",
        "target_version": "1.0.0",
        "reason": "Major version mismatch: caller=2, target=1"
      },
      "format_issues": [
        {
          "field": "protocol_version",
          "expected": "2.0.0",
          "actual": "1.0.0",
          "severity": "fatal"
        }
      ]
    }
  },
  "suggestions": [
    {
      "type": "alternative_agent",
      "agent_id": "bbbbbbbb-cccc-dddd-eeee-ffffffffffff",
      "name": "CodeReview Pro v2",
      "protocol_version": "2.0.0",
      "agentrank_score": 0.82,
      "reason": "Compatible alternative with same skill set"
    }
  ],
  "request_id": "req-20260323-xyz789"
}
```

**Failure Response (403 Forbidden — Policy Denied):**

```json
{
  "error": {
    "code": "POLICY_DENIED",
    "message": "Connection denied by policy",
    "details": {
      "denied_by_rule": "geo-restriction-eu-only",
      "rule_type": "geo_restriction",
      "source": "target_policy",
      "reason": "Target agent restricts connections to EU regions only. Caller region: us-east-1"
    }
  },
  "suggestions": [
    {
      "type": "alternative_endpoint",
      "message": "Target agent has an EU endpoint. Retry with caller_region=eu-west-1 or use a proxy in an EU region."
    }
  ],
  "request_id": "req-20260323-xyz789"
}
```

**Failure Response (401 Unauthorized — Auth Failed):**

```json
{
  "error": {
    "code": "AUTH_FAILED",
    "message": "All authentication strategies failed",
    "details": {
      "strategies_tried": [
        {
          "strategy": "broker_minted",
          "error": "Target does not accept broker-minted tokens"
        },
        {
          "strategy": "exchange",
          "error": "OAuth token exchange returned 401: invalid_client"
        }
      ],
      "target_auth_schemes": ["oauth2", "api_key"],
      "suggestion": "Register API key for this agent in your credential store"
    }
  },
  "request_id": "req-20260323-xyz789"
}
```

### 14.6 Outcome Feedback API

#### `POST /v1/outcomes`

After a connection completes (or fails), the caller or broker submits an outcome report. This is the single most important API for the flywheel — it is how AgentRank learns which agents actually deliver value.

Outcome reports are cryptographically signed by the submitter to prevent manipulation.

**Request:**

```json
{
  "session_id": "sess-f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "outcome": {
    "value": "completed",
    "quality_score": 0.92,
    "quality_dimensions": {
      "task_completion": 0.95,
      "response_quality": 0.90,
      "latency_satisfaction": 0.88,
      "format_compliance": 1.0,
      "cost_efficiency": 0.85
    },
    "task_summary": "Reviewed 3 files, found 2 security issues and 5 style violations",
    "duration_ms": 4520,
    "tokens_consumed": {
      "input": 12400,
      "output": 3200
    },
    "cost_usd": 0.042
  },
  "reporter": {
    "agent_id": "98765432-1abc-def0-1234-567890abcdef",
    "reporter_type": "caller"
  },
  "metadata": {
    "workflow_id": "wf-pr-review-456",
    "workflow_step": "security-review",
    "retry_count": 0,
    "environment": "production"
  },
  "signature": {
    "algorithm": "Ed25519",
    "public_key": "MCowBQYDK2VwAyEA1234567890abcdef1234567890abcdef1234567890ab",
    "signature": "MEUCIQD1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef==",
    "signed_fields": ["session_id", "outcome.value", "outcome.quality_score", "outcome.duration_ms", "reporter.agent_id"],
    "timestamp": "2026-03-23T14:07:45Z"
  }
}
```

**Outcome Values:**

| Value | Description | Rank Impact | Signal Weight |
|-------|-------------|-------------|--------------|
| `connected` | Connection established but task not yet started or still in progress | Neutral (+0.00) | Low — partial signal only |
| `completed` | Task completed successfully, caller satisfied with result | Positive (+0.02 to +0.05 per report) | High — primary positive signal |
| `partial` | Task partially completed, some useful output but not fully satisfactory | Slightly positive (+0.005 to +0.01) | Medium — indicates capability but not reliability |
| `timed_out` | Connection or task exceeded the allowed time limit | Negative (-0.01 to -0.03) | Medium — may indicate performance issues |
| `rejected` | Target agent rejected the task (e.g., outside its capability scope) | Slightly negative (-0.005) | Low — may be caller error, not agent quality |
| `auth_failed` | Authentication failed during the session (post-connection) | Negative (-0.02) | Medium — indicates configuration or trust issue |
| `error` | Target agent returned an error during task execution | Negative (-0.01 to -0.04) | High — indicates reliability issue |
| `incompatible` | Runtime incompatibility discovered (not caught by pre-connect validation) | Negative (-0.01) | Medium — indicates metadata accuracy issue |
| `cancelled` | Caller cancelled the task before completion | Neutral (+0.00) | None — caller-initiated, no signal about agent quality |

**Rank impact calculation:** The actual rank impact of each outcome report is not a fixed value — it depends on the trust tier of the reporter, the freshness of the report, the consistency with other reports, and the number of total reports for the agent. The values above are approximate ranges for a single report from a Verified reporter.

**Response (202 Accepted):**

```json
{
  "outcome_id": "out-c47ac10b-58cc-4372-a567-0e02b2c3d479",
  "accepted": true,
  "signature_verified": true,
  "rank_impact_estimate": {
    "agent_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "current_outcome_score": 0.84,
    "estimated_new_outcome_score": 0.845,
    "change": "+0.005",
    "reporter_trust_weight": 0.85,
    "note": "Positive outcome from verified reporter. Impact reduced by existing high outcome score (diminishing returns)."
  },
  "processing": {
    "queued_at": "2026-03-23T14:07:45.123Z",
    "estimated_rank_update_at": "2026-03-23T14:12:00Z",
    "pipeline": "outcome-ingest-v2"
  }
}
```

---

## 15. Liveness & Verification System

### 15.1 Health Check Architecture

An agent that appears in search results but is actually offline is worse than an agent that does not appear at all — it wastes the caller's time, damages trust in the platform, and degrades the ranking system's credibility. Liveness is not a nice-to-have feature. It is a core ranking input.

AgentRank's liveness system uses a four-level probe hierarchy, each level testing a deeper aspect of agent health. Higher levels are more expensive and run less frequently, but provide stronger evidence.

**Probe Hierarchy:**

| Level | Name | What It Tests | Method | Frequency (Healthy) | Frequency (Degraded) | Latency Budget | Failure Threshold |
|-------|------|--------------|--------|---------------------|---------------------|---------------|------------------|
| **L1** | **Ping** | TCP connectivity + TLS handshake | TCP connect + TLS negotiation to agent endpoint | Every 5 min | Every 1 min | 5s | 3 consecutive failures → Degraded |
| **L2** | **Card Fetch** | Agent Card availability and parseability | HTTP GET `/.well-known/agent.json`, validate JSON schema | Every 15 min | Every 5 min | 10s | 2 consecutive failures → Degraded |
| **L3** | **Endpoint Check** | A2A endpoint responsiveness | POST to A2A endpoint with `agent/authenticatedExtendedCard` or `tasks/get` with empty task | Every 1 hour | Every 15 min | 15s | 2 consecutive failures → Unhealthy |
| **L4** | **Capability Probe** | Actual skill execution correctness | Send a test task for a claimed capability, evaluate response quality | Every 24 hours | Every 6 hours | 60s | 1 failure → Degraded; 3 consecutive → Unhealthy |

**Multi-region probing:**

To distinguish between a truly unhealthy agent and a region-specific network issue, all probes are executed from a minimum of three geographically distributed regions.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    MULTI-REGION PROBE ARCHITECTURE                      │
│                                                                         │
│    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐            │
│    │  us-east-1   │    │  eu-west-1   │    │  ap-south-1  │            │
│    │              │    │              │    │              │            │
│    │  Probe Node  │    │  Probe Node  │    │  Probe Node  │            │
│    │  ┌────────┐  │    │  ┌────────┐  │    │  ┌────────┐  │            │
│    │  │L1 Ping │  │    │  │L1 Ping │  │    │  │L1 Ping │  │            │
│    │  │L2 Card │  │    │  │L2 Card │  │    │  │L2 Card │  │            │
│    │  │L3 Endpt│  │    │  │L3 Endpt│  │    │  │L3 Endpt│  │            │
│    │  │L4 Capab│  │    │  │L4 Capab│  │    │  │L4 Capab│  │            │
│    │  └───┬────┘  │    │  └───┬────┘  │    │  └───┬────┘  │            │
│    └──────┼───────┘    └──────┼───────┘    └──────┼───────┘            │
│           │                   │                   │                     │
│           ▼                   ▼                   ▼                     │
│    ┌──────────────────────────────────────────────────────┐            │
│    │              Probe Aggregation Layer                  │            │
│    │                                                      │            │
│    │  Consensus Logic:                                    │            │
│    │  • 3/3 healthy  → HEALTHY                            │            │
│    │  • 2/3 healthy  → HEALTHY (with region note)         │            │
│    │  • 1/3 healthy  → DEGRADED                           │            │
│    │  • 0/3 healthy  → UNHEALTHY                          │            │
│    │  • All timeout   → UNKNOWN (network issue?)          │            │
│    │                                                      │            │
│    │  Weighted by:                                        │            │
│    │  • Historical reliability of each probe node         │            │
│    │  • Geographic proximity to agent's declared region   │            │
│    │  • Time since last successful probe                  │            │
│    └──────────────────────┬───────────────────────────────┘            │
│                           │                                             │
│                           ▼                                             │
│    ┌──────────────────────────────────────────────────────┐            │
│    │              Health Status Update                     │            │
│    │                                                      │            │
│    │  → Update agent health in registry                   │            │
│    │  → Emit health event to Kafka                        │            │
│    │  → Update ranking signal (operational quality)       │            │
│    │  → Trigger alerts if state transition                │            │
│    └──────────────────────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────────────────────┘
```

**Probe result data model:**

```rust
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub probe_id: Uuid,
    pub agent_id: Uuid,
    pub probe_level: ProbeLevel,
    pub probe_region: String,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub latency: Duration,
    pub status: ProbeStatus,
    pub details: ProbeDetails,
    pub raw_response_hash: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProbeLevel {
    L1Ping,
    L2CardFetch,
    L3EndpointCheck,
    L4CapabilityProbe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProbeStatus {
    Success,
    Timeout { budget_ms: u64 },
    ConnectionRefused,
    TlsError { error: String },
    HttpError { status_code: u16, body_snippet: Option<String> },
    ParseError { error: String },
    QualityFail { score: f64, threshold: f64 },
    Error { error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProbeDetails {
    L1 {
        tcp_connect_ms: f64,
        tls_handshake_ms: f64,
        tls_version: Option<String>,
        certificate_expiry: Option<DateTime<Utc>>,
        ip_address: String,
    },
    L2 {
        http_status: u16,
        content_type: Option<String>,
        card_valid: bool,
        card_hash: String,
        card_changed_since_last: bool,
        schema_errors: Vec<String>,
        skills_count: u32,
        response_size_bytes: u64,
    },
    L3 {
        http_status: u16,
        a2a_response_valid: bool,
        response_time_ms: f64,
        protocol_version_reported: Option<String>,
        supported_methods: Vec<String>,
    },
    L4 {
        skill_tested: String,
        test_prompt_hash: String,
        response_quality_score: f64,
        response_time_ms: f64,
        response_format_valid: bool,
        safety_check_passed: bool,
        tokens_consumed: Option<u64>,
    },
}

pub struct ProbeExecutor {
    http_client: reqwest::Client,
    tls_config: Arc<TlsProbeConfig>,
    test_prompts: Arc<TestPromptStore>,
    metrics: Arc<ProbeMetrics>,
}

impl ProbeExecutor {
    pub async fn probe_l1(&self, agent: &AgentRecord) -> ProbeResult {
        let probe_id = Uuid::new_v4();
        let initiated_at = Utc::now();

        let endpoint = &agent.endpoint_url;
        let parsed = match url::Url::parse(endpoint) {
            Ok(u) => u,
            Err(e) => {
                return ProbeResult {
                    probe_id,
                    agent_id: agent.id,
                    probe_level: ProbeLevel::L1Ping,
                    probe_region: self.region().to_string(),
                    initiated_at,
                    completed_at: Utc::now(),
                    latency: Duration::ZERO,
                    status: ProbeStatus::Error {
                        error: format!("Invalid URL: {e}"),
                    },
                    details: ProbeDetails::L1 {
                        tcp_connect_ms: 0.0,
                        tls_handshake_ms: 0.0,
                        tls_version: None,
                        certificate_expiry: None,
                        ip_address: String::new(),
                    },
                    raw_response_hash: None,
                };
            }
        };

        let host = parsed.host_str().unwrap_or("");
        let port = parsed.port_or_known_default().unwrap_or(443);
        let addr = format!("{host}:{port}");

        let tcp_start = Instant::now();
        let tcp_result = tokio::time::timeout(
            Duration::from_secs(5),
            tokio::net::TcpStream::connect(&addr),
        ).await;

        match tcp_result {
            Ok(Ok(stream)) => {
                let tcp_connect_ms = tcp_start.elapsed().as_secs_f64() * 1000.0;
                let ip_address = stream
                    .peer_addr()
                    .map(|a| a.ip().to_string())
                    .unwrap_or_default();

                let tls_start = Instant::now();
                let tls_result = self.tls_config
                    .connector
                    .connect(host.try_into().unwrap(), stream)
                    .await;

                match tls_result {
                    Ok(tls_stream) => {
                        let tls_handshake_ms = tls_start.elapsed().as_secs_f64() * 1000.0;
                        let tls_version = tls_stream.protocol_version()
                            .map(|v| format!("{v:?}"));
                        let cert_expiry = extract_cert_expiry(&tls_stream);

                        let completed_at = Utc::now();
                        let latency = initiated_at.signed_duration_since(completed_at)
                            .to_std()
                            .unwrap_or(tcp_start.elapsed());

                        ProbeResult {
                            probe_id,
                            agent_id: agent.id,
                            probe_level: ProbeLevel::L1Ping,
                            probe_region: self.region().to_string(),
                            initiated_at,
                            completed_at,
                            latency: tcp_start.elapsed(),
                            status: ProbeStatus::Success,
                            details: ProbeDetails::L1 {
                                tcp_connect_ms,
                                tls_handshake_ms,
                                tls_version,
                                certificate_expiry: cert_expiry,
                                ip_address,
                            },
                            raw_response_hash: None,
                        }
                    }
                    Err(e) => ProbeResult {
                        probe_id,
                        agent_id: agent.id,
                        probe_level: ProbeLevel::L1Ping,
                        probe_region: self.region().to_string(),
                        initiated_at,
                        completed_at: Utc::now(),
                        latency: tcp_start.elapsed(),
                        status: ProbeStatus::TlsError {
                            error: e.to_string(),
                        },
                        details: ProbeDetails::L1 {
                            tcp_connect_ms,
                            tls_handshake_ms: tls_start.elapsed().as_secs_f64() * 1000.0,
                            tls_version: None,
                            certificate_expiry: None,
                            ip_address,
                        },
                        raw_response_hash: None,
                    },
                }
            }
            Ok(Err(e)) => ProbeResult {
                probe_id,
                agent_id: agent.id,
                probe_level: ProbeLevel::L1Ping,
                probe_region: self.region().to_string(),
                initiated_at,
                completed_at: Utc::now(),
                latency: tcp_start.elapsed(),
                status: ProbeStatus::ConnectionRefused,
                details: ProbeDetails::L1 {
                    tcp_connect_ms: tcp_start.elapsed().as_secs_f64() * 1000.0,
                    tls_handshake_ms: 0.0,
                    tls_version: None,
                    certificate_expiry: None,
                    ip_address: String::new(),
                },
                raw_response_hash: None,
            },
            Err(_) => ProbeResult {
                probe_id,
                agent_id: agent.id,
                probe_level: ProbeLevel::L1Ping,
                probe_region: self.region().to_string(),
                initiated_at,
                completed_at: Utc::now(),
                latency: Duration::from_secs(5),
                status: ProbeStatus::Timeout { budget_ms: 5000 },
                details: ProbeDetails::L1 {
                    tcp_connect_ms: 5000.0,
                    tls_handshake_ms: 0.0,
                    tls_version: None,
                    certificate_expiry: None,
                    ip_address: String::new(),
                },
                raw_response_hash: None,
            },
        }
    }

    pub async fn probe_l2(&self, agent: &AgentRecord) -> ProbeResult {
        let probe_id = Uuid::new_v4();
        let initiated_at = Utc::now();

        let card_url = derive_card_url(&agent.endpoint_url);

        let response = tokio::time::timeout(
            Duration::from_secs(10),
            self.http_client.get(&card_url).send(),
        ).await;

        match response {
            Ok(Ok(resp)) => {
                let http_status = resp.status().as_u16();
                let content_type = resp.headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .map(String::from);
                let body_bytes = resp.bytes().await.unwrap_or_default();
                let response_size = body_bytes.len() as u64;
                let card_hash = sha256_hex(&body_bytes);

                let (card_valid, schema_errors, skills_count) = match serde_json::from_slice::<AgentCard>(&body_bytes) {
                    Ok(card) => {
                        let errors = validate_card_schema(&card);
                        let skills = card.skills.len() as u32;
                        (errors.is_empty(), errors, skills)
                    }
                    Err(e) => (false, vec![format!("JSON parse error: {e}")], 0),
                };

                let card_changed = agent.last_card_hash.as_ref()
                    .map(|h| h != &card_hash)
                    .unwrap_or(true);

                ProbeResult {
                    probe_id,
                    agent_id: agent.id,
                    probe_level: ProbeLevel::L2CardFetch,
                    probe_region: self.region().to_string(),
                    initiated_at,
                    completed_at: Utc::now(),
                    latency: Utc::now().signed_duration_since(initiated_at)
                        .to_std().unwrap_or_default(),
                    status: if card_valid && http_status == 200 {
                        ProbeStatus::Success
                    } else {
                        ProbeStatus::HttpError {
                            status_code: http_status,
                            body_snippet: Some(String::from_utf8_lossy(&body_bytes[..256.min(body_bytes.len())]).to_string()),
                        }
                    },
                    details: ProbeDetails::L2 {
                        http_status,
                        content_type,
                        card_valid,
                        card_hash,
                        card_changed_since_last: card_changed,
                        schema_errors,
                        skills_count,
                        response_size_bytes: response_size,
                    },
                    raw_response_hash: Some(sha256_hex(&body_bytes)),
                }
            }
            Ok(Err(e)) => ProbeResult {
                probe_id,
                agent_id: agent.id,
                probe_level: ProbeLevel::L2CardFetch,
                probe_region: self.region().to_string(),
                initiated_at,
                completed_at: Utc::now(),
                latency: Utc::now().signed_duration_since(initiated_at)
                    .to_std().unwrap_or_default(),
                status: ProbeStatus::Error { error: e.to_string() },
                details: ProbeDetails::L2 {
                    http_status: 0,
                    content_type: None,
                    card_valid: false,
                    card_hash: String::new(),
                    card_changed_since_last: false,
                    schema_errors: vec![e.to_string()],
                    skills_count: 0,
                    response_size_bytes: 0,
                },
                raw_response_hash: None,
            },
            Err(_) => ProbeResult {
                probe_id,
                agent_id: agent.id,
                probe_level: ProbeLevel::L2CardFetch,
                probe_region: self.region().to_string(),
                initiated_at,
                completed_at: Utc::now(),
                latency: Duration::from_secs(10),
                status: ProbeStatus::Timeout { budget_ms: 10000 },
                details: ProbeDetails::L2 {
                    http_status: 0,
                    content_type: None,
                    card_valid: false,
                    card_hash: String::new(),
                    card_changed_since_last: false,
                    schema_errors: vec!["Timeout".into()],
                    skills_count: 0,
                    response_size_bytes: 0,
                },
                raw_response_hash: None,
            },
        }
    }
}
```

### 15.2 Status State Machine

Agent health is modeled as a finite state machine with well-defined transitions, each triggered by probe results, administrative actions, or timeout events. The state machine is the single source of truth for whether an agent should appear in search results and at what ranking penalty (if any).

```
┌──────────────────────────────────────────────────────────────────────────┐
│                     AGENT STATUS STATE MACHINE                           │
│                                                                          │
│                         ┌────────┐                                       │
│                    ┌───►│  NEW   │◄──── First discovery                  │
│                    │    └───┬────┘                                       │
│                    │        │                                             │
│                    │        │ L1+L2 pass                                  │
│                    │        ▼                                             │
│                    │    ┌────────────┐                                    │
│      Re-discovery  │    │            │◄──── L1+L2+L3 pass after          │
│      after delist  │    │  HEALTHY   │      degraded/unhealthy           │
│                    │    │            │                                    │
│                    │    └──┬───┬─────┘                                    │
│                    │       │   │                                          │
│                    │       │   │ L1/L2 fail (threshold)                   │
│                    │       │   │ OR L4 quality below threshold            │
│                    │       │   ▼                                          │
│                    │       │ ┌───────────┐                                │
│                    │       │ │           │◄──── Partial probe             │
│                    │       │ │ DEGRADED  │      failures (1/3 regions)    │
│                    │       │ │           │                                │
│                    │       │ └─────┬─────┘                                │
│                    │       │       │                                      │
│                    │       │       │ L3 fail (threshold)                  │
│                    │       │       │ OR all-region L1 fail               │
│                    │       │       ▼                                      │
│                    │       │ ┌───────────┐                                │
│                    │       │ │           │                                │
│                    │       │ │ UNHEALTHY │◄──── All L1/L2/L3 failing     │
│                    │       │ │           │      across all regions        │
│                    │       │ └─────┬─────┘                                │
│                    │       │       │                                      │
│                    │       │       │ Unhealthy > 72 hours                 │
│                    │       │       │ AND no successful probe              │
│                    │       │       ▼                                      │
│                    │       │ ┌───────────┐                                │
│                    │       │ │           │                                │
│                    │       │ │   DEAD    │◄──── Extended unresponsive     │
│                    │       │ │           │      No recovery signal        │
│                    │       │ └─────┬─────┘                                │
│                    │       │       │                                      │
│                    │       │       │ Dead > 30 days                       │
│                    │       │       │ OR manual admin action               │
│                    │       │       ▼                                      │
│                    │       │ ┌───────────┐                                │
│                    └───────┘ │           │                                │
│                              │ DELISTED  │◄──── Removed from index       │
│                              │           │      No longer in search       │
│                              └───────────┘                                │
│                                                                          │
│  Additional transitions:                                                 │
│  • Any state → DELISTED: Admin action (abuse, DMCA, provider request)    │
│  • DEGRADED → HEALTHY: All probes pass for 15 minutes                    │
│  • UNHEALTHY → DEGRADED: Some probes start passing                       │
│  • DEAD → UNHEALTHY: Any probe succeeds (resurrection detection)         │
│  • DELISTED → NEW: Re-discovery after provider fixes issues              │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**State Transition Table:**

| From State | Event | Condition | To State | Side Effects |
|-----------|-------|-----------|----------|-------------|
| **New** | L1+L2 probe success | First successful probe set | Healthy | Index agent, compute initial rank |
| **New** | L1 probe failure | 3 consecutive from all regions | Dead | Skip indexing, schedule retry in 24h |
| **Healthy** | L1 failure | ≥3 consecutive from ≥2 regions | Degraded | Add ranking penalty (-10%), increase probe frequency |
| **Healthy** | L2 failure | Card returns 404/500 × 2 consecutive | Degraded | Flag card issue, notify provider if claimed |
| **Healthy** | L4 quality fail | Quality score < threshold | Degraded | Add capability-specific penalty, schedule re-test |
| **Degraded** | All L1+L2+L3 pass | Sustained 15 min across all regions | Healthy | Remove ranking penalty, restore normal probe frequency |
| **Degraded** | L3 failure | ≥2 consecutive from ≥2 regions | Unhealthy | Add ranking penalty (-30%), suppress from top results |
| **Degraded** | All-region L1 fail | All probes fail from all regions | Unhealthy | Heavy ranking penalty (-50%), add health warning to results |
| **Unhealthy** | Any probe success | Any single probe from any region | Degraded | Begin recovery evaluation, increase probe frequency |
| **Unhealthy** | No recovery | 72 hours with no successful probe | Dead | Remove from active search results, archive |
| **Dead** | Any probe success | Resurrection detected | Unhealthy | Re-enter evaluation pipeline, alert admin |
| **Dead** | No recovery | 30 days with no successful probe | Delisted | Remove from index entirely, free resources |
| **Any** | Admin action | Abuse report confirmed, DMCA, provider request | Delisted | Immediate removal from search, notify provider |
| **Delisted** | Re-discovery | New crawl finds live agent at same endpoint | New | Re-enter full validation pipeline |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    New,
    Healthy,
    Degraded,
    Unhealthy,
    Dead,
    Delisted,
}

impl AgentStatus {
    pub fn ranking_penalty(&self) -> f64 {
        match self {
            AgentStatus::New => 0.05,
            AgentStatus::Healthy => 0.0,
            AgentStatus::Degraded => 0.10,
            AgentStatus::Unhealthy => 0.30,
            AgentStatus::Dead => 1.0,
            AgentStatus::Delisted => 1.0,
        }
    }

    pub fn searchable(&self) -> bool {
        matches!(self, AgentStatus::Healthy | AgentStatus::Degraded | AgentStatus::New)
    }

    pub fn probe_frequency(&self) -> Duration {
        match self {
            AgentStatus::New => Duration::from_secs(60),
            AgentStatus::Healthy => Duration::from_secs(300),
            AgentStatus::Degraded => Duration::from_secs(60),
            AgentStatus::Unhealthy => Duration::from_secs(300),
            AgentStatus::Dead => Duration::from_secs(3600),
            AgentStatus::Delisted => Duration::from_secs(86400),
        }
    }

    pub fn next(
        &self,
        event: &HealthEvent,
        current_metrics: &AgentHealthMetrics,
    ) -> Option<AgentStatus> {
        match (self, event) {
            // New → Healthy: first successful probe set
            (AgentStatus::New, HealthEvent::ProbeSuccess { level, .. })
                if *level >= ProbeLevel::L2CardFetch => Some(AgentStatus::Healthy),

            // New → Dead: repeated failures
            (AgentStatus::New, HealthEvent::ProbeFailure { consecutive_failures, .. })
                if *consecutive_failures >= 3 => Some(AgentStatus::Dead),

            // Healthy → Degraded: probe failures hit threshold
            (AgentStatus::Healthy, HealthEvent::ProbeFailure { level, regions_failing, .. })
                if *regions_failing >= 2 => Some(AgentStatus::Degraded),

            // Healthy → Degraded: L4 quality failure
            (AgentStatus::Healthy, HealthEvent::QualityFail { .. }) =>
                Some(AgentStatus::Degraded),

            // Degraded → Healthy: sustained recovery
            (AgentStatus::Degraded, HealthEvent::SustainedRecovery { duration, .. })
                if *duration >= Duration::from_secs(900) => Some(AgentStatus::Healthy),

            // Degraded → Unhealthy: deeper failures
            (AgentStatus::Degraded, HealthEvent::ProbeFailure { level, regions_failing, .. })
                if *level >= ProbeLevel::L3EndpointCheck && *regions_failing >= 2 =>
                Some(AgentStatus::Unhealthy),

            // Unhealthy → Degraded: partial recovery
            (AgentStatus::Unhealthy, HealthEvent::ProbeSuccess { .. }) =>
                Some(AgentStatus::Degraded),

            // Unhealthy → Dead: extended failure
            (AgentStatus::Unhealthy, HealthEvent::TimeoutExpired { duration, .. })
                if *duration >= Duration::from_secs(72 * 3600) => Some(AgentStatus::Dead),

            // Dead → Unhealthy: resurrection
            (AgentStatus::Dead, HealthEvent::ProbeSuccess { .. }) =>
                Some(AgentStatus::Unhealthy),

            // Dead → Delisted: extended death
            (AgentStatus::Dead, HealthEvent::TimeoutExpired { duration, .. })
                if *duration >= Duration::from_secs(30 * 86400) => Some(AgentStatus::Delisted),

            // Any → Delisted: admin action
            (_, HealthEvent::AdminAction { action: AdminHealthAction::Delist { .. } }) =>
                Some(AgentStatus::Delisted),

            // Delisted → New: re-discovery
            (AgentStatus::Delisted, HealthEvent::Rediscovered { .. }) =>
                Some(AgentStatus::New),

            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HealthEvent {
    ProbeSuccess {
        level: ProbeLevel,
        region: String,
        latency: Duration,
    },
    ProbeFailure {
        level: ProbeLevel,
        region: String,
        consecutive_failures: u32,
        regions_failing: u32,
    },
    QualityFail {
        skill: String,
        score: f64,
        threshold: f64,
    },
    SustainedRecovery {
        duration: Duration,
        probes_passed: u32,
    },
    TimeoutExpired {
        duration: Duration,
    },
    AdminAction {
        action: AdminHealthAction,
    },
    Rediscovered {
        source: String,
    },
}

#[derive(Debug, Clone)]
pub enum AdminHealthAction {
    Delist { reason: String, admin_id: String },
    ForceHealthy { admin_id: String, justification: String },
    ScheduleReview { review_at: DateTime<Utc> },
}

#[derive(Debug, Clone)]
pub struct AgentHealthMetrics {
    pub total_probes_24h: u32,
    pub successful_probes_24h: u32,
    pub success_rate_24h: f64,
    pub avg_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub consecutive_failures: u32,
    pub regions_healthy: u32,
    pub regions_total: u32,
    pub last_status_change: DateTime<Utc>,
    pub time_in_current_status: Duration,
}
```

### 15.3 Capability Verification (L4)

L4 probes go beyond connectivity and card validation to test whether an agent can actually perform its claimed capabilities. This is the most expensive and most valuable probe level — it produces the strongest evidence for ranking.

#### Verification Pipeline

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    L4 CAPABILITY VERIFICATION PIPELINE                    │
│                                                                          │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐                │
│  │    Step 1    │   │    Step 2    │   │    Step 3    │                │
│  │   SELECT     │──►│  GENERATE    │──►│   SEND       │                │
│  │   SKILL      │   │  TEST        │   │   TEST       │                │
│  │              │   │  PROMPT      │   │   TASK       │                │
│  │ Pick skill   │   │ Template +   │   │ Via A2A      │                │
│  │ to test.     │   │ domain-      │   │ tasks/send   │                │
│  │ Rotate       │   │ specific     │   │ with test    │                │
│  │ across       │   │ generation.  │   │ flag.        │                │
│  │ skills each  │   │ Ensure non-  │   │ Timeout:     │                │
│  │ probe cycle. │   │ adversarial. │   │ 60s.         │                │
│  └──────────────┘   └──────────────┘   └──────────────┘                │
│                                                                          │
│  ┌──────────────┐   ┌──────────────┐                                    │
│  │    Step 4    │   │    Step 5    │                                    │
│  │  EVALUATE    │──►│   RECORD     │                                    │
│  │  RESPONSE    │   │   & SCORE    │                                    │
│  │              │   │              │                                    │
│  │ Format       │   │ Store in     │                                    │
│  │ correctness, │   │ benchmark_   │                                    │
│  │ content      │   │ runs table.  │                                    │
│  │ quality,     │   │ Update       │                                    │
│  │ safety       │   │ capability   │                                    │
│  │ check.       │   │ score in     │                                    │
│  │              │   │ agent        │                                    │
│  │              │   │ record.      │                                    │
│  └──────────────┘   └──────────────┘                                    │
└──────────────────────────────────────────────────────────────────────────┘
```

#### Test Prompt Generation

Test prompts are generated from skill-specific templates, designed to be non-adversarial, deterministic enough to evaluate, and varied enough to prevent gaming.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPromptRequest {
    pub skill_id: String,
    pub skill_name: String,
    pub skill_description: String,
    pub skill_tags: Vec<String>,
    pub skill_input_modes: Vec<String>,
    pub skill_output_modes: Vec<String>,
    pub difficulty: TestDifficulty,
    pub seed: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TestDifficulty {
    Basic,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTestPrompt {
    pub prompt_id: String,
    pub prompt_text: String,
    pub expected_output_characteristics: Vec<OutputCharacteristic>,
    pub evaluation_rubric: EvaluationRubric,
    pub max_response_time_ms: u64,
    pub safety_constraints: Vec<SafetyConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputCharacteristic {
    pub name: String,
    pub description: String,
    pub check_type: CheckType,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckType {
    ContainsKeywords { keywords: Vec<String>, min_matches: usize },
    FormatValidation { expected_format: String },
    LengthRange { min_chars: usize, max_chars: usize },
    JsonSchema { schema: serde_json::Value },
    RegexMatch { pattern: String },
    SemanticSimilarity { reference_text: String, min_score: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationRubric {
    pub dimensions: Vec<RubricDimension>,
    pub passing_threshold: f64,
    pub critical_dimensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricDimension {
    pub name: String,
    pub weight: f64,
    pub scoring: ScoringMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoringMethod {
    Binary,
    Scale { min: f64, max: f64 },
    Checklist { items: Vec<String> },
}
```

#### A2A Test Task

The L4 probe sends a real A2A task to the agent, using the standard `tasks/send` or `tasks/sendSubscribe` method. The task includes a metadata flag indicating it is a platform verification probe, allowing agents to handle it appropriately.

```json
{
  "jsonrpc": "2.0",
  "id": "probe-l4-a1b2c3d4-20260323-143022",
  "method": "tasks/send",
  "params": {
    "id": "probe-task-f47ac10b",
    "message": {
      "role": "user",
      "parts": [
        {
          "type": "text",
          "text": "Review the following Python function for potential bugs and suggest improvements:\n\ndef calculate_average(numbers):\n    total = 0\n    for n in numbers:\n        total += n\n    return total / len(numbers)"
        }
      ]
    },
    "metadata": {
      "agentrank_probe": true,
      "probe_type": "L4_capability_verification",
      "probe_id": "probe-l4-a1b2c3d4-20260323-143022",
      "expected_skill": "code-review",
      "timeout_ms": 60000,
      "do_not_bill": true,
      "privacy": "probe_response_may_be_stored_for_evaluation"
    }
  }
}
```

**Expected A2A response:**

```json
{
  "jsonrpc": "2.0",
  "id": "probe-l4-a1b2c3d4-20260323-143022",
  "result": {
    "id": "probe-task-f47ac10b",
    "status": {
      "state": "completed"
    },
    "artifacts": [
      {
        "parts": [
          {
            "type": "text",
            "text": "## Code Review: calculate_average\n\n### Bug Found\n**Division by Zero**: The function will raise a `ZeroDivisionError` when called with an empty list (`[]`), since `len(numbers)` will be 0.\n\n### Suggested Fix\n```python\ndef calculate_average(numbers):\n    if not numbers:\n        return 0.0  # or raise ValueError(\"Cannot average empty list\")\n    return sum(numbers) / len(numbers)\n```\n\n### Additional Improvements\n1. Use built-in `sum()` instead of manual loop\n2. Add type hints: `def calculate_average(numbers: list[float]) -> float`\n3. Consider using `statistics.mean()` from the standard library"
          }
        ]
      }
    ]
  }
}
```

#### Evaluation Scoring

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityScores {
    pub overall: f64,
    pub task_completion: f64,
    pub response_quality: f64,
    pub format_compliance: f64,
    pub response_time_score: f64,
    pub safety_score: f64,
    pub passing: bool,
    pub dimension_scores: Vec<DimensionScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionScore {
    pub dimension: String,
    pub score: f64,
    pub max_score: f64,
    pub weight: f64,
    pub notes: Vec<String>,
}

pub fn evaluate_l4_response(
    response: &A2ATaskResponse,
    prompt: &GeneratedTestPrompt,
    response_time_ms: f64,
) -> CapabilityScores {
    let mut dimension_scores = Vec::new();

    // Task completion: did the agent produce output?
    let task_completion = if response.status.state == "completed"
        && !response.artifacts.is_empty()
    {
        1.0
    } else if response.status.state == "completed" {
        0.5
    } else {
        0.0
    };
    dimension_scores.push(DimensionScore {
        dimension: "task_completion".into(),
        score: task_completion,
        max_score: 1.0,
        weight: 0.30,
        notes: vec![],
    });

    // Response quality: evaluate against rubric
    let response_text = extract_text_from_artifacts(&response.artifacts);
    let mut quality_score = 0.0;
    let mut quality_notes = Vec::new();

    for characteristic in &prompt.expected_output_characteristics {
        let char_score = evaluate_characteristic(&response_text, characteristic);
        quality_score += char_score * characteristic.weight;
        if char_score < 0.5 {
            quality_notes.push(format!(
                "Low score on '{}': {:.2}",
                characteristic.name, char_score
            ));
        }
    }
    dimension_scores.push(DimensionScore {
        dimension: "response_quality".into(),
        score: quality_score,
        max_score: 1.0,
        weight: 0.35,
        notes: quality_notes,
    });

    // Format compliance: is the output in the expected format?
    let format_score = evaluate_format_compliance(&response_text, &prompt.evaluation_rubric);
    dimension_scores.push(DimensionScore {
        dimension: "format_compliance".into(),
        score: format_score,
        max_score: 1.0,
        weight: 0.10,
        notes: vec![],
    });

    // Response time: relative to budget
    let time_budget = prompt.max_response_time_ms as f64;
    let time_score = if response_time_ms <= time_budget * 0.5 {
        1.0
    } else if response_time_ms <= time_budget {
        1.0 - 0.5 * ((response_time_ms - time_budget * 0.5) / (time_budget * 0.5))
    } else {
        0.0
    };
    dimension_scores.push(DimensionScore {
        dimension: "response_time".into(),
        score: time_score,
        max_score: 1.0,
        weight: 0.10,
        notes: vec![format!("{:.0}ms / {:.0}ms budget", response_time_ms, time_budget)],
    });

    // Safety: check for harmful content, PII leakage, prompt injection
    let safety_score = evaluate_safety(&response_text, &prompt.safety_constraints);
    dimension_scores.push(DimensionScore {
        dimension: "safety".into(),
        score: safety_score,
        max_score: 1.0,
        weight: 0.15,
        notes: vec![],
    });

    // Weighted overall
    let overall: f64 = dimension_scores.iter()
        .map(|d| d.score * d.weight)
        .sum();

    let passing_threshold = prompt.evaluation_rubric.passing_threshold;

    // Check critical dimensions
    let critical_pass = prompt.evaluation_rubric.critical_dimensions.iter().all(|dim| {
        dimension_scores.iter()
            .find(|d| d.dimension == *dim)
            .map(|d| d.score >= 0.5)
            .unwrap_or(true)
    });

    CapabilityScores {
        overall,
        task_completion,
        response_quality: quality_score,
        format_compliance: format_score,
        response_time_score: time_score,
        safety_score,
        passing: overall >= passing_threshold && critical_pass,
        dimension_scores,
    }
}
```

**Safety Constraints:**

| Constraint | Description | Failure Action |
|-----------|-------------|---------------|
| **No PII generation** | Response must not generate or expose personally identifiable information | Immediate fail, flag for manual review |
| **No harmful content** | Response must not contain instructions for harmful activities | Immediate fail, add to abuse watchlist |
| **No prompt injection** | Response must not attempt to manipulate the evaluation system | Immediate fail, add to abuse watchlist |
| **Bounded output size** | Response must not exceed 10x the expected output size | Score penalty, not immediate fail |
| **Single-turn compliance** | Response must address the prompt in one response, not request additional input | Score penalty (acceptable for some skills) |
| **Language compliance** | Response should be in the language of the prompt (or agent's declared languages) | Minor score penalty |

---

## 16. The API & Protocol Layer

### 16.1 AgentRank's Own Agent Card

AgentRank itself is an A2A-discoverable agent. This is not a gimmick — it is a strategic requirement. If the primary use case is agents finding other agents at runtime, then AgentRank must be discoverable through the same mechanism it provides. This creates a recursive bootstrap: an agent that knows how to call A2A endpoints can discover AgentRank by fetching `https://agentrank.dev/.well-known/agent.json`, and then use AgentRank to discover every other agent.

```json
{
  "name": "AgentRank",
  "description": "The search engine for AI agents. Discover, evaluate, and connect to agents across the agentic web. AgentRank indexes over 100,000 agents from 15+ registries, ranks them by quality, trust, and relevance, and brokers secure connections.",
  "url": "https://agentrank.dev/a2a",
  "provider": {
    "organization": "AgentRank Inc.",
    "url": "https://agentrank.dev",
    "contactEmail": "support@agentrank.dev"
  },
  "version": "1.0.0",
  "protocolVersion": "1.0",
  "capabilities": {
    "streaming": true,
    "pushNotifications": false,
    "stateTransitionHistory": true
  },
  "authentication": {
    "schemes": [
      {
        "scheme": "bearer",
        "description": "API key obtained from agentrank.dev/console. Free tier: 1000 req/day."
      },
      {
        "scheme": "oauth2",
        "flows": {
          "clientCredentials": {
            "tokenUrl": "https://auth.agentrank.dev/oauth/token",
            "scopes": {
              "search:read": "Search the agent index",
              "agent:read": "Read agent details",
              "connect:write": "Initiate brokered connections",
              "outcome:write": "Submit outcome reports"
            }
          }
        }
      }
    ]
  },
  "skills": [
    {
      "id": "search-agents",
      "name": "Search Agents",
      "description": "Search the AgentRank index to find agents by capability, domain, task description, or natural language query. Returns ranked results with trust scores, health status, and compatibility information. Supports filters by protocol, trust tier, health status, provider, and domain.",
      "tags": ["search", "discovery", "agents", "ranking"],
      "examples": [
        "Find agents that can review Python code for security vulnerabilities",
        "Search for translation agents that support Japanese to English with A2A v1.0",
        "List the top 5 agents for data analysis tasks, trust tier verified or above"
      ],
      "inputModes": ["text"],
      "outputModes": ["text", "data"]
    },
    {
      "id": "agent-details",
      "name": "Agent Details",
      "description": "Retrieve detailed information about a specific agent by ID or name. Returns the full agent card, AgentRank scores, health status, trust tier, skills, provider information, and connection instructions.",
      "tags": ["lookup", "details", "agent-info"],
      "examples": [
        "Get details about agent a1b2c3d4-e5f6-7890-abcd-ef1234567890",
        "What is the trust tier and health status of CodeReview Pro?",
        "Show me the skills and auth requirements for the Terraform Security Agent"
      ],
      "inputModes": ["text"],
      "outputModes": ["text", "data"]
    },
    {
      "id": "agent-recommend",
      "name": "Agent Recommendation",
      "description": "Given a task description and optional constraints, recommend the best agent(s) to handle the task. Considers relevance, trust, health, compatibility, and historical outcome data. Returns actionable recommendations with connection instructions.",
      "tags": ["recommend", "suggestion", "best-match"],
      "examples": [
        "I need an agent to summarize legal documents in French. What's the best option?",
        "Recommend agents for a multi-step data pipeline: extract from Salesforce, transform, load to Snowflake",
        "Which agent should I use for image classification? I need trust tier 'trusted' minimum and under $0.01 per request."
      ],
      "inputModes": ["text"],
      "outputModes": ["text", "data"]
    }
  ],
  "defaultInputModes": ["text"],
  "defaultOutputModes": ["text", "data"],
  "extensions": {
    "agentrank": {
      "self_indexed": true,
      "index_size": 104000,
      "registries_crawled": 15,
      "api_docs": "https://agentrank.dev/docs/api",
      "status_page": "https://status.agentrank.dev"
    }
  }
}
```

### 16.2 A2A-Native Interface

Any A2A-capable agent can search AgentRank by sending a standard `tasks/send` message. This means an agent does not need a custom SDK or REST client — it can use the same A2A client it uses to communicate with any other agent.

**JSON-RPC Request:**

```json
{
  "jsonrpc": "2.0",
  "id": "search-task-001",
  "method": "tasks/send",
  "params": {
    "id": "search-task-001",
    "message": {
      "role": "user",
      "parts": [
        {
          "type": "text",
          "text": "Find the top 5 agents that can perform automated code review for Python projects, with trust tier verified or above, currently healthy."
        }
      ]
    },
    "metadata": {
      "skill_hint": "search-agents",
      "response_format": "structured",
      "max_results": 5
    }
  }
}
```

**JSON-RPC Response:**

```json
{
  "jsonrpc": "2.0",
  "id": "search-task-001",
  "result": {
    "id": "search-task-001",
    "status": {
      "state": "completed"
    },
    "artifacts": [
      {
        "parts": [
          {
            "type": "text",
            "text": "Found 5 agents matching your criteria for automated Python code review:\n\n1. **CodeReview Pro** (score: 0.94) - Trusted tier, healthy, specializes in Python security and style review\n2. **PyLint Agent** (score: 0.89) - Verified tier, healthy, comprehensive Python linting with auto-fix suggestions\n3. **SecureCode Scanner** (score: 0.87) - Verified tier, healthy, security-focused code analysis\n4. **CodeQuality AI** (score: 0.85) - Verified tier, healthy, multi-language code review with Python specialization\n5. **BugHunter** (score: 0.82) - Verified tier, healthy, bug detection and test generation"
          },
          {
            "type": "data",
            "mimeType": "application/json",
            "data": {
              "results": [
                {
                  "agent_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
                  "name": "CodeReview Pro",
                  "provider": "DevTools Inc.",
                  "agentrank_score": 0.94,
                  "trust_tier": "trusted",
                  "health_status": "healthy",
                  "endpoint": "https://codereview-pro.devtools.io/a2a",
                  "protocol_version": "1.0.0",
                  "matched_skills": ["code-review-python", "security-analysis"],
                  "connect_url": "https://agentrank.dev/v1/connect",
                  "auth_schemes": ["bearer", "oauth2"]
                },
                {
                  "agent_id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
                  "name": "PyLint Agent",
                  "provider": "QualityCode Labs",
                  "agentrank_score": 0.89,
                  "trust_tier": "verified",
                  "health_status": "healthy",
                  "endpoint": "https://pylint-agent.qualitycode.dev/a2a",
                  "protocol_version": "1.0.0",
                  "matched_skills": ["python-linting", "code-style-review"],
                  "connect_url": "https://agentrank.dev/v1/connect",
                  "auth_schemes": ["api_key"]
                },
                {
                  "agent_id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                  "name": "SecureCode Scanner",
                  "provider": "SecurityFirst AI",
                  "agentrank_score": 0.87,
                  "trust_tier": "verified",
                  "health_status": "healthy",
                  "endpoint": "https://securecode.securityfirst.ai/a2a",
                  "protocol_version": "1.0.0",
                  "matched_skills": ["security-code-review", "vulnerability-detection"],
                  "connect_url": "https://agentrank.dev/v1/connect",
                  "auth_schemes": ["oauth2"]
                },
                {
                  "agent_id": "d4e5f6a7-b8c9-0123-defa-234567890123",
                  "name": "CodeQuality AI",
                  "provider": "PolyReview Inc.",
                  "agentrank_score": 0.85,
                  "trust_tier": "verified",
                  "health_status": "healthy",
                  "endpoint": "https://codequality.polyreview.com/a2a",
                  "protocol_version": "1.0.0",
                  "matched_skills": ["multi-language-review", "python-analysis"],
                  "connect_url": "https://agentrank.dev/v1/connect",
                  "auth_schemes": ["bearer"]
                },
                {
                  "agent_id": "e5f6a7b8-c9d0-1234-efab-345678901234",
                  "name": "BugHunter",
                  "provider": "TestGen Systems",
                  "agentrank_score": 0.82,
                  "trust_tier": "verified",
                  "health_status": "healthy",
                  "endpoint": "https://bughunter.testgen.io/a2a",
                  "protocol_version": "1.0.0",
                  "matched_skills": ["bug-detection", "test-generation"],
                  "connect_url": "https://agentrank.dev/v1/connect",
                  "auth_schemes": ["api_key", "oauth2"]
                }
              ],
              "total_matches": 47,
              "query_time_ms": 23.4
            }
          }
        ]
      }
    ]
  }
}
```

### 16.3 Search API (REST)

#### `POST /v1/search`

The REST Search API is the primary programmatic interface for agent search. It supports structured queries with filters, context, ranking preferences, and multiple retrieval modes.

**Complete Request Model:**

```json
{
  "query": "automated code review for Python projects with security focus",
  "filters": {
    "trust_tier_min": "verified",
    "health_status": ["healthy", "degraded"],
    "protocol_version": "1.0.*",
    "auth_schemes": ["bearer", "oauth2", "api_key"],
    "provider_domains": null,
    "exclude_providers": ["spam-agents.example.com"],
    "tags": {
      "include_any": ["code-review", "security", "python"],
      "include_all": ["python"],
      "exclude": ["deprecated"]
    },
    "skills": {
      "required": ["code-review"],
      "preferred": ["security-analysis", "auto-fix"]
    },
    "input_modes": ["text"],
    "output_modes": ["text", "data"],
    "languages": ["en"],
    "max_cost_per_request_usd": 0.50,
    "created_after": null,
    "created_before": null,
    "updated_after": "2026-01-01T00:00:00Z"
  },
  "context": {
    "caller_agent_id": "98765432-1abc-def0-1234-567890abcdef",
    "caller_domain": "ci-pipeline.acme.com",
    "caller_region": "us-east-1",
    "tenant_id": "tenant-acme-corp",
    "workflow_type": "ci-cd-pipeline",
    "task_description": "Need an agent to review Python pull requests in our CI pipeline, focusing on security vulnerabilities and code quality issues",
    "urgency": "normal",
    "previous_agents_tried": [
      {
        "agent_id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
        "outcome": "timed_out",
        "reason": "Agent did not respond within 30s SLA"
      }
    ]
  },
  "top_k": 10,
  "offset": 0,
  "mode": "hybrid",
  "ranking_preferences": {
    "weight_overrides": {
      "relevance": 0.35,
      "trust": 0.25,
      "operational_quality": 0.20,
      "outcome_success": 0.15,
      "freshness": 0.05
    },
    "boost_verified_providers": true,
    "penalize_previously_failed": true,
    "diversity_factor": 0.1
  },
  "explain": true,
  "include_facets": true,
  "include_suggestions": true
}
```

**Complete Response:**

```json
{
  "results": [
    {
      "agent_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "name": "CodeReview Pro",
      "description": "Enterprise-grade automated code review for Python, JavaScript, Go, and Rust. Specializes in security vulnerability detection, code style enforcement, and auto-fix suggestions. Trusted by 500+ engineering teams.",
      "provider": {
        "name": "DevTools Inc.",
        "domain": "devtools.io",
        "verified": true
      },
      "endpoint": "https://codereview-pro.devtools.io/a2a",
      "protocol_version": "1.0.0",
      "scores": {
        "agentrank_composite": 0.94,
        "relevance": 0.97,
        "trust": 0.92,
        "operational_quality": 0.95,
        "outcome_success": 0.91,
        "authority": 0.88,
        "freshness": 0.99,
        "documentation_quality": 0.90,
        "economic_efficiency": 0.85
      },
      "trust_tier": "trusted",
      "health_status": "healthy",
      "health_details": {
        "uptime_30d": 0.998,
        "avg_response_time_ms": 1200,
        "last_healthy_probe": "2026-03-23T14:00:00Z"
      },
      "matched_skills": [
        {
          "skill_id": "code-review-python",
          "skill_name": "Python Code Review",
          "relevance_score": 0.98,
          "match_type": "exact"
        },
        {
          "skill_id": "security-analysis",
          "skill_name": "Security Vulnerability Analysis",
          "relevance_score": 0.92,
          "match_type": "semantic"
        }
      ],
      "auth_schemes": ["bearer", "oauth2"],
      "pricing": {
        "model": "per_request",
        "estimated_cost_usd": 0.05,
        "currency": "USD"
      },
      "explanation": {
        "why_ranked_here": "Highest composite score. Strong exact match on 'code-review-python' skill. Trusted tier with 99.8% uptime. 91% outcome success rate across 12,000+ connections.",
        "signal_contributions": {
          "skill_match_exact": "+0.15",
          "skill_match_semantic": "+0.08",
          "trust_tier_bonus": "+0.05",
          "uptime_score": "+0.04",
          "outcome_rate": "+0.03",
          "provider_verified": "+0.02"
        },
        "penalties_applied": [],
        "diversity_note": null
      }
    }
  ],
  "total_matches": 47,
  "returned": 10,
  "offset": 0,
  "query_time_ms": 23.4,
  "index_timestamp": "2026-03-23T14:02:00Z",
  "retrieval_stats": {
    "lexical_candidates": 312,
    "semantic_candidates": 189,
    "graph_candidates": 45,
    "after_filter": 47,
    "after_dedup": 47,
    "fusion_method": "reciprocal_rank_fusion"
  },
  "facets": {
    "trust_tiers": {
      "authoritative": 2,
      "trusted": 8,
      "verified": 24,
      "established": 9,
      "indexed": 4
    },
    "health_status": {
      "healthy": 42,
      "degraded": 5
    },
    "auth_schemes": {
      "bearer": 31,
      "oauth2": 22,
      "api_key": 38,
      "mutual_tls": 3
    },
    "providers": {
      "DevTools Inc.": 3,
      "QualityCode Labs": 2,
      "SecurityFirst AI": 2,
      "Other": 40
    },
    "tags": {
      "code-review": 47,
      "python": 47,
      "security": 31,
      "linting": 18,
      "auto-fix": 12
    }
  },
  "suggestions": [
    {
      "type": "query_refinement",
      "text": "Try adding 'auto-fix' to find agents that can automatically fix issues",
      "refined_query": "automated code review for Python with security focus and auto-fix"
    },
    {
      "type": "filter_suggestion",
      "text": "8 trusted-tier agents match your query. Add trust_tier_min=trusted for highest confidence.",
      "filter": { "trust_tier_min": "trusted" }
    },
    {
      "type": "alternative_skill",
      "text": "Related skill: 'test-generation' — 12 agents can generate tests for the code they review",
      "skill": "test-generation"
    }
  ],
  "request_id": "req-20260323-search-abc123"
}
```

### 16.4 Resolve API

#### `GET /v1/agents/{agent_id}`

Returns the full canonical record for a single agent, including all scores, skills, health details, and connection instructions.

**Response:**

```json
{
  "agent": {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "CodeReview Pro",
    "description": "Enterprise-grade automated code review for Python, JavaScript, Go, and Rust. Specializes in security vulnerability detection, code style enforcement, and auto-fix suggestions.",
    "url": "https://codereview-pro.devtools.io/a2a",
    "card_url": "https://codereview-pro.devtools.io/.well-known/agent.json",
    "card_hash": "sha256:3a4b5c6d7e8f9012345678901234567890abcdef1234567890abcdef12345678",
    "protocol_version": "1.0.0",
    "provider": {
      "id": "prov-devtools-io",
      "name": "DevTools Inc.",
      "domain": "devtools.io",
      "verified": true,
      "verification_method": "dns_txt",
      "verified_at": "2026-02-15T10:00:00Z",
      "agent_count": 3,
      "trust_tier": "trusted"
    },
    "scores": {
      "agentrank_composite": 0.94,
      "search_ranking_score": 0.94,
      "discoverability_score": 0.91,
      "dimensions": {
        "relevance_base": 0.92,
        "trust": 0.92,
        "operational_quality": 0.95,
        "outcome_success": 0.91,
        "authority": 0.88,
        "freshness": 0.99,
        "documentation_quality": 0.90,
        "economic_efficiency": 0.85,
        "compatibility": 0.96
      },
      "computed_at": "2026-03-23T14:00:00Z",
      "version": 1247
    },
    "trust": {
      "tier": "trusted",
      "tier_since": "2026-03-01T00:00:00Z",
      "verification_methods": ["dns_txt", "card_signature", "provider_oauth"],
      "abuse_flags": [],
      "trust_signals": {
        "identity_verified": true,
        "domain_verified": true,
        "card_signed": true,
        "consistent_metadata": true,
        "no_abuse_history": true,
        "outcome_rate_above_threshold": true,
        "uptime_above_threshold": true
      }
    },
    "health": {
      "status": "healthy",
      "status_since": "2026-03-20T08:00:00Z",
      "uptime_7d": 0.999,
      "uptime_30d": 0.998,
      "avg_response_time_ms": 1200,
      "p99_response_time_ms": 3400,
      "last_probe": {
        "level": "L3",
        "status": "success",
        "timestamp": "2026-03-23T14:00:00Z",
        "latency_ms": 89
      },
      "last_l4_probe": {
        "status": "pass",
        "timestamp": "2026-03-23T02:00:00Z",
        "skill_tested": "code-review-python",
        "quality_score": 0.93
      },
      "probe_results_summary": {
        "l1_success_rate_24h": 1.0,
        "l2_success_rate_24h": 1.0,
        "l3_success_rate_24h": 0.98,
        "l4_pass_rate_30d": 0.96
      }
    },
    "skills": [
      {
        "id": "code-review-python",
        "name": "Python Code Review",
        "description": "Comprehensive Python code review: style, security, performance, and correctness. Supports Python 3.8+.",
        "tags": ["code-review", "python", "security", "linting"],
        "input_modes": ["text"],
        "output_modes": ["text", "data"],
        "examples": [
          "Review this Python function for bugs",
          "Check this PR for security vulnerabilities",
          "Analyze code style against PEP 8"
        ],
        "benchmark_scores": {
          "quality": 0.93,
          "latency_avg_ms": 1100,
          "tested_at": "2026-03-23T02:00:00Z"
        }
      },
      {
        "id": "security-analysis",
        "name": "Security Vulnerability Analysis",
        "description": "Deep security analysis: OWASP Top 10, dependency vulnerabilities, secret detection, injection risks.",
        "tags": ["security", "vulnerability", "owasp", "dependency-check"],
        "input_modes": ["text", "data"],
        "output_modes": ["text", "data"],
        "examples": [
          "Scan this codebase for OWASP Top 10 vulnerabilities",
          "Check for hardcoded secrets in these files",
          "Analyze dependencies for known CVEs"
        ],
        "benchmark_scores": {
          "quality": 0.90,
          "latency_avg_ms": 2300,
          "tested_at": "2026-03-22T02:00:00Z"
        }
      }
    ],
    "auth_schemes": [
      {
        "scheme": "bearer",
        "description": "API key from devtools.io dashboard"
      },
      {
        "scheme": "oauth2",
        "token_url": "https://auth.devtools.io/oauth/token",
        "scopes": ["code-review:read", "code-review:write"]
      }
    ],
    "connection": {
      "direct_connect_url": "https://codereview-pro.devtools.io/a2a",
      "brokered_connect_url": "https://agentrank.dev/v1/connect",
      "recommended_mode": "brokered",
      "recommendation_reason": "OAuth2 auth requires token exchange; brokered mode handles this automatically"
    },
    "metadata": {
      "first_discovered": "2026-01-20T15:30:00Z",
      "discovery_source": "well_known_crawl",
      "last_crawled": "2026-03-23T12:00:00Z",
      "last_card_change": "2026-03-15T09:00:00Z",
      "card_version": 14,
      "total_connections_30d": 12847,
      "total_outcomes_30d": 11203,
      "outcome_success_rate_30d": 0.91
    },
    "related_agents": [
      {
        "agent_id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
        "name": "PyLint Agent",
        "relationship": "similar_capability",
        "similarity_score": 0.87
      },
      {
        "agent_id": "f6a7b8c9-d0e1-2345-abcd-456789012345",
        "name": "DevTools Test Generator",
        "relationship": "same_provider",
        "similarity_score": 0.65
      }
    ]
  },
  "request_id": "req-20260323-resolve-def456"
}
```

### 16.5 Connect API

See [§14.5 Connect API](#145-connect-api) for the complete `POST /v1/connect` request and response specification.

### 16.6 Feedback API

See [§14.6 Outcome Feedback API](#146-outcome-feedback-api) for the complete `POST /v1/outcomes` request and response specification.

### 16.7 Lead Submission API

#### `POST /v1/leads`

The Lead Submission API allows external systems, registries, and users to submit discovery leads — hints about agents that may exist at a given URL or domain. Leads are ingested into the crawl frontier for verification and indexing.

**Request:**

```json
{
  "leads": [
    {
      "url": "https://new-agent.example.com/.well-known/agent.json",
      "source": "user_submission",
      "source_detail": "Submitted via Agent Search Console",
      "expected_capabilities": ["data-analysis", "visualization"],
      "priority_hint": "normal",
      "metadata": {
        "submitter_id": "user-12345",
        "submitter_domain": "acme.com",
        "notes": "New data analysis agent from our partner team"
      }
    },
    {
      "url": "https://agents.startup.io",
      "source": "registry_feed",
      "source_detail": "AgentVerse bulk export 2026-03-23",
      "expected_capabilities": null,
      "priority_hint": "low",
      "metadata": {
        "registry_id": "agentverse",
        "registry_listing_id": "av-98765"
      }
    }
  ],
  "submitter": {
    "type": "api_key",
    "identity": "acme-corp-api-key-xyz"
  }
}
```

**Response:**

```json
{
  "accepted": 2,
  "rejected": 0,
  "leads": [
    {
      "lead_id": "lead-a1b2c3d4",
      "url": "https://new-agent.example.com/.well-known/agent.json",
      "status": "queued",
      "estimated_crawl_time": "2026-03-23T15:00:00Z",
      "priority": "normal",
      "dedup_status": "new"
    },
    {
      "lead_id": "lead-e5f6a7b8",
      "url": "https://agents.startup.io",
      "status": "queued",
      "estimated_crawl_time": "2026-03-24T02:00:00Z",
      "priority": "low",
      "dedup_status": "new"
    }
  ],
  "request_id": "req-20260323-leads-ghi789"
}
```

### 16.8 GraphQL API

For clients that need flexible queries (dashboards, consoles, integrations), AgentRank provides a GraphQL API alongside the REST endpoints.

**Complete Schema:**

```graphql
type Query {
  searchAgents(input: SearchInput!): SearchResults!
  agent(id: ID!): Agent
  agentByName(name: String!, provider: String): Agent
  agents(ids: [ID!]!): [Agent]!
  provider(domain: String!): Provider
  agentSkills(agentId: ID!): [Skill!]!
  agentHistory(agentId: ID!, from: DateTime, to: DateTime, limit: Int): [AgentHistoryEntry!]!
  rankHistory(agentId: ID!, from: DateTime, to: DateTime, granularity: Granularity): [RankDataPoint!]!
  healthTimeline(agentId: ID!, from: DateTime, to: DateTime): [HealthEvent!]!
  relatedAgents(agentId: ID!, relationship: RelationshipType, limit: Int): [RelatedAgent!]!
  topAgents(category: String, limit: Int, trustTierMin: TrustTier): [Agent!]!
  platformStats: PlatformStats!
}

type Mutation {
  submitLead(input: LeadInput!): LeadResult!
  submitOutcome(input: OutcomeInput!): OutcomeResult!
  claimAgent(agentId: ID!, verification: VerificationInput!): ClaimResult!
  updateAgentMetadata(agentId: ID!, metadata: AgentMetadataInput!): Agent!
}

type Subscription {
  agentStatusChanged(agentIds: [ID!]): AgentStatusEvent!
  newAgentDiscovered(filters: DiscoveryFilter): AgentDiscoveredEvent!
  rankChanged(agentIds: [ID!], minDelta: Float): RankChangedEvent!
}

type Agent {
  id: ID!
  name: String!
  description: String
  url: String!
  cardUrl: String!
  cardHash: String!
  protocolVersion: String!
  provider: Provider!
  scores: AgentScores!
  trustTier: TrustTier!
  healthStatus: HealthStatus!
  healthDetails: HealthDetails!
  skills: [Skill!]!
  authSchemes: [AuthScheme!]!
  tags: [String!]!
  languages: [String!]!
  inputModes: [String!]!
  outputModes: [String!]!
  metadata: AgentMetadata!
  connection: ConnectionInfo!
  relatedAgents(relationship: RelationshipType, limit: Int): [RelatedAgent!]!
  rankHistory(from: DateTime, to: DateTime, granularity: Granularity): [RankDataPoint!]!
  createdAt: DateTime!
  updatedAt: DateTime!
}

type AgentScores {
  composite: Float!
  searchRanking: Float!
  discoverability: Float!
  relevanceBase: Float!
  trust: Float!
  operationalQuality: Float!
  outcomeSuccess: Float!
  authority: Float!
  freshness: Float!
  documentationQuality: Float!
  economicEfficiency: Float!
  compatibility: Float!
  computedAt: DateTime!
  version: Int!
}

type Skill {
  id: ID!
  name: String!
  description: String
  tags: [String!]!
  inputModes: [String!]!
  outputModes: [String!]!
  examples: [String!]!
  benchmarkScore: Float
  benchmarkTestedAt: DateTime
}

type Provider {
  id: ID!
  name: String!
  domain: String!
  verified: Boolean!
  verificationMethod: String
  verifiedAt: DateTime
  trustTier: TrustTier!
  agentCount: Int!
  agents(limit: Int, offset: Int): [Agent!]!
  avgAgentScore: Float!
}

type SearchResults {
  results: [SearchResult!]!
  totalMatches: Int!
  returned: Int!
  offset: Int!
  queryTimeMs: Float!
  facets: SearchFacets
  suggestions: [SearchSuggestion!]
}

type SearchResult {
  agent: Agent!
  score: Float!
  matchedSkills: [SkillMatch!]!
  explanation: SearchExplanation
}

type SkillMatch {
  skillId: ID!
  skillName: String!
  relevanceScore: Float!
  matchType: MatchType!
}

type SearchExplanation {
  whyRankedHere: String!
  signalContributions: [SignalContribution!]!
  penaltiesApplied: [Penalty!]!
}

type SignalContribution {
  signal: String!
  value: String!
}

type Penalty {
  reason: String!
  impact: String!
}

type SearchFacets {
  trustTiers: [FacetBucket!]!
  healthStatus: [FacetBucket!]!
  authSchemes: [FacetBucket!]!
  providers: [FacetBucket!]!
  tags: [FacetBucket!]!
}

type FacetBucket {
  key: String!
  count: Int!
}

type SearchSuggestion {
  type: SuggestionType!
  text: String!
  refinedQuery: String
  filter: JSON
}

type HealthDetails {
  status: HealthStatus!
  statusSince: DateTime!
  uptime7d: Float!
  uptime30d: Float!
  avgResponseTimeMs: Float!
  p99ResponseTimeMs: Float!
  lastProbe: ProbeInfo!
  lastL4Probe: L4ProbeInfo
}

type ProbeInfo {
  level: ProbeLevel!
  status: ProbeOutcome!
  timestamp: DateTime!
  latencyMs: Float!
}

type L4ProbeInfo {
  status: ProbeOutcome!
  timestamp: DateTime!
  skillTested: String!
  qualityScore: Float!
}

type ConnectionInfo {
  directConnectUrl: String!
  brokeredConnectUrl: String!
  recommendedMode: ConnectionMode!
  recommendationReason: String
}

type AgentMetadata {
  firstDiscovered: DateTime!
  discoverySource: String!
  lastCrawled: DateTime!
  lastCardChange: DateTime!
  cardVersion: Int!
  totalConnections30d: Int!
  totalOutcomes30d: Int!
  outcomeSuccessRate30d: Float!
}

type RelatedAgent {
  agent: Agent!
  relationship: RelationshipType!
  similarityScore: Float!
}

type RankDataPoint {
  timestamp: DateTime!
  composite: Float!
  trust: Float!
  operationalQuality: Float!
  outcomeSuccess: Float!
}

type PlatformStats {
  totalAgentsIndexed: Int!
  totalHealthyAgents: Int!
  totalProviders: Int!
  totalConnections24h: Int!
  totalOutcomes24h: Int!
  avgQueryTimeMs: Float!
  indexFreshness: DateTime!
}

input SearchInput {
  query: String!
  filters: SearchFilters
  topK: Int
  offset: Int
  mode: SearchMode
  rankingPreferences: RankingPreferences
  explain: Boolean
  includeFacets: Boolean
  includeSuggestions: Boolean
}

input SearchFilters {
  trustTierMin: TrustTier
  healthStatus: [HealthStatus!]
  protocolVersion: String
  authSchemes: [String!]
  providerDomains: [String!]
  excludeProviders: [String!]
  tagsIncludeAny: [String!]
  tagsIncludeAll: [String!]
  tagsExclude: [String!]
  skillsRequired: [String!]
  skillsPreferred: [String!]
  inputModes: [String!]
  outputModes: [String!]
  languages: [String!]
  maxCostPerRequestUsd: Float
  updatedAfter: DateTime
}

input RankingPreferences {
  weightOverrides: WeightOverrides
  boostVerifiedProviders: Boolean
  penalizePreviouslyFailed: Boolean
  diversityFactor: Float
}

input WeightOverrides {
  relevance: Float
  trust: Float
  operationalQuality: Float
  outcomeSuccess: Float
  freshness: Float
}

enum TrustTier {
  INDEXED
  ESTABLISHED
  VERIFIED
  TRUSTED
  AUTHORITATIVE
}

enum HealthStatus {
  HEALTHY
  DEGRADED
  UNHEALTHY
  DEAD
  UNKNOWN
}

enum ProbeLevel {
  L1_PING
  L2_CARD_FETCH
  L3_ENDPOINT_CHECK
  L4_CAPABILITY_PROBE
}

enum ProbeOutcome {
  SUCCESS
  FAILURE
  TIMEOUT
  ERROR
}

enum SearchMode {
  LEXICAL
  SEMANTIC
  HYBRID
  GRAPH
}

enum ConnectionMode {
  DIRECT
  BROKERED
}

enum RelationshipType {
  SIMILAR_CAPABILITY
  SAME_PROVIDER
  FREQUENTLY_COMPOSED
  COMPLEMENTARY
  ALTERNATIVE
}

enum MatchType {
  EXACT
  SEMANTIC
  TAG
  GRAPH
}

enum SuggestionType {
  QUERY_REFINEMENT
  FILTER_SUGGESTION
  ALTERNATIVE_SKILL
  RELATED_CATEGORY
}

enum Granularity {
  HOURLY
  DAILY
  WEEKLY
  MONTHLY
}

scalar DateTime
scalar JSON
```

### 16.9 WebSocket Feed

For real-time monitoring, dashboards, and agent orchestrators that need to react to changes in the agent ecosystem, AgentRank provides a WebSocket event feed.

**Subscribe:**

```json
{
  "action": "subscribe",
  "channels": [
    {
      "type": "agent_events",
      "agent_ids": ["a1b2c3d4-e5f6-7890-abcd-ef1234567890"],
      "events": ["rank_changed", "status_changed", "card_updated"]
    },
    {
      "type": "discovery_feed",
      "filters": {
        "tags": ["code-review"],
        "trust_tier_min": "verified"
      },
      "events": ["agent_discovered", "agent_revived"]
    },
    {
      "type": "global_feed",
      "events": ["agent_died"]
    }
  ],
  "auth_token": "agr_ws_v1.eyJhbGciOiJSUzI1NiIs..."
}
```

**Event: `agent_discovered`**

```json
{
  "event": "agent_discovered",
  "timestamp": "2026-03-23T14:15:30.123Z",
  "data": {
    "agent_id": "new-agent-uuid",
    "name": "New Code Analyzer",
    "provider_domain": "codeanalyzer.dev",
    "discovery_source": "well_known_crawl",
    "skills": ["code-analysis", "python-review"],
    "protocol_version": "1.0.0",
    "initial_trust_tier": "indexed",
    "health_status": "new",
    "card_url": "https://codeanalyzer.dev/.well-known/agent.json"
  }
}
```

**Event: `agent_rank_changed`**

```json
{
  "event": "agent_rank_changed",
  "timestamp": "2026-03-23T14:20:00.456Z",
  "data": {
    "agent_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "CodeReview Pro",
    "previous_score": 0.93,
    "new_score": 0.94,
    "delta": 0.01,
    "reason": "outcome_data_update",
    "changed_dimensions": {
      "outcome_success": {
        "previous": 0.90,
        "new": 0.91,
        "contributing_factor": "247 new positive outcomes in last 24h"
      }
    },
    "rank_position": {
      "category": "code-review",
      "previous_position": 2,
      "new_position": 1
    }
  }
}
```

**Event: `agent_died`**

```json
{
  "event": "agent_died",
  "timestamp": "2026-03-23T14:25:00.789Z",
  "data": {
    "agent_id": "dead-agent-uuid",
    "name": "OldLinter v1",
    "provider_domain": "deprecated-tools.com",
    "previous_status": "unhealthy",
    "new_status": "dead",
    "unhealthy_since": "2026-03-20T08:00:00Z",
    "last_successful_probe": "2026-03-19T23:45:00Z",
    "last_probe_error": "Connection refused on all regions",
    "affected_skills": ["python-linting", "style-check"],
    "alternative_agents": [
      {
        "agent_id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
        "name": "PyLint Agent",
        "score": 0.89,
        "similarity": 0.92
      }
    ]
  }
}
```

**Event: `agent_revived`**

```json
{
  "event": "agent_revived",
  "timestamp": "2026-03-23T14:30:00.012Z",
  "data": {
    "agent_id": "revived-agent-uuid",
    "name": "DataPipeline Agent",
    "provider_domain": "datapipeline.ai",
    "previous_status": "dead",
    "new_status": "unhealthy",
    "dead_since": "2026-03-10T00:00:00Z",
    "revived_by_probe": {
      "level": "L1",
      "region": "us-east-1",
      "latency_ms": 234
    },
    "note": "Agent detected alive after 13 days. Entering re-evaluation pipeline. Will probe at elevated frequency for 24h before considering for search results."
  }
}
```

---

## 17. Data Model & Schema Design

### 17.1 Full PostgreSQL Schema

The PostgreSQL schema is the canonical store for all agent metadata, ranking state, crawl history, and trust data. It is designed for:
- Strong consistency (agent identity, trust tiers, policy rules)
- Complex joins (multi-table queries for the Agent Search Console)
- Audit trail (versioned history with JSONB diffs)
- Scale (partitioned tables for time-series data, optimized indexes)

```sql
-- ============================================================================
-- EXTENSIONS
-- ============================================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "btree_gin";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================================
-- ENUMS
-- ============================================================================

CREATE TYPE agent_status AS ENUM (
    'new', 'healthy', 'degraded', 'unhealthy', 'dead', 'delisted'
);

CREATE TYPE trust_tier AS ENUM (
    'indexed', 'established', 'verified', 'trusted', 'authoritative'
);

CREATE TYPE visibility_scope AS ENUM (
    'public', 'tenant_private', 'partner_restricted', 'internal'
);

CREATE TYPE discovery_source AS ENUM (
    'well_known_crawl', 'registry_feed', 'dns_txt', 'certificate_transparency',
    'code_repository', 'interaction_graph', 'user_submission', 'api_registration',
    'partner_feed', 'sitemap_crawl', 'web_mention', 'manual_import'
);

CREATE TYPE verification_method AS ENUM (
    'dns_txt', 'meta_tag', 'file_upload', 'card_signature', 'domain_whois',
    'provider_oauth', 'manual_review', 'certificate_pin'
);

CREATE TYPE auth_scheme_type AS ENUM (
    'none', 'api_key', 'bearer', 'oauth2', 'mutual_tls', 'custom'
);

CREATE TYPE connection_mode AS ENUM (
    'direct', 'handshake_only', 'proxy_first', 'full_proxy', 'managed_session'
);

CREATE TYPE outcome_value AS ENUM (
    'connected', 'completed', 'partial', 'timed_out', 'rejected',
    'auth_failed', 'error', 'incompatible', 'cancelled'
);

CREATE TYPE edge_type AS ENUM (
    'similar_capability', 'same_provider', 'frequently_composed',
    'complementary', 'alternative', 'dependency', 'successor', 'fork'
);

CREATE TYPE probe_level AS ENUM ('l1_ping', 'l2_card_fetch', 'l3_endpoint_check', 'l4_capability_probe');

CREATE TYPE probe_status AS ENUM ('success', 'timeout', 'connection_refused', 'tls_error', 'http_error', 'parse_error', 'quality_fail', 'error');

-- ============================================================================
-- CORE TABLES
-- ============================================================================

-- Primary agent table: one row per canonical agent
CREATE TABLE agents (
    -- Identity
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    canonical_url           TEXT NOT NULL,
    card_url                TEXT NOT NULL,
    endpoint_url            TEXT NOT NULL,
    name                    TEXT NOT NULL,
    description             TEXT,
    short_description       TEXT,

    -- Provider link
    provider_id             UUID REFERENCES providers(id),
    provider_domain         TEXT NOT NULL,

    -- Protocol
    protocol_version        TEXT NOT NULL DEFAULT '1.0',
    agent_version           TEXT,
    card_hash               TEXT NOT NULL,
    card_raw                JSONB,

    -- Classification
    tags                    TEXT[] NOT NULL DEFAULT '{}',
    categories              TEXT[] NOT NULL DEFAULT '{}',
    languages               TEXT[] NOT NULL DEFAULT '{}',
    input_modes             TEXT[] NOT NULL DEFAULT '{}',
    output_modes            TEXT[] NOT NULL DEFAULT '{}',
    auth_schemes            auth_scheme_type[] NOT NULL DEFAULT '{}',

    -- Ranking scores (denormalized for fast reads)
    agentrank_composite     REAL NOT NULL DEFAULT 0.0,
    score_relevance_base    REAL NOT NULL DEFAULT 0.0,
    score_trust             REAL NOT NULL DEFAULT 0.0,
    score_operational       REAL NOT NULL DEFAULT 0.0,
    score_outcome           REAL NOT NULL DEFAULT 0.0,
    score_authority         REAL NOT NULL DEFAULT 0.0,
    score_freshness         REAL NOT NULL DEFAULT 0.0,
    score_documentation     REAL NOT NULL DEFAULT 0.0,
    score_economic          REAL NOT NULL DEFAULT 0.0,
    score_compatibility     REAL NOT NULL DEFAULT 0.0,
    discoverability_score   REAL NOT NULL DEFAULT 0.0,
    rank_version            BIGINT NOT NULL DEFAULT 0,
    rank_computed_at        TIMESTAMPTZ,

    -- Trust
    trust_tier              trust_tier NOT NULL DEFAULT 'indexed',
    trust_tier_since        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    verification_methods    verification_method[] NOT NULL DEFAULT '{}',
    verified_at             TIMESTAMPTZ,
    abuse_flags             TEXT[] NOT NULL DEFAULT '{}',
    spam_score              REAL NOT NULL DEFAULT 0.0,

    -- Liveness
    health_status           agent_status NOT NULL DEFAULT 'new',
    health_status_since     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_healthy_at         TIMESTAMPTZ,
    uptime_7d               REAL,
    uptime_30d              REAL,
    avg_response_time_ms    REAL,
    p99_response_time_ms    REAL,
    consecutive_probe_failures INTEGER NOT NULL DEFAULT 0,

    -- Crawl metadata
    discovery_source        discovery_source NOT NULL,
    first_discovered_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_crawled_at         TIMESTAMPTZ,
    last_card_change_at     TIMESTAMPTZ,
    card_version_count      INTEGER NOT NULL DEFAULT 1,
    crawl_priority          REAL NOT NULL DEFAULT 0.5,
    crawl_interval_secs     INTEGER NOT NULL DEFAULT 3600,
    next_crawl_at           TIMESTAMPTZ,
    crawl_failures          INTEGER NOT NULL DEFAULT 0,
    last_crawl_error        TEXT,

    -- Outcome aggregates (denormalized)
    total_connections_7d    INTEGER NOT NULL DEFAULT 0,
    total_connections_30d   INTEGER NOT NULL DEFAULT 0,
    total_outcomes_30d      INTEGER NOT NULL DEFAULT 0,
    outcome_success_rate_30d REAL,
    avg_outcome_quality_30d REAL,

    -- Visibility and access
    visibility              visibility_scope NOT NULL DEFAULT 'public',
    tenant_id               UUID,
    delisted                BOOLEAN NOT NULL DEFAULT FALSE,
    delist_reason           TEXT,
    delisted_at             TIMESTAMPTZ,

    -- Pricing
    pricing_model           TEXT,
    estimated_cost_usd      REAL,
    pricing_details         JSONB,

    -- Metadata
    extra_metadata          JSONB NOT NULL DEFAULT '{}',
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT unique_canonical_url UNIQUE (canonical_url),
    CONSTRAINT valid_scores CHECK (
        agentrank_composite >= 0 AND agentrank_composite <= 1
        AND score_trust >= 0 AND score_trust <= 1
        AND score_operational >= 0 AND score_operational <= 1
    )
);

-- Skills associated with each agent
CREATE TABLE agent_skills (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id            UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    skill_id            TEXT NOT NULL,
    name                TEXT NOT NULL,
    description         TEXT,
    tags                TEXT[] NOT NULL DEFAULT '{}',
    input_modes         TEXT[] NOT NULL DEFAULT '{}',
    output_modes        TEXT[] NOT NULL DEFAULT '{}',
    examples            TEXT[] NOT NULL DEFAULT '{}',
    embedding           vector(768),
    benchmark_quality   REAL,
    benchmark_latency_ms REAL,
    benchmark_tested_at TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_agent_skill UNIQUE (agent_id, skill_id)
);

-- Multi-type embeddings for semantic retrieval
CREATE TABLE agent_embeddings (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id        UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    embedding_type  TEXT NOT NULL,   -- 'capability', 'domain', 'interaction'
    model_id        TEXT NOT NULL,   -- e.g. 'bge-base-en-v1.5'
    embedding       vector(768) NOT NULL,
    source_text     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_agent_embedding_type UNIQUE (agent_id, embedding_type)
);

-- Agent graph: relationships between agents
CREATE TABLE agent_edges (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_agent_id UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    target_agent_id UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    edge_type       edge_type NOT NULL,
    weight          REAL NOT NULL DEFAULT 1.0,
    evidence        JSONB NOT NULL DEFAULT '{}',
    first_observed  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_observed   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    observation_count INTEGER NOT NULL DEFAULT 1,
    confidence      REAL NOT NULL DEFAULT 0.5,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_edge UNIQUE (source_agent_id, target_agent_id, edge_type),
    CONSTRAINT no_self_edge CHECK (source_agent_id != target_agent_id),
    CONSTRAINT valid_weight CHECK (weight >= 0 AND weight <= 10),
    CONSTRAINT valid_confidence CHECK (confidence >= 0 AND confidence <= 1)
);

-- Crawl log: full audit trail of every crawl attempt (partitioned)
CREATE TABLE crawl_log (
    id              UUID NOT NULL DEFAULT uuid_generate_v4(),
    agent_id        UUID,
    url             TEXT NOT NULL,
    fetched_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    http_status     SMALLINT,
    response_time_ms REAL,
    response_size_bytes BIGINT,
    content_hash    TEXT,
    card_valid      BOOLEAN,
    card_changed    BOOLEAN,
    schema_errors   TEXT[],
    error_message   TEXT,
    probe_region    TEXT,
    discovery_source discovery_source,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
) PARTITION BY RANGE (fetched_at);

CREATE TABLE crawl_log_2026_q1 PARTITION OF crawl_log
    FOR VALUES FROM ('2026-01-01') TO ('2026-04-01');
CREATE TABLE crawl_log_2026_q2 PARTITION OF crawl_log
    FOR VALUES FROM ('2026-04-01') TO ('2026-07-01');
CREATE TABLE crawl_log_2026_q3 PARTITION OF crawl_log
    FOR VALUES FROM ('2026-07-01') TO ('2026-10-01');
CREATE TABLE crawl_log_2026_q4 PARTITION OF crawl_log
    FOR VALUES FROM ('2026-10-01') TO ('2027-01-01');

-- Rank history: snapshots of ranking scores over time (partitioned)
CREATE TABLE rank_history (
    id                  UUID NOT NULL DEFAULT uuid_generate_v4(),
    agent_id            UUID NOT NULL,
    computed_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    agentrank_composite REAL NOT NULL,
    score_relevance_base REAL,
    score_trust         REAL,
    score_operational   REAL,
    score_outcome       REAL,
    score_authority     REAL,
    score_freshness     REAL,
    score_documentation REAL,
    score_economic      REAL,
    score_compatibility REAL,
    discoverability_score REAL,
    rank_version        BIGINT NOT NULL,
    trigger_event       TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
) PARTITION BY RANGE (computed_at);

CREATE TABLE rank_history_2026_q1 PARTITION OF rank_history
    FOR VALUES FROM ('2026-01-01') TO ('2026-04-01');
CREATE TABLE rank_history_2026_q2 PARTITION OF rank_history
    FOR VALUES FROM ('2026-04-01') TO ('2026-07-01');
CREATE TABLE rank_history_2026_q3 PARTITION OF rank_history
    FOR VALUES FROM ('2026-07-01') TO ('2026-10-01');
CREATE TABLE rank_history_2026_q4 PARTITION OF rank_history
    FOR VALUES FROM ('2026-10-01') TO ('2027-01-01');

-- Card history: versioned snapshots of agent cards with diffs
CREATE TABLE card_history (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id        UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    version         INTEGER NOT NULL,
    card_hash       TEXT NOT NULL,
    card_raw        JSONB NOT NULL,
    diff_from_prev  JSONB,
    fetched_at      TIMESTAMPTZ NOT NULL,
    source_url      TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_card_version UNIQUE (agent_id, version)
);

-- Providers: organizations that operate agents
CREATE TABLE providers (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                TEXT NOT NULL,
    domain              TEXT NOT NULL,
    description         TEXT,
    contact_email       TEXT,
    website_url         TEXT,
    verified            BOOLEAN NOT NULL DEFAULT FALSE,
    verification_method verification_method,
    verified_at         TIMESTAMPTZ,
    verification_proof  JSONB,
    trust_tier          trust_tier NOT NULL DEFAULT 'indexed',
    agent_count         INTEGER NOT NULL DEFAULT 0,
    avg_agent_score     REAL NOT NULL DEFAULT 0.0,
    total_outcomes_30d  INTEGER NOT NULL DEFAULT 0,
    outcome_success_rate_30d REAL,
    claimed_by_user_id  UUID,
    claimed_at          TIMESTAMPTZ,
    extra_metadata      JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_provider_domain UNIQUE (domain)
);

-- Connection outcomes: individual outcome reports
CREATE TABLE connection_outcomes (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id          UUID NOT NULL,
    caller_agent_id     UUID,
    target_agent_id     UUID NOT NULL REFERENCES agents(id),
    outcome_value       outcome_value NOT NULL,
    quality_score       REAL,
    quality_dimensions  JSONB,
    task_summary        TEXT,
    duration_ms         INTEGER,
    tokens_input        INTEGER,
    tokens_output       INTEGER,
    cost_usd            REAL,
    reporter_type       TEXT NOT NULL,
    reporter_agent_id   UUID,
    connection_mode     connection_mode,
    auth_strategy       TEXT,
    signature_alg       TEXT,
    signature_public_key TEXT,
    signature_value     TEXT,
    signature_verified  BOOLEAN,
    metadata            JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_quality_score CHECK (quality_score IS NULL OR (quality_score >= 0 AND quality_score <= 1))
);

-- Benchmark runs: L4 capability probe results
CREATE TABLE benchmark_runs (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_id            UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    skill_id            TEXT NOT NULL,
    probe_id            UUID,
    prompt_hash         TEXT NOT NULL,
    difficulty          TEXT NOT NULL DEFAULT 'basic',
    overall_score       REAL NOT NULL,
    task_completion     REAL NOT NULL,
    response_quality    REAL NOT NULL,
    format_compliance   REAL NOT NULL,
    response_time_ms    REAL NOT NULL,
    safety_score        REAL NOT NULL,
    passing             BOOLEAN NOT NULL,
    dimension_scores    JSONB NOT NULL DEFAULT '{}',
    probe_region        TEXT,
    model_version       TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- agents: primary query patterns
CREATE INDEX idx_agents_provider ON agents(provider_id);
CREATE INDEX idx_agents_provider_domain ON agents(provider_domain);
CREATE INDEX idx_agents_health_status ON agents(health_status) WHERE NOT delisted;
CREATE INDEX idx_agents_trust_tier ON agents(trust_tier) WHERE NOT delisted;
CREATE INDEX idx_agents_composite_score ON agents(agentrank_composite DESC) WHERE NOT delisted AND health_status IN ('healthy', 'degraded');
CREATE INDEX idx_agents_visibility ON agents(visibility, tenant_id);
CREATE INDEX idx_agents_next_crawl ON agents(next_crawl_at ASC) WHERE NOT delisted;
CREATE INDEX idx_agents_discovery_source ON agents(discovery_source);
CREATE INDEX idx_agents_protocol_version ON agents(protocol_version);

-- agents: GIN indexes for array columns
CREATE INDEX idx_agents_tags_gin ON agents USING GIN (tags);
CREATE INDEX idx_agents_categories_gin ON agents USING GIN (categories);
CREATE INDEX idx_agents_languages_gin ON agents USING GIN (languages);
CREATE INDEX idx_agents_auth_schemes_gin ON agents USING GIN (auth_schemes);

-- agents: trigram indexes for fuzzy text search
CREATE INDEX idx_agents_name_trgm ON agents USING GIN (name gin_trgm_ops);
CREATE INDEX idx_agents_description_trgm ON agents USING GIN (description gin_trgm_ops);

-- agents: partial indexes for common query patterns
CREATE INDEX idx_agents_healthy_searchable ON agents(agentrank_composite DESC)
    WHERE NOT delisted AND health_status IN ('healthy', 'degraded', 'new') AND visibility = 'public';
CREATE INDEX idx_agents_verified_plus ON agents(agentrank_composite DESC)
    WHERE trust_tier IN ('verified', 'trusted', 'authoritative') AND NOT delisted;

-- agent_skills
CREATE INDEX idx_skills_agent ON agent_skills(agent_id);
CREATE INDEX idx_skills_tags_gin ON agent_skills USING GIN (tags);
CREATE INDEX idx_skills_name_trgm ON agent_skills USING GIN (name gin_trgm_ops);

-- agent_embeddings
CREATE INDEX idx_embeddings_agent ON agent_embeddings(agent_id);
CREATE INDEX idx_embeddings_type ON agent_embeddings(embedding_type);

-- agent_edges
CREATE INDEX idx_edges_source ON agent_edges(source_agent_id);
CREATE INDEX idx_edges_target ON agent_edges(target_agent_id);
CREATE INDEX idx_edges_type ON agent_edges(edge_type);
CREATE INDEX idx_edges_weight ON agent_edges(weight DESC);

-- crawl_log
CREATE INDEX idx_crawl_log_agent ON crawl_log(agent_id, fetched_at DESC);
CREATE INDEX idx_crawl_log_url ON crawl_log(url, fetched_at DESC);

-- rank_history
CREATE INDEX idx_rank_history_agent ON rank_history(agent_id, computed_at DESC);

-- card_history
CREATE INDEX idx_card_history_agent ON card_history(agent_id, version DESC);

-- connection_outcomes
CREATE INDEX idx_outcomes_target ON connection_outcomes(target_agent_id, created_at DESC);
CREATE INDEX idx_outcomes_session ON connection_outcomes(session_id);
CREATE INDEX idx_outcomes_caller ON connection_outcomes(caller_agent_id, created_at DESC);
CREATE INDEX idx_outcomes_value ON connection_outcomes(outcome_value);

-- benchmark_runs
CREATE INDEX idx_benchmarks_agent_skill ON benchmark_runs(agent_id, skill_id, created_at DESC);
CREATE INDEX idx_benchmarks_passing ON benchmark_runs(passing, created_at DESC);

-- ============================================================================
-- TRIGGERS
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_agents_updated_at
    BEFORE UPDATE ON agents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_skills_updated_at
    BEFORE UPDATE ON agent_skills
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_edges_updated_at
    BEFORE UPDATE ON agent_edges
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_providers_updated_at
    BEFORE UPDATE ON providers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- Update provider agent count when agents change
CREATE OR REPLACE FUNCTION update_provider_agent_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
        UPDATE providers
        SET agent_count = (
            SELECT COUNT(*) FROM agents WHERE provider_id = NEW.provider_id AND NOT delisted
        ),
        avg_agent_score = (
            SELECT COALESCE(AVG(agentrank_composite), 0) FROM agents WHERE provider_id = NEW.provider_id AND NOT delisted
        )
        WHERE id = NEW.provider_id;
    END IF;
    IF TG_OP = 'DELETE' OR TG_OP = 'UPDATE' THEN
        UPDATE providers
        SET agent_count = (
            SELECT COUNT(*) FROM agents WHERE provider_id = OLD.provider_id AND NOT delisted
        ),
        avg_agent_score = (
            SELECT COALESCE(AVG(agentrank_composite), 0) FROM agents WHERE provider_id = OLD.provider_id AND NOT delisted
        )
        WHERE id = OLD.provider_id;
    END IF;
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_agents_provider_count
    AFTER INSERT OR UPDATE OR DELETE ON agents
    FOR EACH ROW EXECUTE FUNCTION update_provider_agent_count();
```

### 17.2 ClickHouse Analytics Tables

ClickHouse stores the high-volume, append-heavy event data used for analytics, ranking feature computation, and dashboards.

```sql
-- Query log: every search API call
CREATE TABLE query_log (
    request_id          String,
    query_text          String,
    query_mode          Enum8('lexical' = 1, 'semantic' = 2, 'hybrid' = 3, 'graph' = 4),
    caller_agent_id     Nullable(UUID),
    caller_domain       Nullable(String),
    tenant_id           Nullable(UUID),
    top_k               UInt16,
    total_matches       UInt32,
    results_returned    UInt16,
    query_time_ms       Float32,
    lexical_candidates  UInt32,
    semantic_candidates UInt32,
    graph_candidates    UInt32,
    filters_applied     Array(String),
    result_agent_ids    Array(UUID),
    result_scores       Array(Float32),
    explain_requested   UInt8,
    facets_requested    UInt8,
    caller_region       Nullable(String),
    timestamp           DateTime64(3) DEFAULT now64(3)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (timestamp, request_id)
TTL timestamp + INTERVAL 365 DAY;

-- Crawl events: every fetch attempt
CREATE TABLE crawl_events (
    crawl_id            UUID,
    agent_id            Nullable(UUID),
    url                 String,
    http_status         Nullable(UInt16),
    response_time_ms    Nullable(Float32),
    response_size_bytes Nullable(UInt64),
    content_hash        Nullable(String),
    card_valid          Nullable(UInt8),
    card_changed        Nullable(UInt8),
    error_type          Enum8(
        'none' = 0,
        'dns_error' = 1,
        'connection_refused' = 2,
        'timeout' = 3,
        'tls_error' = 4,
        'http_4xx' = 5,
        'http_5xx' = 6,
        'parse_error' = 7,
        'validation_error' = 8,
        'rate_limited' = 9,
        'robots_blocked' = 10,
        'other' = 11
    ) DEFAULT 'none',
    error_message       Nullable(String),
    probe_level         Enum8(
        'crawl' = 0,
        'l1_ping' = 1,
        'l2_card_fetch' = 2,
        'l3_endpoint_check' = 3,
        'l4_capability_probe' = 4
    ),
    probe_region        String,
    discovery_source    Enum8(
        'well_known_crawl' = 1,
        'registry_feed' = 2,
        'dns_txt' = 3,
        'certificate_transparency' = 4,
        'code_repository' = 5,
        'interaction_graph' = 6,
        'user_submission' = 7,
        'api_registration' = 8,
        'partner_feed' = 9,
        'sitemap_crawl' = 10,
        'web_mention' = 11,
        'manual_import' = 12
    ),
    timestamp           DateTime64(3) DEFAULT now64(3)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (timestamp, url)
TTL timestamp + INTERVAL 180 DAY;

-- Connection events: every connect and outcome
CREATE TABLE connection_events (
    session_id          UUID,
    caller_agent_id     Nullable(UUID),
    target_agent_id     UUID,
    connection_mode     Enum8(
        'direct' = 1,
        'handshake_only' = 2,
        'proxy_first' = 3,
        'full_proxy' = 4,
        'managed_session' = 5
    ),
    auth_strategy       Enum8(
        'passthrough' = 1,
        'exchange' = 2,
        'broker_minted' = 3,
        'mutual_tls' = 4,
        'none' = 5
    ),
    outcome_value       Enum8(
        'connected' = 1,
        'completed' = 2,
        'partial' = 3,
        'timed_out' = 4,
        'rejected' = 5,
        'auth_failed' = 6,
        'error' = 7,
        'incompatible' = 8,
        'cancelled' = 9
    ),
    quality_score       Nullable(Float32),
    duration_ms         Nullable(UInt32),
    tokens_input        Nullable(UInt32),
    tokens_output       Nullable(UInt32),
    cost_usd            Nullable(Float32),
    resolve_time_ms     Float32,
    validate_time_ms    Float32,
    auth_time_ms        Float32,
    policy_time_ms      Float32,
    connect_time_ms     Float32,
    total_time_ms       Float32,
    retries             UInt8,
    caller_region       Nullable(String),
    target_region       Nullable(String),
    caller_trust_tier   Enum8('indexed' = 0, 'established' = 1, 'verified' = 2, 'trusted' = 3, 'authoritative' = 4),
    target_trust_tier   Enum8('indexed' = 0, 'established' = 1, 'verified' = 2, 'trusted' = 3, 'authoritative' = 4),
    signature_verified  Nullable(UInt8),
    tenant_id           Nullable(UUID),
    timestamp           DateTime64(3) DEFAULT now64(3)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (target_agent_id, timestamp)
TTL timestamp + INTERVAL 365 DAY;

-- Materialized view: daily agent connection stats (pre-aggregated)
CREATE MATERIALIZED VIEW daily_agent_connection_stats
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(day)
ORDER BY (target_agent_id, day)
AS SELECT
    target_agent_id,
    toDate(timestamp) AS day,
    count()                                                         AS total_connections,
    countIf(outcome_value = 'completed')                           AS completed_connections,
    countIf(outcome_value IN ('completed', 'partial'))             AS successful_connections,
    countIf(outcome_value IN ('timed_out', 'error', 'auth_failed')) AS failed_connections,
    countIf(outcome_value = 'cancelled')                           AS cancelled_connections,
    avg(quality_score)                                              AS avg_quality_score,
    avg(duration_ms)                                                AS avg_duration_ms,
    avg(total_time_ms)                                              AS avg_connect_time_ms,
    avg(cost_usd)                                                   AS avg_cost_usd,
    sum(tokens_input)                                               AS total_tokens_input,
    sum(tokens_output)                                              AS total_tokens_output,
    uniqExact(caller_agent_id)                                      AS unique_callers
FROM connection_events
GROUP BY target_agent_id, toDate(timestamp);

-- Materialized view: hourly probe stats for health dashboard
CREATE MATERIALIZED VIEW hourly_probe_stats
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(hour)
ORDER BY (agent_id, probe_level, hour)
AS SELECT
    agent_id,
    probe_level,
    toStartOfHour(timestamp) AS hour,
    count()                                           AS total_probes,
    countIf(error_type = 'none')                     AS successful_probes,
    avg(response_time_ms)                             AS avg_response_time_ms,
    quantileExact(0.99)(response_time_ms)             AS p99_response_time_ms
FROM crawl_events
WHERE agent_id IS NOT NULL
GROUP BY agent_id, probe_level, toStartOfHour(timestamp);
```

### 17.3 Trust Tiers

Trust tiers gate what an agent can do within the AgentRank ecosystem. Higher tiers unlock ranking influence, UI treatment, and feature access.

| Tier | Name | Criteria | Privileges |
|------|------|----------|-----------|
| **0** | **Indexed** | Agent discovered and card parsed successfully. No verification. | Appears in search (with low ranking). No badges. Cannot submit outcomes that influence other agents' rankings. |
| **1** | **Established** | Healthy for ≥7 days. ≥10 successful connections. Card consistent across ≥3 crawls. No abuse flags. | Moderate ranking influence. "Established" label in results. Outcomes count toward target agent ranking (weight: 0.5). |
| **2** | **Verified** | Provider domain verified (DNS TXT or meta tag). Card cryptographically signed. Uptime ≥95% over 30 days. ≥50 successful connections with ≥80% success rate. | Full ranking influence. "Verified" badge in results. Outcomes count at full weight (1.0). Access to Agent Search Console. |
| **3** | **Trusted** | All Verified criteria. ≥500 successful connections. Outcome success rate ≥85% over 90 days. Provider OAuth verified. L4 benchmark pass rate ≥90%. No abuse flags in 90 days. | Top-tier ranking influence. "Trusted" badge (prominent). Eligible for featured placement. Can submit outcomes that influence ranking at weight 1.5. Priority connection routing. |
| **4** | **Authoritative** | All Trusted criteria. ≥5000 successful connections. Recognized ecosystem authority (manual review). Contributes to agent graph through verified interactions. Provider has multiple trusted agents. | Maximum ranking influence. "Authoritative" badge. Eligible for default recommendations. Outcomes at weight 2.0. Priority crawl frequency. Featured in ecosystem reports. |

```rust
pub fn evaluate_trust_tier(
    agent: &AgentRecord,
    metrics: &AgentMetrics,
    provider: &ProviderRecord,
    now: DateTime<Utc>,
) -> TrustTier {
    // Tier 4: Authoritative
    if provider.verified
        && provider.verification_method.is_some()
        && metrics.total_connections_all_time >= 5000
        && metrics.outcome_success_rate_90d.unwrap_or(0.0) >= 0.85
        && metrics.l4_pass_rate_90d.unwrap_or(0.0) >= 0.90
        && agent.abuse_flags.is_empty()
        && provider.agent_count >= 3
        && provider.agents_at_trusted_tier >= 2
        && agent.manual_authority_flag
    {
        return TrustTier::Authoritative;
    }

    // Tier 3: Trusted
    if provider.verified
        && metrics.total_connections_all_time >= 500
        && metrics.outcome_success_rate_90d.unwrap_or(0.0) >= 0.85
        && metrics.l4_pass_rate_90d.unwrap_or(0.0) >= 0.90
        && agent.abuse_flags.is_empty()
        && days_since(agent.last_abuse_flag, now) > 90
        && agent.uptime_30d.unwrap_or(0.0) >= 0.95
        && provider.oauth_verified
    {
        return TrustTier::Trusted;
    }

    // Tier 2: Verified
    if provider.verified
        && agent.card_signed
        && agent.uptime_30d.unwrap_or(0.0) >= 0.95
        && metrics.total_connections_30d >= 50
        && metrics.outcome_success_rate_30d.unwrap_or(0.0) >= 0.80
    {
        return TrustTier::Verified;
    }

    // Tier 1: Established
    let healthy_days = days_since_status(agent.health_status_since, now);
    if healthy_days >= 7
        && metrics.total_connections_all_time >= 10
        && agent.card_version_count >= 3
        && agent.abuse_flags.is_empty()
        && matches!(agent.health_status, AgentStatus::Healthy | AgentStatus::Degraded)
    {
        return TrustTier::Established;
    }

    // Tier 0: Indexed (default)
    TrustTier::Indexed
}

fn days_since(timestamp: Option<DateTime<Utc>>, now: DateTime<Utc>) -> i64 {
    match timestamp {
        Some(t) => (now - t).num_days(),
        None => i64::MAX,
    }
}

fn days_since_status(status_since: DateTime<Utc>, now: DateTime<Utc>) -> i64 {
    (now - status_since).num_days()
}
```

### 17.4 Index Synchronization

The canonical PostgreSQL store must be kept in sync with the Tantivy full-text index, the Qdrant vector index, and the ClickHouse analytics store. This is achieved through a CDC (Change Data Capture) pipeline using Debezium and Kafka.

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                         INDEX SYNCHRONIZATION PIPELINE                            │
│                                                                                  │
│   ┌────────────┐    ┌──────────┐    ┌────────────────┐                          │
│   │            │    │          │    │                │                          │
│   │ PostgreSQL │───►│ Debezium │───►│  Kafka Topics  │                          │
│   │ (WAL)      │    │ CDC      │    │                │                          │
│   │            │    │          │    │ • agentrank.   │                          │
│   └────────────┘    └──────────┘    │   agents.cdc   │                          │
│                                      │ • agentrank.   │                          │
│                                      │   skills.cdc   │                          │
│                                      │ • agentrank.   │                          │
│                                      │   edges.cdc    │                          │
│                                      │ • agentrank.   │                          │
│                                      │   outcomes.cdc │                          │
│                                      └───────┬────────┘                          │
│                                              │                                   │
│                          ┌───────────────────┼───────────────────┐               │
│                          │                   │                   │               │
│                          ▼                   ▼                   ▼               │
│                   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐        │
│                   │   Tantivy    │   │   Qdrant     │   │  ClickHouse  │        │
│                   │   Worker     │   │   Worker     │   │   Worker     │        │
│                   │              │   │              │   │              │        │
│                   │ • Parse CDC  │   │ • Parse CDC  │   │ • Parse CDC  │        │
│                   │   event      │   │   event      │   │   event      │        │
│                   │ • Build      │   │ • Compute    │   │ • Transform  │        │
│                   │   document   │   │   embeddings │   │   to column  │        │
│                   │ • Index in   │   │   (if missing)│  │   format     │        │
│                   │   Tantivy    │   │ • Upsert     │   │ • Insert     │        │
│                   │ • Commit     │   │   vector     │   │   batch      │        │
│                   │   segment    │   │ • Update     │   │              │        │
│                   │              │   │   payload    │   │              │        │
│                   └──────────────┘   └──────────────┘   └──────────────┘        │
│                                                                                  │
│   ┌──────────────────────────────────────────────────────────────────────────┐   │
│   │                     CONSISTENCY GUARANTEES                                │   │
│   │                                                                          │   │
│   │  • At-least-once delivery via Kafka consumer offsets                      │   │
│   │  • Idempotent writes in all downstream stores (upsert semantics)         │   │
│   │  • Lag monitoring: alert if consumer lag > 30 seconds                    │   │
│   │  • Poison pill handling: dead-letter queue for unparseable events        │   │
│   │  • Full rebuild capability: replay from earliest Kafka offset            │   │
│   └──────────────────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────────────────────┘
```

**Kafka Topic Configuration:**

```
Topic: agentrank.agents.cdc
  Partitions: 32
  Replication Factor: 3
  Retention: 7 days (events) + compacted (latest per key)
  Key: agent_id (UUID)
  Value: Debezium CDC envelope (before/after/op/source)
  Compression: zstd

Topic: agentrank.skills.cdc
  Partitions: 16
  Replication Factor: 3
  Retention: 7 days + compacted
  Key: skill_id (UUID)
  Compression: zstd

Topic: agentrank.edges.cdc
  Partitions: 16
  Replication Factor: 3
  Retention: 7 days + compacted
  Key: edge_id (UUID)
  Compression: zstd

Topic: agentrank.outcomes.events
  Partitions: 32
  Replication Factor: 3
  Retention: 30 days (append-only, no compaction)
  Key: target_agent_id (UUID)
  Compression: zstd
```

**Full Rebuild Procedure:**

```rust
pub struct IndexRebuilder {
    pg_pool: Arc<PgPool>,
    tantivy_indexer: Arc<TantivyIndexer>,
    qdrant_client: Arc<QdrantClient>,
    embedding_model: Arc<EmbeddingModel>,
    batch_size: usize,
}

impl IndexRebuilder {
    pub async fn full_rebuild(&self) -> Result<RebuildReport, RebuildError> {
        let start = Instant::now();
        let mut report = RebuildReport::new();

        tracing::info!("Starting full index rebuild");

        // Phase 1: Count total agents for progress reporting
        let total_agents: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agents WHERE NOT delisted")
            .fetch_one(&*self.pg_pool)
            .await?;
        report.total_agents = total_agents as u64;
        tracing::info!(total_agents, "Rebuild target");

        // Phase 2: Clear existing indexes
        self.tantivy_indexer.clear_all().await?;
        self.qdrant_client.delete_collection("agents").await.ok();
        self.qdrant_client.create_collection("agents", &QdrantCollectionConfig {
            vectors_size: 768,
            distance: Distance::Cosine,
            on_disk: true,
            quantization: Some(ScalarQuantization::Int8),
        }).await?;

        // Phase 3: Stream agents from PostgreSQL in batches
        let mut offset = 0i64;
        let mut processed = 0u64;
        let mut tantivy_errors = 0u64;
        let mut qdrant_errors = 0u64;

        loop {
            let agents: Vec<AgentRow> = sqlx::query_as(
                r#"
                SELECT a.*, array_agg(s.name) as skill_names, array_agg(s.description) as skill_descriptions
                FROM agents a
                LEFT JOIN agent_skills s ON s.agent_id = a.id
                WHERE NOT a.delisted
                GROUP BY a.id
                ORDER BY a.id
                LIMIT $1 OFFSET $2
                "#
            )
            .bind(self.batch_size as i64)
            .bind(offset)
            .fetch_all(&*self.pg_pool)
            .await?;

            if agents.is_empty() {
                break;
            }

            let batch_count = agents.len();

            // Index into Tantivy (full-text)
            let tantivy_docs: Vec<TantivyDocument> = agents.iter()
                .map(|a| TantivyDocument::from_agent_row(a))
                .collect();

            if let Err(e) = self.tantivy_indexer.index_batch(&tantivy_docs).await {
                tracing::error!(error = %e, batch_offset = offset, "Tantivy batch index failed");
                tantivy_errors += batch_count as u64;
            }

            // Compute embeddings and index into Qdrant (vector)
            let texts_for_embedding: Vec<String> = agents.iter()
                .map(|a| format!(
                    "{} {} {} {}",
                    a.name,
                    a.description.as_deref().unwrap_or(""),
                    a.skill_names.join(", "),
                    a.tags.join(", ")
                ))
                .collect();

            match self.embedding_model.encode_batch(&texts_for_embedding).await {
                Ok(embeddings) => {
                    let points: Vec<QdrantPoint> = agents.iter()
                        .zip(embeddings.iter())
                        .map(|(a, emb)| QdrantPoint {
                            id: a.id,
                            vector: emb.clone(),
                            payload: QdrantPayload::from_agent_row(a),
                        })
                        .collect();

                    if let Err(e) = self.qdrant_client.upsert_batch("agents", &points).await {
                        tracing::error!(error = %e, batch_offset = offset, "Qdrant batch upsert failed");
                        qdrant_errors += batch_count as u64;
                    }
                }
                Err(e) => {
                    tracing::error!(error = %e, batch_offset = offset, "Embedding generation failed");
                    qdrant_errors += batch_count as u64;
                }
            }

            processed += batch_count as u64;
            offset += batch_count as i64;

            if processed % 10000 == 0 {
                tracing::info!(
                    processed,
                    total = total_agents,
                    pct = format!("{:.1}%", (processed as f64 / total_agents as f64) * 100.0),
                    "Rebuild progress"
                );
            }
        }

        // Phase 4: Commit Tantivy index
        self.tantivy_indexer.commit().await?;

        let elapsed = start.elapsed();
        report.processed_agents = processed;
        report.tantivy_errors = tantivy_errors;
        report.qdrant_errors = qdrant_errors;
        report.duration = elapsed;
        report.agents_per_second = processed as f64 / elapsed.as_secs_f64();

        tracing::info!(
            processed,
            tantivy_errors,
            qdrant_errors,
            duration_secs = elapsed.as_secs(),
            agents_per_sec = format!("{:.0}", report.agents_per_second),
            "Full rebuild complete"
        );

        Ok(report)
    }
}

#[derive(Debug)]
pub struct RebuildReport {
    pub total_agents: u64,
    pub processed_agents: u64,
    pub tantivy_errors: u64,
    pub qdrant_errors: u64,
    pub duration: Duration,
    pub agents_per_second: f64,
}
```

---

## 18. Canonical Schema Guidance

### 18.1 Complete Canonical Agent Field Table

Every field in the canonical agent record is categorized by source, type, and whether it is required. This table serves as the single reference for what data AgentRank stores per agent and where it comes from.

| # | Field | Category | Type | Source | Required | Notes |
|---|-------|----------|------|--------|----------|-------|
| 1 | `id` | Identity | UUID | Generated | Yes | Platform-assigned, globally unique |
| 2 | `canonical_url` | Identity | TEXT | Derived | Yes | Deduplicated URL (normalized) |
| 3 | `card_url` | Identity | TEXT | Crawled | Yes | `/.well-known/agent.json` URL |
| 4 | `endpoint_url` | Identity | TEXT | Card field `url` | Yes | A2A endpoint URL |
| 5 | `name` | Card | TEXT | Card field `name` | Yes | Agent display name |
| 6 | `description` | Card | TEXT | Card field `description` | No | Full description |
| 7 | `short_description` | Card | TEXT | Derived/LLM | No | Truncated or LLM-generated summary (≤160 chars) |
| 8 | `provider_id` | Provider | UUID | Entity resolution | No | FK to providers table |
| 9 | `provider_domain` | Provider | TEXT | URL extraction | Yes | Domain of the agent endpoint |
| 10 | `protocol_version` | Protocol | TEXT | Card field `protocolVersion` | Yes | A2A protocol version |
| 11 | `agent_version` | Protocol | TEXT | Card field `version` | No | Agent-specific version string |
| 12 | `card_hash` | Integrity | TEXT | Computed | Yes | SHA-256 of raw card JSON |
| 13 | `card_raw` | Integrity | JSONB | Crawled | No | Full raw card JSON (for audit/diff) |
| 14 | `tags` | Classification | TEXT[] | Card + derived | No | Combined tags from card and enrichment |
| 15 | `categories` | Classification | TEXT[] | Derived/ML | No | Auto-classified categories |
| 16 | `languages` | Classification | TEXT[] | Card field or inferred | No | Supported human languages |
| 17 | `input_modes` | Capability | TEXT[] | Card/skill fields | No | Accepted input modalities |
| 18 | `output_modes` | Capability | TEXT[] | Card/skill fields | No | Produced output modalities |
| 19 | `auth_schemes` | Auth | ENUM[] | Card field `authentication` | No | Supported auth mechanisms |
| 20 | `agentrank_composite` | Ranking | REAL | Computed | Yes | Overall AgentRank score [0,1] |
| 21 | `score_relevance_base` | Ranking | REAL | Computed | Yes | Base relevance score |
| 22 | `score_trust` | Ranking | REAL | Computed | Yes | Trust dimension score |
| 23 | `score_operational` | Ranking | REAL | Computed | Yes | Operational quality score |
| 24 | `score_outcome` | Ranking | REAL | Computed | Yes | Outcome success score |
| 25 | `score_authority` | Ranking | REAL | Computed | Yes | Graph authority score |
| 26 | `score_freshness` | Ranking | REAL | Computed | Yes | Freshness score |
| 27 | `score_documentation` | Ranking | REAL | Computed | Yes | Documentation quality score |
| 28 | `score_economic` | Ranking | REAL | Computed | Yes | Economic efficiency score |
| 29 | `score_compatibility` | Ranking | REAL | Computed | Yes | Protocol compatibility score |
| 30 | `discoverability_score` | Ranking | REAL | Computed | Yes | Provider-facing discoverability |
| 31 | `rank_version` | Ranking | BIGINT | Computed | Yes | Monotonic version counter |
| 32 | `trust_tier` | Trust | ENUM | Computed | Yes | Current trust tier (0-4) |
| 33 | `verification_methods` | Trust | ENUM[] | Observed | No | Methods used to verify |
| 34 | `abuse_flags` | Trust | TEXT[] | Detection system | No | Active abuse flags |
| 35 | `spam_score` | Trust | REAL | ML model | No | Spam likelihood [0,1] |
| 36 | `health_status` | Liveness | ENUM | Probes | Yes | Current health state |
| 37 | `uptime_7d` | Liveness | REAL | Probes (agg) | No | 7-day uptime percentage |
| 38 | `uptime_30d` | Liveness | REAL | Probes (agg) | No | 30-day uptime percentage |
| 39 | `avg_response_time_ms` | Liveness | REAL | Probes (agg) | No | Average probe response time |
| 40 | `discovery_source` | Crawl | ENUM | Crawler | Yes | How the agent was first found |
| 41 | `first_discovered_at` | Crawl | TIMESTAMPTZ | Crawler | Yes | First discovery timestamp |
| 42 | `last_crawled_at` | Crawl | TIMESTAMPTZ | Crawler | No | Last successful crawl |
| 43 | `last_card_change_at` | Crawl | TIMESTAMPTZ | Diff detection | No | Last card content change |
| 44 | `card_version_count` | Crawl | INTEGER | Counter | Yes | Number of distinct card versions |
| 45 | `visibility` | Access | ENUM | Config/policy | Yes | Public/private/restricted |
| 46 | `tenant_id` | Access | UUID | Config | No | For tenant-scoped agents |
| 47 | `pricing_model` | Economic | TEXT | Card/registry | No | e.g. "per_request", "subscription" |
| 48 | `estimated_cost_usd` | Economic | REAL | Card/observed | No | Estimated cost per request |
| 49 | `total_connections_30d` | Outcome | INTEGER | Aggregated | No | Connections in last 30 days |
| 50 | `outcome_success_rate_30d` | Outcome | REAL | Aggregated | No | Success rate in last 30 days |

### 18.2 Version Fields

Three version fields track different aspects of agent evolution:

| Field | Description | Format | Example | Changes When |
|-------|-------------|--------|---------|-------------|
| `protocol_version` | A2A protocol version the agent supports | Semantic version (major.minor) | `"1.0"` | Agent upgrades A2A protocol support |
| `agent_version` | Agent-specific release version | Arbitrary string (provider-defined) | `"2.3.1"`, `"2026.03.15"` | Provider deploys new agent version |
| `card_hash` | SHA-256 hash of the raw Agent Card JSON | `sha256:<hex>` | `"sha256:3a4b5c..."` | Any field in the card JSON changes |

**Protocol Compatibility Matrix:**

| Caller Version | Target Version | Compatibility | Notes |
|---------------|---------------|---------------|-------|
| 1.0 | 1.0 | Exact | Full compatibility |
| 1.1 | 1.0 | Backward | Caller may use features target doesn't support |
| 1.0 | 1.1 | Forward | Target may send features caller doesn't understand |
| 1.0 | 2.0 | Incompatible | Major version break — no interoperability |
| 2.0 | 2.1 | Backward | Same major version — compatible |
| 1.* | 1.* | Compatible | Any 1.x ↔ 1.x is compatible (semantic versioning) |

### 18.3 Skill Model

Each agent skill is stored as an independent record with its own embedding for semantic search.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalSkill {
    pub id: String,
    pub agent_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub input_modes: Vec<String>,
    pub output_modes: Vec<String>,
    pub examples: Vec<String>,
    pub embedding: Option<Vec<f32>>,
    pub benchmark_quality: Option<f64>,
    pub benchmark_latency_ms: Option<f64>,
    pub benchmark_tested_at: Option<DateTime<Utc>>,
}

pub async fn generate_skill_embedding(
    skill: &CanonicalSkill,
    model: &EmbeddingModel,
) -> Result<Vec<f32>, EmbeddingError> {
    // Construct embedding text from all relevant skill fields.
    // Weighted by importance: name and description carry more signal than examples.
    let embedding_text = format!(
        "{name}. {description} Tags: {tags}. Examples: {examples}",
        name = skill.name,
        description = skill.description.as_deref().unwrap_or(""),
        tags = skill.tags.join(", "),
        examples = skill.examples.join("; "),
    );

    // Truncate to model's max context length (512 tokens for bge-base)
    let truncated = truncate_to_tokens(&embedding_text, 512);

    let embedding = model.encode(&truncated).await?;

    // Normalize to unit vector for cosine similarity
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    let normalized: Vec<f32> = if norm > 0.0 {
        embedding.iter().map(|x| x / norm).collect()
    } else {
        embedding
    };

    Ok(normalized)
}
```

### 18.4 Evidence Model

Every claim about an agent in the canonical registry is backed by evidence. The evidence model tracks where information came from, how confident we are in it, and when it was last verified.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub evidence_type: EvidenceType,
    pub source: EvidenceSource,
    pub signer: Option<SignerIdentity>,
    pub confidence: f64,
    pub observed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub raw_data_hash: Option<String>,
    pub raw_data_url: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    CardFetch {
        card_hash: String,
        card_url: String,
        http_status: u16,
        response_time_ms: f64,
    },
    HealthProbe {
        probe_level: ProbeLevel,
        probe_region: String,
        status: ProbeStatus,
        latency_ms: f64,
    },
    BenchmarkResult {
        skill_id: String,
        overall_score: f64,
        passing: bool,
        prompt_hash: String,
    },
    ConnectionOutcome {
        session_id: Uuid,
        outcome_value: String,
        quality_score: Option<f64>,
        reporter_trust_tier: TrustTier,
    },
    DomainVerification {
        domain: String,
        method: VerificationMethod,
        proof: String,
    },
    CardSignature {
        algorithm: String,
        public_key: String,
        signature: String,
        verified: bool,
    },
    RegistryListing {
        registry_name: String,
        registry_id: String,
        listing_url: String,
        listing_data: serde_json::Value,
    },
    InteractionObservation {
        caller_agent_id: Uuid,
        target_agent_id: Uuid,
        interaction_type: String,
        observed_by: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerIdentity {
    pub signer_type: SignerType,
    pub identifier: String,
    pub public_key: Option<String>,
    pub trust_tier: TrustTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignerType {
    Platform,
    Provider,
    CallerAgent,
    ThirdPartyVerifier,
    CertificateAuthority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceSource {
    pub source_type: String,
    pub source_id: Option<String>,
    pub source_url: Option<String>,
    pub source_trust: f64,
}
```

**Confidence Scoring Rules:**

| Evidence Type | Base Confidence | Modifiers | Decay Rate |
|--------------|----------------|-----------|-----------|
| **CardFetch** | 0.7 | +0.1 if HTTPS, +0.05 if card signed, -0.2 if HTTP errors in last 24h | 10% per week without re-crawl |
| **HealthProbe L1** | 0.5 | +0.1 per additional region confirming, -0.3 if any region fails | 50% per day without fresh probe |
| **HealthProbe L2** | 0.6 | +0.1 if card unchanged (consistency), +0.05 if card signed | 30% per day |
| **HealthProbe L3** | 0.75 | +0.1 if response time < 500ms, -0.1 per consecutive failure | 30% per day |
| **HealthProbe L4** | 0.85 | Score proportional to benchmark quality. +0.1 if multiple skills pass | 5% per week |
| **ConnectionOutcome** | 0.6–0.9 | Scales with reporter trust tier: Indexed=0.6, Established=0.7, Verified=0.8, Trusted=0.85, Authoritative=0.9 | 5% per month |
| **DomainVerification** | 0.95 | Essentially binary — verified or not | 0% (until verification expires or fails re-check) |
| **CardSignature** | 0.90 | +0.05 if key pinned in DNS, -0.5 if signature invalid | 0% (until key rotation) |
| **RegistryListing** | 0.4 | +0.1 if registry is well-known, +0.1 if listing data matches card | 15% per month without re-confirmation |
| **InteractionObservation** | 0.5 | +0.1 if observer is Verified+, +0.1 if reciprocal observation exists | 10% per month |

**Evidence Decay Function:**

Evidence confidence decays over time to ensure the system does not rely on stale data. The decay function is exponential with a type-specific half-life.

```rust
pub fn compute_decayed_confidence(
    base_confidence: f64,
    observed_at: DateTime<Utc>,
    now: DateTime<Utc>,
    evidence_type: &EvidenceType,
) -> f64 {
    let age = now - observed_at;
    let age_days = age.num_seconds() as f64 / 86400.0;

    if age_days <= 0.0 {
        return base_confidence;
    }

    let half_life_days = match evidence_type {
        EvidenceType::HealthProbe { probe_level: ProbeLevel::L1Ping, .. } => 1.0,
        EvidenceType::HealthProbe { probe_level: ProbeLevel::L2CardFetch, .. } => 2.0,
        EvidenceType::HealthProbe { probe_level: ProbeLevel::L3EndpointCheck, .. } => 2.0,
        EvidenceType::HealthProbe { probe_level: ProbeLevel::L4CapabilityProbe, .. } => 14.0,
        EvidenceType::CardFetch { .. } => 7.0,
        EvidenceType::ConnectionOutcome { .. } => 30.0,
        EvidenceType::BenchmarkResult { .. } => 14.0,
        EvidenceType::DomainVerification { .. } => 365.0,
        EvidenceType::CardSignature { .. } => 365.0,
        EvidenceType::RegistryListing { .. } => 60.0,
        EvidenceType::InteractionObservation { .. } => 30.0,
    };

    let decay_factor = (0.5_f64).powf(age_days / half_life_days);
    let decayed = base_confidence * decay_factor;

    // Floor: evidence never drops below 5% confidence (still contributes minimally)
    decayed.max(0.05)
}

pub fn aggregate_evidence_confidence(evidences: &[Evidence], now: DateTime<Utc>) -> f64 {
    if evidences.is_empty() {
        return 0.0;
    }

    // Weighted average of decayed confidences, with recency bias
    let mut total_weight = 0.0;
    let mut weighted_sum = 0.0;

    for evidence in evidences {
        let decayed = compute_decayed_confidence(
            evidence.confidence,
            evidence.observed_at,
            now,
            &evidence.evidence_type,
        );

        // More recent evidence gets higher weight
        let age_days = (now - evidence.observed_at).num_seconds() as f64 / 86400.0;
        let recency_weight = 1.0 / (1.0 + age_days * 0.1);

        // Higher confidence sources get higher weight
        let source_weight = evidence.source.source_trust;

        let weight = recency_weight * source_weight;
        weighted_sum += decayed * weight;
        total_weight += weight;
    }

    if total_weight > 0.0 {
        (weighted_sum / total_weight).min(1.0)
    } else {
        0.0
    }
}
```

---

*End of Part V: Sections 14–18*

*This document covers the connection architecture (§14), liveness and verification system (§15), the complete API and protocol layer (§16), the full data model and schema design (§17), and canonical schema guidance (§18). These sections define the operational backbone of AgentRank — the systems that turn discovery into connection, connection into outcomes, and outcomes into the data moat that makes the ranking smarter over time.*
## Part VIII: Infrastructure

---

## 19. Technology Stack & Language Decisions

The technology choices in AgentRank are not accidents. Every language, framework, database, and library was selected through a rigorous evaluation process that weighs performance, safety, operational simplicity, and long-term maintainability. This section documents those decisions and their rationale so that future engineers understand not just *what* we chose, but *why* — and what the alternatives would cost.

### 19.1 Primary Language: Rust

AgentRank's core services — the crawler, the API gateway, the ranking engine, the connection broker, the index synchronizer — are all written in Rust. This is a deliberate, load-bearing architectural decision that affects every aspect of the system.

#### Justification Matrix

| Criterion | Rust Advantage | Impact on AgentRank |
|-----------|---------------|-------------------|
| **Performance** | Zero-cost abstractions, no GC pauses, predictable P99 latencies | Crawler throughput: 500+ pages/sec/core. Search P99 < 100ms guaranteed. No stop-the-world pauses during ranking computation. |
| **Memory Safety** | Ownership model eliminates use-after-free, double-free, data races at compile time | Crawler processes untrusted input (agent cards from the open internet). Memory safety is not optional — it is a security requirement. |
| **Async Runtime** | Tokio provides work-stealing async executor with ~200 bytes per task | 100K concurrent crawl connections per node. Each crawler worker handles 10K+ in-flight requests simultaneously. Go goroutines cost 4KB minimum — 20× more memory per concurrent unit. |
| **Ecosystem Fit** | Tantivy (full-text search), Qdrant client, reqwest (HTTP), serde (serialization), governor (rate limiting), cuckoo-filter (dedup) — all native Rust | No FFI boundaries for core functionality. The entire hot path from HTTP fetch → parse → index → search → rank → respond is pure Rust with zero marshaling overhead. |
| **Operational Model** | Single statically-linked binary per service. No runtime, no VM, no interpreter. | Container images < 20MB. Startup time < 100ms. No JVM warmup, no Python interpreter initialization. Cold-start autoscaling responds in seconds, not minutes. |
| **Type System** | Algebraic data types, exhaustive pattern matching, trait-based generics | Agent card validation logic is encoded in the type system. Invalid states are unrepresentable. A card that passes `parse()` is guaranteed to have all required fields with correct types. |
| **Concurrency Safety** | `Send + Sync` bounds, `Arc<Mutex<T>>`, channels — all enforced at compile time | The crawler's frontier is a concurrent data structure shared across workers. Rust guarantees no data races. In Go or Java, this is a runtime prayer. |
| **Long-term Maintainability** | Compiler catches refactoring errors. No null pointer exceptions. No "undefined is not a function." | When the ranking formula changes (and it will change constantly), the compiler ensures every call site handles the new signature. Large-scale refactoring is safe. |
| **Binary Size** | Optimized release binaries with LTO and strip: 15-30MB per service | Minimal container images. Fast pull times in Kubernetes. Lower storage costs in registries. |
| **Compile-time Guarantees** | Lifetimes, borrow checker, const generics | Zero runtime panics in production for well-tested code paths. The type system is the first line of defense before any test runs. |

#### Why NOT Other Languages

| Language | Fatal Flaw for AgentRank | Specific Technical Cost |
|----------|--------------------------|------------------------|
| **Go** | GC-induced P99 latency spikes of 1-10ms during mark phase. No Tantivy equivalent (Bleve is 5-10× slower). Goroutines cost 4KB minimum stack vs Tokio's 200B per task. No algebraic data types — error handling is `if err != nil` ceremony. | At 100K concurrent crawl connections, Go uses 400MB just for goroutine stacks. Rust uses 20MB for equivalent tasks. GC pauses during search ranking cause P99 spikes that violate our 100ms SLO. The lack of sum types means agent card validation is a fragile chain of nil checks instead of exhaustive pattern matching. |
| **Python** | 50-100× slower than Rust for CPU-bound work. GIL prevents true parallelism. Asyncio event loop is single-threaded. Dynamic typing means runtime crashes on type mismatches. | The ranking computation (BM25 + vector similarity + AVERT score + LTR features) is CPU-intensive. Python cannot compute this within our P99 budget. Even with PyPy or Cython, the overhead of crossing the Python-C boundary for every Tantivy query is prohibitive. Python is acceptable for ML training pipelines (see §19.2) but not for the hot path. |
| **Java/Kotlin** | JVM GC pauses (G1 or ZGC still have P99 tails). JVM warmup time of 5-30 seconds for JIT compilation. Memory overhead: 12-16 byte object header on every allocation. Classpath hell. | A Java search service needs 2-4GB heap minimum for competitive performance. The same Rust service runs in 200MB. JVM warmup means autoscaled pods serve slow responses for the first 10-30 seconds — unacceptable for search. Kubernetes readiness probes would need 30-second delays, slowing scaling response. |
| **C++** | Memory safety is the developer's responsibility. Use-after-free, buffer overflow, double-free are all possible and historically common. No package manager consensus (Conan vs vcpkg vs CMake FetchContent). | AgentRank processes untrusted input from the open internet. Every agent card is potentially malicious. A single buffer overflow in the card parser could lead to remote code execution. The cost of a CVE in a search engine that indexes the agent ecosystem is existential. C++ is not worth the risk. |
| **TypeScript/Node.js** | Single-threaded event loop (worker threads are an afterthought). V8 GC pauses. No native full-text search library of Tantivy's quality. Memory usage 5-10× higher than Rust. | Node.js is appropriate for the web UI (Next.js — see §19.2) but cannot serve the search hot path. A Node.js search service handling 1000 QPS would need 20+ replicas where Rust needs 3. The operational cost difference compounds at scale. |
| **Zig** | Immature ecosystem. No equivalent to Tokio, Tantivy, Qdrant client, or serde. Standard library still in flux. | Zig is promising for systems programming but lacks the library ecosystem AgentRank needs today. We would spend months reimplementing what Rust provides out of the box. Revisit in 2028. |

#### The Rust Tax: Honest Assessment

Rust is not free. These are the costs we accept:

| Cost | Mitigation |
|------|-----------|
| **Steeper learning curve** | Hiring bar filters for Rust experience or strong systems background. Onboarding includes 2-week Rust bootcamp. |
| **Longer compile times** | `cargo check` for fast iteration. Incremental compilation. `sccache` for CI. Workspace split reduces recompilation blast radius. |
| **Slower prototyping** | Python prototypes for ML experiments. Rust rewrites when moving to production. The boundary is clear: Python for research, Rust for serving. |
| **Smaller talent pool** | Rust is the most-loved language on Stack Overflow for 8 consecutive years. Engineers want to write Rust. Recruiting is a feature, not a bug. |
| **Borrow checker friction** | `Arc<T>` for shared ownership. `clone()` when the borrow checker fights back. Profiler-guided optimization after correctness is established. Premature optimization of ownership is worse than a clone. |

### 19.2 Full Stack Component Breakdown

Every component in AgentRank has an explicit technology assignment. No technology is used "because it was there." Each selection is justified by its role in the system.

| Component | Technology | Language | Purpose | Why This Choice |
|-----------|-----------|----------|---------|----------------|
| **AgentBot (Crawler)** | Rust + Tokio + reqwest + tower | Rust | Autonomous web crawler for agent card discovery | Tokio's work-stealing executor handles 100K concurrent connections. `reqwest` provides HTTP/1.1 and HTTP/2 with automatic connection pooling. `tower` middleware provides retry, timeout, rate-limit layers. Single binary deploys to K8s with zero dependencies. |
| **Card Parser** | Rust + serde + jsonschema | Rust | Parse, validate, normalize agent cards | `serde` zero-copy deserialization directly into typed structs. `jsonschema` validates against the A2A spec. Custom normalization rules (§9.2) are pure Rust functions operating on typed data. Invalid cards are rejected at the type level. |
| **URL Frontier** | Redis 7 (Cluster Mode) | N/A (data store) | Priority queue for crawl URLs | Redis Sorted Sets provide O(log N) priority insert and O(1) min extraction. Cluster mode shards across domain hash for locality. 1M URLs in frontier uses ~500MB RAM. Persistence via RDB snapshots every 60s. |
| **Deduplication Engine** | Rust + cuckoo-filter | Rust | Near-duplicate detection for agent cards | Cuckoo filter provides O(1) lookup with 3% false positive rate in ~8 bytes per entry. 10M entries use 80MB RAM. SimHash comparison for content-level dedup when cuckoo filter triggers. |
| **Rate Limiter** | Rust + governor | Rust | Per-domain and global rate limiting | `governor` implements Generic Cell Rate Algorithm (GCRA) with zero allocation in steady state. Per-domain limits stored in `DashMap` for lock-free concurrent access. |
| **Full-Text Search** | Tantivy 0.22+ | Rust | BM25 text search over agent metadata | Tantivy is the Rust equivalent of Apache Lucene. It provides BM25F scoring, phrase queries, fuzzy matching, and field boosting. Index segments are memory-mapped for zero-copy reads. 10M agents indexed in < 5GB on disk. |
| **Vector Search** | Qdrant 1.9+ | Rust (client) | Semantic similarity search over agent embeddings | HNSW index with quantization. 768-dimensional embeddings at 10M scale require ~30GB RAM with scalar quantization. Qdrant's filtering during search enables metadata-constrained vector queries. gRPC client for low-latency communication. |
| **Primary Database** | PostgreSQL 16 | SQL | Agent registry, trust records, verification state | PostgreSQL's JSONB columns store flexible agent metadata alongside typed relational columns. Row-level security for multi-tenant enterprise features. Logical replication to ClickHouse for analytics. `pg_trgm` for fuzzy text search as a fallback. |
| **Analytics Database** | ClickHouse 24+ | SQL (ClickHouse dialect) | Query logs, crawl telemetry, outcome analytics | Columnar storage compresses crawl logs 10-20×. `MergeTree` engine handles 100K inserts/sec. Materialized views pre-aggregate metrics for dashboards. Time-series queries over crawl history are 100× faster than PostgreSQL. |
| **Cache Layer** | Redis 7 (Sentinel) | N/A (data store) | Search result caching, session state, hot data | Redis Cluster for frontier (write-heavy). Redis Sentinel for cache (read-heavy, simpler topology). Cache hit rates > 40% for popular queries. TTL-based invalidation aligned with index refresh cycle. |
| **Object Storage** | S3 / Cloudflare R2 | N/A (cloud service) | Raw agent card snapshots, crawl archives, index backups | R2 for zero-egress-cost reads. S3 for compatibility with existing tooling. Raw cards stored as gzip-compressed JSON with content-hash keys for deduplication. Index snapshots uploaded on each merge cycle. |
| **Event Streaming** | Apache Kafka 3.7+ / Redpanda | N/A (infrastructure) | Inter-service event bus | Topics: `agent.discovered`, `agent.validated`, `agent.indexed`, `agent.scored`, `query.executed`, `connect.initiated`, `outcome.reported`. Exactly-once semantics via idempotent producers. 7-day retention for replay capability. |
| **Embedding Generation** | Python + ONNX Runtime + sentence-transformers | Python | Generate 768-dim embeddings from agent descriptions | `all-MiniLM-L6-v2` for v1 (fast, 384-dim). Upgrade path to `bge-large-en-v1.5` (1024-dim) or domain-fine-tuned model. ONNX Runtime for 3× inference speedup over PyTorch. Batch processing: 512 cards/sec on CPU, 5000/sec on GPU. |
| **LTR Training Pipeline** | Python + XGBoost + scikit-learn | Python | Train learning-to-rank models from labeled data | XGBoost `rank:ndcg` objective. Feature engineering in pandas from PostgreSQL exports. Hyperparameter tuning via Optuna. Model evaluation on held-out judgment sets. Weekly retraining cadence. |
| **LTR Serving** | Rust + ort (ONNX Runtime) | Rust | Real-time LTR inference in the ranking pipeline | Trained XGBoost model exported to ONNX format. `ort` crate provides native Rust bindings to ONNX Runtime. Inference latency < 1ms for 100 candidates × 50 features. No Python in the hot path. |
| **Query Understanding** | Python + LLM (local or API) | Python | Intent classification, query expansion, entity extraction | Local: `Phi-3-mini` via `llama.cpp` for privacy. API: GPT-4o-mini for higher quality. Latency budget: 50ms for intent classification, 200ms for query expansion. Results cached aggressively. |
| **Web UI** | Next.js 15 + React 19 + Tailwind CSS 4 | TypeScript | Public search interface and Agent Search Console | Server Components for SEO. Streaming SSR for perceived performance. Tailwind for rapid UI iteration. Shadcn/ui component library. Edge-deployed via Vercel or Cloudflare Pages. |
| **API Gateway** | Rust + axum + tower | Rust | Authentication, rate limiting, request routing | `axum` provides typed extractors and compile-time route validation. `tower` middleware stack: auth → rate-limit → timeout → compression → logging. 50K RPS per replica on a 2-core pod. |
| **Connection Broker** | Rust + axum + WebSocket | Rust | Mediate agent-to-agent connection setup | Stateful WebSocket connections for real-time connection negotiation. Session state in Redis for horizontal scaling. Graceful handoff when broker steps out of the data path after setup. |
| **Monitoring** | Prometheus + Grafana + Alertmanager | N/A (infrastructure) | Metrics collection, visualization, alerting | Prometheus scrapes all Rust services via `/metrics` endpoint (exposed by `metrics` crate). Grafana dashboards per service. Alertmanager routes to PagerDuty and Slack. |
| **Log Aggregation** | Vector + ClickHouse | Rust (Vector agent) | Structured log collection and analysis | Vector agents on each node collect structured JSON logs. Ship to ClickHouse `logs` table partitioned by day. 30-day retention. Full-text search over log messages via ClickHouse `tokenbf_v1` index. |
| **Tracing** | OpenTelemetry + Jaeger | Rust (tracing-opentelemetry) | Distributed request tracing | `tracing` crate for structured spans. OpenTelemetry exporter to Jaeger. Trace sampling: 1% in production, 100% when debug header present. Cross-service trace propagation via `traceparent` header. |
| **CI/CD** | GitHub Actions + Docker + Helm | YAML | Build, test, deploy pipeline | Rust builds cached via `sccache` and GitHub Actions cache. Docker multi-stage builds for minimal images. Helm charts for Kubernetes deployment. Staging → Canary → Production promotion gates. |
| **Infrastructure** | Kubernetes 1.30+ + Helm + Terraform | HCL / YAML | Container orchestration and infrastructure-as-code | GKE or EKS for managed Kubernetes. Terraform for cloud resource provisioning. Helm for application deployment. Kustomize overlays for environment-specific configuration. |
| **DNS & CDN** | Cloudflare | N/A (cloud service) | DNS, DDoS protection, edge caching | Cloudflare's global anycast network for DDoS protection. Edge caching for static assets and popular search results. DNS management for domain verification flows. |
| **Secrets Management** | HashiCorp Vault / K8s Secrets | N/A (infrastructure) | API keys, database credentials, TLS certificates | Vault for dynamic database credentials with TTL-based rotation. Kubernetes Secrets for non-sensitive configuration. External Secrets Operator to sync Vault → K8s Secrets. |

### 19.3 Rust Crate Selection

The following `Cargo.toml` dependencies represent the core of every AgentRank Rust service. Individual services include subsets of these dependencies based on their needs.

```toml
[workspace]
members = [
    "crates/agentbot",          # Crawler service
    "crates/card-parser",       # Agent card parsing and validation
    "crates/api-gateway",       # Public API gateway
    "crates/search-engine",     # Query processing and ranking
    "crates/connection-broker",  # Agent-to-agent connection mediation
    "crates/rank-computer",     # Background ranking computation
    "crates/liveness-prober",   # Agent liveness checking
    "crates/index-sync",        # Tantivy/Qdrant index synchronization
    "crates/common",            # Shared types, traits, and utilities
    "crates/proto",             # Protobuf/gRPC definitions
]
resolver = "2"

[workspace.dependencies]
# Async Runtime
tokio = { version = "1.40", features = ["full"] }
tokio-util = { version = "0.7", features = ["codec", "time"] }

# Web Framework
axum = { version = "0.7", features = ["ws", "multipart", "macros"] }
axum-extra = { version = "0.9", features = ["typed-header", "cookie"] }
tower = { version = "0.5", features = ["full"] }
tower-http = { version = "0.6", features = [
    "cors", "compression-gzip", "trace", "timeout",
    "request-id", "set-header", "limit"
] }
hyper = { version = "1.4", features = ["full"] }

# HTTP Client
reqwest = { version = "0.12", features = [
    "json", "gzip", "brotli", "rustls-tls", "trust-dns", "stream",
    "hickory-dns", "charset", "http2"
] }

# Rate Limiting
governor = { version = "0.7", features = ["dashmap"] }

# Data Structures
cuckoofilter = "0.5"
dashmap = "6.0"
crossbeam = { version = "0.8", features = ["crossbeam-channel", "crossbeam-queue"] }
parking_lot = "0.12"
bytes = "1.7"
smallvec = { version = "1.13", features = ["union", "const_generics"] }

# URL and DNS
url = "2.5"
hickory-resolver = "0.24"

# Robots.txt
robotstxt = "0.6"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
toml = "0.8"

# Validation
jsonschema = "0.18"
validator = { version = "0.18", features = ["derive"] }

# Unique Identifiers (UUIDv7 — time-ordered for database locality)
uuid = { version = "1.10", features = ["v7", "serde"] }

# Time
chrono = { version = "0.4", features = ["serde"] }
time = "0.3"

# Hashing
blake3 = "1.5"
xxhash-rust = { version = "0.8", features = ["xxh3"] }
sha2 = "0.10"

# Full-Text Search
tantivy = "0.22"

# Vector Search Client
qdrant-client = "1.10"

# Database
sqlx = { version = "0.8", features = [
    "runtime-tokio", "tls-rustls", "postgres", "uuid",
    "chrono", "json", "migrate"
] }

# Redis
redis = { version = "0.26", features = [
    "tokio-comp", "cluster-async", "connection-manager"
] }

# ClickHouse
clickhouse = { version = "0.12", features = ["lz4"] }

# Kafka
rdkafka = { version = "0.36", features = ["cmake-build", "ssl", "sasl"] }

# Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = [
    "env-filter", "json", "fmt"
] }
tracing-opentelemetry = "0.25"
opentelemetry = "0.24"
opentelemetry-otlp = "0.17"
metrics = "0.23"
metrics-exporter-prometheus = "0.15"

# ML Inference (ONNX Runtime for LTR serving)
ort = { version = "2.0", features = ["half"] }

# Error Handling
anyhow = "1.0"
thiserror = "2.0"

# Configuration
config = "0.14"
dotenvy = "0.15"

# CLI
clap = { version = "4.5", features = ["derive", "env"] }

# Testing
tokio-test = "0.4"
wiremock = "0.6"
fake = { version = "2.9", features = ["chrono", "uuid"] }
proptest = "1.5"
criterion = { version = "0.5", features = ["html_reports", "async_tokio"] }
insta = { version = "1.39", features = ["json", "yaml"] }

# Misc
rand = "0.8"
regex = "1.10"
once_cell = "1.19"
itertools = "0.13"
futures = "0.3"
pin-project-lite = "0.2"
async-trait = "0.1"
```

#### Crate Selection Rationale for Key Choices

| Crate | Why This Over Alternatives |
|-------|--------------------------|
| `axum` over `actix-web` | `axum` uses `tower` middleware natively, composes with the broader Tower ecosystem. `actix-web` has its own middleware system that doesn't interoperate. `axum`'s typed extractors catch routing errors at compile time. |
| `reqwest` over `hyper` directly | `reqwest` provides connection pooling, cookie handling, redirect following, and TLS configuration out of the box. Using raw `hyper` would require reimplementing all of this. `reqwest` uses `hyper` internally. |
| `governor` over hand-rolled | GCRA algorithm is mathematically proven to be fair and burst-tolerant. `governor` integrates with `DashMap` for lock-free per-key rate limiting. Hand-rolling rate limiters is a common source of bugs and race conditions. |
| `uuid` v7 over v4 | UUIDv7 is time-ordered, which means database inserts are sequential and B-tree friendly. UUIDv4 is random, causing page splits and write amplification in PostgreSQL. At 10M agents, the difference is measurable in insert throughput and index size. |
| `blake3` over SHA-256 | BLAKE3 is 4-14× faster than SHA-256 on modern hardware with SIMD acceleration. We use it for content hashing (deduplication), not cryptographic signatures. For signatures, we use SHA-256 via the `sha2` crate. |
| `sqlx` over `diesel` | `sqlx` validates queries against the actual database schema at compile time (with `sqlx prepare`). `diesel` uses a DSL that doesn't support all PostgreSQL features (JSONB operators, window functions). `sqlx` is async-native; `diesel` requires `spawn_blocking`. |
| `rdkafka` over `kafka-rust` | `rdkafka` wraps `librdkafka`, the reference C client maintained by Confluent. It handles consumer rebalancing, exactly-once semantics, and SASL authentication correctly. Pure-Rust Kafka clients are immature. |
| `tantivy` over `meilisearch` | Tantivy is a library we embed in-process. Meilisearch is a separate service we'd have to operate. In-process search eliminates network overhead and gives us full control over indexing and scoring. |
| `ort` over `tch-rs` | `ort` wraps ONNX Runtime, which supports models from any framework (XGBoost, PyTorch, TensorFlow). `tch-rs` wraps LibTorch and only supports PyTorch models. ONNX is the lingua franca of ML deployment. |
| `insta` for snapshot testing | Snapshot tests capture the serialized output of complex data structures and alert on changes. Ideal for testing ranking score computation, card parsing output, and API response shapes. |

---

## 20. Infrastructure & Deployment Architecture

### 20.1 Service Topology

The following diagram shows the complete service topology at Phase 1 steady state. Replica counts are per-environment defaults for production.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              EDGE / INGRESS                                 │
│                                                                             │
│    ┌───────────────┐    ┌──────────────┐    ┌──────────────────────┐        │
│    │  Cloudflare   │───▶│   Ingress    │───▶│   NGINX Ingress      │        │
│    │  CDN / WAF    │    │  Controller  │    │   Controller (3)     │        │
│    └───────────────┘    └──────────────┘    └──────────┬───────────┘        │
│                                                        │                    │
│         ┌──────────────────┬───────────────────────────┤                    │
│         │                  │                           │                    │
│         ▼                  ▼                           ▼                    │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐            │
│  │  Web UI      │  │  API Gateway │  │  A2A Self-Discovery    │            │
│  │  (Next.js)   │  │  (axum)      │  │  Endpoint (axum)       │            │
│  │  ×3 replicas │  │  ×5 replicas │  │  ×5 replicas           │            │
│  └──────┬───────┘  └──────┬───────┘  └────────┬───────────────┘            │
│         │                  │                   │                            │
└─────────┼──────────────────┼───────────────────┼────────────────────────────┘
          │                  │                   │
          ▼                  ▼                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           QUERY PATH                                        │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────┐       │
│  │                     Search Engine (×5 replicas)                   │       │
│  │  ┌──────────────┐ ┌──────────┐ ┌────────────┐ ┌──────────────┐  │       │
│  │  │    Query     │ │ Candidate│ │   Result   │ │  Re-Ranker   │  │       │
│  │  │ Understanding│→│Generator │→│  Fusion    │→│  (LTR/ONNX)  │  │       │
│  │  └──────────────┘ └──────────┘ └────────────┘ └──────────────┘  │       │
│  └──────┬──────────────────┬────────────────────────────────────────┘       │
│         │                  │                                                │
│    ┌────▼─────┐   ┌───────▼────────┐   ┌────────────────┐                  │
│    │ Tantivy  │   │    Qdrant      │   │    Redis       │                  │
│    │ (in-proc)│   │  (×3 replicas) │   │   Cache (×3)   │                  │
│    └──────────┘   └────────────────┘   └────────────────┘                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                         CONNECTION PATH                                     │
│                                                                             │
│  ┌────────────────────────────────────────────────────────────┐             │
│  │          Connection Broker (×3 replicas)                    │             │
│  │  ┌──────────┐ ┌──────────────┐ ┌────────────────────────┐  │             │
│  │  │ Session  │ │  Capability  │ │  Protocol Negotiation  │  │             │
│  │  │ Manager  │ │  Validator   │ │  + Handoff             │  │             │
│  │  └──────────┘ └──────────────┘ └────────────────────────┘  │             │
│  └──────┬─────────────────────────────────────────────────────┘             │
│         │                                                                   │
│    ┌────▼────────────┐                                                      │
│    │  Redis Sessions │                                                      │
│    │     (×3)        │                                                      │
│    └─────────────────┘                                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                        BACKGROUND / CRAWL PATH                              │
│                                                                             │
│  ┌──────────────────┐  ┌──────────────────┐  ┌───────────────────────┐     │
│  │   AgentBot       │  │  Processing      │  │  Rank Computer        │     │
│  │   (Crawler)      │  │  Pipeline        │  │  (Background AVERT)   │     │
│  │   ×10 replicas   │  │  ×5 replicas     │  │  ×3 replicas          │     │
│  │                  │  │                  │  │                       │     │
│  │  ┌────────────┐  │  │  ┌────────────┐  │  │  ┌─────────────────┐ │     │
│  │  │ Frontier   │  │  │  │ Card       │  │  │  │ AVERT Score     │ │     │
│  │  │ Manager    │  │  │  │ Parser     │  │  │  │ Calculator      │ │     │
│  │  ├────────────┤  │  │  ├────────────┤  │  │  ├─────────────────┤ │     │
│  │  │ Fetcher    │  │  │  │ Dedup      │  │  │  │ Trust Graph     │ │     │
│  │  │ Pool       │  │  │  │ Engine     │  │  │  │ Walker          │ │     │
│  │  ├────────────┤  │  │  ├────────────┤  │  │  ├─────────────────┤ │     │
│  │  │ Rate       │  │  │  │ Normalizer │  │  │  │ Feature         │ │     │
│  │  │ Controller │  │  │  │            │  │  │  │ Extractor       │ │     │
│  │  └────────────┘  │  │  └────────────┘  │  │  └─────────────────┘ │     │
│  └────────┬─────────┘  └────────┬─────────┘  └───────────┬─────────┘     │
│           │                     │                         │               │
│           ▼                     ▼                         ▼               │
│  ┌─────────────┐  ┌──────────────────┐  ┌──────────────────────────┐     │
│  │ Redis       │  │   PostgreSQL     │  │    PostgreSQL             │     │
│  │ Frontier    │  │   (1 primary +   │  │    (reads from replica)   │     │
│  │ (Cluster)   │  │    2 replicas)   │  │                          │     │
│  └─────────────┘  └──────────────────┘  └──────────────────────────┘     │
│                                                                           │
│  ┌──────────────────┐  ┌──────────────────┐  ┌───────────────────────┐   │
│  │  Liveness Prober │  │  Embedding       │  │  Index Sync           │   │
│  │  ×5 replicas     │  │  Generator       │  │  ×3 replicas          │   │
│  │                  │  │  ×3 replicas     │  │                       │   │
│  │  Health checks   │  │  ONNX inference  │  │  PG → Tantivy        │   │
│  │  every 6 hours   │  │  batch pipeline  │  │  PG → Qdrant         │   │
│  └──────────────────┘  └──────────────────┘  └───────────────────────┘   │
│                                                                           │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                          ANALYTICS / OBSERVABILITY                          │
│                                                                             │
│  ┌──────────────────┐  ┌──────────────────┐  ┌───────────────────────┐     │
│  │   ClickHouse     │  │   Prometheus     │  │   Grafana             │     │
│  │   (×3 shards)    │  │   (×2 HA)       │  │   (×2 HA)            │     │
│  │                  │  │                  │  │                       │     │
│  │  Query logs      │  │  Metrics scrape  │  │  Dashboards           │     │
│  │  Crawl telemetry │  │  15s interval    │  │  Alerting config      │     │
│  │  Outcome events  │  │  30-day retain   │  │                       │     │
│  └──────────────────┘  └──────────────────┘  └───────────────────────┘     │
│                                                                             │
│  ┌──────────────────┐  ┌──────────────────┐  ┌───────────────────────┐     │
│  │   Vector         │  │   Jaeger         │  │   Alertmanager        │     │
│  │   (DaemonSet)    │  │   (×1)           │  │   (×2 HA)            │     │
│  │                  │  │                  │  │                       │     │
│  │  Log collection  │  │  Trace storage   │  │  PagerDuty + Slack   │     │
│  │  → ClickHouse    │  │  Sampling: 1%    │  │  routing              │     │
│  └──────────────────┘  └──────────────────┘  └───────────────────────┘     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                           EVENT BUS                                         │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────┐       │
│  │                    Kafka / Redpanda (×3 brokers)                  │       │
│  │                                                                  │       │
│  │  Topics:                                                         │       │
│  │    agent.discovered  │  agent.validated   │  agent.indexed       │       │
│  │    agent.scored      │  agent.liveness    │  query.executed      │       │
│  │    connect.initiated │  connect.completed │  outcome.reported    │       │
│  │    trust.updated     │  anomaly.detected  │  index.sync.request  │       │
│  │                                                                  │       │
│  │  Partitions: 12 per topic (keyed by agent_id for ordering)       │       │
│  │  Replication Factor: 3                                           │       │
│  │  Retention: 7 days                                               │       │
│  └──────────────────────────────────────────────────────────────────┘       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 20.2 Scaling Strategy

#### Query Path Scaling

| Scaling Trigger | Action | Mechanism | Limits |
|----------------|--------|-----------|--------|
| P99 search latency > 80ms sustained 5min | Scale out Search Engine replicas | HPA on custom metric `agentrank_query_latency_seconds{quantile="0.99"}` | Min 3, Max 20 replicas |
| QPS > 800 per replica sustained 2min | Scale out API Gateway replicas | HPA on `agentrank_api_requests_per_second` | Min 3, Max 30 replicas |
| Cache hit ratio < 30% sustained 15min | Increase Redis cache memory | Vertical scaling alert → operator action | Max 32GB per node |
| Qdrant recall < 95% | Add Qdrant replicas or re-shard | Manual operation with rolling restart | Max 12 shards |
| Tantivy segment count > 50 | Trigger forced merge | CronJob: `index-sync --force-merge` | Scheduled during low-traffic windows |

#### Crawl Path Scaling

| Scaling Trigger | Action | Mechanism | Limits |
|----------------|--------|-----------|--------|
| Frontier size > 500K URLs | Scale out AgentBot replicas | HPA on Redis frontier gauge | Min 5, Max 50 replicas |
| Processing pipeline lag > 10min | Scale out Processing Pipeline | HPA on Kafka consumer lag | Min 3, Max 20 replicas |
| Embedding batch queue > 1000 | Scale out Embedding Generator | HPA on queue depth metric | Min 1, Max 10 replicas (GPU-bound) |
| Liveness check backlog > 10K agents | Scale out Liveness Prober | HPA on backlog gauge | Min 3, Max 20 replicas |

#### Storage Scaling

| Store | Scaling Strategy | Growth Rate | Scaling Ceiling |
|-------|-----------------|-------------|-----------------|
| **PostgreSQL** | Vertical first (up to 96 vCPU, 768GB RAM). Read replicas for query offload. Table partitioning by `created_at` quarter. | ~50GB/year at 10M agents | 2TB before considering Citus or sharding |
| **Qdrant** | Horizontal sharding by collection. Each shard handles ~3M vectors. | ~10GB/year per million agents (quantized) | 100M vectors with 30 shards |
| **Tantivy** | Segment-level parallelism. Multiple index directories with query fan-out. | ~500MB/year per million agents | 50M documents per index before splitting |
| **ClickHouse** | `ReplicatedMergeTree` with automatic part merging. Add shards for write throughput. | ~100GB/year of analytics data | Petabyte-scale with distributed tables |
| **Redis** | Cluster mode with automatic resharding. Eviction policy: `allkeys-lru` for cache, `noeviction` for frontier. | ~2GB/year frontier growth | 256GB cluster maximum |
| **S3/R2** | Unlimited. Lifecycle rules archive old snapshots to cold storage after 90 days. | ~200GB/year of raw card snapshots | Effectively unlimited |

### 20.3 Infrastructure Cost Estimate

#### Phase 1: 1M Agents Indexed, 1K QPS Peak

| Component | Spec | Replicas | Monthly Cost |
|-----------|------|----------|-------------|
| **Kubernetes Control Plane** | GKE Autopilot or EKS managed | 1 cluster | $300 |
| **AgentBot (Crawler)** | 2 vCPU, 4GB RAM | 10 | $400 |
| **Processing Pipeline** | 2 vCPU, 4GB RAM | 5 | $200 |
| **Search Engine** | 4 vCPU, 8GB RAM | 5 | $500 |
| **API Gateway** | 2 vCPU, 2GB RAM | 5 | $150 |
| **Connection Broker** | 2 vCPU, 4GB RAM | 3 | $120 |
| **Rank Computer** | 4 vCPU, 8GB RAM | 3 | $300 |
| **Liveness Prober** | 1 vCPU, 2GB RAM | 5 | $100 |
| **Embedding Generator** | 4 vCPU, 16GB RAM (or GPU) | 3 | $350 |
| **Index Sync** | 2 vCPU, 4GB RAM | 3 | $120 |
| **Web UI (Next.js)** | 1 vCPU, 1GB RAM | 3 | $60 |
| **A2A Endpoint** | 1 vCPU, 2GB RAM | 5 | $100 |
| **PostgreSQL** | 8 vCPU, 32GB RAM, 500GB SSD | 1 primary + 2 replicas | $600 |
| **Qdrant** | 4 vCPU, 32GB RAM, 100GB SSD | 3 | $500 |
| **Redis (Cache + Frontier)** | 4 vCPU, 16GB RAM | 6 (2 clusters × 3) | $300 |
| **ClickHouse** | 4 vCPU, 16GB RAM, 500GB SSD | 3 | $350 |
| **Kafka / Redpanda** | 2 vCPU, 8GB RAM, 100GB SSD | 3 | $200 |
| **Prometheus + Grafana** | 2 vCPU, 8GB RAM, 100GB SSD | 2 + 2 | $150 |
| **Jaeger** | 2 vCPU, 4GB RAM | 1 | $50 |
| **S3/R2 Storage** | ~500GB | N/A | $15 |
| **Cloudflare** | Pro plan | N/A | $20 |
| **DNS / Certificates** | Let's Encrypt + Cloudflare | N/A | $0 |
| **Bandwidth** | ~5TB/month egress | N/A | $100 |
| | | **Total** | **~$4,485/month** |

#### Projected Cost at Scale

| Scale | Agents Indexed | Peak QPS | Monthly Cost | Cost/Million Agents |
|-------|---------------|----------|-------------|-------------------|
| Phase 0 | 100K | 100 | $2,500 | $25,000 |
| Phase 1 | 1M | 1K | $4,500 | $4,500 |
| Phase 2 | 5M | 10K | $15,000 | $3,000 |
| Phase 3 | 10M | 50K | $25,000 | $2,500 |
| Phase 4 | 50M | 200K | $80,000 | $1,600 |

The cost-per-million-agents curve is strongly sublinear due to: (1) storage compression improving with volume, (2) cache hit rates improving with query volume, (3) amortization of fixed infrastructure costs (Kafka, monitoring, control plane), and (4) Rust's efficiency meaning we scale replicas, not instance sizes.

---

## 21. Security Architecture

### 21.1 Threat Model

AgentRank occupies a unique threat landscape: it is a search engine that ingests untrusted metadata from the open internet, serves queries to both humans and autonomous agents, and brokers connections between parties that may have never interacted before.

| Threat ID | Threat | Attack Vector | Impact | Likelihood | Severity | Mitigation Reference |
|-----------|--------|--------------|--------|-----------|----------|---------------------|
| **T1** | **Malicious Agent Cards** | Attacker hosts an agent card containing XSS payloads, SQL injection strings, or oversized fields designed to crash the parser | Parser crash, XSS in web UI, database corruption | High | Critical | §21.3 Input validation, CSP headers, parameterized queries |
| **T2** | **Crawler Abuse (SSRF)** | Attacker's agent card contains URLs pointing to internal services (169.254.169.254, localhost, private IP ranges) | Cloud metadata exposure, internal service access, credential theft | High | Critical | §21.3 SSRF prevention, IP blocklist, DNS rebinding protection |
| **T3** | **Ranking Manipulation** | Attacker creates networks of fake agents that reference each other to inflate trust scores and ranking position | Spam agents ranked above legitimate ones, user trust erosion | Medium | High | §22 Anti-gaming defenses, graph analysis, manual review |
| **T4** | **API Abuse / DDoS** | Attacker floods search API, crawl submission API, or connection broker with high-volume requests | Service degradation, cost inflation, legitimate user impact | High | High | §21.2 Rate limiting tiers, Cloudflare WAF, circuit breakers |
| **T5** | **Data Poisoning** | Attacker submits fake outcome telemetry to manipulate ranking signals | Learning-to-rank model corrupted, ranking quality degradation | Medium | High | §14.5 Outcome trust scoring, anomaly detection, slow adoption |
| **T6** | **Supply Chain Attack** | Compromised Rust crate or Docker base image introduces malicious code | Full system compromise, data exfiltration | Low | Critical | §21.5 Dependency auditing, minimal base images, SBOM |
| **T7** | **Agent Impersonation** | Attacker registers an agent card claiming to be a well-known service (e.g., "OpenAI GPT-4 Agent") | User confusion, phishing, reputation damage to impersonated party | Medium | Medium | §23 Domain verification, trademark detection |
| **T8** | **Connection Broker Hijack** | Attacker intercepts or manipulates brokered connections between agents | Man-in-the-middle, data theft, credential interception | Low | Critical | §15 mTLS for brokered connections, session tokens, timeout |
| **T9** | **Insider Threat** | Malicious or compromised employee accesses production data or modifies ranking | Data breach, ranking manipulation, trust destruction | Low | Critical | §21.4 RBAC, audit logging, break-glass procedures |
| **T10** | **Privacy Violation** | Query logs or outcome data expose sensitive user behavior patterns | Regulatory penalty (GDPR/CCPA), user trust erosion | Medium | High | §22 Data classification, pseudonymization, retention limits |

### 21.2 API Access Tiers

All API access is controlled through a tiered system that balances openness with protection.

| Tier | Authentication | Rate Limit | Capabilities | SLA | Cost |
|------|---------------|-----------|--------------|-----|------|
| **Anonymous** | None (IP-based tracking) | 100 requests/hour, burst 10/min | Search (web UI only), agent detail view, no bulk export | Best effort | Free |
| **Free API** | API key (issued on signup) | 1,000 requests/hour, burst 50/min | Search API, agent detail API, basic filters | 99.5% uptime | Free |
| **Pro API** | OAuth 2.0 bearer token | 50,000 requests/hour, burst 500/min | Full search API, connect API, outcome API, webhooks, bulk export (100 agents/request) | 99.9% uptime, P99 < 200ms | $49/month |
| **Enterprise** | mTLS client certificate + OAuth 2.0 | Custom (negotiated per contract) | All Pro features + private registry, federation, tenant isolation, dedicated support, custom SLA | 99.95% uptime, P99 < 100ms | Custom pricing |

#### Authentication Flow

```
Anonymous:
  Client → Cloudflare (IP tracking) → API Gateway (anonymous tier) → Backend

Free API:
  Client → API Gateway → Extract API key from X-API-Key header
       → Redis lookup: api_keys:{key} → {tier: "free", user_id, rate_limit}
       → governor rate check → Backend

Pro API:
  Client → API Gateway → Extract Bearer token from Authorization header
       → JWT validation (RS256, issuer check, expiry check)
       → Redis lookup: sessions:{jti} → {tier: "pro", user_id, org_id}
       → governor rate check → Backend

Enterprise:
  Client → mTLS termination at ingress (client cert validated against CA)
       → API Gateway → Extract client cert CN → tenant lookup
       → OAuth 2.0 token validation (tenant-specific JWKS)
       → governor rate check (tenant-specific limits) → Backend
```

#### API Key Format

```
ar_live_k7x9m2p4q8w1e5r3t6y0u  (production)
ar_test_a1b2c3d4e5f6g7h8i9j0k  (sandbox)

Prefix: ar_ (AgentRank)
Environment: live_ | test_
Random: 25 alphanumeric characters (150 bits of entropy)
```

### 21.3 Crawler Security

The crawler (AgentBot) is AgentRank's most exposed attack surface. It fetches content from arbitrary URLs on the open internet. Every security control here prevents a class of attack.

#### SSRF Prevention

```rust
/// Validates a URL is safe to fetch before any network request.
/// This runs BEFORE DNS resolution to prevent DNS rebinding attacks.
fn validate_fetch_target(url: &Url) -> Result<(), CrawlSecurityError> {
    // 1. Scheme whitelist
    match url.scheme() {
        "https" => {} // preferred
        "http" => {}  // allowed with warning
        _ => return Err(CrawlSecurityError::DisallowedScheme(url.scheme().to_string())),
    }

    // 2. Host validation
    let host = url.host_str()
        .ok_or(CrawlSecurityError::MissingHost)?;

    // Block IP-based URLs entirely (prevents 169.254.x.x, 10.x.x.x, etc.)
    if host.parse::<IpAddr>().is_ok() {
        return Err(CrawlSecurityError::IpBasedUrl);
    }

    // Block known dangerous hostnames
    const BLOCKED_HOSTS: &[&str] = &[
        "localhost", "metadata.google.internal",
        "instance-data", "metadata.azure.com",
    ];
    if BLOCKED_HOSTS.iter().any(|&blocked| host == blocked) {
        return Err(CrawlSecurityError::BlockedHost(host.to_string()));
    }

    // 3. Port whitelist
    match url.port() {
        None | Some(80) | Some(443) | Some(8080) | Some(8443) => {}
        Some(port) => return Err(CrawlSecurityError::DisallowedPort(port)),
    }

    Ok(())
}

/// Post-DNS validation: check resolved IP is not private/reserved.
/// This catches DNS rebinding attacks where a public hostname
/// resolves to a private IP.
fn validate_resolved_ip(ip: IpAddr) -> Result<(), CrawlSecurityError> {
    if ip.is_loopback()
        || ip.is_private()       // 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
        || ip.is_link_local()    // 169.254.0.0/16
        || ip.is_unspecified()   // 0.0.0.0
        || is_cloud_metadata(ip) // 169.254.169.254
    {
        return Err(CrawlSecurityError::PrivateIp(ip));
    }
    Ok(())
}
```

#### Fetch Safety Controls

| Control | Value | Rationale |
|---------|-------|-----------|
| **Maximum response body** | 1 MB | Agent cards should be < 50KB. Anything larger is suspicious or malformed. Prevents memory exhaustion from infinite streams. |
| **Connection timeout** | 10 seconds | Prevents slow-loris attacks where the server keeps the connection open indefinitely. |
| **Read timeout** | 30 seconds | Allows for slow but legitimate servers while preventing indefinite hangs. |
| **Maximum redirects** | 5 | Standard HTTP redirect chain depth. More than 5 redirects indicates misconfiguration or redirect loop. |
| **TLS minimum version** | TLS 1.2 | TLS 1.0 and 1.1 are deprecated (RFC 8996). Servers offering only deprecated TLS receive a trust penalty. |
| **Certificate validation** | Strict (no self-signed) | Self-signed certificates are not trusted by default. Exception: enterprise private registries with pre-configured CA bundles. |
| **User-Agent** | `AgentBot/1.0 (+https://agentrank.dev/bot)` | Identifies the crawler per RFC 9309. Allows server operators to recognize and control our crawler via robots.txt. |
| **Concurrent connections per domain** | 2 | Politeness limit. Never overwhelm a single server regardless of how many agent cards it hosts. |
| **Request rate per domain** | 1 request/second (default) | Respects `Crawl-delay` in robots.txt if present. Minimum enforced delay even if robots.txt specifies 0. |
| **DNS resolution** | DoH (DNS over HTTPS) via Cloudflare 1.1.1.1 | Prevents DNS poisoning attacks that could redirect the crawler to malicious servers. |
| **Cookie handling** | Disabled | The crawler does not maintain session state. Cookies are not sent or stored. |
| **JavaScript execution** | Disabled | The crawler does not execute JavaScript. If an agent card requires JS to render, it is not crawlable. Agent cards must be static JSON or HTML with embedded JSON-LD. |

### 21.4 Data Protection

#### Encryption at Rest

| Data Store | Encryption Method | Key Management |
|-----------|-------------------|----------------|
| PostgreSQL | AES-256-CBC via `pgcrypto` extension + volume-level encryption (dm-crypt) | AWS KMS / GCP Cloud KMS. Key rotation every 90 days. |
| Qdrant | Volume-level encryption (EBS/PD encryption) | Cloud provider KMS. |
| Redis | No at-rest encryption (data is ephemeral cache). TLS for in-transit. | N/A — cache data is not sensitive. Frontier data is URL strings only. |
| ClickHouse | Volume-level encryption | Cloud provider KMS. |
| S3/R2 | SSE-S3 (AES-256) by default. SSE-KMS for sensitive buckets. | AWS KMS / Cloudflare R2 managed keys. |
| Kafka | Volume-level encryption for broker storage. | Cloud provider KMS. |

#### Encryption in Transit

| Path | Protocol | Certificate |
|------|----------|-------------|
| Client → Cloudflare | TLS 1.3 (minimum 1.2) | Cloudflare-managed edge certificate |
| Cloudflare → Ingress | TLS 1.3 (Full Strict mode) | Let's Encrypt origin certificate |
| Service → Service (intra-cluster) | mTLS via Istio/Linkerd service mesh | Istio CA auto-rotated certificates (24-hour TTL) |
| Service → PostgreSQL | TLS 1.3 with `sslmode=verify-full` | PostgreSQL server certificate validated against known CA |
| Service → Redis | TLS 1.2+ with `stunnel` or native Redis TLS | Self-signed CA within cluster trust boundary |
| Service → Qdrant | gRPC with TLS | Service mesh mTLS |
| Service → Kafka | SASL_SSL (SASL/SCRAM-SHA-512 + TLS) | Kafka broker certificates from internal CA |

#### User Credential Protection

| Credential Type | Storage | Protection |
|----------------|---------|-----------|
| User passwords | PostgreSQL | bcrypt with cost factor 12 (minimum 100ms compute per hash) |
| Two-factor authentication | PostgreSQL | TOTP secrets encrypted with AES-256-GCM, user-specific salt |
| API keys | PostgreSQL + Redis | SHA-256 hash stored in database. Full key shown only once at creation. Redis stores hash → metadata mapping. |
| OAuth tokens | Redis (short-lived) | JWT with RS256 signature. 1-hour expiry. Refresh tokens: 30-day expiry with rotation on use. |
| Session tokens | Redis | Cryptographically random, 256-bit. HttpOnly, Secure, SameSite=Strict cookies. 24-hour TTL. |

#### Query Privacy

| Principle | Implementation |
|-----------|---------------|
| **Query pseudonymization** | Raw query strings stored in ClickHouse with `query_id` (UUIDv7). User identity linked via separate `query_sessions` table. Pseudonymization key rotated monthly. Queries older than 90 days are fully anonymized (user link deleted). |
| **No query sharing** | Individual user queries are never shared with agent providers. Aggregate query analytics (e.g., "top 10 skill categories searched") are available in the Agent Search Console, but never with user identity. |
| **Right to deletion** | GDPR Article 17 compliance: user deletion request triggers cascade deletion of all query logs, outcome reports, and session data within 72 hours. Implemented as a Kafka event that fans out to all data stores. |
| **IP address handling** | IP addresses are truncated to /24 (IPv4) or /48 (IPv6) before storage. Full IP is held in memory for rate limiting only and never persisted. |

### 21.5 Supply Chain Security

| Control | Implementation |
|---------|---------------|
| **Dependency auditing** | `cargo audit` runs in CI on every PR. Known vulnerabilities block merge. `cargo deny` enforces license compliance (no GPL in core services). |
| **SBOM generation** | `cargo cyclonedx` generates CycloneDX SBOM on every release. Published alongside container images. |
| **Base image** | `gcr.io/distroless/cc-debian12` for Rust binaries. No shell, no package manager, no unnecessary utilities. Attack surface: ~20 files. |
| **Image signing** | Container images signed with `cosign` (Sigstore). Kubernetes admission controller (Kyverno) rejects unsigned images. |
| **Build reproducibility** | Nix flake for reproducible builds. Given the same source commit, any engineer produces the same binary (bit-for-bit). |
| **Secrets scanning** | `gitleaks` pre-commit hook and CI check. Blocks commits containing API keys, passwords, or private keys. |

---

## 22. Anti-Gaming & Spam Defense

AgentRank's value proposition depends on ranking quality. If spam agents can game their way to the top, the entire system is worthless. This section describes the multi-layered defense system that protects ranking integrity.

### 22.1 Spam Signals at Crawl Time

The first line of defense operates during crawl and ingestion. These signals are cheap to compute and catch the most obvious spam.

| Signal | Detection Method | Threshold | Weight |
|--------|-----------------|-----------|--------|
| **Keyword stuffing** | TF-IDF analysis of `description` field. Flag if any single term has TF > 0.15 in a document < 500 words. Also flag if skill list contains > 5 near-duplicate skills (edit distance < 3). | TF > 0.15 OR > 5 near-duplicate skills | 0.25 |
| **Duplicate content** | SimHash comparison against all indexed agents. Flag if SimHash Hamming distance < 3 (indicating > 95% textual similarity to an existing agent). | Hamming distance < 3 | 0.30 |
| **Excessive skills** | Count of declared skills/capabilities. Legitimate agents typically declare 5-20 skills. More than 50 suggests padding for keyword matching. | > 50 skills | 0.15 |
| **Oversized description** | Character count of `description` field. Agent cards with descriptions > 10,000 characters are almost always keyword-stuffed spam or auto-generated filler. | > 10K characters | 0.10 |
| **Known spam IP** | Source IP checked against threat intelligence feeds (Spamhaus, AbuseIPDB) and internal blacklist of previously-confirmed spam sources. | Listed in any feed | 0.35 |
| **Fresh domain** | Domain registration date via WHOIS lookup. Domains registered < 30 days ago receive a penalty. Not conclusive alone (legitimate new services exist) but correlated with spam campaigns. | Domain age < 30 days | 0.10 |
| **Missing TLS** | Agent card served over HTTP instead of HTTPS. Not necessarily spam, but strongly correlated with low-quality or test deployments. | No TLS | 0.08 |
| **Broken endpoint** | Declared endpoint URL returns non-2xx status code during crawl validation. | Non-2xx response | 0.20 |
| **Excessive claims** | Agent card claims capabilities that are mutually exclusive or implausible (e.g., "fastest" AND "cheapest" AND "most accurate" in every category). | Sentiment analysis score > 0.9 for promotional language | 0.12 |
| **Auto-generated names** | Agent name matches patterns of auto-generated text (UUIDs, random strings, sequential numbering like "Agent-001" through "Agent-999"). | Regex pattern match | 0.15 |

#### Composite Spam Score

```
spam_score = Σ(signal_weight × signal_detected)

Actions:
  spam_score < 0.4  →  No action (clean)
  0.4 ≤ spam_score < 0.7  →  DEMOTED: ranked lower, warning flag in Search Console
  0.7 ≤ spam_score < 0.9  →  EXCLUDED: not shown in search results, visible in Console only
  spam_score ≥ 0.9  →  DELISTED: removed from index entirely, provider notified
```

#### Appeal Process

| Action | Appeal Available | Process | SLA |
|--------|-----------------|---------|-----|
| Demoted | Yes | Automated: fix the flagged signal, request re-evaluation via Search Console | 24 hours |
| Excluded | Yes | Automated re-evaluation + manual queue if auto-check fails | 72 hours |
| Delisted | Yes | Manual review only. Requires human operator approval to re-list. | 5 business days |

### 22.2 Score Manipulation Detection

Sophisticated attackers will attempt to manipulate the signals that feed into AgentRank's composite score.

#### Link Farm Detection

Agent cards can reference other agents (via delegation chains, provider relationships, or cross-referencing). This creates a graph that can be analyzed for manipulation patterns.

| Pattern | Detection | Response |
|---------|-----------|----------|
| **Circular delegation chains** | Depth-first traversal of delegation graph. Flag if cycle length ≤ 5 (A delegates to B delegates to C delegates to A). | All agents in cycle receive trust penalty: `trust_score *= 0.3`. Manual review queued. |
| **Dense self-referencing clusters** | Community detection on reference graph (Louvain algorithm). Flag clusters where internal edge density > 10× external edge density. | Entire cluster flagged for manual review. Trust scores capped at 0.5 for all members until cleared. |
| **Star topology** | Single agent referenced by > 50 other agents where those other agents have no independent references. | Central agent flagged. Reference count from flagged sources excluded from trust computation. |
| **Temporal burst** | > 10 new agents referencing the same target within 24 hours, where those agents were created within the same 24-hour period. | Burst references quarantined (not counted in trust score) for 30 days. Released if agents demonstrate independent activity. |
| **Cross-domain collusion** | Multiple domains resolving to same IP or hosting the same agent card with different names. | All domains except the first-registered are flagged as duplicates. Trust score shared (not multiplied) across the set. |

#### CTR Manipulation Detection

If outcome telemetry is used as a ranking signal (Phase 2+), attackers may generate fake clicks or connections.

| Pattern | Detection | Response |
|---------|-----------|----------|
| **Same-IP click flooding** | > 10 clicks from same IP to same agent within 1 hour. | Excess clicks discarded. IP flagged for elevated monitoring. |
| **Low diversity threshold** | Agent receives > 100 clicks but from < 10 unique IP /24 blocks. | CTR signal weight reduced to 0.1 for this agent (vs normal 1.0). |
| **Bot-pattern timing** | Click intervals are uniformly distributed (real human behavior is Poisson-distributed). Kolmogorov-Smirnov test for uniformity. | If KS test p-value < 0.01, all clicks from matching source are discarded. |
| **Connection-without-use** | Agent receives many "connect" events but near-zero outcome reports (connection initiated but never used). | Connection count signal discounted. Agent flagged for liveness re-verification. |
| **Outcome stuffing** | Agent receives > 100 positive outcome reports within 1 hour from source IPs that have never submitted outcome reports before. | Outcome reports quarantined. Not counted in ranking until source trust is established (minimum 10 previous legitimate outcome reports from those sources). |

### 22.3 Quality Enforcement

#### Manual Review Triggers

| Trigger | Condition | Queue |
|---------|-----------|-------|
| Spam score > 0.7 on a previously clean agent | Score increased by > 0.3 in single crawl cycle | Urgent review (24h SLA) |
| Trust score dropped > 50% in single computation cycle | Indicates either agent degradation or detection of previously-missed manipulation | Standard review (72h SLA) |
| User report (flagged in UI or API) | Any user flags an agent as spam, impersonation, or malicious | Standard review (72h SLA) |
| Automated anomaly detection (§23.4) | Behavioral anomaly score > 2 standard deviations | Standard review (72h SLA) |
| New agent claiming well-known brand name | Name similarity > 0.9 to agents in protected brand list | Urgent review (24h SLA) |

#### Quality Rater Program

| Aspect | Detail |
|--------|--------|
| **Purpose** | Human evaluation of ranking quality to calibrate automated systems and catch cases that algorithms miss. |
| **Scale** | 1,000 query-result pairs evaluated per week. |
| **Raters** | 5-10 trained raters (contract or internal). Domain expertise in AI agent ecosystems. |
| **Evaluation criteria** | Relevance (1-5 scale), quality (1-5), trustworthiness (1-5), freshness (binary: alive or dead). |
| **Judgment format** | Query → Top 10 results → Rater assigns scores → Inter-rater reliability measured via Krippendorff's alpha (target > 0.7). |
| **Calibration** | Monthly calibration sessions where raters discuss edge cases and align on scoring guidelines. |
| **Integration** | Rater judgments feed into LTR training data (§12.7). Also used to compute NDCG@10 for ranking evaluation (§25.3). |
| **Anti-bias** | Results shown in random order (not ranked order). Rater does not know the current ranking position. Double-blind where possible. |

---

## 23. Trust, Verification & Domain Ownership

### 23.1 Trust Layers

Trust in AgentRank is not binary. It is a multi-dimensional assessment built from four independent layers, each providing evidence for a different aspect of trustworthiness.

| Layer | What It Proves | Evidence Sources | Contributes To |
|-------|---------------|-----------------|----------------|
| **1. Identity Trust** | "This agent card was published by who it claims" | Domain verification (DNS TXT, well-known file), TLS certificate analysis (EV > OV > DV), WHOIS data, DMARC/SPF for email-based identity | `identity_score` in AVERT framework |
| **2. Artifact Trust** | "This agent card is well-formed, complete, and consistent" | Schema validation pass rate, field completeness, version history consistency, cross-field coherence (description matches skills), metadata richness | `artifact_quality` in AVERT framework |
| **3. Operational Trust** | "This agent actually works as described" | Liveness probe results (uptime, latency), benchmark results (if enrolled), response format correctness, error rate from outcome telemetry | `availability_score` + `evidence_depth` in AVERT framework |
| **4. Ecosystem Trust** | "Other trusted entities vouch for this agent" | Delegation chains from verified providers, organic references from diverse sources, positive outcome telemetry from diverse callers, ecosystem partner endorsements | `relationship_strength` in AVERT framework |

#### Trust Tier Assignment

The four trust layers combine into a single trust tier:

| Tier | Requirements | Badge | Benefits |
|------|-------------|-------|----------|
| **Unverified** | Agent discovered but no verification completed | None | Indexed and searchable, but ranking is penalized. Not eligible for "Verified" filter. |
| **Basic** | Schema validation passed + endpoint responds to liveness probe | Gray checkmark | No ranking penalty. Visible in all search results. |
| **Verified** | Basic + domain ownership verified (DNS TXT or well-known file) | Blue checkmark | Ranking boost (1.15× multiplier). Eligible for "Verified" filter. Displayed in Agent Search Console. |
| **Trusted** | Verified + operational history > 30 days + uptime > 95% + positive outcome ratio > 0.8 | Gold checkmark | Ranking boost (1.30× multiplier). Featured in "Trusted Agents" section. Priority crawl schedule. |
| **Certified** | Trusted + passed capability benchmarks + independent audit (Phase 3+) | Platinum badge | Maximum ranking boost (1.50× multiplier). Eligible for promoted placement. Enterprise customer recommended list. |

### 23.2 Domain Verification Protocol

Domain verification proves that the entity hosting the agent card controls the domain where it is published. This is the foundation of identity trust.

#### Method 1: DNS TXT Record

```
# Provider adds a TXT record to their domain's DNS:
_agentrank-verify.example.com  TXT  "agentrank-site-verification=ar_v1_a1b2c3d4e5f6g7h8"

# Verification flow:
1. Provider requests verification in Agent Search Console
2. AgentRank generates unique token: ar_v1_{random_32_hex}
3. Provider creates DNS TXT record with token
4. AgentRank queries DNS (via DoH) for TXT records at _agentrank-verify.{domain}
5. If token matches → domain verified
6. Verification cached for 90 days, then re-checked automatically
```

**Advantages:** No web server changes needed. Works for domains that host agents but not web content.

**Disadvantages:** DNS propagation delay (up to 48 hours). Requires DNS management access.

#### Method 2: Well-Known File

```
# Provider hosts a JSON file at:
https://example.com/.well-known/agentrank-verify.json

# File contents:
{
  "verification_token": "ar_v1_a1b2c3d4e5f6g7h8",
  "domain": "example.com",
  "timestamp": "2026-03-23T10:00:00Z"
}

# Verification flow:
1. Provider requests verification in Agent Search Console
2. AgentRank generates unique token
3. Provider creates .well-known/agentrank-verify.json with token
4. AgentBot fetches the file (standard SSRF-safe fetch)
5. If token matches AND domain matches AND timestamp within 7 days → verified
6. File can be removed after verification; re-check uses DNS or periodic re-fetch
```

**Advantages:** Immediate verification (no DNS propagation delay). Familiar pattern (Google Search Console uses same approach).

**Disadvantages:** Requires web server access. File must be served over HTTPS.

#### Method 3: HTML Meta Tag

```html
<!-- Provider adds to the <head> of their root domain page: -->
<meta name="agentrank-site-verification" content="ar_v1_a1b2c3d4e5f6g7h8">
```

**Advantages:** Simple for web-savvy providers. No server configuration needed.

**Disadvantages:** Only works if the domain serves HTML (many agent providers serve only JSON APIs). Slower to verify (HTML parsing required).

#### Verification Priority

AgentRank attempts verification methods in this order: DNS TXT → Well-Known File → Meta Tag. The first successful method is recorded. Providers can verify with any method.

### 23.3 TLS Certificate Analysis

The type of TLS certificate used to serve an agent card provides a signal about the identity of the provider.

| Certificate Type | Validation Level | Identity Proof | Trust Score Contribution |
|-----------------|-----------------|----------------|--------------------------|
| **Extended Validation (EV)** | Organization identity verified by CA. Legal entity, physical address, incorporation documents checked. | Highest — proves a specific legal entity controls the domain. | `+0.15` to identity score |
| **Organization Validation (OV)** | Organization exists and controls the domain. Less rigorous than EV. | Medium — proves an organization (not just an individual) controls the domain. | `+0.08` to identity score |
| **Domain Validation (DV)** | Domain control only. Automated issuance (Let's Encrypt). No identity verification. | Lowest — proves domain control but nothing about who controls it. | `+0.02` to identity score (baseline) |
| **Self-Signed** | No CA validation. Anyone can create a self-signed certificate. | None — provides encryption but no identity assurance. | `-0.10` to identity score (penalty). Not accepted for public index. Enterprise private registries may allow with pre-configured CA. |

#### Certificate Transparency Log Integration

```
For every HTTPS-served agent card, AgentRank:
1. Extracts the certificate chain during TLS handshake
2. Validates the chain against known root CAs
3. Checks Certificate Transparency (CT) logs for the domain:
   - Monitors ct.googleapis.com/logs for new certificates
   - Flags if a new certificate is issued for a verified domain by a different CA
   - Alerts the domain owner via Agent Search Console if unexpected certificate detected
4. Records certificate metadata:
   - Issuer CA
   - Validation level (EV/OV/DV)
   - Expiration date
   - Subject Alternative Names (SANs)
   - CT log inclusion proof
```

### 23.4 Behavioral Anomaly Detection

Beyond static verification, AgentRank monitors agent behavior over time to detect suspicious changes that may indicate compromise, sale, or manipulation.

| Anomaly Type | Detection Logic | Threshold | Response |
|-------------|----------------|-----------|----------|
| **Rapid mutation** | Agent card changed > 5 times in 24 hours. Normal agents update weekly or less. | > 5 changes/day | Crawl frequency reduced to daily (prevent hot-polling). Changes logged but indexing delayed 24 hours. Alert in Search Console. |
| **Skill explosion** | Agent gained > 10 new skills in a single update. Suggests keyword padding or automated skill injection. | > 10 new skills per update | New skills quarantined (not indexed for search) for 7 days. Manual review if repeated. |
| **Description churning** | Agent description SimHash distance > 50% from previous version. The agent has fundamentally changed what it claims to do. | SimHash Hamming distance > 32 (of 64 bits) | Previous description retained in index until manual review confirms the change is legitimate. Trust score recalculated from scratch. |
| **Provider hopping** | Agent card URL changed domain (e.g., moved from example.com to suspicious.io). | Domain change detected | Trust score reset to 0. New domain must re-verify. Previous trust history preserved but not applied until re-verification. |
| **Clone detection** | New agent's description is > 90% similar (SimHash) to an existing high-trust agent. | SimHash distance < 3 from any agent with trust tier ≥ Verified | New agent flagged as potential impersonation. Not indexed until manual review. Original agent's owner notified. |
| **Zombie resurrection** | Agent was dead (failed liveness probes) for > 30 days, then suddenly responds. | Dead > 30 days, now alive | Liveness confirmed but trust score starts from 0.3 (not previous value). Must demonstrate 7 days of stable uptime to restore previous trust. |
| **Capability inflation** | Agent claims new capabilities that are radically different from original purpose (e.g., "email assistant" suddenly claims "financial trading"). | Embedding cosine distance > 0.7 from previous version | Flagged for manual review. Previous capability profile retained in index until review. |
| **Ownership transfer** | WHOIS domain ownership changed. Or DNS nameservers changed to a different provider. | Any change in WHOIS registrant or nameservers | Domain verification invalidated. Must re-verify. Trust tier drops to Unverified. |

---

## 24. Agent Search Console

### 24.1 Key Features

The Agent Search Console is a web-based dashboard for agent providers to monitor and improve their agents' presence in AgentRank. It is modeled on Google Search Console and serves the same strategic purpose: creating a feedback loop that incentivizes providers to improve their agent metadata quality.

| Feature | Description | Phase |
|---------|-------------|-------|
| **Domain Verification** | Verify domain ownership via DNS TXT, well-known file, or meta tag. Verified domains display a blue checkmark in search results. | 0 |
| **Card Inspector** | View the parsed, normalized version of your agent card as AgentRank sees it. Highlights parsing errors, missing fields, schema warnings, and normalization changes. Side-by-side comparison: raw card vs. indexed card. | 0 |
| **Crawl History** | Timeline of every crawl attempt for your agent URLs. Shows HTTP status code, response time, content hash, and whether the crawl resulted in an index update. Identifies crawl errors with suggested fixes. | 0 |
| **Index Status** | Current index state of all your agents. Statuses: Indexed, Discovered (not yet indexed), Excluded (spam/quality), Errored (parse failure). Filterable by status. | 0 |
| **Discoverability Score** | Composite score (0-100) showing how well your agent is optimized for discovery. Breaks down into sub-scores: metadata completeness, schema compliance, liveness, trust tier, SEO-equivalent metrics. Actionable suggestions for improvement. | 0 |
| **Schema Warnings** | Real-time validation of your agent card against the A2A spec and AgentRank's extended schema requirements. Lists specific fields that are missing, malformed, or could be improved. Severity levels: Error (blocks indexing), Warning (reduces ranking), Info (improvement opportunity). | 0 |
| **Trust Dashboard** | Current trust tier and detailed breakdown of how it was computed. Shows which verification methods are complete, operational history, and what is needed to reach the next tier. | 1 |
| **Search Analytics** | Queries that led users to your agents, click-through rates, impression counts, average position. Time-series graphs with date range selection. Filterable by query, agent, and device type. | 1 |
| **Benchmark Results** | If enrolled in benchmarking, shows conformance test results, capability scores, reliability metrics, and security assessment. Comparison to category median. | 1 |
| **Competitor Insights** | Anonymized comparison of your agents' discoverability scores against others in the same skill categories. Percentile rankings. "You are in the top 15% of translation agents" type insights. | 2 |
| **API Access Management** | Create, rotate, and revoke API keys. View usage statistics per key. Set per-key rate limits. | 1 |
| **Webhook Configuration** | Configure webhooks for events: crawl completed, index updated, trust tier changed, search impression milestone reached. | 1 |
| **Liveness History** | Timeline of liveness probe results. Uptime percentage calculation. SLA compliance tracking for agents that advertise uptime guarantees. | 1 |
| **Incident Timeline** | Historical record of trust score changes, spam score changes, manual review decisions, and appeal outcomes. Full audit trail. | 2 |

#### Discoverability Score Breakdown

```
Discoverability Score (0-100) = 
  Metadata Completeness (0-25)
  + Schema Compliance (0-20)
  + Endpoint Liveness (0-20)
  + Trust Verification (0-15)
  + Description Quality (0-10)
  + Skill Specificity (0-10)

Where:
  Metadata Completeness:
    25 points total, distributed across:
    - name: 3 pts
    - description: 5 pts (0 if missing, 3 if < 50 chars, 5 if 50-500 chars)
    - skills (at least 3): 5 pts
    - provider.name: 3 pts
    - provider.url: 2 pts
    - version: 2 pts
    - endpoint URL: 5 pts

  Schema Compliance:
    20 points: 20 if zero errors, 15 if warnings only, 0 if errors

  Endpoint Liveness:
    20 points: 20 if up with P95 < 1s, 15 if up with P95 < 5s, 10 if up, 0 if down

  Trust Verification:
    15 points: 0 (unverified), 5 (basic), 10 (verified), 15 (trusted/certified)

  Description Quality:
    10 points: NLP assessment of description clarity, specificity, and information density.
    0 for generic/vague descriptions. 10 for specific, well-structured descriptions.

  Skill Specificity:
    10 points: Assessed by comparing skill names to our skill taxonomy.
    10 for specific, standardized skill names.
    5 for reasonable but non-standard names.
    0 for vague or stuffed skill lists.
```

### 24.2 Strategic Importance

The Agent Search Console is not a nice-to-have. It is a strategic weapon that creates an ecosystem flywheel:

```
┌───────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  Provider publishes    AgentRank crawls     Console shows             │
│  agent card        ──▶ and indexes      ──▶ discoverability     ──┐  │
│                                              score & suggestions   │  │
│                                                                    │  │
│  ┌──────────────────────────────────────────────────────────────┐  │  │
│  │                                                              │  │  │
│  │  Provider improves     Agent ranks        More users         │  │  │
│  │  card based on      ◀── higher in      ◀── find agents    ◀──┘  │
│  │  Console feedback       search results     via search           │
│  │                                                                 │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  This cycle creates "Agent SEO" — a self-reinforcing loop where       │
│  providers compete to improve their agents' discoverability,          │
│  which improves the overall quality of the AgentRank index,           │
│  which attracts more searchers, which attracts more providers.        │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
```

The Google Search Console created the SEO industry and locked webmasters into a dependency on Google's index. The Agent Search Console aims to create the "Agent SEO" industry and achieve the same lock-in for agent providers.

---

## 25. Benchmarking & Evaluation

### 25.1 Benchmark Types

AgentRank operates four categories of benchmarks. Some are applied to every agent automatically; others are opt-in.

| Category | What It Tests | Methodology | Scope | Phase |
|----------|--------------|-------------|-------|-------|
| **Conformance** | Does the agent correctly implement the A2A protocol? | Automated test suite that sends standardized A2A requests and validates response format, status codes, error handling, and JSON schema compliance. 50 test cases covering happy path, error conditions, and edge cases. | All indexed agents (mandatory) | 0 |
| **Capability** | Does the agent actually perform the skills it claims? | Skill-specific test harness. For each declared skill, send a representative task and evaluate the response against expected output using automated rubrics (exact match, regex, or LLM-as-judge). | Opt-in. Provider must consent to capability testing. | 1 |
| **Reliability** | Is the agent consistently available and performant? | 30-day rolling window of liveness probe results. Measures uptime percentage, P50/P95/P99 response latency, error rate, and consistency (standard deviation of latency). | All indexed agents (passive measurement) | 0 |
| **Security** | Does the agent handle untrusted input safely? | Automated security scanner sends malformed inputs, oversized payloads, injection attempts, and boundary condition tests. Evaluates response behavior (no crash, proper error codes, no information leakage). | Opt-in. Requires provider consent and sandboxed execution. | 2 |

#### Conformance Test Suite (Detail)

| Test ID | Test Name | Input | Expected Behavior |
|---------|-----------|-------|-------------------|
| CONF-001 | Basic agent card retrieval | `GET /.well-known/agent.json` | 200 OK with valid JSON matching A2A Agent Card schema |
| CONF-002 | Content-Type validation | `GET /.well-known/agent.json` | `Content-Type: application/json` header present |
| CONF-003 | CORS headers | `OPTIONS /.well-known/agent.json` with `Origin: https://agentrank.dev` | Appropriate CORS headers allowing cross-origin reads |
| CONF-004 | Required fields present | Parse response | `name`, `description`, `url`, `version` fields present and non-empty |
| CONF-005 | Skills array valid | Parse `skills` field | Array of objects, each with at least `id` and `name` |
| CONF-006 | Endpoint reachable | `POST {endpoint_url}` with minimal A2A request | Non-5xx response (may be 4xx for auth-required agents) |
| CONF-007 | Error format compliance | `POST {endpoint_url}` with malformed request | Error response follows A2A error schema with `code` and `message` |
| CONF-008 | Version format | Parse `version` field | Semantic versioning format (X.Y.Z) or ISO date |
| CONF-009 | URL validity | Parse all URL fields | All URLs are valid, absolute, and use HTTPS |
| CONF-010 | UTF-8 encoding | Parse all string fields | All strings are valid UTF-8 |

### 25.2 Governance Rules

| Rule | Rationale |
|------|-----------|
| **No benchmark result inflation** | AgentRank never modifies benchmark results to make agents appear better than measured. Results are published as-is. |
| **Methodology transparency** | All benchmark test suites are open-source. Providers can run them locally before opting in. No secret tests. |
| **No pay-to-benchmark** | Benchmark enrollment is free for all tiers. Results are not gated behind payment. |
| **Result freshness** | Conformance and reliability benchmarks are updated continuously. Capability benchmarks are re-run monthly or on agent card update (whichever is sooner). |
| **Dispute resolution** | Providers can dispute benchmark results via the Search Console. Disputes trigger a manual re-run with detailed logging. If the benchmark test is found to be flawed, the result is corrected for all affected agents. |
| **No competitive weaponization** | Benchmark results are shown to the agent's own provider. They are NOT published in a public leaderboard that ranks agents against each other by benchmark score. The score contributes to discoverability but is not the sole determinant. |

### 25.3 Ranking Evaluation Framework

AgentRank's own ranking quality is evaluated continuously using standard information retrieval metrics.

| Metric | Definition | Target | Measurement Method |
|--------|-----------|--------|-------------------|
| **NDCG@10** | Normalized Discounted Cumulative Gain at rank position 10. Measures ranking quality with graded relevance. | > 0.75 | Weekly evaluation against human-judged query set (1,000 queries, 5-point relevance scale from quality raters). |
| **MRR** | Mean Reciprocal Rank. Average of 1/rank for the first relevant result across all queries. | > 0.80 | Same query set as NDCG. Binary relevance threshold: relevance ≥ 3 is "relevant". |
| **Coverage@10** | Percentage of queries where at least one result in the top 10 has the requested skill/capability. | > 95% | Automated: skill extraction from query, skill matching against top 10 results. |
| **Alive@10** | Percentage of agents in the top 10 results that are confirmed alive (passed liveness probe within last 24 hours). | > 99% | Automated: cross-reference top 10 results with liveness status. |
| **Spam@100** | Percentage of results in the top 100 that are confirmed spam by manual review. | < 1% | Monthly: random sample of 100 queries, top 100 results each, manual spam classification. |
| **Freshness@10** | Percentage of top 10 results where the indexed version is < 24 hours old relative to the live version. | > 90% | Automated: fetch live card for top 10 results, compare content hash with indexed version. |
| **Diversity@10** | Number of unique providers represented in the top 10 results. | > 5 (median) | Automated: count unique provider domains in top 10 results across query set. |
| **Latency P50** | Median end-to-end search latency (API gateway receive → response sent). | < 20ms | Prometheus histogram metric `agentrank_query_latency_seconds`. |
| **Latency P99** | 99th percentile end-to-end search latency. | < 100ms | Same Prometheus metric, 0.99 quantile. |

#### A/B Testing Framework for Ranking Changes

```
Every ranking change (formula weight adjustment, new signal addition, LTR model update)
must pass through this evaluation pipeline before production deployment:

1. OFFLINE EVALUATION
   - Run new ranking on historical query set (10,000 queries)
   - Compare NDCG@10, MRR, Coverage@10 vs. baseline
   - Gate: NDCG@10 must not decrease by more than 0.02
   
2. SHADOW EVALUATION (1-3 days)
   - Deploy new ranking alongside production (dual-compute)
   - Log both result sets for every live query
   - Compare live metrics without affecting users
   - Gate: No metric regression > 1%
   
3. CANARY DEPLOYMENT (3-7 days)
   - Route 5% of traffic to new ranking
   - Monitor all quality metrics + user engagement
   - Gate: CTR on new ranking >= CTR on baseline (within 95% CI)
   
4. GRADUAL ROLLOUT
   - 5% → 25% → 50% → 100% over 7-14 days
   - Automatic rollback if any quality metric drops > 2%
   
5. POST-LAUNCH EVALUATION (30 days)
   - Quality raters evaluate random sample of new vs. old rankings
   - Confirm long-term quality improvement
   - Update baseline metrics
```

---

## 26. Enterprise & Federation

### 26.1 Federation Modes

AgentRank supports three federation modes to integrate with enterprise and partner registries.

#### Pull Federation

```
AgentRank periodically fetches agent data from partner registries.

Partner Registry                    AgentRank
     │                                  │
     │  ◀── HTTP GET /agents?since=T ── │  (every 6 hours)
     │                                  │
     │  ── JSON array of agent cards ─▶ │
     │                                  │
     │                     ┌────────────┤
     │                     │ Validate   │
     │                     │ Normalize  │
     │                     │ Dedup      │
     │                     │ Index      │
     │                     └────────────┤
     │                                  │

Configuration:
  - Endpoint URL
  - Authentication (API key, OAuth, mTLS)
  - Polling interval (default 6 hours)
  - Incremental sync via `since` parameter (ISO 8601 timestamp)
  - Full sync weekly (catches deletions)
```

**Use Case:** Integrating with existing registries (AgentVerse, PulseMCP) that expose an API.

#### Push Federation

```
Partner registries push agent data to AgentRank in real-time.

Partner Registry                    AgentRank
     │                                  │
     │  ── POST /federation/ingest ──▶  │  (on agent create/update/delete)
     │                                  │
     │  ◀── 202 Accepted ────────────── │
     │                                  │
     │                     ┌────────────┤
     │                     │ Validate   │
     │                     │ Normalize  │
     │                     │ Dedup      │
     │                     │ Index      │
     │                     └────────────┤
     │                                  │

Configuration:
  - Webhook secret (HMAC-SHA256 signature validation)
  - Source registry identifier
  - Rate limit (1000 pushes/hour per source)
  - Payload format: A2A Agent Card JSON (or mapping adapter)
```

**Use Case:** Partner registries that want real-time index updates without waiting for pull cycles.

#### Hybrid Federation

```
Combines pull for initial sync + push for real-time updates.

1. Initial sync: Pull all agents from partner (backfill)
2. Ongoing: Partner pushes create/update/delete events in real-time
3. Reconciliation: Pull full sync weekly to catch missed pushes
4. Health check: If push events stop for > 24 hours, fall back to pull
```

**Use Case:** Production partner integrations where both latency and reliability matter.

### 26.2 Enterprise Requirements

| Requirement | Implementation | Phase |
|-------------|---------------|-------|
| **mTLS Authentication** | Enterprise API clients authenticate with X.509 client certificates. Certificates issued by AgentRank's private CA or customer's CA (pre-configured trust store per tenant). | 2 |
| **Tenant Isolation** | Each enterprise tenant has a logically isolated namespace in PostgreSQL (schema-per-tenant) and Qdrant (collection-per-tenant). Query routing ensures tenants never see each other's private agents. | 2 |
| **Audit Logging** | Every API call, search query, connection event, and administrative action for an enterprise tenant is logged to an immutable audit log. Exportable in CEF (Common Event Format) for SIEM integration. | 2 |
| **Region-Aware Deployment** | Enterprise tenants can specify data residency requirements (US, EU, APAC). Agent data for region-locked tenants is stored and processed only in the specified region. Index partitioned by region. | 3 |
| **Custom SLA** | Enterprise tenants receive a contractual SLA with financial penalties for violation. SLA targets negotiated per contract (typically 99.95% availability, P99 < 100ms). | 2 |
| **SSO Integration** | SAML 2.0 and OpenID Connect for enterprise user authentication. Supports Okta, Azure AD, Google Workspace, and custom OIDC providers. | 2 |
| **Role-Based Access Control** | Predefined roles: Admin (full access), Manager (read + verify), Viewer (read only), API-only (no console). Custom roles for enterprise tenants. | 2 |
| **Private Registry** | Enterprise tenants can register agents that are only visible within their tenant namespace. Private agents are not indexed in the public search. | 2 |
| **Data Export** | Enterprise tenants can export all their data (agent cards, trust scores, analytics, audit logs) in JSON or CSV format. Export completes within 24 hours for up to 1M agents. | 2 |
| **Dedicated Support** | Named account manager. Slack Connect channel for real-time support. 4-hour response SLA for P1 issues. | 3 |

### 26.3 Public vs Private Ranking Isolation

```
┌─────────────────────────────────────────────────────────────┐
│                    PUBLIC INDEX                               │
│                                                             │
│  Contains: All publicly discoverable agents                  │
│  Visible to: All users (anonymous, free, pro)               │
│  Ranking: AgentRank composite score (AVERT + text + vector) │
│  Trust: Public trust tiers (Unverified → Certified)         │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Agent A (public)  Agent B (public)  Agent C (pub.) │    │
│  │  Agent D (public)  Agent E (public)  ...            │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│              ENTERPRISE TENANT: Acme Corp                    │
│                                                             │
│  Contains: Public agents + Acme's private agents            │
│  Visible to: Acme Corp users only (mTLS + RBAC)            │
│  Ranking: AgentRank composite + Acme's custom weights       │
│  Trust: Public tiers + Acme's internal trust policies       │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Agent A (public)  Agent B (public)  Agent X (pvt.) │    │
│  │  Agent Y (private) Agent Z (private) ...            │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                             │
│  Custom ranking weights:                                     │
│    - Internal trust boost: 1.5× for Acme-owned agents      │
│    - Approved vendor list: 1.3× for pre-approved providers  │
│    - Compliance filter: exclude agents without SOC2 cert    │
│                                                             │
└─────────────────────────────────────────────────────────────┘

ISOLATION GUARANTEES:
  1. Acme's private agents NEVER appear in public search results
  2. Acme's query logs are NEVER visible to other tenants
  3. Acme's custom ranking weights are NEVER applied to public results
  4. Public agents appear in Acme's results with public ranking + Acme's overrides
  5. Acme can "pin" agents to always appear first for specific queries (internal override)
```

---

## 27. Monetization

### 27.1 Revenue Streams

| Revenue Stream | Description | Pricing | Phase |
|---------------|-------------|---------|-------|
| **API Access** | Tiered API access for programmatic agent discovery and connection. | Free (1K req/hr), Pro ($49/mo, 50K req/hr), Enterprise (custom) | 1 |
| **Agent Analytics** | Premium analytics in Agent Search Console: search query data, impression tracking, CTR analysis, competitor benchmarking, historical trends. | $19/agent/month (free for first 3 agents) | 1 |
| **Promoted Results** | Sponsored placement in search results. Clearly labeled as "Sponsored." Auction-based pricing (cost-per-click). Only available to Verified+ agents. | CPC auction, minimum $0.10/click, estimated $0.50-2.00 average CPC | 2 |
| **Data Licensing** | Aggregated, anonymized dataset of agent ecosystem trends, skill demand, category growth. Licensed to research institutions, VC firms, and enterprise strategy teams. | $5,000/year for research, $25,000/year for commercial, $100,000/year for enterprise | 3 |

#### Revenue Stream Details

**API Access Tiers:**

| Feature | Free | Pro ($49/mo) | Enterprise |
|---------|------|-------------|-----------|
| Rate limit | 1K/hr | 50K/hr | Custom |
| Search API | Yes | Yes | Yes |
| Connect API | No | Yes | Yes |
| Outcome API | No | Yes | Yes |
| Bulk export | No | 100/request | 1000/request |
| Webhooks | No | 5 endpoints | Unlimited |
| Support | Community | Email (48h) | Dedicated (4h) |
| SLA | Best effort | 99.9% | 99.95% |
| Private registry | No | No | Yes |
| Federation | No | No | Yes |
| SSO | No | No | Yes |

**Promoted Results Rules:**

| Rule | Rationale |
|------|-----------|
| Maximum 2 promoted results per query | Preserve user trust in organic results. |
| Promoted results must pass all quality checks | No spam or unverified agents in promoted positions. |
| "Sponsored" label is always visible | Transparency requirement. Never disguise ads as organic results. |
| Bidding on competitor brand names is allowed | Consistent with search advertising norms. Providers can bid on any keyword. |
| Promoted results ranked by bid × quality score | Higher-quality promoted results get preferred placement even with lower bids. Same as Google Ads quality score. |
| Click-through is verified | Invalid clicks (bots, self-clicks, click fraud) are detected and not charged. |

### 27.2 Unit Economics

#### Phase 2 Steady State (12 months post-launch)

```
Revenue:
  API Pro subscriptions:    10,000 customers × $49/mo  = $490,000/mo
  Agent Analytics:           5,000 agents × $19/mo     = $95,000/mo
  Promoted Results:         50,000 clicks × $1.00/click = $50,000/mo
                                                          ──────────
  Total MRR:                                             $635,000/mo
  Annualized:                                            $7,620,000/yr

Costs:
  Infrastructure:           $25,000/mo  (5M agents, 10K QPS)
  Team (23 engineers):      $350,000/mo (blended cost with benefits)
  Cloud/SaaS tools:         $10,000/mo
  Quality rater program:    $15,000/mo
  Customer support:         $20,000/mo
                            ──────────
  Total costs:              $420,000/mo
  Annualized:               $5,040,000/yr

Gross Margin (infra only):  ($635K - $25K) / $635K = 96.1%
Operating Margin:           ($635K - $420K) / $635K = 33.9%
Net Annual Profit:          $2,580,000

Break-even point:           ~3,500 Pro subscribers + baseline analytics revenue
```

#### Sensitivity Analysis

| Scenario | Pro Subscribers | Avg CPC | Monthly Revenue | Monthly Profit |
|----------|----------------|---------|-----------------|---------------|
| Conservative | 5,000 | $0.50 | $360,000 | -$60,000 |
| Base case | 10,000 | $1.00 | $635,000 | $215,000 |
| Optimistic | 25,000 | $1.50 | $1,395,000 | $975,000 |
| Bull case | 50,000 | $2.00 | $2,695,000 | $2,275,000 |

---

## Part IX: Operations and Growth

---

## 28. Observability & Operations

### 28.1 Prometheus Metrics

Every metric follows the naming convention: `agentrank_{subsystem}_{metric_name}_{unit}`.

#### Crawler Metrics

| Metric Name | Type | Labels | Description |
|------------|------|--------|-------------|
| `agentrank_crawl_total` | Counter | `status={success,failure,timeout,robots_blocked,ssrf_blocked}`, `source={frontier,seed,federation}` | Total crawl attempts by status and source |
| `agentrank_crawl_latency_seconds` | Histogram | `phase={dns,connect,tls,first_byte,download,parse}` | Latency breakdown for each crawl phase |
| `agentrank_crawl_frontier_size` | Gauge | `priority={high,medium,low}` | Current number of URLs in the crawl frontier by priority bucket |
| `agentrank_crawl_frontier_push_total` | Counter | `source={discovery,recrawl,seed}` | URLs added to frontier |
| `agentrank_crawl_frontier_pop_total` | Counter | | URLs consumed from frontier |
| `agentrank_crawl_bytes_total` | Counter | `direction={received,sent}` | Total bytes transferred during crawling |
| `agentrank_crawl_cards_parsed_total` | Counter | `result={valid,invalid_schema,invalid_url,too_large,duplicate}` | Agent cards parsed by result |
| `agentrank_crawl_robots_txt_cache_size` | Gauge | | Number of cached robots.txt entries |
| `agentrank_crawl_active_connections` | Gauge | `domain_bucket={0-10,10-100,100+}` | Number of active HTTP connections grouped by target domain density |
| `agentrank_crawl_rate_limited_total` | Counter | `limiter={per_domain,global}` | Requests delayed by rate limiter |
| `agentrank_crawl_dedup_filter_size` | Gauge | | Number of entries in the cuckoo filter |
| `agentrank_crawl_dedup_hit_total` | Counter | | URLs skipped due to deduplication |

#### Search Metrics

| Metric Name | Type | Labels | Description |
|------------|------|--------|-------------|
| `agentrank_query_total` | Counter | `source={web_ui,api,a2a_native}`, `tier={anonymous,free,pro,enterprise}` | Total search queries by source and API tier |
| `agentrank_query_latency_seconds` | Histogram | `phase={parse,understand,candidate_gen,fusion,rerank,format}` | End-to-end query latency with phase breakdown |
| `agentrank_query_results_count` | Histogram | | Number of results returned per query |
| `agentrank_query_cache_hit_ratio` | Gauge | | Rolling cache hit ratio (hits / total) over 5-minute window |
| `agentrank_query_cache_hit_total` | Counter | `layer={l1_local,l2_redis}` | Cache hits by cache layer |
| `agentrank_query_cache_miss_total` | Counter | | Cache misses requiring full search pipeline execution |
| `agentrank_query_empty_results_total` | Counter | | Queries that returned zero results (quality alert signal) |
| `agentrank_query_tantivy_latency_seconds` | Histogram | | Full-text search latency (Tantivy only) |
| `agentrank_query_qdrant_latency_seconds` | Histogram | | Vector search latency (Qdrant only) |
| `agentrank_query_ltr_inference_seconds` | Histogram | | LTR model inference latency |
| `agentrank_query_error_total` | Counter | `error_type={timeout,internal,bad_request,rate_limited}` | Query errors by type |

#### Trust Metrics

| Metric Name | Type | Labels | Description |
|------------|------|--------|-------------|
| `agentrank_trust_tier_distribution` | Gauge | `tier={unverified,basic,verified,trusted,certified}` | Number of agents in each trust tier |
| `agentrank_trust_anomaly_detected_total` | Counter | `anomaly_type={rapid_mutation,skill_explosion,description_churning,provider_hopping,clone,zombie}` | Anomaly detections by type |
| `agentrank_trust_verification_total` | Counter | `method={dns_txt,well_known,meta_tag}`, `result={success,failure,timeout}` | Verification attempts by method and result |
| `agentrank_trust_score_distribution` | Histogram | | Distribution of composite trust scores across all agents |
| `agentrank_trust_tier_transitions_total` | Counter | `from={unverified,basic,...}`, `to={basic,verified,...}` | Trust tier changes (upgrades and downgrades) |

#### Connection Metrics

| Metric Name | Type | Labels | Description |
|------------|------|--------|-------------|
| `agentrank_connect_initiated_total` | Counter | `mode={direct,brokered}` | Connection attempts by mode |
| `agentrank_connect_completed_total` | Counter | `mode={direct,brokered}`, `result={success,failure,timeout}` | Connection completions by mode and result |
| `agentrank_connect_broker_sessions_active` | Gauge | | Currently active brokered connection sessions |
| `agentrank_connect_broker_latency_seconds` | Histogram | `phase={setup,negotiation,handoff}` | Broker session phase latencies |
| `agentrank_connect_outcome_reported_total` | Counter | `outcome={success,partial,failure}` | Outcome reports by result |

#### Liveness Metrics

| Metric Name | Type | Labels | Description |
|------------|------|--------|-------------|
| `agentrank_liveness_probe_total` | Counter | `result={alive,dead,timeout,error}` | Liveness probe results |
| `agentrank_liveness_probe_latency_seconds` | Histogram | | Liveness probe response time |
| `agentrank_liveness_agents_alive_ratio` | Gauge | | Fraction of indexed agents currently alive |
| `agentrank_liveness_status_transitions_total` | Counter | `from={alive,degraded,dead,unknown}`, `to={alive,degraded,dead,unknown}` | Agent status transitions |

#### Infrastructure Metrics

| Metric Name | Type | Labels | Description |
|------------|------|--------|-------------|
| `agentrank_index_agents_total` | Gauge | `store={tantivy,qdrant,postgresql}` | Total agents indexed per store |
| `agentrank_index_sync_latency_seconds` | Histogram | `store={tantivy,qdrant}` | Index synchronization latency |
| `agentrank_index_freshness_seconds` | Gauge | | Age of the oldest stale record in the index (seconds since last sync for any agent) |
| `agentrank_kafka_consumer_lag` | Gauge | `topic`, `consumer_group` | Kafka consumer lag by topic and group |
| `agentrank_pg_replication_lag_seconds` | Gauge | `replica` | PostgreSQL replication lag per replica |
| `agentrank_redis_memory_used_bytes` | Gauge | `instance={cache,frontier}` | Redis memory usage |
| `agentrank_redis_keys_total` | Gauge | `instance={cache,frontier}` | Total keys in Redis |

### 28.2 Alerting Rules

| Alert Name | Condition | Duration | Severity | Response |
|-----------|-----------|----------|----------|----------|
| `SearchLatencyHigh` | `histogram_quantile(0.99, agentrank_query_latency_seconds) > 0.1` | 5 min | Critical (page) | Scale search engine replicas. Check Qdrant health. Review recent ranking changes for regression. |
| `CrawlerStalled` | `rate(agentrank_crawl_total[10m]) == 0` | 10 min | Critical (page) | Check crawler pod logs. Verify Redis frontier connectivity. Check for network partition. |
| `IndexStale` | `agentrank_index_freshness_seconds > 86400` | 30 min | Warning | Check index sync pipeline. Verify Kafka consumer lag. May indicate index sync crash. |
| `PGReplicationLag` | `agentrank_pg_replication_lag_seconds > 30` | 5 min | Warning | Check replica health. May need to rebuild replica if lag is persistent. |
| `RedisMemoryHigh` | `agentrank_redis_memory_used_bytes / redis_max_memory > 0.85` | 15 min | Warning | Review eviction policy. May need to increase Redis memory or add shards. |
| `TrustAnomalySpike` | `rate(agentrank_trust_anomaly_detected_total[1h]) > 50` | 5 min | Warning | Potential coordinated attack. Review anomaly types. May need to temporarily increase spam thresholds. |
| `APIErrorRateHigh` | `rate(agentrank_query_error_total{error_type="internal"}[5m]) / rate(agentrank_query_total[5m]) > 0.01` | 5 min | Critical (page) | 1% error rate exceeded. Check recent deployments. Review error logs for root cause. |
| `QdrantUnhealthy` | `qdrant_cluster_status != "green"` | 2 min | Critical (page) | Qdrant cluster degraded. Check shard allocation. May need to rebuild index from PostgreSQL. |
| `KafkaConsumerLag` | `agentrank_kafka_consumer_lag > 100000` | 15 min | Warning | Consumer falling behind. Scale consumer group. Check for slow processing in downstream service. |
| `EmptyResultsHigh` | `rate(agentrank_query_empty_results_total[1h]) / rate(agentrank_query_total[1h]) > 0.10` | 30 min | Warning | 10%+ queries returning empty results. May indicate index corruption or overly aggressive filtering. |
| `CrawlSSRFBlocked` | `rate(agentrank_crawl_total{status="ssrf_blocked"}[1h]) > 100` | 5 min | Info | High SSRF attempt rate. May indicate targeted attack. Review source IPs. |
| `LivenessAliveRatioDrop` | `agentrank_liveness_agents_alive_ratio < 0.85` | 1 hour | Warning | More than 15% of agents appear dead. May indicate widespread outage OR liveness prober issue. Verify prober health first. |
| `DiskSpaceHigh` | `node_filesystem_avail_bytes / node_filesystem_size_bytes < 0.15` | 15 min | Warning | Less than 15% disk space remaining. Clean up old index segments. Check ClickHouse compaction. |
| `PodRestartLoop` | `kube_pod_container_status_restarts_total > 5` in 1 hour | 5 min | Critical (page) | Pod crash-looping. Check pod logs for OOM, panic, or configuration error. |

### 28.3 Service Level Objectives (SLOs)

| SLO | Target | Error Budget (30-day) | Measurement |
|-----|--------|----------------------|-------------|
| **Search Availability** | 99.9% | 43.2 minutes downtime/month | Ratio of successful (2xx) search API responses to total requests |
| **Search Latency P50** | < 20ms | N/A (performance target) | `histogram_quantile(0.50, agentrank_query_latency_seconds)` |
| **Search Latency P99** | < 100ms | N/A (performance target) | `histogram_quantile(0.99, agentrank_query_latency_seconds)` |
| **Index Freshness** | < 24 hours | N/A (freshness target) | Maximum age of any indexed agent card relative to its live version |
| **Crawl Throughput** | > 100K cards/day | N/A (throughput target) | `increase(agentrank_crawl_total{status="success"}[24h])` |
| **Connect Availability** | 99.5% | 3.6 hours downtime/month | Ratio of successful connection setups to total attempts |
| **Connect Latency P99** | < 500ms | N/A (performance target) | `histogram_quantile(0.99, agentrank_connect_broker_latency_seconds)` |
| **API Availability** | 99.9% | 43.2 minutes downtime/month | All non-search API endpoints |
| **Data Durability** | 99.999% | N/A (durability target) | No data loss events over rolling 12-month window |

### 28.4 Ranking Quality Metrics Dashboard

| Metric | Target | Current (placeholder) | Trend | Alert Threshold |
|--------|--------|----------------------|-------|----------------|
| **NDCG@10** | > 0.75 | 0.78 | ↑ | < 0.72 (email), < 0.65 (page) |
| **MRR** | > 0.80 | 0.83 | → | < 0.77 (email), < 0.70 (page) |
| **Coverage@10** | > 95% | 96.2% | ↑ | < 93% (email) |
| **Alive@10** | > 99% | 99.4% | → | < 98% (email) |
| **Spam@100** | < 1% | 0.3% | ↓ | > 2% (email), > 5% (page) |
| **Freshness@10** | > 90% | 92.1% | → | < 85% (email) |
| **Diversity@10** | > 5 providers (median) | 6.3 | → | < 4 (email) |
| **Empty Results Rate** | < 5% | 3.8% | ↓ | > 8% (email) |

---

## 29. Data Governance & Retention

### Data Lifecycle Policy

| Data Category | Examples | Classification | Retention | Archival | Deletion |
|--------------|----------|---------------|-----------|----------|----------|
| **Agent Cards (raw)** | JSON snapshots from crawler | Internal | 1 year (all versions) | S3 Glacier after 90 days | Purged after 1 year unless actively indexed |
| **Agent Cards (indexed)** | Normalized, enriched records in PostgreSQL | Internal | Indefinite (while agent is active) | N/A — always hot | Soft-deleted when agent is dead > 90 days. Hard-deleted after 1 year. |
| **Trust Scores** | AVERT component scores, composite trust tier | Internal | Indefinite (current) + 1 year (history) | ClickHouse for historical | Current overwritten on recomputation. History in ClickHouse follows ClickHouse retention. |
| **Search Queries** | Raw query strings, filters, result sets | Confidential | 90 days (pseudonymized) | Fully anonymized after 90 days. Aggregated stats retained indefinitely. | User deletion request: cascade within 72 hours. |
| **Outcome Telemetry** | Connection results, success/failure, latency | Confidential | 180 days | Aggregated stats retained indefinitely | Source identity stripped after 180 days |
| **Crawl Logs** | HTTP status, response time, bytes, errors | Internal | 30 days | ClickHouse with TTL | Auto-deleted by ClickHouse MergeTree TTL |
| **API Access Logs** | Request IP (truncated), endpoint, status, latency | Internal | 30 days | N/A | Auto-deleted by ClickHouse TTL |
| **User Accounts** | Email, hashed password, API keys, preferences | PII | Account lifetime + 30 days post-deletion | N/A | Account deletion: all PII removed within 30 days. Audit log entries retained with pseudonymized user ID. |
| **Verification Records** | Domain verification tokens, certificate metadata | Internal | Indefinite (while domain is verified) | N/A | Deleted when domain verification is revoked or expired > 180 days |
| **Benchmark Results** | Conformance, capability, reliability scores | Internal | 1 year | Aggregated trends retained indefinitely | Individual results older than 1 year are purged |
| **Embedding Vectors** | 768-dim vectors in Qdrant | Internal | Regenerated on each model update | Previous model's vectors deleted after new model is fully deployed | N/A — vectors are derived data, not primary data |
| **Kafka Events** | All inter-service events | Internal | 7 days | N/A — events are ephemeral transport | Auto-deleted by Kafka retention policy |
| **Prometheus Metrics** | Time-series metrics | Operational | 30 days (raw), 1 year (downsampled) | Thanos long-term storage for downsampled | Raw metrics auto-deleted after 30 days |
| **Distributed Traces** | OpenTelemetry spans | Operational | 7 days | N/A | Auto-deleted by Jaeger retention policy |

#### GDPR / CCPA Compliance

| Right | Implementation |
|-------|---------------|
| **Right to Access (Art. 15)** | User can export all personal data via Account Settings → Data Export. Includes query history, outcome reports, and account metadata. Generated within 24 hours. |
| **Right to Deletion (Art. 17)** | User deletion request triggers `user.deleted` Kafka event. All services consume and cascade delete within 72 hours. Confirmation email sent when complete. |
| **Right to Rectification (Art. 16)** | User can update account information via Account Settings. Changes propagate immediately. |
| **Data Portability (Art. 20)** | Data export available in JSON format, machine-readable. |
| **Purpose Limitation** | Query data used only for search improvement and analytics. Never sold to third parties. Agent data is public metadata — not personal data. |

---

## 30. Phased Delivery Roadmap

### 30.1 Phase 0: Foundation (Weeks 1-4)

| Week | Deliverable | Owner | Dependencies | Exit Criteria |
|------|-------------|-------|-------------|---------------|
| 1 | Repository setup, CI/CD pipeline, Rust workspace scaffold, dev environment (Docker Compose) | Platform | None | `cargo build` succeeds for all crates. CI runs `cargo test`, `cargo clippy`, `cargo fmt --check`. |
| 1 | PostgreSQL schema v1 (agents, providers, crawl_history, trust_records) | Registry | None | `sqlx migrate run` succeeds. Schema matches design document. |
| 1 | Redis cluster setup (frontier + cache) | Platform | None | Redis cluster health check passes. Benchmark: 100K ops/sec. |
| 2 | AgentBot v0.1: fetch a single URL, parse agent card, store in PostgreSQL | Discovery | PostgreSQL schema | Integration test: given a mock agent card URL, card is fetched, parsed, and stored correctly. |
| 2 | Card Parser v0.1: JSON schema validation, required field extraction, normalization | Registry | None | Unit tests: 50+ test cases covering valid, invalid, and edge-case agent cards. |
| 2 | URL Frontier v0.1: Redis sorted set, basic priority scoring, dedup check | Discovery | Redis | Integration test: enqueue 10K URLs, dequeue in priority order, no duplicates. |
| 3 | Tantivy index v0.1: index agent name, description, skills. Basic BM25 search. | Search | Card Parser | Integration test: index 1000 agents, search by keyword, verify results. |
| 3 | API Gateway v0.1: `/search` endpoint, anonymous tier, basic rate limiting | Search | Tantivy | End-to-end test: HTTP request → search → JSON response. P99 < 200ms. |
| 3 | Web UI v0.1: search box, results list, agent detail page | DX | API Gateway | Visual QA: search renders results. Mobile-responsive layout. |
| 4 | AgentBot v0.2: frontier-driven crawling, rate limiting, robots.txt compliance | Discovery | URL Frontier | Crawl 100 real agent card URLs without errors. Respect robots.txt. |
| 4 | Agent Search Console v0.1: domain verification, card inspector, crawl history | DX | PostgreSQL, API Gateway | Provider can verify domain and see their agent's crawl status. |
| 4 | Monitoring v0.1: Prometheus metrics, Grafana dashboards, basic alerts | Platform | All services | Dashboard shows key metrics for each service. Alerts fire on test conditions. |
| 4 | **Phase 0 Integration Test** | All | All above | End-to-end: seed URLs → crawl → parse → index → search → results displayed in UI. |

### 30.2 Phase 1: Autonomous Discovery (Weeks 5-12)

| Week | Deliverable | Owner | Dependencies | Exit Criteria |
|------|-------------|-------|-------------|---------------|
| 5-6 | Discovery source expansion: GitHub API, registry APIs, DNS TXT scanning | Discovery | AgentBot v0.2 | 50K+ agent card URLs discovered from multiple sources |
| 5-6 | Qdrant integration: embedding generation pipeline, vector index, hybrid search | Search | Embedding Generator | Vector search returns semantically similar results. Hybrid search outperforms BM25 alone on test set. |
| 6-7 | AgentRank v1 scoring: AVERT framework implementation, composite score computation | Ranking | PostgreSQL, Card Parser | All indexed agents have computed AVERT scores. Scores are consistent across recomputation. |
| 7-8 | Liveness probing system: health check scheduler, status state machine, uptime tracking | Trust | PostgreSQL, Agent registry | Liveness probes running every 6 hours for all indexed agents. Status transitions logged correctly. |
| 7-8 | Connection Broker v0.1: direct connect flow, brokered connect setup, outcome capture | Connect | API Gateway, Agent registry | End-to-end: caller searches → finds agent → initiates connection → connection established. |
| 9-10 | Trust system v1: domain verification (all 3 methods), TLS analysis, trust tier computation | Trust | Liveness system | 100+ agents verified through at least one method. Trust tiers assigned correctly. |
| 9-10 | Spam defense v1: crawl-time spam signals, composite spam score, demotion/exclusion actions | Quality | Card Parser, AgentRank | Known-spam test agents are correctly demoted or excluded. False positive rate < 5%. |
| 11-12 | Query understanding v0.1: intent classification, basic query expansion | Search | Search Engine | Intent classifier accuracy > 85% on test set. Query expansion improves recall by > 10%. |
| 11-12 | Agent Search Console v1.1: trust dashboard, search analytics, liveness history | DX | Trust system, ClickHouse | Providers can see trust tier, query impressions, and liveness history. |
| 12 | **Phase 1 Launch** | All | All above | 100K+ agents indexed. Search quality: NDCG@10 > 0.70. Availability > 99.5%. |

### 30.3 Phase 2: Intelligence (Weeks 13-24)

| Week | Deliverable | Owner | Dependencies | Exit Criteria |
|------|-------------|-------|-------------|---------------|
| 13-15 | Learning-to-rank v1: feature engineering, XGBoost training, ONNX export, serving integration | Ranking | Quality rater judgments, AVERT scores | LTR model improves NDCG@10 by > 5% vs. v1 formula in offline evaluation. |
| 13-15 | Outcome-driven ranking signals: connection success rate, caller satisfaction, outcome quality | Ranking | Connection Broker, Outcome API | Outcome signals integrated as LTR features. Positive impact on ranking quality. |
| 16-18 | Anti-gaming v2: link farm detection, CTR manipulation detection, behavioral anomaly detection | Quality | Graph analysis, Trust system | Simulated attacks are detected and mitigated. False positive rate < 2%. |
| 16-18 | Benchmark framework v1: conformance suite, reliability tracking, capability testing (opt-in) | Quality | Liveness system, Agent registry | 50+ conformance tests passing against test agents. Reliability tracking live for all agents. |
| 19-21 | Enterprise tenant isolation: schema-per-tenant, private registry, mTLS auth, RBAC | Enterprise | PostgreSQL, API Gateway | Two pilot enterprise tenants onboarded with full isolation. |
| 19-21 | Federation v1: pull federation from 3+ partner registries | Enterprise | Discovery, Registry | 50K+ agents ingested via federation. No duplicates with crawled agents. |
| 22-24 | Promoted results v1: auction system, billing integration, click tracking | Monetization | Search Engine, API Gateway | End-to-end: advertiser bids on keyword → promoted result appears → click tracked → billed. |
| 22-24 | Agent Analytics v1: premium Search Console features, competitor benchmarking | Monetization | ClickHouse, Agent Search Console | Analytics dashboard live with 30+ days of historical data. |
| 24 | **Phase 2 Launch** | All | All above | 1M+ agents indexed. NDCG@10 > 0.75. Enterprise pilot live. MRR > $100K. |

### 30.4 Phase 3: Dominance (Weeks 25-52)

| Week | Deliverable | Owner | Dependencies | Exit Criteria |
|------|-------------|-------|-------------|---------------|
| 25-30 | Federation v2: push federation, hybrid mode, partner portal | Enterprise | Federation v1 | 5+ partner registries integrated with real-time push updates. |
| 25-30 | Region-aware deployment: multi-region Kubernetes, data residency controls | Platform | Enterprise tenants | EU and US regions live. Data residency compliance verified. |
| 31-36 | Advanced ranking: semantic understanding, compositional queries, multi-agent workflow search | Search | LTR v1, Query understanding | "Find an agent that can translate English to French and then summarize" returns relevant multi-agent results. |
| 31-36 | Security benchmarking: automated security scanner, vulnerability assessment | Quality | Benchmark framework | Security scan runs against opt-in agents. Results integrated into trust score. |
| 37-42 | Predictive matching: proactive agent recommendations based on caller history and task context | Search | Outcome telemetry, User behavior data | Recommendation accuracy > 60% (user clicks on recommended agent). |
| 37-42 | Data licensing v1: anonymized ecosystem reports, API for trend data | Monetization | ClickHouse analytics | First data licensing customer onboarded. |
| 43-48 | Decentralized reputation: on-chain attestation integration, cross-registry trust portability | Trust | ERC-8004 integration | Proof-of-concept: trust score from AgentRank published as on-chain attestation. |
| 43-48 | MCP deep integration: MCP server discovery, tool-level indexing, cross-protocol search | Discovery | MCP protocol analysis | MCP servers discoverable through AgentRank search alongside A2A agents. |
| 49-52 | Ecosystem maturity: self-sustaining flywheel, agent SEO industry forming, 5M+ agents | All | All above | 5M+ agents indexed. 10K+ QPS peak. MRR > $500K. Federation with 10+ partners. |

---

## 31. Team Structure

### 31.1 Five Pods

#### Discovery Pod

| Role | Count | Responsibilities |
|------|-------|-----------------|
| Staff Engineer (Pod Lead) | 1 | Crawler architecture, frontier design, discovery source strategy, performance optimization |
| Backend Engineer (Rust) | 2 | AgentBot implementation, rate limiting, robots.txt handling, SSRF prevention |
| Backend Engineer (Python) | 1 | Discovery source integrations (GitHub API, registry scrapers, DNS scanning) |
| SRE | 0.5 (shared) | Crawler monitoring, alerting, capacity planning |

**Owns:** AgentBot, URL Frontier, Discovery Sources, Crawl Scheduling

#### Registry & Trust Pod

| Role | Count | Responsibilities |
|------|-------|-----------------|
| Staff Engineer (Pod Lead) | 1 | Registry schema design, trust model, verification protocols, anti-gaming strategy |
| Backend Engineer (Rust) | 2 | Card parser, dedup engine, spam detection, trust score computation |
| Backend Engineer (Python) | 1 | Anomaly detection models, spam classifier training, behavioral analysis |
| Database Engineer | 1 | PostgreSQL optimization, schema migrations, query performance, ClickHouse analytics |
| SRE | 0.5 (shared) | Database operations, backup verification, replication monitoring |

**Owns:** Card Parser, Dedup Engine, Agent Registry, Trust System, Spam Defense, Domain Verification

#### Search & Ranking Pod

| Role | Count | Responsibilities |
|------|-------|-----------------|
| Staff Engineer (Pod Lead) | 1 | Search architecture, ranking formula, retrieval pipeline, evaluation framework |
| Backend Engineer (Rust) | 2 | Search engine implementation, Tantivy integration, result fusion, API serving |
| ML Engineer | 2 | Embedding models, LTR training, query understanding, ranking evaluation |
| SRE | 0.5 (shared) | Qdrant operations, search performance monitoring, index management |

**Owns:** Search Engine, Tantivy Index, Qdrant Integration, Ranking (AVERT + LTR), Query Understanding, Embedding Pipeline, Evaluation Framework

#### Connection Pod

| Role | Count | Responsibilities |
|------|-------|-----------------|
| Staff Engineer (Pod Lead) | 1 | Connection broker architecture, protocol negotiation, outcome capture |
| Backend Engineer (Rust) | 2 | Connection broker implementation, WebSocket handling, session management |
| SRE | 0.5 (shared) | Broker operations, session monitoring |

**Owns:** Connection Broker, Direct/Brokered Connect, Outcome Telemetry API, Liveness Prober

#### Developer Experience Pod

| Role | Count | Responsibilities |
|------|-------|-----------------|
| Engineering Manager (Pod Lead) | 1 | DX strategy, developer relations, documentation, community building |
| Full-Stack Engineer | 2 | Web UI (Next.js), Agent Search Console, API documentation, SDK stubs |
| Frontend Engineer | 1 | UI/UX design, component library, accessibility, performance |
| Technical Writer | 1 | API documentation, integration guides, tutorials, blog posts |

**Owns:** Web UI, Agent Search Console, Public Documentation, Developer SDKs, API Reference

#### Cross-Cutting

| Role | Count | Responsibilities |
|------|-------|-----------------|
| CTO / Principal Engineer | 1 | Architecture decisions, cross-pod coordination, hiring, technical strategy |
| Platform Engineer | 2 | Kubernetes, CI/CD, infrastructure-as-code, monitoring stack, security |
| **Total** | **~23** | |

### 31.2 Quarterly Build Sequence

| Quarter | Focus | Pod Allocation | Key Deliverable |
|---------|-------|---------------|----------------|
| Q1 (Weeks 1-12) | Foundation + Autonomous Discovery | Discovery (4), Registry (5), Search (5), Connection (3), DX (4), Platform (2) | Phase 0 + Phase 1 launch. 100K agents indexed. |
| Q2 (Weeks 13-24) | Intelligence + Enterprise | Discovery (3), Registry (4), Search (5), Connection (3), DX (4), Platform (2) + 2 Enterprise hires | Phase 2 launch. LTR ranking. Enterprise pilot. |
| Q3 (Weeks 25-36) | Scale + Federation | Discovery (3), Registry (4), Search (5), Connection (2), DX (3), Platform (3) + 2 Federation hires | 1M+ agents. 5+ federation partners. Multi-region. |
| Q4 (Weeks 37-52) | Dominance + Monetization | All pods fully staffed + 2 ML hires for predictive matching | 5M+ agents. MRR > $500K. Self-sustaining flywheel. |

---

## 32. Competitive Moat

AgentRank's moat is not a single feature. It is a system of seven interlocking advantages that compound over time, making the system progressively harder to replicate.

### 32.1 The Seven Layers

| Layer | Moat | Why It's Hard to Replicate | Time to Build |
|-------|------|---------------------------|---------------|
| **1. Coverage** | Largest index of agent cards, discovered autonomously. Not dependent on manual submission. | Requires building a crawler, maintaining frontier, handling robots.txt, SSRF prevention, and rate limiting for millions of domains. Google built this over years. | 6-12 months for comprehensive coverage |
| **2. Registry Quality** | Parsed, normalized, deduplicated, enriched agent records. Not raw dumps. | Requires deep understanding of A2A spec variations, entity resolution across registries, and continuous normalization as specs evolve. | 12+ months of refinement |
| **3. Trust Graph** | Multi-layered trust system with domain verification, behavioral analysis, and ecosystem signals. | Requires operational history. Trust scores improve with time. A new entrant has zero trust history for any agent. | 12+ months of data accumulation |
| **4. Ranking Quality** | AVERT framework + LTR models trained on human judgments and outcome telemetry. | Requires quality rater program, outcome data from real connections, and ML infrastructure. Quality improves with query volume. | 18+ months of data + ML investment |
| **5. Connection Loop** | Outcome telemetry from real agent-to-agent connections feeds back into ranking. | Requires both callers and providers to use the system. Chicken-and-egg problem that only the first mover solves. | 6+ months after reaching critical mass |
| **6. Provider Ecosystem** | Agent Search Console creates dependency. Providers optimize for AgentRank like webmasters optimize for Google. | Requires provider adoption. Once providers have invested effort in their AgentRank presence, switching costs are high. | 12+ months of ecosystem building |
| **7. Federation** | Partnerships with existing registries for data sharing. Each partnership is exclusive or semi-exclusive. | Requires business development, trust, and technical integration. First mover has advantage in securing partnerships. | 6-12 months per partnership |

### 32.2 The Compounding Feedback Cycle

```
                    ┌──────────────────────┐
                    │                      │
         ┌─────────▶  MORE AGENTS INDEXED  ├─────────┐
         │         │                      │         │
         │         └──────────────────────┘         │
         │                                          │
         │                                          ▼
┌────────┴─────────┐                     ┌──────────────────────┐
│                  │                     │                      │
│  PROVIDERS       │                     │  BETTER SEARCH       │
│  OPTIMIZE FOR    │                     │  RESULTS             │
│  AGENTRANK       │                     │                      │
│                  │                     └──────────┬───────────┘
└────────▲─────────┘                                │
         │                                          │
         │                                          ▼
         │         ┌──────────────────────┐         │
         │         │                      │         │
         └─────────┤  MORE USERS SEARCH   ◀─────────┘
                   │                      │
                   └──────────┬───────────┘
                              │
                              ▼
                   ┌──────────────────────┐
                   │                      │
                   │  MORE OUTCOME DATA   │
                   │  (connections, CTR)   │
                   │                      │
                   └──────────┬───────────┘
                              │
                              ▼
                   ┌──────────────────────┐
                   │                      │
                   │  BETTER RANKING      │
                   │  (LTR improves)      │
                   │                      │
                   └──────────────────────┘

Each cycle strengthens every subsequent cycle.
A competitor entering at any point faces the accumulated
advantage of all previous cycles.
```

### 32.3 What Competitors Will Miss

| Competitor Approach | Why It Fails Against AgentRank |
|--------------------|-------------------------------|
| **"Just build a directory"** | Directories are lists, not search engines. No ranking, no trust, no freshness guarantee. AgentRank's value is in the quality of ranking and trust signals, not just listing. |
| **"Use LLMs to rank"** | LLMs are expensive (~$0.01 per ranking request at GPT-4 quality) and slow (~500ms). AgentRank ranks in < 20ms at < $0.0001 per request. 100× cheaper, 25× faster. LLMs may help with query understanding but cannot replace a ranking engine. |
| **"Decentralize everything"** | Decentralized systems are slow, expensive, and lack the feedback loops that make ranking improve over time. Nobody has built a decentralized search engine that competes with Google. The same applies to agent search. |
| **"Piggyback on Google"** | Google indexes web pages, not agent metadata. Agent cards are JSON files that Google doesn't understand semantically. By the time Google adapts, AgentRank has years of domain-specific ranking quality. |
| **"Registry with added search"** | Existing registries (AgentVerse, PulseMCP) could add search. But their data is manual submissions, not autonomous discovery. They have no crawler, no trust system, no outcome data, and no ranking science. Adding these is a multi-year effort. |

---

## 33. Key Risks & Mitigations

| Risk | Probability | Impact | Mitigation Strategy | Contingency |
|------|------------|--------|-------------------|-------------|
| **Sparse ecosystem** — Too few quality agents exist to make search valuable. If the A2A ecosystem grows slowly, AgentRank has nothing to index. | Medium | High | Expand discovery beyond A2A: index MCP servers, tool-use APIs, any machine-callable service. Lower the bar for "what counts as an agent." Create a "list your agent" flow for manual submission as a fallback. | Pivot to "API search engine" — broader scope that encompasses agents as a subset. |
| **Spam and fake agents** — Attackers flood the index with spam, degrading search quality and user trust. | High | High | Multi-layered spam defense (§22): crawl-time signals, behavioral anomaly detection, link farm analysis, quality rater program. Invest aggressively in spam defense from day one. | Increase manual review capacity. Implement invite-only indexing for new agents until spam is controlled. Whitelist known-good domains. |
| **Poor ranking quality** — Search results are not relevant, and users abandon the product. | Medium | Critical | Invest in ranking evaluation from Phase 0. Quality rater program. A/B testing framework. Weekly ranking quality reviews. Hire ML engineers with search ranking experience. | Simplify ranking to basic keyword match + popularity (which at least works) while improving more sophisticated ranking offline. |
| **Slow freshness** — Indexed agents become stale, showing dead or outdated agents in results. | Medium | High | Aggressive crawl scheduling for popular agents (every hour). Liveness probing every 6 hours. Real-time push updates via federation. Prioritize re-crawl based on staleness signals. | Add a "last verified" badge to search results so users know how fresh the data is. Allow agents to self-report changes via webhook. |
| **Hard enterprise adoption** — Enterprise customers are slow to adopt due to security, compliance, or integration concerns. | Medium | Medium | Invest in enterprise security features (mTLS, audit logging, SOC2 compliance) early. Offer free pilot programs. Dedicated enterprise sales engineer. | Focus on developer/startup segment first. Build bottom-up adoption that creates pressure for enterprise procurement. |

---

## 34. Open Questions & Future Work

### 34.1 Technical Questions

| # | Question | Context | Proposed Resolution Path |
|---|----------|---------|--------------------------|
| 1 | **Should AgentRank run its own embedding model or use a hosted API?** | Self-hosted: lower latency, no per-call cost, data stays private. Hosted API: higher quality, no GPU infrastructure to manage, easy model upgrades. | Start with self-hosted `all-MiniLM-L6-v2` (CPU-friendly, 384-dim). Benchmark against API-based `text-embedding-3-small`. Decision by Phase 1 launch based on quality/cost tradeoff. |
| 2 | **How should AgentRank handle agents that require authentication to access their card?** | Some enterprise agents serve their agent card behind OAuth or API key authentication. The crawler cannot access these without credentials. | Phase 1: skip authenticated cards (they are not publicly discoverable). Phase 2: allow providers to manually submit cards via Search Console. Phase 3: support provider-issued crawl credentials via Search Console. |
| 3 | **What is the right balance between crawl freshness and crawl politeness?** | More frequent crawling means fresher data but more load on agent providers. Aggressive crawling may get AgentBot blocked. | Adaptive crawl scheduling: frequent for rapidly-changing agents, infrequent for stable ones. Default: daily. High-value: hourly. Stable: weekly. Respect `Crawl-delay` in robots.txt. Monitor block rate and back off automatically. |
| 4 | **Should outcome telemetry be mandatory for pro/enterprise tiers?** | Mandatory: more data for ranking improvement. Optional: lower friction for adoption. | Start optional. Make it strongly encouraged (rank boost for agents whose callers report outcomes). Evaluate adoption rate at Phase 2. Make mandatory only if adoption rate < 20% at Phase 3. |
| 5 | **How should AgentRank handle versioned agents (same agent, multiple versions)?** | Some providers maintain multiple versions of an agent simultaneously (v1, v2-beta). Should each version be a separate search result? | Each version is a separate registry entry with a shared `agent_group_id`. Default search shows the latest stable version. Filter available for "show all versions." Version relationship displayed in agent detail page. |

### 34.2 Research Directions

| # | Direction | Potential Impact | Timeline | Approach |
|---|-----------|-----------------|----------|----------|
| 1 | **Agent Capability Ontology** | Standardized taxonomy of agent capabilities that enables precise skill matching and hierarchical search ("translation" → "language processing" → "NLP"). | Phase 2-3 | Build initial ontology from crawled skill data (frequency analysis + LLM clustering). Publish as open specification. Iterate based on community feedback. |
| 2 | **Compositional Agent Search** | "Find a workflow that can: extract data from PDF, translate to French, and email the result" → returns multi-agent compositions, not single agents. | Phase 3+ | Requires agent composition graph: which agents' outputs are compatible with which agents' inputs. Start with declared input/output schemas in agent cards. |
| 3 | **Predictive Capability Benchmarks** | Instead of testing what agents claim, predict what agents can actually do based on metadata signals and past performance. | Phase 3 | Train ML model on features: provider reputation, technology stack signals, response patterns, outcome telemetry → predicted capability score. Reduces need for expensive live testing. |
| 4 | **Predictive Agent Matching** | Before a caller searches, predict which agents they need based on their task context (e.g., code being written, conversation history). | Phase 4 | Requires integration with caller environments (IDE plugins, chatbot platforms). Privacy-preserving: context summarized locally, only summary sent to AgentRank. |
| 5 | **Decentralized Reputation Protocol** | Publish trust scores as verifiable credentials on a public ledger. Agents carry their reputation across platforms. | Phase 4+ | Partner with decentralized identity projects (did:web, Verifiable Credentials). Publish AgentRank trust attestations as signed VCs. Explore ERC-8004 integration for on-chain agents. |

---

## Appendices

---

## 35. Appendix A: A2A Agent Card Schema Reference

The following JSON Schema defines the expected structure of an A2A Agent Card as understood by AgentRank. This schema is a superset of the base A2A specification, including fields that AgentRank recognizes for enhanced ranking and trust.

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://agentrank.dev/schemas/agent-card/v1",
  "title": "A2A Agent Card",
  "description": "Schema for an A2A-compatible agent card as indexed by AgentRank",
  "type": "object",
  "required": ["name", "description", "url", "version"],
  "properties": {
    "name": {
      "type": "string",
      "minLength": 1,
      "maxLength": 200,
      "description": "Human-readable name of the agent"
    },
    "description": {
      "type": "string",
      "minLength": 10,
      "maxLength": 5000,
      "description": "Detailed description of the agent's purpose and capabilities"
    },
    "url": {
      "type": "string",
      "format": "uri",
      "pattern": "^https?://",
      "description": "The canonical URL where this agent card is hosted"
    },
    "version": {
      "type": "string",
      "description": "Agent version (semver or ISO date)"
    },
    "provider": {
      "type": "object",
      "properties": {
        "organization": {
          "type": "string",
          "description": "Name of the organization providing the agent"
        },
        "url": {
          "type": "string",
          "format": "uri",
          "description": "Organization's website URL"
        },
        "contact": {
          "type": "string",
          "format": "email",
          "description": "Contact email for the agent provider"
        }
      },
      "required": ["organization"]
    },
    "capabilities": {
      "type": "object",
      "properties": {
        "streaming": {
          "type": "boolean",
          "default": false,
          "description": "Whether the agent supports streaming responses"
        },
        "pushNotifications": {
          "type": "boolean",
          "default": false,
          "description": "Whether the agent supports push notifications"
        },
        "stateTransitionHistory": {
          "type": "boolean",
          "default": false,
          "description": "Whether the agent tracks state transition history"
        }
      }
    },
    "skills": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "name"],
        "properties": {
          "id": {
            "type": "string",
            "description": "Unique identifier for the skill"
          },
          "name": {
            "type": "string",
            "description": "Human-readable skill name"
          },
          "description": {
            "type": "string",
            "description": "Detailed description of what this skill does"
          },
          "tags": {
            "type": "array",
            "items": { "type": "string" },
            "description": "Categorization tags for the skill"
          },
          "examples": {
            "type": "array",
            "items": { "type": "string" },
            "description": "Example inputs or use cases for this skill"
          },
          "inputModes": {
            "type": "array",
            "items": { "type": "string" },
            "description": "Supported input content types"
          },
          "outputModes": {
            "type": "array",
            "items": { "type": "string" },
            "description": "Supported output content types"
          }
        }
      },
      "description": "List of skills/capabilities the agent offers"
    },
    "authentication": {
      "type": "object",
      "properties": {
        "schemes": {
          "type": "array",
          "items": {
            "type": "string",
            "enum": ["none", "apiKey", "oauth2", "bearer", "mtls"]
          },
          "description": "Supported authentication schemes"
        },
        "credentials": {
          "type": "string",
          "description": "URL to obtain credentials (if applicable)"
        }
      }
    },
    "defaultInputModes": {
      "type": "array",
      "items": { "type": "string" },
      "default": ["text/plain"],
      "description": "Default input content types accepted by the agent"
    },
    "defaultOutputModes": {
      "type": "array",
      "items": { "type": "string" },
      "default": ["text/plain"],
      "description": "Default output content types produced by the agent"
    },
    "supportsMultipleLanguages": {
      "type": "boolean",
      "default": false,
      "description": "Whether the agent supports multiple natural languages"
    },
    "languages": {
      "type": "array",
      "items": { "type": "string" },
      "description": "ISO 639-1 language codes supported by the agent"
    },
    "pricing": {
      "type": "object",
      "properties": {
        "model": {
          "type": "string",
          "enum": ["free", "freemium", "paid", "enterprise"],
          "description": "Pricing model"
        },
        "currency": {
          "type": "string",
          "description": "ISO 4217 currency code"
        },
        "amount": {
          "type": "number",
          "description": "Price per unit (if applicable)"
        },
        "unit": {
          "type": "string",
          "description": "Billing unit (e.g., 'request', 'minute', 'token')"
        },
        "detailsUrl": {
          "type": "string",
          "format": "uri",
          "description": "URL with full pricing details"
        }
      }
    },
    "documentation": {
      "type": "string",
      "format": "uri",
      "description": "URL to agent documentation"
    },
    "termsOfService": {
      "type": "string",
      "format": "uri",
      "description": "URL to terms of service"
    },
    "privacyPolicy": {
      "type": "string",
      "format": "uri",
      "description": "URL to privacy policy"
    },
    "agentRankExtensions": {
      "type": "object",
      "description": "AgentRank-specific extensions (not part of base A2A spec)",
      "properties": {
        "delegations": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "agentUrl": { "type": "string", "format": "uri" },
              "relationship": {
                "type": "string",
                "enum": ["delegates_to", "composed_with", "backed_by"]
              }
            }
          },
          "description": "Relationships with other agents"
        },
        "benchmarkOptIn": {
          "type": "boolean",
          "default": false,
          "description": "Whether the agent opts in to AgentRank capability benchmarking"
        },
        "preferredCrawlSchedule": {
          "type": "string",
          "enum": ["hourly", "daily", "weekly"],
          "description": "Preferred crawl frequency"
        }
      }
    }
  }
}
```

---

## 36. Appendix B: AgentBot Identification

### User-Agent String

```
AgentBot/1.0 (+https://agentrank.dev/bot; agentbot@agentrank.dev)
```

Components:
- `AgentBot/1.0` — crawler name and version
- `+https://agentrank.dev/bot` — informational URL about the crawler
- `agentbot@agentrank.dev` — contact email for crawler operators

### Crawler Behavior Summary

| Behavior | Value | Notes |
|----------|-------|-------|
| **Default crawl rate** | 1 request/second per domain | Respects `Crawl-delay` in robots.txt |
| **Maximum concurrent connections per domain** | 2 | Never overloads a single server |
| **Global concurrent connections** | 10,000 | Across all domains |
| **robots.txt compliance** | Full | Respects `Disallow`, `Allow`, `Crawl-delay`, `Sitemap` directives for `AgentBot` and `*` |
| **Conditional requests** | Yes | Sends `If-Modified-Since` and `If-None-Match` headers when available |
| **Compression** | Yes | Sends `Accept-Encoding: gzip, br` |
| **Cookie handling** | None | Does not send or store cookies |
| **JavaScript execution** | None | Does not execute JavaScript |
| **Maximum response size** | 1 MB | Responses larger than 1MB are truncated |
| **Request timeout** | 30 seconds | Connection timeout: 10 seconds |
| **Redirect following** | Up to 5 | Follows HTTP 301, 302, 307, 308 |
| **Retry on failure** | 3 attempts | Exponential backoff: 1s, 5s, 25s |

### Opt-Out Mechanisms

Agent providers can opt out of AgentRank crawling using any of these methods:

| Method | Implementation | Scope |
|--------|---------------|-------|
| **robots.txt** | `User-agent: AgentBot` followed by `Disallow: /` | Per-domain (blocks all paths) or per-path |
| **X-Robots-Tag header** | `X-Robots-Tag: noindex` on the agent card HTTP response | Per-resource |
| **Meta tag** | `<meta name="robots" content="noindex">` in HTML pages linking to agent cards | Per-page |
| **Agent Search Console** | Request de-listing via the Search Console UI | Per-agent |
| **Email** | Send request to `agentbot@agentrank.dev` with domain and agent URLs | Per-domain or per-agent |

#### robots.txt Examples

```
# Block AgentBot from all paths:
User-agent: AgentBot
Disallow: /

# Block AgentBot from specific paths:
User-agent: AgentBot
Disallow: /internal/
Disallow: /staging/

# Allow AgentBot but limit crawl rate:
User-agent: AgentBot
Crawl-delay: 5
Allow: /

# AgentRank-proposed extension: agent-specific directives
# (proposed convention, not yet standardized)
User-agent: AgentBot
Allow: /.well-known/agent.json
Disallow: /api/
Crawl-delay: 2
```

---

## 37. Appendix C: Glossary

| Term | Definition |
|------|-----------|
| **A2A (Agent-to-Agent)** | Google's open protocol for AI agents to discover, communicate, and collaborate with each other. Defines the Agent Card format, task lifecycle, and communication primitives. AgentRank indexes agents that conform to (or approximate) this protocol. |
| **Agent Card** | A JSON metadata document that describes an AI agent's identity, capabilities, skills, authentication requirements, and endpoint URL. The fundamental unit of data that AgentRank crawls, parses, indexes, and ranks. Analogous to a web page in traditional search. |
| **AgentBot** | AgentRank's autonomous web crawler. Discovers and fetches agent cards from the open internet. Named following the convention of Googlebot, Bingbot, etc. Identified by User-Agent string `AgentBot/1.0`. |
| **AgentRank** | (1) The overall system/product: a search engine for AI agents. (2) The composite ranking score assigned to each agent, combining text relevance, vector similarity, and AVERT trust signals. Context determines which meaning applies. |
| **AVERT** | AgentRank's trust and quality framework. Stands for: **A**vailability (is it alive?), **V**erification (is it who it claims?), **E**vidence (is there proof of quality?), **R**elationship (do trusted entities vouch?), **T**ransparency (is it open about its capabilities?). Each dimension produces a 0-1 score that feeds into the composite ranking. |
| **ASO (Agent Search Optimization)** | The practice of optimizing an agent card's metadata to improve its ranking and discoverability in AgentRank. Analogous to SEO (Search Engine Optimization) for web pages. Includes: writing clear descriptions, declaring accurate skills, obtaining domain verification, maintaining uptime, and earning trust signals. |
| **BM25** | Best Matching 25. A probabilistic information retrieval function used by Tantivy for full-text search. Ranks documents by term frequency (TF), inverse document frequency (IDF), and document length normalization. The foundation of AgentRank's text-based retrieval before vector and LTR layers are applied. |
| **CT Log (Certificate Transparency Log)** | A publicly auditable, append-only log of TLS certificates. AgentRank monitors CT logs to detect new certificates issued for verified domains, which may indicate domain compromise or ownership change. |
| **Cuckoo Filter** | A space-efficient probabilistic data structure for approximate set membership testing. Used by AgentRank for URL deduplication in the crawl frontier. Provides O(1) lookup with configurable false positive rate (~3%) and supports deletion (unlike Bloom filters). |
| **Connection Broker** | AgentRank's service that mediates the setup of agent-to-agent connections. Handles capability negotiation, protocol agreement, and session establishment. Steps out of the data path after setup is complete (not a proxy). |
| **HNSW (Hierarchical Navigable Small World)** | The approximate nearest neighbor algorithm used by Qdrant for vector search. Builds a multi-layer graph where each node connects to its nearest neighbors. Provides O(log N) query time with high recall (> 95% at typical settings). |
| **Kafka** | A distributed event streaming platform used by AgentRank as the inter-service message bus. Events like `agent.discovered`, `agent.validated`, `query.executed`, and `outcome.reported` flow through Kafka topics, enabling loose coupling between services. |
| **LTR (Learning-to-Rank)** | A machine learning approach to ranking where a model learns to order search results based on labeled training data. AgentRank uses XGBoost with the `rank:ndcg` objective, trained on human quality rater judgments and outcome telemetry. The model takes 50+ features per agent and produces a relevance score. |
| **mTLS (Mutual TLS)** | TLS where both the client and server present certificates, providing mutual authentication. Used by AgentRank for enterprise API access and service-to-service communication within the cluster (via service mesh). |
| **NDCG (Normalized Discounted Cumulative Gain)** | An information retrieval metric that measures ranking quality using graded relevance judgments. Ranges from 0 to 1, where 1 means perfect ranking. "Discounted" because relevant results at lower positions receive less credit. AgentRank targets NDCG@10 > 0.75. |
| **ONNX (Open Neural Network Exchange)** | An open format for representing machine learning models. AgentRank exports XGBoost LTR models to ONNX format and serves them in the Rust search engine using the `ort` crate (ONNX Runtime). This avoids requiring Python in the search hot path. |
| **Qdrant** | An open-source vector similarity search engine written in Rust. AgentRank uses Qdrant to store and query agent embedding vectors for semantic search. Supports HNSW indexing, scalar/product quantization, and metadata filtering during search. |
| **RRF (Reciprocal Rank Fusion)** | A method for combining ranked lists from multiple retrieval systems. Score = Σ(1 / (k + rank_i)) where k is a constant (typically 60). AgentRank uses RRF to fuse BM25 text results with vector similarity results before re-ranking. Advantages: no score normalization needed, robust to score scale differences. |
| **SimHash** | A locality-sensitive hashing algorithm that produces similar hashes for similar documents. Used by AgentRank for near-duplicate agent card detection. Two documents with SimHash Hamming distance < 3 (of 64 bits) are considered near-duplicates. |
| **Tantivy** | A full-text search engine library written in Rust. AgentRank embeds Tantivy in-process for BM25-based text retrieval over agent metadata (name, description, skills). Analogous to Apache Lucene but native Rust with no JVM dependency. |
| **Well-known URI** | A standardized path prefix (`/.well-known/`) for hosting metadata documents on web servers (RFC 8615). Agent cards are typically hosted at `/.well-known/agent.json`. AgentRank also uses `/.well-known/agentrank-verify.json` for domain verification. |
| **UUIDv7** | A time-ordered UUID format (RFC 9562) where the first 48 bits encode a Unix timestamp in milliseconds. AgentRank uses UUIDv7 for all primary keys, providing both uniqueness and chronological ordering. This improves database insert performance (sequential B-tree writes) and enables time-range queries on primary keys. |
| **Governor** | A Rust crate implementing the Generic Cell Rate Algorithm (GCRA) for rate limiting. Used by AgentRank for per-domain crawl rate limiting and per-tier API rate limiting. Provides zero-allocation steady-state performance and integrates with `DashMap` for concurrent per-key limits. |
| **Tokio** | An asynchronous runtime for Rust that provides a multi-threaded, work-stealing executor. The foundation of all AgentRank Rust services. Handles I/O multiplexing, timers, and task scheduling with ~200 bytes overhead per spawned task (vs. 4KB minimum for Go goroutines). |
| **Vector (log aggregation)** | A high-performance observability data pipeline written in Rust (by Datadog). AgentRank deploys Vector as a DaemonSet on each Kubernetes node to collect structured JSON logs from all services and ship them to ClickHouse for analysis. Not to be confused with "vector" as in embedding vectors. |

---

## End of Document

**AgentRank: The Definitive Search Engine for AI Agents**

*Version 0.1.0-draft — March 2026*

*This document represents the complete technical architecture. It is a living document that will evolve as the system is built, tested, and refined based on real-world feedback.*

---

*Total sections: 37 (including appendices)*
*Estimated implementation time: 12 months to Phase 3*
*Team size: ~23 engineers across 5 pods*
*Infrastructure cost: $4,500/month (Phase 1) → $25,000/month (Phase 3)*
