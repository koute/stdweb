use webcore::value::Reference;
use webcore::try_from::TryInto;

/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/JavaScript/Reference/Global_Objects/Date)
/// https://www.ecma-international.org/ecma-262/6.0/#sec-date-constructor
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Date")]
pub struct Date( Reference );

impl Date {
    /// Creates a JavaScript Date instance that represents a single moment in time.
    /// Date objects are based on a time value that is the number of milliseconds since 1 January 1970 UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date-constructor-date
    pub fn new() -> Self {
        js!(
            return new Date();
        ).try_into().unwrap()
    }

   
    /// Creates a JavaScript Date instance that represents a single moment in time.
    /// Date objects are based on a time value that is the number of milliseconds since 1 January 1970 UTC.
    /// 
    /// year is an integer value representing the year. Values from 0 to 99 map to the years 1900 to 1999.
    /// month is an integer value representing the month, beginning with 0 for January to 11 for December
    /// day is an integer value representing the day of the month (normally from 1 to 31)
    /// hours an integer value representing the minute segment of a time
    /// seconds an integer value representing the second segment of a time
    /// milliseconds an integer value representing the millisecond segment of a time
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date-year-month-date-hours-minutes-seconds-ms
    pub fn from_datetime(year: i32, month: i32, day: i32, hours: i32, minutes: i32, seconds: i32, milliseconds: i32) -> Self {
        js!(
            return new Date(@{year}, @{month}, @{day}, @{hours}, @{minutes}, @{seconds}, @{milliseconds});
        ).try_into().unwrap()
    }

    /// Creates a JavaScript Date instance that represents a single moment in time.
    /// Date objects are based on a time value that is the number of milliseconds since 1 January 1970 UTC.
    /// 
    /// String value representing a date. The string should be in a format recognized by
    /// the Date.parse() method (IETF-compliant RFC 2822 timestamps and also a version of ISO8601).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date-value
    pub fn from_iso8601(date_string: &str) -> Self {
        js!(
            return new Date(@{date_string});
        ).try_into().unwrap()
    }
    
    /// Creates a JavaScript Date instance that represents a single moment in time.
    /// Date objects are based on a time value that is the number of milliseconds since 1 January 1970 UTC.
    /// 
    /// Integer value representing the number of milliseconds since January 1, 1970, 00:00:00 UTC,
    /// with leap seconds ignored (Unix Epoch; but consider that most Unix timestamp functions count in seconds).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date-value
    pub fn from_time(now: f64) -> Self {
        js!(
            return new Date(@{now});
        ).try_into().unwrap()
    }

    /// The Date.UTC() method accepts the same parameters as the longest form of the constructor, and
    /// returns the number of milliseconds in a Date object since January 1, 1970, 00:00:00, universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.utc
    pub fn utc(year: i32, month: i32, day: i32, hours: i32, minutes: i32, seconds: i32, milliseconds: i32) -> f64 {
        js!(
            return Date.UTC(@{year}, @{month}, @{day}, @{hours}, @{minutes}, @{seconds}, @{milliseconds});
        ).try_into().unwrap()
    }
    
    /// The Date.parse() method parses a string representation of a date, and returns the number of
    /// milliseconds since January 1, 1970, 00:00:00 UTC or NaN if the string is unrecognized or, in
    /// some cases, contains illegal date values (e.g. 2015-02-31).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.parse
    pub fn parse(date_string: &str) -> f64 {
        js!(
            return Date.parse(@{date_string});
        ).try_into().unwrap()
    }

    /// The Date.now() method returns the number of milliseconds elapsed since 1 January 1970 00:00:00 UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.now
    pub fn now() -> f64 {
        js!(
            return Date.now();
        ).try_into().unwrap()
    }
    
    /// The getDate() method returns the day of the month for the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDate)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getdate
    pub fn get_date(&self) -> i32 {
        js!(
            return @{self}.getDate();
        ).try_into().unwrap()
    }

    /// The getDay() method returns the day of the week for the specified date according to local time,
    /// where 0 represents Sunday. For the day of the month see getDate().
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getday
    pub fn get_day(&self) -> i32 {
        js!(
            return @{self}.getDay();
        ).try_into().unwrap()
    }

    /// The getFullYear() method returns the year of the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getfullyear
    pub fn get_full_year(&self) -> i32 {
        js!(
            return @{self}.getFullYear();
        ).try_into().unwrap()
    }

