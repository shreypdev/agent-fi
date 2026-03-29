-- Week 6: recrawl / frontier URL state + index job retry queue

CREATE TABLE frontier_url_state (
    url TEXT PRIMARY KEY,
    last_content_hash TEXT,
    last_ingest_at TIMESTAMPTZ,
    next_fetch_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    change_streak INT NOT NULL DEFAULT 0,
    stable_streak INT NOT NULL DEFAULT 0
);

CREATE INDEX idx_frontier_url_state_next_fetch ON frontier_url_state (next_fetch_at);

CREATE TABLE index_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id UUID NOT NULL UNIQUE REFERENCES agents (id) ON DELETE CASCADE,
    attempts INT NOT NULL DEFAULT 0,
    next_run_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_index_jobs_next_run ON index_jobs (next_run_at);
CREATE INDEX idx_index_jobs_agent ON index_jobs (agent_id);
