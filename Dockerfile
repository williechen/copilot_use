# Multi-stage release build for independently deployable Chat and Analytics services.
# This Dockerfile does not package the Tauri Desktop Pet (native desktop build).
FROM rust:1-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --locked --release -p day9-chat -p day9-analytics

FROM debian:bookworm-slim AS chat
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /src/target/release/day9-chat /app/server
COPY --from=builder /src/apps/chat/migrations /app/migrations
ENV CHAT_LISTEN=0.0.0.0:3001
USER 65532:65532
EXPOSE 3001
ENTRYPOINT ["/app/server"]

FROM debian:bookworm-slim AS analytics
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /src/target/release/day9-analytics /app/server
COPY --from=builder /src/apps/analytics/migrations /app/migrations
ENV ANALYTICS_LISTEN=0.0.0.0:3100
USER 65532:65532
EXPOSE 3100
ENTRYPOINT ["/app/server"]
