use webcore::value::Value;
use webcore::try_from::TryInto;

/// A type representing a JavaScript `Symbol`.
#[derive(Debug)]
pub struct Symbol( pub(crate) i32 );

impl Clone for Symbol {
    fn clone( &self ) -> Self {
        let id = __js_raw_asm_int!( concat!(
            "var value = Module.STDWEB_PRIVATE.get_raw_value( $0 );",
            "return Module.STDWEB_PRIVATE.register_raw_value( value );"
        ), self.0 );

        Symbol( id )
    }
}

impl PartialEq for Symbol {
    fn eq( &self, rhs: &Symbol ) -> bool {
        // TODO: Speed this up.
        js!(
            return Module.STDWEB_PRIVATE.get_raw_value( @{self.0} ) === Module.STDWEB_PRIVATE.get_raw_value( @{rhs.0} );
        ).try_into().unwrap()
    }
}

impl Drop for Symbol {
    fn drop( &mut self ) {
        js!( @(no_return)
            Module.STDWEB_PRIVATE.unregister_raw_value( @{self.0} );
        );
    }
}

impl From< Symbol > for Value {
    #[inline]
    fn from( symbol: Symbol ) -> Self {
        Value::Symbol( symbol )
    }
}
