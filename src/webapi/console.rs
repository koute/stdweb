#[doc(hidden)]
#[macro_export]
macro_rules! __internal_console_unsafe {
    ( $name:ident ) => {{
        $crate::js! { @(no_return)
            console.$name();
        }
        ()
    }};

    ( $name:ident, $( $args:expr ),* ) => {{
        $crate::js! { @(no_return)
            console.$name( $( @{$args} ),* );
        }
        ()
    }};
}


/// Calls methods on the JavaScript `console` object.
///
/// This should **only** be used for debugging purposes, its behavior is
/// **not** standardized: it **will** vary with different browsers
/// and Node.js.
///
/// The first argument is the name of the `console` method.
///
/// The remaining arguments can be anything which can be sent to JavaScript,
/// and they do not need to be the same type.
///
/// If you want to print things to the console in a standardized way, use
/// [`println!`](https://doc.rust-lang.org/std/macro.println.html) instead.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/console)
///
/// # Examples
///
/// ## log
///
/// Print a newline:
///
/// ```rust
/// console!(log, "");
/// ```
///
/// Print one value:
///
/// ```rust
/// console!(log, "Hello world!");
/// ```
///
/// Print more than one value:
///
/// ```rust
/// console!(log, 1, "test", vec![2, 3]);
/// ```
///
/// Use [string substitution](https://developer.mozilla.org/en-US/docs/Web/API/console#Using_string_substitutions) to control how the values are printed:
///
/// ```rust
/// console!(log, "foo: %s bar: %s", vec![1, 2], vec![3, 4]);
/// ```
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
///
/// ## error
///
/// This is exactly the same as `log`, except it prints an error message rather than a normal message.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/error)
#[macro_export]
macro_rules! console {
    ( log, $( $args:expr ),+ ) => { $crate::__internal_console_unsafe!( log, $( $args ),+ ) };
    ( error, $( $args:expr ),+ ) => { $crate::__internal_console_unsafe!( error, $( $args ),+ ) };
}
