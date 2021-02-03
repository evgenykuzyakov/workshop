#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release --features for_real
cp target/wasm32-unknown-unknown/release/berry_bot.wasm ./res/

