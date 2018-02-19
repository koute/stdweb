use std::marker::PhantomData;
use webcore::try_from::TryInto;

/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/JavaScript/Reference/Global_Objects/Date)
#[derive(Debug)]
pub struct Date {
    dummy: PhantomData< () >
}

impl Date {
    /// The Date.now() method returns the number of milliseconds elapsed since 1 January 1970 00:00:00 UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.now
    pub fn now() -> f64 {
        js!(
            return Date.now();
        ).try_into().unwrap()
    }
}

#[test]
fn test_date_now() {
    let now = Date::now();
    assert!( now > 0.0 );
}
