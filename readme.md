# Blackbird



## Notes

- Frontend
    - [Bevy](https://bevyengine.org)
    - [Leptos](https://github.com/leptos-rs/leptos) web framework
    - [TailwindCSS](https://tailwindcss.com/)   
 - Backend
   - Axum
   - Leptos SSR
   - Postgres
   


## Getting Started

Set sqlx-cli

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

Install lld and mold, see bevy [notes](https://bevyengine.org/learn/quick-start/getting-started/setup/#cranelift) for more info

```bash
sudo apt-get install lld clang
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

get open ssl

```bash
sudo apt install libssl-dev
```

setup rust 
```bash 
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

setup cargo leptos, leptosfmt, wasm-opt

```bash
cargo install leptosfmt
cargo install wasm-opt --locked
cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos
```

> cargo-leptos does not use tailwind vite cli, so dont need to get anything from node right now
<!--
> Install node packages

> ```bash
> npm install
> ```
> -->

## Docker Install (Linux)

```bash
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# fix permissions
sudo usermod -a -G docker $USER
newgrp docker
```

## Deployment

Site is deployed to AWS

Install [aws cli](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)


- Leptos SSR

# Commands

Common commands (run one):

```bash
# run server locally
cargo leptos watch

# run local game
cargo run -p breakout
```

Docker commands (run one):

```bash
# build docker
docker build .

# clear docker cache
docker buildx prune -f
```

## Addons

Install [Tailwind CSS Intellisense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss).

