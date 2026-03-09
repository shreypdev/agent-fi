import readline from "node:readline";
import { A2AClient } from "@a2a-js/sdk/client";
import { AGENT_URLS, agentCardUrl } from "./config.js";
import { makeUserMessage, type Task } from "./helpers.js";

function extractResponseText(result: any): string {
  if (result.kind === "message") {
    for (const part of result.parts) {
      if (part.kind === "text") return part.text;
    }
  }
  if (result.kind === "task") {
    const task = result as Task;
    const msg = task.status?.message;
    if (msg) {
      for (const part of msg.parts) {
        if (part.kind === "text") return part.text;
      }
    }
  }
  return JSON.stringify(result, null, 2);
}

async function sendToTravelPlanner(userText: string): Promise<string> {
  console.log("\n🤖 [PersonalAgent] Forwarding your request to TravelPlannerAgent via A2A...\n");

  const client = await A2AClient.fromCardUrl(agentCardUrl(AGENT_URLS.TRAVEL_PLANNER));
  const response = await client.sendMessage({
    message: makeUserMessage(userText),
    configuration: { blocking: true },
  });

  if ("error" in response) {
    return `Error: ${JSON.stringify(response.error)}`;
  }

  return extractResponseText(response.result);
}

async function discoverAgents(): Promise<void> {
  console.log("\n🔍 Discovering available agents via A2A Agent Cards...\n");

  const agents = [
    { name: "Expedia", url: AGENT_URLS.EXPEDIA },
    { name: "Marriott", url: AGENT_URLS.MARRIOTT },
    { name: "Restaurant", url: AGENT_URLS.RESTAURANT },
    { name: "Travel Planner", url: AGENT_URLS.TRAVEL_PLANNER },
  ];

  for (const { name, url } of agents) {
    try {
      const client = await A2AClient.fromCardUrl(agentCardUrl(url));
      const card = await client.getAgentCard();
      const skills = card.skills.map((s) => s.name).join(", ");
      console.log(`  ✅ ${card.name} — ${card.description}`);
      console.log(`     Skills: ${skills}\n`);
    } catch {
      console.log(`  ❌ ${name} (${url}) — not reachable\n`);
    }
  }
}

async function main(): Promise<void> {
  console.log("═══════════════════════════════════════════════════");
  console.log("   🧳  Personal Travel Agent  (A2A Demo)");
  console.log("═══════════════════════════════════════════════════");
  console.log("  Your personal agent that talks to other agents");
  console.log("  using the A2A protocol to plan your trip.\n");
  console.log('  Type "discover" to see available agents.');
  console.log('  Type "quit" to exit.\n');
  console.log("  Example: Plan a 5-day Italy trip visiting Rome, Florence, and Venice\n");

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  const prompt = (): void => {
    rl.question("You > ", async (input) => {
      const trimmed = input.trim();

      if (!trimmed) {
        prompt();
        return;
      }

      if (trimmed.toLowerCase() === "quit" || trimmed.toLowerCase() === "exit") {
        console.log("\n👋 Goodbye!\n");
        rl.close();
        process.exit(0);
      }

      if (trimmed.toLowerCase() === "discover") {
        await discoverAgents();
        prompt();
        return;
      }

      try {
        const result = await sendToTravelPlanner(trimmed);
        console.log("\n" + result + "\n");
      } catch (err) {
        const msg = err instanceof Error ? err.message : String(err);
        console.error(`\n❌ Error: ${msg}\n`);
        console.log("Make sure all service agents are running (npm run services).\n");
      }

      prompt();
    });
  };

  prompt();
}

main();
