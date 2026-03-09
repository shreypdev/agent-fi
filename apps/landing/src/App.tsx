import "./App.css";

const CONNECT_URL = import.meta.env.VITE_CONNECT_URL ?? "/connect";
const PUBLIC_AGENT_URL = import.meta.env.VITE_PUBLIC_AGENT_URL ?? "";

export default function App() {
  return (
    <main className="landing">
      <h1>The governance layer for the agent economy</h1>
      <p className="teaser">
        Observability, identity, and trust for multi-agent systems. One-click
        connect from ChatGPT, Claude, or Cursor.
      </p>
      <div className="links">
        <a href={CONNECT_URL} className="btn">
          Connect your agent
        </a>
        {PUBLIC_AGENT_URL && (
          <a href={PUBLIC_AGENT_URL} className="btn">
            Try the public agent
          </a>
        )}
      </div>
      <section className="waitlist" aria-label="Email waitlist">
        <p style={{ margin: "0 0 0.75rem", color: "#64748b" }}>
          Get notified when we launch.
        </p>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            // Placeholder: no backend yet
          }}
        >
          <input
            type="email"
            placeholder="you@example.com"
            aria-label="Email address"
          />
          <button type="submit">Notify me</button>
        </form>
      </section>
    </main>
  );
}
