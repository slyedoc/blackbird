# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# Install required tools
# Tailwind/cli: nodejs npm 
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang nodejs npm

#RUN apk update && \
#    apk add --no-cache bash curl nodejs npm build-base gcc wget libc-dev binaryen ca-certificates fuse3 sqlite openssl-dev

RUN rustup show

#RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
#RUN cargo install --git https://github.com/bram209/leptosfmt.git 
RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos


# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
# Make app directory
WORKDIR /app
COPY . .

# Build the app
RUN npm install
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs
COPY --from=builder /app/target/release/blackbird /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/public /app/public
COPY --from=builder /app/assets /app/asssets
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/tools/docker_start.sh /app/

RUN chmod +x /app/docker_start.sh
ENV BEVY_ASSET_ROOT=/app/
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

# https://docs.docker.com/engine/containers/multi-service_container/
ENTRYPOINT "./app/docker_start.sh"