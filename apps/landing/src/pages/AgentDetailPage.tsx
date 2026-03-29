import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { getAgent, isSearchApiConfigured, type AgentDetail } from "../lib/searchApi";
import "./AgentDetailPage.css";

export default function AgentDetailPage() {
  const { id } = useParams<{ id: string }>();
  const [data, setData] = useState<AgentDetail | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const configured = isSearchApiConfigured();

  useEffect(() => {
    if (!id || !configured) {
      setLoading(false);
      return;
    }
    let cancelled = false;
    (async () => {
      try {
        const d = await getAgent(id);
        if (!cancelled) setData(d);
      } catch (e) {
        if (!cancelled) {
          setErr(e instanceof Error ? e.message : "Failed to load agent");
        }
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [id, configured]);

  if (!configured) {
    return (
      <main className="page agent-detail-page">
        <section className="section">
          <p className="agent-detail-banner">
            Set <code className="search-code">VITE_SEARCH_API_BASE_URL</code> to load agent
            details.
          </p>
          <Link to="/search">Back to search</Link>
        </section>
      </main>
    );
  }

  if (loading) {
    return (
      <main className="page agent-detail-page">
        <section className="section">
          <p className="section-sub">Loading…</p>
        </section>
      </main>
    );
  }

  if (err || !data) {
    return (
      <main className="page agent-detail-page">
        <section className="section">
          <p className="search-error" role="alert">
            {err ?? "Not found"}
          </p>
          <Link to="/search">Back to search</Link>
        </section>
      </main>
    );
  }

  const cardPretty = JSON.stringify(data.card_json, null, 2);

  return (
    <main className="page agent-detail-page">
      <section className="section agent-detail-inner">
        <Link to="/search" className="agent-back">
          ← Search
        </Link>
        <h1 className="section-heading agent-detail-title">{data.name}</h1>
        <p className="agent-detail-tier">{data.trust_tier}</p>
        <p className="agent-detail-desc">{data.description}</p>
        <dl className="agent-detail-dl">
          <dt>Endpoint</dt>
          <dd>
            <a href={data.endpoint_url} target="_blank" rel="noreferrer">
              {data.endpoint_url}
            </a>
          </dd>
          <dt>Protocol</dt>
          <dd>{data.protocol_version}</dd>
          {data.provider_display_name && (
            <>
              <dt>Provider</dt>
              <dd>{data.provider_display_name}</dd>
            </>
          )}
        </dl>
        <h2 className="agent-json-heading">Agent card (JSON)</h2>
        <pre className="agent-json-pre">{cardPretty}</pre>
      </section>
    </main>
  );
}
