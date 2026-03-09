/**
 * Roadmap data — single source for the landing page.
 * Keep in sync with root TODO.md when completing tasks.
 */
export interface RoadmapItem {
  id: string;
  week: number;
  title: string;
  done: boolean;
  detail?: string;
}

export const roadmapByWeek: { week: number; label: string; items: RoadmapItem[] }[] = [
  {
    week: 0,
    label: "Prep",
    items: [
      { id: "0-1", week: 0, title: "Monorepo setup", done: true },
    ],
  },
  {
    week: 1,
    label: "Public Agent + Landing + Connect",
    items: [
      { id: "1-1", week: 1, title: "Public Agent — Echo, Agent Card", done: true },
      { id: "1-2", week: 1, title: "REST bridge + OpenAPI", done: true },
      { id: "1-3", week: 1, title: "Remote MCP server, .mcpb for Claude", done: false },
      { id: "1-4", week: 1, title: "ChatGPT Custom GPT / GPT Store", done: false },
      { id: "1-5", week: 1, title: "Landing page + Connect page", done: false },
      { id: "1-6", week: 1, title: "One-click: Cursor, Claude, ChatGPT, Browser", done: false },
      { id: "1-7", week: 1, title: "Internal test harness verified", done: true },
      { id: "1-8", week: 1, title: "LinkedIn Post #1", done: false },
    ],
  },
  {
    week: 2,
    label: "Observability SDK + Registry",
    items: [
      { id: "2-1", week: 2, title: "Turborepo, packages: common, trace, shield, collector", done: false },
      { id: "2-2", week: 2, title: "@agentfi/common — spans, A-JWT, delegation types", done: false },
      { id: "2-3", week: 2, title: "Ed25519 crypto helpers", done: false },
      { id: "2-4", week: 2, title: "@agentfi/trace — A2A middleware + client", done: false },
      { id: "2-5", week: 2, title: "Collector MVP — OTLP, SQLite, span API", done: false },
      { id: "2-6", week: 2, title: "@agentfi/shield — Registry, A-JWT, JWKS", done: false },
      { id: "2-7", week: 2, title: "Instrument travel agents with trace + shield", done: false },
      { id: "2-8", week: 2, title: "Public Agent Layer 2 — Observability", done: false },
      { id: "2-9", week: 2, title: "LinkedIn Post #2", done: false },
    ],
  },
  {
    week: 3,
    label: "Dashboard: Trace Views",
    items: [
      { id: "3-1", week: 3, title: "Dashboard shell — /traces, /agents, /compliance", done: false },
      { id: "3-2", week: 3, title: "Trace list page", done: false },
      { id: "3-3", week: 3, title: "Trace waterfall view", done: false },
      { id: "3-4", week: 3, title: "Span detail panel", done: false },
      { id: "3-5", week: 3, title: "Trace call graph", done: false },
      { id: "3-6", week: 3, title: "Public Agent Layer 3 — Identity", done: false },
      { id: "3-7", week: 3, title: "LinkedIn Post #3", done: false },
    ],
  },
  {
    week: 4,
    label: "Shield Views + Guardrails",
    items: [
      { id: "4-1", week: 4, title: "Agent list + detail pages", done: false },
      { id: "4-2", week: 4, title: "Identity badges on spans", done: false },
      { id: "4-3", week: 4, title: "Token history page", done: false },
      { id: "4-4", week: 4, title: "Dashboard polish", done: false },
      { id: "4-5", week: 4, title: "Public Agent Layer 4 — Guardrails", done: false },
      { id: "4-6", week: 4, title: "LinkedIn Post #4", done: false },
    ],
  },
  {
    week: 5,
    label: "Delegation + MCP",
    items: [
      { id: "5-1", week: 5, title: "Delegation API + chain verification", done: false },
      { id: "5-2", week: 5, title: "MCP transport adapter", done: false },
      { id: "5-3", week: 5, title: "Public Agent Layer 5 — Wallet", done: false },
      { id: "5-4", week: 5, title: "LinkedIn Post #5", done: false },
    ],
  },
  {
    week: 6,
    label: "Spend + Human Approval",
    items: [
      { id: "6-1", week: 6, title: "Spend ledger + limit enforcement", done: false },
      { id: "6-2", week: 6, title: "Human-in-the-loop approval", done: false },
      { id: "6-3", week: 6, title: "Public Agent Layer 6 — LLM", done: false },
      { id: "6-4", week: 6, title: "LinkedIn Post #6", done: false },
    ],
  },
  {
    week: 7,
    label: "Alerting + Redaction",
    items: [
      { id: "7-1", week: 7, title: "Alert rules + delivery", done: false },
      { id: "7-2", week: 7, title: "PII redaction, health metrics", done: false },
      { id: "7-3", week: 7, title: "Public Agent Layer 7 — Delegation chains", done: false },
      { id: "7-4", week: 7, title: "LinkedIn Post #7", done: false },
    ],
  },
  {
    week: 8,
    label: "Launch",
    items: [
      { id: "8-1", week: 8, title: "Documentation site", done: false },
      { id: "8-2", week: 8, title: "Landing + demo video", done: false },
      { id: "8-3", week: 8, title: "npm publish @agentfi/trace, @agentfi/shield", done: false },
      { id: "8-4", week: 8, title: "Docker images, docker compose up", done: false },
      { id: "8-5", week: 8, title: "Public Agent — Full stack, all 7 layers", done: false },
      { id: "8-6", week: 8, title: "Community launch + LinkedIn Post #8", done: false },
    ],
  },
];

export const totalWeeks = 9;
export const completedCount = roadmapByWeek.flatMap((g) => g.items).filter((i) => i.done).length;
export const totalCount = roadmapByWeek.flatMap((g) => g.items).length;
