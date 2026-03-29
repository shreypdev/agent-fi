# Build from repository root:
#   docker build -f apps/agentrank/Dockerfile -t agentrank-searchd .
#
# Runtime runs migrate + index rebuild + searchd (see docker/entrypoint-searchd.sh).
# Override SEARCHD_BOOT_MIGRATE=0 / SEARCHD_BOOT_REBUILD_INDEX=0 if you handle those elsewhere.
FROM rust:1.88-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY apps/agentrank /app
RUN cargo build --release -p agentrank-searchd \
  && cargo build --release -p agentrank-search-index --bin agentrank-index
RUN cargo install sqlx-cli --version 0.8.2 --root /opt/sqlx \
  --no-default-features --features rustls,postgres \
  && mv /opt/sqlx/bin/sqlx /opt/sqlx/sqlx

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/sqlx/sqlx /usr/local/bin/sqlx
COPY --from=builder /app/target/release/searchd /usr/local/bin/searchd
COPY --from=builder /app/target/release/agentrank-index /usr/local/bin/agentrank-index
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/docker/entrypoint-searchd.sh /entrypoint-searchd.sh
RUN chmod +x /entrypoint-searchd.sh
WORKDIR /app
ENV PORT=8080
ENV SEARCH_INDEX_PATH=/tmp/agentrank-index
EXPOSE 8080
ENTRYPOINT ["/entrypoint-searchd.sh"]
