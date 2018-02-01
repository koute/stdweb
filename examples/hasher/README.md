This example shows how to export methods from Rust and call
them from JavaScript, either in the browser or from Nodejs.

***WARNING***: This is only supported for Rust's native `wasm32-unknown-unknown` target!

See README in the root of this project for a more in-depth explanation.

### Running the demo (browser)

1. Start the demo:

    $ cargo-web start --target=wasm32-unknown-unknown

2. Visit `http://localhost:8000` with your browser.

### Running the demo (nodejs)

1. Build the example:

    $ cargo-web build --target=wasm32-unknown-unknown

2. Run it:

    $ node example.js
