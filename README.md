# Uuid Cluster

## Setup

Setup Rust, wasm-pack and NodeJS as directed in https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

run:

1. `cargo test`
1. `cd ./site`
1. `npm run build`
1. `npm run test:ts-node`
1. `npm run test:node`
1. `npm run serve`

Issues:
https://github.com/microsoft/TypeScript/issues/46452
https://github.com/rustwasm/wasm-pack/issues/1039
