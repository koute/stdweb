use webcore::try_from::TryInto;
use webcore::value::Value;

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

#[doc(hidden)]
#[macro_export]
macro_rules! __js_raw_asm {
    ($code:expr, $($token:expr),*) => {{
        #[$crate::private::js_raw_attr]
        fn snippet() {
            call!( $code, $($token),* );
        }

        snippet( $($token as *const u8),* )
    }};

    ($code:expr) => { $crate::__js_raw_asm!( $code, ) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __js_raw_asm_int {
    ($code:expr, $($token:expr),*) => {{
        #[$crate::private::js_raw_attr]
        fn snippet() -> i32 {
            call!( $code, $($token),* );
        }

        snippet( $($token as *const u8),* )
    }};

    ($code:expr) => { $crate::__js_raw_asm_int!( $code, ) };
}

// TODO: This should be handled inside of the procedural macro.
#[cfg(not(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web))))]
#[doc(hidden)]
#[macro_export]
macro_rules! __js_raw_asm_bool {
    ($code:expr, $($token:expr),*) => {{
        #[$crate::private::js_raw_attr]
        fn snippet() -> i32 {
            call!( $code, $($token),* );
        }

        snippet( $($token as *const u8),* )
    } == 1};

    ($code:expr) => { $crate::__js_raw_asm_bool!( $code, ) };
}

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))]
#[doc(hidden)]
#[macro_export]
macro_rules! __js_raw_asm_bool {
    ($code:expr, $($token:expr),*) => {{
        #[$crate::private::js_raw_attr]
        fn snippet() -> bool {
            call!( $code, $($token),* );
        }

        snippet( $($token as *const u8),* )
    }};

    ($code:expr) => { $crate::__js_raw_asm_bool!( $code, ) };
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
    (@if no_return in [no_return $($rest:tt)*] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        $($true_case)*
    };

    (@if $condition:tt in [] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        $($false_case)*
    };

    (@if $condition:tt in [$token:tt $($rest:tt)*] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        $crate::_js_impl!( @if $condition in [$($rest)*] {$($true_case)*} else {$($false_case)*} );
    };

    (@serialize [] [$($names:tt)*]) => {};
    (@serialize [$arg:tt $($rest_args:tt)*] [$name:tt $($rest_names:tt)*]) => {
        let $name = $arg;
        let $name = $crate::private::IntoNewtype::into_newtype( $name );
        let mut $name = Some( $name );
        let $name = $crate::private::JsSerializeOwned::into_js_owned( &mut $name );
        let $name = &$name as *const $crate::private::SerializedValue as *const _;
        $crate::_js_impl!( @serialize [$($rest_args)*] [$($rest_names)*] );
    };

    (@call [$($code:tt)*] [$($flags:tt)*] [$($args:tt)*] [$($arg_names:tt)*] [$($unused_arg_names:tt)*] ->) => {
        // It'd be nice to put at least some of this inside a function inside the crate,
        // but then it wouldn't work (I tried!) as the string with the code wouldn't be
        // passed as a direct reference to a constant, and Emscripten needs that to actually
        // use the JavaScript code we're passing to it.
        {
            if cfg!( test ) {
                $crate::initialize();
            }

            let restore_point = $crate::private::ArenaRestorePoint::new();
            $crate::_js_impl!( @serialize [$($args)*] [$($arg_names)*] );

            #[allow(unused_unsafe, unused_parens)]
            let result = unsafe {
                $crate::_js_impl!(
                    @if no_return in [$($flags)*] {{
                        #[$crate::private::js_no_return_attr]
                        fn snippet() {
                            call!( $($code)* );
                        }

                        snippet( $($arg_names),* );
                    }} else {{
                        let mut result: $crate::private::SerializedValue = std::default::Default::default();
                        let result_ptr = &mut result as *mut $crate::private::SerializedValue as *mut _;

                        #[$crate::private::js_attr]
                        fn snippet() {
                            call!( $($code)* );
                        }

                        snippet( result_ptr, $($arg_names),* );

                        result.deserialize()
                    }}
                )
            };

            std::mem::drop( restore_point );
            result
        }
    };

    (@call [$($code:tt)*] [$($flags:tt)*] [$($args:tt)*] [$($arg_names:tt)*] [$($unused_arg_names:tt)*] -> { $($inner:tt)* } $($rest:tt)*) => {
        $crate::_js_impl!( @call [$($code)*] [$($flags)*] [$($args)*] [$($arg_names)*] [$($unused_arg_names)*] -> $($inner)* $($rest)* );
    };

    (@call [$($code:tt)*] [$($flags:tt)*] [$($args:tt)*] [$($arg_names:tt)*] [$($unused_arg_names:tt)*] -> ( $($inner:tt)* ) $($rest:tt)*) => {
        $crate::_js_impl!( @call [$($code)*] [$($flags)*] [$($args)*] [$($arg_names)*] [$($unused_arg_names)*] -> $($inner)* $($rest)* );
    };

    (@call [$($code:tt)*] [$($flags:tt)*] [$($args:tt)*] [$($arg_names:tt)*] [$($unused_arg_names:tt)*] -> [ $($inner:tt)* ] $($rest:tt)*) => {
        $crate::_js_impl!( @call [$($code)*] [$($flags)*] [$($args)*] [$($arg_names)*] [$($unused_arg_names)*] -> $($inner)* $($rest)* );
    };

    (@call [$($code:tt)*] [$($flags:tt)*] [$($args:tt)*] [$($arg_names:tt)*] [$arg_name:tt $($unused_arg_names:tt)*] -> @{$arg:expr} $($rest:tt)*) => {
        $crate::_js_impl!( @call [$($code)*] [$($flags)*] [$($args)* $arg] [$($arg_names)* $arg_name] [$($unused_arg_names)*] -> $($rest)* );
    };

    (@call [$($code:tt)*] [$($flags:tt)*] [$($args:tt)*] [$($arg_names:tt)*] [$($unused_arg_names:tt)*] -> $token:tt $($rest:tt)*) => {
        $crate::_js_impl!( @call [$($code)*] [$($flags)*] [$($args)*] [$($arg_names)*] [$($unused_arg_names)*] -> $($rest)* );
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
/// ## Regular Usage
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
///
/// Note: you **must** include the `return ...;` statement to get a value.
///
/// ## No Return
///
/// If you don't need to return a value from your snippet you can add a @(no_return) attribute to
/// slightly improve performance.
///
/// ```
/// let name = "Bob";
/// js! { @(no_return)
///     console.log( "Hello " + @{name} + "!" );
/// };
/// ```
#[macro_export]
macro_rules! js {
    (@($($flags:tt),*) $($token:tt)*) => {
        $crate::_js_impl!( @call [$($token)*] [$($flags)*] [] [] [a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15] -> $($token)* )
    };

    ($($token:tt)*) => {
        js! { @() $($token)* }
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
        impl< $($impl_arg)* > $crate::private::JsSerializeOwned for $($kind_arg)* where $($bounds)* {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self > ) -> $crate::private::SerializedValue< '_a > {
                $crate::private::JsSerialize::_into_js( value.as_ref().unwrap() )
            }
        }

        impl< '_r, $($impl_arg)* > $crate::private::JsSerializeOwned for &'_r $($kind_arg)* where $($bounds)* {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self > ) -> $crate::private::SerializedValue< '_a > {
                $crate::private::JsSerialize::_into_js( value.unwrap() )
            }
        }
    };
}

