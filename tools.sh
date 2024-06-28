#!/usr/bin/env bash

# show wat
wasm-tools print *.wasm

# show export, import
wasm-tools component wit target/wasm32-wasi/release/adder.wasm
