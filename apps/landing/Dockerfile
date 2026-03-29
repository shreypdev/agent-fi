# Pronox landing — static site (pnpm workspace). Build from repository root:
#   docker build -f apps/landing/Dockerfile -t pronox-landing .
#
# Pass public URLs at build time (Vite bakes them in):
#   docker build -f apps/landing/Dockerfile \
#     --build-arg VITE_SEARCH_API_BASE_URL=https://searchd.example.com \
#     --build-arg VITE_PUBLIC_AGENT_URL=https://agent.example.com \
#     -t pronox-landing .
#
# Railway (Landing service): Root Directory = repo root, Dockerfile path = apps/landing/Dockerfile,
# set the same variables under Docker → Build Args (or Variables → Build).

FROM node:22-bookworm-slim AS builder
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable && corepack prepare pnpm@9.14.2 --activate
WORKDIR /repo

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml turbo.json ./
COPY apps/landing/package.json ./apps/landing/

RUN pnpm install --frozen-lockfile --filter "@agentfi/landing..."

COPY apps/landing ./apps/landing

ARG VITE_SEARCH_API_BASE_URL
ARG VITE_PUBLIC_AGENT_URL
ENV VITE_SEARCH_API_BASE_URL=$VITE_SEARCH_API_BASE_URL
ENV VITE_PUBLIC_AGENT_URL=$VITE_PUBLIC_AGENT_URL

RUN pnpm --filter @agentfi/landing run build

FROM nginx:1.27-alpine AS runner
COPY --from=builder /repo/apps/landing/docker/nginx.conf.template /etc/nginx/nginx.conf.template
COPY --from=builder /repo/apps/landing/dist /usr/share/nginx/html
ENV PORT=3000
EXPOSE 3000
# Railway sets PORT; substitute into generated nginx main config (writable /tmp for pid).
CMD sh -c 'sed "s/PORT_PLACEHOLDER/${PORT}/g" /etc/nginx/nginx.conf.template > /tmp/nginx.conf && exec nginx -c /tmp/nginx.conf -g "daemon off;"'