macro_rules! error_boilerplate {
    ($type_name:ident) => {
        impl std::fmt::Display for $type_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "{}: {}", stringify!($type_name), self.message())
            }
        }

        impl std::error::Error for $type_name {
            fn description(&self) -> &str {
                stringify!($type_name)
            }
        }
    };

    ($type_name:ident, dom_exception = $error_name:expr) => {
        impl ::InstanceOf for $type_name {
            #[inline]
            fn instance_of( reference: &Reference ) -> bool {
                $crate::__js_raw_asm_bool!(
                    concat!(
                        "var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );",
                        "return (r instanceof DOMException) && (r.name === \"", $error_name, "\");"
                    ),
                    reference.as_raw()
                )
            }
        }

        error_boilerplate!( $type_name );
    };
}

macro_rules! instanceof {
    ($value:expr, $kind:ident) => {{
        use $crate::unstable::TryInto;
        let reference: Option< &$crate::Reference > = (&$value).try_into().ok();
        reference.map( |reference| {
            $crate::__js_raw_asm_int!(
                concat!( "return (Module.STDWEB_PRIVATE.acquire_js_reference( $0 ) instanceof ", stringify!( $kind ), ") | 0;" ),
                reference.as_raw()
            ) == 1
        }).unwrap_or( false )
    }};
}

