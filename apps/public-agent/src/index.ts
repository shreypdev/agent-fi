import { v4 as uuidv4 } from "uuid";
import cors from "cors";
import express, { type Request, type Response } from "express";
import type { AgentCard, Message } from "@a2a-js/sdk";
import {
  DefaultRequestHandler,
  InMemoryTaskStore,
  type AgentExecutor,
  type RequestContext,
  type ExecutionEventBus,
} from "@a2a-js/sdk/server";
import { A2AExpressApp } from "@a2a-js/sdk/server/express";
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp";
import { StreamableHTTPServerTransport } from "@modelcontextprotocol/sdk/server/streamableHttp";
import { z } from "zod";

const PORT = Number(process.env.PORT) || 3010;
const BASE_URL = process.env.PUBLIC_AGENT_URL ?? `http://localhost:${PORT}`;

function extractText(message: Message): string {
  for (const part of message.parts) {
    if (part.kind === "text") return part.text;
  }
  return "";
}

/** Echo response text. Used by both A2A and REST. */
export function echoResponse(userMessage: string): string {
  return `Hello agent! I received your message: ${userMessage}`;
}

class EchoExecutor implements AgentExecutor {
  async execute(ctx: RequestContext, eventBus: ExecutionEventBus): Promise<void> {
    const text = extractText(ctx.userMessage);
    const response = echoResponse(text);
    const msg: Message = {
      kind: "message",
      messageId: uuidv4(),
      role: "agent",
      parts: [{ kind: "text", text: response }],
      taskId: ctx.taskId,
      contextId: ctx.contextId,
    };
    eventBus.publish(msg);
  }

  async cancelTask(_taskId: string, eventBus: ExecutionEventBus): Promise<void> {
    eventBus.finished();
  }
}

export const publicAgentCard: AgentCard = {
  name: "AgentFi Playground Agent",
  description: "Public demo agent for AgentFi. Echoes your message.",
  url: BASE_URL,
  version: "1.0.0",
  protocolVersion: "0.2.1",
  capabilities: { streaming: false, pushNotifications: false },
  defaultInputModes: ["text/plain"],
  defaultOutputModes: ["text/plain"],
  skills: [
    {
      id: "echo",
      name: "Echo",
      description: "Echo back the user's message",
      tags: ["demo", "echo"],
      examples: ["Hello", "Say hi"],
    },
  ],
};

function getOpenApiSpec(serverUrl: string): object {
  return {
    openapi: "3.0.0",
    info: {
      title: "AgentFi Public Agent API",
      description: "REST bridge to the AgentFi playground echo agent.",
      version: "1.0.0",
    },
    servers: [{ url: serverUrl }],
    paths: {
      "/api/chat": {
        get: {
          summary: "Send a message (query)",
          operationId: "chatGet",
          parameters: [
            {
              name: "message",
              in: "query",
              required: true,
              schema: { type: "string" },
              description: "Message to send to the agent",
            },
          ],
          responses: {
            "200": {
              description: "Agent response (plain text)",
              content: { "text/plain": { schema: { type: "string" } } },
            },
          },
        },
        post: {
          summary: "Send a message (body)",
          operationId: "chatPost",
          requestBody: {
            content: {
              "application/json": {
                schema: {
                  type: "object",
                  properties: { message: { type: "string" } },
                  required: ["message"],
                },
              },
              "application/x-www-form-urlencoded": {
                schema: {
                  type: "object",
                  properties: { message: { type: "string" } },
                  required: ["message"],
                },
              },
            },
          },
          responses: {
            "200": {
              description: "Agent response (plain text)",
              content: { "text/plain": { schema: { type: "string" } } },
            },
          },
        },
      },
    },
  };
}

