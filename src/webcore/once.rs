use std::fmt;

/// A wrapper for passing `FnOnce` callbacks into the `js!` macro.
///
/// Since it only supports `FnOnce` callbacks there is no need to
/// `drop()` them manually on the JavaScript side provided they
/// were actually called.
///
/// You still need to `drop()` any callbacks which were **not** called.
///
/// # Examples
///
/// ```rust
/// let callback = || { println!( "Hello world!" ); };
/// js! {
///     var cb = @{Once(callback)};
///     cb();
///     // There is no need to drop it; since the function
///     // is only callable once it automatically drops
///     // itself after being called.
/// }
/// ```
pub struct Once< T >( pub T );

impl< T > fmt::Debug for Once< T > {
    #[inline]
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        write!( formatter, "Once" )
    }
}
