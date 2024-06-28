#!/usr/bin/env bash

./build.sh

profile=release

final=./target/wasm32-wasi/$profile/final.wasm

wasmtime $final 1 2 add
