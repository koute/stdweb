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

1. Install [cargo-web]:

       $ cargo install -f cargo-web

3. Go into `examples/todomvc` and start the example using one of these commands:

    * Compile to [WebAssembly] using Rust's native WebAssembly backend (requires Rust nightly!):

          $ cargo web start --target=wasm32-unknown-unknown

    * Compile to [asm.js] using Emscripten:

          $ cargo web start --target=asmjs-unknown-emscripten

    * Compile to [WebAssembly] using Emscripten:

          $ cargo web start --target=wasm32-unknown-emscripten

4. Visit `http://localhost:8000` with your browser.

For the `*-emscripten` targets `cargo-web` is not necessary, however
the native `wasm32-unknown-unknown` which doesn't need Emscripten
**requires** `cargo-web` to work!

[cargo-web]: https://github.com/koute/cargo-web
[asm.js]: https://en.wikipedia.org/wiki/Asm.js
[WebAssembly]: https://en.wikipedia.org/wiki/WebAssembly

## Exposing Rust functions to JavaScript

***WARNING***: This is only supported for Rust's native `wasm32-unknown-unknown` target
and requires Rust nightly!

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

If you compile this code with `cargo-web build --target=wasm32-unknown-unknown` you'll get two files:

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

## Parcel plugin

There is also an **experimental** [Parcel] plugin [here](https://github.com/koute/parcel-plugin-cargo-web).

[Parcel]: https://parceljs.org/

## Changelog

   * `0.4`
      * (breaking change) Renamed:
         * `KeydownEvent` -> `KeyDownEvent`
         * `KeyupEvent` -> `KeyUpEvent`
         * `KeypressEvent` -> `KeyPressEvent`
         * `ReadyState` -> `FileReaderReadyState`
      * (breaking change) Changed return types:
         * `INode::remove_child` now returns `Node` in the `Ok` case
         * The following now return a `Result`:
            * `INode::insert_before`
            * `INode::replace_child`
            * `INode::clone_node`
            * `StringMap::insert`
            * `TokenList::add`
            * `TokenList::remove`
            * `Document::create_element`
            * `IEventTarget::dispatch_event`
            * `FileReader::read_as_text`
            * `FileReader::read_as_array_buffer`
            * `FileReader::read_as_text`
            * `History::replace_state`
            * `History::go`
            * `History::back`
            * `History::forward`
            * `Location::href`
            * `Location::hash`
        * `INode::base_uri` now returns a `String` instead of `Option< String >`
      * (breaking change) `INode::inner_text` was moved to `IHtmlElement::inner_text`
      * (breaking change) `Document::query_selector` and `Document::query_selector_all` were moved to `IParentNode`
      * (breaking change) `IElement::query_selector` and `IElement::query_selector_all` were moved to `IParentNode`
      * When building using a recent `cargo-web` it's not necessary to call
        `stdweb::initialize` nor `stdweb::event_loop` anymore
      * Support for `cdylib` crates on `wasm32-unknown-unknown`
      * New bindings:
         * `XmlHttpRequest`
         * `WebSocket`
         * `MutationObserver`
         * `History`
         * `TextAreaElement`
         * `CanvasElement`
      * New event types:
         * `MouseDownEvent`
         * `MouseUpEvent`
         * `MouseMoveEvent`
         * `PopStateEvent`
         * `ResizeEvent`
         * `ReadyStateChange`
         * `SocketCloseEvent`
         * `SocketErrorEvent`
         * `SocketOpenEvent`
         * `SocketMessageEvent`
      * Initial support for the Canvas APIs
      * Add `#[js_export]` procedural attribute (`wasm32-unknown-unknown` only)
      * Add `DomException` and subtypes for passing around JavaScript exceptions
      * `IElement` now inherits from `INode`

   * `0.3`
      * (breaking change) Deleted `ErrorEvent` methods
      * (breaking change) Renamed:
         * `LoadEvent` -> `ResourceLoadEvent`
         * `AbortEvent` -> `ResourceAbortEvent`
         * `ErrorEvent` -> `ResourceErrorEvent`
      * Add `UnsafeTypedArray` for zero cost slice passing to `js!`
      * Add `Once` for passing `FnOnce` closures to `js!`

## License

Licensed under either of

  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Snippets of documentation which come from [Mozilla Developer Network] are covered under the [CC-BY-SA, version 2.5] or later.

[Mozilla Developer Network]: https://developer.mozilla.org/en-US/
[CC-BY-SA, version 2.5]: https://developer.mozilla.org/en-US/docs/MDN/About#Copyrights_and_licenses

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

You can run `stdweb`'s tests with `cargo web test --features web_test`, which will
run them under headless Chromium.
