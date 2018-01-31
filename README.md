<p align="center">
    <img src="info/logo.png">
</p>

[![Build Status](https://api.travis-ci.org/koute/stdweb.svg)](https://travis-ci.org/koute/stdweb)
[![Join the chat at https://gitter.im/stdweb-rs/stdweb](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/stdweb-rs/stdweb?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

# A standard library for the client-side Web

[![Documentation](https://docs.rs/stdweb/badge.svg)](https://docs.rs/stdweb/*/stdweb/)

The goal of this crate is to provide Rust bindings to the Web APIs and to allow
a high degree of interoperability between Rust and JavaScript.

## Donate

[![Become a patron](https://koute.github.io/img/become_a_patron_button.png)](https://www.patreon.com/koute)

## Patrons

This software was brought to you thanks to these wonderful people:
  * Ben Berman

Thank you!

## Examples

You can directly embed JavaScript code into Rust:

```rust
let message = "Hello, 世界!";
let result = js! {
    alert( @{message} );
    return 2 + 2 * 2;
};

println!( "2 + 2 * 2 = {:?}", result );
```

Even closures are supported:

```rust
let print_hello = |name: String| {
    println!( "Hello, {}!", name );
};

js! {
    var print_hello = @{print_hello};
    print_hello( "Bob" );
    print_hello.drop(); // Necessary to clean up the closure on Rust's side.
}
```

You can also pass arbitrary structures thanks to [serde]:

```rust
#[derive(Serialize)]
struct Person {
    name: String,
    age: i32
}

js_serializable!( Person );

js! {
    var person = @{person};
    console.log( person.name + " is " + person.age + " years old." );
};
```

[serde]: https://serde.rs/

This crate also exposes a number of Web APIs, for example:

```rust
let button = document().query_selector( "#hide-button" ).unwrap();
button.add_event_listener( move |_: ClickEvent| {
    for anchor in document().query_selector_all( "#main a" ) {
        js!( @{anchor}.style = "display: none;"; );
    }
});
```

## Design goals

  * Expose a full suite of Web APIs as exposed by web browsers.
  * Try to follow the original JavaScript conventions and structure as much as possible,
    except in cases where doing otherwise results in a clearly superior design.
  * Be a building block from which higher level frameworks and libraries
    can be built.
  * Make it convenient and easy to embed JavaScript code directly into Rust
    and to marshal data between the two.
  * Integrate with the wider Rust ecosystem, e.g. support marshaling of structs
    which implement serde's Serializable.
  * Put Rust in the driver's seat where a non-trivial Web application can be
    written without touching JavaScript at all.
  * Allow Rust to take part in the upcoming WebAssembly (re)volution.
  * Make it possible to trivially create standalone libraries which are
    easily callable from JavaScript.

## Getting started

Take a look at some of the examples:

  * `examples/minimal` - a totally minimal example which calls [alert]
  * `examples/todomvc` - a naively implemented [TodoMVC] application; shows how to call into the DOM
  * `examples/hasher` - shows how to export Rust functions to JavaScript and how to call them from
                        the browser or Nodejs
  * [`pinky-web`] - an NES emulator; you can play with the [precompiled version here](http://koute.github.io/pinky-web/)

[alert]: https://developer.mozilla.org/en-US/docs/Web/API/Window/alert
[TodoMVC]: http://todomvc.com/
[`pinky-web`]: https://github.com/koute/pinky/tree/master/pinky-web

## Running the examples

1. Add one of Rust's web targets with [rustup].

    * For compiling to asmjs through Emscripten:

          $ rustup target add asmjs-unknown-emscripten

    * For compiling to WebAssembly through Emscripten:

          $ rustup target add wasm32-unknown-emscripten

    * For compiling to WebAssembly through Rust's native backend:

          $ rustup target add wasm32-unknown-unknown

2. Install [cargo-web]:

       $ cargo install -f cargo-web

3. Go into `examples/todomvc` and start the example.

    * For the `asmjs-unknown-emscripten` backend:

          $ cargo web start --target-asmjs-emscripten

    * For the `wasm32-unknown-emscripten`:

          $ cargo web start --target-webasm-emscripten

    * For the `wasm32-unknown-unknown`:

          $ cargo web start --target-webasm

4. Visit `http://localhost:8000` with your browser.

For the `*-emscripten` targets `cargo-web` is not necessary, however
the native `wasm32-unknown-unknown` which doesn't need Emscripten
**requires** `cargo-web` to work!

[cargo-web]: https://github.com/koute/cargo-web
[rustup]: https://www.rustup.rs/

## Exposing Rust functions to JavaScript

***WARNING***: This is only supported for Rust's native `wasm32-unknown-unknown` target!

(Note: this is based on the `examples/hasher` example)

With the `stdweb` crate you can easily expose a Rust function
to JavaScript like this:

```rust
#[macro_use]
extern crate stdweb;
extern crate sha1;

use sha1::Sha1;

fn hash( string: String ) -> String {
    let mut hasher = Sha1::new();
    hasher.update( string.as_bytes() );
    hasher.digest().to_string()
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.sha1 = @{hash};
    }
}
```

If you compile this code with `cargo-web build --target-webasm` you'll get two files:

   * `target/wasm32-unknown-unknown/release/hasher.js`
   * `target/wasm32-unknown-unknown/release/hasher.wasm`

You can copy them into your JavaScript project and load like any other JavaScript file:

```html
<script src="hasher.js"></script>
```

After it's loaded you can access `Rust.hasher`, which is a [Promise] which
will resolve once the WebAssembly module is loaded. Inside that promise
you'll find the contents of `Module.exports` which we've set from our
Rust code, which includes our exported function which you can now call:

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

## Breaking changes

   * `0.3`
      * Deleted `ErrorEvent` methods
      * Renamed:
         * `LoadEvent` -> `ResourceLoadEvent`
         * `AbortEvent` -> `ResourceAbortEvent`
         * `ErrorEvent` -> `ResourceErrorEvent`

## License

Licensed under either of

  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Snippets of documentation which come from [Mozilla Developer Network] are covered under the [CC-BY-SA, version 2.5] or later.

[Mozilla Developer Network]: https://developer.mozilla.org/en-US/
[CC-BY-SA, version 2.5]: https://developer.mozilla.org/en-US/docs/MDN/About#Copyrights_and_licenses

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
