use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::fmt;
use std::error;
use std::mem;
use std::borrow::Cow;
use webcore::void::Void;
use webcore::try_from::{TryFrom, TryInto};
use webcore::number::{self, Number};
use webcore::object::Object;
use webcore::array::Array;
use webcore::serialization::JsSerialize;
use webcore::reference_type::ReferenceType;
use webcore::instance_of::InstanceOf;
use webcore::symbol::Symbol;
use webcore::type_name::type_name_opt;
use webapi::error::TypeError;

/// A unit type representing JavaScript's `undefined`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Undefined;

/// A unit type representing JavaScript's `null`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Null;

/// A type representing a reference to a JavaScript value.
#[repr(C)]
#[derive(Debug)]
pub struct Reference( i32 );

impl Reference {
    #[doc(hidden)]
    #[inline]
    pub unsafe fn from_raw_unchecked( refid: i32 ) -> Reference {
        __js_raw_asm!( "Module.STDWEB_PRIVATE.increment_refcount( $0 );", refid );
        Reference( refid )
    }

    #[doc(hidden)]
    #[inline]
    pub(crate) unsafe fn from_raw_unchecked_noref( refid: i32 ) -> Reference {
        Reference( refid )
    }

    #[doc(hidden)]
    #[inline]
    pub fn as_raw( &self ) -> i32 {
        self.0
    }

    /// Converts this reference into the given type `T`; checks whenever the reference
    /// is really of type `T` and returns `None` if it's not.
    #[inline]
    pub fn downcast< T: ReferenceType >( self ) -> Option< T > {
        if T::instance_of( &self ) {
            Some( unsafe { T::from_reference_unchecked( self ) } )
        } else {
            None
        }
    }
}

impl PartialEq for Reference {
    #[inline]
    fn eq( &self, other: &Reference ) -> bool {
        let result = self.0 == other.0;

        debug_assert_eq!( {
            let real_result: bool = js!( return @{self} === @{other}; ).try_into().unwrap();
            real_result
        }, result );

        result
    }
}

impl Eq for Reference {}

impl Clone for Reference {
    #[inline]
    fn clone( &self ) -> Self {
        unsafe {
            Reference::from_raw_unchecked( self.as_raw() )
        }
    }
}

impl Drop for Reference {
    #[inline]
    fn drop( &mut self ) {
        __js_raw_asm!( "Module.STDWEB_PRIVATE.decrement_refcount( $0 );", self.0 );
    }
}

impl AsRef< Reference > for Reference {
    #[inline]
    fn as_ref( &self ) -> &Self {
        self
    }
}

macro_rules! __impl_infallible_try_from {
    (($($impl_arg:tt)*) ($($src_arg:tt)*) ($($dst_arg:tt)*) ($($bounds:tt)*)) => {
        impl< $($impl_arg)* > TryFrom< $($src_arg)* > for $($dst_arg)* where $($bounds)* {
            type Error = $crate::unstable::Void;

            #[inline]
            fn try_from( source: $($src_arg)* ) -> Result< Self, Self::Error > {
                Ok( source.into() )
            }
        }
    };
}

macro_rules! impl_infallible_try_from {
    (impl< $($impl_arg:tt),* > for $src:ty => $dst:ty where ($($bounds:tt)*); $($rest:tt)*) => {
        __impl_infallible_try_from!( ($($impl_arg),*) ($src) ($dst) ($($bounds)*) );
        impl_infallible_try_from!( $($rest)* );
    };

    (impl< $($impl_arg:tt),* > for $src:ty => $dst:ty; $($rest:tt)*) => {
        __impl_infallible_try_from!( ($($impl_arg),*) ($src) ($dst) () );
        impl_infallible_try_from!( $($rest)* );
    };

    ($src:ty => $dst:ty; $($rest:tt)*) => {
        __impl_infallible_try_from!( () ($src) ($dst) () );
        impl_infallible_try_from!( $($rest)* );

    };

    () => {};
}

impl_infallible_try_from! {
    Reference => Reference;
    impl< 'a > for &'a Reference => &'a Reference;
}

/// A type representing a JavaScript value.
///
/// This type implements a rich set of conversions
/// from and into standard Rust types, for example:
///
/// ```rust
/// let v1: Value = "Hello world!".into();
/// let v2: Value = true.into();
/// let v3: Value = vec![ 1, 2, 3 ].into();
/// let v4: Value = Null.into();
/// let v5: Value = 123_u64.try_into().unwrap();
///
/// let v1_r: String = v1.try_into().unwrap();
/// let v2_r: bool = v2.try_into().unwrap();
/// let v3_r: Vec< i32 > = v3.try_into().unwrap();
/// let v4_r: Option< String > = v4.try_into().unwrap(); // Will be `None`.
/// let v5_r: u64 = v5.try_into().unwrap();
/// ```
#[allow(missing_docs)]
#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Undefined,
    Null,
    Bool( bool ),
    Number( Number ),
    Symbol( Symbol ),
    String( String ),
    Reference( Reference )
}

