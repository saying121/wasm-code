#!/usr/bin/env bash

./build.sh

dir=./target/wasm32-wasi/release/

n_comp="$dir"final.wasm
comp="$dir"final.cwasm

# Ahead-of-Time 编译
wasmtime compile $n_comp -o $comp

hyperfine -i "wasmtime --allow-precompiled $comp 10 2 add" "wasmtime $n_comp 10 2 add"
