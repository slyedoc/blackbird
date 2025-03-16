- [Bevy WebGL2 and WebGPU](https://github.com/bevyengine/bevy/tree/main/examples#webgl2-and-webgpu)
- [webgpu support](https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API#browser_compatibility)
- [bevy_hanabi overfiew](https://github.com/djeedai/bevy_hanabi/blob/main/docs/wasm.md)



## TODOS

 - [Audio autoplay](https://developer.chrome.com/blog/web-audio-autoplay/)

# WASM, for 

This is run down of build times for hot reload debugging

| options  | time   | size |
| -------- | ------ | ---- |
| (none)   | 6m 15s | 49M  |
| --no-opt | 6.7s   | 76M  |


```bash
➜  blackbird git:(main) ✗ wasm-pack build --features hydrate         
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
    Finished `release` profile [optimized] target(s) in 0.17s
[INFO]: ⬇️  Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: ✨   Done in 6m 15s
[INFO]: 📦   Your wasm pkg is ready to publish at /home/slyedoc/code/p/blackbird/pkg.
➜  blackbird git:(main) ✗ ls -l -h ./pkg/*.wasm                      
-rw-rw-r-- 1 slyedoc slyedoc 49M Mar  9 17:00 ./pkg/blackbird_bg.wasm
```

```bash
➜  blackbird git:(main) ✗ wasm-pack build --no-opt --features hydrate
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
    Finished `release` profile [optimized] target(s) in 0.17s
[INFO]: ⬇️  Installing wasm-bindgen...
[INFO]: ✨   Done in 6.71s
[INFO]: 📦   Your wasm pkg is ready to publish at /home/slyedoc/code/p/blackbird/pkg.
➜  blackbird git:(main) ✗ ls -l -h ./pkg/*.wasm                      
-rw-rw-r-- 1 slyedoc slyedoc 76M Mar  9 17:01 ./pkg/blackbird_bg.wasm
```



