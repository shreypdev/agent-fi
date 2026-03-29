#!/usr/bin/env python3
"""Emit SQL INSERTs for synthetic agents (deterministic UUIDv5). For local/CI load tests.

Usage:
  python3 scripts/gen_1k_agents_sql.py --count 1000 > /tmp/seed.sql
  psql "$DATABASE_URL" -f /tmp/seed.sql

CI often uses a smaller count (e.g. 100) for faster indexer smoke.
"""
from __future__ import annotations

import argparse
import json
import uuid

NS = uuid.UUID("6ba7b810-9dad-11d1-80b4-00c04fd430c8")


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--count", type=int, default=1000, help="number of agent rows")
    args = p.parse_args()
    print("BEGIN;")
    for i in range(args.count):
        uid = uuid.uuid5(NS, f"agent-{i}")
        ext = f"seed-{i}"
        endpoint = f"https://seed.example/agents/{i}"
        card = {
            "name": f"Seed Agent {i}",
            "description": f"Synthetic agent number {i} for search load testing.",
            "version": "1",
            "url": endpoint,
            "skills": [
                {
                    "name": f"skill-{i % 50}",
                    "tags": [f"tag-{i % 20}", "seed"],
                    "description": f"Capability slice {i}",
                }
            ],
        }
        card_s = json.dumps(card, separators=(",", ":"))
        # content_hash is required; use stable placeholder for synthetic rows
        ch = f"seedhash{i:08d}"
        print(
            f"INSERT INTO agents (id, external_id, source_url, canonical_url, card_json, content_hash, "
            f"name, description, endpoint_url, protocol_version) VALUES ("
            f"'{uid}', '{ext}', '{endpoint}/card', '{endpoint}/card', "
            f"$seed${card_s}$seed$::jsonb, '{ch}', "
            f"'Seed Agent {i}', 'Synthetic agent number {i} for search load testing.', "
            f"'{endpoint}', '1');"
        )
    print("COMMIT;")


if __name__ == "__main__":
    main()
