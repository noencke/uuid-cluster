# Uuid Cluster

## Setup

Setup Rust, wasm-pack and NodeJS as directed in https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

Installing wasm-pack on windows may be easier via: https://rustwasm.github.io/wasm-pack/installer/

run:

1. `cargo test` to make sure rust is setup
1. `cd ./site`
1. `npm run build` to wasm-pack for both bundler and nodejs, and compile site's test.ts file with tsc
1. `npm run webpack` to test that webpack works with the wasm module and our typescript settings
1. `npm run test:ts-node` run nodejs via ts-mocha
1. `npm run test:node` run node with the compiled js file
1. `npm run serve` serve webpacked data with webpack dev server

To make this all work, we run wasm pack twice and combine the results into one package (workaround for not having https://github.com/rustwasm/wasm-pack/issues/313)

Issues:
https://github.com/microsoft/TypeScript/issues/46452
https://github.com/rustwasm/wasm-pack/issues/1039
