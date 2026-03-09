import { PORTS, SUPPORTED_CITIES, type City } from "../config.js";
import { HOTELS, type Hotel } from "../data/hotels.js";
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

function formatHotels(city: City, hotels: Hotel[]): string {
  const header = `🏨  Hotels in ${city.charAt(0).toUpperCase() + city.slice(1)}:\n`;
  const rows = hotels.map(
    (h, i) =>
      `  ${i + 1}. ${h.name} ${"★".repeat(h.stars)} — ${h.neighborhood}\n` +
      `     $${h.pricePerNight}/night | ${h.highlights.join(", ")}`
  );
  return header + rows.join("\n");
}

class MarriottExecutor implements AgentExecutor {
  async execute(ctx: RequestContext, eventBus: ExecutionEventBus): Promise<void> {
    const text = extractText(ctx.userMessage);
    const city = findCity(text);

    if (!city) {
      publishFailed(eventBus, ctx, `Could not find hotels. Supported cities: ${SUPPORTED_CITIES.join(", ")}`);
      return;
    }

    const hotels = HOTELS[city];
    publishCompleted(eventBus, ctx, formatHotels(city, hotels));
  }

  async cancelTask(_taskId: string, eventBus: ExecutionEventBus): Promise<void> {
    eventBus.finished();
  }
}

export const marriottAgentCard: AgentCard = {
  name: "Marriott Agent",
  description: "Searches and books hotels in Italian cities. Powered by Marriott.",
  url: `http://localhost:${PORTS.MARRIOTT}`,
  version: "1.0.0",
  protocolVersion: "0.2.1",
  capabilities: { streaming: false, pushNotifications: false },
  defaultInputModes: ["text/plain"],
  defaultOutputModes: ["text/plain"],
  skills: [
    {
      id: "search_hotels",
      name: "Search Hotels",
      description: "Find available hotels in an Italian city",
      tags: ["hotels", "accommodation", "booking"],
      examples: ["Find hotels in Florence", "Search hotels in Rome"],
    },
  ],
};

export function startMarriottAgent(): void {
  startAgentServer(marriottAgentCard, new MarriottExecutor(), PORTS.MARRIOTT);
}
