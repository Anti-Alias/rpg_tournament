#!/usr/bin/env bash

# Builds as wasm project
cargo build --release --target wasm32-unknown-unknown

# Creates js bindings and copies asset directory
wasm-bindgen --no-typescript --target web \
    --out-dir "$WASM_OUT" \
    --out-name "rpg_tournament" \
    ./target/wasm32-unknown-unknown/release/rpg_tournament.wasm

# Copies to wasm out directory
cp -r assets $WASM_OUT/..