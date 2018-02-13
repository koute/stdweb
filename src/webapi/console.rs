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
/// ## clear
///
/// Clear the console:
///
/// ```rust
/// console!(clear);
/// ```
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/clear)
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
/// All of the `log` examples also work with `error`, `debug`, `info`, and `warn`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
#[macro_export]
macro_rules! console {
    ( $name:ident ) => {{
        js! { @(no_return)
            console.$name();
        }
        ()
    }};

    ( $name:ident, $( $args:expr ),* ) => {{
        js! { @(no_return)
            console.$name( $( @{$args} ),* );
        }
        ()
    }};
}
