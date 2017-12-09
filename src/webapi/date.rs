use std::collections::HashMap;
use webcore::try_from::{TryFrom, TryInto};
use webcore::value::{Value, Reference};

/// A date object
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Date)
pub trait IDate: AsRef< Reference > + TryFrom< Value >  {
    /// The Date.now() method returns the number of milliseconds elapsed since 1 January 1970 00:00:00 UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
    #[inline]
    fn now() -> f64 {
        // em_asm_double!( "return Date.now();" )
        js!( return Date.now(); ).try_into().unwrap()
    }

    /// Date.utc
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/utc)
    #[inline]
    fn utc(args: &[i32] ) -> f64 {
        // args represents:
        // [<year_value>, <month_value>, <day_value>, <hour_value>, <minute_value>,
        // <second_value>, <millisecond_value>]
        match args.len() {
            7 => {
                let date_obj: HashMap< String, Value > = [
                    ("year".to_string(),        Value::Number( args[0].into())),
                    ("month".to_string(),       Value::Number( args[1].into())),
                    ("day".to_string(),         Value::Number( args[2].into())),
                    ("hour".to_string(),        Value::Number( args[3].into())),
                    ("minute".to_string(),      Value::Number( args[4].into())),
                    ("second".to_string(),      Value::Number( args[5].into())),
                    ("millisecond".to_string(), Value::Number( args[6].into()))
                ].iter().cloned().collect();

                js!(
                    var dateObj = @{date_obj};

                    return Date.UTC(
                        dateObj.year,
                        dateObj.month,
                        dateObj.day,
                        dateObj.hour,
                        dateObj.minute,
                        dateObj.second,
                        dateObj.millisecond
                    );
                ).try_into().unwrap()
            },
            6 => {
                let date_obj: HashMap< String, Value > = [
                    ("year".to_string(),        Value::Number( args[0].into())),
                    ("month".to_string(),       Value::Number( args[1].into())),
                    ("day".to_string(),         Value::Number( args[2].into())),
                    ("hour".to_string(),        Value::Number( args[3].into())),
                    ("minute".to_string(),      Value::Number( args[4].into())),
                    ("second".to_string(),      Value::Number( args[5].into()))
                ].iter().cloned().collect();

                js!(
                    var dateObj = @{date_obj};

                    return Date.UTC(
                        dateObj.year,
                        dateObj.month,
                        dateObj.day,
                        dateObj.hour,
                        dateObj.minute,
                        dateObj.second
                    );
                ).try_into().unwrap()
            },
            5 => {
                let date_obj: HashMap< String, Value > = [
                    ("year".to_string(),        Value::Number( args[0].into())),
                    ("month".to_string(),       Value::Number( args[1].into())),
                    ("day".to_string(),         Value::Number( args[2].into())),
                    ("hour".to_string(),        Value::Number( args[3].into())),
                    ("minute".to_string(),      Value::Number( args[4].into()))
                ].iter().cloned().collect();

                js!(
                    var dateObj = @{date_obj};

                    return Date.UTC(
                        dateObj.year,
                        dateObj.month,
                        dateObj.day,
                        dateObj.hour,
                        dateObj.minute
                    );
                ).try_into().unwrap()
            },
            4 => {
                js!( return Date.UTC( @{args[0]}, @{args[1]}, @{args[2]}, @{args[3]} ); ).try_into().unwrap()
            },
            3 => {
                js!( return Date.UTC( @{args[0]}, @{args[1]}, @{args[2]} ); ).try_into().unwrap()
            },
            2 => {
                js!( return Date.UTC( @{args[0]}, @{args[1]} ); ).try_into().unwrap()
            },
            1 => js!( return Date.UTC( @{args[0]}); ).try_into().unwrap(),
            _ => js!( return Date.UTC(); ).try_into().unwrap(),
        }
    }

    /// The Date.parse method parses a string representation of a date, and returns the number of
    /// milliseconds since 1 January 1970 00:00:00 UTC, or NaN if the string is unrecognize or, in
    /// some cases, contains illegal data values (e.g. 2015-02-31)
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/parse)
    #[inline]
    fn parse( _date_string: &str ) -> f64 {
        em_asm_double!( "return Date.parse(@{_date_string}" )
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDate)
    #[inline]
    fn get_date( &self ) -> i32 {
        js!( return @{self.as_ref()}.getDate() ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay)
    #[inline]
    fn get_day( &self ) -> i32 {
        js!( return @{self.as_ref()}.getDay(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getFullYear)
    #[inline]
    fn get_full_year( &self ) -> f64 {
        js!( return @{self.as_ref()}.getFullYear(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getHours)
    #[inline]
    fn get_hours( &self) -> f64 {
        js!( return @{self.as_ref()}.getHours(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMilliseconds)
    #[inline]
    fn get_milliseconds( &self ) -> f64 {
        js!( return @{self.as_ref()}.getMilliseconds(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMinutes)
    #[inline]
    fn get_minutes( &self ) -> i16 {
        js!( return @{self.as_ref()}.getMinutes(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMonth)
    #[inline]
    fn get_month( &self ) -> i8 {
        js!( return @{self.as_ref()}.getMonth(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getSeconds)
    #[inline]
    fn get_seconds( &self ) -> i16 {
        js!( return @{self.as_ref()}.getSeconds(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTime)
    #[inline]
    fn get_time( &self ) -> f64 {
        js!( return @{self.as_ref()}.getTime(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset)
    #[inline]
    fn get_timezone_offset( &self ) -> f64 {
        js!( return @{self.as_ref()}.getTimezoneOffset(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDate)
    #[inline]
    fn get_utc_date( &self ) -> f64 {
        js!( return @{self.as_ref()}.getUTCDate(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDay)
    #[inline]
    fn get_utc_day( &self ) -> f64 {
        js!( return @{self.as_ref()}.getUTCDay(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCFullYear)
    #[inline]
    fn get_utc_full_year( &self ) -> f64 {
        js!( return @{self.as_ref()}.getUTCFullYear(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCHours)
    #[inline]
    fn get_utc_hours( &self ) -> f64 {
        js!( return @{self.as_ref()}.getUTCHours(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMilliseconds)
    #[inline]
    fn get_utc_milliseconds( &self ) -> f64 {
        js!( return @{self.as_ref()}.getUTCMilliseconds(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMinutes)
    #[inline]
    fn get_utc_minutes( &self ) -> f64 {
        js!( return @{self.as_ref()}.getUTCMinutes(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMonth)
    #[inline]
    fn get_utc_month( &self ) -> f64 {
        js!( return @{self.as_ref()}.getUTCMonth(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCSeconds)
    #[inline]
    fn get_utc_seconds( &self ) -> f64 {
        js!( return @{self.as_ref()}.getUTCSeconds(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getYear)
    #[inline]
    fn get_year( &self ) -> f64 {
        js!( return @{self.as_ref()}.getYear(); ).try_into().unwrap()
    }

    /// NOTE: for the following functions that take arguments in addition to self, the arguments
    /// are allowed to be empty. From the user's perspective, the first argument is mandatory, with
    /// the rest optional. However, browsers do not need all arguments, even the first, but calling
    /// with empty arguments returns NaN.

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setDate)
    #[inline]
    fn set_date( &self, _day_value: f64 ) -> f64 {
        js!( return @{self.as_ref()}.setDate(@{_day_value}); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear)
    #[inline]
    fn set_full_year( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<year_value>, <month_value>, <day_value>]

        match args.len() {
            3 => {
                let args_obj: HashMap< String, Value > = [
                    ("year".to_string(),  Value::Number( args[0].into())),
                    ("month".to_string(), Value::Number( args[1].into())),
                    ("day".to_string(),   Value::Number( args[2].into()))
                ].iter().cloned().collect();

                js!(
                    var args = @{args_obj};
                    return @{self.as_ref()}.setFullYear(args.year, args.month, args.day);
                ).try_into().unwrap()
            },
            2 => {
                js!(
                    return @{self.as_ref()}.setFullYear(@{args[0]}, @{args[1]});
                ).try_into().unwrap()
            },
            1 => {
                js!(
                    return @{self.as_ref()}.setFullYear(@{args[0]});
                ).try_into().unwrap()
            },
            _ => js!( return @{self.as_ref()}.setFullYear(); ).try_into().unwrap()
        }
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours)
    #[inline]
    fn set_hours( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<hours_value>, <minutes_value>, <seconds_value>, <ms_value>]
        match args.len() {
            4 => {
                let args_obj: HashMap< String, Value > = [
                    ("hours".to_string(),   Value::Number( args[0].into())),
                    ("minutes".to_string(), Value::Number( args[1].into())),
                    ("seconds".to_string(), Value::Number( args[2].into())),
                    ("ms".to_string(),      Value::Number( args[3].into()))
                ].iter().cloned().collect();

                js!(
                    var args = @{args_obj};
                    return @{self.as_ref()}.setHours(args.hours, args.minutes, args.seconds, args.ms);
                ).try_into().unwrap()
            },
            3 => {
                js!(
                    return @{self.as_ref()}.setHours(@{args[0]}, @{args[1]}, @{args[2]});
                ).try_into().unwrap()
            },
            2 => {
                js!(
                    return @{self.as_ref()}.setHours(@{args[0]}, @{args[1]});
                ).try_into().unwrap()
            },
            1 => {
                js!( return @{self.as_ref()}.setHours(@{args[0]}); ).try_into().unwrap()
            },
            _ => js!( return @{self.as_ref()}.setHours(); ).try_into().unwrap()
        }
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMilliseconds)
    #[inline]
    fn set_milliseconds( &self, _milliseconds_value: f64 ) -> f64 {
        js!( return @{self.as_ref()}.setMilliseconds(@{_milliseconds_value}); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes)
    #[inline]
    fn set_minutes( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<minutes_value>, <seconds_value>, <ms_value>]
        match args.len() {
            3 => js!(
                return @{self.as_ref()}.setMinutes(@{args[0]}, @{args[1]}, @{args[2]});
            ).try_into().unwrap(),
            2 => js!( return @{self.as_ref()}.setMinutes(@{args[0]}, @{args[1]}); ).try_into().unwrap(),
            1 => js!( return @{self.as_ref()}.setMinutes(@{args[0]}); ).try_into().unwrap(),
            _ => js!( return @{self.as_ref()}.setMinutes();).try_into().unwrap()
        }
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMonth)
    #[inline]
    fn set_month( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<month_value>, <day_value>]
        match args.len() {
            2 => js!( return @{self.as_ref()}.setMonth(@{args[0]}, @{args[1]}); ).try_into().unwrap(),
            1 => js!( return @{self.as_ref()}.setMonth(@{args[0]}); ).try_into().unwrap(),
            _ => js!( return @{self.as_ref()}.setMonth(); ).try_into().unwrap()
        }
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setSeconds)
    #[inline]
    fn set_seconds( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<seconds_value>, <ms_value>]
        match args.len() {
            2 => js!( return @{self.as_ref()}.setSeconds(@{args[0]}, @{args[1]}); ).try_into().unwrap(),
            1 => js!( return @{self.as_ref()}.setSeconds(@{args[0]}); ).try_into().unwrap(),
            _ => js!( return @{self.as_ref()}.setSeconds(); ).try_into().unwrap()
        }
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setTime)
    #[inline]
    fn set_time( &self, _time_value: f64 ) -> f64 {
        js!( return @{self.as_ref()}.setTime(@{_time_value}); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCDate)
    #[inline]
    fn set_utc_date( &self, _day_value: f64 ) -> f64 {
        js!( return @{self.as_ref()}.setUTCDate(@{_day_value}); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear)
    #[inline]
    fn set_utc_full_year( &self, _year_value: f64, _month_value: f64, _day_value: f64 ) -> f64 {
        js!(
            return @{self.as_ref()}.setUTCFullYear(@{_year_value}, @{_month_value}, @{_day_value});
        ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours)
    #[inline]
    fn set_utc_hours( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<hours_value>, <minutes_value>, <seconds_value>, <ms_value>]
        match args.len() {
            4 => {
                let args_obj: HashMap< String, Value > = [
                    ("hours".to_string(),   Value::Number( args[0].into())),
                    ("minutes".to_string(), Value::Number( args[1].into())),
                    ("seconds".to_string(), Value::Number( args[2].into())),
                    ("ms".to_string(),      Value::Number( args[3].into()))
                ].iter().cloned().collect();

                js!(
                    var args = @{args_obj};
                    return @{self.as_ref()}.setUTCHours(args.hours, args.minutes, args.seconds, args.ms);
                ).try_into().unwrap()
            },
            3 => js!(
                return @{self.as_ref()}.setUTCHours(@{args[0]}, @{args[1]}, @{args[2]});
            ).try_into().unwrap(),
            2 => js!( return @{self.as_ref()}.setUTCHours(@{args[0]}, @{args[1]}); ).try_into().unwrap(),
            1 => js!( return @{self.as_ref()}.setUTCHours(@{args[0]}); ).try_into().unwrap(),
            _ => js!( return @{self.as_ref()}.setUTCHours(); ).try_into().unwrap()
        }
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMilliseconds)
    #[inline]
    fn set_utc_milliseconds( &self, _milliseconds_value: f64 ) -> f64 {
        js!( return @{self.as_ref()}.setUTCMilliseconds(@{_milliseconds_value}); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes)
    #[inline]
    fn set_utc_minutes( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<minutes_value>, <seconds_value>, <ms_value>]
        match args.len() {
            3 => js!(
                return @{self.as_ref()}.setUTCMinutes(@{args[0]}, @{args[1]}, @{args[2]});
            ).try_into().unwrap(),
            2 => js!( return @{self.as_ref()}.setUTCMinutes(@{args[0]}, @{args[1]}); ).try_into().unwrap(),
            1 => js!( return @{self.as_ref()}.setUTCMinutes(@{args[0]}); ).try_into().unwrap(),
            _ => js!( return @{self.as_ref()}.setUTCMinutes(); ).try_into().unwrap()

        }
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMonth)
    #[inline]
    fn set_utc_month( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<month_value>, <day_value>]
        match args.len() {
            2 => js!( return @{self.as_ref()}.setUTCMonth(@{args[0]}, @{args[1]}); ).try_into().unwrap(),
            1 => js!( return @{self.as_ref()}.setUTCMonth(@{args[0]}); ).try_into().unwrap(),
            _ => js!( return @{self.as_ref()}.setUTCMonth(); ).try_into().unwrap()

        }
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCSeconds)
    #[inline]
    fn set_utc_seconds( &self, args: &[i32] ) -> f64 {
        // args represents:
        // [<seconds_value>, <ms_value>]
        match args.len() {
            2 => js!( return @{self.as_ref()}.setUTCSeconds(@{args[0]}, @{args[1]}); ).try_into().unwrap(),
            1 => js!( return @{self.as_ref()}.setUTCSeconds(@{args[0]}); ).try_into().unwrap(),
            _ => js!( return @{self.as_ref()}.setUTCSeconds(); ).try_into().unwrap()

        }
    }

    /// DEPRECATED
    /// fn set_year() -> f64 { }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toDateString)
    #[inline]
    fn to_date_string( &self ) -> String {
        js! ( return @{self.as_ref()}.toDateString; ).try_into().unwrap()
    }

    /// DEPRECATED
    /// fn to_gmt_string( &self ) -> f64 { }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
    #[inline]
    fn to_iso_string( &self ) -> String {
        js! ( return @{self.as_ref()}.toISOString(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toJSON)
    #[inline]
    fn to_json( &self ) -> String {
        js! ( return @{self.as_ref()}.toJSON(); ).try_into().unwrap()
    }

    /// to_locale_date_string
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleDateString)
    #[inline]
    fn to_locale_date_string( &self, _locales: Option< &str >, _options: Option< &HashMap< String, String > >) -> String {
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

    /// NON-STANDARD
    /// fn to_locale_format( &self ) -> f64 { }

    /// to_locale_string
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString)
    #[inline]
    fn to_locale_string( &self, _locales: Option< &str >, _options: Option< &HashMap< String, String > >) -> String {
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

    /// to_locale_time_string
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleTimeString)
    #[inline]
    fn to_locale_time_string( &self, _locales: Option< &str >, _options: Option< &HashMap< String, String > > ) -> String {
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

    /// NON-STANDARD
    /// fn to_source( &self ) -> f64 { }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toString)
    #[inline]
    fn toString( &self ) -> String {
        js! ( return @{self.as_ref()}.toString(); ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toTimeString)
    #[inline]
    fn to_time_string( &self ) -> String {
        js! ( return @{self.as_ref()}.toTimeString; ).try_into().unwrap()
    }

    ///
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString)
    #[inline]
    fn to_utc_string( &self ) -> String {
        js! ( return @{self.as_ref()}.toUTCString; ).try_into().unwrap()
    }

    ///
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

#[cfg(web_api_tests)]
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

    }

    #[test]
    fn test_parse() {

    }

    #[test]
    fn test_get_date() {

    }

    #[test]
    fn test_get_day() {

    }

    #[test]
    fn test_get_full_year() {

    }

    #[test]
    fn test_get_hours() {

    }

    #[test]
    fn test_get_milliseconds() {

    }

    #[test]
    fn test_get_minutes() {

    }

    #[test]
    fn test_get_month() {

    }

    #[test]
    fn test_get_seconds() {

    }

    #[test]
    fn test_get_time() {

    }

    #[test]
    fn test_get_timezone_offset() {

    }

    #[test]
    fn test_get_utc_date() {

    }

    #[test]
    fn test_get_utc_day() {

    }

    #[test]
    fn test_get_utc_full_year() {

    }

    #[test]
    fn test_utc_milliseconds() {

    }

    #[test]
    fn test_get_utc_minutes() {

    }

    #[test]
    fn test_get_utc_month() {

    }

    #[test]
    fn test_get_utc_seconds() {

    }

    #[test]
    fn test_get_year() {

    }

    #[test]
    fn test_set_date() {

    }

    #[test]
    fn test_set_full_year() {

    }

    #[test]
    fn test_set_hours() {

    }

    #[test]
    fn test_set_milliseconds() {

    }

    #[test]
    fn test_set_minutes() {

    }

    #[test]
    fn test_set_month() {

    }

    #[test]
    fn test_set_seconds() {

    }
    #[test]
    fn test_set_time() {

    }
    #[test]
    fn test_set_utc_date() {

    }
    #[test]
    fn test_set_utc_full_year() {

    }
    #[test]
    fn test_set_utc_hours() {

    }
    #[test]
    fn test_set_utc_milliseconds() {

    }
    #[test]
    fn test_set_utc_minutes() {

    }
    #[test]
    fn test_set_utc_month() {

    }
    #[test]
    fn test_set_utc_seconds() {

    }

    #[test]
    fn test_to_date_string() {

    }

    #[test]
    fn test_to_iso_string() {

    }

    #[test]
    fn test_to_json() {

    }

    #[test]
    fn test_to_locale_date_string() {

    }

    #[test]
    fn test_to_locale_string() {

    }

    #[test]
    fn test_to_locale_time_string() {

    }

    #[test]
    fn test_to_string() {

    }

    #[test]
    fn test_to_time_string() {

    }

    #[test]
    fn test_to_utc_string() {

    }

    #[test]
    fn test_value_of() {

    }
}

