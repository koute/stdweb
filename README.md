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
  * Embark Studios
  * Joe Narvaez
  * Eduard Knyshov
  * Anselm Eickhoff
  * Johan Andersson
  * Stephen Sugden
  * is8ac

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

Closures are also supported:

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
let button = document().query_selector( "#hide-button" ).unwrap().unwrap();
button.add_event_listener( move |_: ClickEvent| {
    for anchor in document().query_selector_all( "#main a" ) {
        js!( @{anchor}.style = "display: none;"; );
    }
});
```

Exposing Rust functions to JavaScript is supported too:

```rust
#[js_export]
fn hash( string: String ) -> String {
    let mut hasher = Sha1::new();
    hasher.update( string.as_bytes() );
    hasher.digest().to_string()
}
```

Then you can do this from Node.js:

```js
var hasher = require( "hasher.js" ); // Where `hasher.js` is generated from Rust code.
console.log( hasher.hash( "Hello world!" ) );
```

Or you can take the same `.js` file and use it in a web browser:

```html
<script src="hasher.js"></script>
<script>
    Rust.hasher.then( function( hasher ) {
        console.log( hasher.hash( "Hello world!" ) );
    });
</script>
```

If you're using [Parcel] you can also use our [experimental Parcel plugin];
first do this in your existing Parcel project:

    $ npm install --save parcel-plugin-cargo-web

And then simply:

```js
import hasher from "./hasher/Cargo.toml";
console.log( hasher.hash( "Hello world!" ) );
```

[Parcel]: https://parceljs.org/
[experimental Parcel plugin]: https://github.com/koute/parcel-plugin-cargo-web

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
                        a vanilla web browser environment or from Nodejs
  * `examples/hasher-parcel` - shows how to import and call exported Rust functions in a [Parcel] project
  * [`pinky-web`] - an NES emulator; you can play with the [precompiled version here](http://koute.github.io/pinky-web/)

[The API documentation](https://docs.rs/stdweb/*/stdweb/) is also available for you to look at.

[alert]: https://developer.mozilla.org/en-US/docs/Web/API/Window/alert
[TodoMVC]: http://todomvc.com/
[`pinky-web`]: https://github.com/koute/pinky/tree/master/pinky-web

## Running the examples

1. Install [cargo-web]:

       $ cargo install -f cargo-web

3. Go into `examples/todomvc` and start the example using one of these commands:

    * Compile to [WebAssembly] using Rust's native WebAssembly backend:

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

## Changelog
   * `stdweb 0.4.20`
      * Compatibility with the newest `wasm-bindgen`
      * New events:
         * `FullscreenChangeEvent`
   * `stdweb 0.4.19`
      * New methods:
         * `Document::fullscreen_enabled`
         * `Document::fullscreen_element`
         * `InputElement::selection_start`
         * `InputElement::selection_end`
         * `InputElement::set_selection_start`
         * `InputElement::set_selection_end`
         * `Object::to_iter`
         * `Window::confirm`
      * `&Array`s can now be converted to `Vec`s through `TryFrom`
      * The runtime should now be compatible with newer versions of Emscripten
      * The unstable `futures`-related APIs were updated to work with the latest nightlies
      * The `syn` dependency was updated to version 1
   * `stdweb 0.4.18`
      * The `js!` macro can now be imported with an `use`
      * New events:
         * `BeforeUnloadEvent`
         * `UnloadEvent`
      * New methods:
         * `IBlob::slice`
         * `IBlob::slice_with_content_type`
         * `IWindowOrWorker::set_clearable_timeout`
   * `stdweb 0.4.17`
      * The unstable `futures`-related APIs were updated to work with the latest nightlies
   * `stdweb 0.4.16`
      * Initial `wasm-bindgen` compatibility; you can now use `stdweb` in projects
        using `wasm-bindgen`
      * Minimum supported Rust version is now 1.33.0
      * Minimum required `cargo-web` version is now 0.6.24
   * `stdweb 0.4.15`
      * The unstable `futures`-related APIs were updated to work with the latest nightlies
      * New types:
         * `FormData`
         * `FormDataEntry`
         * `MouseButtonsState`
      * New methods:
         * `Blob::new`
   * `stdweb 0.4.14`
      * The `js!` macro now generates slightly more efficient code
        if you're not returning anything from your JS snippet. This makes
        it unnecessary to add the `@(no_return)` annotation in the vast
        majority of cases.
      * New types:
         * `File`
   * `stdweb 0.4.13`
      * Fixed the procedural `js!` macro's whitespace handling
      * New types:
         * `ITouchEvent`
         * `Touch`
         * `TouchType`
      * New events:
         * `TouchEvent`
         * `TouchMove`
         * `TouchLeave`
         * `TouchEnter`
         * `TouchEnd`
         * `TouchCancel`
         * `TouchStart`
      * New methods:
         * `XmlHttpRequest::set_response_type`
   * `stdweb 0.4.12`
      * Improved diagnostics when trying to compile for the `wasm32-unknown-unknown` target without `cargo-web`
   * `stdweb 0.4.11`
      * The minimum required Rust version is now 1.30.1
      * The minimum required `cargo-web` version is now 0.6.22
      * `wasm32-unknown-unknown` is now officially supported on stable Rust
      * Debug builds on `wasm32-unknown-unknown` are now supported
      * The `js!` macro is now partially implemented using a procedural macro
      * String decoding/encoding is now a lot faster due to the use of native codec APIs
      * New methods:
        * `Document::import_node`
        * `IElement::slot`
        * `IElement::attach_shadow`
        * `IElement::shadow_root`
      * New types:
        * `ISlotable`
        * `ShadowRoot`
        * `ShadowRootMode`
        * `TemplateElement`
        * `SlotElement`
      * New events:
        * `SlotChangeEvent`
      * `IParentNode::query_selector` and `IParentNode::query_selector_all` now return a proper error type
   * `stdweb 0.4.10`, `stdweb-derive 0.5.1`
      * New methods:
        * `IElement::insert_adjacent_html`
        * `IElement::insert_html_before`
        * `IElement::insert_html_after`
        * `IElement::prepend_html`
        * `IElement::append_html`
        * `IElement::namespace_uri`
        * `IElement::closest`
        * `Document::create_element_ns`
        * `Window::get_selection`
      * New types:
        * `AbortError`
        * `SelectionType`
        * `Selection`
        * `Range`
      * The error messages for failed type conversions are now improved
      * The error type of failed conversions (when using `.try_into()`/`.try_from()`) is now convertible into a `TypeError`
      * Aggregate error types (like, e.g. `DrawImageError`) are now serializable through the `js!` macro
      * `TypeError` is now fixed (it was incorrectly treated as a `DOMException`)
      * `Number` can now be converted into `f64` with `.into()`/`.from()`
      * Added `Mut`, which is a new wrapper type for safely passing `FnMut` closures into the `js!` macro;
        it is optional for now, however the usage of this wrapper type **will be mandatory** in the future!
      * `FnMut` closures cannot be called recursively anymore
      * `#[derive(ReferenceType)]` now supports a limited subset of generic types
      * Asynchronous unit tests are now supported with a new `#[async_test]` attribute macro (nightly only)
      * Updated to `futures 0.3` (nightly only)
   * `stdweb 0.4.9`, `stdweb-derive 0.5.0`
      * Performance improvements; serialization through serde is now twice as fast
      * New events:
        * `ScrollEvent`
        * `DragRelatedEvent`
        * `DragEvent`
        * `DragStartEvent`
        * `DragEndEvent`
        * `DragEnterEvent`
        * `DragLeaveEvent`
        * `DragOverEvent`
        * `DragExitEvent`
        * `DragDropEvent`
      * New types:
        * `DataTransfer`
        * `EffectAllowed`
        * `DropEffect`
        * `DataTransferItemList`
        * `DataTransferItem`
        * `DataTransferItemKind`
        * `IDragEvent`
      * `Value`s can now be converted to `Option< Serde< T > >` with `try_into`
      * Deserialization of numbers through serde now works in the majority of cases
        allowing types other than `i32` and `f64` to be used
      * All of the events are now more strongly-typed
          * Previously in was possible to deserialize e.g. a `keyup` event
            as a `KeyDownEvent` since only the event's JS type was checked
            and both `keyup` and `keydown` share the same JS type (`KeyboardEvent`).
            From now on the `type` field of the event is also checked, so
            such conversions are not allowed anymore.
   * `0.4.8`
      * Fixed compilation on the newest nightly when targeting `wasm32-unknown-unknown`
      * New events:
        * `PointerLockChangeEvent`
        * `PointerLockErrorEvent`
        * `MouseWheelEvent`
      * New types:
        * `MouseWheelDeltaMode`
        * `XhrResponseType`
      * New methods:
        * `XmlHttpRequest::raw_response`
        * `Window::device_pixel_ratio`
        * `Document::pointer_lock_element`
        * `Document::exit_pointer_lock`
   * `0.4.7`
      * New events:
         * `AuxClickEvent`
         * `MouseEnterEvent`
         * `MouseLeaveEvent`
         * `ContextMenuEvent`
         * `SelectionChangeEvent`
      * New types:
        * `FileList`
        * `FileReaderReadyState`
      * Implement gamepad APIs:
         * `Gamepad`
         * `GamepadButton`
         * `GamepadButtonMapping`
         * `GamepadEvent`
      * Fixed `CanvasRenderingContext2d::clear_rect`
      * Fixed a leak when creating `TypedArray`s from
        `Vec`s and `ArrayBuffer`s.
   * `0.4.6`
      * Fix `docs.rs` again
      * New types:
         * `SubmitEvent`
         * `IChildNode`
      * Fix `CanvasElement::to_data_url`
   * `0.4.5`
      * New types:
         * `DocumentFragment`
         * `SelectElement`
         * `OptionElement`
         * `HtmlCollection`
      * New methods:
         * `Node::from_html`
         * `Value::is_null`
      * Expose enums:
         * `SocketMessageData`
         * `NodeType`
      * Update to `futures` 0.2
   * `0.4.4`
      * Fix `docs.rs` (hopefully).
      * New methods:
         * `Location::origin`
         * `Location::protocol`
         * `Location::host`
         * `Location::hostname`
         * `Location::port`
         * `Location::pathname`
         * `Location::search`
      * These now return `SecurityError` in the error case:
         * `Location::hash`
         * `Location::href`
   * `0.4.3`
      * Objects which cannot be used as keys in a `WeakMap`
        should be supported now (e.g. some of the WebGL-related objects under Firefox)
      * New methods:
         * `Element::get_bounding_client_rect`
         * `Element::scroll_top`
         * `Element::scroll_left`
         * `Window::page_x_offset`
         * `Window::page_y_offset`
         * `NodeList::item`
         * `Document::body`
         * `Document::head`
         * `Document::title`
         * `Document::set_title`
         * `IMouseEvent::offset_x`
         * `IMouseEvent::offset_y`
      * Expose more canvas related types:
         * `CompositeOperation`
         * `LineCap`
         * `LineJoin`
         * `Repetition`
         * `TextAlign`
         * `TextBaseline`
      * Expose canvas related error types: `AddColorStopError`, `DrawImageError`, `GetImageDataError`
      * New events:
         * `MouseOverEvent`
         * `MouseOutEvent`
         * `PointerOverEvent`
         * `PointerEnterEvent`
         * `PointerDownEvent`
         * `PointerMoveEvent`
         * `PointerUpEvent`
         * `PointerCancelEvent`
         * `PointerOutEvent`
         * `PointerLeaveEvent`
         * `GotPointerCaptureEvent`
         * `LostPointerCaptureEvent`
      * New interface for pointer events: `IPointerEvent`
   * `0.4.2`
      * Fixed a leak when deserializing references
      * Fixed `CanvasRenderingContext2d::get_canvas`
      * Exposed `FillRule` and `SocketReadyState`
      * New attribute related methods added to `IElement`
      * New `Date` bindings
   * `0.4.1`
      * Support for newest nightly Rust on `wasm32-unknown-unknown`
      * Exposed `SocketBinaryType` enum
      * New canvas APIs:
         * Numerous new methods for `CanvasRenderingContext2d`
         * New types: `CanvasGradient`, `CanvasPattern`, `CanvasStyle`, `ImageData`, `TextMetrics`
      * New error types: `IndexSizeError`, `NotSupportedError`, `TypeError`
   * `0.4`
      * (breaking change) Removed `Array` and `Object` variants from `Value`; these are now treated as `Reference`s
      * (breaking change) The `Value` has an extra variant: `Symbol`
      * (breaking change) Removed:
         * `InputElement::set_kind`
         * `InputElement::files`
      * (breaking change) Renamed:
         * `KeydownEvent` -> `KeyDownEvent`
         * `KeyupEvent` -> `KeyUpEvent`
         * `KeypressEvent` -> `KeyPressEvent`
         * `ReadyState` -> `FileReaderReadyState`
         * `InputElement::value` -> `InputElement::raw_value`
         * `InputElement::set_value` -> `InputElement::set_raw_value`
      * (breaking change) `ArrayBuffer::new` now takes an `u64` argument
      * (breaking change) `InputElement::set_raw_value` now takes `&str` instead of `Into< Value >`
      * (breaking change) Changed return types:
         * Every method which returned `usize` now returns `u32`
         * `INode::remove_child` now returns `Node` in the `Ok` case
         * The following now return an `u64`:
            * `ArrayBuffer::len`
         * The following now return an `i32` instead of `f64`:
            * `IMouseEvent::client_x`
            * `IMouseEvent::client_y`
            * `IMouseEvent::movement_x`
            * `IMouseEvent::movement_y`
            * `IMouseEvent::screen_x`
            * `IMouseEvent::screen_y`
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
            * `CanvasElement::to_data_url`
            * `CanvasElement::to_blob`
            * `ArrayBuffer::new`
        * `INode::base_uri` now returns a `String` instead of `Option< String >`
        * `InputElement::raw_value` now returns a `String` instead of `Value`
      * (breaking change) `INode::inner_text` was moved to `IHtmlElement::inner_text`
      * (breaking change) `Document::query_selector` and `Document::query_selector_all` were moved to `IParentNode`
      * (breaking change) `IElement::query_selector` and `IElement::query_selector_all` were moved to `IParentNode`
      * (breaking change) `Document::get_element_by_id` was moved to `INonElementParentNode`
      * (breaking change) A blanket impl for converting between arbitrary reference-like objects using
        `TryFrom`/`TryInto` has been removed
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
      * New traits: `ReferenceType` and `InstanceOf`
      * Add `#[derive(ReferenceType)]` in `stdweb-derive` crate; it's now possible
        to define custom API bindings outside of `stdweb`
      * Add `#[js_export]` procedural attribute (`wasm32-unknown-unknown` only)
      * Add `DomException` and subtypes for passing around JavaScript exceptions
      * `IElement` now inherits from `INode`
      * Every interface now inherits from `ReferenceType`
      * Add `stdweb::traits` module to act as a prelude for `use`-ing all of our interface traits
      * Add `console!` macro
      * Most types now implement `PartialEq` and `Eq`

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

See [CONTRIBUTING.md](https://github.com/koute/stdweb/blob/master/CONTRIBUTING.md)
