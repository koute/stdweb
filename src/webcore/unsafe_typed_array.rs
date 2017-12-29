use std::fmt;

/// A wrapper type for exposing raw Rust slices as `TypedArray`s
/// at zero cost without copying.
///
/// The only thing you can do with this is to pass it to the `js!` macro.
///
/// Using this is **highly unsafe**! After you pass it to the `js!` macro
/// you **must** use it **before** triggering any Rust code whatsoever,
/// either directly or indirectly. Breaking this rule will result
/// in undefined behavior!
///
/// # Examples
///
/// ```rust
/// let slice: &[u8] = &[1, 2, 3];
/// let slice = unsafe { UnsafeTypedArray::new( slice ) };
/// js!(
///     var slice = @{slice};
///     // `slice` is an Uint8Array
///     var sum = slice[0] + slice[1] + slice[2];
///     console.log( "Sum of array elements: ", sum );
/// );
/// ```
pub struct UnsafeTypedArray< 'a, T: 'a >( pub(crate) &'a [T] );

impl< 'a, T > fmt::Debug for UnsafeTypedArray< 'a, T > {
    #[inline]
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        write!( formatter, "UnsafeTypedArray" )
    }
}

impl< 'a, T > UnsafeTypedArray< 'a, T > {
    /// Creates a new `UnsafeTypedArray`.
    ///
    /// Even though this function is marked as `unsafe`
    /// the unsafely only comes into play after you
    /// pass it to the `js!` macro.
    #[inline]
    pub unsafe fn new( slice: &'a [T] ) -> Self {
        UnsafeTypedArray( slice )
    }
}
