use utils::*;

pub mod exports {
    use stdweb::js_export;
    use stdweb::web::TypedArray;

    #[js_export]
    fn i8_to_i8( value: i8 ) -> i8 { value + 1 }

    #[js_export]
    fn i16_to_i16( value: i16 ) -> i16 { value + 1 }

    #[js_export]
    fn i32_to_i32( value: i32 ) -> i32 { value + 1 }

    #[js_export]
    fn u8_to_u8( value: u8 ) -> u8 { value + 1 }

    #[js_export]
    fn u16_to_u16( value: u16 ) -> u16 { value + 1 }

    #[js_export]
    fn u32_to_u32( value: u32 ) -> u32 { value + 1 }

    #[js_export]
    fn f64_to_f64( value: f64 ) -> f64 { value * 2.0 }

    #[js_export]
    fn rstr_to_string( string: &str ) -> String {
        format!( "{}...", string )
    }

    #[js_export]
    fn string_to_string( string: String ) -> String {
        format!( "{}...", string )
    }

    #[js_export]
    fn rstr() -> &'static str {
        "A string"
    }

    #[js_export]
    fn bool_to_bool( value: bool ) -> bool {
        !value
    }

    #[js_export]
    fn vec_to_vec( input: Vec< u8 > ) -> Vec< u8 > {
        input.into_iter().map( |value| value + 1 ).collect()
    }

    #[js_export]
    fn slice_to_vec( input: &[u8] ) -> Vec< u8 > {
        input.iter().map( |&value| value + 1 ).collect()
    }

    #[js_export]
    fn slice() -> &'static [u8] {
        &[ 1, 2, 3 ]
    }

    #[js_export]
    fn typed_array_to_typed_array_identity( array: TypedArray< u8 > ) -> TypedArray< u8 > {
        array
    }

    #[js_export]
    fn typed_array_to_typed_array( array: TypedArray< u8 > ) -> TypedArray< u8 > {
        let vec: Vec< _ > = array.to_vec().into_iter().map( |value| value + 1 ).collect();
        vec.as_slice().into()
    }
}

pub fn run() {
    test( "i8_to_i8", || { js! { assert.strictEqual( Module.exports.i8_to_i8( 33 ), 34 ); }});
    test( "i16_to_i16", || { js! { assert.strictEqual( Module.exports.i16_to_i16( 33 ), 34 ); }});
    test( "i32_to_i32", || { js! { assert.strictEqual( Module.exports.i32_to_i32( 33 ), 34 ); }});
    test( "u8_to_u8", || { js! { assert.strictEqual( Module.exports.u8_to_u8( 33 ), 34 ); }});
    test( "u16_to_ui6", || { js! { assert.strictEqual( Module.exports.u16_to_u16( 33 ), 34 ); }});
    test( "u32_to_u32", || { js! { assert.strictEqual( Module.exports.u32_to_u32( 33 ), 34 ); }});
    test( "f64_to_f64", || { js! { assert.strictEqual( Module.exports.f64_to_f64( 3.33 ), 6.66 ); }});

    test( "rstr_to_string", || { js! {
        assert.strictEqual( Module.exports.rstr_to_string( "ABC" ), "ABC..." );
    }});

    test( "string_to_string", || { js! {
        assert.strictEqual( Module.exports.string_to_string( "ABC" ), "ABC..." );
    }});

    test( "rstr", || { js! {
        assert.strictEqual( Module.exports.rstr(), "A string" );
    }});

    test( "bool_to_bool", || { js! {
        assert.strictEqual( Module.exports.bool_to_bool( true ), false );
        assert.strictEqual( Module.exports.bool_to_bool( false ), true );
    }});

    test( "vec_to_vec", || { js! {
        assert.deepEqual( Module.exports.vec_to_vec( [ 1, 2, 3 ] ), [ 2, 3, 4 ] );
    }});

    test( "slice_to_vec", || { js! {
        assert.deepEqual( Module.exports.slice_to_vec( [ 1, 2, 3 ] ), [ 2, 3, 4 ] );
    }});

    test( "slice", || { js! {
        assert.deepEqual( Module.exports.slice(), [ 1, 2, 3 ] );
    }});

    test( "typed_array_to_typed_array_identity", || { js! {
        let array_in = Uint8Array.from( [ 1, 2, 3 ] );
        let array_out = Module.exports.typed_array_to_typed_array_identity( array_in );
        assert.strictEqual( array_in, array_out );
    }});

    test( "typed_array_to_typed_array", || { js! {
        let array_in = Uint8Array.from( [ 1, 2, 3 ] );
        let array_out = Module.exports.typed_array_to_typed_array( array_in );
        assert.deepEqual( Array.from( array_in ), [ 1, 2, 3 ] );
        assert.deepEqual( Array.from( array_out ), [ 2, 3, 4 ] );
    }});
}
