// TODO: Handle NaN and Infinity.

use std::{i8, i16, i32, i64, u8, u16, u32, u64, usize, f32, f64};
use std::error;
use std::fmt;
use webcore::try_from::TryFrom;

// 2^53 - 1
const MAX_SAFE_INTEGER_F64: i64 = 9007199254740991;
const MIN_SAFE_INTEGER_F64: i64 = -9007199254740991;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Storage {
    // Technically JavaScript only has f64 numbers, however at least the V8
    // treats numbers which can be represented as 31-bit integers more optimally.
    //
    // Now, I have absolutely no idea if doing this is worth it as opposed to
    // just sticking with always using f64; it's definitely worth investigating
    // in the future.
    I32( i32 ),
    F64( f64 )
}

/// A type representing a JavaScript number.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Number( Storage );

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConversionError {
    OutOfRange,
    NotAnInteger
}

impl fmt::Display for ConversionError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        let message = error::Error::description( self );
        write!( formatter, "{}", message )
    }
}

impl error::Error for ConversionError {
    fn description( &self ) -> &str {
        match *self {
            ConversionError::OutOfRange => "number out of range",
            ConversionError::NotAnInteger => "number not an integer"
        }
    }
}

// We don't want to make the inner value public, hence this accessor.
#[inline]
pub fn get_storage( number: &Number ) -> &Storage {
    &number.0
}

impl AsRef< Number > for Number {
    #[inline]
    fn as_ref( &self ) -> &Self {
        self
    }
}

impl From< i8 > for Number {
    #[inline]
    fn from( value: i8 ) -> Self {
        Number( Storage::I32( value as i32 ) )
    }
}

impl From< i16 > for Number {
    #[inline]
    fn from( value: i16 ) -> Self {
        Number( Storage::I32( value as i32 ) )
    }
}

impl From< i32 > for Number {
    #[inline]
    fn from( value: i32 ) -> Self {
        Number( Storage::I32( value ) )
    }
}

impl TryFrom< i64 > for Number {
    type Error = ConversionError;

    fn try_from( value: i64 ) -> Result< Self, Self::Error > {
        if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            Ok( Number( Storage::I32( value as i32 ) ) )
        } else if value >= MIN_SAFE_INTEGER_F64 && value <= MAX_SAFE_INTEGER_F64 {
            Ok( Number( Storage::F64( value as f64 ) ) )
        } else {
            Err( ConversionError::OutOfRange )
        }
    }
}

impl From< u8 > for Number {
    #[inline]
    fn from( value: u8 ) -> Self {
        Number( Storage::I32( value as i32 ) )
    }
}

impl From< u16 > for Number {
    #[inline]
    fn from( value: u16 ) -> Self {
        Number( Storage::I32( value as i32 ) )
    }
}

impl From< u32 > for Number {
    #[inline]
    fn from( value: u32 ) -> Self {
        if value <= i32::MAX as u32 {
            Number( Storage::I32( value as i32 ) )
        } else {
            Number( Storage::F64( value as f64 ) )
        }
    }
}

impl TryFrom< u64 > for Number {
    type Error = ConversionError;

    fn try_from( value: u64 ) -> Result< Self, Self::Error > {
        if value <= i32::MAX as u64 {
            Ok( Number( Storage::I32( value as i32 ) ) )
        } else if value <= MAX_SAFE_INTEGER_F64 as u64 {
            Ok( Number( Storage::F64( value as f64 ) ) )
        } else {
            Err( ConversionError::OutOfRange )
        }
    }
}

// Since technically `usize` can be 64-bit we have to do this.
impl TryFrom< usize > for Number {
    type Error = ConversionError;

    fn try_from( value: usize ) -> Result< Self, Self::Error > {
        if value <= i32::MAX as usize {
            Ok( Number( Storage::I32( value as i32 ) ) )
        } else if value <= MAX_SAFE_INTEGER_F64 as usize {
            Ok( Number( Storage::F64( value as f64 ) ) )
        } else {
            Err( ConversionError::OutOfRange )
        }
    }
}

impl From< f32 > for Number {
    #[inline]
    fn from( value: f32 ) -> Self {
        Number( Storage::F64( value as f64 ) )
    }
}

impl From< f64 > for Number {
    #[inline]
    fn from( value: f64 ) -> Self {
        Number( Storage::F64( value ) )
    }
}

impl From< Number > for f64 {
    #[inline]
    fn from( number: Number ) -> Self {
        match number.0 {
            Storage::I32( value ) => value as f64,
            Storage::F64( value ) => value
        }
    }
}

macro_rules! impl_trivial_try_from {
    ($($kind:ty),+) => {
        $(
            impl TryFrom< $kind > for Number {
                type Error = $crate::unstable::Void;

                #[inline]
                fn try_from( value: $kind ) -> Result< Self, Self::Error > {
                    Ok( value.into() )
                }
            }
        )+
    }
}

