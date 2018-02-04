#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

set +e
echo "$(rustc --version)" | grep -q "nightly"
if [ "$?" = "0" ]; then
    export IS_NIGHTLY=1
else
    export IS_NIGHTLY=0
fi
set -e

echo "Is Rust from nightly: $IS_NIGHTLY"

echo "Testing for asmjs-unknown-emscripten..."
cargo web test --features web_test --target=asmjs-unknown-emscripten

pushd examples/todomvc > /dev/null
cargo web build --release --target=asmjs-unknown-emscripten
popd > /dev/null

echo "Testing for wasm32-unknown-emscripten..."
cargo web test --features web_test --target=wasm32-unknown-emscripten

pushd examples/todomvc > /dev/null
cargo web build --release --target=wasm32-unknown-emscripten
popd > /dev/null

if [ "$IS_NIGHTLY" = "1" ]; then
    cargo web test --nodejs --target=wasm32-unknown-unknown

    pushd examples/todomvc > /dev/null
    cargo web build --release --target=wasm32-unknown-unknown
    popd > /dev/null

    pushd standalone-tests > /dev/null
    cargo-web build --release --target=wasm32-unknown-unknown
    node target/wasm32-unknown-unknown/release/standalone-tests.js
    popd > /dev/null
fi
