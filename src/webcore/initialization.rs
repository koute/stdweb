use std::panic;
use webcore::ffi;

/// Initializes the library.
///
/// Calling this is required for anything to work.
pub fn initialize() {
    static mut INITIALIZED: bool = false;
    unsafe {
        if INITIALIZED {
            return;
        }

        INITIALIZED = true;
    }

    js! { @(no_return)
        Module.STDWEB = {};
        Module.STDWEB.to_js = function to_js( address ) {
            var kind = HEAPU8[ address + 12 ];
            if( kind === 0 ) {
                return undefined;
            } else if( kind === 1 ) {
                return null;
            } else if( kind === 2 ) {
                return HEAP32[ address / 4 ];
            } else if( kind === 3 ) {
                return HEAPF64[ address / 8 ];
            } else if( kind === 4 ) {
                var pointer = HEAPU32[ address / 4 ];
                var length = HEAPU32[ (address + 4) / 4 ];
                return Module.STDWEB.to_js_string( pointer, length );
            } else if( kind === 5 ) {
                return false;
            } else if( kind === 6 ) {
                return true;
            } else if( kind === 7 ) {
                var pointer = HEAPU32[ address / 4 ];
                var length = HEAPU32[ (address + 4) / 4 ];
                var output = [];
                for( var i = 0; i < length; ++i ) {
                    output.push( Module.STDWEB.to_js( pointer + i * 16 ) );
                }
                return output;
            } else if( kind === 8 ) {
                var value_array_pointer = HEAPU32[ address / 4 ];
                var length = HEAPU32[ (address + 4) / 4 ];
                var key_array_pointer = HEAPU32[ (address + 8) / 4 ];
                var output = {};
                for( var i = 0; i < length; ++i ) {
                    var key_pointer = HEAPU32[ (key_array_pointer + i * 8) / 4 ];
                    var key_length = HEAPU32[ (key_array_pointer + 4 + i * 8) / 4 ];
                    var key = Module.STDWEB.to_js_string( key_pointer, key_length );
                    var value = Module.STDWEB.to_js( value_array_pointer + i * 16 );
                    output[ key ] = value;
                }
                return output;
            } else if( kind === 9 ) {
                return Module.STDWEB.acquire_js_reference( HEAP32[ address / 4 ] );
            } else if( kind === 10 ) {
                var adapter_pointer = HEAPU32[ address / 4 ];
                var pointer = HEAPU32[ (address + 4) / 4 ];
                var deallocator_pointer = HEAPU32[ (address + 8) / 4 ];
                var output = function() {
                    var args = _malloc( 16 );
                    Module.STDWEB.from_js( args, arguments );
                    Runtime.dynCall( "vii", adapter_pointer, [pointer, args] );
                    var result = Module.STDWEB.tmp;
                    Module.STDWEB.tmp = null;

                    return result;
                };

                output.drop = function() {
                    output.drop = null;
                    Runtime.dynCall( "vi", deallocator_pointer, [pointer] );
                };

                return output;
            }
        };
    };

    js! { @(no_return)
        Module.STDWEB.from_js = function from_js( address, value ) {
            var kind = Object.prototype.toString.call( value );
            if( kind === "[object String]" ) {
                var length = lengthBytesUTF8( value );
                var pointer = _malloc( length + 1 );
                stringToUTF8( value, pointer, length + 1 );
                HEAPU8[ address + 12 ] = 4;
                HEAPU32[ address / 4 ] = pointer;
                HEAPU32[ (address + 4) / 4 ] = length;
            } else if( kind === "[object Number]" ) {
                if( value === (value|0) ) {
                    HEAPU8[ address + 12 ] = 2;
                    HEAP32[ address / 4 ] = value;
                } else {
                    HEAPU8[ address + 12 ] = 3;
                    HEAPF64[ address / 8 ] = value;
                }
            } else if( value === null ) {
                HEAPU8[ address + 12 ] = 1;
            } else if( value === undefined ) {
                HEAPU8[ address + 12 ] = 0;
            } else if( value === false ) {
                HEAPU8[ address + 12 ] = 5;
            } else if( value === true ) {
                HEAPU8[ address + 12 ] = 6;
            } else if( kind === "[object Array]" || kind === "[object Arguments]" ) {
                var length = value.length;
                var pointer = _malloc( length * 16 );
                HEAPU8[ address + 12 ] = 7;
                HEAPU32[ address / 4 ] = pointer;
                HEAPU32[ (address + 4) / 4 ] = length;
                for( var i = 0; i < length; ++i ) {
                    Module.STDWEB.from_js( pointer + i * 16, value[ i ] );
                }
            } else if( kind === "[object Object]" ) {
                var keys = Object.keys( value );
                var length = keys.length;
                var key_array_pointer = _malloc( length * 8 );
                var value_array_pointer = _malloc( length * 16 );
                HEAPU8[ address + 12 ] = 8;
                HEAPU32[ address / 4 ] = value_array_pointer;
                HEAPU32[ (address + 4) / 4 ] = length;
                HEAPU32[ (address + 8) / 4 ] = key_array_pointer;
                for( var i = 0; i < length; ++i ) {
                    var key = keys[ i ];
                    var key_length = lengthBytesUTF8( key );
                    var key_pointer = _malloc( key_length + 1 );
                    stringToUTF8( key, key_pointer, key_length + 1 );

                    var key_address = key_array_pointer + i * 8;
                    HEAPU32[ key_address / 4 ] = key_pointer;
                    HEAPU32[ (key_address + 4) / 4 ] = key_length;

                    Module.STDWEB.from_js( value_array_pointer + i * 16, value[ key ] );
                }
            } else {
                var refid = Module.STDWEB.acquire_rust_reference( value );
                HEAPU8[ address + 12 ] = 9;
                HEAP32[ address / 4 ] = refid;
            }
        };
    };

    js! { @(no_return)
        // This is ported from Rust's stdlib; it's faster than
        // the string conversion from Emscripten.
        Module.STDWEB.to_js_string = function to_js_string( index, length ) {
            index = index|0;
            length = length|0;
            var end = (index|0) + (length|0);
            var output = "";
            while( index < end ) {
                var x = HEAPU8[ index++ ];
                if( x < 128 ) {
                    output += String.fromCharCode( x );
                    continue;
                }
                var init = (x & (0x7F >> 2));
                var y = 0;
                if( index < end ) {
                    y = HEAPU8[ index++ ];
                }
                var ch = (init << 6) | (y & 63);
                if( x >= 0xE0 ) {
                    var z = 0;
                    if( index < end ) {
                        z = HEAPU8[ index++ ];
                    }
                    var y_z = ((y & 63) << 6) | (z & 63);
                    ch = init << 12 | y_z;
                    if( x >= 0xF0 ) {
                        var w = 0;
                        if( index < end ) {
                            w = HEAPU8[ index++ ];
                        }
                        ch = (init & 7) << 18 | ((y_z << 6) | (w & 63));
                    }
                }
                output += String.fromCharCode( ch );
                continue;
            }
            return output;
        };
    };

    js! { @(no_return)
        var id_to_ref_map = {};
        var id_to_refcount_map = {};
        var ref_to_id_map = new WeakMap();
        var last_refid = 1;

        Module.STDWEB.acquire_rust_reference = function( reference ) {
            if( reference === undefined || reference === null ) {
                return 0;
            }

            var refid = ref_to_id_map.get( reference );
            if( refid === undefined ) {
                refid = last_refid++;
                ref_to_id_map.set( reference, refid );
                id_to_ref_map[ refid ] = reference;
                id_to_refcount_map[ refid ] = 1;
            } else {
                id_to_refcount_map[ refid ]++;
            }

            return refid;
        };

        Module.STDWEB.acquire_js_reference = function( refid ) {
            return id_to_ref_map[ refid ];
        };

        Module.STDWEB.increment_refcount = function( refid ) {
            id_to_refcount_map[ refid ]++;
        };

        Module.STDWEB.decrement_refcount = function( refid ) {
            id_to_refcount_map[ refid ]--;
            if( id_to_refcount_map[ refid ] === 0 ) {
                var reference = id_to_ref_map[ refid ];
                delete id_to_ref_map[ refid ];
                delete id_to_refcount_map[ refid ];
                ref_to_id_map.delete( reference );
            }
        };
    }

    if cfg!( test ) == false {
        panic::set_hook( Box::new( |info| {
            em_asm_int!( "console.error( 'Encountered a panic!' );" );
            if let Some( value ) = info.payload().downcast_ref::< String >() {
                em_asm_int!( "\
                    console.error( 'Panic error message:', Module.STDWEB.to_js_string( $0, $1 ) );\
                ", value.as_ptr(), value.len() );
            }
            if let Some( location ) = info.location() {
                let file = location.file();
                em_asm_int!( "\
                    console.error( 'Panic location:', Module.STDWEB.to_js_string( $0, $1 ) + ':' + $2 );\
                ", file.as_ptr(), file.len(), location.line() );
            }
        }));
    }
}

/// Runs the event loop.
///
/// You should call this before returning from `main()`,
/// otherwise bad things will happen.
pub fn event_loop() -> ! {
    unsafe {
        ffi::emscripten_set_main_loop( Some( ffi::emscripten_pause_main_loop ), 0, 1 );
    }

    unreachable!();
}
