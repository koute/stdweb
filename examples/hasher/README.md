This example shows how to export methods from Rust and call
them from JavaScript, either in the browser or from Nodejs.

***WARNING***: This is only supported for Rust's native `wasm32-unknown-unknown` target on Rust nightly!

### Running the demo (browser)

1. Start the demo:

       $ cargo-web start

2. Visit `http://localhost:8000` with your browser.

### Running the demo (nodejs)

1. Build the example:

       $ cargo-web build

2. Run it:

       $ node example.js

### How does this work?

`stdweb` exports a procedural attribute macro called `js_export`
which can be used to mark arbitrary functions for export, e.g.:

```rust
#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;
extern crate sha1;

use stdweb::js_export;
use sha1::Sha1;

#[js_export]
fn sha1( string: String ) -> String {
    let mut hasher = Sha1::new();
    hasher.update( string.as_bytes() );
    hasher.digest().to_string()
}
```

This supports almost every type you can pass through the `js!` macro,
which includes objects, arrays, arbitrary DOM types, etc.

A current limitation of the `#[js_export]` is that all of the functions
you want to export must be defined in your `lib.rs`, or alternatively
they can be defined in another file, but you'll have to import them
with `pub use another_module::*` into your `lib.rs`.

If you compile this code with `cargo web build --target=wasm32-unknown-unknown` you'll get two files:

   * `target/wasm32-unknown-unknown/release/hasher.js`
   * `target/wasm32-unknown-unknown/release/hasher.wasm`

You can copy them into your JavaScript project and load like any other JavaScript file:

```html
<script src="hasher.js"></script>
```

After it's loaded you can access `Rust.hasher`, which is a [Promise] that
will be resolved once the WebAssembly module is loaded. Inside that promise
you'll find everything which you've marked with `#[js_export]`:

```html
<script>
    Rust.hasher.then( function( hasher ) {
        const string = "fiddlesticks";
        const hash = hasher.sha1( string );

        console.log( "Hash of " + string + " is '" + hash + "'" );
    });
</script>
```

You can also use the very same `hasher.js` from Nodejs:

```js
const hasher = require( "hasher.js" );

const string = "fiddlesticks";
const hash = hasher.sha1( string );

console.log( "Hash of " + string + " is '" + hash + "'" );
```

For the Nodejs environment the WebAssembly is compiled synchronously.

[Promise]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise

### Integration with JavaScript bundlers

You can take a look at the `hasher-parcel` example for how to integrate
with the [Parcel] bundler.

[Parcel]: https://parceljs.org/