    /// The getHours() method returns the hour for the specified date, according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getHours)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.gethours
    pub fn get_hours(&self) -> i32 {
        js!(
            return @{self}.getHours();
        ).try_into().unwrap()
    }

    /// The getMilliseconds() method returns the milliseconds in the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMilliseconds)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getmilliseconds
    pub fn get_milliseconds(&self) -> i32 {
        js!(
            return @{self}.getMilliseconds();
        ).try_into().unwrap()
    }

    /// The getMinutes() method returns the minutes in the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMinutes)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getminutes
    pub fn get_minutes(&self) -> i32 {
        js!(
            return @{self}.getMinutes();
        ).try_into().unwrap()
    }

    /// The getMonth() method returns the month in the specified date according to local time, as a
    /// zero-based value (where zero indicates the first month of the year).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMonth)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getmonth
    pub fn get_month(&self) -> i32 {
        js!(
            return @{self}.getMonth();
        ).try_into().unwrap()
    }

    /// The getSeconds() method returns the seconds in the specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getSeconds)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getseconds
    pub fn get_seconds(&self) -> i32 {
        js!(
            return @{self}.getSeconds();
        ).try_into().unwrap()
    }

    /// The getTime() method returns the numeric value corresponding to the time for the specified
    /// date according to universal time.
    ///
    /// getTime() always uses UTC for time representation. For example, a client browser in one timezone,
    /// getTime() will be the same as a client browser in any other timezone.
    ///
    /// You can use this method to help assign a date and time to another Date object. This method is
    /// functionally equivalent to the valueOf() method.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTime)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.gettime
    pub fn get_time(&self) -> f64 {
        js!(
            return @{self}.getTime();
        ).try_into().unwrap()
    }

    /// The getTimezoneOffset() method returns the time zone difference, in minutes, from current locale (host system settings) to UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.gettimezoneoffset
    pub fn get_timezone_offset(&self) -> i32 {
        js!(
            return @{self}.getTimezoneOffset();
        ).try_into().unwrap()
    }
    
    /// The getUTCDate() method returns the day (date) of the month in the specified date according to
    /// universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDate)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getutcdate
    pub fn get_utc_date(&self) -> i32 {
        js!(
            return @{self}.getUTCDate();
        ).try_into().unwrap()
    }

    /// The getUTCDay() method returns the day of the week in the specified date according to universal
    /// time, where 0 represents Sunday.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDay)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getutcday
    pub fn get_utc_day(&self) -> i32 {
        js!(
            return @{self}.getUTCDay();
        ).try_into().unwrap()
    }

    /// The getUTCFullYear() method returns the year in the specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCFullYear)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getutcfullyear
    pub fn get_utc_full_year(&self) -> i32 {
        js!(
            return @{self}.getUTCFullYear();
        ).try_into().unwrap()
    }

    /// The getUTCHours() method returns the hours in the specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCHours)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getutchours
    pub fn get_utc_hours(&self) -> i32 {
        js!(
            return @{self}.getUTCHours();
        ).try_into().unwrap()
    }

    /// The getUTCMilliseconds() method returns the milliseconds in the specified date according to
    /// universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMilliseconds)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getutcmilliseconds
    pub fn get_utc_milliseconds(&self) -> i32 {
        js!(
            return @{self}.getUTCMilliseconds();
        ).try_into().unwrap()
    }

    /// The getUTCMinutes() method returns the minutes in the specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMinutes)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getutcminutes
    pub fn get_utc_minutes(&self) -> i32 {
        js!(
            return @{self}.getUTCMinutes();
        ).try_into().unwrap()
    }

    /// The getUTCMonth() returns the month of the specified date according to universal time, as a
    /// zero-based value (where zero indicates the first month of the year).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMonth)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getutcmonth
    pub fn get_utc_month(&self) -> i32 {
        js!(
            return @{self}.getUTCMonth();
        ).try_into().unwrap()
    }
    
    /// The getUTCSeconds() method returns the seconds in the specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCSeconds)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.getutcseconds
    pub fn get_utc_seconds(&self) -> i32 {
        js!(
            return @{self}.getUTCSeconds();
        ).try_into().unwrap()
    }

    /// The setDate() method sets the day of the Date object relative to the beginning of the currently set month.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setDate)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setdate
    pub fn set_date(&self, date: i32) {
        js!{ @(no_return)
            @{self}.setDate(@{date});
        }
    }

