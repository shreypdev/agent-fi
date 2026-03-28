-- AgentRank schema v1 (Week 1): providers, agents, crawl_history, trust_records

CREATE TABLE providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    primary_domain TEXT NOT NULL,
    display_name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT providers_primary_domain_unique UNIQUE (primary_domain)
);

CREATE TABLE agents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider_id UUID REFERENCES providers (id) ON DELETE SET NULL,
    external_id TEXT NOT NULL,
    source_url TEXT NOT NULL,
    canonical_url TEXT NOT NULL,
    card_json JSONB NOT NULL,
    content_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    endpoint_url TEXT NOT NULL,
    protocol_version TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT agents_external_id_unique UNIQUE (external_id)
);

CREATE INDEX idx_agents_provider_id ON agents (provider_id);

CREATE INDEX idx_agents_canonical_url ON agents (canonical_url);

CREATE INDEX idx_agents_content_hash ON agents (content_hash);

CREATE TABLE crawl_history (
    id BIGSERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    agent_id UUID REFERENCES agents (id) ON DELETE SET NULL,
    http_status INT,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    error_code TEXT,
    error_detail TEXT,
    response_bytes INT
);

CREATE INDEX idx_crawl_history_url ON crawl_history (url);

CREATE INDEX idx_crawl_history_fetched_at ON crawl_history (fetched_at);

CREATE TABLE trust_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id UUID NOT NULL UNIQUE REFERENCES agents (id) ON DELETE CASCADE,
    trust_tier TEXT NOT NULL DEFAULT 'indexed',
    score_json JSONB,
    computed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Documented trust_tier values (application-enforced until CHECK is added):
-- indexed, established, verified, trusted, authoritative
