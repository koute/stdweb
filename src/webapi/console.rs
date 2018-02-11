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


#[macro_export]
macro_rules! console_clear {
    () => {
    	js_call!( console.clear, );
    };
}

#[macro_export]
macro_rules! console_log {
    ( $( $args:expr ),* ) => {
    	js_call!( console.log, $( $args ),* );
    };
}

#[macro_export]
macro_rules! console_error {
    ( $( $args:expr ),* ) => {
    	js_call!( console.error, $( $args ),* );
    };
}

#[macro_export]
macro_rules! console_debug {
    ( $( $args:expr ),* ) => {
    	js_call!( console.debug, $( $args ),* );
    };
}

#[macro_export]
macro_rules! console_info {
    ( $( $args:expr ),* ) => {
    	js_call!( console.info, $( $args ),* );
    };
}
