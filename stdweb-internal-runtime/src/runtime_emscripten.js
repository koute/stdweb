Module.STDWEB_PRIVATE.alloc = function alloc( size ) {
    return _malloc( size );
};

Module.STDWEB_PRIVATE.dyncall = function( signature, ptr, args ) {
    return dynCall( signature, ptr, args );
};

Module.STDWEB_PRIVATE.utf8_len = function utf8_len( str ) {
    return lengthBytesUTF8( str );
};
