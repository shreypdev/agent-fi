-- Agent Search Console v0.1: domain claim stub (manual / future DNS verify)

CREATE TABLE console_domain_claims (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    domain TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    verification_token TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    verified_at TIMESTAMPTZ,
    CONSTRAINT console_domain_claims_domain_unique UNIQUE (domain)
);

CREATE INDEX idx_console_domain_claims_status ON console_domain_claims (status);
