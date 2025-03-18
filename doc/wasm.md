# Wasm build issues

Spent 2 weeks tracking down a few issues

1. Cargo-leptos fails to run wasm-opt if optimzation passes are used, will give random segfaults and other memory errors, but wasm-opt from binaryen works fine, have a [fork](https://github.com/slyedoc/cargo-leptos) of cargo-leptos fork that disabled its, then run [wasm-opt](https://github.com/WebAssembly/binaryen/releases) manually after ```cargo letpos build -r -v --features all_games```, see [build_opt.sh](../tools/build_opt.sh) to see results of different options
   - trunk and wasm-pack dont have this issue, have tried stable and nightly any many versions trying to figure out workaround, for now maualstep it is

2. I plan on using webgpu since I want compute shader support.  But in chrome I was getting cpu rendering instead of gpu with bevy/webgpu, turns out at least on popos if you set ozone-platform=wayland then vulkan becomes disabled, for now wayland is the only choice

```
--ozone-platform=wayland' is not compatible with Vulkan. Consider switching to '--ozone-platform=x11' or disabling Vulkan
```

## Resouces
- [Bevy WebGL2 and WebGPU](https://github.com/bevyengine/bevy/tree/main/examples#webgl2-and-webgpu)
- [webgpu support](https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API#browser_compatibility)
- [bevy_hanabi overfiew](https://github.com/djeedai/bevy_hanabi/blob/main/docs/wasm.md)
- [binaryen/release](https://github.com/WebAssembly/binaryen/releases)
  - wasm-opt


## TODOS

```
wasm-opt -Os --output output.wasm input.wasm
```


 - [Audio autoplay](https://developer.chrome.com/blog/web-audio-autoplay/)


# Build sizes

| options  | time   | size | features    |
| -------- | ------ | ---- |  --------           |
|  release  | 6m 15s | 47k |             |
|  dev  | 2m 40s | 47k |          |
| release |    | 57M  | hydrate   |    
- wasm-pack build --release && ls -lsh ./pkg/*.wasm | awk '{print $6}'
- 

# WASM, for 

This is run down of build times for hot reload debugging


in chrome, if wayland is enabled, vulkan is disabled, making webgpu cpu based: chrome will log this message

