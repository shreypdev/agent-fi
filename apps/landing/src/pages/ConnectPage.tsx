import { useState, useCallback, useMemo } from "react";
import { motion } from "framer-motion";
import { PUBLIC_AGENT_URL } from "../config";
import {
  isSearchApiConfigured,
  searchApiBase,
  searchMcpManifestUrl,
  searchMcpUrl,
} from "../lib/searchApi";
import TryInBrowserWidget from "../components/TryInBrowserWidget";
import ConnectCard from "../components/ConnectCard";
import "../App.css";
import "./ConnectPage.css";

type TermTokenType = "command" | "flag" | "string" | "url" | "default";

function tokenizeTerminal(s: string): { type: TermTokenType; value: string }[] {
  const tokens: { type: TermTokenType; value: string }[] = [];
  let i = 0;
  const len = s.length;
  const rest = () => s.slice(i);

  while (i < len) {
    const r = rest();
    if (r.match(/^curl\b/)) {
      tokens.push({ type: "command", value: "curl" });
      i += 4;
      continue;
    }
    const flagMatch = r.match(/^(-[\w-]+)/);
    if (flagMatch) {
      tokens.push({ type: "flag", value: flagMatch[1] });
      i += flagMatch[1].length;
      continue;
    }
    const dqMatch = r.match(/^"(?:[^"\\]|\\.)*"/);
    if (dqMatch) {
      tokens.push({ type: "string", value: dqMatch[0] });
      i += dqMatch[0].length;
      continue;
    }
    if (r.startsWith("'")) {
      const afterOpen = r.slice(1);
      const lastQuote = afterOpen.lastIndexOf("'");
      const value =
        lastQuote === -1 ? r : r.slice(0, lastQuote + 2);
      tokens.push({ type: "string", value });
      i += value.length;
      continue;
    }
    const urlMatch = r.match(/^https?:\/\/[^\s'"]+/);
    if (urlMatch) {
      tokens.push({ type: "url", value: urlMatch[0] });
      i += urlMatch[0].length;
      continue;
    }
    const defMatch = r.match(/^\s+|^\\\s*|^[^\s'"-][^\s'"]*|^./);
    const val = defMatch ? defMatch[0] : r[0] ?? "";
    tokens.push({ type: "default", value: val });
    i += val.length || 1;
  }
  return tokens;
}

function HighlightedTerminalCode({ snippet }: { snippet: string }) {
  const tokens = useMemo(() => tokenizeTerminal(snippet), [snippet]);
  return (
    <>
      {tokens.map((t, idx) => (
        <span key={idx} className={`term-${t.type}`}>
          {t.value}
        </span>
      ))}
    </>
  );
}

const mcpManifestUrl = `${PUBLIC_AGENT_URL}/.well-known/mcp.json`;

/** Cursor MCP install deep link so "Add to Cursor" opens Cursor and prompts to add the server. */
function buildCursorMcpInstallLink(): string {
  const name = "Pronox";
  const config = { url: `${PUBLIC_AGENT_URL}/mcp` };
  const configB64 = typeof btoa !== "undefined" ? btoa(JSON.stringify(config)) : "";
  return `cursor://anysphere.cursor-deeplink/mcp/install?name=${encodeURIComponent(name)}&config=${encodeURIComponent(configB64)}`;
}

const cursorInstallHref = buildCursorMcpInstallLink();

/** Cursor deeplink for AgentRank searchd MCP (uses `VITE_SEARCH_API_BASE_URL`). */
function buildAgentRankCursorMcpInstallLink(): string {
  const base = searchApiBase();
  if (!base) return "#";
  const name = "AgentRank Search";
  const config = { url: `${base}/mcp` };
  const configB64 =
    typeof btoa !== "undefined" ? btoa(JSON.stringify(config)) : "";
  return `cursor://anysphere.cursor-deeplink/mcp/install?name=${encodeURIComponent(name)}&config=${encodeURIComponent(configB64)}`;
}

type PlatformCard = {
  id: string;
  title: string;
  description: string;
  href: string;
  external: boolean;
  copy: string | null;
  logo: string;
  logoBg?: "white" | "transparent";
};

const platformCards: PlatformCard[] = [
  {
    id: "cursor",
    title: "Add to Cursor",
    description: "Use the Pronox agent from Cursor via MCP.",
    href: cursorInstallHref,
    external: true,
    copy: "Opens Cursor to add this MCP server in one click.",
    logo: "https://images.seeklogo.com/logo-png/61/1/cursor-logo-png_seeklogo-611587.png",
  },
  {
    id: "claude",
    title: "Add to Claude",
    description: "Install the MCP server in Claude Desktop.",
    href: mcpManifestUrl,
    external: true,
    copy: "Click to copy config — paste in Claude Desktop: Settings → Developer → Edit Config.",
    logo: "https://upload.wikimedia.org/wikipedia/commons/b/b0/Claude_AI_symbol.svg",
  },
  {
    id: "chatgpt",
    title: "Open in ChatGPT",
    description: "Use our Custom GPT with the Pronox agent.",
    href: "https://chat.openai.com/g/g-agentfi",
    external: true,
    copy: null,
    logo: "https://upload.wikimedia.org/wikipedia/commons/thumb/e/ef/ChatGPT-Logo.svg/960px-ChatGPT-Logo.svg.png",
    logoBg: "white",
  },
];

const curlSnippet = `curl "${PUBLIC_AGENT_URL}/api/chat?message=Hello"`;

const a2aSnippet = `curl -X POST ${PUBLIC_AGENT_URL} \\
  -H "Content-Type: application/json" \\
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "message/send",
    "params": {
      "message": {
        "kind": "message",
        "messageId": "msg-1",
        "role": "user",
        "parts": [{ "kind": "text", "text": "Hello" }]
      }
    }
  }'`;

export default function ConnectPage() {
  const [copiedId, setCopiedId] = useState<string | null>(null);

  const copyToClipboard = useCallback(async (text: string, id: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedId(id);
      setTimeout(() => setCopiedId(null), 2000);
    } catch {
      // ignore
    }
  }, []);

  /** Claude Desktop has no install URL scheme; copy config snippet so user can paste in Edit Config. */
  const handleAddToClaude = useCallback(async () => {
    const snippet = `"pronox": {
  "url": "${PUBLIC_AGENT_URL}/mcp"
}`;
    await copyToClipboard(snippet, "claude");
  }, [copyToClipboard]);

  return (
    <motion.main
      className="page connect-page"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.4, ease: [0.16, 1, 0.3, 1] }}
    >
      <section className="section connect-hero">
        <motion.h1
          className="connect-title"
          initial={{ opacity: 0, y: 24 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.1, ease: [0.16, 1, 0.3, 1] }}
        >
          Connect to the Pronox agent in one click
        </motion.h1>
        <motion.p
          className="connect-sub"
          initial={{ opacity: 0, y: 16 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.1, ease: [0.16, 1, 0.3, 1] }}
        >
          Pick your platform and start chatting. No SDK required.
        </motion.p>
      </section>

      {isSearchApiConfigured() && (
        <section className="section connect-hero" style={{ paddingTop: 0 }}>
          <h2 className="section-heading">Add AgentRank search to your AI</h2>
          <p className="connect-sub">
            MCP Streamable HTTP: <code className="search-code">{searchMcpUrl()}</code>
            {" · "}
            Discovery: <code className="search-code">{searchMcpManifestUrl()}</code>
          </p>
          <p className="connect-sub">
            <a
              href={buildAgentRankCursorMcpInstallLink()}
              target="_blank"
              rel="noreferrer"
            >
              Add AgentRank search to Cursor
            </a>
          </p>
        </section>
      )}

      <section className="section connect-cards-wrap">
        <div className="connect-cards">
          {platformCards.map((card, i) => (
            <motion.div
              key={card.id}
              initial={{ opacity: 0, y: 16 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.4, delay: 0.1 + i * 0.06 }}
            >
              <ConnectCard
                title={card.title}
                description={card.description}
                href={card.id === "claude" ? undefined : card.href}
                external={card.external}
                copyText={card.id === "claude" && copiedId === "claude" ? "Copied! Paste in Settings → Developer → Edit Config." : card.copy}
                logo={card.logo}
                logoBg={card.logoBg}
                onClick={card.id === "claude" ? handleAddToClaude : undefined}
              />
            </motion.div>
          ))}
          <motion.div
            className="connect-widget-card"
            initial={{ opacity: 0, y: 16 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.4, delay: 0.28 }}
          >
            <TryInBrowserWidget />
          </motion.div>
        </div>
      </section>

      <section className="section connect-snippets">
        <h2 className="section-heading">Or use the API</h2>
        <div className="terminal-window">
          <div className="terminal-titlebar">
            <div className="terminal-dots" aria-hidden>
              <span className="terminal-dot terminal-dot-red" />
              <span className="terminal-dot terminal-dot-yellow" />
              <span className="terminal-dot terminal-dot-green" />
            </div>
            <span className="terminal-title">REST (curl)</span>
            <button
              type="button"
              className="terminal-copy"
              onClick={() => copyToClipboard(curlSnippet, "curl")}
              aria-label="Copy curl command"
            >
              {copiedId === "curl" ? "Copied" : "Copy"}
            </button>
          </div>
          <pre className="terminal-code"><span className="terminal-prompt">$ </span><HighlightedTerminalCode snippet={curlSnippet} /></pre>
        </div>
        <div className="terminal-window">
          <div className="terminal-titlebar">
            <div className="terminal-dots" aria-hidden>
              <span className="terminal-dot terminal-dot-red" />
              <span className="terminal-dot terminal-dot-yellow" />
              <span className="terminal-dot terminal-dot-green" />
            </div>
            <span className="terminal-title">A2A (JSON-RPC)</span>
            <button
              type="button"
              className="terminal-copy"
              onClick={() => copyToClipboard(a2aSnippet, "a2a")}
              aria-label="Copy A2A curl command"
            >
              {copiedId === "a2a" ? "Copied" : "Copy"}
            </button>
          </div>
          <pre className="terminal-code"><span className="terminal-prompt">$ </span><HighlightedTerminalCode snippet={a2aSnippet} /></pre>
        </div>
      </section>
    </motion.main>
  );
}
