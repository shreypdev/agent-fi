import { useState, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { PUBLIC_AGENT_URL } from "../config";
import "./TryInBrowserWidget.css";

export default function TryInBrowserWidget() {
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState<{ role: "user" | "agent"; text: string }[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const bottomRef = useRef<HTMLDivElement>(null);

  const send = async () => {
    const text = input.trim();
    if (!text || loading) return;

    setInput("");
    setMessages((m) => [...m, { role: "user", text }]);
    setLoading(true);
    setError(null);

    try {
      const url = `${PUBLIC_AGENT_URL}/api/chat?message=${encodeURIComponent(text)}`;
      const res = await fetch(url, { method: "GET" });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const reply = await res.text();
      setMessages((m) => [...m, { role: "agent", text: reply }]);
      bottomRef.current?.scrollIntoView({ behavior: "smooth" });
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
      {isDev && (
        <p className="try-widget-hint" role="status">
          Using agent at <code>{PUBLIC_AGENT_URL}</code>. Start it with <code>pnpm run public:agent</code> if needed.
        </p>
      )}
      <div className="try-widget-input-wrap">
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
      <div className="try-widget-messages" role="log" aria-live="polite">
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
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
