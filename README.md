# Roc Playground Experiment

Uses cargo wasi, install with `cargo install cargo-wasi`

## Run locally

```sh
$ cargo wasi run
```

## Run in runno

```sh
$ cargo build --release
$ cp target/wasm32-wasi/release/playground.wasm playground.wasi.wasm
```

Then upload `playground.wasi.wasm` to https://runno.dev/wasi
