Module.STDWEB.alloc = function alloc( size ) {
    return _malloc( size );
};

Module.STDWEB.dyncall = function( signature, ptr, args ) {
    return Runtime.dynCall( signature, ptr, args );
};

Module.STDWEB.utf8_len = function utf8_len( str ) {
    return lengthBytesUTF8( str );
};
