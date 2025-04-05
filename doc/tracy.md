# Tracy 

[Tracy](https://github.com/wolfpld/tracy)
[Bevy Profiling](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md)


## Install for Pop OS 24.04


## Dependencies

```
sudo apt install cmake dbus wayland-protocols libglvnd-dev libdbus-1-dev
```

So bevy uses [Rust-Tracy-client](https://github.com/nagisa/rust_tracy_client) and it doesnt support 11.2 yet, which is sad because building 11.1 on wayland was a pain, but after hours of messing around, foud the ```-DTBB_STRICT=OFF``` will igrone the build errors and seems to be working


- 11.1 

```
cmake -B profiler/build -S profiler -DCMAKE_BUILD_TYPE=Release -DTBB_STRICT=OFF
cmake --build profiler/build --config Release --parallel
```
- 11.2

```
cmake -B profiler/build -S profiler -DCMAKE_BUILD_TYPE=Release
cmake --build profiler/build --config Release --parallel
```