# KTX-Software

 - [Release](https://github.com/KhronosGroup/KTX-Software/releases)
  
  Creating skybox textures:

Break up vertial stack
```bash
convert space.png -crop 1434x1434 +repage face_%d.png 
```

```bash
mv face_0.png posx.png
mv face_1.png negx.png
mv face_2.png posy.png
mv face_3.png negy.png
mv face_4.png posz.png
mv face_5.png negz.png
```

```bash
ktx create --format R8G8B8A8_SRGB --assign-tf sRGB --cubemap posx.png negx.png posy.png negy.png posz.png negz.png skybox.ktx2
```

```
ktx create --cubemap --format R8G8B8A8_SRGB --zstd 18 --assign-tf sRGB --assign-primaries bt709 --generate-mipmap posx.png negx.png posy.png negy.png posz.png negz.png skybox.ktx2
```

### 

The pisa_*.ktx2 files were generated from https://github.com/KhronosGroup/glTF-Sample-Environments/blob/master/pisa.hdr using the following tools and commands:
- IBL environment map prefiltering to cubemaps: https://github.com/KhronosGroup/glTF-IBL-Sampler
  - Diffuse: ./cli -inputPath pisa.hdr -outCubeMap pisa_diffuse.ktx2 -distribution Lambertian -cubeMapResolution 32
  - Specular: ./cli -inputPath pisa.hdr -outCubeMap pisa_specular.ktx2 -distribution GGX -cubeMapResolution 512
- Converting to rgb9e5 format with zstd 'supercompression': https://github.com/DGriffin91/bevy_mod_environment_map_tools
  - cargo run --release -- --inputs pisa_diffuse.ktx2,pisa_specular.ktx2 --outputs pisa_diffuse_rgb9e5_zstd.ktx2,pisa_specular_rgb9e5_zstd.ktx2