#!/bin/bash
log_file="command_times.log"

# Delete the log file at the start to ensure a fresh log
if [[ -f "$log_file" ]]; then
    rm "$log_file"
    #echo "Log File Reset: $log_file"
fi

# Create the log file
touch "$log_file"

log_time() {
    local start=$(date +%s)
    echo "starting: ($@)"  # Run the command passed as arguments 
    "$@"  # Run the command passed as arguments
    local exit_status=$?  # Capture the exit status
    local end=$(date +%s)
    local duration=$((end - start))
    echo 
    # Format duration to be 5 characters wide, right-aligned
    printf -v formatted_duration "%5d" "$duration"

    # get file size    
    echo "$(date '+%Y-%m-%d %H:%M:%S') ${formatted_duration} - ${exit_status} - $*" >> $log_file
}

opt() {
    
    log_time wasm-opt -O$1 --output ./target/opt//blackbird_$1.wasm ./target/opt/blackbird.wasm  
}


target="front"

#log_time cargo leptos build -r -v --features all_games

log_time cargo build --package=blackbird --lib --target-dir=/home/slyedoc/code/p/blackbird/target/$target --target=wasm32-unknown-unknown --features=hydrate,all_games --profile=wasm-release
mkdir ./target/opt/
cp ./target/$target/wasm32-unknown-unknown/wasm-release/blackbird.wasm ./target/opt/

opt "0" $target
opt "1" $target
opt "2" $target
opt "3" $target
opt "4" $target
opt "s" $target
opt "z" $target



#log_time cargo build --features ssr --release
#log_time cargo build --lib --target wasm32-unknown-unknown --profile wasm-release --features hydrate
#log_time cargo build --lib --target wasm32-unknown-unknown --profile wasm-release

#echo "Sizes:"
#cat "$log_file"

ls -lsh ./target/opt/*.wasm | awk '{print $6, $10}'