    /// The setFullYear() method sets the full year for a specified date according to local time. Returns new timestamp.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setfullyear
    pub fn set_full_year(&self, full_year: i32) {
        js!{ @(no_return)
            @{self}.setFullYear(@{full_year});
        }
    }

    /// The setHours() method sets the hours for a specified date according to local time, and returns the number of milliseconds
    /// since January 1, 1970 00:00:00 UTC until the time represented by the updated Date instance.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.sethours
    pub fn set_hours(&self, hours: i32) {
        js!{ @(no_return)
            @{self}.setHours(@{hours});
        }
    }

    /// The setMilliseconds() method sets the milliseconds for a specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMilliseconds)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setmilliseconds
    pub fn set_milliseconds(&self, milliseconds: i32) {
        js!{ @(no_return)
            @{self}.setMilliseconds(@{milliseconds});
        }
    }

    /// The setMinutes() method sets the minutes for a specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setminutes
    pub fn set_minutes(&self, minutes: i32) {
        js!{ @(no_return)
            @{self}.setMinutes(@{minutes});
        }
    }

    /// The setMonth() method sets the month for a specified date according to the currently set year.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMonth)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setmonth
    pub fn set_month(&self, month: i32) {
        js!{ @(no_return)
            @{self}.setMonth(@{month});
        }
    }

    /// The setSeconds() method sets the seconds for a specified date according to local time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setSeconds)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setseconds
    pub fn set_seconds(&self, seconds: i32) {
        js!{ @(no_return)
            @{self}.setSeconds(@{seconds});
        }
    }

    /// The setTime() method sets the Date object to the time represented by a number of milliseconds since
    /// January 1, 1970, 00:00:00 UTC.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setTime)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.settime
    pub fn set_time(&self, time: f64) {
        js!{ @(no_return)
            @{self}.setTime(@{time});
        }
    }

    /// The setUTCDate() method sets the day of the month for a specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCDate)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setutcdate
    pub fn set_utc_date(&self, date: i32) {
        js!{ @(no_return)
            @{self}.setUTCDate(@{date});
        }
    }

    /// The setUTCFullYear() method sets the full year for a specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setutcfullyear
    pub fn set_utc_full_year(&self, full_year: i32) {
        js!{ @(no_return)
            @{self}.setUTCFullYear(@{full_year});
        }
    }

    /// The setUTCHours() method sets the hour for a specified date according to universal time, and returns the number
    /// of milliseconds since  January 1, 1970 00:00:00 UTC until the time represented by the updated Date instance.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setutchours
    pub fn set_utc_hours(&self, hours: i32) {
        js!{ @(no_return)
            @{self}.setUTCHours(@{hours});
        }
    }

    /// The setUTCMilliseconds() method sets the milliseconds for a specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMilliseconds)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setutcmilliseconds
    pub fn set_utc_milliseconds(&self, milliseconds: i32) {
        js!{ @(no_return)
            @{self}.setUTCMilliseconds(@{milliseconds});
        }
    }

    /// The setUTCMinutes() method sets the minutes for a specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setutcminutes
    pub fn set_utc_minutes(&self, minutes: i32) {
        js!{ @(no_return)
            @{self}.setUTCMinutes(@{minutes});
        }
    }

    /// The setUTCMonth() method sets the month for a specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMonth)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setutcmonth
    pub fn set_utc_month(&self, month: i32) {
        js!{ @(no_return)
            @{self}.setUTCMonth(@{month});
        }
    }

    /// The setUTCSeconds() method sets the seconds for a specified date according to universal time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCSeconds)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.setutcseconds
    pub fn set_utc_seconds(&self, seconds: i32) {
        js!{ @(no_return)
            @{self}.setUTCSeconds(@{seconds});
        }
    }

    /// The toDateString() method returns the date portion of a Date object in human readable form in American English.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.todatestring
    #[inline]
    pub fn to_date_string(&self) -> String {
        js!(
            return @{self}.toDateString();
        ).try_into().unwrap()
    }

    /// The toISOString() method returns a string in simplified extended ISO format (ISO 8601), which is always 24 or 27
    /// characters long (YYYY-MM-DDTHH:mm:ss.sssZ or ±YYYYYY-MM-DDTHH:mm:ss.sssZ, respectively). The timezone is always zero
    /// UTC offset, as denoted by the suffix "Z".
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.toisostring
    #[inline]
    pub fn to_iso_string(&self) -> String {
        js!(
            return @{self}.toISOString();
        ).try_into().unwrap()
    }

