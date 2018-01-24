Module.STDWEB.alloc = function alloc( size ) {
    return Module.web_malloc( size );
};

Module.STDWEB.dyncall = function( signature, ptr, args ) {
    return Module.web_table.get( ptr ).apply( null, args );
};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB.utf8_len = function utf8_len( str ) {
    let len = 0;
    for( let i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        let u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            ++len;
        } else if( u <= 0x7FF ) {
            len += 2;
        } else if( u <= 0xFFFF ) {
            len += 3;
        } else if( u <= 0x1FFFFF ) {
            len += 4;
        } else if( u <= 0x3FFFFFF ) {
            len += 5;
        } else {
            len += 6;
        }
    }
    return len;
};
