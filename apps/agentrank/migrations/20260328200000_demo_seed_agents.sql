-- Idempotent demo catalog rows for prod/staging smoke (ON CONFLICT DO NOTHING).
INSERT INTO agents (
    id,
    external_id,
    source_url,
    canonical_url,
    card_json,
    content_hash,
    name,
    description,
    endpoint_url,
    protocol_version
)
VALUES
    (
        'a0000001-0001-4001-8001-000000000001',
        'demo:pronox-landing',
        'https://pronox.dev/demo/card',
        'https://pronox.dev/demo/card',
        '{"name":"Pronox Demo Agent","description":"Example agent card for the public catalog demo.","version":"1","url":"https://pronox-public-agent.up.railway.app","skills":[{"name":"Echo","tags":["demo","a2a"],"description":"Demonstration skill for discovery UI."}]}'::jsonb,
        'demohash000000000000000000000000000000000000000000000000000000000000',
        'Pronox Demo Agent',
        'Example agent card for the public catalog demo.',
        'https://pronox-public-agent.up.railway.app',
        '1'
    ),
    (
        'a0000002-0002-4002-8002-000000000002',
        'demo:acme-analytics',
        'https://agents.acmecorp.com/card',
        'https://agents.acmecorp.com/card',
        '{"name":"Acme Analytics Agent","description":"Portfolio and risk analytics for demo search.","version":"1","url":"https://agents.acmecorp.com/financial-analysis","skills":[{"name":"Portfolio analysis","tags":["finance","portfolio"],"description":"Analyzes investment mix and risk."}]}'::jsonb,
        'demohash000000000000000000000000000000000000000000000000000000000001',
        'Acme Analytics Agent',
        'Portfolio and risk analytics for demo search.',
        'https://agents.acmecorp.com/financial-analysis',
        '1'
    )
ON CONFLICT (external_id) DO NOTHING;
