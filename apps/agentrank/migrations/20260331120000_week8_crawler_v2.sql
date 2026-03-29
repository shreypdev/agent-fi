-- Week 8: registry sync cursors, sitemap per-card state, agent URL aliases (dedup)

CREATE TABLE registry_sync_state (
    registry_name TEXT PRIMARY KEY,
    last_cursor TEXT,
    last_sync_at TIMESTAMPTZ,
    total_synced BIGINT NOT NULL DEFAULT 0,
    errors INT NOT NULL DEFAULT 0,
    last_error TEXT
);

CREATE TABLE sitemap_card_state (
    card_url TEXT PRIMARY KEY,
    last_seen_sitemap_lastmod TIMESTAMPTZ,
    etag TEXT,
    last_fetched_at TIMESTAMPTZ
);

CREATE INDEX idx_sitemap_card_state_lastmod ON sitemap_card_state (last_seen_sitemap_lastmod);

CREATE TABLE agent_aliases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    agent_id UUID NOT NULL REFERENCES agents (id) ON DELETE CASCADE,
    alias_url TEXT NOT NULL,
    source TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT agent_aliases_agent_url_unique UNIQUE (agent_id, alias_url)
);

CREATE INDEX idx_agent_aliases_alias_url ON agent_aliases (alias_url);