impl_trivial_try_from!( i8, i16, i32, u8, u16, u32, f32, f64 );

macro_rules! impl_conversion_into_rust_types {
    ($(into $($kind:tt),+: { from i32: $integer_callback:ident, from f64: $float_callback:ident }),+) => {
        $(
            $(
                impl TryFrom< Number > for $kind {
                    type Error = ConversionError;

                    #[allow(trivial_numeric_casts)]
                    fn try_from( number: Number ) -> Result< Self, Self::Error > {
                        match number.0 {
                            Storage::I32( value ) => {
                                $integer_callback!( value, $kind )
                            },
                            Storage::F64( value ) => {
                                $float_callback!( value, $kind )
                            }
                        }
                    }
                }
            )+
        )+
    }
}

macro_rules! i32_to_small_integer {
    ($value:expr, $kind:tt) => {
        if $value <= $kind::MAX as i32 && $value >= $kind::MIN as i32 {
            Ok( $value as $kind )
        } else {
            Err( ConversionError::OutOfRange )
        }
    }
}

macro_rules! direct_cast {
    ($value:expr, $kind:tt) => {
        Ok( $value as $kind )
    }
}

macro_rules! i32_to_big_unsigned_integer {
    ($value:expr, $kind:tt) => {
        if $value >= 0 {
            Ok( $value as $kind )
        } else {
            Err( ConversionError::OutOfRange )
        }
    }
}

macro_rules! f64_to_integer {
    ($value:expr, $kind:tt) => {{
        if $value.floor() != $value {
            return Err( ConversionError::NotAnInteger );
        }

        if $value <= $kind::MAX as f64 && $value >= $kind::MIN as f64 {
            Ok( $value as $kind )
        } else {
            Err( ConversionError::OutOfRange )
        }
    }}
}

impl_conversion_into_rust_types! {
    into i8, i16, i32, u8, u16: {
        from i32: i32_to_small_integer,
        from f64: f64_to_integer
    },
    into i64: {
        from i32: direct_cast,
        from f64: f64_to_integer
    },
    into u32, u64, usize: {
        from i32: i32_to_big_unsigned_integer,
        from f64: f64_to_integer
    },
    into f64: {
        from i32: direct_cast,
        from f64: direct_cast
    }
}

