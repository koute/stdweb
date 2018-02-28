# Contributing to Web APIs

For concrete Javascript types, define a struct as an `instance_of` the concrete Js type.

eg:
```rust
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "CanvasGradient")]
pub struct CanvasGradient(Reference);
```

Make sure to document the struct according to the documentation in MDN and provide a link.

eg:
```rust
/// The CanvasGradient struct represents an opaque object describing a gradient. 
/// It is returned by the methods CanvasRenderingContext2D.createLinearGradient() or 
/// CanvasRenderingContext2D.createRadialGradient().
/// 
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient)
```

Remember these are Rust docs so certain keywords such as `DOMString` and `Interface` need to be "translated" into Rust equivalents.

eg: 

    `DOMString` -> `String`/`Enum` (whichever is more appropriate)
    `Interface` -> `trait`
    
Also add a comment linking the actual HTML spec for that particular object

eg:

`// https://html.spec.whatwg.org/#canvasgradient`

When implementing properties and methods, follow the HTML spec.

A helpful resource for translating types from the HTML spec to Rust can be found in the `TypedArray objects` table [here](https://html.spec.whatwg.org/#canvasgradient).

Try to stay as close as possible to the original JS name while maintaining Rust naming conventions.

For functions that can't be overloaded properly with traits, define multiple functions with a suffix to specify their use. Try to find one "general" or "basic" function that can take the original non-suffixed name.

