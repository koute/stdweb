macro_rules! next {
    (empty) => {};

    ((peel, $callback:tt, ($value:tt))) => {
        $callback!( empty => );
    };

    ((peel, $callback:tt, ($value:tt, $($other:tt),+))) => {
        $callback!( (peel, $callback, ($($other),+)) => $($other),+ );
    };
}

macro_rules! foreach {
    ($callback:tt => $($values:tt),*) => {
        $callback!( (peel, $callback, ($($values),*)) => $($values),* );
    };
}

macro_rules! loop_through_identifiers {
    ($callback:tt) => {
        foreach!( $callback => A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12 );
    };
}

macro_rules! em_asm_int {
    ($code:expr, $($arg:expr),*) => {
        {
            const CODE: &'static str = concat!( $code, "\0" );

            #[allow(unused_unsafe)]
            unsafe {
                use webcore::ffi;
                ffi::emscripten_asm_const_int(
                    CODE as *const _ as *const u8,
                    $($arg),*
                )
            }
        }
    };
    ($code:expr) => {
        {
            const CODE: &'static str = concat!( $code, "\0" );
            unsafe {
                use webcore::ffi;
                ffi::emscripten_asm_const_int(
                    CODE as *const _ as *const u8
                )
            }
        }
    };
}

macro_rules! em_asm_double {
    ($code:expr, $($arg:expr),*) => {
        {
            const CODE: &'static str = concat!( $code, "\0" );

            #[allow(unused_unsafe)]
            unsafe {
                use webcore::ffi;
                ffi::emscripten_asm_const_double(
                    CODE as *const _ as *const u8,
                    $($arg),*
                )
            }
        }
    };
    ($code:expr) => {
        {
            const CODE: &'static str = concat!( $code, "\0" );
            unsafe {
                use webcore::ffi;
                ffi::emscripten_asm_const_double(
                    CODE as *const _ as *const u8
                )
            }
        }
    };
}

