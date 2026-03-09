import { useState, useRef, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { PUBLIC_AGENT_URL } from "../config";
import "./TryInBrowserWidget.css";

type TransportMode = "rest" | "a2a";

function extractA2AResponseText(result: unknown): string {
  if (!result || typeof result !== "object") return String(result);
  const r = result as Record<string, unknown>;
  if (r.kind === "message" && Array.isArray(r.parts)) {
    for (const part of r.parts as { kind?: string; text?: string }[]) {
      if (part?.kind === "text" && typeof part.text === "string") return part.text;
    }
  }
  if (r.kind === "task" && r.status && typeof r.status === "object") {
    const msg = (r.status as Record<string, unknown>).message;
    if (msg && typeof msg === "object" && Array.isArray((msg as Record<string, unknown>).parts)) {
      for (const part of (msg as { parts: { kind?: string; text?: string }[] }).parts) {
        if (part?.kind === "text" && typeof part.text === "string") return part.text;
      }
    }
  }
  return JSON.stringify(result, null, 2);
}

export default function TryInBrowserWidget() {
  const [mode, setMode] = useState<TransportMode>("rest");
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState<{ role: "user" | "agent"; text: string }[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const messagesContainerRef = useRef<HTMLDivElement>(null);

  const scrollMessagesToBottom = () => {
    const el = messagesContainerRef.current;
    if (el) el.scrollTo({ top: el.scrollHeight, behavior: "smooth" });
  };

  useEffect(() => {
    const t = setTimeout(scrollMessagesToBottom, 50);
    return () => clearTimeout(t);
  }, [messages]);

  const send = async () => {
    const text = input.trim();
    if (!text || loading) return;

    setInput("");
    setMessages((m) => [...m, { role: "user", text }]);
    setLoading(true);
    setError(null);

    try {
      if (mode === "rest") {
        const url = `${PUBLIC_AGENT_URL}/api/chat?message=${encodeURIComponent(text)}`;
        const res = await fetch(url, { method: "GET" });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const reply = await res.text();
        setMessages((m) => [...m, { role: "agent", text: reply }]);
      } else {
        const messageId = `msg-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
        const body = {
          jsonrpc: "2.0",
          id: 1,
          method: "message/send",
          params: {
            message: {
              kind: "message",
              messageId,
              role: "user",
              parts: [{ kind: "text" as const, text }],
            },
          },
        };
        const res = await fetch(PUBLIC_AGENT_URL, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(body),
        });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const data = (await res.json()) as { result?: unknown; error?: { message?: string } };
        if (data.error) throw new Error(data.error.message ?? JSON.stringify(data.error));
        const reply = extractA2AResponseText(data.result);
        setMessages((m) => [...m, { role: "agent", text: reply }]);
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : "Request failed";
      const isNetwork =
        msg === "Failed to fetch" ||
        msg.toLowerCase().includes("network") ||
        msg.toLowerCase().includes("cors");
      setError(
        isNetwork
          ? `Can't reach the agent at ${PUBLIC_AGENT_URL}. Start it locally with "pnpm run public:agent" or ensure the deployed agent is up and has CORS enabled.`
          : msg
      );
    } finally {
      setLoading(false);
    }
  };

  const isDev = import.meta.env.DEV;

  return (
    <div className="try-widget">
      <h3 className="connect-card-title">Try in Browser</h3>
      <p className="connect-card-desc">Send a message and see the agent respond below.</p>
      <div className="try-widget-mode-wrap" role="tablist" aria-label="Transport mode">
        <button
          type="button"
          role="tab"
          aria-selected={mode === "rest"}
          aria-controls="try-widget-panel"
          id="try-widget-tab-rest"
          className={`try-widget-mode-tab ${mode === "rest" ? "try-widget-mode-tab-active" : ""}`}
          onClick={() => setMode("rest")}
        >
          REST
        </button>
        <button
          type="button"
          role="tab"
          aria-selected={mode === "a2a"}
          aria-controls="try-widget-panel"
          id="try-widget-tab-a2a"
          className={`try-widget-mode-tab ${mode === "a2a" ? "try-widget-mode-tab-active" : ""}`}
          onClick={() => setMode("a2a")}
        >
          A2A
        </button>
      </div>
      <p className="try-widget-mode-desc" id="try-widget-mode-desc">
        {mode === "rest"
          ? "Uses GET /api/chat?message=... (REST bridge)."
          : "Uses POST with JSON-RPC message/send (Agent-to-Agent protocol)."}
      </p>
      {isDev && (
        <p className="try-widget-hint" role="status">
          Using agent at <code>{PUBLIC_AGENT_URL}</code>. Start it with <code>pnpm run public:agent</code> if needed.
        </p>
      )}
      <div id="try-widget-panel" className="try-widget-input-wrap" role="tabpanel" aria-describedby="try-widget-mode-desc">
        <input
          type="text"
          className="try-widget-input"
          placeholder="Type a message..."
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && send()}
          aria-label="Message to agent"
          disabled={loading}
        />
        <button
          type="button"
          className="try-widget-send btn-primary"
          onClick={send}
          disabled={loading || !input.trim()}
          aria-label="Send message"
        >
          {loading ? "…" : "Send"}
        </button>
      </div>
      <div ref={messagesContainerRef} className="try-widget-messages" role="log" aria-live="polite">
        <AnimatePresence initial={false}>
          {messages.map((msg, i) => (
            <motion.div
              key={i}
              className={`try-widget-msg try-widget-msg-${msg.role}`}
              initial={{ opacity: 0, y: 8 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.25 }}
            >
              <span className="try-widget-msg-role">{msg.role}</span>
              <span className="try-widget-msg-text">{msg.text}</span>
            </motion.div>
          ))}
        </AnimatePresence>
        {error && (
          <motion.div
            className="try-widget-msg try-widget-msg-error"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
          >
            Error: {error}
          </motion.div>
        )}
      </div>
    </div>
  );
}
