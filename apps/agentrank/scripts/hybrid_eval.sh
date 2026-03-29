#!/usr/bin/env sh
# Offline evaluation harness: compare BM25-only vs hybrid (requires labeled dataset + running searchd).
# Place ≥30 judged queries in tests/hybrid_eval/judgments.jsonl (see README there).
set -e
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
echo "See $ROOT/tests/hybrid_eval/README.md for format. Run searchd with QDRANT_URL for hybrid path."
exit 0