impl Value {
    /// Checks whenever the Value is of the Null variant.
    #[inline]
    pub fn is_null( &self ) -> bool {
        if let Value::Null = *self {
            true
        } else {
            false
        }
    }

    /// Checks whenever the Value is of the Symbol variant.
    #[inline]
    pub fn is_symbol( &self ) -> bool {
        if let Value::Symbol( _ ) = *self {
            true
        } else {
            false
        }
    }

    /// Checks whenever the Value is of the Reference variant.
    #[inline]
    pub fn is_reference( &self ) -> bool {
        if let Value::Reference( _ ) = *self {
            true
        } else {
            false
        }
    }

    /// Checks whenever the Value is a reference to an `Object`.
    #[inline]
    pub fn is_object( &self ) -> bool {
        if let Value::Reference( ref reference ) = *self {
            Object::instance_of( reference )
        } else {
            false
        }
    }

    /// Checks whenever the Value is a reference to an `Array`.
    #[inline]
    pub fn is_array( &self ) -> bool {
        if let Value::Reference( ref reference ) = *self {
            Array::instance_of( reference )
        } else {
            false
        }
    }

    /// Gets a reference to the [Reference](struct.Reference.html) inside this `Value`.
    #[inline]
    pub fn as_reference( &self ) -> Option< &Reference > {
        match *self {
            Value::Reference( ref reference ) => Some( reference ),
            _ => None
        }
    }

    /// Gets a reference to the [Object](struct.Object.html) inside this `Value`.
    #[inline]
    pub fn as_object( &self ) -> Option< &Object > {
        match *self {
            Value::Reference( ref reference ) if Object::instance_of( reference ) => {
                unsafe {
                    Some( mem::transmute( reference ) )
                }
            },
            _ => None
        }
    }

    /// Gets a reference to the [Array](struct.Array.html) inside this `Value`.
    #[inline]
    pub fn as_array( &self ) -> Option< &Array > {
        match *self {
            Value::Reference( ref reference ) if Array::instance_of( reference ) => {
                unsafe {
                    Some( mem::transmute( reference ) )
                }
            },
            _ => None
        }
    }

    /// Returns the [Reference](struct.Reference.html) inside this `Value`.
    #[inline]
    pub fn into_reference( self ) -> Option< Reference > {
        match self {
            Value::Reference( reference ) => Some( reference ),
            _ => None
        }
    }

    /// Returns the [Object](struct.Object.html) inside this `Value`.
    #[inline]
    pub fn into_object( self ) -> Option< Object > {
        match self {
            Value::Reference( reference ) => reference.try_into().ok(),
            _ => None
        }
    }

    /// Returns the [Array](struct.Array.html) inside this `Value`.
    #[inline]
    pub fn into_array( self ) -> Option< Array > {
        match self {
            Value::Reference( reference ) => reference.try_into().ok(),
            _ => None
        }
    }

    /// Converts a [Reference](struct.Reference.html) inside this `Value` into
    /// the given type `T`; doesn't check whenever the reference is really of type `T`.
    ///
    /// In cases where the value is not a `Reference` a `None` is returned.
    #[inline]
    pub unsafe fn into_reference_unchecked< T: ReferenceType >( self ) -> Option< T > {
        let reference: Option< Reference > = self.try_into().ok();
        reference.map( |reference| T::from_reference_unchecked( reference ) )
    }

    /// Returns the `String` inside this `Value`.
    #[inline]
    pub fn into_string( self ) -> Option< String > {
        match self {
            Value::String( string ) => Some( string ),
            _ => None
        }
    }

    /// Returns a borrow of the string inside this `Value`.
    #[inline]
    pub fn as_str( &self ) -> Option< &str > {
        match *self {
            Value::String( ref string ) => Some( string.as_str() ),
            _ => None
        }
    }
}

impl AsRef< Value > for Value {
    #[inline]
    fn as_ref( &self ) -> &Self {
        self
    }
}

impl From< Undefined > for Value {
    #[inline]
    fn from( _: Undefined ) -> Self {
        Value::Undefined
    }
}

impl< 'a > From< &'a Undefined > for Value {
    #[inline]
    fn from( _: &'a Undefined ) -> Self {
        Value::Undefined
    }
}

impl< 'a > From< &'a mut Undefined > for Value {
    #[inline]
    fn from( _: &'a mut Undefined ) -> Self {
        Value::Undefined
    }
}

impl From< Null > for Value {
    #[inline]
    fn from( _: Null ) -> Self {
        Value::Null
    }
}

impl< 'a > From< &'a Null > for Value {
    #[inline]
    fn from( _: &'a Null ) -> Self {
        Value::Null
    }
}

impl< 'a > From< &'a mut Null > for Value {
    #[inline]
    fn from( _: &'a mut Null ) -> Self {
        Value::Null
    }
}

impl From< bool > for Value {
    #[inline]
    fn from( value: bool ) -> Self {
        Value::Bool( value )
    }
}

impl< 'a > From< &'a bool > for Value {
    #[inline]
    fn from( value: &'a bool ) -> Self {
        Value::Bool( *value )
    }
}

