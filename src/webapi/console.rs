#[doc(hidden)]
#[macro_export]
macro_rules! js_call {
	( $f:expr, $( $args:expr ),* ) => {{
    	js! { @(no_return)
    		$f( $( @{$args} ),* );
    	}
    	()
    }};
}


/// Clears the console.
///
/// If it cannot clear the console then it does nothing.
///
/// # Examples
///
/// ```rust
/// console_clear!();
/// ```
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/clear)
#[macro_export]
macro_rules! console_clear {
    () => {
    	js_call!( console.clear, );
    };
}


/// Prints zero or more values to the console.
///
/// The values can be anything that can be sent to JavaScript, and they do not
/// need to be the same type.
///
/// Usually it will print a space in between each value, and it will always print
/// a newline at the end.
///
/// However, this is *not guaranteed*, the printing behavior **will** vary between
/// different browsers and Node.js, therefore you should use `console_log` *only*
/// for debugging purposes.
///
/// If the first value is a string, then you can use string substitutions to
/// control how the remaining values are printed. Please see [this site](https://developer.mozilla.org/en-US/docs/Web/API/console#Using_string_substitutions)
/// for more information about string substitutions.
///
/// # Examples
///
/// Print a newline:
///
/// ```rust
/// console_log!("");
/// ```
///
/// Print one value:
///
/// ```rust
/// console_log!("Hello world!");
/// ```
///
/// Print more than one value:
///
/// ```rust
/// console_log!(1, "test", vec![2, 3]);
/// ```
///
/// Print more than one value with Rust's formatting:
///
/// ```rust
/// console_log!(format!("{:#?} {:#?} {:#?}", 1, "test", vec![2, 3]));
/// ```
///
/// Use string substitution to control how the values are printed:
///
/// ```rust
/// console_log!("foo: %s bar: %s", vec![1, 2], vec![3, 4]);
/// ```
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
#[macro_export]
macro_rules! console_log {
    ( $( $args:expr ),* ) => {
    	js_call!( console.log, $( $args ),* );
    };
}


/// Prints zero or more values to the console.
///
/// This is exactly the same as [`console_log`](macro.console_log.html), except that it prints a special "error" message.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/error)
#[macro_export]
macro_rules! console_error {
    ( $( $args:expr ),* ) => {
    	js_call!( console.error, $( $args ),* );
    };
}


/// Prints zero or more values to the console.
///
/// This is exactly the same as [`console_log`](macro.console_log.html), except that it prints a special "debug" message.
///
/// In some JavaScript environments it won't print anything at all, so do not use it
/// for anything other than debugging.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/debug)
#[macro_export]
macro_rules! console_debug {
    ( $( $args:expr ),* ) => {
    	js_call!( console.debug, $( $args ),* );
    };
}


/// Prints zero or more values to the console.
///
/// This is exactly the same as [`console_log`](macro.console_log.html), except that it prints a special "info" message.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Console/info)
#[macro_export]
macro_rules! console_info {
    ( $( $args:expr ),* ) => {
    	js_call!( console.info, $( $args ),* );
    };
}
