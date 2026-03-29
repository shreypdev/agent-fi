import { FormEvent, useState } from "react";
import { Link } from "react-router-dom";
import {
  isSearchApiConfigured,
  postSearch,
  type SearchResultItem,
} from "../lib/searchApi";
import "./SearchPage.css";

export default function SearchPage() {
  const [q, setQ] = useState("");
  const [loading, setLoading] = useState(false);
  const [err, setErr] = useState<string | null>(null);
  const [results, setResults] = useState<SearchResultItem[] | null>(null);
  const [meta, setMeta] = useState<{ ms: number } | null>(null);

  const configured = isSearchApiConfigured();

  async function onSubmit(e: FormEvent) {
    e.preventDefault();
    if (!configured) return;
    setErr(null);
    setLoading(true);
    setResults(null);
    setMeta(null);
    try {
      const data = await postSearch(q.trim(), 20, 0);
      setResults(data.results);
      setMeta({ ms: data.query_time_ms });
    } catch (er) {
      setErr(er instanceof Error ? er.message : "Search failed");
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="page search-page">
      <section className="section">
        <h1 className="section-heading">Discover agents</h1>
        <p className="section-sub">
          Keyword search over the AgentRank catalog. Machines should call{" "}
          <code className="search-code">POST /v1/search</code> on the same API.
        </p>

        {!configured && (
          <div className="search-banner" role="status">
            Search API URL is not configured. Set{" "}
            <code className="search-code">VITE_SEARCH_API_BASE_URL</code> at build time
            (e.g. your <code className="search-code">searchd</code> HTTPS URL).
          </div>
        )}

        <form className="search-form" onSubmit={onSubmit}>
          <label className="visually-hidden" htmlFor="search-q">
            Search query
          </label>
          <input
            id="search-q"
            className="search-input"
            type="search"
            placeholder="Try portfolio, finance, demo…"
            value={q}
            onChange={(e) => setQ(e.target.value)}
            disabled={!configured || loading}
            autoComplete="off"
          />
          <button
            type="submit"
            className="btn-primary search-submit"
            disabled={!configured || loading || !q.trim()}
          >
            {loading ? "Searching…" : "Search"}
          </button>
        </form>

        {err && (
          <p className="search-error" role="alert">
            {err}
          </p>
        )}

        {meta && (
          <p className="search-meta">
            {results?.length ?? 0} results · {meta.ms} ms
          </p>
        )}

        {results && results.length === 0 && !loading && (
          <p className="search-empty">No matches. Try another keyword.</p>
        )}

        {results && results.length > 0 && (
          <ul className="search-results">
            {results.map((r) => (
              <li key={r.agent_id} className="search-card">
                <Link to={`/agents/${r.agent_id}`} className="search-card-title">
                  {r.name}
                </Link>
                <p className="search-card-tier">{r.trust_tier}</p>
                {r.snippet && <p className="search-card-snippet">{r.snippet}</p>}
                <p className="search-card-desc">{r.description}</p>
                <a
                  href={r.endpoint_url}
                  className="search-card-link"
                  target="_blank"
                  rel="noreferrer"
                >
                  Endpoint
                </a>
              </li>
            ))}
          </ul>
        )}
      </section>
    </main>
  );
}