impl< 'a > From< &'a mut bool > for Value {
    #[inline]
    fn from( value: &'a mut bool ) -> Self {
        (value as &bool).into()
    }
}

impl< 'a > From< &'a str > for Value {
    #[inline]
    fn from( value: &'a str ) -> Self {
        Value::String( value.to_string() )
    }
}

impl< 'a > From< &'a mut str > for Value {
    #[inline]
    fn from( value: &'a mut str ) -> Self {
        (value as &str).into()
    }
}

impl From< String > for Value {
    #[inline]
    fn from( value: String ) -> Self {
        Value::String( value )
    }
}

impl< 'a > From< &'a String > for Value {
    #[inline]
    fn from( value: &'a String ) -> Self {
        Value::String( value.clone() )
    }
}

impl< 'a > From< &'a mut String > for Value {
    #[inline]
    fn from( value: &'a mut String ) -> Self {
        (value as &String).into()
    }
}

impl From< char > for Value {
    #[inline]
    fn from( value: char ) -> Self {
        let mut buffer: [u8; 4] = [0; 4];
        let string = value.encode_utf8( &mut buffer );
        string.to_owned().into()
    }
}

impl< 'a > From< &'a char > for Value {
    #[inline]
    fn from( value: &'a char ) -> Self {
        (*value).into()
    }
}

impl< 'a > From< &'a mut char > for Value {
    #[inline]
    fn from( value: &'a mut char ) -> Self {
        (*value).into()
    }
}

impl< T > From< Vec< T > > for Value where T: JsSerialize {
    #[inline]
    fn from( value: Vec< T > ) -> Self {
        value[..].into()
    }
}

impl< 'a, T > From< &'a Vec< T > > for Value where T: JsSerialize {
    #[inline]
    fn from( value: &'a Vec< T > ) -> Self {
        value[..].into()
    }
}

impl< 'a, T > From< &'a mut Vec< T > > for Value where T: JsSerialize {
    #[inline]
    fn from( value: &'a mut Vec< T > ) -> Self {
        value[..].into()
    }
}

impl< 'a, T > From< &'a [T] > for Value where T: JsSerialize {
    #[inline]
    fn from( value: &'a [T] ) -> Self {
        let array: Array = value.into();
        Value::Reference( array.into() )
    }
}

impl< 'a, T > From< &'a mut [T] > for Value where T: JsSerialize {
    #[inline]
    fn from( value: &'a mut [T] ) -> Self {
        (value as &[T]).into()
    }
}

impl< K, V > From< BTreeMap< K, V > > for Value where K: AsRef< str >, V: JsSerialize {
    #[inline]
    fn from( value: BTreeMap< K, V > ) -> Self {
        let object: Object = value.into();
        Value::Reference( object.into() )
    }
}

impl< 'a, K, V > From< &'a BTreeMap< K, V > > for Value where K: AsRef< str >, V: JsSerialize {
    #[inline]
    fn from( value: &'a BTreeMap< K, V > ) -> Self {
        let object: Object = value.into();
        Value::Reference( object.into() )
    }
}

impl< 'a, K, V > From< &'a mut BTreeMap< K, V > > for Value where K: AsRef< str >, V: JsSerialize {
    #[inline]
    fn from( value: &'a mut BTreeMap< K, V > ) -> Self {
        let object: Object = value.into();
        Value::Reference( object.into() )
    }
}

impl< K, V > From< HashMap< K, V > > for Value where K: AsRef< str > + Eq + Hash, V: JsSerialize {
    #[inline]
    fn from( value: HashMap< K, V > ) -> Self {
        let object: Object = value.into();
        Value::Reference( object.into() )
    }
}

impl< 'a, K, V > From< &'a HashMap< K, V > > for Value where K: AsRef< str > + Eq + Hash, V: JsSerialize {
    #[inline]
    fn from( value: &'a HashMap< K, V > ) -> Self {
        let object: Object = value.into();
        Value::Reference( object.into() )
    }
}

impl< 'a, K, V > From< &'a mut HashMap< K, V > > for Value where K: AsRef< str > + Eq + Hash, V: JsSerialize {
    #[inline]
    fn from( value: &'a mut HashMap< K, V > ) -> Self {
        let object: Object = value.into();
        Value::Reference( object.into() )
    }
}

impl From< Reference > for Value {
    #[inline]
    fn from( value: Reference ) -> Self {
        Value::Reference( value )
    }
}

impl< 'a > From< &'a Reference > for Value {
    #[inline]
    fn from( value: &'a Reference ) -> Self {
        Value::Reference( value.clone() )
    }
}

impl< 'a > From< &'a mut Reference > for Value {
    #[inline]
    fn from( value: &'a mut Reference ) -> Self {
        (value as &Reference).into()
    }
}

