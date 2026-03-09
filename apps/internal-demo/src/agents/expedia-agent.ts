import { PORTS, SUPPORTED_CITIES, type City } from "../config.js";
import { FLIGHTS, type Flight } from "../data/flights.js";
import {
  extractText,
  publishCompleted,
  publishFailed,
  startAgentServer,
  type AgentCard,
  type AgentExecutor,
  type RequestContext,
  type ExecutionEventBus,
} from "../helpers.js";

function findCity(text: string): City | undefined {
  const lower = text.toLowerCase();
  return SUPPORTED_CITIES.find((c) => lower.includes(c));
}

function formatFlights(city: City, flights: Flight[]): string {
  const header = `✈️  Flights to ${city.charAt(0).toUpperCase() + city.slice(1)}:\n`;
  const rows = flights.map(
    (f, i) =>
      `  ${i + 1}. ${f.airline} | ${f.from} → ${f.to}\n` +
      `     Departs ${f.departure}, arrives ${f.arrival} (${f.duration}, ${f.stops === 0 ? "nonstop" : f.stops + " stop"})\n` +
      `     $${f.price}`
  );
  return header + rows.join("\n");
}

class ExpediaExecutor implements AgentExecutor {
  async execute(ctx: RequestContext, eventBus: ExecutionEventBus): Promise<void> {
    const text = extractText(ctx.userMessage);
    const city = findCity(text);

    if (!city) {
      publishFailed(eventBus, ctx, `Could not find flights. Supported cities: ${SUPPORTED_CITIES.join(", ")}`);
      return;
    }

    const flights = FLIGHTS[city];
    publishCompleted(eventBus, ctx, formatFlights(city, flights));
  }

  async cancelTask(_taskId: string, eventBus: ExecutionEventBus): Promise<void> {
    eventBus.finished();
  }
}

export const expediaAgentCard: AgentCard = {
  name: "Expedia Agent",
  description: "Searches and books flights to Italian cities. Powered by Expedia.",
  url: `http://localhost:${PORTS.EXPEDIA}`,
  version: "1.0.0",
  protocolVersion: "0.2.1",
  capabilities: { streaming: false, pushNotifications: false },
  defaultInputModes: ["text/plain"],
  defaultOutputModes: ["text/plain"],
  skills: [
    {
      id: "search_flights",
      name: "Search Flights",
      description: "Find available flights to an Italian city",
      tags: ["flights", "travel", "booking"],
      examples: ["Find flights to Rome", "Search flights to Venice"],
    },
  ],
};

export function startExpediaAgent(): void {
  startAgentServer(expediaAgentCard, new ExpediaExecutor(), PORTS.EXPEDIA);
}
