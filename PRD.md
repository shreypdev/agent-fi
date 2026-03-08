# AgentFi — Product Requirements Document

**The Compliance, Observability, and Trust Infrastructure for the Multi-Agent Economy**

Version: 0.2.0
Date: March 8, 2026
Status: Draft

---

## 1. Vision

Every autonomous AI agent interaction — across any transport, any framework, any organization — is observable, authenticated, authorized, and auditable through AgentFi.

We are building the governance layer for the agent economy: the infrastructure that makes multi-agent systems trustworthy enough for production, compliant enough for regulators, and transparent enough for the humans whose intent they carry.

**One-liner:** AgentFi is the Datadog + Auth0 for AI agents — unified observability and identity, transport-agnostic, compliance-native.

---

## 2. Problem Statement

Multi-agent systems are moving from demos to production. Companies are deploying chains of agents that search, decide, delegate, and transact on behalf of humans. Three fundamental problems block enterprise adoption:

### 2.1 Blindness (The Observability Gap)

When a chain of 5 agents produces a bad outcome, there is no unified way to trace what happened across independent agent services.

Consider a real scenario: a user asks their personal agent to book a trip. The personal agent delegates to a travel planner, which fans out to flight, hotel, and restaurant agents. The trip gets booked but the hotel is in the wrong city. Where did the failure happen?

- Was the user's request ambiguous?
- Did the personal agent misparse the city?
- Did the travel planner send the wrong city to the hotel agent?
- Did the hotel agent return results for a different city?

Today, there is **no tool** that shows the full message flow across all 5 agents in a single view. Existing tools trace LLM calls *inside* a single agent (LangSmith, LangFuse) but cannot trace *between* agents running as separate services.

This is the distributed tracing problem for agents — the same problem that Zipkin, Jaeger, and Datadog APM solved for microservices, but nobody has solved for autonomous agents.

### 2.2 Lawlessness (The Identity Gap)

Agents call other agents with no standardized identity, authorization, or boundary enforcement.

Consider what happens today when Agent A calls Agent B over A2A:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "message/send",
  "params": {
    "message": {
      "kind": "message",
      "messageId": "msg-001",
      "role": "user",
      "parts": [{ "kind": "text", "text": "Book a flight to Rome for $2000" }]
    }
  }
}
```

Agent B has **zero information** about:

- Who is Agent A? Is it really who it claims to be?
- Who authorized Agent A to book flights?
- What is Agent A's spending limit?
- Is there a human behind this request, and did they consent?
- If Agent A misbehaves, who is accountable?

The message carries no identity, no authorization, no delegation proof. Any agent can impersonate any other agent. Any agent can claim any capability. There are no guardrails.

### 2.3 Liability (The Compliance Gap)

The EU AI Act (Regulation 2024/1689, enforcement: August 2026) mandates specific requirements for high-risk AI systems:

**Article 12 — Record-Keeping:**
> "High-risk AI systems shall technically allow for the automatic recording of events ('logs') over the lifetime of the system."

Logs must enable: identifying risks, facilitating post-market monitoring, and monitoring system operation. For systems in Annex III 1(a), logs must record: period of use, reference databases, input data leading to matches, and identification of persons involved in verification.

**Article 13 — Transparency:**
> "High-risk AI systems shall be designed and developed in such a way as to ensure that their operation is sufficiently transparent to enable deployers to interpret a system's output and use it appropriately."

**Article 14 — Human Oversight:**
> "High-risk AI systems shall be designed and developed in such a way, including with appropriate human-machine interface tools, that they can be effectively overseen by natural persons."

**The penalty for non-compliance: up to €35 million or 7% of global annual turnover** — whichever is higher.

No existing product provides a unified compliance surface for multi-agent systems that satisfies all three articles. AgentFi does.

---

## 3. Target Users

### 3.1 Primary: Agent Developer

**Profile:** Software engineer building multi-agent systems with A2A, MCP, LangGraph, CrewAI, or AutoGen.

**Pain points:**
- "My 5-agent chain produced wrong results and I spent 3 hours adding console.log statements to figure out where it broke"
- "I have no way to see the full message flow across agents"
- "When an agent is slow, I can't tell if it's the agent itself or a downstream dependency"

**What AgentFi gives them:**
- One-line middleware integration → full traces in a dashboard
- Waterfall view showing every agent-to-agent call with timing and payloads
- Error highlighting that pinpoints exactly which agent in the chain failed

**Buying motion:** Bottom-up. Discovers via npm / GitHub / Hacker News. Starts with free tier. Champions internally.

### 3.2 Primary: Platform Engineer

**Profile:** Engineer responsible for the infrastructure that multiple agent teams deploy on.

**Pain points:**
- "We have 12 internal agents and no way to enforce who can call what"
- "Agent team A accidentally called Agent team B's staging endpoint in production"
- "We need to rate-limit our expensive agents but have no infrastructure for it"

**What AgentFi gives them:**
- Agent registry with capability declarations and constraints
- Middleware that enforces authorization on every call
- Per-agent rate limiting and spend tracking

**Buying motion:** Adopts after developer team starts using Trace. Expands to Shield for governance.

### 3.3 Expansion: CISO / Security Lead

**Profile:** Responsible for security posture of AI agent deployments.

**Pain points:**
- "We can't prove that agent X was authorized to perform action Y"
- "If an agent is compromised, we have no way to trace what it accessed"
- "Our auditors are asking about AI agent controls and we have nothing"

**What AgentFi gives them:**
- Cryptographic identity for every agent
- Full delegation chain from human authorization to agent action
- Audit trail with tamper-evident logging

### 3.4 Expansion: Compliance Officer

**Profile:** Responsible for EU AI Act compliance.

**Pain points:**
- "We need to demonstrate Article 12/13/14 compliance by August 2026"
- "Our AI agents have no logging, transparency, or human oversight mechanisms"
- "The €35M fine is keeping our legal team up at night"

**What AgentFi gives them:**
- Compliance dashboard mapping system state to EU AI Act articles
- Exportable audit reports for regulators
- Human-in-the-loop approval workflows with full audit trail

---

## 4. Competitive Landscape

### 4.1 Observability Competitors — Detailed Analysis

#### Spanora

- **What they do:** OpenTelemetry-native AI agent observability. Trace visualization (Gantt timeline), LLM cost tracking, prompt inspection, tool monitoring.
- **Pricing:** Free tier (1,000 spans/month, 24h retention). Paid tiers available.
- **Strengths:** Framework-agnostic (LangChain, Vercel AI SDK, raw OTEL). Span-level cost attribution. SDK-optional (raw OTLP works).
- **Critical gaps:**
  - No identity/authorization context on spans
  - No concept of delegation chains or agent ownership
  - No cross-organization tracing
  - No compliance reporting
  - Focused on LLM-based agents; limited value for non-LLM agent-to-agent flows

#### Traceloop

- **What they do:** Local-first AI observability. Real-time dashboard. Privacy-focused (traces stay on-machine).
- **Strengths:** Open source. Privacy-first design. Developer-friendly.
- **Critical gaps:**
  - Single-process focus — doesn't trace across agent services
  - No agent-to-agent semantic understanding
  - No identity integration
  - No compliance features
  - Local-only limits enterprise adoption

#### Trace (London, $3M seed, Feb 2026)

- **What they do:** Knowledge graphs for agent orchestration and observability. Maps corporate environments. Automates agent onboarding.
- **Strengths:** Well-funded. Enterprise focus. Knowledge graph approach is differentiated.
- **Critical gaps:**
  - Proprietary approach (not based on OpenTelemetry)
  - Not transport-agnostic
  - No identity/trust layer
  - Focus is on agent orchestration, not governance

#### LangSmith / LangFuse

- **What they do:** LLM call tracing. Prompt management. Evaluation frameworks.
- **Strengths:** Large user base. Mature product. Deep LLM-specific features.
- **Critical gaps:**
  - Traces inside one agent, not between agents
  - No understanding of A2A tasks, MCP tools, or agent cards
  - Useless for non-LLM agent-to-agent communication
  - No identity or compliance features

### 4.2 Identity Competitors — Detailed Analysis

#### Vouched (Agent Checkpoint)

- **What they do:** Agent identification, permissioning, delegation via OAuth scopes. Launched Feb 2026.
- **Strengths:** Production-ready. OAuth integration. Cryptographic proofs. Audit trails. Already seeing real agent traffic (0.5-16% of customer traffic).
- **Critical gaps:**
  - No observability — can't trace agent interactions
  - Closed-source — limits developer adoption
  - Not aligned with IETF standards (A-JWT, DAAP)
  - No cross-agent trace correlation
  - No expense/spend tracking at the protocol level

#### Beyond Identity (Ceros)

- **What they do:** Agentic AI trust layer. Hardware-vaulted API keys. Identity binding to verified humans/devices. Real-time policy enforcement.
- **Strengths:** Enterprise-grade security. Hardware root of trust. Established company.
- **Critical gaps:**
  - Enterprise-heavy (not developer-friendly)
  - No agent-to-agent delegation chains
  - No observability integration
  - Device-binding doesn't map well to cloud-native agents

#### OpenAgents (AgentID)

- **What they do:** Cryptographic identity using W3C DIDs and X.509 certificates. Three verification levels.
- **Strengths:** Open source. Standards-based (W3C DID). Lightweight.
- **Critical gaps:**
  - Identity-only — no capability enforcement, no expense limits
  - No observability
  - No delegation chain verification
  - No middleware for A2A or MCP integration

#### Agent Passport Protocol

- **What they do:** Open-source credential system. Ed25519 signatures. Delegation chains. Reputation scoring.
- **Strengths:** Open source. Ed25519 (same choice as us). Delegation chains. Zero heavy dependencies.
- **Critical gaps:**
  - Early stage (v1.0 just shipped)
  - No middleware for A2A/MCP
  - No observability
  - No expense/spend tracking
  - No compliance reporting

### 4.3 Feature Matrix

```
Feature                          │ AgentFi │ Spanora │ Traceloop │ Vouched │ OpenAgents │ Passport
─────────────────────────────────┼─────────┼─────────┼───────────┼─────────┼────────────┼─────────
Cross-agent trace waterfall      │   ✅    │   ⚠️¹   │    ❌     │   ❌    │     ❌     │   ❌
Agent-to-agent semantic spans    │   ✅    │   ❌    │    ❌     │   ❌    │     ❌     │   ❌
A2A protocol support             │   ✅    │   ❌    │    ❌     │   ❌    │     ❌     │   ❌
MCP protocol support             │   ✅    │   ⚠️²   │    ❌     │   ❌    │     ❌     │   ❌
Cryptographic agent identity     │   ✅    │   ❌    │    ❌     │   ✅    │     ✅     │   ✅
Delegation chains                │   ✅    │   ❌    │    ❌     │   ⚠️³   │     ❌     │   ✅
Expense/spend tracking           │   ✅    │   ❌    │    ❌     │   ❌    │     ❌     │   ❌
Capability enforcement           │   ✅    │   ❌    │    ❌     │   ✅    │     ❌     │   ❌
Human-in-the-loop approval       │   ✅    │   ❌    │    ❌     │   ❌    │     ❌     │   ❌
EU AI Act compliance reporting   │   ✅    │   ❌    │    ❌     │   ❌    │     ❌     │   ❌
OpenTelemetry compatible         │   ✅    │   ✅    │    ✅     │   ❌    │     ❌     │   ❌
Open-source core                 │   ✅    │   ❌    │    ✅     │   ❌    │     ✅     │   ✅
Transport-agnostic               │   ✅    │   ⚠️⁴   │    ❌     │   ❌    │     ❌     │   ❌
Unified observability + identity │   ✅    │   ❌    │    ❌     │   ❌    │     ❌     │   ❌