macro_rules! impl_from_number {
    ($($kind:ty)+) => {
        $(
            impl From< $kind > for Value {
                #[inline]
                fn from( value: $kind ) -> Self {
                    Value::Number( value.into() )
                }
            }

            impl< 'a > From< &'a $kind > for Value {
                #[inline]
                fn from( value: &'a $kind ) -> Self {
                    Value::Number( (*value).into() )
                }
            }

            impl< 'a > From< &'a mut $kind > for Value {
                #[inline]
                fn from( value: &'a mut $kind ) -> Self {
                    (value as &$kind).into()
                }
            }

            impl_infallible_try_from!( $kind => Value; );
        )+
    };
}

impl_from_number!( i8 i16 i32 u8 u16 u32 f32 f64 );
impl_infallible_try_from! {
    Value => Value;
    Undefined => Value;
    impl< 'a > for &'a Undefined => Value;
    impl< 'a > for &'a mut Undefined => Value;
    Null => Value;
    impl< 'a > for &'a Null => Value;
    impl< 'a > for &'a mut Null => Value;
    bool => Value;
    impl< 'a > for &'a bool => Value;
    impl< 'a > for &'a mut bool => Value;
    impl< 'a > for &'a str => Value;
    impl< 'a > for &'a mut str => Value;
    String => Value;
    impl< 'a > for &'a String => Value;
    impl< 'a > for &'a mut String => Value;
    char => Value;
    impl< 'a > for &'a char => Value;
    impl< 'a > for &'a mut char => Value;
    impl< T > for Vec< T > => Value where (T: JsSerialize);
    impl< 'a, T > for &'a Vec< T > => Value where (T: JsSerialize);
    impl< 'a, T > for &'a mut Vec< T > => Value where (T: JsSerialize);
    impl< 'a, T > for &'a [T] => Value where (T: JsSerialize);
    impl< 'a, T > for &'a mut [T] => Value where (T: JsSerialize);

    impl< K, V > for BTreeMap< K, V > => Value where (K: AsRef< str >, V: JsSerialize);
    impl< 'a, K, V > for &'a BTreeMap< K, V > => Value where (K: AsRef< str >, V: JsSerialize);
    impl< 'a, K, V > for &'a mut BTreeMap< K, V > => Value where (K: AsRef< str >, V: JsSerialize);
    impl< K, V > for HashMap< K, V > => Value where (K: AsRef< str > + Eq + Hash, V: JsSerialize);
    impl< 'a, K, V > for &'a HashMap< K, V > => Value where (K: AsRef< str > + Eq + Hash, V: JsSerialize);
    impl< 'a, K, V > for &'a mut HashMap< K, V > => Value where (K: AsRef< str > + Eq + Hash, V: JsSerialize);

    Symbol => Value;
    Reference => Value;

    // TODO: Move these to object.rs
    impl< K, V > for BTreeMap< K, V > => Object where (K: AsRef< str >, V: JsSerialize);
    impl< 'a, K, V > for &'a BTreeMap< K, V > => Object where (K: AsRef< str >, V: JsSerialize);
    impl< 'a, K, V > for &'a mut BTreeMap< K, V > => Object where (K: AsRef< str >, V: JsSerialize);
    impl< K, V > for HashMap< K, V > => Object where (K: AsRef< str > + Eq + Hash, V: JsSerialize);
    impl< 'a, K, V > for &'a HashMap< K, V > => Object where (K: AsRef< str > + Eq + Hash, V: JsSerialize);
    impl< 'a, K, V > for &'a mut HashMap< K, V > => Object where (K: AsRef< str > + Eq + Hash, V: JsSerialize);

    // TODO: Move these to array.rs
    impl< T > for Vec< T > => Array where (T: JsSerialize);
    impl< 'a, T > for &'a Vec< T > => Array where (T: JsSerialize);
    impl< 'a, T > for &'a mut Vec< T > => Array where (T: JsSerialize);
    impl< 'a, T > for &'a [T] => Array where (T: JsSerialize);
    impl< 'a, T > for &'a mut [T] => Array where (T: JsSerialize);
}

macro_rules! impl_try_from_number {
    ($($kind:ty)+) => {
        $(
            impl TryFrom< $kind > for Value {
                type Error = <Number as TryFrom< $kind >>::Error;

                #[inline]
                fn try_from( value: $kind ) -> Result< Self, Self::Error > {
                    Ok( Value::Number( value.try_into()? ) )
                }
            }
        )+
    };
}

impl_try_from_number!( i64 u64 usize );

impl PartialEq< Undefined > for Value {
    #[inline]
    fn eq( &self, _: &Undefined ) -> bool {
        match *self {
            Value::Undefined => true,
            _ => false
        }
    }
}

impl PartialEq< Null > for Value {
    #[inline]
    fn eq( &self, _: &Null ) -> bool {
        match *self {
            Value::Null => true,
            _ => false
        }
    }
}

impl PartialEq< bool > for Value {
    #[inline]
    fn eq( &self, right: &bool ) -> bool {
        match *self {
            Value::Bool( left ) => left == *right,
            _ => false
        }
    }
}

impl PartialEq< str > for Value {
    #[inline]
    fn eq( &self, right: &str ) -> bool {
        match *self {
            Value::String( ref left ) => left == right,
            _ => false
        }
    }
}

