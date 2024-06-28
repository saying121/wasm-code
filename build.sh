#!/usr/bin/env bash

cargo component build -r

profile=release

adder=./target/wasm32-wasi/$profile/adder.wasm
calc=./target/wasm32-wasi/$profile/calculator.wasm
calc_pluged=./target/wasm32-wasi/$profile/calc-pluged.wasm
command=./target/wasm32-wasi/$profile/command.wasm
final=./target/wasm32-wasi/$profile/final.wasm

wac plug $calc --plug $adder -o $calc_pluged
wac plug $command --plug $calc_pluged -o $final