¹ Spanora traces LLM calls, not agent-to-agent protocol calls
² Spanora supports some MCP via OTEL but lacks deep MCP semantic understanding
³ Vouched has delegation scopes via OAuth but not cryptographic chain verification
⁴ Spanora is OTEL-based but doesn't understand agent protocol semantics
```

**Bottom line:** AgentFi is the only product that checks every box because it is the only product designed from the start to unify observability and identity for multi-agent systems.

---

## 5. Product Architecture

### 5.1 System Overview

```
┌───────────────────────────────────────────────────────────────────────────┐
│                           Agent Ecosystem                                  │
│                                                                           │
│  ┌──────────┐    A2A     ┌──────────┐    MCP     ┌──────────┐           │
│  │ Agent A   │──────────►│ Agent B   │──────────►│ Agent C   │           │
│  │           │◄──────────│           │◄──────────│           │           │
│  │ @agentfi/ │           │ @agentfi/ │           │ @agentfi/ │           │
│  │  trace    │           │  trace    │           │  trace    │           │
│  │  shield   │           │  shield   │           │  shield   │           │
│  └─────┬─────┘           └─────┬─────┘           └─────┬─────┘           │
│        │                       │                       │                  │
└────────┼───────────────────────┼───────────────────────┼──────────────────┘
         │    OTLP + Shield      │                       │
         │    Events             │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                                 ▼
                    ┌────────────────────────┐
                    │   AgentFi Collector     │
                    │                        │
                    │  ┌──────────────────┐  │
                    │  │ OTLP Receiver    │  │
                    │  │ (spans, metrics) │  │
                    │  └────────┬─────────┘  │
                    │           │             │
                    │  ┌────────▼─────────┐  │
                    │  │ Shield Receiver  │  │
                    │  │ (identity events)│  │
                    │  └────────┬─────────┘  │
                    │           │             │
                    │  ┌────────▼─────────┐  │
                    │  │ Correlation      │  │
                    │  │ Engine           │  │
                    │  │ (joins trace +   │  │
                    │  │  identity data)  │  │
                    │  └────────┬─────────┘  │
                    │           │             │
                    │  ┌────────▼─────────┐  │
                    │  │ Storage          │  │
                    │  │ (SQLite/Postgres)│  │
                    │  └────────┬─────────┘  │
                    └───────────┼─────────────┘
                                │
              ┌─────────────────┼─────────────────┐
              │                 │                  │
              ▼                 ▼                  ▼
    ┌─────────────────┐ ┌────────────┐ ┌────────────────┐
    │ AgentFi          │ │ AgentFi    │ │ AgentFi        │
    │ Registry         │ │ Query API  │ │ Dashboard      │
    │                  │ │            │ │ (React SPA)    │
    │ - Agent CRUD     │ │ - Traces   │ │                │
    │ - A-JWT Issuer   │ │ - Spans    │ │ - Waterfall    │
    │ - JWKS endpoint  │ │ - Agents   │ │ - Call Graph   │
    │ - Delegation mgr │ │ - Metrics  │ │ - Agent Mgmt   │
    │ - Spend tracker  │ │ - Alerts   │ │ - Compliance   │
    └─────────────────┘ └────────────┘ └────────────────┘
```

### 5.2 Monorepo Structure

```
agentfi/
├── packages/
│   ├── common/                     # Shared TypeScript types and utilities
│   │   ├── src/
│   │   │   ├── types/
│   │   │   │   ├── span.ts         # AgentFiSpan, AgentInfo, RedactedPayload
│   │   │   │   ├── agent.ts        # RegisteredAgent, AgentCapability
│   │   │   │   ├── token.ts        # AgentFiJWT, DelegationLink
│   │   │   │   ├── events.ts       # ShieldEvent, TraceEvent
│   │   │   │   └── index.ts
│   │   │   ├── constants.ts        # Version, default config
│   │   │   └── utils/
│   │   │       ├── redaction.ts     # PII redaction patterns
│   │   │       └── crypto.ts       # Ed25519 helpers
│   │   └── package.json            # @agentfi/common
│   │
│   ├── trace-sdk/                  # @agentfi/trace — open source
│   │   ├── src/
│   │   │   ├── middleware/
│   │   │   │   ├── a2a.ts          # A2A Express middleware
│   │   │   │   ├── mcp.ts          # MCP middleware
│   │   │   │   └── http.ts         # Generic HTTP middleware
│   │   │   ├── client/
│   │   │   │   ├── a2a.ts          # Traced A2A client wrapper
│   │   │   │   └── mcp.ts          # Traced MCP client wrapper
│   │   │   ├── propagation/
│   │   │   │   ├── w3c.ts          # W3C Trace Context propagation
│   │   │   │   └── context.ts      # Trace context management
│   │   │   ├── exporter/
│   │   │   │   ├── otlp.ts         # OTLP HTTP exporter
│   │   │   │   └── console.ts      # Console exporter (dev)
│   │   │   └── index.ts            # Public API
│   │   └── package.json
│   │
│   ├── shield-sdk/                 # @agentfi/shield — open source
│   │   ├── src/
│   │   │   ├── middleware/
│   │   │   │   ├── verify.ts       # A-JWT verification middleware
│   │   │   │   └── enforce.ts      # Capability & constraint enforcement
│   │   │   ├── client/
│   │   │   │   ├── shielded-a2a.ts # A-JWT-attaching A2A client
│   │   │   │   └── registry.ts     # Registry API client
│   │   │   ├── token/
│   │   │   │   ├── issuer.ts       # A-JWT token creation
│   │   │   │   ├── verifier.ts     # A-JWT token verification
│   │   │   │   └── delegation.ts   # Delegation chain verification
│   │   │   ├── crypto/
│   │   │   │   └── ed25519.ts      # Key generation, signing, verification
│   │   │   └── index.ts
│   │   └── package.json
│   │
│   ├── collector/                  # AgentFi Collector service
│   │   ├── src/
│   │   │   ├── receivers/
│   │   │   │   ├── otlp.ts         # OTLP HTTP receiver
│   │   │   │   └── shield.ts       # Shield event receiver
│   │   │   ├── storage/
│   │   │   │   ├── sqlite.ts       # SQLite adapter
│   │   │   │   ├── postgres.ts     # Postgres adapter
│   │   │   │   └── interface.ts    # Storage interface
│   │   │   ├── correlation/
│   │   │   │   └── engine.ts       # Join trace + identity data
│   │   │   ├── api/
│   │   │   │   ├── traces.ts       # Trace query endpoints
│   │   │   │   ├── spans.ts        # Span query endpoints
│   │   │   │   ├── metrics.ts      # Metrics query endpoints
│   │   │   │   └── alerts.ts       # Alert configuration
│   │   │   └── index.ts
│   │   ├── Dockerfile
│   │   └── package.json
│   │
│   ├── registry/                   # AgentFi Registry service
│   │   ├── src/
│   │   │   ├── api/
│   │   │   │   ├── agents.ts       # Agent CRUD
│   │   │   │   ├── tokens.ts       # A-JWT issuance
│   │   │   │   ├── delegations.ts  # Delegation management
│   │   │   │   ├── spend.ts        # Expense tracking
│   │   │   │   └── jwks.ts         # JWKS endpoint
│   │   │   ├── auth/
│   │   │   │   └── oauth.ts        # Owner verification (Google, GitHub)
│   │   │   ├── storage/
│   │   │   │   ├── sqlite.ts
│   │   │   │   └── interface.ts
│   │   │   └── index.ts
│   │   ├── Dockerfile
│   │   └── package.json
│   │
│   └── dashboard/                  # AgentFi Dashboard (React SPA)
│       ├── src/
│       │   ├── pages/
│       │   │   ├── traces/          # Trace list, waterfall, call graph
│       │   │   ├── agents/          # Agent registry, management
│       │   │   ├── delegations/     # Delegation chain viewer
│       │   │   ├── spend/           # Expense tracking dashboard
│       │   │   ├── compliance/      # EU AI Act compliance view
│       │   │   └── alerts/          # Alert configuration and history
│       │   └── components/
│       │       ├── Waterfall.tsx
│       │       ├── CallGraph.tsx
│       │       ├── SpanDetail.tsx
│       │       └── DelegationChain.tsx
│       └── package.json
│
├── agent-demo/                     # Existing A2A travel demo
│   └── ...                         # (already built)
│
├── examples/
│   ├── basic-trace/                # Minimal trace integration example
│   ├── basic-shield/               # Minimal shield integration example
│   └── full-travel-demo/           # Travel demo with full AgentFi integration
│
├── docs/                           # Documentation site source
├── PRD.md                          # This document
├── turbo.json
├── pnpm-workspace.yaml
└── package.json
```

### 5.3 Transport Abstraction Layer

AgentFi SDKs operate at the transport layer. Each supported transport gets an adapter that knows how to:
1. Intercept inbound/outbound messages
2. Inject/extract trace context
3. Inject/extract identity tokens
4. Capture agent-specific semantic attributes

#### A2A Transport Adapter

**Trace context location:** `message.metadata["traceparent"]` and `message.metadata["tracestate"]`

**Identity token location:** `message.metadata["x-agentfi-token"]`

Example A2A message with AgentFi headers injected:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "message/send",
  "params": {
    "message": {
      "kind": "message",
      "messageId": "msg-abc-123",
      "role": "user",
      "parts": [
        { "kind": "text", "text": "Find flights to Rome" }
      ],
      "metadata": {
        "traceparent": "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        "tracestate": "agentfi=span:a1b2c3d4",
        "x-agentfi-token": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwczovL3JlZ2lzdHJ5LmFnZW50Zmkua..."
      }
    },
    "configuration": { "blocking": true }
  }
}
```

#### MCP Transport Adapter

**Trace context location:** `params._meta.traceparent` and `params._meta.tracestate` (per OpenTelemetry MCP semantic conventions)

**Identity token location:** `params._meta["x-agentfi-token"]`

Example MCP tool call with AgentFi headers:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "search_flights",
    "arguments": {
      "destination": "Rome",
      "date": "2026-04-01"
    },
    "_meta": {
      "traceparent": "00-4bf92f3577b34da6a3ce929d0e0e4736-d4e5f6a7b8c90000-01",
      "tracestate": "agentfi=span:e5f6a7b8",
      "x-agentfi-token": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9..."
    }
  }
}
```

#### HTTP Transport Adapter

**Trace context location:** Standard `traceparent` / `tracestate` HTTP headers (W3C Trace Context)

**Identity token location:** `Authorization: Bearer <A-JWT>` header

```http
POST /api/search-flights HTTP/1.1
Host: expedia-agent.example.com
Content-Type: application/json
Authorization: Bearer eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9...
traceparent: 00-4bf92f3577b34da6a3ce929d0e0e4736-f0e1d2c3b4a50000-01
tracestate: agentfi=span:f0e1d2c3

{"destination": "Rome", "date": "2026-04-01"}
```

---

## 6. Core Data Models

### 6.1 Trace Span

```typescript
interface AgentFiSpan {
  // === Identification ===
  traceId: string;                  // W3C trace ID (32 hex chars), propagated across all agents
  spanId: string;                   // Unique ID for this interaction (16 hex chars)
  parentSpanId?: string;            // The span that triggered this one

  // === Participants ===
  source: AgentInfo;                // The calling agent
  target: AgentInfo;                // The receiving agent

  // === Transport ===
  transport: "a2a" | "mcp" | "http" | "grpc";

  // === A2A-specific attributes ===
  a2a?: {
    method: string;                 // "message/send", "tasks/get", "tasks/cancel"
    taskId?: string;                // A2A task ID
    taskState?: TaskState;          // "submitted" | "working" | "completed" | "failed" | ...
    skillId?: string;               // Which skill was invoked (from agent card)
    contextId?: string;             // A2A context ID
  };

  // === MCP-specific attributes ===
  mcp?: {
    method: string;                 // "tools/call", "resources/read", "prompts/get"
    toolName?: string;              // Name of the MCP tool called
    resourceUri?: string;           // URI of the MCP resource accessed
  };

  // === Payloads ===
  request: RedactedPayload;         // Inbound message/request (with PII redaction applied)
  response?: RedactedPayload;       // Response (with PII redaction applied)

  // === Timing ===
  startedAt: string;                // ISO 8601 timestamp
  completedAt?: string;             // ISO 8601 timestamp
  latencyMs?: number;               // Computed: completedAt - startedAt

  // === Status ===
  status: "ok" | "error";
  error?: {
    code: string;                   // Error code (e.g., "CAPABILITY_DENIED", "TIMEOUT")
    message: string;                // Human-readable error description
    a2aErrorCode?: number;          // JSON-RPC error code if A2A
  };

  // === Identity correlation (from Shield) ===
  shield?: {
    tokenId?: string;               // A-JWT jti (links to specific token)
    agentId?: string;               // Verified agent ID from registry
    ownerId?: string;               // Agent's owner ID
    delegationChainHash?: string;   // SHA-256 of the delegation chain
    delegationDepth?: number;       // How many hops from the original human
    verified: boolean;              // Was identity verification attempted and successful?
    policyResult: "allow" | "deny" | "unverified";
    denialReason?: string;          // If denied: why? (e.g., "capability_not_authorized")
    spendAmount?: number;           // If this action had a cost, how much?
    spendCurrency?: string;
    remainingBudget?: number;       // Remaining budget after this action
  };

  // === OpenTelemetry compatibility ===
  otel?: {
    serviceName: string;            // Maps to OTEL service.name
    serviceVersion?: string;
    attributes: Record<string, string | number | boolean>;
  };
}

interface AgentInfo {
  agentId?: string;                 // From agent registry (if registered)
  name: string;                     // Human-readable name (from agent card or config)
  url?: string;                     // Agent's endpoint URL
  organizationId?: string;          // Organization that operates this agent
  agentCardUrl?: string;            // URL to agent's A2A Agent Card
}

interface RedactedPayload {
  raw?: string;                     // Original payload (if redaction is disabled)
  summary: string;                  // Short summary (e.g., "text message: 'Find flights to Rome'")
  parts?: RedactedPart[];           // Parsed parts with redaction applied
  redactedFields?: string[];        // List of field paths that were redacted
  sizeBytes: number;                // Original payload size
}

