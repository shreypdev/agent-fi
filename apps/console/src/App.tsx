import { useState } from "react";
import { Link, Navigate, Route, Routes, useParams } from "react-router-dom";
import { apiBase, consoleFetch, getApiKey, setApiKey } from "./api";

function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div>
      <header
        style={{
          borderBottom: "1px solid #2f3b47",
          padding: "0.75rem 1.5rem",
          display: "flex",
          gap: "1rem",
          alignItems: "center",
        }}
      >
        <strong>Agent Search Console</strong>
        <nav style={{ display: "flex", gap: "1rem" }}>
          <Link to="/">Setup</Link>
          <Link to="/claims">Domain claims</Link>
          <Link to="/history">Crawl history</Link>
        </nav>
      </header>
      <main>{children}</main>
    </div>
  );
}

function SetupPage() {
  const [key, setKey] = useState(getApiKey());
  const base = apiBase();
  return (
    <Layout>
      <h1>Setup</h1>
      <div className="card">
        <p>
          API base (from <code>VITE_CONSOLE_API_BASE</code> at build):{" "}
          <strong>{base || "(not set)"}</strong>
        </p>
        <label>
          <div>Console API key</div>
          <input
            type="password"
            value={key}
            onChange={(e) => setKey(e.target.value)}
            style={{ width: "100%", maxWidth: 420, marginTop: 6 }}
            placeholder="Same value as CONSOLE_API_KEY on consoled"
          />
        </label>
        <p>
          <button type="button" onClick={() => setApiKey(key)}>
            Save to session
          </button>
        </p>
      </div>
    </Layout>
  );
}

function ClaimsPage() {
  const [rows, setRows] = useState<unknown[] | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const [domain, setDomain] = useState("");
  const load = async () => {
    setErr(null);
    try {
      const r = await consoleFetch("/v1/console/domain-claims");
      if (!r.ok) throw new Error(`${r.status}: ${await r.text()}`);
      setRows(await r.json());
    } catch (e) {
      setErr(String(e));
    }
  };
  const submit = async () => {
    setErr(null);
    try {
      const r = await consoleFetch("/v1/console/domain-claims", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ domain }),
      });
      if (!r.ok) throw new Error(`${r.status}: ${await r.text()}`);
      setDomain("");
      await load();
    } catch (e) {
      setErr(String(e));
    }
  };
  return (
    <Layout>
      <h1>Domain claims</h1>
      <div className="card">
        <p>Stub flow: create pending claim; verify manually in DB for v0.1.</p>
        <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
          <input
            value={domain}
            onChange={(e) => setDomain(e.target.value)}
            placeholder="example.com"
          />
          <button type="button" onClick={submit}>
            Create / refresh claim
          </button>
          <button type="button" onClick={load}>
            List claims
          </button>
        </div>
        {err && <p className="err">{err}</p>}
        {rows && <pre>{JSON.stringify(rows, null, 2)}</pre>}
      </div>
    </Layout>
  );
}

function HistoryPage() {
  const [domain, setDomain] = useState("");
  const [rows, setRows] = useState<unknown[] | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const load = async () => {
    setErr(null);
    try {
      const q = new URLSearchParams({ domain, limit: "50" });
      const r = await consoleFetch(`/v1/console/crawl-history?${q}`);
      if (!r.ok) throw new Error(`${r.status}: ${await r.text()}`);
      setRows(await r.json());
    } catch (e) {
      setErr(String(e));
    }
  };
  return (
    <Layout>
      <h1>Crawl history</h1>
      <div className="card">
        <div style={{ display: "flex", gap: 8 }}>
          <input
            value={domain}
            onChange={(e) => setDomain(e.target.value)}
            placeholder="primary domain"
          />
          <button type="button" onClick={load}>
            Load
          </button>
        </div>
        {err && <p className="err">{err}</p>}
        {rows && <pre>{JSON.stringify(rows, null, 2)}</pre>}
      </div>
      <p>
        Agent inspector: <code>/agents/&lt;uuid&gt;</code>
      </p>
    </Layout>
  );
}

function AgentPage() {
  const { id } = useParams<{ id: string }>();
  const [data, setData] = useState<object | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const load = async () => {
    if (!id) return;
    setErr(null);
    try {
      const r = await consoleFetch(`/v1/console/agents/${id}`);
      if (!r.ok) throw new Error(`${r.status}: ${await r.text()}`);
      setData((await r.json()) as object);
    } catch (e) {
      setErr(String(e));
    }
  };
  return (
    <Layout>
      <h1>Agent card</h1>
      <div className="card">
        <button type="button" onClick={load}>
          Load {id}
        </button>
        {err && <p className="err">{err}</p>}
        {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
      </div>
    </Layout>
  );
}

export default function App() {
  if (!apiBase()) {
    return (
      <main>
        <h1>Console</h1>
        <p className="err">
          Set <code>VITE_CONSOLE_API_BASE</code> when building (e.g. https://consoled.example.com).
        </p>
      </main>
    );
  }
  return (
    <Routes>
      <Route path="/" element={<SetupPage />} />
      <Route path="/claims" element={<ClaimsPage />} />
      <Route path="/history" element={<HistoryPage />} />
      <Route path="/agents/:id" element={<AgentPage />} />
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}
