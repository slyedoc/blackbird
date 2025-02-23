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

See the [Examples README](../README.md) for setup and run instructions.

## Tailwind

`Trunk.toml` is configured to build the CSS automatically.

```bash
npm install -D tailwindcss
```

## Setting up with VS Code and Additional Tools



Install [Tailwind CSS Intellisense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss).

Install [VS Browser](https://marketplace.visualstudio.com/items?itemName=Phu1237.vs-browser) extension (allows you to open a browser at the right window).

Allow vscode Ports forward: 3000, 3001.

### Attribution

Many thanks to GreatGreg for putting together this guide. You can find the original, with added details, [here](https://github.com/leptos-rs/leptos/discussions/125).

## Quick Start

Run `trunk serve --open` to run this example.