impl PartialEq< String > for Value {
    #[inline]
    fn eq( &self, right: &String ) -> bool {
        match *self {
            Value::String( ref left ) => left == right,
            _ => false
        }
    }
}

impl PartialEq< Number > for Value {
    #[inline]
    fn eq( &self, right: &Number ) -> bool {
        match *self {
            Value::Number( left ) => left == *right,
            _ => false
        }
    }
}

impl PartialEq< Symbol > for Value {
    #[inline]
    fn eq( &self, right: &Symbol ) -> bool {
        match *self {
            Value::Symbol( ref left ) => *left == *right,
            _ => false
        }
    }
}

impl< T: AsRef< Reference > > PartialEq< T > for Value {
    #[inline]
    fn eq( &self, right: &T ) -> bool {
        match *self {
            Value::Reference( ref left ) => left == right.as_ref(),
            _ => false
        }
    }
}

impl< 'a > PartialEq< Reference > for &'a Value {
    #[inline]
    fn eq( &self, right: &Reference ) -> bool {
        (*self).eq( right )
    }
}

impl PartialEq< Value > for Reference {
    #[inline]
    fn eq( &self, right: &Value ) -> bool {
        right.eq( self )
    }
}

impl< 'a > PartialEq< &'a Value > for Reference {
    #[inline]
    fn eq( &self, right: &&'a Value ) -> bool {
        let right: &'a Value = right;
        right.eq( self )
    }
}

impl< 'a > PartialEq< Value > for &'a Reference {
    #[inline]
    fn eq( &self, right: &Value ) -> bool {
        (*self).eq( right )
    }
}

macro_rules! impl_partial_eq_boilerplate {
    ( $( $kind:ty ),+ ) => {
        $(
            impl< 'a > PartialEq< &'a $kind > for Value {
                #[inline]
                fn eq( &self, right: &&'a $kind ) -> bool {
                    let right: &'a $kind = right;
                    self.eq( right )
                }
            }

            impl< 'a > PartialEq< $kind > for &'a Value {
                #[inline]
                fn eq( &self, right: &$kind ) -> bool {
                    (*self).eq( right )
                }
            }

            impl PartialEq< Value > for $kind {
                #[inline]
                fn eq( &self, right: &Value ) -> bool {
                    right == self
                }
            }

            impl< 'a > PartialEq< &'a Value > for $kind {
                #[inline]
                fn eq( &self, right: &&'a Value ) -> bool {
                    let right: &'a Value = right;
                    right == self
                }
            }

            impl< 'a > PartialEq< Value > for &'a $kind {
                #[inline]
                fn eq( &self, right: &Value ) -> bool {
                    (*self).eq( right )
                }
            }
        )+
    }
}

macro_rules! impl_partial_eq_to_number {
    ($($kind:ty)+) => {
        $(
            impl PartialEq< $kind > for Value {
                #[inline]
                fn eq( &self, right: &$kind ) -> bool {
                    match *self {
                        Value::Number( left ) => left == *right,
                        _ => false
                    }
                }
            }

            impl_partial_eq_boilerplate!( $kind );
        )+
    };
}

impl_partial_eq_to_number!( i8 i16 i32 i64 u8 u16 u32 u64 usize f32 f64 );

impl_partial_eq_boilerplate! {
    Undefined,
    Null,
    bool,
    str,
    String,
    Number,
    Symbol
}

/// A structure denoting a conversion error encountered when
/// converting to or from a `Value`.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConversionError {
    TypeMismatch {
        expected: Cow< 'static, str >,
        actual: Cow< 'static, str >
    },
    NumericConversionError( number::ConversionError ),
    ValueConversionError( Box< ConversionError > ),
    Custom( String )
}

fn extract_type_name( value: &Value ) -> Cow< 'static, str > {
    match *value {
        Value::Undefined => "undefined".into(),
        Value::Null => "null".into(),
        Value::Bool( _ ) => "bool".into(),
        Value::Number( _ ) => "Number".into(),
        Value::Symbol( _ ) => "Symbol".into(),
        Value::String( _ ) => "String".into(),
        Value::Reference( _ ) => "Reference".into()
    }
}

impl fmt::Display for ConversionError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        match *self {
            ConversionError::TypeMismatch { ref expected, ref actual } => {
                write!( formatter, "type mismatch; expected {}, got {}", expected, actual )
            },
            ConversionError::NumericConversionError( ref inner ) => write!( formatter, "{}", inner ),
            ConversionError::ValueConversionError( ref inner ) => write!( formatter, "value conversion error: {}", inner ),
            ConversionError::Custom( ref message ) => write!( formatter, "{}", message )
        }
    }
}

impl error::Error for ConversionError {
    fn description( &self ) -> &str {
        match *self {
            ConversionError::TypeMismatch { .. } => "type mismatch",
            ConversionError::NumericConversionError( ref inner ) => inner.description(),
            ConversionError::ValueConversionError( _ ) => "value conversion error",
            ConversionError::Custom( ref message ) => message
        }
    }
}

