use std::collections::HashMap;
use webcore::try_from::{TryFrom, TryInto};
use webcore::value::{Value, Reference};

/// A date object
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
pub trait IDate: AsRef< Reference > + TryFrom< Value >  {
    /// Returns the number of milliseconds elapsed since 1 January 1970 00:00:00 UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
    #[inline]
    fn now() -> f64 {
        js!( return Date.now(); ).try_into().unwrap()
    }

    /// Accepts the same parameters as the longest form of the constructor,
    /// and returns the number of milliseconds in a Date object since January 1, 1970, 00:00:00,
    /// universal time
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/utc)
    #[inline]
    fn utc( year: i32, month: u8, day: u8, hour: u8, minutes: u16, seconds: u16, milliseconds: u32) -> i64 {
        let args_obj: HashMap< &str, Value > = [
            ("year",         Value::Number( year.into() )),
            ("month",        Value::Number( month.into() )),
            ("day",          Value::Number( day.into() )),
            ("hour",         Value::Number( hour.into() )),
            ("minutes",      Value::Number( minutes.into() )),
            ("seconds",      Value::Number( seconds.into() )),
            ("milliseconds", Value::Number( milliseconds.into() ))
        ].iter().cloned().collect();

        js!(
            var args = @{args_obj};
            return Date.UTC( args.year, args.month, args.day, args.hour, args.minutes, args.seconds, args.milliseconds );
        ).try_into().unwrap()
    }

    /// Parses a string representation of a date, and returns the number of
    /// milliseconds since 1 January 1970 00:00:00 UTC, or NaN if the string is unrecognize or, in
    /// some cases, contains illegal data values (e.g. 2015-02-31)
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/parse)
    #[inline]
    fn parse( date_string: &str ) -> i64 {
        js!( return Date.parse( @{date_string} ); ).try_into().unwrap()
    }

    /// Returns the day of the month for the specified date according to local
    /// time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDate)
    #[inline]
    fn get_date( &self ) -> i32 {
        js!( return @{self.as_ref()}.getDate(); ).try_into().unwrap()
    }

    /// Returns the day of the week for the specified date according to local
    /// time, where 0 represents Sunday. For the day of the month see getDate().
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay)
    #[inline]
    fn get_day( &self ) -> i32 {
        js!( return @{self.as_ref()}.getDay(); ).try_into().unwrap()
    }

    /// Returns the year of the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getFullYear)
    #[inline]
    fn get_full_year( &self ) -> i32 {
        js!( return @{self.as_ref()}.getFullYear(); ).try_into().unwrap()
    }

    /// Returns the hour for the specified date, according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getHours)
    #[inline]
    fn get_hours( &self) -> i32 {
        js!( return @{self.as_ref()}.getHours(); ).try_into().unwrap()
    }

    /// Returns the milliseconds in the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMilliseconds)
    #[inline]
    fn get_milliseconds( &self ) -> i32 {
        js!( return @{self.as_ref()}.getMilliseconds(); ).try_into().unwrap()
    }

    /// Returns the minutes in the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMinutes)
    #[inline]
    fn get_minutes( &self ) -> i32 {
        js!( return @{self.as_ref()}.getMinutes(); ).try_into().unwrap()
    }

    /// Returns the month in the specified date according to local time, as a
    /// zero-based value (where zero indicates the first month of the year).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMonth)
    #[inline]
    fn get_month( &self ) -> i32 {
        js!( return @{self.as_ref()}.getMonth(); ).try_into().unwrap()
    }

    /// Returns the seconds in the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getSeconds)
    #[inline]
    fn get_seconds( &self ) -> i32 {
        js!( return @{self.as_ref()}.getSeconds(); ).try_into().unwrap()
    }

    /// Returns the numeric value corresponding to the time for the specified
    /// date according to universal time.
    ///
    /// getTime() always uses UTC for time representation. For example, a client browser in one
    /// timezone, getTime() will be the same as a client browser in any other timezone.
    ///
    /// You can use this method to help assign a date and time to another Date object. This method
    /// is functionally equivalent to the valueOf() method.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTime)
    #[inline]
    fn get_time( &self ) -> f64 {
        js!( return @{self.as_ref()}.getTime(); ).try_into().unwrap()
    }

    /// Returns the time zone difference, in minutes, from UTC to
    /// current locale (host system settings).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset)
    #[inline]
    fn get_timezone_offset( &self ) -> i32 {
        js!( return @{self.as_ref()}.getTimezoneOffset(); ).try_into().unwrap()
    }

    /// Returns the day (date) of the month in the specified date according
    /// to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDate)
    #[inline]
    fn get_utc_date( &self ) -> i32 {
        js!( return @{self.as_ref()}.getUTCDate(); ).try_into().unwrap()
    }

    /// Returns the day of the week in the specified date according to
    /// universal time, where 0 represents Sunday.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDay)
    #[inline]
    fn get_utc_day( &self ) -> i32 {
        js!( return @{self.as_ref()}.getUTCDay(); ).try_into().unwrap()
    }

    /// Returns the year in the specified date according to universal
    /// time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCFullYear)
    #[inline]
    fn get_utc_full_year( &self ) -> i32 {
        js!( return @{self.as_ref()}.getUTCFullYear(); ).try_into().unwrap()
    }

    /// Returns the hours in the specified date according to universal
    /// time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCHours)
    #[inline]
    fn get_utc_hours( &self ) -> i32 {
        js!( return @{self.as_ref()}.getUTCHours(); ).try_into().unwrap()
    }

    /// Returns the milliseconds in the specified date according to
    /// universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMilliseconds)
    #[inline]
    fn get_utc_milliseconds( &self ) -> i32 {
        js!( return @{self.as_ref()}.getUTCMilliseconds(); ).try_into().unwrap()
    }

    /// Returns the minutes in the specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMinutes)
    #[inline]
    fn get_utc_minutes( &self ) -> i32 {
        js!( return @{self.as_ref()}.getUTCMinutes(); ).try_into().unwrap()
    }

    /// Returns the month of the specified date according to universal time, as a
    /// zero-based value (where zero indicates the first month of the year).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMonth)
    #[inline]
    fn get_utc_month( &self ) -> i32 {
        js!( return @{self.as_ref()}.getUTCMonth(); ).try_into().unwrap()
    }

    /// Returns the seconds in the specified date according to universal
    /// time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCSeconds)
    #[inline]
    fn get_utc_seconds( &self ) -> i32 {
        js!( return @{self.as_ref()}.getUTCSeconds(); ).try_into().unwrap()
    }

    // DEPRECATED
    // fn get_year( &self ) -> i32 { }

    /// Sets the day of the Date object relative to the beginning of the
    /// currently set month.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setDate)
    #[inline]
    fn set_date( &self, day_value: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setDate(@{day_value});
        };
    }

    /// Sets the full year for a specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear)
    #[inline]
    fn set_full_year( &self, year: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setFullYear(@{year});
        };
    }

    /// Sets the hours for a specified date according to local time, and
    /// returns the number of milliseconds since 1 January 1970 00:00:00 UTC until the time
    /// represented by the updated Date instance.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours)
    #[inline]
    fn set_hours( &self, hour: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setHours( @{hour} );
        }
    }

    /// Sets the milliseconds for a specified date according to local
    /// time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMilliseconds)
    #[inline]
    fn set_milliseconds( &self, milliseconds_value: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setMilliseconds(@{milliseconds_value});
        };
    }

    /// Sets the minutes for a specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes)
    #[inline]
    fn set_minutes( &self, minutes: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setMinutes( @{minutes} );
        }
    }

    /// Sets the month for a specified date according to the currently set
    /// year.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMonth)
    #[inline]
    fn set_month( &self, month: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setMonth(@{month});
        }
    }

    /// Sets the seconds for a specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setSeconds)
    #[inline]
    fn set_seconds( &self, seconds: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setSeconds(@{seconds});
        }
    }

    /// Sets the Date object to the time represented by a number of
    /// milliseconds since January 1, 1970, 00:00:00 UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setTime)
    #[inline]
    fn set_time( &self, time_value: f64 ) {
        js! { @(no_return)
            @{self.as_ref()}.setTime(@{time_value});
        };
    }

    /// Sets the day of the month for a specified date according to
    /// universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCDate)
    #[inline]
    fn set_utc_date( &self, day_value: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setUTCDate(@{day_value});
        };
    }

    /// Sets the full year for a specified date according to universal
    /// time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear)
    #[inline]
    fn set_utc_full_year( &self, year: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setUTCFullYear(@{year});
        }
    }

    /// Sets the hour for a specified date according to universal time
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours)
    #[inline]
    fn set_utc_hours( &self, hours: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setUTCHours(@{hours});
        }
    }

    /// Sets the milliseconds for a specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMilliseconds)
    #[inline]
    fn set_utc_milliseconds( &self, milliseconds_value: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setUTCMilliseconds(@{milliseconds_value});
        };
    }

    /// Sets the minutes for a specified date according to universal
    /// time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes)
    #[inline]
    fn set_utc_minutes( &self, minutes: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setUTCMinutes(@{minutes});
        }
    }

    /// Sets the month for a specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMonth)
    #[inline]
    fn set_utc_month( &self, month: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setUTCMonth(@{month});
        }
    }

    /// Sets the seconds for a specified date according to universal
    /// time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCSeconds)
    #[inline]
    fn set_utc_seconds( &self, seconds: i32 ) {
        js! { @(no_return)
            @{self.as_ref()}.setUTCSeconds(@{seconds});
        }
    }

    // DEPRECATED
    // fn set_year() -> i32 { }

    /// Returns the date portion of a Date object in human readable form in American English.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toDateString)
    #[inline]
    fn to_date_string( &self ) -> String {
        js! ( return @{self.as_ref()}.toDateString(); ).try_into().unwrap()
    }

    // DEPRECATED
    // fn to_gmt_string( &self ) -> i32 { }

    /// Returns a string in simplified extended ISO format (ISO 8601),
    /// which is always 24 or 27 characters long (YYYY-MM-DDTHH:mm:ss.sssZ or
    /// ±YYYYYY-MM-DDTHH:mm:ss.sssZ, respectively). The timezone is always zero UTC offset, as
    /// denoted by the suffix "Z".
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
    #[inline]
    fn to_iso_string( &self ) -> String {
        js! ( return @{self.as_ref()}.toISOString(); ).try_into().unwrap()
    }

    /// Returns a string representation of the Date object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toJSON)
    #[inline]
    fn to_json( &self ) -> String {
        js! ( return @{self.as_ref()}.toJSON(); ).try_into().unwrap()
    }

    /// Returns a string with a language sensitive representation
    /// of the date portion of this date. The new locales and options arguments let applications
    /// specify the language whose formatting conventions should be used and allow to customize the
    /// behavior of the function. In older implementations, which ignore the locales and options
    /// arguments, the locale used and the form of the string returned are entirely implementation
    /// dependent.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleDateString)
    #[inline]
    fn to_locale_date_string( &self, _locales: Option< &str >, _options: Option< &HashMap< &str, &str > >) -> String {
        match _locales {
            Some(_locales) => match _options {
                Some(_options) => return js! (
                    var _options = @{_options};

                    return @{self.as_ref()}.toLocaleDateString(@{_locales}, _options);
                 ).try_into().unwrap(),

                 None => return js! (
                     return @{self.as_ref()}.toLocaleDateString(@{_locales});
                 ).try_into().unwrap()
            },

            None => return js! ( return @{self.as_ref()}.toLocaleDateString(); ).try_into().unwrap()
        }

    }

    // NON-STANDARD
    // fn to_locale_format( &self ) -> i32 { }

    /// Returns a string with a language sensitive representation of
    /// this date. The new locales and options arguments let applications specify the language
    /// whose formatting conventions should be used and customize the behavior of the function. In
    /// older implementations, which ignore the locales and options arguments, the locale used and
    /// the form of the string returned are entirely implementation dependent.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString)
    #[inline]
    fn to_locale_string( &self, _locales: Option< &str >, _options: Option< &HashMap< &str, &str > >) -> String {
        match _locales {
            Some(_locales) => match _options {
                Some(_options) => return js! (
                    var _options = @{_options};

                    return @{self.as_ref()}.toLocaleString(@{_locales}, _options);
                ).try_into().unwrap(),

                None => return js! (
                    return @{self.as_ref()}.toLocaleString(@{_locales});
                ).try_into().unwrap()
            },

            None => return js! ( return @{self.as_ref()}.toLocaleString(); ).try_into().unwrap()
        }
    }

    /// Returns a string with a language sensitive representation
    /// of the time portion of this date. The new locales and options arguments let applications
    /// specify the language whose formatting conventions should be used and customize the behavior
    /// of the function. In older implementations, which ignore the locales and options arguments,
    /// the locale used and the form of the string returned are entirely implementation dependent.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleTimeString)
    #[inline]
    fn to_locale_time_string( &self, _locales: Option< &str >, _options: Option< &HashMap< &str, &str > > ) -> String {
        match _locales {
            Some(_locales) => match _options {
                Some(_options) => return js! (
                    var _options = @{_options};

                    return @{self.as_ref()}.toLocaleTimeString(@{_locales}, _options);
                ).try_into().unwrap(),

                None => return js! (
                    return @{self.as_ref()}.toLocaleTimeString(@{_locales});
                ).try_into().unwrap()
            },

            None => return js! ( return @{self.as_ref()}.toLocaleTimeString(); ).try_into().unwrap()
        }
    }

    // NON-STANDARD
    // fn to_source( &self ) -> i32 { }

    /// Returns the time portion of a Date object in human readable form
    /// in American English.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toTimeString)
    #[inline]
    fn to_time_string( &self ) -> String {
        js! ( return @{self.as_ref()}.toTimeString(); ).try_into().unwrap()
    }

    /// Converts a date to a string, using the UTC time zone.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString)
    #[inline]
    fn to_utc_string( &self ) -> String {
        js! ( return @{self.as_ref()}.toUTCString(); ).try_into().unwrap()
    }

    /// Returns the primitive value of a Date object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/valueOf)
    #[inline]
    fn value_of( &self ) -> f64 {
        js!( return @{self.as_ref()}.valueOf(); ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IDate](trait.IDate.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/JavaScript/Reference/Global_Objects/Date)
pub struct Date( Reference );

impl IDate for Date {}

reference_boilerplate! {
    Date,
    instanceof Date
}

/////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let now = Date::now();

        /// could make test assertion more specific...
        assert!( now > 0.0 );
    }

    #[test]
    fn test_utc() {
        // mostly default values...
        let d_utc = Date::utc( 2000, 0, 1, 0, 0, 0, 0 ); // y2k ftw
        assert!( d_utc == 946684800000 );
    }

    #[test]
    fn test_parse() {
        assert!(Date::parse("Wed, 09 Aug 1995 00:00:00 GMT") == 807926400000);
    }

    #[test]
    fn test_get_date() {
        let d: Date = js!( return new Date(2000, 0, 1); ).try_into().unwrap();
        assert!( d.get_date() == 1 ); // the day
    }

    #[test]
    fn test_get_day() {
        let d: Date = js!( return new Date(2000, 0, 1); ).try_into().unwrap();
        assert!( d.get_day() == 6 ); // the day "index"
    }

    #[test]
    fn test_get_full_year() {
        let d: Date = js!( return new Date(2000, 0, 1); ).try_into().unwrap();
        assert!( d.get_full_year() == 2000 );
    }

    #[test]
    fn test_get_hours() {
        let d: Date = js!( return new Date(2000, 0, 1, 12); ).try_into().unwrap();
        assert!( d.get_hours() == 12 );
    }

    #[test]
    fn test_get_milliseconds() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th...
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_milliseconds() == 10 );
    }

    #[test]
    fn test_get_minutes() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_minutes() == 5 );
    }

    #[test]
    fn test_get_month() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_month() == 0 );
    }

    #[test]
    fn test_get_seconds() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_seconds() == 7 );
    }

    #[test]
    fn test_get_time() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_time() == 946746307010.0 );
    }

    #[test]
    fn test_get_timezone_offset() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_timezone_offset() == 300 );
    }

    #[test]
    fn test_get_utc_date() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_utc_date() == 1 );
    }

    #[test]
    fn test_get_utc_day() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_utc_day() == 6 );
    }

    #[test]
    fn test_get_utc_full_year() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_utc_full_year() == 2000 );
    }

    #[test]
    fn test_utc_milliseconds() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_utc_milliseconds() == 10 );
    }

    #[test]
    fn test_get_utc_minutes() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_utc_minutes() == 5 );
    }

    #[test]
    fn test_get_utc_month() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_utc_month() == 0 );
    }

    #[test]
    fn test_get_utc_seconds() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.get_utc_seconds() == 7 );
    }

    #[test]
    fn test_set_date() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();

        d.set_date(8); // change day to 8th...
        assert!( d.get_date() == 8 );
    }

    #[test]
    fn test_set_full_year() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();

        d.set_full_year( 2002 );
        assert!( d.get_full_year() == 2002 );
    }

    #[test]
    fn test_set_hours() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();

        d.set_hours( 3 );
        assert!( d.get_hours() == 3 );
    }

    #[test]
    fn test_set_milliseconds() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();

        d.set_milliseconds( 80 );
        assert!( d.get_milliseconds() == 80 );
    }

    #[test]
    fn test_set_minutes() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();

        d.set_minutes( 20 );
        assert!( d.get_minutes() == 20 );
    }

    #[test]
    fn test_set_month() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();

        d.set_month( 3 );
        assert!( d.get_month() == 3 );
    }

    #[test]
    fn test_set_seconds() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();

        d.set_seconds( 42 );
        assert!( d.get_seconds() == 42 );
    }

    #[test]
    fn test_set_time() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d1: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        let d2: Date = js!( return new Date(); ).try_into().unwrap();

        d2.set_time( d1.get_time() );
        assert!( d1.get_time() == d2.get_time() );
    }

    #[test]
    fn test_set_utc_date() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        d.set_utc_date(11);
        assert!( d.get_utc_date() == 11 );
    }

    #[test]
    fn test_set_utc_full_year() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        d.set_utc_full_year( 2222 );
        assert!( d.get_utc_full_year() == 2222 );
    }

    #[test]
    fn test_set_utc_hours() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        d.set_utc_hours( 9 );
        assert!( d.get_utc_hours() == 9 );
    }

    #[test]
    fn test_set_utc_milliseconds() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        d.set_utc_milliseconds(999);
        assert!( d.get_utc_milliseconds() == 999 );
    }

    #[test]
    fn test_set_utc_minutes() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        d.set_utc_minutes( 33 );
        assert!( d.get_utc_minutes() == 33 );
    }

    #[test]
    fn test_set_utc_month() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        d.set_utc_month( 4 );
        assert!( d.get_utc_month() == 4 );
    }

    #[test]
    fn test_set_utc_seconds() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        d.set_utc_seconds( 59 );
        assert!( d.get_utc_seconds() == 59 );
    }

    #[test]
    fn test_to_date_string() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.to_date_string() == "Sat Jan 01 2000".to_string() );
    }

    #[test]
    fn test_to_iso_string() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.to_iso_string() == "2000-01-01T17:05:07.010Z".to_string() );
    }

    #[test]
    fn test_to_json() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.to_json() == "2000-01-01T17:05:07.010Z".to_string() );
    }

    #[test]
    fn test_to_locale_date_string() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.to_locale_date_string( None, None ) == "1/1/2000".to_string() );
    }

    #[test]
    fn test_to_locale_string() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.to_locale_string( None, None ) == "1/1/2000, 12:05:07 PM".to_string() );
    }

    #[test]
    fn test_to_locale_time_string() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.to_locale_time_string( None, None ) == "12:05:07 PM".to_string() );
    }

    #[test]
    fn test_to_time_string() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.to_time_string() == "12:05:07 GMT-0500 (EST)".to_string() );
    }

    #[test]
    fn test_to_utc_string() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.to_utc_string() == "Sat, 01 Jan 2000 17:05:07 GMT".to_string() );
    }

    #[test]
    fn test_value_of() {
        // year: 2000, month: January, day: 1st, hour: 12th, minute: 5th, second: 7th, millisecond:
        // 10th
        let d: Date = js!( return new Date(2000, 0, 1, 12, 5, 7, 10); ).try_into().unwrap();
        assert!( d.value_of() == 946746307010.0 );
    }
}