macro_rules! newtype_enum {
    ($name:ident {
        $(
            $(#[$attr:meta])*
            $variant:ident = $value:expr
        ),* $(,)*
    }) => {
        impl $name {
            $(
                $(#[$attr])*
                pub const $variant: $name = $name($value);
            )*
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self.0 {
                    $($value => write!(formatter, "{}::{}", stringify!($name), stringify!($variant)),)*
                    other => write!(formatter, "{}({})", stringify!($name), other)
                }
            }
        }
    }
}

// This helps with type inference and converts the outer error
// type when the `TryInto`'s error type in the `js_try!`'s
// success and error cases differ.
#[inline]
pub fn js_try_convert< T, E, P >( value: Value ) -> Result< Result< T, E >, P >
    where Value: TryInto< T >, <Value as TryInto< T >>::Error: Into< P >
{
    match value.try_into() {
        Ok( value ) => Ok( Ok( value ) ),
        Err( error ) => Err( error.into() )
    }
}

/// Embeds JavaScript code into your Rust program similar to the `js!` macro, but
/// catches errors that may be thrown.
///
/// This macro will attempt to coerce the value into the inferred `Result` type.
/// The success and error types should implement `TryFrom<Value>`.
///
/// # Examples
///
/// ```
/// let result: Result<i32, String> = js_try! {
///     throw "error";
/// }.unwrap();
/// assert_eq!(result, Err("error".to_string()));
/// ```
macro_rules! js_try {
    (@(no_return) $($token:tt)*) => {{
        let result = js! {
            try {
                $($token)*
                return {
                    success: true
                };
            } catch( error ) {
                return {
                    error: error,
                    success: false
                };
            }
        };

        use ::webcore::try_from::TryInto;
        if js!( return @{result.as_ref()}.success; ) == true {
            Ok(Ok(()))
        } else {
            match js!( return @{result}.error; ).try_into() {
                Ok(e) => Ok(Err(e)),
                Err(e) => Err(e),
            }
        }
    }};

    ($($token:tt)*) => {{
        let result = js! {
            try {
                return {
                    value: function() { $($token)* }(),
                    success: true
                };
            } catch( error ) {
                return {
                    error: error,
                    success: false
                };
            }
        };

        use webcore::try_from::TryInto;
        if js!( return @{result.as_ref()}.success; ) == true {
            ::webcore::macros::js_try_convert( js!( return @{result}.value; ) )
        } else {
            match js!( return @{result}.error; ).try_into() {
                Ok(e) => Ok(Err(e)),
                Err(e) => Err(e),
            }
        }
    }};
}

macro_rules! comma_join {
    ($a:ident) => {
        stringify!( $a )
    };

    ($a:ident $b:ident) => {
        concat!(
            stringify!( $a ),
            " or ",
            stringify!( $b ),
        )
    };

    ($a:ident $($item:ident)+) => {
        concat!(
            stringify!( $a ),
            ", ",
            comma_join!( $($item)+ )
        )
    };
}

macro_rules! error_enum_boilerplate {
    ($( #[ $error_meta:meta ] )* $error_name:ident, $( $( #[ $variant_meta:meta ] )* $variant:ident),*) => {
        $( #[ $error_meta ] )*
        #[derive(Debug, Clone)]
        pub enum $error_name {
            $(
                $( #[ $variant_meta ] )*
                $variant($variant)
            ),*
        }

        impl TryFrom<::webcore::value::Value> for $error_name {
            type Error = ::webcore::value::ConversionError;

            fn try_from(value: ::webcore::value::Value) -> Result<Self, Self::Error> {
                $(
                    if let Ok(v) = $variant::try_from(value.clone()) {
                        return Ok($error_name::$variant(v));
                    }
                )*

                let expected = comma_join!( $($variant)+ ).into();
                Err( ::webcore::value::ConversionError::type_mismatch( &value, expected ) )
            }
        }

        impl std::fmt::Display for $error_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $($error_name::$variant( ref r ) => r.fmt(formatter),)*
                }
            }
        }

        impl std::error::Error for $error_name {
            fn description(&self) -> &str {
                stringify!($error_name)
            }
        }

        impl ::webcore::serialization::JsSerialize for $error_name {
            #[doc(hidden)]
            #[inline]
            fn _into_js< 'a >( &'a self ) -> ::webcore::serialization::SerializedValue< 'a > {
                let reference: &::webcore::value::Reference = match self {
                    $(
                        &$error_name::$variant( ref variant ) => variant.as_ref(),
                    )+
                };

                reference._into_js()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use webcore::value::Value;

    #[test]
    fn js_try() {
        let v: Result<Value, Value> = js_try!( return "test"; ).unwrap();
        assert_eq!( v, Ok(Value::String("test".to_string())) );

        let v: Result<bool, bool> = js_try!( return true; ).unwrap();
        assert_eq!( v, Ok(true) );

        let v: Result<bool, bool> = js_try!( throw true; ).unwrap();
        assert_eq!( v, Err(true) );

        let v: Result<i32, String> = js_try!( throw "error"; ).unwrap();
        assert_eq!( v, Err("error".to_string()) );

        let v: Result<(), String> = js_try!( @(no_return) 2+2; ).unwrap();
        assert_eq!( v, Ok(()) );

        let v: Result<(), f64> = js_try!( @(no_return) throw 3.3; ).unwrap();
        assert_eq!( v, Err(3.3) );

        let v: Result< Result<i32, i32>, _ > = js_try!( return "f"; );
        assert!( v.is_err() );

        let v: Result< Result<i32, i32>, _ > = js_try!( throw "Broken"; );
        assert!( v.is_err() );
    }

    #[test]
    fn js_try_from_value_to_value() {
        let output: Result< Value, String > = js_try!( return null; ).unwrap();
        assert_eq!( output, Ok( Value::Null ) );
    }
}