    /// The toJSON() method returns a string representation of the Date object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toJSON)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.tojson
    #[inline]
    pub fn to_json(&self) -> String {
        js!(
            return @{self}.toJSON();
        ).try_into().unwrap()
    }

    /// The toString() method returns a string representing the specified Date object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toString)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.tostring
    #[inline]
    pub fn to_string(&self) -> String {
        js!(
            return @{self}.toString();
        ).try_into().unwrap()
    }

    /// The toTimeString() method returns the time portion of a Date object in human readable form in American English.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toTimeString)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.totimestring
    #[inline]
    pub fn to_time_string(&self) -> String {
        js!(
            return @{self}.toTimeString();
        ).try_into().unwrap()
    }

    /// The toUTCString() method converts a date to a string, using the UTC time zone.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.toutcstring
    #[inline]
    pub fn to_utc_string(&self) -> String {
        js!(
            return @{self}.toUTCString();
        ).try_into().unwrap()
    }

    /// The valueOf() method returns the primitive value of a Date object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/valueOf)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype.valueof
    pub fn value_of(&self) -> f64 {
        js!(
            return @{self}.valueOf();
        ).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_now() {
        let now = Date::now();
        assert!( now > 0.0 );
    }

    #[test]
    fn test_date_utc() {
        let now = Date::utc(96, 1, 2, 3, 4, 5, 0);
        assert_eq!(now, 823230245000.0);

        let now = Date::utc(0, 0, 0, 0, 0, 0, 0);
        assert_eq!(now, -2209075200000.0);
    }

    #[test]
    fn test_date_parse() {
        let now = Date::parse("01 Jan 1970 00:00:00 GMT");
        assert_eq!(now, 0.0);

        let now = Date::parse("04 Dec 1995 00:12:00 GMT");
        assert_eq!(now, 818035920000.0);
    }

    #[test]
    fn test_date_get_date() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        assert_eq!( now.get_date(), 19);
    }

