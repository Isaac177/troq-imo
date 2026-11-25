FROM --platform=$BUILDPLATFORM rust:1.93-bookworm AS builder
ARG ZIG_VERSION=0.13.0
RUN apt-get update && apt-get install -y --no-install-recommends cmake xz-utils && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://ziglang.org/download/${ZIG_VERSION}/zig-linux-$(uname -m)-${ZIG_VERSION}.tar.xz | tar -xJ -C /opt && \
    ln -s /opt/zig-linux-$(uname -m)-${ZIG_VERSION}/zig /usr/local/bin/zig
RUN rustup target add x86_64-unknown-linux-musl && cargo install cargo-zigbuild --locked
WORKDIR /app
COPY backend/ ./
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=shared \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=shared \
    --mount=type=cache,target=/root/.cache/zig,sharing=shared \
    --mount=type=cache,target=/app/target,id=mungini-api,sharing=locked \
    cargo zigbuild --release --target x86_64-unknown-linux-musl -p api && \
    cp target/x86_64-unknown-linux-musl/release/mungini-api /mungini-api

FROM alpine:3.22 AS runtime
RUN apk add --no-cache ca-certificates tini
ENTRYPOINT ["/sbin/tini", "--"]
COPY --from=builder /mungini-api /usr/local/bin/mungini-api
COPY --from=builder /app/migrations /migrations
ENV APP_ENV=production
EXPOSE 8080
CMD ["mungini-api"]
