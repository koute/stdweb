# Derive macros for the `stdweb` crate

This crate currently defines a derive macro for [stdweb]
which allows you to define custom reference types outside
of `stdweb`.

For example:

```rust
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Error")]
pub struct Error( Reference );

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TypeError")]
#[reference(subclass_of(Error))]
pub struct TypeError( Reference );
```

And then you can do:

```rust
// You can use `try_into` to cast a `Value` to your type.
let error: TypeError = js!( return new TypeError(); ).try_into().unwrap();

// You can also pass your type freely into the `js!` macro:
js!( console.log( @{error} ); );
```

[stdweb]: https://github.com/koute/stdweb
