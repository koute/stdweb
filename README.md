<p align="center">
    <img src="info/logo.png">
</p>

[![Build Status](https://api.travis-ci.org/koute/stdweb.svg)](https://travis-ci.org/koute/stdweb)
[![Join the chat at https://gitter.im/stdweb-rs/stdweb](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/stdweb-rs/stdweb?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

# A standard library for the client-side Web

[![Documentation](https://docs.rs/stdweb/badge.svg)](https://docs.rs/stdweb/*/stdweb/)

The goal of this crate is to provide Rust bindings to the Web APIs and to allow
a high degree of interoperability between Rust and JavaScript.

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

## Getting started

WARNING: This crate is still a work-in-progress. Things might not work.
Things might break. The APIs are in flux. Please do not use it in production.

1. Add an asmjs target with rustup:

       $ rustup target add asmjs-unknown-emscripten

2. Install [cargo-web]; it's not strictly necessary but it makes things
   more convenient:

       $ cargo install cargo-web

3. Go into `examples/todomvc` and type:

       $ cargo web start

4. Visit `http://localhost:8000` with your browser.

[cargo-web]: https://github.com/koute/cargo-web

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