// Abandon all hope, ye who enter here!
//
// If there was a contest for the ugliest and most hacky macro ever written,
// I would enter this one.
//
// There is probably a more clever way to write this macro, but oh well.
#[doc(hidden)]
#[macro_export]
macro_rules! _js_impl {
    (@_inc @_stringify "0" $($rest:tt)*) => { _js_impl!( @_stringify "1" $($rest)* ) };
    (@_inc @_stringify "1" $($rest:tt)*) => { _js_impl!( @_stringify "2" $($rest)* ) };
    (@_inc @_stringify "2" $($rest:tt)*) => { _js_impl!( @_stringify "3" $($rest)* ) };
    (@_inc @_stringify "3" $($rest:tt)*) => { _js_impl!( @_stringify "4" $($rest)* ) };
    (@_inc @_stringify "4" $($rest:tt)*) => { _js_impl!( @_stringify "5" $($rest)* ) };
    (@_inc @_stringify "5" $($rest:tt)*) => { _js_impl!( @_stringify "6" $($rest)* ) };
    (@_inc @_stringify "6" $($rest:tt)*) => { _js_impl!( @_stringify "7" $($rest)* ) };
    (@_inc @_stringify "7" $($rest:tt)*) => { _js_impl!( @_stringify "8" $($rest)* ) };
    (@_inc @_stringify "8" $($rest:tt)*) => { _js_impl!( @_stringify "9" $($rest)* ) };
    (@_inc @_stringify "9" $($rest:tt)*) => { _js_impl!( @_stringify "10" $($rest)* ) };

    (@_stringify $arg_counter:tt [$terminator:tt $($terminator_rest:tt)*] [$($out:tt)*] -> [] $next:tt $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator_rest)*] [$($out)* ($terminator)] -> $next $($rest)* )
    };

    (@_stringify $arg_counter:tt [] [$($out:tt)*] -> []) => {
        concat!( $(concat! $out),* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [($($inner:tt)*) $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [")" $($terminator)*] [$($out)* ("(")] -> [$($inner)*] [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [[$($inner:tt)*] $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter ["]" $($terminator)*] [$($out)* ("[")] -> [$($inner)*] [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [{$($inner:tt)*} $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter ["}" $($terminator)*] [$($out)* ("{")] -> [$($inner)*] [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [@{$arg:expr} $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_inc @_stringify $arg_counter [$($terminator)*] [$($out)* ("Module.STDWEB.to_js($") ($arg_counter) (")")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [++ $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* ("++")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [-- $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* ("--")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [=== $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* ("===")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [!== $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* ("!==")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [$token:tt . $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* (stringify!( $token )) (".")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [$token:tt] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* (stringify!( $token ))] -> [] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [$token:tt $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* (stringify!( $token )) (" ")] -> [$($remaining)*] $($rest)* )
    };

    (@stringify [$($flags:tt)*] -> $($rest:tt)*) => {
        _js_impl!( @if no_return in [$($flags)*] {
            _js_impl!( @_stringify "0" [] [] -> [$($rest)*] )
        } else {
            _js_impl!( @_stringify "1" [] [] -> [$($rest)*] )
        })
    };

    (@if no_return in [no_return $($rest:tt)*] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        $($true_case)*
    };

    (@if $condition:tt in [] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        $($false_case)*
    };

    (@if $condition:tt in [$token:tt $($rest:tt)*] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        _js_impl!( @if $condition in [$($rest)*] {$($true_case)*} else {$($false_case)*} );
    };

    (@prepare $memory_required:ident [] [$($names:tt)*]) => {};
    (@prepare $memory_required:ident [$arg:tt $($rest_args:tt)*] [$name:tt $($rest_names:tt)*]) => {
        let $name = $arg;
        let $name = $crate::private::IntoNewtype::into_newtype( $name );
        $memory_required += $crate::private::JsSerializableOwned::memory_required_owned( &$name );
        _js_impl!( @prepare $memory_required [$($rest_args)*] [$($rest_names)*] );
    };

    (@serialize $arena:ident [] [$($names:tt)*]) => {};
    (@serialize $arena:ident [$arg:tt $($rest_args:tt)*] [$name:tt $($rest_names:tt)*]) => {
        let mut $name = Some( $name );
        let $name = $crate::private::JsSerializableOwned::into_js_owned( &mut $name, &$arena );
        let $name = &$name as *const _;
        _js_impl!( @serialize $arena [$($rest_args)*] [$($rest_names)*] );
    };

    (@call_emscripten [$a0:tt] [$a0_name:tt $($arg_names:tt)*]) => {
        $crate::private::emscripten_asm_const_int( $a0_name );
    };

    (@call_emscripten [$a0:tt $a1:tt] [$a0_name:tt $a1_name:tt $($arg_names:tt)*]) => {
        $crate::private::emscripten_asm_const_int( $a0_name, $a1_name );
    };

    (@call_emscripten [$a0:tt $a1:tt $a2:tt] [$a0_name:tt $a1_name:tt $a2_name:tt $($arg_names:tt)*]) => {
        $crate::private::emscripten_asm_const_int( $a0_name, $a1_name, $a2_name );
    };

    (@call_emscripten [$a0:tt $a1:tt $a2:tt $a3:tt] [$a0_name:tt $a1_name:tt $a2_name:tt $a3_name:tt $($arg_names:tt)*]) => {
        $crate::private::emscripten_asm_const_int( $a0_name, $a1_name, $a2_name, $a3_name );
    };

    (@call_emscripten [$a0:tt $a1:tt $a2:tt $a3:tt $a4:tt] [$a0_name:tt $a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $($arg_names:tt)*]) => {
        $crate::private::emscripten_asm_const_int( $a0_name, $a1_name, $a2_name, $a3_name, $a4_name );
    };

    (@call_emscripten [$a0:tt $a1:tt $a2:tt $a3:tt $a4:tt $a5:tt] [$a0_name:tt $a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $($arg_names:tt)*]) => {
        $crate::private::emscripten_asm_const_int( $a0_name, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] ->) => {
        // It'd be nice to put at least some of this inside a function inside the crate,
        // but then it wouldn't work (I tried!) as the string with the code wouldn't be
        // passed as a direct reference to a constant, and Emscripten needs that to actually
        // use the JavaScript code we're passing to it.
        {
            if cfg!( test ) {
                $crate::initialize();
            }

            const CODE_STR: &'static str = _js_impl!(
                @if no_return in [$($flags)*] {
                    concat!( $code, "\0" )
                } else {
                    concat!( "Module.STDWEB.from_js($0, (function(){", $code, "})());\0" )
                }
            );

            #[allow(dead_code)]
            const CODE: *const u8 = CODE_STR as *const _ as *const u8;

            let mut memory_required = 0;
            _js_impl!( @prepare memory_required [$($args)*] [a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10] );

            #[allow(unused_variables)]
            let arena = $crate::private::PreallocatedArena::new( memory_required );

            _js_impl!( @serialize arena [$($args)*] [a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10] );
            arena.assert_no_free_space_left();

            $crate::private::noop( &mut memory_required );

            #[allow(unused_unsafe)]
            unsafe {
                _js_impl!(
                    @if no_return in [$($flags)*] {
                        _js_impl!( @call_emscripten [CODE $($args)*] [CODE a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10] );
                    } else {{
                        let mut result: $crate::private::SerializedValue = Default::default();
                        _js_impl!( @call_emscripten [CODE RESULT $($args)*] [CODE (&mut result as *mut _) a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10] );
                        result.deserialize()
                    }}
                )
            }
        }
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> { $($inner:tt)* } $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)*] -> $($inner)* $($rest)* );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> ( $($inner:tt)* ) $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)*] -> $($inner)* $($rest)* );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> [ $($inner:tt)* ] $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)*] -> $($inner)* $($rest)* );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> @{$arg:expr} $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)* $arg] -> $($rest)* );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> $token:tt $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)*] -> $($rest)* );
    };
}

