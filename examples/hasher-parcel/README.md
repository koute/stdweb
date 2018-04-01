This example shows how to integrate Rust with the [Parcel] bundler.

Please see the `hasher` example first for how to export methods from Rust
so that they're callable on the JavaScript side.

[Parcel]: https://parceljs.org/

### Running the demo

1. Install the dependencies:

       $ npm install

2. Start the demo:

       $ $(npm bin)/parcel index.html

3. Visit `http://localhost:1234` with your browser.

### How does this work?

This uses our [Parcel plugin] to integrate `cargo-web` with Parcel.

If you have an existing Parcel project you can simply add our plugin:

    $ npm install --save parcel-plugin-cargo-web

and then just import the `Cargo.toml` of a Rust crate, where in this
case the whole code looks like this:

```js
import hasher from "../hasher/Cargo.toml";

var input = document.getElementById( "input" );
var output = document.getElementById( "output" );
output.innerText = hasher.sha1( input.value );

input.addEventListener( "keyup", function( event ) {
    output.innerText = hasher.sha1( input.value );
});
```

[Parcel plugin]: https://github.com/koute/parcel-plugin-cargo-web
