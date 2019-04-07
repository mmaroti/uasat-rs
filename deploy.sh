#!/bin/bash

set -x
rm docs/uasat.js docs/uasat_bg.wasm
cargo +nightly build --target wasm32-unknown-unknown --release --no-default-features --features wasm
wasm-bindgen target/wasm32-unknown-unknown/release/uasat.wasm --target web --no-typescript --out-dir docs
