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
This is done in `./site`'s build script.
It deletes the inner package.json (which is unneeded).
Excluding those nested package.json files using gitignore or npmignore does not work.
Deleting these nested package.json files causes the .gitignore in the nested folders to over ride the top level files list, so we delete those as well.

I tried to copy the readme into the cop level package with `copyfiles -V ../pkg_combo/bundler/README.md ../pkg_combo/`

but that fails to do anything useful:

```
unglobed path: ../pkg_combo/bundler/README.md
copy from: ../pkg_combo/bundler/README.md
copy to: ../pkg_combo/bundler/README.md
```

so no package readme for now.

Issues:

https://github.com/microsoft/TypeScript/issues/46452

https://github.com/rustwasm/wasm-pack/issues/1039

# Publishing

1. `cd ./site`
1. Update version in `Cargo.toml`
1. `npm run build`
1. `cd ../pkg_combo`
1. Update version in `pkg_combo\package.json`
1. `npm publish` (I recommend `npm publish --dry-run` first)

If you see an error like

```
npm ERR! code E404
npm ERR! 404 Not Found - PUT https://registry.npmjs.org/uuid-cluster - Not found
npm ERR! 404
npm ERR! 404  'uuid-cluster@0.2.1' is not in the npm registry.
npm ERR! 404 You should bug the author to publish it (or use the name yourself!)
npm ERR! 404
npm ERR! 404 Note that you can also install from a
npm ERR! 404 tarball, folder, http url, or git url.
```

This means you need to `npm login`.
