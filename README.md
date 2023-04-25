# hello_wasm_hermes

This is an example WASM-based consumer for Hermes.
Here is a rough step-by-step guide for how to make your own consumer.
Note: it is pretty much copied verbatim from [here](https://github.com/bytecodealliance/wit-bindgen#guest-rust), except with a few hermes-specific tweaks
 - make sure you have the `wasm32-wasi` target installed with `rustup target add wasm32-wasi`
 - make a new project with `cargo new --lib <name>`
 - add `wit-bindgen` to the `Cargo.toml`
 - make a new directory called `wit` and copy `hermes.wit` into it
 - add `wit_bindgen::generate!("hermes");` to generate the glue code
 - create a struct (here we call it `Host`, but the name doesn't matter), and call `export_hermes!(Host);`. You should get an error: `the trait bound Host: Hermes is not satisfied`
 - implement `Hermes` for your struct, and add whatever logic you want there
 - build the WASM binary with `cargo build --target wasm32-wasi --release`
 - build the WASM component with `wasm-tools component new ./target/wasm32-wasi/release/hello_wasm_hermes.wasm -o hermes-consumer.wasm --adapt ./wasi_snapshot_preview1.wasm` - you can get the `wasi-snapshot-preview1.wasm` file from <https://github.com/bytecodealliance/preview2-prototyping/releases/tag/latest> (note - you'll want the `reactor` version, not `command` version)
 - or just use `./build.sh`