impl PartialEq< i8 > for Number {
    #[inline]
    fn eq( &self, right: &i8 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left == *right as i32,
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< i16 > for Number {
    #[inline]
    fn eq( &self, right: &i16 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left == *right as i32,
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< i32 > for Number {
    #[inline]
    fn eq( &self, right: &i32 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left == *right,
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< i64 > for Number {
    #[inline]
    fn eq( &self, right: &i64 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left as i64 == *right,
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< u8 > for Number {
    #[inline]
    fn eq( &self, right: &u8 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left == *right as i32,
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< u16 > for Number {
    #[inline]
    fn eq( &self, right: &u16 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left == *right as i32,
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< u32 > for Number {
    #[inline]
    fn eq( &self, right: &u32 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left as i64 == *right as i64,
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< u64 > for Number {
    #[inline]
    fn eq( &self, right: &u64 ) -> bool {
        match self.0 {
            Storage::I32( left ) => {
                if *right >= i32::MAX as u64 {
                    // The right side is not convertible to an i32.
                    return false;
                }
                left == *right as i32
            },
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< usize > for Number {
    #[inline]
    fn eq( &self, right: &usize ) -> bool {
        match self.0 {
            Storage::I32( left ) => {
                if *right >= i32::MAX as usize {
                    // The right side is not convertible to an i32.
                    return false;
                }
                left == *right as i32
            },
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< f32 > for Number {
    #[inline]
    fn eq( &self, right: &f32 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left as f64 == *right as f64,
            Storage::F64( left ) => left == *right as f64
        }
    }
}

impl PartialEq< f64 > for Number {
    #[inline]
    fn eq( &self, right: &f64 ) -> bool {
        match self.0 {
            Storage::I32( left ) => left as f64 == *right,
            Storage::F64( left ) => left == *right
        }
    }
}

macro_rules! impl_symmetric_partial_eq {
    ( $($kind:tt),+ ) => {
        $(
            impl PartialEq< Number > for $kind {
                #[inline]
                fn eq( &self, right: &Number ) -> bool {
                    right == self
                }
            }
        )+
    }
}

impl_symmetric_partial_eq! {
    u8, u16, u32, u64,
    usize,
    i8, i16, i32, i64,
    f32, f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::u8;

    // 2^23 - 1
    const MAX_SAFE_INTEGER_F32: i32 = 8388607;
    const MIN_SAFE_INTEGER_F32: i32 = -8388607;

    trait ExampleValues: Sized {
        fn example_values() -> Vec< Self >;
    }

    macro_rules! example_values {
        (
            $($kind:tt => [$($value:expr),+]),+
        ) => {
            $(
                impl ExampleValues for $kind {
                    fn example_values() -> Vec< Self > { vec![ $($value),+ ] }
                }
            )+
        }
    }

    example_values! {
         u8 => [ 0, 1,  u8::MAX - 1,  u8::MAX ],
        u16 => [ 0, 1, u16::MAX - 1, u16::MAX ],
        u32 => [ 0, 1, u32::MAX - 1, u32::MAX ],
        u64 => [ 0, 1, u64::MAX - 1, u64::MAX ],
      usize => [ 0, 1, usize::MAX - 1, usize::MAX ],
         i8 => [  i8::MIN,  i8::MIN + 1, -1, 0, 1,  i8::MAX - 1,  i8::MAX ],
        i16 => [ i16::MIN, i16::MIN + 1, -1, 0, 1, i16::MAX - 1, i16::MAX ],
        i32 => [ i32::MIN, i32::MIN + 1, -1, 0, 1, i32::MAX - 1, i32::MAX ],
        i64 => [ i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX ],
        f32 => [
            f32::MIN, f32::MIN + 1.0, -1.0, 0.0, 1.0, f32::MAX - 1.0, f32::MAX,
            -0.33, 0.33, -3.33, 3.33,
            MIN_SAFE_INTEGER_F32 as f32,
            MIN_SAFE_INTEGER_F32 as f32 - 100.0,
            MAX_SAFE_INTEGER_F32 as f32,
            MAX_SAFE_INTEGER_F32 as f32 + 100.0
        ],
        f64 => [
            f64::MIN, f64::MIN + 1.0, -1.0, 0.0, 1.0, f64::MAX - 1.0, f64::MAX,
            -0.33, 0.33, -3.33, 3.33,
            MIN_SAFE_INTEGER_F32 as f64,
            MIN_SAFE_INTEGER_F32 as f64 - 100.0,
            MAX_SAFE_INTEGER_F32 as f64,
            MAX_SAFE_INTEGER_F32 as f64 + 100.0,
            MIN_SAFE_INTEGER_F64 as f64,
            MIN_SAFE_INTEGER_F64 as f64 - 100.0,
            MAX_SAFE_INTEGER_F64 as f64,
            MAX_SAFE_INTEGER_F64 as f64 + 100.0
        ]
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    enum Kind {
        U,
        I,
        F
    }

    macro_rules! type_kind {
        (u8) => (U);
        (u16) => (U);
        (u32) => (U);
        (u64) => (U);
        (usize) => (U);
        (i8) => (I);
        (i16) => (I);
        (i32) => (I);
        (i64) => (I);
        (f32) => (F);
        (f64) => (F);
    }

    macro_rules! is_convertible {
        ($value:expr, $src_type:tt => $dst_type:tt) => {{
            let src_size = mem::size_of::< $src_type >();
            let dst_size = mem::size_of::< $dst_type >();
            let src_kind = type_kind!( $src_type );
            let dst_kind = type_kind!( $dst_type );

            use self::Kind::*;
            match (src_kind, dst_kind) {
                (F, F) => {
                    if dst_size >= src_size {
                        true
                    } else /* dst_size < src_size */ {
                        let value = $value as f64;
                        let in_range = match dst_size {
                            4 => value <= MAX_SAFE_INTEGER_F32 as f64 && value >= MIN_SAFE_INTEGER_F32 as f64,
                            _ => unreachable!()
                        };

                        in_range && ($value as $dst_type) as $src_type == $value
                    }
                },
                (U, U) | (I, I) => {
                    if dst_size >= src_size {
                        true
                    } else /* dst_size < src_size */ {
                        $value >= ($dst_type::MIN as $src_type) &&
                        $value <= ($dst_type::MAX as $src_type)
                    }
                },
                (U, I) => {
                    if dst_size > src_size {
                        true
                    } else /* dst_size <= src_size */ {
                        $value <= ($dst_type::MAX as $src_type)
                    }
                },
                (I, U) => {
                    if $value < (0 as $src_type) {
                        false
                    } else if dst_size >= src_size {
                        true
                    } else /* dst_size < src_size */ {
                        $value <= ($dst_type::MAX as $src_type)
                    }
                },
                (F, U) => {
                    ($value as f64).floor() == ($value as f64) &&
                    ($value as f64) >= ($dst_type::MIN as f64) &&
                    ($value as f64) <= ($dst_type::MAX as f64)
                },
                (F, I) => {
                    ($value as f64).floor() == ($value as f64) &&
                    ($value as f64) >= ($dst_type::MIN as f64) &&
                    ($value as f64) <= ($dst_type::MAX as f64)
                },
                (I, F) => {
                    let value = $value as i64;
                    match dst_size {
                        4 => value <= MAX_SAFE_INTEGER_F32 as i64 && value >= MIN_SAFE_INTEGER_F32 as i64,
                        8 => value <= MAX_SAFE_INTEGER_F64 as i64 && value >= MIN_SAFE_INTEGER_F64 as i64,
                        _ => unreachable!()
                    }
                },
                (U, F) => {
                    let value = $value as u64;
                    match dst_size {
                        4 => value <= MAX_SAFE_INTEGER_F32 as u64,
                        8 => value <= MAX_SAFE_INTEGER_F64 as u64,
                        _ => unreachable!()
                    }
                }
            }
        }}
    }

    macro_rules! conversion_test_body {
        ($value:expr, $src_type:tt, $dst_type:tt) => {{
            let is_convertible_into_number = is_convertible!( $value, $src_type => f64 );
            let number: Result< Number, _ > = $value.try_into();
            let number = match number {
                Ok( number ) => {
                    if !is_convertible_into_number {
                        panic!( "Type {} should NOT be convertible into Number yet it is: {:?}", stringify!( $src_type ), $value );
                    }
                    number
                },
                Err( _ ) => {
                    if is_convertible_into_number {
                        panic!( "Type {} should be convertible into Number yet it isn't: {:?}", stringify!( $src_type ), $value );
                    }
                    return;
                }
            };

            let is_convertible = is_convertible!( $value, $src_type => $dst_type );
            let output = number.try_into();
            if is_convertible {
                let result = output == Ok( $value as $dst_type );
                assert!( result, "Conversion should succeed yet it didn't for {:?}", $value );
            } else {
                let result = output.is_err();
                assert!( result, "Conversion should NOT succeed yet it did for {:?}", $value );
            };
        }}
    }

    macro_rules! conversion_test {
        ($src_type:tt, $(($dst_type:tt, $test_name:ident)),+) => {
            $(
                #[allow(trivial_numeric_casts, const_err, unreachable_patterns)]
                #[test]
                fn $test_name() {
                    for value in $src_type::example_values() {
                        conversion_test_body!( value, $src_type, $dst_type );
                    }
                }
            )+
        }
    }

    macro_rules! generate_conversion_tests {
        ($raw_type:tt) => {
            conversion_test! {
                $raw_type,
                (u8, into_u8),
                (u16, into_u16),
                (u32, into_u32),
                (u64, into_u64),
                (usize, into_usize),
                (i8, into_i8),
                (i16, into_i16),
                (i32, into_i32),
                (i64, into_i64),
                // No conversion to f32.
                (f64, into_f64)
            }
        }
    }

    macro_rules! generate_number_tests {
        ($(($raw_type:tt, $name:ident)),+) => {
            $(
                mod $name {
                    use super::*;
                    use webcore::try_from::TryInto;

                    #[test]
                    fn round_trip() {
                        for left in $raw_type::example_values() {
                            let number: Number = left.into();
                            let right: $raw_type = number.try_into().unwrap();
                            assert!( left == right, "Failed for: {:?}", left );
                        }
                    }

                    #[test]
                    fn equality() {
                        for value in $raw_type::example_values() {
                            let number: Number = value.into();
                            assert!( number == value, "Failed for: {:?}", value );
                            assert!( value == number, "Failed for: {:?}", value );
                        }
                    }

                    generate_conversion_tests! { $raw_type }
                }
            )+
        }
    }

    generate_number_tests! {
        (u8, for_u8),
        (u16, for_u16),
        (u32, for_u32),
        (i8, for_i8),
        (i16, for_i16),
        (i32, for_i32),
        (f64, for_f64)
    }

    mod for_f32 {
        use super::*;
        use webcore::try_from::TryInto;
        generate_conversion_tests! { f32 }
    }

    mod for_u64 {
        use super::*;
        use webcore::try_from::TryInto;
        generate_conversion_tests! { u64 }
    }

    mod for_usize {
        use super::*;
        use webcore::try_from::TryInto;
        generate_conversion_tests! { usize }
    }

    mod for_i64 {
        use super::*;
        use webcore::try_from::TryInto;
        generate_conversion_tests! { u64 }
    }

    #[test]
    fn test_number_into_f64() {
        assert_eq!(f64::from(Number(Storage::F64(7.))), 7.);
        assert_eq!(f64::from(Number(Storage::I32(7 ))), 7.);
        assert_eq!({ let x : f64 = Number(Storage::F64(7.)).into(); x }, 7.);
        assert_eq!({ let x : f64 = Number(Storage::I32(7 )).into(); x }, 7.);
    }
}
