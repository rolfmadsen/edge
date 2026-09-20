# Multi-stage minimal Dockerfile for kant-relay (< 15MB)
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev binutils

WORKDIR /workspace

# Copy root workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/kant-relay ./crates/kant-relay

# Create minimal dummy src for desktop crate so workspace compiles cleanly
RUN mkdir -p src && echo "pub fn dummy() {}" > src/lib.rs

# Build release binary for kant-relay and strip symbols
RUN cargo build --release --package kant-relay && \
    strip /workspace/target/release/kant-relay

# Minimal runtime image
FROM alpine:3.20 AS runtime
RUN apk --no-cache add ca-certificates tzdata

WORKDIR /app
COPY --from=builder /workspace/target/release/kant-relay /usr/local/bin/kant-relay

ENV HOST=0.0.0.0 \
    PORT=8080 \
    RUST_LOG=info

USER nobody:nobody
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/kant-relay"]
