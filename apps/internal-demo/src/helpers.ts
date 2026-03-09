import { v4 as uuidv4 } from "uuid";
import express from "express";
import type {
  AgentCard,
  Message,
  Task,
  TaskStatusUpdateEvent,
} from "@a2a-js/sdk";
import {
  DefaultRequestHandler,
  InMemoryTaskStore,
  type AgentExecutor,
  type RequestContext,
  type ExecutionEventBus,
} from "@a2a-js/sdk/server";
import { A2AExpressApp } from "@a2a-js/sdk/server/express";

export { uuidv4 };

export function extractText(message: Message): string {
  for (const part of message.parts) {
    if (part.kind === "text") return part.text;
  }
  return "";
}

export function makeUserMessage(text: string): Message {
  return {
    kind: "message",
    messageId: uuidv4(),
    role: "user",
    parts: [{ kind: "text", text }],
  };
}

/**
 * Initializes a Task on the event bus so the ResultManager can track subsequent
 * status-update events. Required before publishing any TaskStatusUpdateEvent.
 */
export function initTask(
  eventBus: ExecutionEventBus,
  ctx: RequestContext
): void {
  const task: Task = {
    kind: "task",
    id: ctx.taskId,
    contextId: ctx.contextId,
    status: { state: "submitted", timestamp: new Date().toISOString() },
    history: [ctx.userMessage],
  };
  eventBus.publish(task);
}

/**
 * Publish a completed response. For simple request/response flows, publishes a
 * Message directly (no prior initTask needed). For flows that previously called
 * initTask, publishes a final TaskStatusUpdateEvent instead.
 */
export function publishCompleted(
  eventBus: ExecutionEventBus,
  ctx: RequestContext,
  responseText: string,
  asTask = false
): void {
  if (asTask) {
    const update: TaskStatusUpdateEvent = {
      kind: "status-update",
      taskId: ctx.taskId,
      contextId: ctx.contextId,
      final: true,
      status: {
        state: "completed",
        timestamp: new Date().toISOString(),
        message: {
          kind: "message",
          messageId: uuidv4(),
          role: "agent",
          parts: [{ kind: "text", text: responseText }],
          taskId: ctx.taskId,
          contextId: ctx.contextId,
        },
      },
    };
    eventBus.publish(update);
  } else {
    const msg: Message = {
      kind: "message",
      messageId: uuidv4(),
      role: "agent",
      parts: [{ kind: "text", text: responseText }],
      taskId: ctx.taskId,
      contextId: ctx.contextId,
    };
    eventBus.publish(msg);
  }
}

export function publishFailed(
  eventBus: ExecutionEventBus,
  ctx: RequestContext,
  errorText: string,
  asTask = false
): void {
  if (asTask) {
    const update: TaskStatusUpdateEvent = {
      kind: "status-update",
      taskId: ctx.taskId,
      contextId: ctx.contextId,
      final: true,
      status: {
        state: "failed",
        timestamp: new Date().toISOString(),
        message: {
          kind: "message",
          messageId: uuidv4(),
          role: "agent",
          parts: [{ kind: "text", text: errorText }],
          taskId: ctx.taskId,
          contextId: ctx.contextId,
        },
      },
    };
    eventBus.publish(update);
  } else {
    const msg: Message = {
      kind: "message",
      messageId: uuidv4(),
      role: "agent",
      parts: [{ kind: "text", text: `Error: ${errorText}` }],
      taskId: ctx.taskId,
      contextId: ctx.contextId,
    };
    eventBus.publish(msg);
  }
}

export function startAgentServer(
  agentCard: AgentCard,
  executor: AgentExecutor,
  port: number
): void {
  const taskStore = new InMemoryTaskStore();
  const handler = new DefaultRequestHandler(agentCard, taskStore, executor);
  const app = new A2AExpressApp(handler).setupRoutes(express());

  app.listen(port, () => {
    console.log(`[${agentCard.name}] listening on http://localhost:${port}`);
  });
}

export type { AgentExecutor, RequestContext, ExecutionEventBus };
export type { AgentCard, Message, Task, TaskStatusUpdateEvent };
