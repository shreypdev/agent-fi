# AgentFi — Root TODO

Single source of truth for milestone progress. Update checkboxes as tasks complete. See [MILESTONES.md](MILESTONES.md) for full details.

---

## Week 0 / Prep

- [x] **Monorepo setup** — Turborepo + workspaces. Rename agent-demo → apps/internal-demo. Scaffold apps/public-agent. Done when: `npm run services` and `npm run playground` work from root.

---

## Week 1 — Public Agent + Landing Page + Connect Page

- [x] **1.1** Public Agent — Echo. A2A on 3010, Agent Card at `/.well-known/agent-card.json`, skill `echo`. Expose via ngrok.
- [x] **1.2** REST bridge + OpenAPI. `GET/POST /api/chat?message=<text>`, `/openapi.json`. Translates REST to A2A.
- [ ] **1.3** Remote MCP server. `/mcp`, tool `talk_to_agentfi(message)`, `/.well-known/mcp.json`. Package as `.mcpb` for Claude.
- [ ] **1.4** ChatGPT Custom GPT. Publish "AgentFi Agent" on GPT store with OpenAPI action. Shareable link.
- [ ] **1.5** agentfi.com landing page. Hero, teaser, email waitlist, link to connect.
- [ ] **1.6** agentfi.com/connect page. One-click: Add to Cursor, Add to Claude, Open in ChatGPT, Try in Browser, curl, A2A.
- [x] **1.7** Internal test harness. Verify 5 travel agents run. Document running services + playground side by side.
- [ ] **1.8** LinkedIn Post #1. "We put an AI agent on the internet. One click to connect."

---

## Week 2 — Core Types + Observability SDK + Registry

- [ ] **2.1** Turborepo monorepo with pnpm workspaces. Packages: common, trace-sdk, shield-sdk, collector, registry, dashboard. CI: lint + typecheck.
- [ ] **2.2** `@agentfi/common` — AgentFiSpan, RegisteredAgent, AgentFiJWT, DelegationLink, AgentCapability. JSDoc.
- [ ] **2.3** Ed25519 crypto helpers. Unit tests: generate → sign → verify → JWKS.
- [ ] **2.4** `@agentfi/trace` — A2A server middleware + client. Spans, traceparent in metadata.
- [ ] **2.5** Collector MVP — OTLP/HTTP receiver, SQLite, span query API.
- [ ] **2.6** `@agentfi/shield` — Registry API, A-JWT issuance, JWKS endpoint.
- [ ] **2.7** Instrument internal-demo travel agents with trace + shield.
- [ ] **2.8** Public Agent Layer 2 — Observability. Response includes trace ID. `/observability/log` viewer.
- [ ] **2.9** LinkedIn Post #2. "Our agent now watches everything."

---

## Week 3 — Dashboard: Trace Views

- [ ] **3.1** Dashboard shell — React + Vite. Routes: /traces, /traces/:id, /agents, /compliance. Sidebar.
- [ ] **3.2** Trace list page — paginated table, search by trace ID / agent name.
- [ ] **3.3** Trace waterfall view — timeline bars, nested, green/red, duration labels.
- [ ] **3.4** Span detail panel — source/target, transport, A2A state, latency, error.
- [ ] **3.5** Trace call graph — nodes = agents, edges = calls with latency.
- [ ] **3.6** Public Agent Layer 3 — Identity. x-agentfi-token check. "Verified" or "Not identified". POST /shield/register.
- [ ] **3.7** LinkedIn Post #3. "Our agent now knows your name."

---

## Week 4 — Dashboard: Shield Views + Polish

- [ ] **4.1** Agent list page — registered agents table. Filter by status/owner.
- [ ] **4.2** Agent detail page — edit capabilities, suspend/revoke.
- [ ] **4.3** Identity badges on trace spans — verified / unverified / denied.
- [ ] **4.4** Token history page — A-JWTs issued per agent.
- [ ] **4.5** Dashboard polish — skeletons, error/empty states, responsive, auto-refresh.
- [ ] **4.6** Internal demo recording — 3-min screen recording.
- [ ] **4.7** Public Agent Layer 4 — Guardrails. Rate limit, max length, blocked patterns, skill allowlist.
- [ ] **4.8** LinkedIn Post #4. "Try to break our agent. Guardrails are live."

---

## Week 5 — Delegation Chains + MCP Support

- [ ] **5.1** Delegation API — POST/GET/DELETE /delegations.
- [ ] **5.2** Delegation chain verification. Constraint narrowing.
- [ ] **5.3** Cascade revocation.
- [ ] **5.4** Delegation chain in A-JWT. Shield verifies full chain.
- [ ] **5.5** MCP transport adapter — trace context, tools/call spans.
- [ ] **5.6** Dashboard delegation view — chain diagram.
- [ ] **5.7** Public Agent Layer 5 — Wallet. $500 demo balance, echo free, ask $0.01.
- [ ] **5.8** LinkedIn Post #5. "Our agent now tracks your spending."

---

## Week 6 — Expense Tracking + Human Approval

- [ ] **6.1** Spend ledger — per agent, delegation, action. SQLite.
- [ ] **6.2** Spend limit enforcement in Shield middleware.
- [ ] **6.3** Spend threshold webhooks — 50%, 80%, 100%.
- [ ] **6.4** Human-in-the-loop approval. Webhook, approve/reject callback.
- [ ] **6.5** Spend dashboard — budget utilization, charts.
- [ ] **6.6** Shielded A2A client — auto A-JWT, refresh on expiry.
- [ ] **6.7** Public Agent Layer 6 — LLM. Skill `ask` (GPT-4o-mini), requires identity, $0.01.
- [ ] **6.8** LinkedIn Post #6. "Our agent can think now."