    #[test]
    fn test_date_get_day() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        assert_eq!(now.get_day(), 2);
    }

    #[test]
    fn test_date_get_full_year() {
        let now = Date::from_iso8601("August 19, 75 23:15:30");
        assert_eq!(now.get_full_year(), 1975);
    }

    #[test]
    fn test_date_get_hours() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        assert_eq!(now.get_hours(), 23);
    }

    #[test]
    fn test_date_get_milliseconds() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_milliseconds(123);
        assert_eq!(now.get_milliseconds(), 123);
    }

    #[test]
    fn test_date_get_minutes() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        assert_eq!(now.get_minutes(), 15);
    }

    #[test]
    fn test_date_get_month() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        assert_eq!(now.get_month(), 7);
    }

    #[test]
    fn test_date_get_seconds() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        assert_eq!(now.get_seconds(), 30);
    }

    #[test]
    fn test_date_get_time() {
        let now = Date::from_iso8601("July 20, 69 00:20:18 GMT+00:00");
        assert_eq!(now.get_time(), -14254782000.0);
    }

    #[test]
    fn test_date_get_timezone_offset() {
        // this is impossible to test like this, since this function depends on local time only.
        // and there is no easy way to mock the system time, so the only real thing to check
        // is that two dates return the same timezone offset.
        let t1 = Date::from_iso8601("August 19, 1975 23:15:30 GMT+07:00");
        let t2 = Date::from_iso8601("August 19, 1975 23:15:30 GMT-02:00");
        assert_eq!(t1.get_timezone_offset(), t2.get_timezone_offset());
    }

    #[test]
    fn test_date_get_utc_date() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30 GMT+11:00");
        assert_eq!(now.get_utc_date(), 19);
        let now = Date::from_iso8601("August 19, 1975 23:15:30 GMT-11:00");
        assert_eq!(now.get_utc_date(), 20);
    }

    #[test]
    fn test_date_get_utc_day() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30 GMT+11:00");
        assert_eq!( now.get_utc_day(), 2 );
        let now = Date::from_iso8601("August 19, 1975 23:15:30 GMT-11:00");
        assert_eq!( now.get_utc_day(), 3 );
    }

    #[test]
    fn test_date_get_utc_full_year() {
        let now = Date::from_iso8601("December 31, 1975 23:15:30 GMT+11:00");
        assert_eq!(now.get_utc_full_year(), 1975 );
        let now = Date::from_iso8601("December 31, 1975 23:15:30 GMT-11:00");
        assert_eq!(now.get_utc_full_year(), 1976 );
    }

    #[test]
    fn test_date_get_utc_milliseconds() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_milliseconds(123);
        assert_eq!(now.get_utc_milliseconds(), 123);
    }

    #[test]
    fn test_date_get_utc_minutes() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        assert_eq!(now.get_utc_minutes(), 15);
    }

    #[test]
    fn test_date_get_utc_month() {
        let now = Date::from_iso8601("December 31, 1975 23:15:30 GMT+11:00");
        assert_eq!(now.get_utc_month(), 11);
        let now = Date::from_iso8601("December 31, 1975 23:15:30 GMT-11:00");
        assert_eq!(now.get_utc_month(), 0);
    }

    #[test]
    fn test_date_set_date() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_date(3);
        assert_eq!(now.get_date(), 3);
    }

    #[test]
    fn test_date_set_full_year() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_full_year(1969);
        assert_eq!(now.get_full_year(), 1969);
    }

    #[test]
    fn test_date_set_hours() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_hours(15);
        assert_eq!(now.get_hours(), 15);
    }

    #[test]
    fn test_date_set_milliseconds() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_milliseconds(123);
        assert_eq!(now.get_milliseconds(), 123);
    }

    #[test]
    fn test_date_set_minutes() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_minutes(42);
        assert_eq!(now.get_minutes(), 42);
    }

    #[test]
    fn test_date_set_month() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_month(9);
        assert_eq!(now.get_month(), 9);
    }

    #[test]
    fn test_date_set_seconds() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_seconds(59);
        assert_eq!(now.get_seconds(), 59);
    }

    #[test]
    fn test_date_set_time() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_time(818035920000.0);
        assert_eq!(now.to_utc_string(), "Mon, 04 Dec 1995 00:12:00 GMT");
    }

    #[test]
    fn test_date_set_utc_date() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_utc_date(3);
        assert_eq!(now.get_utc_date(), 3);
    }

    #[test]
    fn test_date_set_utc_full_year() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_utc_full_year(1969);
        assert_eq!(now.get_utc_full_year(), 1969);
    }

    #[test]
    fn test_date_set_utc_hours() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_utc_hours(15);
        assert_eq!(now.get_utc_hours(), 15);
    }

    #[test]
    fn test_date_set_utc_milliseconds() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_utc_milliseconds(123);
        assert_eq!(now.get_utc_milliseconds(), 123);
    }

    #[test]
    fn test_date_set_utc_minutes() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_utc_minutes(42);
        assert_eq!(now.get_utc_minutes(), 42);
    }

    #[test]
    fn test_date_set_utc_month() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_utc_month(9);
        assert_eq!(now.get_utc_month(), 9);
    }

    #[test]
    fn test_date_set_utc_seconds() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30");
        now.set_utc_seconds(59);
        assert_eq!(now.get_utc_seconds(), 59);
    }

    #[test]
    fn test_date_to_date_string() {
        let now = Date::from_datetime(1993, 6, 28, 14, 39, 7, 0);
        assert_eq!(now.to_date_string(), "Wed Jul 28 1993");
    }

    #[test]
    fn test_date_to_iso_string() {
        let now = Date::from_iso8601("05 October 2011 14:48 UTC");
        assert_eq!(now.to_iso_string(), "2011-10-05T14:48:00.000Z");
    }

    #[test]
    fn test_date_to_json() {
        let now = Date::from_iso8601("August 19, 1975 23:15:30 UTC");
        assert_eq!(now.to_iso_string(), "1975-08-19T23:15:30.000Z");
    }

    #[test]
    fn test_date_to_time_string() {
        // not easy to test this due to time-zones
    }

    #[test]
    fn test_date_to_utc_string() {
        let now = Date::from_time(Date::utc(96, 1, 2, 3, 4, 5, 0));
        assert_eq!(now.to_utc_string(), "Fri, 02 Feb 1996 03:04:05 GMT");
    }

    #[test]
    fn test_date_value_of() {
        let now = Date::from_time(Date::utc(96, 1, 2, 3, 4, 5, 0));
        assert_eq!(now.value_of(), 823230245000.0);
    }

}