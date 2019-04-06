#!/bin/bash

set -x
rm docs/uasatrs.js docs/uasatrs_bg.wasm
cargo +nightly build --target wasm32-unknown-unknown --release --no-default-features --features wasm
wasm-bindgen target/wasm32-unknown-unknown/release/uasatrs.wasm --target web --no-typescript --out-dir docs