---

## Week 7 — Alerting + Redaction + Metrics

- [ ] **7.1** Alert rules engine — error rate, latency p95, unreachable.
- [ ] **7.2** Alert delivery — webhook (Slack, Discord, PagerDuty).
- [ ] **7.3** PII redaction engine — patterns, SDK-level redaction.
- [ ] **7.4** Agent health metrics — req/s, latency, error rate. Time-series charts.
- [ ] **7.5** Alert management UI.
- [ ] **7.6** Compliance posture widget — EU AI Act Art 12/13/14.
- [ ] **7.7** Public Agent Layer 7 — Delegation chains. Full chain verified in response.
- [ ] **7.8** LinkedIn Post #7. "Our agent now trusts delegation chains."

---

## Week 8 — Launch

- [ ] **8.1** Documentation site — quickstart, API reference, integration guides.
- [ ] **8.2** Landing page (agentfi.dev) — hero, demo video, waitlist.
- [ ] **8.3** Demo video — 3-min screencast.
- [ ] **8.4** npm publish — @agentfi/trace, @agentfi/shield.
- [ ] **8.5** Docker images — collector, registry. docker compose up.
- [ ] **8.6** Launch blog post.
- [ ] **8.7** Community launch — HN, Reddit, Twitter, LinkedIn, Product Hunt.
- [ ] **8.8** Public Agent — Full stack. All 7 layers active.
- [ ] **8.9** LinkedIn Post #8. "AgentFi MVP is live. 8 weeks, 7 layers."

---

## Week 9 — OTEL Export + HTTP/gRPC Transports

- [ ] **9.1** OTEL Collector export — OTLP to Datadog/Grafana/Jaeger.
- [ ] **9.2** HTTP transport adapter — OTEL + AgentFi semantics.
- [ ] **9.3** gRPC transport adapter.
- [ ] **9.4** Shield for MCP — A-JWT in params._meta.
- [ ] **9.5** Public Agent Layer 8 — Transaction rails. Paid skill, receipt.
- [ ] **9.6** LinkedIn Post #9. "Agent-to-agent payments are live."

---

## Week 10 — Reputation + Postgres + Multi-Tenancy

- [ ] **10.1** Agent reputation scoring (0-100).
- [ ] **10.2** PostgreSQL storage — collector + registry. Storage abstraction.
- [ ] **10.3** Multi-tenancy — workspaces, invitations, RBAC.
- [ ] **10.4** Trace diffing — side-by-side comparison.
- [ ] **10.5** Public Agent Layer 9 — Reputation in response.
- [ ] **10.6** LinkedIn Post #10. "Agents now have credit scores."

---

## Week 11 — Framework Adapters + Python SDK

- [ ] **11.1** LangGraph adapter — AgentFi spans.
- [ ] **11.2** CrewAI adapter — delegation traces.
- [ ] **11.3** Python SDK (trace) — agentfi-trace PyPI. FastAPI/Flask middleware.
- [ ] **11.4** Python SDK (shield) — agentfi-shield PyPI.
- [ ] **11.5** Public Agent Layer 10 — Transaction netting.
- [ ] **11.6** LinkedIn Post #11. "47 transactions, 1 settlement."

---

## Week 12 — Compliance Reports + Security Hardening

- [ ] **12.1** Compliance report generator — PDF/JSON, EU AI Act Art 12-14.
- [ ] **12.2** Token revocation list — real-time revocation check.
- [ ] **12.3** Audit log — immutable registry operations.
- [ ] **12.4** HTTPS enforcement. Certificate docs.
- [ ] **12.5** Public Agent Layer 11 — GET /compliance/report.
- [ ] **12.6** LinkedIn Post #12. "Your agent's EU AI Act compliance report."

---

## Week 13 — Multi-Org Federation + SSO

- [ ] **13.1** Multi-org federation — cross-org delegation, JWKS trust.
- [ ] **13.2** SSO/SAML — Okta, Azure AD, Google Workspace.
- [ ] **13.3** Policy-as-code — YAML/JSON, agentfi apply CLI.

---

## Week 14 — Anomaly Detection + Scaling

- [ ] **14.1** Anomaly detection — latency, error rate, spending. Auto-alerts.
- [ ] **14.2** Horizontal scaling — collector LB, registry read replicas.
- [ ] **14.3** Trace sampling — head/tail, configurable strategies.

---

## Week 15 — Cloud KMS + IaC + AutoGen Adapter

- [ ] **15.1** Cloud KMS — Ed25519 in AWS/GCP/Azure. Key rotation.
- [ ] **15.2** Terraform provider — agentfi_agent, agentfi_delegation, agentfi_alert_rule.
- [ ] **15.3** AutoGen adapter — group chat as AgentFi spans.

---

## Week 16 — Enterprise Launch

- [ ] **16.1** Enterprise landing page — CISOs, compliance, ROI, "Book a demo."
- [ ] **16.2** EU AI Act compliance whitepaper — 15-page PDF.
- [ ] **16.3** SOC 2 Type I preparation.
- [ ] **16.4** SLA and support tiers.
- [ ] **16.5** Enterprise outreach — 20 EU companies, pilots.
