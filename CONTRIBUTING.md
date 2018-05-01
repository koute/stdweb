Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# Contributing to Web APIs

1. **When implementing properties and methods, follow the HTML spec**

    * A helpful resource for translating types from the HTML spec to Rust can be found in the `TypedArray objects` table [here](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray#TypedArray_objects)

    * Try to stay as close as possible to the original JS name while maintaining Rust naming conventions

2. **You can run `stdweb`'s tests with `cargo web test --features web_test`**
    
     This will run them under headless Chromium

3. **For concrete Javascript types, define a struct as an `instance_of` the concrete Js type**

    eg:
    ```rust
    #[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
    #[reference(instance_of = "CanvasGradient")]
    pub struct CanvasGradient(Reference);
    ```

4. **Make sure to document the struct according to the documentation in MDN and provide a link**

    eg:
    ```rust
    /// The CanvasGradient struct represents an opaque object describing a gradient. 
    /// It is returned by the methods CanvasRenderingContext2D.createLinearGradient() or 
    /// CanvasRenderingContext2D.createRadialGradient().
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient)
    ```

    Remember these are Rust docs so certain keywords such as `DOMString` and `Interface` need to be "translated" into Rust equivalents

    eg: 

        `DOMString` -> `String`/`Enum` (whichever is more appropriate)
        `Interface` -> `trait`
        
    Also add a comment linking the actual HTML spec for that particular object

    eg:

    `// https://html.spec.whatwg.org/#canvasgradient`


5. **For functions that can't be overloaded properly with traits, define multiple functions with a suffix to specify their use**
    
     Try to find one "general" or "basic" function that can take the original non-suffixed name

6. **You can export structs and enums by adding them to [lib.rs](https://github.com/koute/stdweb/blob/master/src/lib.rs)**
