import { PORTS, SUPPORTED_CITIES, type City } from "../config.js";
import { RESTAURANTS, type Restaurant } from "../data/restaurants.js";
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

function formatRestaurants(city: City, restaurants: Restaurant[]): string {
  const header = `🍝  Restaurants in ${city.charAt(0).toUpperCase() + city.slice(1)}:\n`;
  const rows = restaurants.map(
    (r, i) =>
      `  ${i + 1}. ${r.name} (${r.cuisine}) — ${r.neighborhood}\n` +
      `     ${r.priceRange} | Rating: ${r.rating}/5\n` +
      `     Must try: ${r.mustTry}`
  );
  return header + rows.join("\n");
}

class RestaurantExecutor implements AgentExecutor {
  async execute(ctx: RequestContext, eventBus: ExecutionEventBus): Promise<void> {
    const text = extractText(ctx.userMessage);
    const city = findCity(text);

    if (!city) {
      publishFailed(eventBus, ctx, `Could not find restaurants. Supported cities: ${SUPPORTED_CITIES.join(", ")}`);
      return;
    }

    const restaurants = RESTAURANTS[city];
    publishCompleted(eventBus, ctx, formatRestaurants(city, restaurants));
  }

  async cancelTask(_taskId: string, eventBus: ExecutionEventBus): Promise<void> {
    eventBus.finished();
  }
}

export const restaurantAgentCard: AgentCard = {
  name: "Restaurant Agent",
  description: "Recommends restaurants and dining experiences in Italian cities.",
  url: `http://localhost:${PORTS.RESTAURANT}`,
  version: "1.0.0",
  protocolVersion: "0.2.1",
  capabilities: { streaming: false, pushNotifications: false },
  defaultInputModes: ["text/plain"],
  defaultOutputModes: ["text/plain"],
  skills: [
    {
      id: "recommend_restaurants",
      name: "Recommend Restaurants",
      description: "Find top restaurant recommendations in an Italian city",
      tags: ["restaurants", "dining", "food"],
      examples: ["Recommend restaurants in Venice", "Where to eat in Milan"],
    },
  ],
};

export function startRestaurantAgent(): void {
  startAgentServer(restaurantAgentCard, new RestaurantExecutor(), PORTS.RESTAURANT);
}
