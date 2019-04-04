#!/bin/bash

cargo +nightly build --target wasm32-unknown-unknown --lib --release
wasm-bindgen target/wasm32-unknown-unknown/release/uasatlib.wasm --no-typescript --out-dir docs