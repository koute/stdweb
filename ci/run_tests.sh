#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

export RUST_BACKTRACE=1

CARGO_WEB=${CARGO_WEB:-cargo-web}
SKIP_RUNTIME_COMPATIBILITY_CHECK=${SKIP_RUNTIME_COMPATIBILITY_CHECK:-0}

set +e
echo "$(rustc --version)" | grep -q "nightly"
if [ "$?" = "0" ]; then
    export IS_NIGHTLY=1
else
    export IS_NIGHTLY=0
fi
set -e

echo "Is Rust from nightly: $IS_NIGHTLY"

pushd stdweb-internal-macros > /dev/null
cargo test
popd > /dev/null

# echo "Testing for wasm32-unknown-unknown (wasm-bindgen)..."
# wasm-pack test --headless --chrome -- --features web_test

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

echo "Testing for wasm32-unknown-unknown..."
$CARGO_WEB test --nodejs --target=wasm32-unknown-unknown

pushd examples/todomvc > /dev/null
$CARGO_WEB build --release --target=wasm32-unknown-unknown
popd > /dev/null

echo "Building standalone tests..."
pushd standalone-tests > /dev/null
$CARGO_WEB build --release --target=wasm32-unknown-unknown

echo "Running standalone tests..."
node target/wasm32-unknown-unknown/release/standalone-tests.js
popd > /dev/null

if [ "$SKIP_RUNTIME_COMPATIBILITY_CHECK" == "0" ]; then
    echo "Checking whenever the old version still works with the newest runtime..."

    if [ ! -d target/old-version ]; then
        git fetch origin refs/tags/0.4.9:refs/tags/0.4.9

        pushd target > /dev/null
        git clone .. old-version
        popd > /dev/null
    fi

    pushd target/old-version > /dev/null
    git checkout 0.4.9

    set +e
    grep -q 'path = "../../stdweb-internal-runtime"' Cargo.toml
    if [ "$?" = "0" ]; then
        ALREADY_PATCHED=1
    else
        ALREADY_PATCHED=0
    fi
    set -e

    if [ "$ALREADY_PATCHED" = "0" ]; then
        sed 's/path = "stdweb-internal-runtime"/path = "..\/..\/stdweb-internal-runtime"/' -i Cargo.toml
        # Sanity check to make sure the replacement was done successfully.
        grep -q 'path = "../../stdweb-internal-runtime"' Cargo.toml
    fi

    echo "Testing old version on asmjs-unknown-emscripten..."
    $CARGO_WEB test --features web_test --target=asmjs-unknown-emscripten

    echo "The runtime is compatible!"
    popd > /dev/null
fi

NIGHTLY_EXAMPLES=()
STABLE_EXAMPLES=(canvas echo minimal todomvc webgl hasher)
if [ "$IS_NIGHTLY" = "1" ]; then
    set +u
    EXAMPLES=( "${NIGHTLY_EXAMPLES[@]}" "${STABLE_EXAMPLES[@]}" )
    set -u
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

echo "Trying to run the hasher example..."
pushd examples/hasher > /dev/null
node example.js
popd > /dev/null

if [ "$IS_NIGHTLY" = "1" ]; then
    echo "Trying to build with parcel..."
    pushd examples/hasher-parcel > /dev/null
    npm install
    $(npm bin)/parcel build index.html
    popd > /dev/null
fi