function startServer(): void {
  const taskStore = new InMemoryTaskStore();
  const handler = new DefaultRequestHandler(
    publicAgentCard,
    taskStore,
    new EchoExecutor()
  );
  const app = express();
  app.use(express.json());
  app.use(express.urlencoded({ extended: true }));
  // CORS so "Try in Browser" on the landing page can call /api/chat from another origin
  app.use(cors({ origin: true, methods: ["GET", "POST", "OPTIONS"], allowedHeaders: ["Content-Type"] }));
  new A2AExpressApp(handler).setupRoutes(app);

  // REST bridge: GET/POST /api/chat
  app.get("/api/chat", (req: Request, res: Response) => {
    const message = (req.query.message as string) ?? "";
    res.type("text/plain").send(echoResponse(message));
  });

  app.post("/api/chat", (req: Request, res: Response) => {
    const message =
      typeof req.body?.message === "string"
        ? req.body.message
        : typeof req.query?.message === "string"
          ? req.query.message
          : "";
    res.type("text/plain").send(echoResponse(message));
  });

  // OpenAPI spec for ChatGPT Action editor
  app.get("/openapi.json", (req: Request, res: Response) => {
    const baseUrl = BASE_URL || `${req.protocol}://${req.get("host") || `localhost:${PORT}`}`;
    res.json(getOpenApiSpec(baseUrl));
  });

  // MCP Streamable HTTP endpoint (for Cursor, Claude, etc.)
  app.post("/mcp", async (req: Request, res: Response) => {
    const server = new McpServer(
      {
        name: publicAgentCard.name,
        version: publicAgentCard.version,
      },
      { capabilities: { tools: {} } }
    );
    server.registerTool(
      "echo",
      {
        description: "Send a message to the AgentFi playground agent and get an echo response.",
        inputSchema: {
          message: z.string().describe("Message to send to the agent"),
        },
      },
      async ({ message }: { message?: string }) => {
        const response = echoResponse(message ?? "");
        return {
          content: [{ type: "text" as const, text: response }],
        };
      }
    );
    const transport = new StreamableHTTPServerTransport({
      sessionIdGenerator: undefined,
    });
    try {
      await server.connect(transport);
      await transport.handleRequest(req, res, req.body);
      res.on("close", () => {
        transport.close();
        server.close();
      });
    } catch (err) {
      console.error("[MCP] Error handling request:", err);
      if (!res.headersSent) {
        res.status(500).json({
          jsonrpc: "2.0",
          error: { code: -32603, message: "Internal server error" },
          id: null,
        });
      }
    }
  });
  app.get("/mcp", (_req: Request, res: Response) => {
    res.writeHead(405, { "Content-Type": "application/json" }).end(
      JSON.stringify({
        jsonrpc: "2.0",
        error: { code: -32000, message: "Method not allowed. Use POST." },
        id: null,
      })
    );
  });

  // MCP discovery manifest (for "Add to Cursor" / "Add to Claude" – clients probe this URL)
  app.get("/.well-known/mcp.json", (req: Request, res: Response) => {
    const resolvedBase =
      BASE_URL ||
      `${req.protocol}://${req.get("host") || `localhost:${PORT}`}`;
    const baseUrl = resolvedBase.replace(/\/$/, "");
    res
      .set({
        "Content-Type": "application/json",
        "X-Content-Type-Options": "nosniff",
        "Cache-Control": "public, max-age=3600",
        "Access-Control-Allow-Origin": "*",
      })
      .json({
        version: "1.0",
        protocolVersion: "2025-06-18",
        serverInfo: {
          name: publicAgentCard.name,
          version: publicAgentCard.version,
          description: publicAgentCard.description,
          homepage: baseUrl,
        },
        transport: {
          type: "streamable-http",
          url: `${baseUrl}/mcp`,
        },
        capabilities: {
          tools: true,
          resources: false,
          prompts: false,
        },
      });
  });

  app.listen(PORT, () => {
    console.log(`[AgentFi Public Agent] listening on http://localhost:${PORT}`);
    console.log(`  Agent Card: ${BASE_URL}/.well-known/agent-card.json`);
    console.log(`  MCP:       ${BASE_URL}/mcp (tool: echo)`);
    console.log(`  MCP manifest: ${BASE_URL}/.well-known/mcp.json`);
    console.log(`  REST chat:  ${BASE_URL}/api/chat?message=Hello`);
    console.log(`  OpenAPI:   ${BASE_URL}/openapi.json`);
  });
}

startServer();
