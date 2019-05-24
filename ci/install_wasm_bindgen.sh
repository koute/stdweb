set -euo pipefail
IFS=$'\n\t'

WASM_BINDGEN_RELEASE=$(curl -L -s -H 'Accept: application/json' https://github.com/rustwasm/wasm-bindgen/releases/latest)
WASM_BINDGEN_VERSION=$(echo $WASM_BINDGEN_RELEASE | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')
WASM_BINDGEN_URL="https://github.com/rustwasm/wasm-bindgen/releases/download/$WASM_BINDGEN_VERSION/wasm-bindgen-$WASM_BINDGEN_VERSION-x86_64-unknown-linux-musl.tar.gz"

echo "Downloading wasm-bindgen from: $WASM_BINDGEN_URL"
echo $WASM_BINDGEN_URL

cd /tmp

curl -L $WASM_BINDGEN_URL | tar zxf -
export DIR="wasm-bindgen-$WASM_BINDGEN_VERSION-x86_64-unknown-linux-musl"

mkdir -p ~/.cargo/bin
mv "$DIR/wasm-bindgen" ~/.cargo/bin
mv "$DIR/wasm-bindgen-test-runner" ~/.cargo/bin
mv "$DIR/wasm2es6js" ~/.cargo/bin
