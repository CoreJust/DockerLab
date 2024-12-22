FROM rust:1.81-alpine AS builder

# Build
WORKDIR /app
RUN apk update && apk add --no-cache musl-dev libpq-dev
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo fetch # Prepare for the build \
    && rm -rf src
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres --target x86_64-unknown-linux-musl
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl --release # Build the app

# Deploy
FROM alpine:latest AS runner
COPY migrations migrations
COPY .env .
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/dockerapp .
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/bin
RUN apk update && apk add --no-cache ca-certificates libpq
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser
ENV PORT=1337
EXPOSE 1337

CMD ["sh", "-c", "sqlx migrate run && ./dockerapp"]
