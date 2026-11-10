#!/bin/sh
# Builds the client into web/dist: the page, the wasm-bindgen shim and the
# WebAssembly module, each with a gzip sibling the server prefers.
#   ./build.sh          release (small, slow to build)
#   ./build.sh dev      debug
set -eu
cd "$(dirname "$0")"
profile=${1:-release}
if [ "$profile" = release ]; then flag=--release; else flag=; profile=debug; fi
cargo build -p tall-web --target wasm32-unknown-unknown $flag
rm -rf web/dist && mkdir -p web/dist
wasm-bindgen --target web --no-typescript --out-name tall --out-dir web/dist \
  "target/wasm32-unknown-unknown/$profile/tall_web.wasm"
cp web/static/index.html web/dist/
gzip -9 -k web/dist/*
ls -l web/dist