impl From< number::ConversionError > for ConversionError {
    fn from( inner: number::ConversionError ) -> Self {
        ConversionError::NumericConversionError( inner )
    }
}

impl From< Void > for ConversionError {
    fn from( _: Void ) -> Self {
        unreachable!();
    }
}

impl From< ConversionError > for TypeError {
    fn from( error: ConversionError ) -> TypeError {
        (&error).into()
    }
}

impl< 'a > From< &'a ConversionError > for TypeError {
    fn from( error: &'a ConversionError ) -> TypeError {
        js!( return new TypeError( @{format!( "{}", error )} ); ).try_into().unwrap()
    }
}

impl ConversionError {
    #[inline]
    pub(crate) fn type_mismatch( actual_value: &Value, expected: Cow< 'static, str > ) -> Self {
        ConversionError::TypeMismatch {
            actual: extract_type_name( actual_value ),
            expected
        }
    }

    #[inline]
    pub(crate) fn value_conversion_error( inner: ConversionError ) -> Self {
        ConversionError::ValueConversionError( Box::new( inner ) )
    }
}

impl TryFrom< Value > for Undefined {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Undefined => Ok( Undefined ),
            _ => Err( ConversionError::type_mismatch( &value, "undefined".into() ) )
        }
    }
}

impl TryFrom< Value > for Null {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Null => Ok( Null ),
            _ => Err( ConversionError::type_mismatch( &value, "null".into() ) )
        }
    }
}

impl TryFrom< Value > for () {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Null | Value::Undefined => Ok( () ),
            _ => Err( ConversionError::type_mismatch( &value, "null or undefined".into() ) )
        }
    }
}

impl TryFrom< Value > for bool {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Bool( value ) => Ok( value ),
            _ => Err( ConversionError::type_mismatch( &value, "bool".into() ) )
        }
    }
}

macro_rules! impl_try_into_number {
    ($($kind:ty)+) => {
        $(
            impl TryFrom< Value > for $kind {
                type Error = ConversionError;

                #[inline]
                fn try_from( value: Value ) -> Result< Self, Self::Error > {
                    match value {
                        Value::Number( value ) => {
                            let result: Result< Self, _ > = value.try_into();
                            result.map_err( |error| error.into() )
                        },
                        _ => {
                            let expected = concat!( "Number which fits into ", stringify!( $kind ) );
                            Err( ConversionError::type_mismatch( &value, expected.into() ) )
                        }
                    }
                }
            }
        )+
    };
}

impl_try_into_number!( u8 u16 u32 u64 usize i8 i16 i32 i64 f64 );

impl< E: Into< ConversionError >, V: TryFrom< Value, Error = E > > TryFrom< Value > for BTreeMap< String, V > {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Reference( reference ) => {
                let object: Object = reference.try_into()?;
                object.try_into()
            },
            _ => {
                let expected = match type_name_opt::< V >() {
                    Some( type_name ) => format!( "Object with values of type {}", type_name ).into(),
                    None => "Object".into()
                };

                Err( ConversionError::type_mismatch( &value, expected ) )
            }
        }
    }
}

impl< E: Into< ConversionError >, V: TryFrom< Value, Error = E > > TryFrom< Value > for HashMap< String, V > {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Reference( reference ) => {
                let object: Object = reference.try_into()?;
                object.try_into()
            },
            _ => {
                let expected = match type_name_opt::< V >() {
                    Some( type_name ) => format!( "Object with values of type {}", type_name ).into(),
                    None => "Object".into()
                };

                Err( ConversionError::type_mismatch( &value, expected ) )
            }
        }
    }
}

impl< E: Into< ConversionError >, T: TryFrom< Value, Error = E > > TryFrom< Value > for Vec< T > {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Reference( reference ) => {
                let array: Array = reference.try_into()?;
                array.try_into()
            },
            _ => {
                let expected = match type_name_opt::< T >() {
                    Some( type_name ) => format!( "Array with elements of type {}", type_name ).into(),
                    None => "Array".into()
                };

                Err( ConversionError::type_mismatch( &value, expected ) )
            }
        }
    }
}

impl TryFrom< Value > for String {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::String( value ) => Ok( value ),
            _ => Err( ConversionError::type_mismatch( &value, "String".into() ) )
        }
    }
}

impl TryFrom< Value > for Symbol {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Symbol( value ) => Ok( value ),
            _ => Err( ConversionError::type_mismatch( &value, "Symbol".into() ) )
        }
    }
}

impl TryFrom< Value > for Reference {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Reference( value ) => Ok( value ),
            _ => Err( ConversionError::type_mismatch( &value, "Reference".into() ) )
        }
    }
}

impl< 'a > TryFrom< &'a Value > for &'a str {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: &'a Value ) -> Result< Self, Self::Error > {
        match *value {
            Value::String( ref value ) => Ok( value ),
            _ => Err( ConversionError::type_mismatch( &value, "String".into() ) )
        }
    }
}

