use std::marker::PhantomData;

/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/JavaScript/Reference/Global_Objects/Date)
#[derive(Debug)]
pub struct Date {
    dummy: PhantomData< () >
}

impl Date {
    /// The Date.now() method returns the number of milliseconds elapsed since 1 January 1970 00:00:00 UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
    pub fn now() -> f64 {
        em_asm_double!( "return Date.now();" )
    }
}

#[test]
fn test_date_now() {
    let now = Date::now();
    assert!( now > 0.0 );
}