interface RedactedPart {
  kind: "text" | "file" | "data";
  text?: string;                    // Text content (with PII replaced by [REDACTED])
  fileName?: string;
  dataSchema?: string;              // JSON schema of data part
}
```

### 6.2 Agent Identity (Registry)

```typescript
interface RegisteredAgent {
  // === Core identity ===
  agentId: string;                  // Unique ID (UUID v4), immutable after creation
  name: string;                     // Human-readable name
  description: string;              // What this agent does
  version: string;                  // Agent's own version string

  // === Ownership ===
  owner: AgentOwner;

  // === Cryptographic identity ===
  publicKey: string;                // Base64-encoded Ed25519 public key
  keyAlgorithm: "Ed25519";
  keyFingerprint: string;           // SHA-256 of the public key (for quick matching)

  // === Capabilities (what this agent can do) ===
  capabilities: AgentCapability[];

  // === External references ===
  agentCardUrl?: string;            // URL to A2A Agent Card
  mcpManifestUrl?: string;          // URL to MCP server manifest
  documentationUrl?: string;
  sourceCodeUrl?: string;           // Git repo URL (for auditability)

  // === Status ===
  status: "active" | "suspended" | "revoked";
  statusReason?: string;            // Why the agent was suspended/revoked
  statusChangedAt?: string;

  // === Timestamps ===
  registeredAt: string;             // ISO 8601
  updatedAt: string;
  lastSeenAt?: string;              // Last time this agent made or received an A-JWT call
  lastTokenIssuedAt?: string;

  // === Metadata ===
  tags?: string[];                  // For filtering/grouping (e.g., ["production", "travel"])
  metadata?: Record<string, string>;
}

interface AgentOwner {
  type: "user" | "organization";
  id: string;                       // External ID from identity provider
  email?: string;                   // Contact email
  name?: string;                    // Display name
  verifiedVia: string;              // "google-oauth" | "github" | "saml" | "api-key"
  verifiedAt: string;               // When ownership was last verified
}

interface AgentCapability {
  skillId: string;                  // Unique skill identifier (e.g., "search_flights")
  name: string;                     // Human-readable name (e.g., "Search Flights")
  description?: string;

  actions: string[];                // Specific actions allowed:
                                    //   ["search"] — read-only
                                    //   ["search", "book"] — can read and write
                                    //   ["search", "book", "cancel"] — full access
                                    //   ["*"] — all actions (admin)

  constraints: CapabilityConstraints;
}

interface CapabilityConstraints {
  // === Spending limits ===
  maxSpendPerAction?: number;       // Max spend for a single action (in minor units, e.g., cents)
  maxSpendPerDay?: number;          // Daily spending cap
  maxSpendPerMonth?: number;        // Monthly spending cap
  currency?: string;                // ISO 4217 currency code (e.g., "USD", "EUR")

  // === Rate limiting ===
  rateLimit?: {
    maxRequests: number;            // Max requests allowed
    windowMs: number;               // Time window in milliseconds
  };

  // === Geographic restrictions ===
  allowedRegions?: string[];        // ISO 3166-1 alpha-2 country codes or region names

  // === Temporal restrictions ===
  allowedTimeWindows?: {
    timezone: string;               // IANA timezone (e.g., "America/New_York")
    windows: Array<{
      dayOfWeek: number[];          // 0=Sunday, 6=Saturday
      startHour: number;            // 0-23
      endHour: number;
    }>;
  };

  // === Human oversight ===
  requireHumanApproval?: boolean;   // All actions require human approval
  humanApprovalThreshold?: number;  // Actions above this spend amount require approval
  humanApprovalTimeout?: number;    // Seconds to wait for approval before rejecting

  // === Data access ===
  allowedDataTypes?: string[];      // MIME types this agent can access
  prohibitedFields?: string[];      // Field paths that must be redacted
}
```

### 6.3 A-JWT Token (Agentic JWT)

This is AgentFi's implementation of the Agentic JWT specification (draft-goswami-agentic-jwt-00).

#### Token Header

```json
{
  "alg": "EdDSA",
  "typ": "JWT",
  "kid": "agentfi:registry:key-2026-03"
}
```

#### Token Payload (Claims)

```typescript
interface AgentFiJWTPayload {
  // === Standard JWT claims (RFC 7519) ===
  iss: string;                      // Issuer: AgentFi Registry URL
                                    //   e.g., "https://registry.agentfi.dev"
  sub: string;                      // Subject: Agent ID (the agent this token represents)
                                    //   e.g., "agent:travel-planner-004"
  aud: string | string[];           // Audience: Target agent ID(s) this token is valid for
                                    //   e.g., "agent:expedia-001" or ["agent:expedia-001", "agent:marriott-002"]
  exp: number;                      // Expiry: Unix timestamp (short-lived: 5-15 minutes)
  iat: number;                      // Issued At: Unix timestamp
  nbf?: number;                     // Not Before: Unix timestamp
  jti: string;                      // JWT ID: Unique token identifier for revocation tracking

  // === A-JWT claims (per draft-goswami-agentic-jwt-00) ===
  agent_checksum: string;           // SHA-256 hash of agent's configuration:
                                    //   SHA256(system_prompt + tool_definitions + config_params)
                                    //   This ensures the agent hasn't been tampered with since registration.
                                    //   If the checksum doesn't match what's in the registry, the token is rejected.

