# Blackbird



## Notes

- Frontend
    - [Bevy](https://bevyengine.org)
    - [Leptos](https://github.com/leptos-rs/leptos) web framework
    - [TailwindCSS](https://tailwindcss.com/)   
- Tools
    - [trunk](https://github.com/thedodd/trunk) tool.

- 
## Getting Started

Set env var for asset path from project root:

```bash
export BEVY_ASSET_ROOT=${PWD}
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

setup cargo leptos and leptosfmt

```bash
cargo install leptosfmt
cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos
```

Install node packages

```bash
npm install
```

Docker

```bash
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# fix permissions
sudo usermod -a -G docker $USER
newgrp docker
```



## Deployment

using fly.io with setup from [leptos book](https://book.leptos.dev/deployment/ssr.html#deploy-to-flyio)

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
