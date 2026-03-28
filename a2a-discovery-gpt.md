# A2A Discovery GPT: Master Product Requirements and Technical Architecture

## The Definitive Blueprint for a Market-Leading Agent Discovery, Trust, and Connection Platform

**Version:** 1.0.0-draft  
**Date:** 2026-03-23  
**Status:** Master synthesis document  
**Purpose:** Combine the strongest ideas from prior discovery proposals, resolve inconsistencies, add missing strategic and technical detail, and produce a category-winning architecture for the agent discovery layer.

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Strategic Thesis](#2-strategic-thesis)
3. [Why Now](#3-why-now)
4. [Market Problem and Competitive Lens](#4-market-problem-and-competitive-lens)
5. [Vision, Positioning, and Product Definition](#5-vision-positioning-and-product-definition)
6. [Design Principles](#6-design-principles)
7. [User Personas and Jobs To Be Done](#7-user-personas-and-jobs-to-be-done)
8. [Core Product Surfaces](#8-core-product-surfaces)
9. [Protocol Foundation and Discovery Compatibility Strategy](#9-protocol-foundation-and-discovery-compatibility-strategy)
10. [Discovery Model](#10-discovery-model)
11. [Crawl Frontier, Scheduling, and Fetch Architecture](#11-crawl-frontier-scheduling-and-fetch-architecture)
12. [Parsing, Validation, and Normalization](#12-parsing-validation-and-normalization)
13. [Canonical Registry and Entity Resolution](#13-canonical-registry-and-entity-resolution)
14. [Search and Retrieval Architecture](#14-search-and-retrieval-architecture)
15. [Ranking System: AgentRank](#15-ranking-system-agentrank)
16. [Trust, Verification, and Abuse Resistance](#16-trust-verification-and-abuse-resistance)
17. [Liveness, Benchmarking, and Outcome Quality](#17-liveness-benchmarking-and-outcome-quality)
18. [Connection and Orchestration Plane](#18-connection-and-orchestration-plane)
19. [Enterprise, Federation, and Private Discovery](#19-enterprise-federation-and-private-discovery)
20. [Provider Experience: Agent Search Console](#20-provider-experience-agent-search-console)
21. [Public APIs and Protocol Contracts](#21-public-apis-and-protocol-contracts)
22. [Data Platform and Storage Architecture](#22-data-platform-and-storage-architecture)
23. [Service Architecture and Language Decisions](#23-service-architecture-and-language-decisions)
24. [Security Architecture](#24-security-architecture)
25. [Privacy, Governance, and Compliance](#25-privacy-governance-and-compliance)
26. [Observability, Evaluation, and Experimentation](#26-observability-evaluation-and-experimentation)
27. [Monetization and Business Model](#27-monetization-and-business-model)
28. [Rollout Plan and Team Topology](#28-rollout-plan-and-team-topology)
29. [Key Risks and Mitigations](#29-key-risks-and-mitigations)
30. [Category Strategy and Long-Term Moat](#30-category-strategy-and-long-term-moat)
31. [Opinionated Final Recommendation](#31-opinionated-final-recommendation)
32. [Immediate Next Design Artifacts](#32-immediate-next-design-artifacts)
33. [Appendix A: Canonical API Shapes](#33-appendix-a-canonical-api-shapes)
34. [Appendix B: Canonical Schema Guidance](#34-appendix-b-canonical-schema-guidance)
35. [Appendix C: Discovery Extensions to Standardize](#35-appendix-c-discovery-extensions-to-standardize)

---

## 1. Executive Summary

The agent ecosystem will not be organized by manual directories. It will be organized by the platform that can:

- discover agents automatically
- canonicalize fragmented metadata into a trustworthy registry
- rank agents by relevance, trust, compatibility, and demonstrated success
- connect agents safely with minimal friction
- learn from real outcomes and use those outcomes to improve future ranking

That platform is not merely a directory, a registry, or a search box. It is the **discovery and trust layer of the agentic internet**.

This document proposes a product and architecture for a market-winning A2A discovery platform with five integrated pillars:

1. **Discovery Engine**  
   Continuously finds public and permissioned A2A-compatible agents without relying on manual registration.

2. **Canonical Registry**  
   Builds a versioned, evidence-backed source of truth that merges duplicate records and preserves provenance.

3. **Search and Ranking Engine**  
   Supports hybrid lexical, semantic, graph, and policy-aware retrieval, then ranks by AgentRank.

4. **Trust and Verification Layer**  
   Measures identity, provenance, operational quality, protocol conformance, and abuse risk.

5. **Connection and Outcome Plane**  
   Helps the caller connect to the selected agent and captures outcome telemetry that feeds ranking, trust, and explainability.

The key strategic insight is this:

> Search alone is not enough. The market winner will own the closed loop from discovery to successful connection to measured outcome.

That closed loop becomes the moat:

`discovery -> ranking -> selection -> connection -> outcome -> trust -> better ranking`

This architecture is explicitly designed to win across both:

- the open public internet
- enterprise-private agent networks

Recommended stack:

- `Go` for the core online path, control plane, discovery services, and connection broker
- `Vespa` for large-scale hybrid retrieval and ranking
- `Kafka` for event streaming
- `Postgres` for canonical metadata and transactional control-plane state
- `ClickHouse` for analytics, telemetry, and ranking features
- `Object Storage` for raw evidence, snapshots, and audit artifacts
- `Python` only for offline ML, evaluation, labeling, and experimentation
- `TypeScript` for the provider console and admin UIs

If one main language must be chosen for the first 12 months, choose `Go`.

---

## 2. Strategic Thesis

### 2.1 The Category Will Be Won by a Search Company, a Trust Company, and a Network Company

The right mental model is not "build a marketplace."

The right mental model is:

- a **search company**, because discovery quality determines demand capture
- a **trust infrastructure company**, because spam, impersonation, and weak quality signals will otherwise destroy search quality
- a **protocol-native network company**, because value compounds when agent interactions and outcomes feed back into discovery

### 2.2 The Market Winner Will Not Depend on Manual Registration

Manual registration fails for the same reasons it failed on the web:

- incomplete coverage
- stale data
- high operator burden
- weak incentives to maintain accuracy
- easy bias toward incumbents or pay-to-play behavior

The platform must support optional claiming, verification, and optimization, but discovery itself must be autonomous.

### 2.3 The Moat Is Not Just the Index

A raw list of agents is not defensible.

The defensible asset is the combination of:

- crawl coverage
- canonical registry quality
- evidence graph
- trust graph
- benchmark history
- connection graph
- outcome graph
- provider optimization workflows
- enterprise federation footprint

### 2.4 The Winning Product Must Handle Public and Private Agents

If the platform only works on the public internet, it may become a useful search engine but not the control plane of the category.

If the platform can unify:

- public internet discovery
- partner ecosystem federation
- tenant-private enterprise registries
- scoped visibility and policy-aware search

then it becomes infrastructure, not just discovery.

---

## 3. Why Now

### 3.1 A2A Has Created the Right Root Primitive

Agent metadata is increasingly converging around well-known discovery artifacts and agent cards. That means discovery is now crawlable by design.

### 3.2 The Ecosystem Is Fragmenting Faster Than Standards Are Settling

This is the exact moment a platform opportunity appears:

- many agents
- many registries
- many frameworks
- inconsistent metadata
- no dominant search/ranking layer
- no authoritative trust system

### 3.3 Enterprises Need Discovery, but Also Need Control

Enterprise buyers will not adopt a discovery platform that cannot explain:

- why a result ranked highly
- how it was verified
- whether it is policy-compatible
- what data it can access
- whether the connection can be mediated or audited

### 3.4 Hyperscaler Risk Creates a Narrow Window

There is likely a 12-24 month window before:

- a hyperscaler bundles discovery into its broader agent stack
- a protocol body standardizes a basic default that reduces greenfield opportunity
- fragmented directories become sticky enough to be "good enough"

The product must move with the urgency of a category-defining infrastructure build.

---

## 4. Market Problem and Competitive Lens

### 4.1 The Five Core Problems

#### Problem 1: Discovery Is Manual or Tribal

Agents are found through:

- direct knowledge
- private docs
- Slack messages
- ad hoc config
- manually curated registries

This does not scale.

#### Problem 2: There Is No Reliable Quality Layer

Today, the ecosystem lacks mature ranking by:

- task fit
- trust
- operational quality
- policy compatibility
- actual success rate

#### Problem 3: Metadata Is Self-Reported and Inconsistent

Agent cards are necessary but insufficient.

Providers often:

- overclaim capabilities
- use inconsistent tags
- omit auth details
- publish stale endpoints
- duplicate agents across environments and versions

#### Problem 4: The Registry Model Creates Staleness

Static registries decay quickly. Agents go offline, change versions, rotate endpoints, or evolve skills.

#### Problem 5: Discovery Without Connection Has Limited Value

A good search result is not enough if:

- auth negotiation fails
- policy blocks usage
- the endpoint is down
- the agent is incompatible
- the selected agent underperforms in practice

### 4.2 The Real Competitive Landscape

Competitors will typically cluster into four buckets:

#### Bucket A: Manual Directories

Strength:

- easy to build

Weakness:

- weak coverage
- stale data
- low trust
- weak moat

#### Bucket B: Protocol Lookup Systems

Strength:

- standards-friendly

Weakness:

- weak ranking
- weak semantics
- weak quality differentiation

#### Bucket C: Search-Only Discovery Engines

Strength:

- better retrieval and relevance

Weakness:

- no connection control loop
- weaker outcome signals
- weaker enterprise leverage

#### Bucket D: Closed Ecosystem Marketplaces

Strength:

- strong distribution inside one platform

Weakness:

- poor cross-ecosystem neutrality
- lower openness
- weaker universal coverage

### 4.3 Winning Position

This platform should position itself as:

**the neutral discovery, trust, and connection layer for the agent internet**

That means:

- protocol-native
- registry-agnostic
- open to public and private deployment models
- explainable
- outcome-aware

---

## 5. Vision, Positioning, and Product Definition

### 5.1 Vision Statement

Build the default discovery and trust layer for the agentic internet: the place where agents, orchestrators, platforms, and enterprises go to find the best agent for a task, validate that it is trustworthy and compatible, and connect to it safely in seconds.

### 5.2 Product Definition

This product is **not**:

- just a directory
- just a registry
- just a benchmark site
- just a query API
- just a protocol gateway

This product **is**:

- a discovery engine
- a canonical registry
- a ranking engine
- a trust and reputation system
- a connection broker
- a policy-aware enterprise control plane
- a provider optimization platform

### 5.3 Brand Concepts

The external flagship concepts should be:

- **AgentRank**: the ranking and trust concept providers can understand
- **Agent Search Console**: the optimization interface providers use
- **Connect Broker**: the mediated connection plane for policy and auth
- **Verified Agent** and **Trusted Agent** tiers: user-visible trust markers

### 5.4 Product Promise

If a caller asks:

> "Find the best agent for this task under these constraints,"

the platform should answer:

- what exists
- what is most relevant
- what is compatible
- what is trustworthy
- what is currently healthy
- what is likely to succeed
- how to connect

---

## 6. Design Principles

### 6.1 Non-Negotiable Principles

1. **No manual registration required**  
   Discovery cannot depend on operator submission forms.

2. **Protocol-native first**  
   Use A2A artifacts and conventions as the primary discovery root.

3. **Compatibility over dogma**  
   Support multiple metadata conventions and evolving paths while standards stabilize.

4. **Evidence over claims**  
   Self-reported metadata informs ranking but never dominates it.

5. **Search engine, not catalog**  
   The architecture must support crawling, freshness, ranking, anti-spam, and evaluation.

6. **Trust-aware by default**  
   Relevance without trust becomes spam.

7. **Connection quality matters**  
   Discovery value is only realized if the selected agent can be used safely and successfully.

8. **Public and private by design**  
   Use one architecture that can serve internet-scale and enterprise-private discovery.

9. **Interpretability before black-box optimization**  
   Ranking must be explainable before it becomes highly learned.

10. **Build toward a compounding data moat**  
    Every product surface should increase coverage, trust, or outcome intelligence.

### 6.2 Product Rules

- Never let sponsored placement define the core organic ranking.
- Never let self-reported metadata outweigh verified evidence.
- Never require the provider dashboard for an agent to be discoverable.
- Never leak tenant-private ranking signals across tenants.
- Never proxy agent traffic by default when direct connection is sufficient.

---

## 7. User Personas and Jobs To Be Done

### 7.1 Calling Agent

An agent that needs another agent to complete a subtask.

Needs:

- low-latency search
- natural-language capability matching
- structured filters
- compatibility checks
- direct or brokered connect
- reliability and trust signals

### 7.2 Agent Orchestrator or Platform

A runtime or workflow engine that dynamically composes agents.

Needs:

- stable APIs
- high-QPS search
- result explainability
- outcome capture
- fallback suggestions
- policy-aware connect flows

### 7.3 Agent Provider

A team that publishes one or more A2A-compatible agents.

Needs:

- automatic discovery
- crawl/index visibility
- discoverability improvement guidance
- verification tools
- benchmark and trust visibility
- fair ranking without pay-to-play dependence

### 7.4 Enterprise Platform Team

A company running public and private agents across business units and geographies.

Needs:

- tenant-private indexing
- scoped visibility
- mTLS and enterprise auth support
- auditability
- region-aware search
- policy enforcement

### 7.5 Ecosystem Integrator

A cloud platform, marketplace, framework, or workflow vendor that wants discovery infrastructure.

Needs:

- embeddable APIs
- partner feeds
- federation contracts
- ranking and trust overlays
- analytics and reporting

### 7.6 Human Explorer

A developer or architect researching the agent ecosystem.

Needs:

- web UI
- comparison workflows
- trust badges
- documentation links
- benchmark summaries

---

## 8. Core Product Surfaces

### 8.1 Public Search API

Used by agents, orchestrators, apps, and partner platforms.

### 8.2 Connect API

Used to resolve and optionally broker an agent connection.

### 8.3 Registry Resolve API

Used to fetch the canonical record and evidence-backed metadata for a known agent.

### 8.4 Outcome API

Used to record trusted outcomes and connection telemetry.

### 8.5 Agent Search Console

Used by providers to claim domains, diagnose crawl/index issues, and improve discoverability.

### 8.6 Admin and Trust Console

Used internally for adjudication, abuse review, benchmark governance, and tenant management.

### 8.7 Enterprise Federation Connectors

Used to ingest private registries and expose restricted search views.

---

## 9. Protocol Foundation and Discovery Compatibility Strategy

### 9.1 The Discovery Root Is the Agent Card

The core discovery artifact is the A2A agent metadata document, commonly called the **Agent Card**.

The platform should treat the agent card as:

- the canonical origin document
- the root of first-party claims
- the starting point for validation and enrichment

It should **not** treat the agent card as the complete truth.

### 9.2 Resolve Current Spec and Convention Drift

Across the ecosystem, there may be multiple conventions such as:

- `/.well-known/agent-card.json`
- `/.well-known/agent.json`
- provider-linked metadata documents
- authenticated extended cards

The platform should support a compatibility matrix rather than betting on only one path.

### 9.3 Compatibility Strategy

#### Preferred Discovery Order

1. `/.well-known/agent-card.json`
2. `/.well-known/agent.json`
3. `Link` headers pointing to canonical agent metadata
4. provider documentation or manifest references
5. curated or federated registry references

#### Canonicalization Rule

Even if multiple metadata endpoints are found, the registry should establish a single canonical source hierarchy per agent version:

1. signed first-party well-known metadata
2. unsigned first-party well-known metadata
3. authenticated extended metadata
4. provider-linked manifests
5. third-party registry copies

### 9.4 Discovery Extensions to Standardize

The platform should help the ecosystem converge around several extensions:

- `agent-sitemap.xml` or `agent-sitemap.json`
- `agent-robots.txt`
- `agent-proof.json`
- DNS discovery hints such as `_a2a._tcp`
- signed federation feeds
- capability taxonomy documents
- signed outcome attestations

These should start as platform-supported conventions and gradually become de facto or formal standards.

---

## 10. Discovery Model

### 10.1 Discovery Sources

The system should ingest from at least six source classes.

#### 10.1.1 Direct A2A Well-Known Discovery

- well-known agent metadata endpoints
- authenticated extended card endpoints
- provider-declared canonical metadata URLs

#### 10.1.2 Registry Federation

- public registries
- ecosystem partner registries
- vertical directories
- enterprise-private registries

These are seed sources, not trust sources.

#### 10.1.3 Open Web Discovery

- provider homepages
- product pages
- documentation sites
- SDK docs
- GitHub and GitLab repositories
- package metadata
- OCI artifact metadata

#### 10.1.4 Infrastructure Signals

- DNS zone and passive DNS sources
- certificate transparency logs
- TLS certificate SAN relationships
- hosting platform patterns

#### 10.1.5 Graph and Referral Discovery

- provider cross-links
- links between agent docs
- known task delegations
- partner references

#### 10.1.6 Runtime and Outcome Signals

- health probes
- benchmark runs
- connect attempts
- successful outcomes
- structured failure telemetry

### 10.2 Discovery Leads vs Registration

Allow:

- user-submitted leads
- partner-submitted leads
- provider-submitted leads

But define clearly:

> A lead is not a listing.

A lead only enters the crawl frontier. It receives no ranking benefit unless discovery and validation succeed.

### 10.3 Discovery Pipeline

1. Collect seeds and leads
2. Normalize candidate URIs
3. Assign crawl priority
4. Fetch with politeness controls
5. Parse and validate artifacts
6. Extract structured and linked metadata
7. Canonicalize and merge entities
8. Enrich with embeddings, taxonomy, graph, and operational evidence
9. Write to registry, search index, evidence store, and analytics store
10. Schedule re-crawl and feedback loops

### 10.4 Strategic Discovery Insight

Discovery itself becomes a moat when the system learns:

- which sources reliably yield valid agents
- which patterns correlate with high-quality providers
- which metadata or host signals indicate spam
- which source types produce agents that succeed for certain tasks

---

## 11. Crawl Frontier, Scheduling, and Fetch Architecture

### 11.1 Frontier Goals

The crawl frontier must optimize for:

- coverage
- freshness
- cost efficiency
- politeness
- resilience
- abuse isolation

### 11.2 Frontier Partitioning

Partition the frontier by:

- provider or domain cluster
- visibility scope
- source type
- trust tier
- tenant

This supports:

- fair scheduling
- targeted backoff
- provider protection
- failure containment

### 11.3 Frontier Priority Score

Recommended v1 scheduling function:

`schedule_score =`

- `0.28 source_confidence`
- `0.18 expected_change_rate`
- `0.15 popularity_demand`
- `0.12 trust_priority`
- `0.10 recrawl_deadline_urgency`
- `0.07 link_or_referral_strength`
- `0.05 low_fetch_cost_probability`
- `0.05 benchmark_or_connect_relevance`

### 11.4 Fetch Rules

- respect caching headers
- use `ETag` and conditional requests
- maintain per-domain politeness budgets
- use adaptive retries and exponential backoff
- hash content for change detection
- maintain fetch provenance and TLS metadata

### 11.5 Crawl Politeness

Support:

- `robots.txt` semantics where applicable
- future `agent-robots.txt`
- `Retry-After` headers
- crawl-delay conventions
- provider opt-out for nonessential probes

### 11.6 Failure Handling

- `4xx`: lower confidence quickly unless clearly auth-gated
- `5xx`: exponential backoff
- timeouts: reduce budget
- malformed metadata: store evidence, retry later, penalize quality
- redirect loops: lower confidence sharply
- private address SSRF attempts: hard block and flag

### 11.7 Crawler Architecture

Recommended structure:

- `Seed Manager`
- `Frontier Scheduler`
- `Fetcher Pool`
- `Parser/Extractor`
- `Validation Service`
- `Evidence Writer`
- `Recrawl Planner`

Recommended implementation:

- stateless fetch workers in `Go`
- shared frontier state in transactional or streaming control plane
- event emission to `Kafka`

### 11.8 Scale Targets

Design targets:

- `10M+` canonical agents long term
- `100M+` versions and evidence objects
- `50k+` fetches/sec potential scale envelope
- median discovery for new public agents from known high-confidence seeds under `15 minutes`

---

## 12. Parsing, Validation, and Normalization

### 12.1 Validation Layers

Every fetched artifact should pass through several validation stages.

#### Schema Validation

- required fields
- field types
- field length constraints
- MIME modes and URL validity

#### Protocol Conformance Validation

- A2A card semantics
- interaction capability consistency
- auth declaration consistency

#### Operational Validation

- endpoint reachability
- transport compatibility
- TLS correctness
- supported method checks where safe

#### Semantic Validation

- duplicated or contradictory skills
- obviously spammy descriptions
- impossible combinations

### 12.2 Normalization Goals

Normalize:

- URLs
- provider identities
- agent names
- version identifiers
- auth scheme names
- skill aliases
- capability labels
- regions and compliance tags

### 12.3 Important Rule

Maintain both:

- the **raw source payload**
- the **normalized canonical representation**

Never destroy raw evidence.

### 12.4 Extraction Outputs

Extract at minimum:

- card fields
- endpoint metadata
- provider metadata
- auth schemes
- skills and examples
- docs and repo references
- benchmark references
- signatures and proofs
- outgoing links to related agents or assets

---

## 13. Canonical Registry and Entity Resolution

### 13.1 Registry Purpose

The canonical registry is the product's backbone. It answers:

- what agents exist
- which records refer to the same agent
- which version is current
- what evidence supports each claim
- what trust tier applies
- which view is visible to which principal

### 13.2 Core Entities

- `Provider`
- `ProviderIdentity`
- `Agent`
- `AgentVersion`
- `Endpoint`
- `Skill`
- `CapabilityProfile`
- `AuthProfile`
- `Evidence`
- `HealthSnapshot`
- `BenchmarkRun`
- `ConnectionOutcome`
- `ReviewSignal`
- `PolicyProfile`
- `VisibilityScope`

### 13.3 Entity Resolution Strategy

Entity resolution is one of the highest-value technical systems in the company.

Use a layered approach:

#### Strong Signals

- same well-known source URI after normalization
- same verified signing identity
- same endpoint URL after normalization
- same verified provider domain

#### Medium Signals

- same docs repo lineage
- same auth endpoints
- same package or OCI lineage
- same examples and skill structure

#### Weak Signals

- similar names
- similar descriptions
- similar tags

### 13.4 Resolution Policy

1. deterministic matching first
2. candidate generation using similarity search
3. ML-assisted merge scoring second
4. low-confidence merges held apart
5. early-stage human adjudication for critical or high-value conflicts

### 13.5 Evidence Model

Every significant claim should be traceable to evidence.

Evidence types:

- fetched artifact
- DNS proof
- signature proof
- benchmark result
- health probe result
- outcome event
- third-party citation
- provider verification proof

### 13.6 Visibility Model

Support:

- `Public`
- `Tenant Private`
- `Partner Restricted`
- `Internal`

This allows one canonical architecture for all discovery modes.

---

## 14. Search and Retrieval Architecture

### 14.1 Retrieval Philosophy

Use multi-stage retrieval:

1. query understanding
2. candidate generation
3. eligibility filtering
4. lightweight scoring
5. heavy re-ranking
6. diversity and policy adjustment
7. connection readiness packaging

### 14.2 Query Understanding

The query system should infer:

- intent type
- vertical/domain
- capability requirements
- input and output modes
- auth constraints
- policy requirements
- geography or data residency
- latency and price constraints

### 14.3 Candidate Generation Modes

#### Lexical Retrieval

Search across:

- names
- descriptions
- skills
- examples
- docs
- taxonomy labels

#### Semantic Retrieval

Use embeddings over:

- agent descriptions
- skill descriptions
- examples
- docs summaries
- schema summaries

#### Graph Retrieval

Use graph expansion over:

- provider relationships
- invocation graph
- workflow co-occurrence
- taxonomy proximity
- citations and recommendations

#### Constraint Retrieval

Retrieve agents already known to satisfy strict structured constraints:

- auth
- modality
- compliance
- region
- SLA
- tenant scope

### 14.4 Result Fusion

Use parallel retrieval with robust fusion such as:

- Reciprocal Rank Fusion in v1
- score-aware learned fusion in later phases

### 14.5 Re-Ranking

Use a multi-layer re-ranker:

- transparent heuristic ranker in v1
- feature-based learning-to-rank in v2
- contextual re-ranking in v3

### 14.6 Explainability

Every result should return compact reasons such as:

- matched capabilities
- trust tier
- last verified
- auth compatibility
- benchmark summary
- main ranking factors
- policy exclusion or suppression reasons if relevant

Explainability is mandatory for enterprise trust and ranking debugging.

---

## 15. Ranking System: AgentRank

### 15.1 AgentRank Concept

`AgentRank` should be the external-facing concept that turns discovery into an ecosystem discipline, similar to search quality and SEO on the web.

It should combine:

- task relevance
- compatibility
- trust
- operational quality
- authority
- freshness
- economic efficiency
- demonstrated outcomes
- documentation quality

### 15.2 Two Different Scores

Do not confuse these:

#### A. Search Ranking Score

Used at query time to order results.

#### B. Discoverability Score

Provider-facing score used in Search Console to help improve crawlability and metadata quality.

### 15.3 Top-Level Ranking Components

- `Task Match`
- `Compatibility`
- `Trust`
- `Operational Quality`
- `Authority`
- `Freshness`
- `Outcome Success`
- `Economic Efficiency`
- `Documentation Quality`
- `Diversity Adjustment`

### 15.4 Initial Transparent Formula

Recommended v1 query-time score:

`final_score = eligibility_gate * trust_floor * weighted_sum`

Where:

- `eligibility_gate` is binary or near-binary
- `trust_floor` suppresses low-trust results without always excluding them

Recommended v1 `weighted_sum`:

- `0.25 task_match`
- `0.15 compatibility`
- `0.14 trust`
- `0.11 operational_quality`
- `0.10 outcome_success`
- `0.08 authority`
- `0.07 freshness`
- `0.05 economic_efficiency`
- `0.05 documentation_quality`

### 15.5 Signal Families

#### Task Match

- lexical query match
- semantic similarity
- example alignment
- taxonomy overlap
- I/O mode fit

#### Compatibility

- auth support
- transport fit
- modality fit
- sync vs async fit
- enterprise policy fit

#### Trust

- domain verification
- signed metadata
- provenance confidence
- provider reputation
- abuse risk

#### Operational Quality

- uptime
- p95 latency
- error rate
- consistency
- successful handshake rate

#### Outcome Success

- connect success rate
- task completion rate
- repeat usage
- low abandonment
- signed or trusted success attestations

#### Authority

- provider authority
- graph centrality
- trusted citations
- workflow inclusion

#### Freshness

- card recency
- evidence freshness
- docs recency
- recrawl freshness

#### Economic Efficiency

- price per successful outcome
- integration friction
- latency-adjusted quality

#### Documentation Quality

- examples
- schema clarity
- auth clarity
- error semantics
- completeness

### 15.6 Discoverability Score

Expose a provider-facing optimization score such as:

- metadata completeness
- skill specificity
- examples quality
- crawlability
- canonicalization hygiene
- proof coverage
- docs quality

This becomes the "Agent SEO" mechanism.

### 15.7 Abuse Resistance

Never let raw self-declared metadata dominate ranking.

Core anti-gaming mechanisms:

- duplicate-content detection
- provider clustering
- Sybil detection
- authority propagation rate limits
- anomaly detection on traffic spikes
- suspicious review or outcome clustering
- trust decay for unverified claims

### 15.8 Learning-to-Rank Roadmap

#### Phase 1

Transparent heuristic ranker.

#### Phase 2

Offline LTR using:

- judged query sets
- click/connect data
- benchmark labels
- successful invocation labels

#### Phase 3

Contextual ranking using:

- tenant preferences
- user or agent context
- vertical priors
- policy posture

### 15.9 Important Ranking Principle

Do not jump to black-box ranking too early.

If the team cannot explain:

- why a result ranked
- why one result outranked another
- why a trusted provider was suppressed

then the system is not mature enough for a highly learned primary ranker.

---

## 16. Trust, Verification, and Abuse Resistance

### 16.1 Threat Model

Assume from day one:

- fake agents
- cloned cards
- misleading capability claims
- spam farms
- malicious endpoints
- fake outcome events
- fake reviews
- impersonation of known providers
- auth phishing

### 16.2 Trust Layers

#### Identity Trust

- domain verification
- org verification
- signing identity verification
- DNS or cert proofs

#### Artifact Trust

- signed metadata
- provenance attestations
- immutable version hashes
- transparency logs where available

#### Operational Trust

- health history
- benchmark reproducibility
- uptime
- protocol conformance

#### Ecosystem Trust

- references from trusted providers
- real adoption
- low abuse history
- stable long-term presence

### 16.3 Trust Tiers

Recommended example:

- `Indexed`
- `Established`
- `Verified`
- `Trusted`
- `Authoritative`
- `Suspicious`

Trust tiers should influence:

- ranking
- UI badges
- policy filters
- enterprise defaults

### 16.4 Verification Methods

- DNS TXT verification
- well-known proof file
- signed `agent-proof.json`
- verified code/repo linkage
- enterprise tenant identity assertions

### 16.5 Abuse Controls

- duplicate suppression
- domain-age and provider-age weighting
- provider cluster rate limits
- anomaly detection
- manual review queue for severe trust incidents
- quarantine mode for suspect agents

### 16.6 Security Controls

- HTTPS required for public discovery
- mTLS for private federation where needed
- sandboxed benchmark and probe execution
- signed internal telemetry events
- tamper-evident audit logs
- hard SSRF protections in crawlers and probes

---

## 17. Liveness, Benchmarking, and Outcome Quality

### 17.1 Liveness Is Multi-Level

Use layered probing:

- `L1` TCP or TLS reachability
- `L2` metadata fetch and validation
- `L3` protocol handshake
- `L4` safe capability probe

### 17.2 Status State Machine

States:

- `New`
- `Healthy`
- `Degraded`
- `Unhealthy`
- `Dead`
- `Delisted`

The product should demote or delist gracefully rather than abruptly where appropriate.

### 17.3 Capability Verification

Use safe, non-destructive tests to verify that an agent can do what it claims.

Rules:

- tests must be reproducible
- tests must be low-risk
- providers can see benchmark results
- provider-submitted results must be labeled separately from platform-run results

### 17.4 Benchmark Types

- protocol conformance
- capability benchmarks
- reliability benchmarks
- security benchmarks
- integration benchmarks

### 17.5 Outcome Telemetry

This is one of the biggest moat builders.

Capture:

- search shown
- search selected
- connect attempted
- connect succeeded
- auth failed
- timed out
- task completed
- user or orchestrator rejected output

### 17.6 Trustworthiness of Outcome Signals

Outcome events should be classified by trust level:

- platform-observed
- broker-observed
- partner-signed
- provider-reported
- self-asserted

Higher-trust events should carry more ranking weight.

---

## 18. Connection and Orchestration Plane

### 18.1 Why Connection Matters

Many competitors will stop at search. That leaves major value, data, and moat untapped.

The connection plane creates:

- lower-friction adoption
- better auth negotiation
- policy enforcement
- observability
- fallback routing
- outcome capture

### 18.2 Connection Modes

#### Direct Connect

The platform returns a result and the caller connects directly.

Best for:

- simple public agents
- low-friction auth
- minimal mediation

#### Brokered Connect

The platform mediates setup.

Best for:

- auth exchange
- policy enforcement
- tenant routing
- observability
- retries and failover

### 18.3 Broker Responsibilities

- resolve final endpoint
- validate compatibility
- negotiate auth path
- enforce policy
- create session metadata
- optionally relay or proxy if required
- emit outcome telemetry

### 18.4 Strategic Constraint

The platform should not force every interaction through a relay. That would increase friction and reduce openness.

Instead:

- broker where it adds value
- step out of path where direct use is better

### 18.5 Connect Decision Flow

1. Caller submits task and constraints
2. Search returns ranked candidates
3. Policy and compatibility filters narrow the set
4. Broker decides direct vs mediated path
5. Auth path is negotiated
6. Session descriptor is returned or session is created
7. Outcomes are captured

---

## 19. Enterprise, Federation, and Private Discovery

### 19.1 Why Enterprise Support Is Essential

If the product cannot support private discovery, it will be important but not central.

Enterprise support is what turns discovery into a control plane.

### 19.2 Federation Modes

#### Pull Federation

The platform crawls a partner or tenant registry with scoped credentials.

#### Push Federation

The tenant publishes signed change events.

#### Hybrid Federation

Use push for freshness and pull for verification.

This hybrid model is the recommended default.

### 19.3 Enterprise Requirements

- mTLS
- tenant-scoped auth
- scoped visibility
- audit trails
- region-aware indexing
- private ranking features isolated from public data
- tenant-level policy packs

### 19.4 Public vs Private Ranking

Maintain separation where appropriate:

- public popularity should not dominate enterprise-private results
- private success signals must not leak across tenants

### 19.5 Access Control Model

Every search request should resolve:

- caller identity
- tenant context
- visibility scope
- permitted ranking data
- allowed connect modes

---

## 20. Provider Experience: Agent Search Console

### 20.1 Why This Matters

This is not a nice-to-have. It is a flywheel builder.

Without provider visibility:

- discovery remains opaque
- metadata quality improves slowly
- providers feel powerless
- ecosystem standards evolve more slowly

### 20.2 Key Features

- domain claiming and verification
- discovered card inspection
- crawl history
- indexing status
- discoverability score
- trust tier and missing proofs
- benchmark status
- ranking opportunities
- auth and compatibility warnings
- canonicalization conflict reports

### 20.3 Search Console Outcomes

The console should tell providers:

- why they are or are not discoverable
- why they are underperforming
- what evidence is missing
- what metadata is weak
- what policy or compatibility gaps exist

### 20.4 Ecosystem Benefit

This creates the equivalent of an SEO market for agents, but rooted in quality rather than manipulation.

---

## 21. Public APIs and Protocol Contracts

### 21.1 API-First Strategy

The platform should be API-first.

The UI is important, but long-term market power will come from being embedded in:

- agents
- orchestrators
- partner platforms
- cloud products
- enterprise runtime layers

### 21.2 Core APIs

- `POST /v1/search`
- `GET /v1/agents/{id}`
- `POST /v1/connect`
- `POST /v1/outcomes`
- `POST /v1/leads`
- `GET /v1/providers/{id}`

### 21.3 Search API Requirements

Input should support:

- natural-language query
- structured filters
- caller context
- tenant context
- result mode: `search`, `recommend`, `connect-ready`

Output should include:

- ranked results
- explainability payloads
- compatibility info
- trust tier
- freshness and health info
- connect metadata

### 21.4 Connect API Requirements

Should:

- accept agent selection or search result reference
- validate policy and trust requirements
- broker auth if needed
- return a connection-ready session descriptor

### 21.5 Outcome API Requirements

Should:

- accept signed or trusted outcome events
- classify evidence level
- support aggregation rather than full sensitive payload storage

### 21.6 A2A-Native Interface

The platform itself should be an A2A-discoverable agent.

That matters for:

- dogfooding
- protocol legitimacy
- agent-native adoption

---

## 22. Data Platform and Storage Architecture

### 22.1 Storage Roles

#### `Vespa`

Use for:

- hybrid retrieval
- multi-stage ranking
- low-latency serving

#### `Postgres`

Use for:

- canonical registry
- workflow state
- tenant config
- policy metadata

#### `Kafka`

Use for:

- crawl events
- parse events
- health signals
- benchmark events
- connect outcomes
- feature updates

#### `ClickHouse`

Use for:

- telemetry
- analytics
- ranking features
- dashboards
- experiment analysis

#### `Object Storage`

Use for:

- raw artifacts
- snapshots
- signatures
- benchmark logs
- exports

### 22.2 Why Not Start with a Dedicated Graph Database

Graph traversal is important, but a graph database should not be a default dependency unless it becomes central to the product surface.

Recommended approach:

- store graph edges in relational or columnar form
- compute graph features in stream or batch jobs
- feed those features into the ranking layer

### 22.3 Data Lineage

Every ranking-significant field should be explainable in terms of:

- source
- timestamp
- parser version
- confidence
- last verification

This is non-negotiable for trust and debugging.

---

## 23. Service Architecture and Language Decisions

### 23.1 Strong Recommendation

Use a polyglot architecture with sharp boundaries:

- `Go` for core online services and discovery
- `Python` for offline ML, embeddings, labeling, and evaluation
- `TypeScript` for provider/admin UI
- `Rust` selectively for specialized low-latency or safety-critical components only if justified

### 23.2 Why `Go` for the Core

Use `Go` for:

- API gateway
- search API service
- discovery control plane
- crawl scheduler
- fetch workers
- verification services
- connect broker
- federation connectors

Reasons:

- strong concurrency
- excellent HTTP/networking
- simpler team ramp
- strong operational maturity
- faster iteration than forcing a Rust-only posture

### 23.3 Why Not Python for the Core Online Path

Python is strong for:

- ML
- experimentation
- evaluation
- offline pipelines

But it is weaker for the latency-sensitive, high-concurrency control plane and fetcher path.

### 23.4 Why Not Over-Commit to Rust

Rust may be appropriate in select components, but forcing it everywhere early can slow:

- hiring
- product iteration
- scope changes
- integration work

The ambition here is to win the market, not win a language purity contest.

### 23.5 Service Boundaries

Recommended major services:

- `Search API`
- `Query Understanding Service`
- `Discovery Control Plane`
- `Fetch and Extraction Workers`
- `Validation Service`
- `Registry Service`
- `Ranking Service`
- `Trust Service`
- `Benchmark Service`
- `Connect Broker`
- `Federation Service`
- `Provider Console Backend`

---

## 24. Security Architecture

### 24.1 Threat Model Areas

- untrusted internet input
- crawler abuse
- search API abuse
- score manipulation
- provider impersonation
- fake telemetry
- cross-tenant data leakage
- SSRF and internal network reachability

### 24.2 Security Controls

- hard deny private-IP SSRF from crawlers
- body size limits
- timeouts and redirect limits
- same-origin redirect policies where needed
- mTLS for internal and enterprise-sensitive flows
- API auth tiers
- signed internal events
- secret isolation from indexed metadata

### 24.3 Auth and Access

#### External APIs

- anonymous limited search
- API key or bearer for full developer access
- enterprise auth for private features

#### Internal Services

- service identity
- mTLS
- least-privilege credentials

### 24.4 Auditability

All critical decisions should be auditable:

- ranking changes
- trust tier changes
- provider verification changes
- tenant visibility changes
- manual abuse interventions

---

## 25. Privacy, Governance, and Compliance

### 25.1 Data Classes

- public metadata
- tenant-private metadata
- benchmark artifacts
- connect telemetry
- outcome summaries
- signed proofs

### 25.2 Privacy Principles

- avoid storing full task payloads unless required
- prefer structured outcome summaries
- separate sensitive raw data from ranking features
- support tenant retention controls

### 25.3 Retention Guidance

- raw fetch artifacts: short to medium retention
- canonical records and versions: long-lived
- benchmark artifacts used for trust decisions: long-lived
- outcome telemetry: retain at privacy-preserving resolution where required

### 25.4 Governance

Establish governance for:

- benchmark suites
- trust tier criteria
- abuse adjudication
- provider appeals
- standards proposals

---

## 26. Observability, Evaluation, and Experimentation

### 26.1 Core Metrics

#### Discovery

- new agents discovered per day
- valid agent cards discovered
- crawl success rate
- duplicate suppression rate
- median discovery latency

#### Search

- QPS
- p50, p95, p99 latency
- zero-result rate
- result diversity
- offline NDCG, MRR, Recall@K

#### Connect

- connect success rate
- auth failure rate
- median time to connect
- fallback rate

#### Trust

- spoofing incidents
- abuse cluster rate
- signed coverage
- benchmark reproducibility rate

### 26.2 Evaluation Framework

You need:

- golden query sets
- vertical-specific judged queries
- benchmark-driven labels
- online A/B or interleaving tests
- post-connect success labels

### 26.3 Experimentation Rules

- always preserve a baseline
- instrument every rank-affecting change
- ship explainability before major automation
- require offline wins before online rollout where possible

---

## 27. Monetization and Business Model

### 27.1 Core Revenue Streams

#### API Access

The most natural first revenue stream.

#### Enterprise Discovery and Federation

High-value product for private/public hybrid discovery.

#### Provider Analytics and Search Console Premium

Advanced diagnostics, benchmarking, and competitive insights.

#### Brokered Connect Premium Features

Policy packs, auth mediation, audit workflows, and SLA-backed connection flows.

### 27.2 What Not to Do Early

- do not let ads or sponsorship corrupt core ranking
- do not anchor the business on marketplace take rates before discovery quality is strong
- do not optimize revenue before trust and relevance are clearly differentiated

### 27.3 Strategic Monetization Position

The company should monetize:

- infrastructure
- trust
- enterprise control
- intelligence

Not ranking manipulation.

---

## 28. Rollout Plan and Team Topology

### 28.1 Phase 0: Foundations

Build:

- canonical schema
- discovery crawler
- parser and validator
- basic registry
- public search API

Goal:

- automatically discover and index public agents

### 28.2 Phase 1: Search Quality

Build:

- hybrid retrieval
- AgentRank v1
- Search Console v1
- benchmark framework

Goal:

- materially better search than tag matching or flat registries

### 28.3 Phase 2: Trust and Connect

Build:

- domain verification
- signed metadata support
- connect broker
- outcome telemetry loop

Goal:

- trusted discovery plus successful connection

### 28.4 Phase 3: Enterprise Federation

Build:

- tenant-private registries
- mTLS and private federation
- policy-aware search
- scoped visibility

Goal:

- become the default discovery layer for enterprise agent networks

### 28.5 Phase 4: Learning Network Effects

Build:

- learned ranking
- workflow graph signals
- outcome-driven recommendations
- proactive discovery suggestions

Goal:

- best-in-class ranking moat

### 28.6 Initial Team Pods

- `Discovery Pod`
- `Registry and Trust Pod`
- `Search and Ranking Pod`
- `Connect and Federation Pod`
- `Provider Experience Pod`

---

## 29. Key Risks and Mitigations

### Risk 1: Sparse Ecosystem Early

Mitigation:

- ingest from multiple discovery sources
- federate from existing registries
- expose Search Console early

### Risk 2: Spam and Fake Agents

Mitigation:

- trust floor
- strong canonicalization
- proof support
- anomaly detection

### Risk 3: Poor Ranking Quality

Mitigation:

- hybrid retrieval from day one
- transparent ranking first
- benchmark-driven evaluation
- outcome loops

### Risk 4: Over-Building Too Early

Mitigation:

- phase aggressively
- design interfaces ahead of implementation depth
- avoid shipping enterprise-only complexity into public MVP

### Risk 5: Standards Churn

Mitigation:

- compatibility matrix
- artifact versioning
- discovery abstraction layer

### Risk 6: Enterprise Trust Bar Too High

Mitigation:

- explainable ranking
- policy-aware search
- audit trails
- private/public signal isolation

---

## 30. Category Strategy and Long-Term Moat

### 30.1 The Moat Is a Compounding Loop, Not a Feature

The moat is the system that gets smarter every time the network uses it:

`more discovery coverage`
-> `better candidate recall`
-> `better ranking`
-> `better selections`
-> `higher connection success`
-> `more trusted outcomes`
-> `better trust and ranking`
-> `more demand routed through the platform`

### 30.2 Moat Components

- best crawl coverage
- best canonical registry
- best trust graph
- best outcome graph
- best enterprise federation footprint
- best provider optimization console
- best explainability

### 30.3 What Competitors Will Miss

Most competitors will stop at one of these:

- directory
- registry
- search engine
- benchmark lab
- enterprise catalog

The winner will integrate all of them without losing coherence.

---

## 31. Opinionated Final Recommendation

If the goal is to win the market, the right approach is:

1. Treat the A2A agent card as the discovery root, but support compatibility across evolving conventions.
2. Build a real search engine, not a static directory.
3. Make `AgentRank` the ecosystem concept providers optimize against.
4. Build a canonical registry with evidence-backed entity resolution.
5. Make trust, provenance, and abuse resistance first-class product features.
6. Add a provider-facing `Agent Search Console` early.
7. Close the loop with a connection broker and outcome telemetry.
8. Support enterprise-private federation early enough that the platform becomes infrastructure, not just public search.
9. Use `Go + Vespa + Kafka + Postgres + ClickHouse + Object Storage`, with `Python` only for offline ML and evaluation.
10. Optimize for interpretability first, then learning systems second.

This gives the best balance of:

- speed to market
- technical depth
- operational realism
- enterprise readiness
- long-term defensibility

---

## 32. Immediate Next Design Artifacts

The next documents to produce after this one should be:

1. `Agent Card canonical schema and normalization spec`
2. `Discovery compatibility and well-known endpoint resolution spec`
3. `Search API and Connect API contract`
4. `AgentRank v1 feature dictionary and scoring guide`
5. `Discovery frontier and recrawl design`
6. `Trust, proof, and verification spec`
7. `Agent Search Console PRD`
8. `Enterprise federation architecture`
9. `Outcome telemetry and signed event spec`
10. `Benchmark governance and conformance suite spec`

---

## 33. Appendix A: Canonical API Shapes

### A.1 `POST /v1/search`

#### Request

```json
{
  "query": "Find the best agent to review Terraform changes for security and compliance",
  "filters": {
    "auth_schemes": ["oauth2"],
    "regions": ["us-east-1", "us-west-2"],
    "min_trust_tier": "verified",
    "supports_streaming": true
  },
  "context": {
    "tenant_id": "acme",
    "caller_type": "agent",
    "industry": "enterprise",
    "latency_preference": "balanced",
    "budget_preference": "quality_first"
  },
  "top_k": 10,
  "mode": "connect-ready"
}
```

#### Response

```json
{
  "query_id": "qry_123",
  "results": [
    {
      "agent_id": "agt_abc",
      "version_id": "ver_2026_03_23",
      "provider_id": "prov_xyz",
      "score": 0.934,
      "trust_tier": "verified",
      "health": {
        "status": "healthy",
        "last_verified_at": "2026-03-23T18:20:00Z"
      },
      "compatibility": {
        "a2a": true,
        "auth_match": true,
        "policy_match": true,
        "supported_modalities": ["text", "json"]
      },
      "highlights": [
        "Strong Terraform security review capability match",
        "Supports OAuth2 and structured JSON outputs",
        "High benchmark and connect success rate"
      ],
      "explanations": {
        "top_reasons": [
          "High task relevance",
          "Verified provider and strong uptime",
          "Strong outcome success on similar tasks"
        ]
      },
      "connect": {
        "mode": "brokered",
        "endpoint": "https://agent.example.com/a2a",
        "auth_profile_id": "auth_001"
      }
    }
  ]
}
```

### A.2 `GET /v1/agents/{agent_id}`

Returns:

- canonical metadata
- best current version
- evidence summary
- trust and health summary
- visibility-scoped details

### A.3 `POST /v1/connect`

```json
{
  "agent_id": "agt_abc",
  "caller_identity": {
    "tenant_id": "acme",
    "agent_id": "caller_789"
  },
  "task_context": {
    "goal": "review infrastructure plan for policy violations",
    "requires_async": true
  }
}
```

### A.4 `POST /v1/outcomes`

```json
{
  "query_id": "qry_123",
  "agent_id": "agt_abc",
  "event_type": "task_completed",
  "trust_level": "broker_observed",
  "summary": {
    "success": true,
    "latency_ms": 1820
  }
}
```

### A.5 `POST /v1/leads`

```json
{
  "url": "https://newagent.example.com",
  "source": "partner_feed"
}
```

Important:

- a lead is not a registration
- the URL enters crawl validation
- no ranking boost is granted automatically

---

## 34. Appendix B: Canonical Schema Guidance

### B.1 Canonical Agent Fields

- `agent_id`
- `provider_id`
- `canonical_name`
- `primary_domain`
- `status`
- `visibility_scope`
- `current_version_id`
- `capability_categories`
- `primary_modalities`
- `supported_auth_schemes`
- `trust_tier`
- `authority_score`
- `discoverability_score`
- `operational_score`
- `freshness_score`
- `last_verified_at`
- `evidence_summary`

### B.2 Agent Version Fields

- `version_id`
- `source_uri`
- `source_type`
- `card_hash`
- `fetched_at`
- `declared_version`
- `service_endpoint`
- `skills`
- `auth_profiles`
- `capabilities`
- `cache_headers`
- `validation_status`
- `provenance_status`

### B.3 Skill Model

Each skill should include:

- stable skill id
- title
- normalized description
- aliases
- examples
- input modes
- output modes
- capability taxonomy labels
- embedding references

### B.4 Evidence Model

Each evidence item should include:

- evidence id
- evidence type
- collection time
- source URI
- content hash
- signer identity if present
- parser version
- confidence score

### B.5 Outcome Model

- `outcome_id`
- `query_id`
- `agent_id`
- `caller_identity_scope`
- `event_type`
- `trust_level`
- `success`
- `latency_ms`
- `recorded_at`
- `evidence_uri`

---

## 35. Appendix C: Discovery Extensions to Standardize

### C.1 `agent-sitemap`

Purpose:

- list all agents and versions for a provider
- speed up discovery
- improve freshness

### C.2 `agent-robots`

Purpose:

- communicate crawl policy for agent search systems
- express path rules and preferred budgets

### C.3 `agent-proof.json`

Purpose:

- bind agent metadata to verified provider identity
- support provenance and anti-impersonation

### C.4 DNS Discovery Hints

Examples:

- `_a2a._tcp.example.com`
- TXT record pointing to canonical discovery metadata

Purpose:

- bootstrap discovery quickly
- help private and enterprise deployments

### C.5 Signed Outcome Attestations

Purpose:

- let trusted actors publish verifiable success or failure events
- enrich ranking and trust with stronger evidence

### C.6 Capability Taxonomy Documents

Purpose:

- standardize capability categories and aliases
- improve query understanding
- improve deduplication and ranking consistency

---

## Final Strategic Note

If this company wants to win the category, it should behave less like a marketplace startup and more like:

- a search company
- a trust infrastructure company
- and a protocol-native network company

That mindset will drive the right product boundaries, architecture choices, and moat formation.
