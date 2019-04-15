set -euo pipefail
IFS=$'\n\t'

WASM_PACK_RELEASE=$(curl -L -s -H 'Accept: application/json' https://github.com/rustwasm/wasm-pack/releases/latest)
WASM_PACK_VERSION=$(echo $WASM_PACK_RELEASE | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')
WASM_PACK_URL="https://github.com/rustwasm/wasm-pack/releases/download/$WASM_PACK_VERSION/wasm-pack-$WASM_PACK_VERSION-x86_64-unknown-linux-musl.tar.gz"

echo "Downloading wasm-pack from: $WASM_PACK_URL"
echo $WASM_PACK_URL

cd /tmp

curl -L $WASM_PACK_URL | tar zxf -
export DIR="wasm-pack-$WASM_PACK_VERSION-x86_64-unknown-linux-musl"

mkdir -p ~/.cargo/bin
mv "$DIR/wasm-pack" ~/.cargo/bin
