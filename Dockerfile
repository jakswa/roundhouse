# building rust
FROM rust:1.74-slim as builder
WORKDIR /usr/src
COPY . /usr/src
RUN  --mount=type=cache,target=/usr/src/target \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git \
  cargo build --release; \
  cp target/release/roundhouse-cli .

# building css
FROM oven/bun:latest as bunbuilder
WORKDIR /usr/src/app
COPY . /usr/src/app
RUN bun x tailwindcss -i public/tailwind.css --config public/tailwind.config.js -o public/styles.css;

# combining into shipment
FROM debian:bookworm-slim
WORKDIR /usr/app
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=bunbuilder /usr/src/app/public /usr/app/public
COPY --from=builder /usr/src/roundhouse-cli /usr/app/roundhouse-cli
ENTRYPOINT ["/usr/app/roundhouse-cli", "start"]