/// Embeds JavaScript code into your Rust program.
///
/// This macro supports normal JavaScript syntax, albeit with a few limitations:
///
///   * String literals delimited with `'` are not supported.
///   * Semicolons are always required.
///   * The macro will hit the default recursion limit pretty fast, so you'll
///     probably want to increase it with `#![recursion_limit="500"]`.
///     (This is planned to be fixed once procedural macros land in stable Rust.)
///   * Any callbacks passed into JavaScript will **leak memory** by default!
///     You need to call `.drop()` on the callback from the JavaScript side to free it.
///
/// You can pass Rust expressions into the JavaScript code with `@{...expr...}`.
/// The value returned by this macro is an instance of [Value](enum.Value.html).
///
/// # Examples
///
/// ```
/// let name = "Bob";
/// let result = js! {
///     console.log( "Hello " + @{name} + "!" );
///     return 2 + 2;
/// };
///
/// println!( "2 + 2 = {:?}", result );
/// ```
#[macro_export]
macro_rules! js {
    (@($($flags:tt),*) $($token:tt)*) => {
        _js_impl!( @call [_js_impl!( @stringify [$($flags)*] -> $($token)* ), [$($flags)*]] [] -> $($token)* )
    };

    ($($token:tt)*) => {
        _js_impl!( @call [_js_impl!( @stringify [] -> $($token)* ), []] [] -> $($token)* )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __js_serializable_boilerplate {
    ($kind:tt) => {
        __js_serializable_boilerplate!( () ($kind) () );
    };

    (impl< $($impl_arg:tt),* > for $kind:ty where $($bounds:tt)*) => {
        __js_serializable_boilerplate!( ($($impl_arg),*) ($kind) ($($bounds)*) );
    };

    (impl< $($impl_arg:tt),* > for $kind:ty) => {
        __js_serializable_boilerplate!( ($($impl_arg),*) ($kind) () );
    };

    (($($impl_arg:tt)*) ($($kind_arg:tt)*) ($($bounds:tt)*)) => {
        impl< $($impl_arg)* > $crate::private::JsSerializableOwned for $crate::private::Newtype< (), $($kind_arg)* > where $($bounds)* {
            #[inline]
            fn into_js_owned< 'x >( value: &'x mut Option< Self >, arena: &'x $crate::private::PreallocatedArena ) -> $crate::private::SerializedValue< 'x > {
                $crate::private::JsSerializable::into_js( value.as_ref().unwrap().as_ref(), arena )
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                $crate::private::JsSerializable::memory_required( &**self )
            }
        }

        impl< '_r, $($impl_arg)* > $crate::private::JsSerializableOwned for $crate::private::Newtype< (), &'_r $($kind_arg)* > where $($bounds)* {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self >, arena: &'_a $crate::private::PreallocatedArena ) -> $crate::private::SerializedValue< '_a > {
                $crate::private::JsSerializable::into_js( value.as_ref().unwrap().as_ref(), arena )
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                $crate::private::JsSerializable::memory_required( &**self )
            }
        }

        impl< $($impl_arg)* > $crate::private::JsSerializableOwned for $($kind_arg)* where $($bounds)* {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self >, arena: &'_a $crate::private::PreallocatedArena ) -> $crate::private::SerializedValue< '_a > {
                $crate::private::JsSerializable::into_js( value.as_ref().unwrap(), arena )
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                $crate::private::JsSerializable::memory_required( self )
            }
        }

        impl< '_r, $($impl_arg)* > $crate::private::JsSerializableOwned for &'_r $($kind_arg)* where $($bounds)* {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self >, arena: &'_a $crate::private::PreallocatedArena ) -> $crate::private::SerializedValue< '_a > {
                $crate::private::JsSerializable::into_js( value.unwrap(), arena )
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                $crate::private::JsSerializable::memory_required( *self )
            }
        }
    };
}

macro_rules! __reference_boilerplate {
    ($kind:ident, instanceof $js_name:ident $($rest:tt)*) => {
        impl $crate::private::FromReference for $kind {
            #[inline]
            fn from_reference( reference: Reference ) -> Option< Self > {
                if instanceof!( reference, $js_name ) {
                    Some( $kind( reference ) )
                } else {
                    None
                }
            }
        }

        __reference_boilerplate!( $kind, $($rest)* );
    };

    ($kind:ident, convertible to $base_kind:ident $($rest:tt)*) => {
        impl From< $kind > for $base_kind {
            #[inline]
            fn from( value: $kind ) -> Self {
                use $crate::private::FromReferenceUnchecked;
                let reference: $crate::Reference = value.into();
                unsafe {
                    $base_kind::from_reference_unchecked( reference )
                }
            }
        }

        __reference_boilerplate!( $kind, $($rest)* );
    };

    ($kind:ident,) => {
        impl ::std::fmt::Debug for $kind {
            fn fmt( &self, formatter: &mut ::std::fmt::Formatter ) -> ::std::fmt::Result {
                write!( formatter, concat!( "<", stringify!( $kind ), ":{}>" ), self.0.as_raw() )
            }
        }

        impl Clone for $kind {
            #[inline]
            fn clone( &self ) -> Self {
                $kind( self.0.clone() )
            }
        }

        impl AsRef< $crate::Reference > for $kind {
            #[inline]
            fn as_ref( &self ) -> &$crate::Reference {
                &self.0
            }
        }

        impl $crate::private::FromReferenceUnchecked for $kind {
            #[inline]
            unsafe fn from_reference_unchecked( reference: $crate::Reference ) -> Self {
                $kind( reference )
            }
        }

        impl From< $kind > for $crate::Reference {
            #[inline]
            fn from( value: $kind ) -> Self {
                value.0
            }
        }

        impl $crate::unstable::TryFrom< $kind > for $crate::Reference {
            type Error = $crate::unstable::Void;

            #[inline]
            fn try_from( value: $kind ) -> Result< Self, Self::Error > {
                Ok( value.0 )
            }
        }

        impl< T: $crate::unstable::TryInto< $crate::Reference > > $crate::unstable::TryFrom< T > for $kind
            where <T as $crate::unstable::TryInto< $crate::Reference >>::Error: Into< Box< ::std::error::Error > >
        {
            type Error = Box< ::std::error::Error >; // TODO

            #[inline]
            fn try_from( value: T ) -> Result< Self, Self::Error > {
                value.try_into()
                    .map_err( |error| error.into() )
                    .and_then( |reference: Reference| reference.downcast().ok_or_else( || "reference is of a different type".into() ) )
            }
        }

        impl $crate::private::JsSerializable for $kind {
            #[inline]
            fn into_js< 'a >( &'a self, arena: &'a $crate::private::PreallocatedArena ) -> $crate::private::SerializedValue< 'a > {
                self.0.into_js( arena )
            }

            #[inline]
            fn memory_required( &self ) -> usize {
                Reference::memory_required( &self.0 )
            }
        }

        __js_serializable_boilerplate!( $kind );
    };
}

macro_rules! reference_boilerplate {
    ($kind:ident, $($rest:tt)*) => {
        __reference_boilerplate!( $kind, $($rest)* );
    }
}

macro_rules! instanceof {
    ($value:expr, $kind:ident) => {{
        use $crate::unstable::TryInto;
        let reference: Option< &$crate::Reference > = (&$value).try_into().ok();
        reference.map( |reference| {
            em_asm_int!(
                concat!( "return (Module.STDWEB.acquire_js_reference( $0 ) instanceof ", stringify!( $kind ), ") | 0;" ),
                reference.as_raw()
            ) == 1
        }).unwrap_or( false )
    }};
}

#[cfg(test)]
mod tests {
    macro_rules! stringify_js {
        ($($token:tt)*) => {
            _js_impl!( @stringify [] -> $($token)* )
        };
    }

    #[test]
    fn stringify() {
        assert_eq!( stringify_js! { console.log }, "console.log" );
        assert_eq!( stringify_js! { 1.0 }, "1.0" );
        assert_eq!( stringify_js! { [ 1.0 ] }, "[1.0]" );
        assert_eq!( stringify_js! { { 1.0 } }, "{1.0}" );
        assert_eq!( stringify_js! { ( 1.0 ) }, "(1.0)" );
        assert_eq!( stringify_js! { a b }, "a b" );
        assert_eq!( stringify_js! { === }, "===" );
        assert_eq!( stringify_js! { ++i }, "++i" );
        assert_eq!( stringify_js! { i++ }, "i ++" );
        assert_eq!( stringify_js! { --i }, "--i" );
        assert_eq!( stringify_js! { i-- }, "i --" );
        assert_eq!( stringify_js! { ( @{1} ); }.replace( " ", "" ), "(Module.STDWEB.to_js($1));" );
        assert_eq!(
            stringify_js! {
                console.log( "Hello!", @{1234i32} );
            }.replace( " ", "" ),
            "console.log(\"Hello!\",Module.STDWEB.to_js($1));"
        );
        assert_eq!(
            stringify_js! {
                @{a}.fn( @{b} );
            }.replace( " ", "" ),
            "Module.STDWEB.to_js($1).fn(Module.STDWEB.to_js($2));"
        );
    }
}
