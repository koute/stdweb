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

if [ "$IS_NIGHTLY" = "0" ]; then
    if [ "$TARGET" = "wasm32-unknown-unknown" ]; then
        echo "Skipping tests; wasm32-unknown-unknown is only supported on nightly"
        exit 0
    fi
fi

cargo install cargo-web -f

if [ "$TARGET" = "asmjs-unknown-emscripten" ]; then
    rustup target add asmjs-unknown-emscripten
    export CARGO_WEB_ARGS="--target-asmjs-emscripten"
    cargo web test --features web_test $CARGO_WEB_ARGS
fi

if [ "$TARGET" = "wasm32-unknown-emscripten" ]; then
    rustup target add wasm32-unknown-emscripten
    export CARGO_WEB_ARGS="--target-webasm-emscripten"
    cargo web test --features web_test $CARGO_WEB_ARGS
fi

if [ "$TARGET" = "wasm32-unknown-unknown" ]; then
    rustup target add wasm32-unknown-unknown
    export CARGO_WEB_ARGS="--target-webasm"
    cargo web test --nodejs $CARGO_WEB_ARGS
fi

cd examples/todomvc
cargo web build --release $CARGO_WEB_ARGS
