import { startExpediaAgent } from "./agents/expedia-agent.js";
import { startMarriottAgent } from "./agents/marriott-agent.js";
import { startRestaurantAgent } from "./agents/restaurant-agent.js";
import { startTravelPlannerAgent } from "./agents/travel-planner-agent.js";

console.log("═══════════════════════════════════════════════════");
console.log("   Starting A2A Travel Service Agents...");
console.log("═══════════════════════════════════════════════════\n");

startExpediaAgent();
startMarriottAgent();
startRestaurantAgent();
startTravelPlannerAgent();

console.log("\nAll agents starting. Press Ctrl+C to stop.\n");
