#!/bin/

# Comfyui setup


A thousand ways to install ComfyUI, this is mine:

First assuming you have python `3.12` and nvidia card
with cuda working, see [cuda toolkit](https://developer.nvidia.com/cuda-downloads?target_os=Linux&target_arch=x86_64&Distribution=Ubuntu&target_version=24.04&target_type=deb_network)


To keep assets generated within this repo and make it easy to reproduce things, I start comfy redirecting a few things, and use `./tools/comfy.sh` as a shortcut. your path may differ

```bash
source ../ComfyUI/.venv/bin/activate
python ../ComfyUI/main.py --lowvram --output-directory art/output --user-directory art/user --input-directory art/input #--verbose DEBUG
```


## Setup 

> Needed for Python.h in Hunyuan3D
```bash
    sudo apt-get install python3-dev

```

```bash
git clone https://github.com/slyedoc/ComfyUI.git
cd ComfyUI
```
    
### Env Setup

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install torch torchvision torchaudio --extra-index-url https://download.pytorch.org/whl/cu126
pip install -r requirements.txt
```

### Install ComfyUI_essentials

```bash
cd custom_nodes
git clone https://github.com/cubiq/ComfyUI_essentials.git
cd ComfyUI_essentials
pip install -r requirements.txt
cd ../..
```

### Install Hunyuan3DWrapper 

[ComfyUI-Hunyan3dWrapper](https://github.com/kijai/ComfyUI-Hunyuan3DWrapper)

```bash
cd custom_nodes 
git clone https://github.com/kijai/ComfyUI-Hunyuan3DWrapper.git
cd ComfyUI-Hunyuan3DWrapper
pip install -r requirements.txt
cd hy3dgen/texgen/custom_rasterizer/
python setup.py install
cd ../../..
cd hy3dgen/texgen/differentiable_renderer
python setup.py build_ext --inplace
cd ../../..
cd hy3dgen/shapegen/bpt
pip install -r requirements.txt
cd ../../../../..
```

### Install MVAdapter
[ComfyUI-MVAdater](https://github.com/huanngzh/ComfyUI-MVAdapter)


```bash
cd custom_nodes
git clone https://github.com/huanngzh/ComfyUI-MVAdapter.git
cd ComfyUI-MVAdapter
pip install -r requirements.txt
mv workflow_examples example_workflows
```
> Note: Renaming workflows so they show as example

### other
```bash
    pip install xxhash 
    sudo apt-get install ffmpeg
```


# Point of no return

To get kijai workflow working you have to basiclly blow up your depentency tree, may not be worth it in the end,


-  Install ComfyUI-Manager
[ComfyUI Manager](https://github.com/ltdrdata/ComfyUI-Manager)

```bash
cd custom_nodes
git clone https://github.com/ltdrdata/ComfyUI-Manager comfyui-manager
```

Use manger to install 15+ repos,

### Download models

hunyuan - 3
```bash

mkdir -p models/diffusion_models/hy3dgen
wget -nc https://huggingface.co/Kijai/Hunyuan3D-2_safetensors/resolve/main/hunyuan3d-dit-v2-0-fp16.safetensors -P ./models/diffusion_models/hy3dgen

wget -nc https://huggingface.co/tencent/Hunyuan3D-2/resolve/main/hunyuan3d-dit-v2-0-turbo/model.fp16.safetensors -P ./models/diffusion_models/hy3dgen
mv ./models/diffusion_models/hy3dgen/model.fp16.safetensors ./models/diffusion_models/hy3dgen/hunyuan3d-dit-v2-turbo.safetensors

wget -nc https://huggingface.co/tencent/Hunyuan3D-2mv/resolve/main/hunyuan3d-dit-v2-mv-fast/model.fp16.safetensors -P ./models/diffusion_models/hy3dgen/
mv ./models/diffusion_models/hy3dgen/model.fp16.safetensors ./models/diffusion_models/hy3dgen/hunyuan3d-dit-v2-mv-fast.safetensors

wget -nc https://huggingface.co/tencent/Hunyuan3D-2mini/resolve/main/hunyuan3d-dit-v2-mini-turbo/model.fp16.safetensors -P ./models/diffusion_models/hy3dgen/
mv ./models/diffusion_models/hy3dgen/model.fp16.safetensors ./models/diffusion_models/hy3dgen/hunyuan3d-dit-v2-mv-mini-turbo.safetensors

wget -nc https://huggingface.co/tencent/Hunyuan3D-2mv/resolve/main/hunyuan3d-dit-v2-mv-turbo/model.fp16.safetensors -P ./models/diffusion_models/hy3dgen/
mv ./models/diffusion_models/hy3dgen/model.fp16.safetensors ./models/diffusion_models/hy3dgen/hunyuan3d-dit-v2-mv-turbo-fp16.safetensors

wget -nc https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0/resolve/main/sd_xl_base_1.0.safetensors -P ./models/checkpoints/

wget -nc https://huggingface.co/stabilityai/sdxl-vae/resolve/main/sdxl_vae.safetensors -P ./models/vae/
wget -nc https://huggingface.co/stabilityai/stable-zero123/resolve/main/stable_zero123.ckpt -P ./models/checkpoints/

wget -nc https://huggingface.co/nubby/blessed-sdxl-vae-fp16-fix/resolve/main/sdxl_vae-fp16fix-c-0.9.safetensors -P ./models/vae/

wget -nc https://huggingface.co/dog-god/texture-synthesis-sdxl-lora/resolve/main/texture-synthesis-3d-base-condensed.safetensors -P ./models/loras/SDXL

wget -nc https://huggingface.co/licyk/sd-upscaler-models/resolve/main/ESRGAN/BSRGANx2.pth -P ./models/upscale_models

wget -nc https://huggingface.co/Bingsu/adetailer/resolve/main/face_yolov8n.pt -P ./models/ultralytics/bbox


```
# Work flows

<!-- 

wget -nc https://huggingface.co/madebyollin/sdxl-vae-fp16-fix/resolve/main/sdxl_vae.safetensors -P ./models/vae/ 

# checkpoint
wget -nc https://huggingface.co/moonshotmillion/Photon_LCM_1.5/resolve/main/photonLCM_v10.safetensors -P ./models/checkpoints/
wget -nc https://huggingface.co/SG161222/RealVisXL_V4.0/resolve/main/RealVisXL_V4.0.safetensors -P ./models/checkpoints/sdxl/
wget -nc https://huggingface.co/frankjoshua/juggernautXL_v8Rundiffusion/resolve/main/juggernautXL_v8Rundiffusion.safetensors -P ./models/checkpoints/sdxl/

# clip
wget -nc https://huggingface.co/comfyanonymous/flux_text_encoders/resolve/main/t5xxl_fp8_e4m3fn.safetensors  -P ./models/clip/
wget -nc https://huggingface.co/comfyanonymous/flux_text_encoders/resolve/main/clip_l.safetensors -P ./models/clip/

# clip_vision
wget -nc https://huggingface.co/h94/IP-Adapter/resolve/main/models/image_encoder/model.safetensors -O ./models/clip_vision/CLIP-ViT-H-14-laion2B-s32B-b79K.safetensors
wget -nc https://huggingface.co/h94/IP-Adapter/resolve/main/sdxl_models/image_encoder/model.safetensors -O ./models/clip_vision/CLIP-ViT-bigG-14-laion2B-39B-b160k.safetensors

#loras
wget -nc https://huggingface.co/guoyww/animatediff/resolve/main/v3_sd15_adapter.ckpt -P ./models/loras/

# unet
wget -nc https://huggingface.co/black-forest-labs/FLUX.1-dev/resolve/main/flux1-dev.safetensors -P ./models/unet/
wget -nc https://huggingface.co/black-forest-labs/FLUX.1-schnell/resolve/main/flux1-schnell.safetensors -P ./models/unet/

# animatediff_models
wget -nc https://huggingface.co/moonshotmillion/AnimateDiff_LCM_Motion_Model_v1/resolve/522df61bebb1401910a3f050e943269d92407a74/animatediffLCMMotion_v10.ckpt -P ./models/animatediff_models/

# vae
wget -nc https://huggingface.co/black-forest-labs/FLUX.1-dev/resolve/main/ae.safetensors -P ./models/vae/

# controlnet
wget -nc https://huggingface.co/lllyasviel/sd-controlnet-depth/resolve/main/diffusion_pytorch_model.safetensors -O ./models/controlnet/sdxl/depth_cn.safetensors 
wget -nc https://huggingface.co/lllyasviel/sd_control_collection/resolve/main/sai_xl_depth_256lora.safetensors -P ./models/controlnet/sdxl/
wget -nc https://huggingface.co/thibaud/controlnet-openpose-sdxl-1.0/resolve/main/OpenPoseXL2.safetensors -P ./models/controlnet/sdxl/
wget -nc https://huggingface.co/TheMistoAI/MistoLine/resolve/main/mistoLine_rank256.safetensors -P ./models/controlnet/sdxl/
wget -nc https://huggingface.co/lllyasviel/ControlNet/resolve/main/models/control_sd15_depth.pth -P ./models/controlnet/sd15/
wget -nc https://huggingface.co/lllyasviel/ControlNet/resolve/main/models/control_sd15_openpose.pth -P ./models/controlnet/sd15/

# ipadapter
# See https://github.com/cubiq/ComfyUI_IPAdapter_plus?tab=readme-ov-file

wget -nc https://huggingface.co/h94/IP-Adapter/resolve/main/sdxl_models/ip-adapter-plus-face_sdxl_vit-h.safetensors -P ./models/ipadapter/
wget -nc https://huggingface.co/h94/IP-Adapter/resolve/main/models/ip-adapter-full-face_sd15.safetensors -P ./models/ipadapter/
wget -nc https://huggingface.co/h94/IP-Adapter/resolve/main/sdxl_models/ip-adapter-plus_sdxl_vit-h.safetensors -P ./models/ipadapter/
wget -nc https://huggingface.co/h94/IP-Adapter/resolve/main/sdxl_models/ip-adapter_sdxl.safetensors -P ./models/ipadapter/
wget -nc https://huggingface.co/h94/IP-Adapter/resolve/main/sdxl_models/ip-adapter_sdxl_vit-h.safetensors -P ./models/ipadapter/ -->