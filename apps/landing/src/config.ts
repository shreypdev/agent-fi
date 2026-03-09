/**
 * Shared config for the landing app.
 * Override via env: VITE_PUBLIC_AGENT_URL, VITE_CONNECT_URL.
 * In dev, defaults to localhost:3010 so "Try in Browser" works with a local public-agent.
 */
const PRODUCTION_AGENT_URL = "https://pronox-public-agent.up.railway.app";
const LOCAL_AGENT_URL = "http://localhost:3010";

export const PUBLIC_AGENT_URL =
  import.meta.env.VITE_PUBLIC_AGENT_URL ??
  (import.meta.env.DEV ? LOCAL_AGENT_URL : PRODUCTION_AGENT_URL);

export const CONNECT_PATH = "/connect";
