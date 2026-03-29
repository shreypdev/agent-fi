# Build from repository root:
#   docker build -f apps/agentrank/Dockerfile -t agentrank .
#
# One image, three processes (Railway: duplicate service, same Dockerfile, set AGENTRANK_PROCESS):
#   - searchd  — default; migrations + index boot + searchd (entrypoint-searchd.sh)
#   - consoled — Console API (migrations run in consoled binary)
#   - agentbot — crawl worker (run-loop; scale replicas; shared Redis frontier)
#
#   docker run -e AGENTRANK_PROCESS=consoled -e DATABASE_URL=... -e CONSOLE_API_KEY=... agentrank
#   docker run -e AGENTRANK_PROCESS=agentbot -e DATABASE_URL=... -e REDIS_URL=... agentrank
FROM rust:1.88-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY apps/agentrank /app
RUN cargo build --release -p agentrank-searchd \
  && cargo build --release -p agentrank-search-index --bin agentrank-index \
  && cargo build --release -p agentrank-consoled --bin consoled \
  && cargo build --release -p agentrank-agentbot --bin agentbot
RUN cargo install sqlx-cli --version 0.8.2 --root /opt/sqlx \
  --no-default-features --features rustls,postgres \
  && mv /opt/sqlx/bin/sqlx /opt/sqlx/sqlx

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/sqlx/sqlx /usr/local/bin/sqlx
COPY --from=builder /app/target/release/searchd /usr/local/bin/searchd
COPY --from=builder /app/target/release/agentrank-index /usr/local/bin/agentrank-index
COPY --from=builder /app/target/release/consoled /usr/local/bin/consoled
COPY --from=builder /app/target/release/agentbot /usr/local/bin/agentbot
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/docker/entrypoint-searchd.sh /entrypoint-searchd.sh
COPY --from=builder /app/docker/entrypoint-router.sh /entrypoint-router.sh
COPY --from=builder /app/docker/entrypoint-agentbot.sh /entrypoint-agentbot.sh
RUN chmod +x /entrypoint-searchd.sh /entrypoint-router.sh /entrypoint-agentbot.sh
WORKDIR /app
ENV PORT=8080
ENV SEARCH_INDEX_PATH=/tmp/agentrank-index
# searchd HTTP; optional agentbot Prometheus when AGENTBOT_METRICS_BIND is set
EXPOSE 8080 9092
ENTRYPOINT ["/entrypoint-router.sh"]