impl< 'a > TryFrom< &'a Value > for &'a Symbol {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: &'a Value ) -> Result< Self, Self::Error > {
        match *value {
            Value::Symbol( ref value ) => Ok( value ),
            _ => Err( ConversionError::type_mismatch( &value, "Symbol".into() ) )
        }
    }
}

impl< 'a > TryFrom< &'a Value > for &'a Reference {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: &'a Value ) -> Result< Self, Self::Error > {
        match *value {
            Value::Reference( ref value ) => Ok( value ),
            _ => Err( ConversionError::type_mismatch( &value, "Reference".into() ) )
        }
    }
}

macro_rules! __impl_nullable_try_from_value {
    (($($impl_arg:tt)*) ($($dst_arg:tt)*) ($($bounds:tt)*)) => {
        impl< $($impl_arg)* > TryFrom< Value > for Option< $($dst_arg)* > where $($bounds)* {
            type Error = ConversionError;

            #[inline]
            fn try_from( value: Value ) -> Result< Self, Self::Error > {
                match value {
                    Value::Undefined | Value::Null => Ok( None ),
                    value => value.try_into().map( Some )
                }
            }
        }
    };
}

macro_rules! impl_nullable_try_from_value {
    (impl< $($impl_arg:tt),* > $dst:ty where ($($bounds:tt)*); $($rest:tt)*) => {
        __impl_nullable_try_from_value!( ($($impl_arg),*) ($dst) ($($bounds)*) );
        impl_nullable_try_from_value!( $($rest)* );
    };

    (impl< $($impl_arg:tt),* > $dst:ty; $($rest:tt)*) => {
        __impl_nullable_try_from_value!( ($($impl_arg),*) ($dst) () );
        impl_nullable_try_from_value!( $($rest)* );
    };

    ($dst:ty; $($rest:tt)*) => {
        __impl_nullable_try_from_value!( () ($dst) () );
        impl_nullable_try_from_value!( $($rest)* );

    };

    () => {};
}

impl_nullable_try_from_value! {
    bool;
    u8;
    u16;
    u32;
    u64;
    usize;
    i8;
    i16;
    i32;
    i64;
    f64;
    impl< V > BTreeMap< String, V > where (V: TryFrom< Value, Error = ConversionError >);
    impl< V > HashMap< String, V > where (V: TryFrom< Value, Error = ConversionError >);
    impl< T > Vec< T > where (T: TryFrom< Value, Error = ConversionError >);
    String;
    Symbol;
}

impl< 'a > TryFrom< &'a Value > for Option< &'a str > {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: &'a Value ) -> Result< Self, Self::Error > {
        match *value {
            Value::String( ref value ) => Ok( Some( value ) ),
            ref value => value.try_into().map( Some )
        }
    }
}

impl< 'a > TryFrom< &'a Value > for Option< &'a Reference > {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: &'a Value ) -> Result< Self, Self::Error > {
        match *value {
            Value::Reference( ref value ) => Ok( Some( value ) ),
            ref value => value.try_into().map( Some )
        }
    }
}

