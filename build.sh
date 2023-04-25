#!/bin/sh

cargo build --target wasm32-wasi --release
wasm-tools component new ./target/wasm32-wasi/release/hello_wasm_hermes.wasm -o hermes-consumer.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm

echo "hermes consumer written to ./hermes-consumer.wasm"
