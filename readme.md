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

get open ssl

```bash
sudo apt-get install libssl-dev
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

## Commands

-Dev Site

```bash
cargo leptos watch
```

-App client side

cargo run -p breakout


Install [Tailwind CSS Intellisense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss).

Install [VS Browser](https://marketplace.visualstudio.com/items?itemName=Phu1237.vs-browser) extension (allows you to open a browser at the right window).

Allow vscode Ports forward: 3000, 3001.

### Attribution

Many thanks to GreatGreg for putting together this guide. You can find the original, with added details, [here](https://github.com/leptos-rs/leptos/discussions/125).

## Quick Start

Run `trunk serve --open` to run this example.
