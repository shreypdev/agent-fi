export const PORTS = {
  EXPEDIA: 3001,
  MARRIOTT: 3002,
  RESTAURANT: 3003,
  TRAVEL_PLANNER: 3004,
  PERSONAL: 3005,
} as const;

export const AGENT_CARD_PATH = ".well-known/agent-card.json";

export const AGENT_URLS = {
  EXPEDIA: `http://localhost:${PORTS.EXPEDIA}`,
  MARRIOTT: `http://localhost:${PORTS.MARRIOTT}`,
  RESTAURANT: `http://localhost:${PORTS.RESTAURANT}`,
  TRAVEL_PLANNER: `http://localhost:${PORTS.TRAVEL_PLANNER}`,
  PERSONAL: `http://localhost:${PORTS.PERSONAL}`,
} as const;

export function agentCardUrl(baseUrl: string): string {
  return `${baseUrl}/${AGENT_CARD_PATH}`;
}

export const SUPPORTED_CITIES = ["rome", "florence", "venice", "milan"] as const;
export type City = (typeof SUPPORTED_CITIES)[number];
