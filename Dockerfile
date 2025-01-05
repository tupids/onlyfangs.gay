FROM rust:slim-bookworm AS api-builder

WORKDIR /app

COPY migrations/ migrations/
COPY src/ src/
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release --bin onlyfangs

FROM node:20-bookworm AS frontend-builder

WORKDIR /app

COPY website .

ARG APP_URL=https://onlyfangs.gay
ARG API_URL=https://onlyfangs.gay/api

ENV NODE_ENV=production
ENV PUBLIC_API_URL=${API_URL}
ENV PUBLIC_APP_URL=${APP_URL}

RUN npm install -g pnpm
RUN pnpm install --frozen-lockfile
RUN pnpm run build

# Runtime

FROM debian:bookworm-slim

LABEL org.opencontainers.image.source="https://github.com/tupids/onlyfangs.gay" \
      org.opencontainers.image.description="Onlyfangs Application API" \
      org.opencontainers.image.licenses="MIT OR Apache-2.0"

COPY --from=api-builder /app/target/release/onlyfangs /usr/local/bin/onlyfangs
COPY --from=api-builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=frontend-builder /app/build /app/dist

ARG APP_URL=https://onlyfangs.gay
ARG API_URL=https://onlyfangs.gay/api

ENV PUBLIC_API_URL=${API_URL}
ENV PUBLIC_APP_URL=${APP_URL}
ENV VITE_DIST_DIR=/app/dist

ENTRYPOINT ["/usr/local/bin/onlyfangs"]
