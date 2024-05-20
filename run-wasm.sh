#!/usr/bin/env bash
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner
cargo run \
    --target wasm32-unknown-unknown \
    --profile wasm-release \
