# building rust
FROM rust as builder
WORKDIR /usr/src
COPY . /usr/src
ENV PROTOC_ZIP=protoc-30.0-linux-x86_64.zip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v30.0/$PROTOC_ZIP \
    && unzip -o $PROTOC_ZIP -d /usr/local bin/protoc \
    && unzip -o $PROTOC_ZIP -d /usr/local 'include/*' \ 
    && rm -f $PROTOC_ZIP
# when rust/templates have changed, we'll bust asset caches
RUN find templates -type f -name "*.html.askama" -print0 | xargs -0 sed -i "s/v001/v$(openssl rand -hex 5)/g"
RUN  --mount=type=cache,target=/usr/src/target \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git \
  cargo build --release; \
  cp target/release/roundhouse-cli .

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
ENTRYPOINT ["/usr/app/roundhouse-cli", "start"]
