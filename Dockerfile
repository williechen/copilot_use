# Multi-stage release build for independently deployable Chat and Analytics services.
# This Dockerfile does not package the Tauri Desktop Pet (native desktop build).
ARG COPILOT_BUILD_SHA=unversioned
FROM rust:1-bookworm AS builder
WORKDIR /src
RUN --mount=type=cache,id=copilot-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    rustup target add wasm32-unknown-unknown && cargo install trunk --version 0.21.14 --locked
COPY . .
RUN --mount=type=cache,id=copilot-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=copilot-chat-wasm,target=/src/apps/server/frontend/target,sharing=locked \
    --mount=type=cache,id=copilot-trunk-tools,target=/root/.cache/trunk,sharing=locked \
    find apps crates -type f \( -name '*.rs' -o -name '*.sql' -o -name Cargo.toml \) -exec touch {} + \
    && bash scripts/build-chat-ui.sh
RUN --mount=type=cache,id=copilot-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=copilot-services-target,target=/src/target,sharing=locked \
    find apps crates -type f \( -name '*.rs' -o -name '*.sql' -o -name Cargo.toml \) -exec touch {} + \
    && cargo build --locked --release -p day9-chat -p day9-analytics \
    && mkdir /out && cp target/release/day9-chat target/release/day9-analytics /out/

FROM debian:bookworm-slim AS chat
ARG COPILOT_BUILD_SHA
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 curl \
    && rm -rf /var/lib/apt/lists/*
LABEL org.opencontainers.image.revision=$COPILOT_BUILD_SHA
WORKDIR /app
COPY --from=builder /out/day9-chat /app/server
COPY --from=builder /src/crates/persistence/migrations /app/migrations
COPY --from=builder /src/target/site/pkg /app/site/pkg
RUN mkdir -p /data && chown 65532:65532 /data && chmod 700 /data
ENV CHAT_DATABASE_URL=sqlite:///data/chat.sqlite3
ENV CHAT_SITE_DIR=/app/site/pkg
ENV CHAT_LISTEN=0.0.0.0:3001
USER 65532:65532
EXPOSE 3001
ENTRYPOINT ["/app/server"]

FROM debian:bookworm-slim AS analytics
ARG COPILOT_BUILD_SHA
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 curl \
    && rm -rf /var/lib/apt/lists/*
LABEL org.opencontainers.image.revision=$COPILOT_BUILD_SHA
WORKDIR /app
COPY --from=builder /out/day9-analytics /app/server
COPY --from=builder /src/apps/stats_server/migrations /app/migrations
ENV ANALYTICS_LISTEN=0.0.0.0:3100
USER 65532:65532
EXPOSE 3100
ENTRYPOINT ["/app/server"]
