# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder


RUN apk update && \
    apk add --no-cache bash curl nodejs npm build-base gcc wget libc-dev binaryen ca-certificates fuse3 sqlite openssl-dev




#RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
#RUN cargo install --git https://github.com/bram209/leptosfmt.git 
RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos


# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work

RUN npm install

COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs

COPY --from=builder /work/target/release/blackbird /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/public /app/public
COPY --from=builder /work/assets /app/asssets
COPY --from=builder /work/Cargo.toml /app/
COPY --from=builder /work/tools/docker_start.sh docker_start.sh

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

# https://docs.docker.com/engine/containers/multi-service_container/
CMD "./docker_start.sh"