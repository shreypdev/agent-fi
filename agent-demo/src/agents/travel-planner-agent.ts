import { A2AClient } from "@a2a-js/sdk/client";
import { PORTS, AGENT_URLS, SUPPORTED_CITIES, agentCardUrl, type City } from "../config.js";
import {
  extractText,
  makeUserMessage,
  initTask,
  publishCompleted,
  publishFailed,
  startAgentServer,
  uuidv4,
  type AgentCard,
  type AgentExecutor,
  type RequestContext,
  type ExecutionEventBus,
  type Task,
  type TaskStatusUpdateEvent,
} from "../helpers.js";

function extractCities(text: string): City[] {
  const lower = text.toLowerCase();
  const found = SUPPORTED_CITIES.filter((c) => lower.includes(c));
  return found.length > 0 ? found : ["rome", "florence", "venice"];
}

async function queryAgent(agentUrl: string, queryText: string): Promise<string> {
  const client = await A2AClient.fromCardUrl(agentCardUrl(agentUrl));
  const response = await client.sendMessage({
    message: makeUserMessage(queryText),
    configuration: { blocking: true },
  });

  if ("error" in response) {
    return `[Error from ${agentUrl}: ${JSON.stringify(response.error)}]`;
  }

  const result = response.result;

  if (result.kind === "message") {
    for (const part of result.parts) {
      if (part.kind === "text") return part.text;
    }
    return "[No text in message response]";
  }

  if (result.kind === "task") {
    const task = result as Task;
    const msg = task.status?.message;
    if (msg) {
      for (const part of msg.parts) {
        if (part.kind === "text") return part.text;
      }
    }
    return "[No text in task response]";
  }

  return "[Unknown response format]";
}

function buildItinerary(
  cities: City[],
  flightResults: Map<City, string>,
  hotelResults: Map<City, string>,
  restaurantResults: Map<City, string>
): string {
  const lines: string[] = [];
  lines.push("═══════════════════════════════════════════════════");
  lines.push("        🇮🇹  YOUR ITALY TRAVEL ITINERARY  🇮🇹");
  lines.push("═══════════════════════════════════════════════════\n");

  const daysPerCity = Math.max(1, Math.floor(5 / cities.length));
  let dayCounter = 1;

  for (const city of cities) {
    const cityTitle = city.charAt(0).toUpperCase() + city.slice(1);
    const startDay = dayCounter;
    const endDay = dayCounter + daysPerCity - 1;

    lines.push(`━━━  Day ${startDay}${endDay > startDay ? `–${endDay}` : ""}: ${cityTitle}  ━━━\n`);

    const flights = flightResults.get(city);
    if (flights) {
      lines.push(flights);
      lines.push("");
    }

    const hotels = hotelResults.get(city);
    if (hotels) {
      lines.push(hotels);
      lines.push("");
    }

    const restaurants = restaurantResults.get(city);
    if (restaurants) {
      lines.push(restaurants);
      lines.push("");
    }

    dayCounter += daysPerCity;
  }

  lines.push("═══════════════════════════════════════════════════");
  lines.push("  Itinerary assembled by TravelPlannerAgent");
  lines.push("  Data sourced via A2A from: Expedia, Marriott, Restaurant agents");
  lines.push("═══════════════════════════════════════════════════");

  return lines.join("\n");
}

class TravelPlannerExecutor implements AgentExecutor {
  async execute(ctx: RequestContext, eventBus: ExecutionEventBus): Promise<void> {
    const text = extractText(ctx.userMessage);
    const cities = extractCities(text);

    console.log(`[TravelPlannerAgent] Planning itinerary for: ${cities.join(", ")}`);

    initTask(eventBus, ctx);

    const workingUpdate: TaskStatusUpdateEvent = {
      kind: "status-update",
      taskId: ctx.taskId,
      contextId: ctx.contextId,
      final: false,
      status: {
        state: "working",
        timestamp: new Date().toISOString(),
        message: {
          kind: "message",
          messageId: uuidv4(),
          role: "agent",
          parts: [{ kind: "text", text: `Planning Italy trip for cities: ${cities.join(", ")}. Contacting service agents...` }],
          taskId: ctx.taskId,
          contextId: ctx.contextId,
        },
      },
    };
    eventBus.publish(workingUpdate);

    try {
      const flightResults = new Map<City, string>();
      const hotelResults = new Map<City, string>();
      const restaurantResults = new Map<City, string>();

      for (const city of cities) {
        console.log(`[TravelPlannerAgent] Querying agents for ${city}...`);

        const [flightResp, hotelResp, restaurantResp] = await Promise.all([
          queryAgent(AGENT_URLS.EXPEDIA, `Find flights to ${city}`),
          queryAgent(AGENT_URLS.MARRIOTT, `Find hotels in ${city}`),
          queryAgent(AGENT_URLS.RESTAURANT, `Recommend restaurants in ${city}`),
        ]);

        flightResults.set(city, flightResp);
        hotelResults.set(city, hotelResp);
        restaurantResults.set(city, restaurantResp);
      }

      const itinerary = buildItinerary(cities, flightResults, hotelResults, restaurantResults);
      publishCompleted(eventBus, ctx, itinerary, true);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      publishFailed(eventBus, ctx, `Failed to build itinerary: ${msg}`, true);
    }
  }

  async cancelTask(_taskId: string, eventBus: ExecutionEventBus): Promise<void> {
    eventBus.finished();
  }
}

export const travelPlannerAgentCard: AgentCard = {
  name: "Travel Planner Agent",
  description:
    "Orchestrates travel planning by querying Expedia, Marriott, and Restaurant agents via A2A, then assembles a complete day-by-day itinerary.",
  url: `http://localhost:${PORTS.TRAVEL_PLANNER}`,
  version: "1.0.0",
  protocolVersion: "0.2.1",
  capabilities: { streaming: false, pushNotifications: false },
  defaultInputModes: ["text/plain"],
  defaultOutputModes: ["text/plain"],
  skills: [
    {
      id: "plan_itinerary",
      name: "Plan Itinerary",
      description: "Create a full travel itinerary for Italian cities including flights, hotels, and dining",
      tags: ["travel", "itinerary", "planning"],
      examples: [
        "Plan a 5-day Italy trip visiting Rome, Florence, and Venice",
        "Create an itinerary for Milan and Rome",
      ],
    },
  ],
};

export function startTravelPlannerAgent(): void {
  startAgentServer(travelPlannerAgentCard, new TravelPlannerExecutor(), PORTS.TRAVEL_PLANNER);
}
