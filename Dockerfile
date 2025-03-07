# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder


RUN apk update && \
    apk add --no-cache bash curl nodejs npm build-base gcc wget libc-dev binaryen ca-certificates fuse3 sqlite libssl-dev




#RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
RUN cargo install leptosfmt
RUN cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos


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


ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

ENTRYPOINT litefs mount

CMD [ "sh", "-c" "litefs mount; /app/blackbird"]