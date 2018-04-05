#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

CARGO_WEB=${CARGO_WEB:-cargo-web}

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
$CARGO_WEB test --features web_test --target=asmjs-unknown-emscripten

pushd examples/todomvc > /dev/null
$CARGO_WEB build --release --target=asmjs-unknown-emscripten
popd > /dev/null

echo "Testing for wasm32-unknown-emscripten..."
$CARGO_WEB test --features web_test --target=wasm32-unknown-emscripten

pushd examples/todomvc > /dev/null
$CARGO_WEB build --release --target=wasm32-unknown-emscripten
popd > /dev/null

if [ "$IS_NIGHTLY" = "1" ]; then
    $CARGO_WEB test --nodejs --target=wasm32-unknown-unknown

    pushd examples/todomvc > /dev/null
    $CARGO_WEB build --release --target=wasm32-unknown-unknown
    popd > /dev/null

    pushd standalone-tests > /dev/null
    $CARGO_WEB build --release --target=wasm32-unknown-unknown
    node target/wasm32-unknown-unknown/release/standalone-tests.js
    popd > /dev/null
fi

NIGHTLY_EXAMPLES=(hasher)
STABLE_EXAMPLES=(canvas echo minimal todomvc webgl)
if [ "$IS_NIGHTLY" = "1" ]; then
    EXAMPLES=( "${NIGHTLY_EXAMPLES[@]}" "${STABLE_EXAMPLES[@]}" )
else
    EXAMPLES=( "${STABLE_EXAMPLES[@]}" )
fi

for EXAMPLE in "${EXAMPLES[@]}"; do
    echo "Building example: $EXAMPLE"
    pushd examples/$EXAMPLE > /dev/null
    $CARGO_WEB build
    popd > /dev/null
    echo ""
done

if [ "$IS_NIGHTLY" = "1" ]; then
    pushd examples/hasher > /dev/null
    node example.js
    popd > /dev/null

    pushd examples/hasher-parcel > /dev/null
    npm install
    $(npm bin)/parcel build index.html
    popd > /dev/null
fi
