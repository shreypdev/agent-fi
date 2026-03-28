# ml-pipeline

Python workspace for **offline** AgentRank work: embeddings, learning-to-rank training, evaluation harnesses. This package is intentionally **not** on the search or crawl hot path (see `apps/agentrank` for Rust services).

## Setup (uv)

Install [uv](https://docs.astral.sh/uv/), then:

```bash
cd apps/ml-pipeline
uv sync --extra dev
```

## Commands

```bash
uv run ruff check src tests
uv run ruff format --check src tests
uv run pytest
```

Format in place:

```bash
uv run ruff format src tests
```

## CI

The root GitHub workflow runs Ruff and pytest on this directory.

## Week 1 scope

Placeholder package only — no training code until later milestones.
