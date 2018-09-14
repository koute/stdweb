use std::fmt;

/// A wrapper for passing `FnMut` callbacks into the `js!` macro.
/// 
/// Just like when passing regular `Fn` callbacks, don't forget
/// to `drop()` them on the JavaScript side or else the closure
/// will be leaked.
///
/// # Examples
///
/// ```rust
/// let mut count = 0;
/// let callback = move || {
///     count += 1;
///     println!( "Callback called {} times", count );
/// };
/// js! {
///     var cb = @{Mut(callback)};
///     cb();
///     cb();
///     cb();
///     cb.drop();
/// }
/// ```
pub struct Mut< T >( pub T );

impl< T > fmt::Debug for Mut< T > {
    #[inline]
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        write!( formatter, "Mut" )
    }
}
