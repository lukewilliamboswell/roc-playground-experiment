#!/bin/bash

# https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/
set -euxo pipefail

# remove previous build
rm -f playground.wasi.wasm

# build our wasm module
cargo build

# copy the wasm module to the root directory, with the .wasi.wasm extension required by runno
cp target/wasm32-wasip1/debug/playground.wasm playground.wasi.wasm

# show the size of the wasm module
ls -hl playground.wasi.wasm

# run with wasmtime from the example/ directory
cd example && wasmtime run --dir . ../playground.wasi.wasm
