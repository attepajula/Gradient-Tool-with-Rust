# ── Stage 1: Build frontend ───────────────────────────────────────────────────
FROM node:20-alpine AS frontend
WORKDIR /app/frontend
RUN npm install -g pnpm
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY frontend/ ./
RUN pnpm run build

# ── Stage 2: Build Rust backend ───────────────────────────────────────────────
FROM rust:1.94 AS backend
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
RUN cargo build --release -p api

# ── Stage 3: Runtime ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates libssl3 \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=backend /app/target/release/api ./api
COPY --from=frontend /app/frontend/dist ./frontend/dist

ENV PORT=3000
ENV STATIC_DIR=frontend/dist
EXPOSE 3000

CMD ["./api"]
