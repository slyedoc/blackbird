# could be different for other people
# load comfyui virtual environment
#source ../ComfyUI/.venv/bin/activate

# start comfyui but override the output directory to make life easier
# --lowvram --verbose DEBUG
python ../ComfyUI/main.py  --output-directory art/output --user-directory art/user --input-directory art/input 