import { ensureAbsoluteHttpUrl } from "./absoluteHttpUrl";

/** Base URL for searchd (no trailing slash). Empty if unset. */
export function searchApiBase(): string {
  const raw = import.meta.env.VITE_SEARCH_API_BASE_URL;
  if (raw == null || String(raw).trim() === "") return "";
  return ensureAbsoluteHttpUrl(String(raw));
}

export function isSearchApiConfigured(): boolean {
  return searchApiBase().length > 0;
}

/** MCP Streamable HTTP URL on searchd (same origin as REST). */
export function searchMcpUrl(): string {
  const b = searchApiBase();
  if (!b) return "";
  return `${b}/mcp`;
}

export function searchMcpManifestUrl(): string {
  const b = searchApiBase();
  if (!b) return "";
  return `${b}/.well-known/mcp.json`;
}

export type SearchResultItem = {
  agent_id: string;
  name: string;
  description: string;
  endpoint_url: string;
  protocol_version: string;
  trust_tier: string;
  provider_display_name?: string | null;
  snippet?: string | null;
};

export type SearchResponse = {
  results: SearchResultItem[];
  total_hits_estimate?: number | null;
  query_time_ms: number;
};

export async function postSearch(
  query: string,
  limit = 10,
  offset = 0,
): Promise<SearchResponse> {
  const base = searchApiBase();
  const res = await fetch(`${base}/v1/search`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ query, limit, offset }),
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`search ${res.status}: ${text}`);
  }
  return res.json() as Promise<SearchResponse>;
}

export type AgentDetail = {
  agent_id: string;
  name: string;
  description: string;
  endpoint_url: string;
  protocol_version: string;
  trust_tier: string;
  provider_display_name?: string | null;
  card_json: unknown;
};

export async function getAgent(id: string): Promise<AgentDetail> {
  const base = searchApiBase();
  const res = await fetch(`${base}/v1/agents/${encodeURIComponent(id)}`);
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`agent ${res.status}: ${text}`);
  }
  return res.json() as Promise<AgentDetail>;
}
