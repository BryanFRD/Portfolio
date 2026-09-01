FROM docker.io/library/node:24-alpine AS build
WORKDIR /app
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable
COPY frontend/package.json frontend/pnpm-lock.yaml frontend/pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile
COPY frontend/ ./
RUN pnpm run build

FROM docker.io/library/node:24-alpine
WORKDIR /app
RUN apk upgrade --no-cache \
    && rm -rf /usr/local/lib/node_modules /usr/local/bin/npm /usr/local/bin/npx /usr/local/bin/corepack /opt/yarn*
ENV NODE_ENV=production
ENV PORT=4000
COPY --from=build /app/dist/portfolio-frontend ./dist
USER node
EXPOSE 4000
CMD ["node", "dist/server/server.mjs"]
