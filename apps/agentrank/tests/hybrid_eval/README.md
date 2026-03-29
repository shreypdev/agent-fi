# Hybrid eval (Week 6+)

1. **Judgments:** one JSON line per query: `{"query":"...","relevant_agent_ids":["uuid",...]}`.
2. **Metric:** NDCG@10 (manual or script) comparing BM25-only vs hybrid — target **hybrid > BM25** on the held-out set.
3. **Gate:** Phase 1 uses **≥50** judged queries and **NDCG@10 ≥ 0.65** (see `a2a-discovery-todo.md` Week 12).

`scripts/hybrid_eval.sh` is a placeholder until the offline scorer is wired.
