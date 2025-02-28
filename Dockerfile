# building rust
FROM rust:slim as builder
WORKDIR /usr/src
COPY . /usr/src
# when rust/templates have changed, we'll bust asset caches
RUN find templates -type f -name "*.html.askama" -print0 | xargs -0 sed -i "s/v001/v$(openssl rand -hex 5)/g"
RUN  --mount=type=cache,target=/usr/src/target \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git \
  cargo build --release; \
  cp target/release/roundhouse-cli .; \
  cp target/release/gtfs_import .

# building css
FROM oven/bun:latest as bunbuilder
WORKDIR /usr/src/app
COPY . /usr/src/app
RUN bun install tailwindcss;
RUN bunx @tailwindcss/cli -i config/tailwind.css --config config/tailwind.config.js -o public/assets/styles.css;
RUN gzip -r -k public/assets

# combining into shipment
FROM debian:bookworm-slim
WORKDIR /usr/app
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=bunbuilder /usr/src/app/public /usr/app/public
COPY --from=builder /usr/src/roundhouse-cli /usr/app/roundhouse-cli
COPY --from=builder /usr/src/gtfs_import /usr/app/gtfs_import
ENTRYPOINT ["/usr/app/roundhouse-cli", "start"]
