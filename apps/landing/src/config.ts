import { ensureAbsoluteHttpUrl } from "./lib/absoluteHttpUrl";

/**
 * Shared config for the landing app.
 * Override via env: VITE_PUBLIC_AGENT_URL, VITE_CONNECT_URL.
 * In dev, defaults to localhost:3010 so "Try in Browser" works with a local public-agent.
 */
const PRODUCTION_AGENT_URL = "https://pronox-public-agent.up.railway.app";
const LOCAL_AGENT_URL = "http://localhost:3010";

const agentFromEnv = import.meta.env.VITE_PUBLIC_AGENT_URL;

export const PUBLIC_AGENT_URL =
  agentFromEnv != null && String(agentFromEnv).trim() !== ""
    ? ensureAbsoluteHttpUrl(String(agentFromEnv))
    : import.meta.env.DEV
      ? LOCAL_AGENT_URL
      : PRODUCTION_AGENT_URL;

export const CONNECT_PATH = "/connect";

export const SEARCH_PATH = "/search";

/** React Router pattern for agent detail (id param). */
export const AGENT_DETAIL_PATH = "/agents/:id";
