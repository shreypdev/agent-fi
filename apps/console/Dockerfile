# Agent Search Console — static SPA (pnpm workspace). Build from repository root:
#   docker build -f apps/console/Dockerfile \
#     --build-arg VITE_CONSOLE_API_BASE=https://consoled.example.com \
#     -t agentrank-console .
#
# Railway: Root Directory = repo root, Config as code = apps/console/railway.toml,
# set VITE_CONSOLE_API_BASE under Variables → Build (Docker build args).

FROM node:22-bookworm-slim AS builder
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable && corepack prepare pnpm@9.14.2 --activate
WORKDIR /repo

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml turbo.json ./
COPY apps/console/package.json ./apps/console/

RUN pnpm install --frozen-lockfile --filter "@agentfi/console..."

COPY apps/console ./apps/console

ARG VITE_CONSOLE_API_BASE
ENV VITE_CONSOLE_API_BASE=$VITE_CONSOLE_API_BASE

RUN pnpm --filter @agentfi/console run build

FROM nginx:1.27-alpine AS runner
COPY --from=builder /repo/apps/console/docker/nginx.conf.template /etc/nginx/nginx.conf.template
COPY --from=builder /repo/apps/console/dist /usr/share/nginx/html
ENV PORT=3000
EXPOSE 3000
CMD sh -c 'sed "s/PORT_PLACEHOLDER/${PORT}/g" /etc/nginx/nginx.conf.template > /tmp/nginx.conf && exec nginx -c /tmp/nginx.conf -g "daemon off;"'
