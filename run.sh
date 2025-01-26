#!/bin/bash

# https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/
set -euxo pipefail

# Default build mode
BUILD_MODE=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --release|-r)
      BUILD_MODE="--release"
      shift # Remove --release from processing
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [--release|-r]"
      exit 1
      ;;
  esac
done

# remove previous build
rm -f playground.wasi.wasm

# build our wasm module
cargo build $BUILD_MODE

# determine the source directory based on build mode
if [ "$BUILD_MODE" = "--release" ]; then
  SOURCE_DIR="release"
else
  SOURCE_DIR="debug"
fi

# copy the wasm module to the root directory, with the .wasi.wasm extension required by runno
cp "target/wasm32-wasip1/$SOURCE_DIR/playground.wasm" playground.wasi.wasm

# show the size of the wasm module
ls -hl playground.wasi.wasm

# run with wasmtime from the example/ directory
cd example && wasmtime run --dir . ../playground.wasi.wasm app.roc