  // === AgentFi extension claims ===
  agentfi: {
    version: "1.0";                 // AgentFi claims version

    // --- Identity ---
    owner_id: string;               // Owner of this agent (user or org ID)
    organization_id?: string;       // Organization (if applicable)
    agent_name: string;             // Human-readable agent name

    // --- Authorized capabilities ---
    capabilities: string[];         // Skill IDs this token authorizes
                                    //   e.g., ["search_flights", "search_hotels"]
    actions: string[];              // Specific actions allowed
                                    //   e.g., ["search"]  (NOT ["book"])

    // --- Constraints (enforced by Shield middleware) ---
    constraints: {
      max_spend?: number;           // Maximum spend allowed for this token's lifetime
      currency?: string;            // Currency for spend limit
      rate_limit?: number;          // Max requests allowed with this token
      regions?: string[];           // Allowed regions
      expires_at: string;           // ISO 8601 (redundant with exp, but human-readable)
    };

    // --- Delegation chain ---
    delegation_chain: DelegationLink[];

    // --- Step tracking (per A-JWT spec) ---
    step_sequence_hash?: string;    // SHA-256 of the sequence of workflow steps executed so far
                                    //   Updated as each agent in the chain processes the request
                                    //   Enables detecting if steps were skipped or reordered
  };
}
```

#### Delegation Chain

The delegation chain is an ordered array of cryptographically signed links, each representing one hop in the authorization chain from human to the current agent.

```typescript
interface DelegationLink {
  from: string;                     // Delegating entity ID
                                    //   First link: "user:shrey@example.com" (human)
                                    //   Subsequent links: "agent:personal-agent-001" (agent)
  to: string;                       // Receiving entity ID
                                    //   e.g., "agent:travel-planner-004"
  permissions: string[];            // What is being delegated
                                    //   e.g., ["search_flights", "search_hotels", "book_hotel"]
  constraints?: {                   // Constraints applied at this delegation hop
    max_spend?: number;             //   Can only narrow (reduce), never widen
    currency?: string;
    expires_at?: string;
    regions?: string[];
  };
  granted_at: string;               // ISO 8601 timestamp
  expires_at: string;               // ISO 8601 timestamp
  nonce: string;                    // Random nonce for replay protection (UUID v4)
  signature: string;                // Ed25519 signature of canonical(from + to + permissions + constraints + granted_at + expires_at + nonce)
                                    //   Signed by the `from` entity's private key
}
```

#### Full A-JWT Example

Here is a complete, decoded A-JWT that would flow from TravelPlannerAgent to ExpediaAgent in our travel demo:

```json
{
  "header": {
    "alg": "EdDSA",
    "typ": "JWT",
    "kid": "agentfi:registry:key-2026-03"
  },
  "payload": {
    "iss": "https://registry.agentfi.dev",
    "sub": "agent:travel-planner-004",
    "aud": "agent:expedia-001",
    "exp": 1741473600,
    "iat": 1741472700,
    "jti": "tok_a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "agent_checksum": "sha256:9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",

    "agentfi": {
      "version": "1.0",
      "owner_id": "user:shrey@example.com",
      "organization_id": "org:agentfi-demo",
      "agent_name": "Travel Planner Agent",

      "capabilities": ["search_flights"],
      "actions": ["search"],

      "constraints": {
        "max_spend": 0,
        "currency": "USD",
        "rate_limit": 100,
        "regions": ["US", "EU"],
        "expires_at": "2026-03-08T21:00:00Z"
      },

      "delegation_chain": [
        {
          "from": "user:shrey@example.com",
          "to": "agent:personal-agent-001",
          "permissions": ["search_flights", "search_hotels", "recommend_restaurants", "book_flights", "book_hotels"],
          "constraints": {
            "max_spend": 500000,
            "currency": "USD",
            "expires_at": "2026-03-09T00:00:00Z"
          },
          "granted_at": "2026-03-08T20:00:00Z",
          "expires_at": "2026-03-09T00:00:00Z",
          "nonce": "d4e5f6a7-b8c9-0000-1111-222233334444",
          "signature": "base64:K7gNU3sdo+OL0wNhqoVWhr3g6s1xYv72ol/pe/Unols="
        },
        {
          "from": "agent:personal-agent-001",
          "to": "agent:travel-planner-004",
          "permissions": ["search_flights", "search_hotels", "recommend_restaurants"],
          "constraints": {
            "max_spend": 500000,
            "currency": "USD",
            "expires_at": "2026-03-09T00:00:00Z"
          },
          "granted_at": "2026-03-08T20:00:05Z",
          "expires_at": "2026-03-09T00:00:00Z",
          "nonce": "e5f6a7b8-c9d0-1111-2222-333344445555",
          "signature": "base64:X8hOV4tso+PM1xOirqpXXis4h7t2yYw83pm/qf/Vopm="
        }
      ],

      "step_sequence_hash": "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    }
  },
  "signature": "base64:Y9iPW5uso+QN2yPjsrqYYjt5i8u3zZx94qn/rf/Wqpn=..."
}
```

#### A-JWT Verification Flow

When ExpediaAgent receives a request with an A-JWT, the Shield middleware performs these checks in order:

```
Step 1: Parse & decode JWT
  └─ Reject if malformed

Step 2: Verify signature (EdDSA / Ed25519)
  └─ Fetch public key from JWKS endpoint (cached)
  └─ Reject if signature invalid

Step 3: Check standard claims
  ├─ exp: Is the token expired? → Reject
  ├─ nbf: Is it too early? → Reject
  ├─ iss: Is the issuer our trusted registry? → Reject
  └─ aud: Is this token addressed to ME? → Reject

Step 4: Verify agent checksum
  └─ Look up agent in registry by sub (agent ID)
  └─ Compare agent_checksum in token vs. checksum in registry
  └─ Reject if mismatch (agent was tampered with)

Step 5: Check capabilities
  └─ Does the token's capabilities list include the skill being invoked?
  └─ Does the token's actions list include the action being performed?
  └─ Reject if not authorized

Step 6: Enforce constraints
  ├─ Rate limit: Has this agent exceeded its request quota? → Reject
  ├─ Spend limit: Would this action exceed the remaining budget? → Reject
  ├─ Region: Is this request from an allowed region? → Reject
  └─ Time window: Is this within allowed hours? → Reject

Step 7: Verify delegation chain
  ├─ First link must start from a verified human (user:*)
  ├─ Each link's `to` must match the next link's `from`
  ├─ Last link's `to` must match the token's `sub`
  ├─ Verify each link's Ed25519 signature
  ├─ Check that permissions only narrow (never widen) at each hop
  ├─ Check that no link is expired
  └─ Check that no link has been revoked (query registry)

Step 8: Allow request ✅
  └─ Attach verified identity to the request context
  └─ Emit shield verification event to collector
```

---

## 7. SDK Integration Examples

### 7.1 Adding Trace to an A2A Agent (Server Side)

Before AgentFi:

```typescript
import express from "express";
import { DefaultRequestHandler, InMemoryTaskStore } from "@a2a-js/sdk/server";
import { A2AExpressApp } from "@a2a-js/sdk/server/express";

const handler = new DefaultRequestHandler(agentCard, new InMemoryTaskStore(), executor);
const app = new A2AExpressApp(handler).setupRoutes(express());
app.listen(3001);
```

After AgentFi (one import, one line):

```typescript
import express from "express";
import { DefaultRequestHandler, InMemoryTaskStore } from "@a2a-js/sdk/server";
import { A2AExpressApp } from "@a2a-js/sdk/server/express";
import { agentfiTrace } from "@agentfi/trace";                      // ← add this

const handler = new DefaultRequestHandler(agentCard, new InMemoryTaskStore(), executor);
const app = express();
app.use(agentfiTrace({                                               // ← add this
  collectorUrl: "http://localhost:4318",
  agentName: "expedia-agent",
}));
new A2AExpressApp(handler).setupRoutes(app);
app.listen(3001);
```

### 7.2 Adding Trace to an A2A Client (Caller Side)

Before AgentFi:

```typescript
import { A2AClient } from "@a2a-js/sdk/client";

const client = await A2AClient.fromCardUrl("http://localhost:3001/.well-known/agent-card.json");
const response = await client.sendMessage({ message: userMessage });
```

After AgentFi:

```typescript
import { createTracedClient } from "@agentfi/trace";                // ← change import

const client = await createTracedClient(                             // ← change call
  "http://localhost:3001/.well-known/agent-card.json",
  { collectorUrl: "http://localhost:4318", agentName: "travel-planner" }
);
const response = await client.sendMessage({ message: userMessage }); // same API
```

The traced client automatically:
- Generates a span for the outbound call
- Injects `traceparent` into `message.metadata`
- Records the response and latency
- Exports the span to the collector

### 7.3 Adding Shield to an A2A Agent (Server Side)

```typescript
import express from "express";
import { DefaultRequestHandler, InMemoryTaskStore } from "@a2a-js/sdk/server";
import { A2AExpressApp } from "@a2a-js/sdk/server/express";
import { agentfiTrace } from "@agentfi/trace";
import { agentfiShield } from "@agentfi/shield";                    // ← add this

const handler = new DefaultRequestHandler(agentCard, new InMemoryTaskStore(), executor);
const app = express();

app.use(agentfiTrace({ collectorUrl: "http://localhost:4318", agentName: "expedia-agent" }));
app.use(agentfiShield({                                              // ← add this
  registryUrl: "http://localhost:4319",
  agentId: "agent:expedia-001",
  enforce: true,                   // Set to false for "monitor-only" mode
}));

new A2AExpressApp(handler).setupRoutes(app);
app.listen(3001);
```

### 7.4 Adding Shield to an A2A Client (Caller Side)

```typescript
import { createShieldedClient } from "@agentfi/shield";             // ← import

const client = await createShieldedClient(
  "http://localhost:3001/.well-known/agent-card.json",
  {
    registryUrl: "http://localhost:4319",
    agentId: "agent:travel-planner-004",
    agentPrivateKey: process.env.AGENT_PRIVATE_KEY,   // Ed25519 private key
  }
);

// Token is automatically obtained from registry, attached to message.metadata,
// and refreshed when it expires.
const response = await client.sendMessage({ message: userMessage });
```

### 7.5 Full Integration (Trace + Shield, Server + Client)

```typescript
import express from "express";
import { DefaultRequestHandler, InMemoryTaskStore } from "@a2a-js/sdk/server";
import { A2AExpressApp } from "@a2a-js/sdk/server/express";
import { agentfiTrace, createTracedClient } from "@agentfi/trace";
import { agentfiShield, createShieldedClient } from "@agentfi/shield";

// --- Server setup ---
const handler = new DefaultRequestHandler(agentCard, new InMemoryTaskStore(), executor);
const app = express();
app.use(agentfiTrace({ collectorUrl: "http://localhost:4318", agentName: "travel-planner" }));
app.use(agentfiShield({ registryUrl: "http://localhost:4319", agentId: "agent:travel-planner-004" }));
new A2AExpressApp(handler).setupRoutes(app);

// --- Client setup (for calling downstream agents) ---
const expediaClient = await createShieldedClient(
  "http://localhost:3001/.well-known/agent-card.json",
  { registryUrl: "http://localhost:4319", agentId: "agent:travel-planner-004", agentPrivateKey: process.env.AGENT_KEY }
);
// This client automatically propagates trace context AND attaches A-JWT tokens.

app.listen(3004);
```

---

## 8. API Specifications

### 8.1 Registry API

#### POST /api/v1/agents — Register an Agent

```http
POST /api/v1/agents HTTP/1.1
Host: registry.agentfi.dev
Authorization: Bearer <owner-oauth-token>
Content-Type: application/json

{
  "name": "Expedia Flight Agent",
  "description": "Searches and books flights to Italian cities",
  "version": "1.0.0",
  "publicKey": "MCowBQYDK2VwAyEA... (base64 Ed25519 public key)",
  "capabilities": [
    {
      "skillId": "search_flights",
      "name": "Search Flights",
      "actions": ["search"],
      "constraints": {
        "rateLimit": { "maxRequests": 1000, "windowMs": 60000 },
        "allowedRegions": ["US", "EU"]
      }
    },
    {
      "skillId": "book_flight",
      "name": "Book Flight",
      "actions": ["book", "cancel"],
      "constraints": {
        "maxSpendPerAction": 200000,
        "maxSpendPerDay": 1000000,
        "currency": "USD",
        "requireHumanApproval": true,
        "humanApprovalThreshold": 50000
      }
    }
  ],
  "agentCardUrl": "http://localhost:3001/.well-known/agent-card.json",
  "tags": ["travel", "flights", "production"]
}
```

Response:

```json
{
  "agentId": "agent:a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "Expedia Flight Agent",
  "status": "active",
  "keyFingerprint": "sha256:9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
  "registeredAt": "2026-03-08T20:00:00Z",
  "owner": {
    "type": "user",
    "id": "user:shrey@example.com",
    "verifiedVia": "google-oauth"
  }
}
```

#### POST /api/v1/tokens — Issue an A-JWT

```http
POST /api/v1/tokens HTTP/1.1
Host: registry.agentfi.dev
Content-Type: application/json

{
  "agentId": "agent:travel-planner-004",
  "audience": "agent:expedia-001",
  "capabilities": ["search_flights"],
  "actions": ["search"],
  "constraints": {
    "max_spend": 0,
    "rate_limit": 100
  },
  "delegationChain": [
    {
      "from": "user:shrey@example.com",
      "to": "agent:personal-agent-001",
      "permissions": ["search_flights", "book_flights"],
      "constraints": { "max_spend": 500000, "currency": "USD" },
      "granted_at": "2026-03-08T20:00:00Z",
      "expires_at": "2026-03-09T00:00:00Z",
      "nonce": "d4e5f6a7-b8c9-0000-1111-222233334444",
      "signature": "base64:K7gNU3sdo+OL0wNhqoVWhr3g6s1xYv72ol/pe/Unols="
    }
  ],
  "ttlSeconds": 900
}
```

Response:

```json
{
  "token": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCIsImtpZCI6ImFnZW50Zmk6cmVn...",
  "tokenId": "tok_a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "expiresAt": "2026-03-08T20:15:00Z",
  "agentId": "agent:travel-planner-004",
  "audience": "agent:expedia-001"
}
```

#### POST /api/v1/delegations — Create a Delegation

```http
POST /api/v1/delegations HTTP/1.1
Host: registry.agentfi.dev
Authorization: Bearer <delegator-token>
Content-Type: application/json

{
  "from": "agent:personal-agent-001",
  "to": "agent:travel-planner-004",
  "permissions": ["search_flights", "search_hotels", "recommend_restaurants"],
  "constraints": {
    "max_spend": 500000,
    "currency": "USD",
    "expires_at": "2026-03-09T00:00:00Z",
    "regions": ["EU"]
  }
}
```

Response:

```json
{
  "delegationId": "del_f0e1d2c3-b4a5-6789-0123-456789abcdef",
  "from": "agent:personal-agent-001",
  "to": "agent:travel-planner-004",
  "permissions": ["search_flights", "search_hotels", "recommend_restaurants"],
  "constraints": {
    "max_spend": 500000,
    "currency": "USD",
    "expires_at": "2026-03-09T00:00:00Z",
    "regions": ["EU"]
  },
  "status": "active",
  "grantedAt": "2026-03-08T20:00:05Z",
  "expiresAt": "2026-03-09T00:00:00Z",
  "signature": "base64:X8hOV4tso+PM1xOirqpXXis4h7t2yYw83pm/qf/Vopm=",
  "nonce": "e5f6a7b8-c9d0-1111-2222-333344445555"
}
```

#### DELETE /api/v1/delegations/:id — Revoke a Delegation

```http
DELETE /api/v1/delegations/del_f0e1d2c3-b4a5-6789-0123-456789abcdef HTTP/1.1
Host: registry.agentfi.dev
Authorization: Bearer <delegator-token>
```

Response:

```json
{
  "delegationId": "del_f0e1d2c3-b4a5-6789-0123-456789abcdef",
  "status": "revoked",
  "revokedAt": "2026-03-08T21:30:00Z",
  "cascadeRevoked": [
    "del_a1b2c3d4-0000-0000-0000-000000000001",
    "del_a1b2c3d4-0000-0000-0000-000000000002"
  ]
}
```

Cascade revocation: any downstream delegations that depended on this one are automatically revoked.

#### GET /api/v1/agents/:id/spend — Get Spend Summary

```http
GET /api/v1/agents/agent:travel-planner-004/spend?window=24h HTTP/1.1
Host: registry.agentfi.dev
```

Response:

```json
{
  "agentId": "agent:travel-planner-004",
  "window": "24h",
  "spend": {
    "total": 34700,
    "currency": "USD",
    "bySkill": {
      "search_flights": { "amount": 0, "count": 47 },
      "book_flight": { "amount": 34700, "count": 2 }
    },
    "byDelegation": {
      "del_f0e1d2c3-b4a5-6789-0123-456789abcdef": {
        "amount": 34700,
        "limit": 500000,
        "remaining": 465300,
        "percentUsed": 6.94
      }
    }
  },
  "limits": {
    "daily": { "limit": 1000000, "used": 34700, "remaining": 965300 },
    "monthly": { "limit": null, "used": 34700 }
  }
}
```

### 8.2 Collector API

#### POST /v1/traces — Ingest OTLP Spans

Standard OTLP/HTTP endpoint. Accepts `application/x-protobuf` or `application/json`.

#### GET /api/v1/traces/:traceId — Get Full Trace

```http
GET /api/v1/traces/4bf92f3577b34da6a3ce929d0e0e4736 HTTP/1.1
Host: collector.agentfi.dev
```

Response:

```json
{
  "traceId": "4bf92f3577b34da6a3ce929d0e0e4736",
  "startedAt": "2026-03-08T20:00:00.000Z",
  "completedAt": "2026-03-08T20:00:01.240Z",
  "totalLatencyMs": 1240,
  "spanCount": 5,
  "status": "ok",
  "agents": [
    { "agentId": "agent:personal-agent-001", "name": "Personal Agent" },
    { "agentId": "agent:travel-planner-004", "name": "Travel Planner" },
    { "agentId": "agent:expedia-001", "name": "Expedia Agent" },
    { "agentId": "agent:marriott-002", "name": "Marriott Agent" },
    { "agentId": null, "name": "Restaurant Agent" }
  ],
  "spans": [
    {
      "spanId": "00f067aa0ba902b7",
      "parentSpanId": null,
      "source": { "name": "Personal Agent", "agentId": "agent:personal-agent-001" },
      "target": { "name": "Travel Planner", "agentId": "agent:travel-planner-004" },
      "transport": "a2a",
      "a2a": { "method": "message/send", "taskState": "completed" },
      "latencyMs": 1240,
      "status": "ok",
      "shield": { "verified": true, "policyResult": "allow" }
    },
    {
      "spanId": "a1b2c3d4e5f60001",
      "parentSpanId": "00f067aa0ba902b7",
      "source": { "name": "Travel Planner", "agentId": "agent:travel-planner-004" },
      "target": { "name": "Expedia Agent", "agentId": "agent:expedia-001" },
      "transport": "a2a",
      "a2a": { "method": "message/send", "skillId": "search_flights", "taskState": "completed" },
      "latencyMs": 52,
      "status": "ok",
      "shield": { "verified": true, "policyResult": "allow", "spendAmount": 0 }
    }
  ]
}
```

---

## 9. Security Threat Model

### 9.1 Threats and Mitigations

| # | Threat | Attack Vector | Severity | Mitigation |
|---|---|---|---|---|
| T1 | **Agent impersonation** | Attacker creates a fake agent and claims to be "Expedia Agent" | Critical | A-JWT with Ed25519 signature verification. Agent checksum prevents config tampering. Registry-issued tokens are unforgeable without the private key. |
| T2 | **Token replay** | Attacker captures a valid A-JWT and replays it to make unauthorized requests | High | Short token lifetime (5-15 min). `jti` claim for replay detection. Nonces in delegation links. |
| T3 | **Privilege escalation via delegation** | Agent A has "search" permission, delegates "search + book" to Agent B | High | Constraint narrowing enforced: each delegation link can only reduce permissions, never add new ones. Verified at chain validation time. |
| T4 | **Token theft** | Attacker extracts A-JWT from network traffic or logs | High | HTTPS mandatory in production. Tokens are short-lived. Payload redaction strips tokens from trace storage. Revocation endpoint for compromised tokens. |
| T5 | **Budget exhaustion** | Compromised agent makes many small transactions to drain the budget | Medium | Per-action, daily, and monthly spend limits. Real-time spend tracking. Webhook alerts at thresholds. Cascade revocation on suspicious activity. |
| T6 | **Denial of service** | Attacker floods an agent with A2A requests | Medium | Rate limiting enforced by Shield middleware. Per-agent, per-token rate limits. Registry can suspend agents. |
| T7 | **Man-in-the-middle** | Attacker intercepts A2A messages between agents | High | HTTPS/TLS mandatory. A-JWT signature ensures message integrity. Trace context integrity via `tracestate` signing. |
| T8 | **Registry compromise** | Attacker gains access to the AgentFi Registry | Critical | Ed25519 private keys stored in HSM/KMS (V1). Registry data integrity via hash chains. Audit log of all registry operations. Multi-factor auth for registry admin. |
| T9 | **Observability data poisoning** | Attacker sends fake spans to the collector | Medium | Collector authenticates span submissions via API key. Span source IP validation. Anomaly detection on span patterns (V1). |
| T10 | **PII leakage in traces** | Sensitive user data (credit cards, SSN) stored in trace payloads | High | SDK-level payload redaction with built-in PII patterns. Redaction before data leaves the agent process. Configurable redaction rules. Audit log of redacted fields. |

---

## 10. AgentFi Trace — Observability Product (Detailed)

### 10.0 Principles

1. **Standards-first**: Built on OpenTelemetry. Spans are OTLP-compatible. Export to any OTEL backend.
2. **Agent-aware**: Understands A2A tasks, MCP tools, agent cards — not just HTTP calls.
3. **Transport-agnostic**: Same SDK works regardless of how agents communicate.
4. **Privacy-conscious**: PII redaction built in. Sensitive fields stripped before export.
5. **Zero-config start**: One import, one line of setup. Complexity is opt-in.

### 10.1 P0 — Must Ship (Weeks 1-4)

#### P0-T1: Trace SDK for A2A (`@agentfi/trace`)

Express middleware that wraps A2A request handling and client calls.

**What it captures automatically:**

| Event | Captured Data |
|---|---|
| Inbound A2A `message/send` | Source agent (from agent card), message text, task ID, timing |
| Inbound A2A `tasks/get` | Task ID, requester |
| Outbound A2A client call | Target agent, message text, response, timing |
| Task state transitions | submitted → working → completed/failed |
| Errors | JSON-RPC error codes, error messages, stack traces |

**Configuration:**

```typescript
interface TraceConfig {
  collectorUrl: string;             // URL of the AgentFi Collector
  agentName: string;                // Name of this agent (for span attribution)
  agentId?: string;                 // Agent registry ID (if registered)

  // Optional
  enabled?: boolean;                // Default: true. Set false to disable without removing code.
  sampleRate?: number;              // 0.0 - 1.0. Default: 1.0 (trace everything).
  redaction?: RedactionConfig;      // PII redaction rules
  exportInterval?: number;          // Ms between batch exports. Default: 5000.
  maxExportBatchSize?: number;      // Max spans per batch. Default: 512.
  debug?: boolean;                  // Log spans to console. Default: false.
}
```

#### P0-T2: Trace Collector Service

Lightweight Node.js service that receives, stores, and queries spans.

**Endpoints:**

| Method | Path | Description |
|---|---|---|
| POST | `/v1/traces` | OTLP span ingest (protobuf + JSON) |
| GET | `/api/v1/traces` | List traces (paginated, filterable) |
| GET | `/api/v1/traces/:traceId` | Get full trace with all spans |
| GET | `/api/v1/traces/:traceId/graph` | Get trace as a call graph (nodes + edges) |
| GET | `/api/v1/spans` | Query spans (by agent, time range, status, transport) |
| GET | `/api/v1/agents/:agentId/metrics` | Per-agent metrics (latency, error rate, throughput) |

**Storage schema (SQLite MVP):**

```sql
CREATE TABLE spans (
  span_id         TEXT PRIMARY KEY,
  trace_id        TEXT NOT NULL,
  parent_span_id  TEXT,
  source_agent    TEXT NOT NULL,     -- JSON: AgentInfo
  target_agent    TEXT NOT NULL,     -- JSON: AgentInfo
  transport       TEXT NOT NULL,     -- "a2a" | "mcp" | "http" | "grpc"
  method          TEXT,              -- A2A method or MCP method
  task_id         TEXT,
  task_state      TEXT,
  skill_id        TEXT,
  request_summary TEXT,
  response_summary TEXT,
  request_size    INTEGER,
  response_size   INTEGER,
  started_at      TEXT NOT NULL,
  completed_at    TEXT,
  latency_ms      INTEGER,
  status          TEXT NOT NULL,     -- "ok" | "error"
  error_code      TEXT,
  error_message   TEXT,
  shield_verified INTEGER,          -- 0 | 1
  shield_policy   TEXT,             -- "allow" | "deny" | "unverified"
  shield_token_id TEXT,
  shield_agent_id TEXT,
  shield_spend    INTEGER,
  raw_data        TEXT,             -- Full span JSON (for detail view)
  created_at      TEXT DEFAULT (datetime('now'))
);

CREATE INDEX idx_spans_trace    ON spans(trace_id);
CREATE INDEX idx_spans_agent    ON spans(source_agent, target_agent);
CREATE INDEX idx_spans_time     ON spans(started_at);
CREATE INDEX idx_spans_status   ON spans(status);
CREATE INDEX idx_spans_shield   ON spans(shield_verified);
```

#### P0-T3: Trace Dashboard — Waterfall View

React component rendering a timeline waterfall of spans within a trace.

**Features:**
- Horizontal timeline bars showing duration of each span
- Nested indentation showing parent-child relationships
- Color coding: green (ok), red (error), yellow (unverified identity)
- Click to expand: full request/response, A2A task details, Shield verification status
- Search bar: find traces by trace ID, agent name, time range, or error status
- Auto-refresh toggle for live monitoring

#### P0-T4: Trace Dashboard — Agent Call Graph

React component rendering a directed graph of agent interactions.

**Features:**
- Nodes represent agents (name, ID, colored by status)
- Edges represent A2A/MCP calls (labeled with latency, colored by status)
- Layout: left-to-right or top-to-bottom, auto-positioned
- Hover on edge: message preview popup
- Click on node: navigate to agent detail page
- Zoom and pan for complex graphs

### 10.2 P1 — Important (Weeks 5-6)

#### P1-T1: MCP Transport Support

- Intercept MCP `params._meta` for W3C Trace Context propagation
- Capture `tools/call`, `resources/read`, `prompts/get` as spans
- MCP-specific semantic attributes: `mcp.tool_name`, `mcp.resource_uri`, `mcp.prompt_name`
- Align with OpenTelemetry MCP semantic conventions

#### P1-T2: Agent Health Metrics

Per-agent dashboard derived from ingested spans. No additional instrumentation.

**Metrics computed:**

| Metric | Computation |
|---|---|
| Requests/second | Count of spans targeting this agent, per second |
| p50 / p95 / p99 latency | Percentiles of `latencyMs` for spans targeting this agent |
| Error rate | % of spans with `status: "error"` |
| Availability | % of time windows where the agent responded to at least one request |
| Identity coverage | % of inbound calls with `shield.verified: true` |
| Top callers | Agents that call this agent most frequently |

#### P1-T3: Basic Alerting

Configurable alert rules with webhook delivery.

**Alert rule example:**

```json
{
  "name": "Expedia error rate spike",
  "condition": {
    "metric": "error_rate",
    "agent": "agent:expedia-001",
    "operator": ">",
    "threshold": 0.05,
    "window": "5m"
  },
  "action": {
    "type": "webhook",
    "url": "https://hooks.slack.com/services/T00/B00/xxxx",
    "payload": {
      "text": "🚨 {{agent_name}} error rate is {{value}}% (threshold: {{threshold}}%)"
    }
  }
}
```

#### P1-T4: Payload Redaction Engine

SDK-level redaction applied before spans leave the agent process.

**Built-in patterns:**

| Pattern | Regex | Replacement |
|---|---|---|
| Email | `[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}` | `[EMAIL_REDACTED]` |
| Credit card | `\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b` | `[CC_REDACTED]` |
| SSN | `\b\d{3}-\d{2}-\d{4}\b` | `[SSN_REDACTED]` |
| Phone | `\b\+?1?[\s.-]?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}\b` | `[PHONE_REDACTED]` |
| JWT token | `eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+` | `[TOKEN_REDACTED]` |
| API key | `(api[_-]?key\|apikey\|secret)["\s:=]+\S+` | `[APIKEY_REDACTED]` |

Custom patterns configurable via `RedactionConfig`.

### 10.3 P2 — V1 Features

#### P2-T1: HTTP / gRPC Transport Support
Standard OpenTelemetry auto-instrumentation with AgentFi semantic enrichment.

#### P2-T2: Trace Replay & Diffing
Replay a failed trace against live agents. Diff two traces side by side.

#### P2-T3: Cost Attribution
Attribute LLM token costs to traces. Per-agent, per-trace cost breakdown.

#### P2-T4: OpenTelemetry Export
Export AgentFi spans to any OTEL backend (Datadog, Grafana, Jaeger, Honeycomb).

---

## 11. AgentFi Shield — Identity & Trust Product (Detailed)

### 11.0 Principles

1. **Ownership-first**: Every agent has a registered owner. No anonymous agents.
2. **Capability-bounded**: Agents declare what they can do. Shield enforces boundaries.
3. **Delegation-native**: Authorization flows through signed delegation chains.
4. **Expense-aware**: Spending limits and tracking are first-class.
5. **Standards-aligned**: Implements A-JWT, aligns with DAAP.

### 11.1 P0 — Must Ship (Weeks 1-4)

#### P0-S1: Agent Registry Service

Full CRUD for agent management with ownership verification.

**Endpoints:**

| Method | Path | Description |
|---|---|---|
| POST | `/api/v1/agents` | Register a new agent |
| GET | `/api/v1/agents` | List agents (paginated, filterable by owner, status, tag) |
| GET | `/api/v1/agents/:id` | Get agent details |
| PATCH | `/api/v1/agents/:id` | Update agent (name, description, capabilities) |
| POST | `/api/v1/agents/:id/suspend` | Suspend an agent |
| POST | `/api/v1/agents/:id/revoke` | Permanently revoke an agent |
| POST | `/api/v1/agents/:id/reactivate` | Reactivate a suspended agent |
| GET | `/api/v1/agents/:id/tokens` | List tokens issued for this agent |
| GET | `/api/v1/agents/:id/spend` | Get spend summary |

#### P0-S2: A-JWT Token Issuance

**Endpoints:**

| Method | Path | Description |
|---|---|---|
| POST | `/api/v1/tokens` | Issue a new A-JWT |
| POST | `/api/v1/tokens/verify` | Verify a token (for debugging/testing) |
| POST | `/api/v1/tokens/:jti/revoke` | Revoke a specific token |
| GET | `/.well-known/jwks.json` | Public keys for token verification |

**Token issuance flow:**

```
Agent → POST /api/v1/tokens
  ├─ Authenticate agent (API key or client credentials)
  ├─ Validate requested capabilities against registered capabilities
  ├─ Validate delegation chain (if provided)
  ├─ Compute agent_checksum from registry data
  ├─ Generate JWT with Ed25519 signature
  └─ Return token + metadata
```

#### P0-S3: Shield Verification Middleware

Express middleware that verifies A-JWT on incoming A2A requests.

**Verification result attached to request context:**

```typescript
interface ShieldContext {
  verified: boolean;
  agentId?: string;
  ownerId?: string;
  capabilities?: string[];
  actions?: string[];
  constraints?: object;
  delegationChain?: DelegationLink[];
  delegationDepth?: number;
  remainingBudget?: number;
  policyResult: "allow" | "deny" | "unverified";
  denialReason?: string;
  tokenId?: string;
}
```

Accessible in the A2A executor:

```typescript
class MyExecutor implements AgentExecutor {
  async execute(ctx: RequestContext, eventBus: ExecutionEventBus) {
    // Access Shield context from request
    const shield = (ctx as any).shieldContext as ShieldContext;

    if (shield.verified) {
      console.log(`Verified caller: ${shield.agentId}, owner: ${shield.ownerId}`);
      console.log(`Remaining budget: $${shield.remainingBudget! / 100}`);
    }
  }
}
```

#### P0-S4: Agent Ownership Dashboard

Web UI integrated into the AgentFi Dashboard for managing agents.

**Views:**
- Agent list (filterable by status, owner, tags)
- Agent detail (capabilities editor, constraint editor, key management)
- Token history (issued tokens, their audiences, expiry status)
- Activity log (recent A-JWT verifications, allow/deny decisions)

### 11.2 P1 — Important (Weeks 5-6)

#### P1-S1: Delegation Chain Manager

**Endpoints:**

| Method | Path | Description |
|---|---|---|
| POST | `/api/v1/delegations` | Create a delegation |
| GET | `/api/v1/delegations` | List delegations (by from/to agent, status) |
| GET | `/api/v1/delegations/:id` | Get delegation details |
| DELETE | `/api/v1/delegations/:id` | Revoke delegation (with cascade) |
| POST | `/api/v1/delegations/verify-chain` | Verify a full delegation chain |
| GET | `/api/v1/delegations/:id/downstream` | Get all downstream delegations |

**Constraint narrowing rule:**

At each hop in the delegation chain, the receiving agent's permissions are the **intersection** of:
- The permissions being delegated
- The delegating agent's own permissions

```
User: [search_flights, book_flights, search_hotels, book_hotels]
  └─► PersonalAgent: [search_flights, book_flights, search_hotels]     ← user didn't delegate book_hotels
        └─► TravelPlanner: [search_flights, search_hotels]             ← PersonalAgent didn't delegate book_flights
              └─► ExpediaAgent: [search_flights]                        ← TravelPlanner only delegated search_flights
```

If TravelPlanner tried to delegate `book_flights` to ExpediaAgent, the registry would reject it because TravelPlanner itself doesn't have `book_flights`.

#### P1-S2: Expense & Usage Tracking

Real-time spend tracking with limit enforcement.

**How it works:**

1. When an A-JWT is issued for a spending action, the token's constraints include `max_spend`
2. When Shield middleware verifies the token, it checks the spend tracker
3. If the action's cost would exceed the remaining budget, the request is denied
4. If allowed, the spend is recorded after the action completes
5. Spend is tracked per-agent, per-delegation, and per-time-window

**Database table:**

```sql
CREATE TABLE spend_ledger (
  id              TEXT PRIMARY KEY,
  agent_id        TEXT NOT NULL,
  delegation_id   TEXT,
  token_id        TEXT NOT NULL,
  skill_id        TEXT NOT NULL,
  action          TEXT NOT NULL,
  amount          INTEGER NOT NULL,  -- In minor units (cents)
  currency        TEXT NOT NULL,
  trace_id        TEXT,              -- Links to observability data
  span_id         TEXT,
  recorded_at     TEXT NOT NULL,
  metadata        TEXT               -- JSON: additional context
);

CREATE INDEX idx_spend_agent ON spend_ledger(agent_id, recorded_at);
CREATE INDEX idx_spend_delegation ON spend_ledger(delegation_id);
```

#### P1-S3: Human-in-the-Loop Approval

When an action exceeds the `humanApprovalThreshold`, execution pauses and requests human approval.

**Flow:**

```
1. Agent receives request to book $2000 flight
2. Shield middleware checks: humanApprovalThreshold = $500 → approval required
3. Shield sends webhook to configured approval endpoint (e.g., Slack)
4. Agent returns A2A task state: "input-required"
5. Human approves/rejects via webhook callback
6. If approved: Agent proceeds. Approval logged in audit trail.
7. If rejected: Agent returns "canceled" task state.
8. If timeout (configurable): Agent returns "failed" with reason "approval_timeout"
```

**Approval webhook payload:**

```json
{
  "type": "approval_request",
  "requestId": "apr_a1b2c3d4",
  "agent": {
    "agentId": "agent:travel-planner-004",
    "name": "Travel Planner Agent"
  },
  "action": {
    "skillId": "book_flight",
    "action": "book",
    "description": "Book flight NYC → Rome for $1,847"
  },
  "delegationChain": [
    { "from": "user:shrey@example.com", "to": "agent:personal-agent-001" },
    { "from": "agent:personal-agent-001", "to": "agent:travel-planner-004" }
  ],
  "spend": {
    "amount": 184700,
    "currency": "USD",
    "remainingBudget": 315300
  },
  "approveUrl": "https://registry.agentfi.dev/api/v1/approvals/apr_a1b2c3d4/approve",
  "rejectUrl": "https://registry.agentfi.dev/api/v1/approvals/apr_a1b2c3d4/reject",
  "expiresAt": "2026-03-08T20:05:00Z"
}
```

#### P1-S4: Shielded A2A Client

Client wrapper that automatically obtains and attaches A-JWT tokens.

```typescript
// Token lifecycle managed automatically:
// 1. On first call: obtain token from registry
// 2. On subsequent calls: reuse token until it expires
// 3. On expiry: automatically refresh
// 4. On revocation: obtain new token
// 5. Delegation chain attached from configured delegation links
```

### 11.3 P2 — V1 Features

#### P2-S1: MCP Transport Support for Shield
Inject/verify A-JWT in MCP `params._meta`. Same enforcement for MCP tool calls.

#### P2-S2: Agent Reputation Scoring
Computed score (0-100) based on uptime, error rate, latency, policy compliance, age.

#### P2-S3: Multi-Organization Federation
Cross-org trust establishment. Registry-to-registry trust protocol.

#### P2-S4: Compliance Report Generator
PDF/JSON reports mapping AgentFi data to EU AI Act Articles 12-14.

---

## 12. Detailed Use Case Walkthrough

### 12.1 Travel Demo — Full Message Flow

This walkthrough shows exactly what happens when a user types "Plan a 5-day Italy trip visiting Rome and Venice" into the PersonalAgent CLI, with AgentFi Trace and Shield instrumented.

```
Step 1: User types command in PersonalAgent CLI
─────────────────────────────────────────────────
PersonalAgent generates:
  - traceId: "4bf92f3577b34da6a3ce929d0e0e4736"
  - spanId:  "00f067aa0ba902b7"

PersonalAgent obtains A-JWT from registry:
  POST http://registry:4319/api/v1/tokens
  → Token for audience "agent:travel-planner-004"
  → Delegation chain: user:shrey → personal-agent-001

Step 2: PersonalAgent → TravelPlannerAgent (A2A)
─────────────────────────────────────────────────
Outbound A2A message/send with:
  message.metadata.traceparent = "00-4bf92f35...-00f067aa...-01"
  message.metadata.x-agentfi-token = "eyJhbGci..."
  message.parts[0].text = "Plan a 5-day Italy trip visiting Rome and Venice"

Span recorded by @agentfi/trace:
  { traceId: "4bf92f35...", spanId: "00f067aa...", source: "PersonalAgent",
    target: "TravelPlanner", transport: "a2a", method: "message/send" }

TravelPlannerAgent receives request:
  @agentfi/shield middleware:
    ✅ A-JWT signature valid
    ✅ Token not expired
    ✅ Audience matches (agent:travel-planner-004)
    ✅ Agent checksum matches registry
    ✅ Capability "plan_itinerary" authorized
    ✅ Delegation chain valid: user:shrey → personal-agent-001 → travel-planner-004
  → Request allowed

Step 3: TravelPlannerAgent → ExpediaAgent (A2A)
─────────────────────────────────────────────────
TravelPlanner obtains scoped A-JWT:
  audience: "agent:expedia-001"
  capabilities: ["search_flights"]
  actions: ["search"]
  delegation_chain: user:shrey → personal-agent-001 → travel-planner-004

Outbound A2A message/send:
  message.metadata.traceparent = "00-4bf92f35...-a1b2c3d4...-01"  (same traceId, new spanId)
  message.parts[0].text = "Find flights to Rome"

Span recorded: { spanId: "a1b2c3d4...", parentSpanId: "00f067aa...", target: "ExpediaAgent" }

ExpediaAgent @agentfi/shield:
  ✅ All checks pass
  ✅ "search_flights" capability authorized
  ✅ "search" action authorized
  → Returns flight data

Step 4: TravelPlannerAgent → MarriottAgent (A2A)  [parallel]
─────────────────────────────────────────────────
Same pattern. Span: { spanId: "b2c3d4e5...", parentSpanId: "00f067aa..." }

Step 5: TravelPlannerAgent → RestaurantAgent (A2A)  [parallel]
─────────────────────────────────────────────────
RestaurantAgent has NO A-JWT (not registered with Shield).
@agentfi/shield on RestaurantAgent: ⚠️ No token found → policyResult: "unverified"
Request still allowed (enforce: false mode), but flagged in trace.

Step 6: TravelPlannerAgent assembles itinerary
─────────────────────────────────────────────────
Combines responses from all 3 agents.
Returns itinerary to PersonalAgent.
Task state: "completed"

Step 7: PersonalAgent displays result to user
─────────────────────────────────────────────────
Full trace now in AgentFi Dashboard:
  5 spans, 3 agents verified, 1 unverified, 0 errors
  Total latency: 1,240ms
  Spend: $0 (all actions were searches)
```

---

## 13. Eight-Week Execution Plan (Detailed)

### Week 1: Monorepo Foundation + Core Types

| Day | Track | Task |
|---|---|---|
| Mon | Infra | Set up Turborepo monorepo with pnpm workspaces. Create all package scaffolds. |
| Mon | Infra | Configure TypeScript, ESLint, Prettier across all packages. |
| Tue | Common | Define all shared TypeScript interfaces (span, agent, token, delegation). |
| Tue | Common | Implement Ed25519 key generation, signing, and verification helpers. |
| Wed | Trace | Implement A2A trace middleware (inbound span capture). |
| Wed | Shield | Implement Agent Registry CRUD API with SQLite. |
| Thu | Trace | Implement traced A2A client wrapper (outbound span capture). |
| Thu | Shield | Implement A-JWT token issuance endpoint with Ed25519 signing. |
| Fri | Trace | Implement W3C Trace Context propagation (inject/extract from A2A metadata). |
| Fri | Shield | Implement JWKS endpoint and token verification logic. |

### Week 2: Collector + Registry + Demo Integration

| Day | Track | Task |
|---|---|---|
| Mon | Collector | Implement OTLP HTTP receiver (JSON format). |
| Mon | Collector | Implement SQLite span storage with indexing. |
| Tue | Collector | Implement span query API (by traceId, agentId, time range). |
| Tue | Registry | Implement OAuth owner verification (Google, GitHub). |
| Wed | Registry | Implement agent capability validation and constraint checking. |
| Wed | Shield | Implement Shield verification middleware for A2A Express. |
| Thu | Demo | Integrate @agentfi/trace into agent-demo travel agents. |
| Thu | Demo | Integrate @agentfi/shield into agent-demo travel agents. |
| Fri | Demo | End-to-end test: travel demo produces traces + uses A-JWTs. |
| Fri | CI/CD | Set up GitHub Actions for lint, typecheck, test. |

**Week 2 Checkpoint:** Travel demo runs with real traces flowing to collector and real A-JWTs verified between agents.

### Week 3: Dashboard — Trace Views

| Day | Track | Task |
|---|---|---|
| Mon | Dashboard | Set up React + Vite + TanStack Query. Implement app shell and routing. |
| Mon | Dashboard | Implement trace list page (search, filter, paginate). |
| Tue | Dashboard | Implement waterfall view component (timeline bars, nesting, color coding). |
| Wed | Dashboard | Implement span detail expansion (request/response, A2A task details). |
| Thu | Dashboard | Implement call graph visualization (D3.js or react-flow). |
| Fri | Dashboard | Implement trace search (by trace ID, agent name, time range, status). |

### Week 4: Dashboard — Shield Views + Unified Shell

| Day | Track | Task |
|---|---|---|
| Mon | Dashboard | Implement agent registry management page (list, detail, create). |
| Mon | Dashboard | Implement capability editor (skill, actions, constraints). |
| Tue | Dashboard | Implement token history view. |
| Tue | Dashboard | Integrate Shield data into trace waterfall (identity badges on spans). |
| Wed | Dashboard | Implement unified navigation (Traces, Agents, Delegations, Compliance). |
| Thu | Dashboard | Polish UI: loading states, error handling, responsive layout. |
| Fri | Demo | Full screencast-ready demo: traces + identity in dashboard. |

**Week 4 Checkpoint:** Screencast-ready demo. Dashboard shows traces with identity verification on each span.

### Week 5: Delegation + MCP + Metrics

| Day | Track | Task |
|---|---|---|
| Mon | Shield | Implement delegation CRUD API. |
| Mon | Shield | Implement delegation chain verification with constraint narrowing. |
| Tue | Shield | Implement cascade revocation. |
| Tue | Trace | Implement MCP transport adapter (trace context in `params._meta`). |
| Wed | Shield | Implement expense tracking ledger and limit enforcement. |
| Wed | Trace | Implement per-agent metrics computation from spans. |
| Thu | Dashboard | Implement delegation chain viewer. |
| Thu | Dashboard | Implement per-agent metrics dashboard (latency, error rate, throughput). |
| Fri | Shield | Implement shielded A2A client (auto-attach tokens, auto-refresh). |

### Week 6: Human-in-the-Loop + Alerting + Redaction

| Day | Track | Task |
|---|---|---|
| Mon | Shield | Implement human-in-the-loop approval webhook flow. |
| Mon | Shield | Implement approval timeout and A2A input-required task state. |
| Tue | Shield | Implement spend dashboard (per-agent, per-delegation budget views). |
| Tue | Trace | Implement alerting engine (configurable rules, webhook delivery). |
| Wed | Trace | Implement PII redaction engine with built-in patterns. |
| Wed | Dashboard | Implement alert configuration UI and alert history. |
| Thu | Dashboard | Implement spend tracking dashboard. |
| Thu | Dashboard | Implement compliance posture widget (EU AI Act mapping). |
| Fri | Demo | Full demo: delegation chains + expense limits + human approval. |

**Week 6 Checkpoint:** Full platform demo. Can show: "User sets $500 budget → delegation flows → booking rejected when it exceeds budget → human approves higher spend."

### Week 7: Documentation + Polish

| Day | Track | Task |
|---|---|---|
| Mon | Docs | Write getting started guide (5-minute quickstart). |
| Mon | Docs | Write API reference for Registry and Collector. |
| Tue | Docs | Write integration guides: A2A trace, A2A shield, MCP trace. |
| Tue | Docs | Write architecture overview with diagrams. |
| Wed | Product | Build landing page (agentfi.dev). |
| Wed | Examples | Create `examples/basic-trace/` minimal example. |
| Thu | Examples | Create `examples/basic-shield/` minimal example. |
| Thu | Examples | Create `examples/full-travel-demo/` comprehensive example. |
| Fri | Product | Record 3-minute demo video. Write launch blog post draft. |

### Week 8: Launch

| Day | Track | Task |
|---|---|---|
| Mon | Release | Publish `@agentfi/trace` and `@agentfi/shield` to npm. |
| Mon | Release | Push collector and registry Docker images to GHCR. |
| Tue | Release | Deploy hosted dashboard (agentfi.dev/dashboard). |
| Tue | Release | Deploy documentation site. |
| Wed | Launch | Publish blog post. |
| Wed | Launch | Post on Hacker News, Reddit (r/MachineLearning, r/AI_Agents). |
| Thu | Launch | Post on Twitter/X, LinkedIn. |
| Thu | Launch | Submit to Product Hunt. |
| Fri | Launch | Monitor feedback, triage issues, engage with community. |

---

## 14. Pricing Model

### Open Source (Free Forever)

- `@agentfi/trace` SDK — MIT license
- `@agentfi/shield` SDK — MIT license
- Self-hosted collector — unlimited spans
- Self-hosted registry — unlimited agents

### AgentFi Cloud — Hosted SaaS

| Tier | Price | Spans/month | Agents | Retention | Features |
|---|---|---|---|---|---|
| **Free** | $0 | 10,000 | 10 | 24 hours | Trace waterfall, call graph, basic search |
| **Pro** | $49/mo | 1,000,000 | 100 | 30 days | + Agent health metrics, alerting, PII redaction, Shield identity |
| **Team** | $199/mo | 10,000,000 | 500 | 90 days | + Delegation management, expense tracking, human-in-the-loop |
| **Enterprise** | Custom | Unlimited | Unlimited | 1 year | + Compliance reports, SSO/SAML, SLA, dedicated support, on-prem option |

---

## 15. Go-to-Market Strategy

### Phase 1: Developer Adoption (Weeks 1-12)

**Channel:** Open source → npm → GitHub → developer content

**Tactics:**
1. Publish `@agentfi/trace` and `@agentfi/shield` as MIT-licensed npm packages
2. Write "How to debug multi-agent systems" tutorial (SEO + education)
3. Write "Implementing A-JWT: A reference guide" (positions us as the standard)
4. Post on Hacker News with the travel demo showing traces
5. Build integrations with popular frameworks (LangGraph, CrewAI)
6. Engage in A2A and MCP community discussions

**Goal:** 500+ npm weekly downloads, 200+ GitHub stars

### Phase 2: Team Adoption (Months 3-4)

**Channel:** Pro tier → team collaboration features

**Tactics:**
1. Add team workspaces to the dashboard
2. Write case study from early adopter
3. Content: "Securing multi-agent systems: A practical guide"
4. Speak at AI/agent meetups and conferences
5. Partner with A2A/MCP ecosystem projects

**Goal:** 10+ paying Pro/Team customers

### Phase 3: Enterprise (Months 5-8)

**Channel:** Compliance narrative → enterprise sales

**Tactics:**
1. Publish EU AI Act compliance whitepaper
2. Build compliance report generator
3. Direct outreach to EU-based companies deploying AI agents
4. Partner with compliance consulting firms
5. SOC 2 certification for AgentFi Cloud

**Goal:** 3+ enterprise pilots, $50K+ ARR

---

## 16. V1 Vision (Months 3-6 Post-Launch)

1. **Multi-org federation** — cross-organization agent trust with registry-to-registry protocol
2. **Compliance report generator** — auto-generated EU AI Act Articles 12-14 compliance reports
3. **Framework adapters** — LangGraph, CrewAI, AutoGen, Semantic Kernel adapters
4. **Anomaly detection** — ML-based detection of unusual agent behavior patterns
5. **Terraform / IaC provider** — define agent capabilities, delegations, and policies as code
6. **Python SDK** — `agentfi-trace` and `agentfi-shield` PyPI packages
7. **Agent marketplace integration** — trust layer for agent discovery platforms
8. **SOC 2 / ISO 27001 mapping** — enterprise security framework alignment

---

## 17. Success Metrics

### MVP (Week 8)

| Metric | Target |
|---|---|
| npm weekly downloads (`@agentfi/trace` + `@agentfi/shield`) | 500+ |
| GitHub stars (combined repos) | 200+ |
| Dashboard signups | 50+ |
| Traces ingested (across all users) | 100,000+ |
| Agents registered | 100+ |
| Demo video views | 5,000+ |

### V1 (Month 6)

| Metric | Target |
|---|---|
| npm weekly downloads | 5,000+ |
| Paying customers | 10+ |
| ARR | $50K+ |
| Enterprise pilots | 3+ |
| Transports supported | 4 (A2A, MCP, HTTP, gRPC) |
| Framework adapters | 3+ |

---

## 18. Technical Decisions & Rationale

| Decision | Choice | Rationale |
|---|---|---|
| Language | TypeScript (Node.js) | A2A ecosystem is JS/TS-first. Our demo is TS. npm is the distribution channel. Python SDK follows in V1. |
| Signing algorithm | Ed25519 | 64-byte compact signatures. Sub-millisecond verification. Used by SSH, Signal, Agent Passport Protocol. No patent encumbrances. |
| Trace format | OpenTelemetry (OTLP) | Industry standard. Compatible with Datadog, Grafana, Jaeger, Honeycomb. No vendor lock-in. Semantic conventions for GenAI agents are actively being standardized. |
| Identity token | A-JWT (Agentic JWT) | IETF draft (draft-goswami-agentic-jwt-00) with growing momentum. Extends OAuth 2.0 — familiar to developers. Agent checksum is a unique capability not found in alternatives. |
| Storage (MVP) | SQLite | Zero ops for self-hosted. Fast enough for MVP volumes (100K+ spans). WAL mode for concurrent reads. Postgres migration path via storage interface abstraction. |
| Storage (V1) | PostgreSQL | Scales to millions of spans. JSONB for flexible queries. TimescaleDB extension for time-series metrics. |
| Dashboard framework | React + Vite | Standard, fast, large component ecosystem. TanStack Query for data fetching. react-flow for call graph visualization. |
| Monorepo tool | Turborepo | Fast incremental builds. Good TS support. pnpm workspaces for dependency management. |
| Key management (MVP) | Registry-managed | Simple: keys generated and stored in registry database. Good enough for MVP. |
| Key management (V1) | Cloud KMS integration | AWS KMS, GCP Cloud KMS, Azure Key Vault. HSM-backed key storage for enterprise. |

---

## 19. Open Questions & Risks

| # | Question / Risk | Severity | Mitigation |
|---|---|---|---|
| 1 | **Market timing** — is multi-agent adoption far enough for paying customers? | Medium | Open-source SDK useful for any A2A/MCP user today. EU AI Act deadline (Aug 2026) creates near-term compliance demand. |
| 2 | **A-JWT adoption** — will the IETF draft gain traction or be superseded? | Low | Our implementation is the reference. Core concepts (delegation chains, checksums, constraints) are protocol-agnostic. Token format is a swappable detail. |
| 3 | **A2A vs MCP** — which protocol wins for agent-to-agent? | Low | We're transport-agnostic from day 1. Both supported. We win regardless. |
| 4 | **Performance overhead** — will middleware slow down agents noticeably? | Medium | Target: <2ms per span capture (async export). A-JWT verification: <1ms (Ed25519). Benchmark and publish numbers. Sample rate configuration for high-throughput systems. |
| 5 | **Competitor response** — Vouched adds observability, or Spanora adds identity? | Medium | Our moat is the unified data model. Retrofitting identity onto an observability tool requires re-architecting the span schema, storage, and UI. 6+ month effort. We're designed unified from day 1. |
| 6 | **Enterprise sales cycle** — compliance sales can take 6+ months | Medium | Open-source SDK creates bottom-up adoption. Developers champion internally. Pro/Team tiers bridge the gap. Enterprise layer comes at month 5-6. |
| 7 | **Key management at scale** — Ed25519 key distribution and rotation | Medium | Start simple (registry-managed). V1: cloud KMS integration. V2: automatic rotation with grace periods. |
| 8 | **Multi-org trust** — how do organizations establish trust between registries? | Low (V1) | Not needed for MVP. V1 introduces registry-to-registry federation protocol. Can leverage existing PKI or Web of Trust models. |
| 9 | **GDPR interaction** — trace data may contain personal data | Medium | PII redaction at SDK level (before data leaves agent). Configurable retention policies. Data deletion API for GDPR right-to-erasure. |
| 10 | **Span volume at scale** — millions of spans per day | Medium (V1) | SQLite sufficient for MVP. Postgres + TimescaleDB for V1. Sampling configuration. Tiered storage (hot/warm/cold). |

---

## 20. Jump-Start Advantages

1. **Working A2A demo** — the `agent-demo/` travel itinerary demo is a live 5-agent system we can instrument on day 1. No competitor has a built-in showcase this compelling.

2. **First production A-JWT implementation** — the IETF draft (draft-goswami-agentic-jwt-00) exists as a paper. We ship the code. Every discussion about agent identity will reference our implementation. We become the de facto standard library.

3. **OpenTelemetry alignment** — we don't invent a tracing format. We extend OTEL with agent-specific semantic conventions. Instant interop with Datadog, Grafana, Jaeger, and the entire OTEL ecosystem.

4. **EU AI Act deadline** — August 2026 is 5 months away. Companies are scrambling for compliance solutions. Our timing is not "early" — it's exactly right for the compliance wave.

5. **Unified data model** — designing observability and identity together means our spans natively include identity context. Competitors would need to retrofit this — a 6+ month architecture effort that touches every layer of their stack.

6. **Open-source wedge** — MIT-licensed npm packages drive adoption. The hosted dashboard and compliance reporting are the commercial layer. This is the proven open-core business model (Redis, Elastic, Grafana, etc.).

7. **Protocol-agnostic positioning** — we're "the agent governance platform," not "the A2A tool." This positioning survives protocol wars and framework churn.

8. **The name** — AgentFi. Agent + Finance/Fidelity/Trust. Short, memorable, and perfectly describes what we do: financial-grade trust infrastructure for agents.

9. **agentfi.com/connect** — a one-click connect page that works from ChatGPT, Claude, Cursor, and browsers. Nobody else has this. While competitors require SDK integration and code changes, we let anyone talk to our agent by clicking a button. This page becomes the prototype for the AgentFi agent registry — proof that one-click agent discovery is possible.

10. **ChatGPT GPT Store listing** — "AgentFi Agent" published in the store gives us organic discoverability to millions of ChatGPT users. Every person who uses the GPT experiences the A2A protocol without knowing it — and sees the AgentFi brand.

---

## 21. Live Public Agent — Progressive Build-in-Public Strategy

### 21.1 Concept

AgentFi maintains **two agent environments**:

1. **Public Agent** (`agentfi.com`) — a live, internet-facing A2A agent that anyone can talk to from ChatGPT, Claude, Cursor, or a browser. This is the market-facing showcase that gains a new capability every week.

2. **Internal Test Harness** (`agent-demo/`) — the existing 5-agent travel demo (Expedia, Marriott, Restaurant, TravelPlanner, PersonalAgent) running locally. This is the multi-agent testbed where we develop and validate the Trace and Shield SDKs before applying them to the public agent.

**Week 1 ships the public agent, landing page, and connect page — no SDK work.** This establishes market position immediately. SDK infrastructure starts in Week 2, tested against the internal travel agents first, then applied to the public agent once validated.

The public agent starts dumb (echo "Hello agent!") and progressively gains new capabilities week by week. Each new layer changes how the agent responds, making progress tangible and testable.

This serves four purposes:
1. **Market positioning** — Week 1 landing page + public agent signals "something big is coming" before we've built the full platform
2. **Proof of work** — every week, anyone can hit our agent and see the new layer in action
3. **Content engine** — each new layer is a LinkedIn post with a concrete before/after
4. **Community testbed** — developers test their agents against ours, giving us feedback and bug reports

The agent is hosted locally and exposed via ngrok (or a stable cloud URL), with the Agent Card discoverable at `/.well-known/agent-card.json` and MCP manifest at `/.well-known/mcp.json`.

### 21.2 Progressive Layers

The agent evolves week by week. Each layer changes the agent's behavior in a way that is immediately observable by the caller:

```
Week 1:  PUBLIC LAUNCH — landing page + connect page + public echo agent
         "Hello agent! I received your message: <echo>"
         (one-click from ChatGPT, Claude, Cursor, browser — agentfi.com/connect)
         (internal: agent-demo/ travel agents running as SDK test harness)

Week 2:  SDK FOUNDATION — monorepo + Trace SDK + Shield SDK + collector
         "Hello agent! ... [OBSERVED: trace-id abc123 — your request was logged]"
         (internal: travel demo agents instrumented with real traces + A-JWTs)

Week 3:  IDENTITY — public agent checks identity, self-service registration
         "⚠️ You are not identified. Register at agentfi.com/register"
         "✅ Verified: agent:caller-001 (owner: dev@example.com)"

Week 4:  GUARDRAILS — rate limits, injection protection, capability enforcement
         "🚧 Guardrail check: OK" or "🚫 GUARDRAIL_VIOLATION: rate limit exceeded"

Week 5:  WALLET — demo balances, spend tracking per agent
         "💰 Balance: $499.99 ($0.01 charged for this request)"

Week 6:  LLM — OpenAI-powered answers, requires identity + budget
         "🤖 [GPT-4o] The A2A protocol is an open standard by Google..."

Week 7:  DELEGATION — multi-hop trust chains verified in responses
         "🔗 Delegation: user:bob → agent:helper → agent:caller — chain verified"

Week 8:  MVP LAUNCH — full platform, npm packages, docs, demo video

Week 9+: Tx rails, netting, reputation, compliance reports
```

### 21.3 Agent Behavior by Layer

**Layer 1 — Echo + One-Click Universal Connect (Week 1)**

The agent is reachable from **every major AI platform with one click**. This is the Week 1 headline: *visit agentfi.com/connect, click one button, start talking.*

**The Agent (core):**
- A2A agent on port 3010 with skill `echo`
- Responds: `"Hello agent! I received your message: <their text>"`
- Agent Card at `/.well-known/agent-card.json`
- Hosted locally, exposed via ngrok with stable HTTPS URL

**Protocol Bridges (all on the same Express server):**

| Endpoint | Protocol | Who Uses It |
|---|---|---|
| `/` (POST, JSON-RPC) | A2A native | Any A2A agent, curl with JSON-RPC body |
| `/api/chat` (GET/POST) | REST | ChatGPT Custom GPT, browsers, curl, scripts |
| `/openapi.json` | OpenAPI 3.0 spec | ChatGPT Action editor (import URL) |
| `/mcp` | MCP Streamable HTTP | Claude, Cursor, ChatGPT Apps, any MCP client |
| `/.well-known/mcp.json` | MCP discovery | Auto-discovery by MCP-capable clients |
| `/.well-known/agent-card.json` | A2A discovery | Auto-discovery by A2A-capable agents |

**One-Click Connect Page (`agentfi.com/connect`):**

A landing page with platform-specific install buttons, each using the native zero-friction mechanism:

| Button | Mechanism | User Experience |
|---|---|---|
| **Add to Cursor** | `cursor://anysphere.cursor-deeplink/mcp/install?name=AgentFi&config=<base64>` deeplink | Click → Cursor opens → "Install AgentFi?" dialog → click Install → done |
| **Add to Claude Desktop** | `.mcpb` Desktop Extension download | Click → download .mcpb → double-click → installed |
| **Open in Claude.ai** | Link to Remote MCP Custom Connector with pre-filled URL | Click → Claude settings open → URL pre-filled → click Add → done |
| **Open in ChatGPT** | Direct link to published Custom GPT in GPT store | Click → GPT opens → start chatting immediately |
| **Try in Browser** | Inline chat widget on the connect page itself | Type message → see response → zero navigation |
| **curl** | Copy-paste snippet | Click to copy → paste in terminal → see response |
| **A2A** | curl snippet with JSON-RPC body | For developers building A2A agents |

**ChatGPT Custom GPT (published in GPT store):**
- Name: "AgentFi Agent"
- Description: "Talk to AgentFi's public A2A agent. Test agent identity, observability, and guardrails."
- Action: OpenAPI spec pointing to REST bridge (`/openapi.json`)
- Shareable link: `chatgpt.com/g/g-XXXX-agentfi`
- Anyone with ChatGPT Plus can find it in the store and start chatting. Zero setup.

**MCP Auto-Discovery:**
- `/.well-known/mcp.json` served at the public URL enables MCP clients to auto-discover the agent
- Claude Desktop Extensions (`.mcpb`) bundle the MCP server with all dependencies — double-click to install
- Cursor deeplinks pre-fill the MCP config — one click to install

The result: from Week 1, the path from "never heard of AgentFi" to "chatting with our agent" is **one click and under 60 seconds** on ChatGPT, Claude, Cursor, or any browser. No terminal, no config files, no A2A knowledge required.

**Layer 2 — Observability (Weeks 2-3)**
- Every inbound request is logged: timestamp, caller IP, caller agent info, message summary, latency
- Response now includes: `"[OBSERVED: trace-id abc123 — visit agentfi.dev/traces/abc123 to see your request]"`
- Public trace viewer at `/observability/log` — anyone can see all recent requests
- This is the seed of AgentFi Trace, running live

**Layer 3 — Identity (Weeks 4-5)**
- Agent checks for `x-agentfi-token` in message metadata
- If no token: response includes `"⚠️ You are not identified. Register at agentfi.dev/register to get an AgentFi token."`
- If valid token: response includes `"✅ Verified: agent:<name> (owner: <email>)"`
- Self-service registration endpoint: `POST /shield/register` — provide agent name and email, get back an A-JWT
- Unverified callers still get responses (monitor mode), but the observability log flags them

**Layer 4 — Guardrails (Week 6)**
- Rate limiting: max 20 requests per minute per IP
- Max message length: 2000 characters
- Blocked patterns: prompt injection attempts (`"ignore previous"`, `"system prompt"`, etc.)
- Skill allowlist: only `echo` and `ask` accepted
- Violations return a structured A2A error: `{ "error": "GUARDRAIL_VIOLATION", "rule": "rate_limit", "detail": "20 requests/min exceeded" }`
- All guardrail checks visible in the observability log

**Layer 5 — Wallet & Expense Tracking (Week 7)**
- Each registered agent gets a demo wallet with $500 play money
- `echo` skill is free. `ask` skill costs $0.01 per request (simulated).
- Response includes: `"💰 Balance: $499.99 remaining"`
- When balance hits $0: `"💸 Budget exhausted. Your wallet is empty."`
- Dashboard at `/wallet/balance` shows all registered agents' balances

**Layer 6 — LLM-Powered Responses (Week 8)**
- New skill: `ask` — powered by OpenAI GPT-4o-mini
- System prompt: "You are the AgentFi Playground Agent. Help users understand A2A, agent identity, observability, and multi-agent systems."
- Requires verified identity (Layer 3 token) to use `ask` skill
- Unverified callers get: `"🔒 The ask skill requires AgentFi identity. Register at agentfi.dev/register"`
- Costs $0.01 per request from the caller's demo wallet (Layer 5)

**Layer 7+ — Future Layers (Weeks 9-16)**
- **Delegation chains**: Agent B delegates to Agent C to call our agent on B's behalf
- **Transaction rails**: Simulated payment flows between agents
- **Transaction netting**: Multiple agent-to-agent transactions netted into settlement batches
- **Reputation**: Caller agents accumulate reputation based on behavior (rate limit compliance, valid identity, no guardrail violations)

### 21.4 Weekly LinkedIn Content Plan

Each week, post on LinkedIn with:
1. What layer was added
2. A screenshot or terminal output showing the new behavior
3. The public agent URL so people can try it
4. A curl command to test it immediately

| Week | Post Title | Hook |
|---|---|---|
| 1 | "We put an AI agent on the internet. One click to connect from ChatGPT, Claude, or Cursor." | Visit agentfi.com/connect → click one button → start chatting. Zero setup. The future of agent interop. |
| 2 | "Our agent now watches everything. Observability layer is live." | Every request is traced. See yours at agentfi.dev/traces |
| 3 | "Our agent now knows your name. Identity layer is live." | Register with AgentFi, get a token, see your identity verified in the response |
| 4 | "Try to break our agent. Guardrails are live." | Rate limits, injection protection, capability enforcement. Can you get past them? |
| 5 | "Our agent now tracks your spending. Wallet layer is live." | Every agent gets $500 play money. Watch it drain as you use the `ask` skill. |
| 6 | "Our agent can think now. LLM layer is live." | GPT-4o answers your A2A questions — but only if you're verified and have budget. |
| 7 | "Our agent now trusts delegation chains." | Agent A delegates to Agent B to talk to us. Full chain verified. |
| 8 | "AgentFi MVP is live. Here's everything we built in 8 weeks." | Recap post with full architecture, demo video, and npm links. |

### 21.5 How to Test (For the Community)

Each LinkedIn post links to `agentfi.com/connect`. Week 1 provides **one-click access** for every platform:

**The Connect Page — agentfi.com/connect**

One page, six buttons. Each uses the native install mechanism for that platform:

**Cursor (one click):**
Click "Add to Cursor" → Cursor opens with install dialog → click Install → say "use talk_to_agentfi to say hello" in any chat.

The button is a deeplink:
```
cursor://anysphere.cursor-deeplink/mcp/install?name=AgentFi&config=eyJ1cmwiOiJodHRwczovL2FnZW50ZmkuY29tL21jcCJ9
```

**Claude Desktop (two clicks):**
Click "Add to Claude Desktop" → downloads `agentfi.mcpb` → double-click the file → Claude installs the extension → say "Hi AgentFi" in any conversation.

**ChatGPT (one click):**
Click "Open in ChatGPT" → opens the AgentFi GPT directly → start chatting. No setup, no config, no Actions to configure. The GPT is pre-published in the store.

Link: `https://chatgpt.com/g/g-XXXX-agentfi`

**Claude.ai (paste URL):**
Click "Open in Claude.ai" → navigates to Claude Settings → MCP → Add Custom Connector with URL pre-filled → click Add → say "Hi AgentFi."

**Browser (zero clicks):**
The connect page itself has an inline chat widget. Type a message, see the response. No navigation needed.

**curl (copy-paste):**
```bash
curl "https://agentfi.com/api/chat?message=Hello"
# → "Hello agent! I received your message: Hello"
```

**A2A native (for agent developers):**
```bash
# Discover the agent
curl https://agentfi.com/.well-known/agent-card.json

# Talk via A2A protocol
curl -X POST https://agentfi.com \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0", "id": 1,
    "method": "message/send",
    "params": {
      "message": {
        "kind": "message",
        "messageId": "test-1",
        "role": "user",
        "parts": [{"kind": "text", "text": "Hello from my agent!"}]
      },
      "configuration": {"blocking": true}
    }
  }'
```

By Week 5, the test becomes:

```bash
# Register with AgentFi
curl -X POST https://<ngrok-url>/shield/register \
  -H "Content-Type: application/json" \
  -d '{"name": "My Test Agent", "email": "dev@example.com"}'
# → Returns: { "token": "eyJ...", "agentId": "agent:abc123" }

# Talk with identity
curl -X POST https://<ngrok-url> \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0", "id": 1,
    "method": "message/send",
    "params": {
      "message": {
        "kind": "message",
        "messageId": "test-2",
        "role": "user",
        "parts": [{"kind": "text", "text": "What is the A2A protocol?"}],
        "metadata": {
          "x-agentfi-token": "eyJ..."
        }
      },
      "configuration": {"blocking": true}
    }
  }'
# Response includes: "✅ Verified: agent:My Test Agent | 💰 Balance: $499.99 | 🤖 The A2A protocol is..."
```

### 21.6 Why This Strategy Works

1. **One-click access from day one** — `agentfi.com/connect` gives ChatGPT, Claude, Cursor, and browser users a single-click path to our agent. This maximizes the audience for every LinkedIn post — not just developers, but anyone with ChatGPT or Claude.
2. **The connect page IS the product demo** — it proves that AgentFi makes agent discovery and connection trivial. The connect page itself is a preview of what the AgentFi registry will become: a place where any agent can be discovered and connected to in one click.
3. **Build in public** creates accountability and momentum — we ship every week, visibly
4. **Each post is a mini-launch** — 8 opportunities for engagement instead of one big-bang launch
5. **The agent IS the demo** — no slides, no mockups, just click and chat
6. **Community testing finds bugs** — real users hitting our endpoint surfaces issues before launch
7. **Progressive complexity tells a story** — from "Hello agent!" to full identity + guardrails + LLM shows the complete AgentFi value proposition unfolding in real time
8. **Content compounds** — by Week 8, we have 8 LinkedIn posts and a community of people who've been following along
9. **Cross-protocol proof** — A2A, REST, MCP, and OpenAPI all work from Week 1. This is a live demonstration that AgentFi is transport-agnostic — not just an A2A tool.
10. **GPT store presence** — the Custom GPT in the ChatGPT store gives us discoverability beyond our own network. Anyone searching "A2A agent" or "agent interop" in the store finds us.

---

*This document is a living artifact. Update as we learn from building, user feedback, and market shifts.*
*Next action: Begin Week 1 execution per Section 13. Deploy public agent per Section 21.*
