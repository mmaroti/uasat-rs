#!/bin/bash

set -x
rm docs/uasatlib.js docs/uasatlib_bg.wasm
cargo +nightly build --target wasm32-unknown-unknown --lib --release --features wasm
wasm-bindgen target/wasm32-unknown-unknown/release/uasatlib.wasm --target web --no-typescript --out-dir docs