impl< T: TryFrom< Value, Error = ConversionError > + AsRef< Reference > > TryFrom< Value > for Option< T > {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Undefined | Value::Null => Ok( None ),
            value => value.try_into().map( Some )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Value, Reference, ConversionError};
    use webcore::try_from::TryInto;

    #[test]
    fn string_equality() {
        let value = Value::String( "Hello!".to_owned() );
        assert!( value == "Hello!" );
        assert!( &value == "Hello!" );
        assert!( value == "Hello!".to_owned() );
        assert!( &value == "Hello!".to_owned() );
        assert!( value == &"Hello!".to_owned() );
        assert!( &value == &"Hello!".to_owned() );
        assert!( "Hello!" == value );
        assert!( "Hello!" == &value );
        assert!( "Hello!".to_owned() == value );
        assert!( "Hello!".to_owned() == &value );
        assert!( &"Hello!".to_owned() == value );
        assert!( &"Hello!".to_owned() == &value );

        assert!( value != "Bob" );
    }

    #[test]
    fn reference_equality() {
        let value = js! { return new Date() };
        let reference: Reference = value.clone().try_into().unwrap();

        assert!( value == reference );
        assert!( &value == reference );
        assert!( value == &reference );
        assert!( &value == &reference );
        assert!( reference == value );
        assert!( &reference == value );
        assert!( reference == &value );
        assert!( &reference == &value );
    }

    #[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
    #[reference(instance_of = "Error")]
    pub struct Error( Reference );

    #[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
    #[reference(instance_of = "ReferenceError")]
    #[reference(subclass_of(Error))]
    pub struct ReferenceError( Reference );

    #[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
    #[reference(instance_of = "TypeError")]
    #[reference(subclass_of(Error))]
    pub struct TypeError( Reference );

    #[test]
    fn reference_downcast() {
        let reference = js! { return new ReferenceError(); }.into_reference().unwrap();
        assert!( reference.clone().downcast::< Error >().is_some() );
        assert!( reference.clone().downcast::< ReferenceError >().is_some() );
        assert!( reference.clone().downcast::< TypeError >().is_none() );
    }

    #[test]
    fn reference_try_into_downcast_from_reference() {
        let reference = js! { return new ReferenceError(); }.into_reference().unwrap();
        let typed_reference: Result< Error, _ > = reference.clone().try_into();
        assert!( typed_reference.is_ok() );

        let typed_reference: Result< ReferenceError, _ > = reference.clone().try_into();
        assert!( typed_reference.is_ok() );

        let typed_reference: Result< TypeError, _ > = reference.clone().try_into();
        assert!( typed_reference.is_err() );
    }

    #[test]
    fn reference_try_into_downcast_from_value() {
        let value = js! { return new ReferenceError(); };
        let typed_reference: Result< Error, _ > = value.clone().try_into();
        assert!( typed_reference.is_ok() );

        let typed_reference: Result< ReferenceError, _ > = value.clone().try_into();
        assert!( typed_reference.is_ok() );

        let typed_reference: Result< TypeError, _ > = value.clone().try_into();
        assert!( typed_reference.is_err() );
    }

    #[test]
    fn reference_into_upcast() {
        let reference: ReferenceError = js! { return new ReferenceError(); }.into_reference().unwrap().downcast().unwrap();
        let _: Error = reference.clone().into();
        let _: Reference = reference.clone().into();
    }

    #[test]
    fn reference_try_into_downcast_from_ref_value() {
        let value = js! { return new ReferenceError(); };
        let value: &Value = &value;

        let typed_reference: Result< Error, _ > = value.try_into();
        assert!( typed_reference.is_ok() );

        let typed_reference: Result< ReferenceError, _ > = value.try_into();
        assert!( typed_reference.is_ok() );

        let typed_reference: Result< TypeError, _ > = value.try_into();
        assert!( typed_reference.is_err() );
    }

    #[test]
    fn reference_try_into_downcast_from_ref_reference() {
        let reference: Reference = js! { return new ReferenceError(); }.try_into().unwrap();
        let reference: &Reference = &reference;

        let typed_reference: Result< Error, _ > = reference.try_into();
        assert!( typed_reference.is_ok() );

        let typed_reference: Result< ReferenceError, _ > = reference.try_into();
        assert!( typed_reference.is_ok() );

        let typed_reference: Result< TypeError, _ > = reference.try_into();
        assert!( typed_reference.is_err() );
    }

    #[test]
    fn convert_from_null_or_undefined_to_empty_tuple() {
        let a: Result< (), _ > = js! { return null; }.try_into();
        assert!( a.is_ok() );

        let a: Result< (), _ > = js! { return undefined; }.try_into();
        assert!( a.is_ok() );

        let a: Result< (), _ > = js! { return 1; }.try_into();
        assert!( a.is_err() );
    }

    #[test]
    fn reference_stable() {
        js! { Module.__test = {}; }
        let a = js! { return Module.__test; }.as_reference().unwrap().as_raw();
        let b = js! { return Module.__test; }.as_reference().unwrap().as_raw();
        assert_eq!(a, b);

        let c = js! { return {}; }.as_reference().unwrap().as_raw();
        assert_ne!(a, c);

        js! { delete Module.__test; }
    }

    fn is_known_reference(refid: i32) -> bool {
        let has_refcount: bool = js! {
            return @{refid} in Module.STDWEB_PRIVATE.id_to_refcount_map;
        }.try_into().unwrap();

        let has_ref: bool = js! {
            return @{refid} in Module.STDWEB_PRIVATE.id_to_ref_map;
        }.try_into().unwrap();

        assert_eq!(has_refcount, has_ref);
        has_refcount
    }

    #[test]
    fn reference_refcount() {
        let obj = js! { return new Object(); };
        let refid = obj.as_reference().unwrap().as_raw();
        assert!(is_known_reference(refid));

        drop(obj);
        assert!(!is_known_reference(refid));
    }

    #[test]
    fn reference_refcount_clone() {
        let obj = js! { return new Object(); };
        let obj2 = obj.clone();

        let refid = obj.as_reference().unwrap().as_raw();
        let refid2 = obj.as_reference().unwrap().as_raw();

        assert_eq!(refid, refid2);
        assert!(is_known_reference(refid));

        drop(obj);
        assert!(is_known_reference(refid));

        drop(obj2);
        assert!(!is_known_reference(refid));
    }

    #[test]
    fn conversion_error_string_into_bool() {
        let a = Value::String( "Piggy".into() );
        let b: Result< bool, ConversionError > = a.try_into();
        assert_eq!(
            format!( "{}", b.unwrap_err() ),
            "type mismatch; expected bool, got String"
        );
    }

    #[cfg(rust_nightly)]
    #[test]
    fn conversion_error_string_into_vec_of_bools() {
        let a = Value::String( "Piggy".into() );
        let b: Result< Vec< bool >, ConversionError > = a.try_into();
        assert_eq!(
            format!( "{}", b.unwrap_err() ),
            "type mismatch; expected Array with elements of type bool, got String"
        );
    }
}
