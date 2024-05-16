#!/usr/bin/env bash

DEPLOY_DIR="$1";
ASSETS_DIR="$DEPLOY_DIR/assets"
GAME_DIR="$DEPLOY_DIR/game"

echo "DEPLOY_DIR: $DEPLOY_DIR";
echo "ASSETS_DIR: $ASSETS_DIR";
echo "GAME_DIR: $GAME_DIR"
echo "";

echo "Compiling wasm binary"
cargo build \
    --release \
    --no-default-features \
    --target wasm32-unknown-unknown \

echo "Deploying wasm binary"
wasm-bindgen --no-typescript --target web \
    --out-dir "$GAME_DIR" \
    --out-name "rpg_tournament" \
    ./target/wasm32-unknown-unknown/release/rpg_tournament.wasm

echo "Copying assets into '$ASSETS_DIR'"
rm -r "$ASSETS_DIR" 2> /dev/null
mkdir "$ASSETS_DIR"
cp -r assets "$DEPLOY_DIR"