// To give credit where it is due - a significant chunk of code
// in this file was borrowed from `serde_json`.

use std::fmt;
use std::error;
use std::collections::BTreeMap;
use std::vec;

use serde_crate::ser::{self, Serialize};
use serde_crate::de::{self, Deserialize, Visitor};
use serde_crate::de::IntoDeserializer;

use webcore::value::{
    self,
    Undefined,
    Null,
    Value
};

use webcore::serialization::{
    JsSerialize,
    SerializedValue
};

use webcore::number::{self, Number, Storage, get_storage};
use webcore::try_from::{TryInto, TryFrom};
use webcore::instance_of::InstanceOf;
use webcore::array::Array;
use webcore::object::Object;
use webcore::global_arena;

impl Serialize for Undefined {
    #[inline]
    fn serialize< S: ser::Serializer >( &self, serializer: S ) -> Result< S::Ok, S::Error > {
        serializer.serialize_unit_struct( "undefined" )
    }
}

impl< 'de > Deserialize< 'de > for Undefined {
    #[inline]
    fn deserialize< D: de::Deserializer< 'de > >( deserializer: D ) -> Result< Self, D::Error > {
        struct UndefinedVisitor;
        impl< 'de > Visitor< 'de > for UndefinedVisitor {
            type Value = Undefined;

            fn expecting( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
                formatter.write_str( "undefined" )
            }

            fn visit_unit< E: de::Error >( self ) -> Result< Self::Value, E > {
                Ok( Undefined )
            }
        }

        deserializer.deserialize_unit_struct( "undefined", UndefinedVisitor )
    }
}

impl Serialize for Null {
    #[inline]
    fn serialize< S: ser::Serializer >( &self, serializer: S ) -> Result< S::Ok, S::Error > {
        serializer.serialize_unit_struct( "null" )
    }
}

impl< 'de > Deserialize< 'de > for Null {
    #[inline]
    fn deserialize< D: de::Deserializer< 'de > >( deserializer: D ) -> Result< Self, D::Error > {
        struct NullVisitor;
        impl< 'de > Visitor< 'de > for NullVisitor {
            type Value = Null;

            fn expecting( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
                formatter.write_str( "null" )
            }

            fn visit_unit< E: de::Error >( self ) -> Result< Self::Value, E > {
                Ok( Null )
            }
        }

        deserializer.deserialize_unit_struct( "null", NullVisitor )
    }
}

impl Serialize for Number {
    #[inline]
    fn serialize< S: ser::Serializer >( &self, serializer: S ) -> Result< S::Ok, S::Error > {
        match *get_storage( self ) {
            Storage::I32( value ) => serializer.serialize_i32( value ),
            Storage::F64( value ) => serializer.serialize_f64( value )
        }
    }
}

impl< 'de > Deserialize< 'de > for Number {
    #[inline]
    fn deserialize< D: de::Deserializer< 'de > >( deserializer: D ) -> Result< Self, D::Error > {
        struct NumberVisitor;
        impl< 'de > Visitor< 'de > for NumberVisitor {
            type Value = Number;

            fn expecting( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
                formatter.write_str( "a number" )
            }

            fn visit_i8< E: de::Error >( self, value: i8 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_i16< E: de::Error >( self, value: i16 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_i32< E: de::Error >( self, value: i32 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_i64< E: de::Error >( self, value: i64 ) -> Result< Self::Value, E > {
                value.try_into().map_err( E::custom )
            }

            fn visit_u8< E: de::Error >( self, value: u8 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_u16< E: de::Error >( self, value: u16 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_u32< E: de::Error >( self, value: u32 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_u64< E: de::Error >( self, value: u64 ) -> Result< Self::Value, E > {
                value.try_into().map_err( E::custom )
            }

            fn visit_f32< E: de::Error >( self, value: f32 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_f64< E: de::Error >( self, value: f64 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }
        }

        deserializer.deserialize_f64( NumberVisitor )
    }
}

impl Serialize for Value {
    #[inline]
    fn serialize< S: ser::Serializer >( &self, serializer: S ) -> Result< S::Ok, S::Error > {
        use serde_crate::ser::SerializeMap;
        match *self {
            Value::Undefined => serializer.serialize_unit_struct( "undefined" ),
            Value::Null => serializer.serialize_unit_struct( "null" ),
            Value::Bool( value ) => serializer.serialize_bool( value ),
            Value::Number( ref value ) => value.serialize( serializer ),
            Value::Symbol( _ ) => unimplemented!( "Serialization of symbols is unimplemented!" ),
            Value::String( ref value ) => serializer.serialize_str( value ),
            Value::Reference( ref reference ) => {
                if Array::instance_of( reference ) {
                    let array: Array = reference.try_into().unwrap();
                    let value: Vec< Value > = array.into();
                    value.serialize( serializer )
                } else if Object::instance_of( reference ) {
                    let object: Object = reference.try_into().unwrap();
                    let value: BTreeMap< String, Value > = object.into();
                    let mut map = try!( serializer.serialize_map( Some( value.len() ) ) );
                    for (key, value) in value {
                        try!( map.serialize_key( &key ) );
                        try!( map.serialize_value( &value ) );
                    }

                    map.end()
                } else {
                    let map = try!( serializer.serialize_map( None ) );
                    map.end()
                }
            }
        }
    }
}

impl< 'de > Deserialize< 'de > for Value {
    #[inline]
    fn deserialize< D: de::Deserializer< 'de > >( deserializer: D ) -> Result< Self, D::Error > {
        struct ValueVisitor;
        impl< 'de > Visitor< 'de > for ValueVisitor {
            type Value = Value;

            fn expecting( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
                formatter.write_str( "a value which is convertible into a JavaScript value" )
            }

            fn visit_bool< E: de::Error >( self, value: bool ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_i8< E: de::Error >( self, value: i8 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_i16< E: de::Error >( self, value: i16 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_i32< E: de::Error >( self, value: i32 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_i64< E: de::Error >( self, value: i64 ) -> Result< Self::Value, E > {
                value.try_into().map_err( E::custom )
            }

            fn visit_u8< E: de::Error >( self, value: u8 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_u16< E: de::Error >( self, value: u16 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_u32< E: de::Error >( self, value: u32 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_u64< E: de::Error >( self, value: u64 ) -> Result< Self::Value, E > {
                value.try_into().map_err( E::custom )
            }

            fn visit_f32< E: de::Error >( self, value: f32 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_f64< E: de::Error >( self, value: f64 ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_char< E: de::Error >( self, value: char ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_str< E: de::Error >( self, value: &str ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_string< E: de::Error >( self, value: String ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_unit< E: de::Error >( self ) -> Result< Self::Value, E > {
                Ok( Null.into() )
            }

            fn visit_none< E: de::Error >( self ) -> Result< Self::Value, E > {
                Ok( Null.into() )
            }

            fn visit_some< D: de::Deserializer< 'de > >( self, deserializer: D ) -> Result< Self::Value, D::Error > {
                deserializer.deserialize_any( self )
            }

            fn visit_seq< V: de::SeqAccess< 'de > >( self, mut visitor: V ) -> Result< Self::Value, V::Error > {
                let mut output: Vec< Value > = Vec::with_capacity( visitor.size_hint().unwrap_or( 0 ) );
                while let Some( element ) = visitor.next_element()? {
                    output.push( element );
                }

                Ok( output.into() )
            }

            fn visit_map< V: de::MapAccess< 'de > >( self, mut visitor: V ) -> Result< Self::Value, V::Error > {
                let mut output: BTreeMap< String, Value > = BTreeMap::new();
                while let Some( (key, value) ) = visitor.next_entry()? {
                    output.insert( key, value );
                }

                Ok( output.into() )
            }

            fn visit_bytes< E: de::Error >( self, value: &[u8] ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            fn visit_byte_buf< E: de::Error >( self, value: Vec< u8 > ) -> Result< Self::Value, E > {
                Ok( value.into() )
            }

            // Not really sure how (if?) to implement these at this point:

            // fn visit_newtype_struct< D: de::Deserializer< 'de > >( self, deserializer: D ) -> Result< Self::Value, D::Error > {
            //     unimplemented!();
            // }

            // fn visit_enum< V: de::EnumAccess >( self, visitor: V ) -> Result< Self::Value, V::Error > {
            //     unimplemented!();
            // }
        }

        deserializer.deserialize_any( ValueVisitor )
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum ConversionErrorKind {
    InvalidKey,
    NumberConversionError( number::ConversionError ),
    Custom( String )
}

/// A structure denoting a conversion error encountered during
/// serialization or deserialization.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConversionError {
    kind: ConversionErrorKind
}

impl ConversionError {
    fn invalid_key() -> Self {
        ConversionError {
            kind: ConversionErrorKind::InvalidKey
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        let message = error::Error::description( self );
        write!( formatter, "{}", message )
    }
}

impl error::Error for ConversionError {
    fn description( &self ) -> &str {
        match self.kind {
            ConversionErrorKind::InvalidKey => "key must be either a string or an integer",
            ConversionErrorKind::NumberConversionError( ref error ) => error.description(),
            ConversionErrorKind::Custom( ref message ) => message.as_str()
        }
    }
}

impl ser::Error for ConversionError {
    fn custom< T: fmt::Display >( message: T ) -> Self {
        ConversionError {
            kind: ConversionErrorKind::Custom( message.to_string() )
        }
    }
}

impl de::Error for ConversionError {
    fn custom< T: fmt::Display >( message: T ) -> Self {
        ConversionError {
            kind: ConversionErrorKind::Custom( message.to_string() )
        }
    }
}

impl From< number::ConversionError > for ConversionError {
    fn from( error: number::ConversionError ) -> Self {
        ConversionError {
            kind: ConversionErrorKind::NumberConversionError( error )
        }
    }
}

impl From< ConversionError > for value::ConversionError {
    fn from( error: ConversionError ) -> Self {
        match error.kind {
            ConversionErrorKind::InvalidKey => value::ConversionError::Custom( "key must be either a string or an integer".to_owned() ),
            ConversionErrorKind::NumberConversionError( error ) => error.into(),
            ConversionErrorKind::Custom( message ) => value::ConversionError::Custom( message )
        }
    }
}

#[derive(Debug)]
pub struct Serializer {
}

impl Serializer {
    pub fn new() -> Self {
        Serializer {}
    }
}

impl< 'a > ser::Serializer for &'a mut Serializer {
    type Ok = Value;
    type Error = ConversionError;
    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool( self, value: bool ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_i8( self, value: i8 )  -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_i16( self, value: i16 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_i32( self, value: i32 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_i64( self, value: i64 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.try_into()? )
    }

    fn serialize_u8( self, value: u8 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_u16( self, value: u16 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_u32( self, value: u32 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_u64( self, value: u64 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.try_into()? )
    }

    fn serialize_f32( self, value: f32 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_f64( self, value: f64 ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_char( self, value: char ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_str( self, value: &str ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_bytes( self, value: &[u8] ) -> Result< Self::Ok, Self::Error > {
        Ok( value.into() )
    }

    fn serialize_none( self ) -> Result< Self::Ok, Self::Error > {
        self.serialize_unit()
    }

    fn serialize_some< T: ?Sized + Serialize >( self, value: &T ) -> Result< Self::Ok, Self::Error > {
        value.serialize( self )
    }

    fn serialize_unit( self ) -> Result< Self::Ok, Self::Error > {
        Ok( Null.into() )
    }

    fn serialize_unit_struct( self, _name: &'static str ) -> Result< Self::Ok, Self::Error > {
        self.serialize_unit()
    }

    fn serialize_unit_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str ) -> Result< Self::Ok, Self::Error > {
        self.serialize_str( variant )
    }

    fn serialize_newtype_struct< T: ?Sized + Serialize >( self, _name: &'static str, value: &T ) -> Result< Self::Ok, Self::Error > {
        value.serialize( self )
    }

    fn serialize_newtype_variant< T: ?Sized + Serialize >( self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T ) -> Result< Self::Ok, Self::Error > {
        let mut object = BTreeMap::new();
        object.insert( String::from( variant ), to_value( &value )? );
        let object: Object = object.into();
        Ok( Value::Reference( object.into() ) )
    }

    fn serialize_seq( self, length: Option< usize > ) -> Result< Self::SerializeSeq, Self::Error > {
        Ok( SerializeVec {
            elements: Vec::with_capacity( length.unwrap_or( 0 ) )
        })
    }

    fn serialize_tuple( self, length: usize ) -> Result< Self::SerializeTuple, Self::Error > {
        self.serialize_seq( Some( length ) )
    }

    fn serialize_tuple_struct( self, _name: &'static str, length: usize ) -> Result< Self::SerializeTupleStruct, Self::Error > {
        self.serialize_seq( Some( length ) )
    }

    fn serialize_tuple_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, length: usize ) -> Result< Self::SerializeTupleVariant, Self::Error > {
        Ok( SerializeTupleVariant {
            name: String::from( variant ),
            elements: Vec::with_capacity( length ),
        })
    }

    fn serialize_map( self, _length: Option< usize > ) -> Result< Self::SerializeMap, Self::Error > {
        Ok( SerializeMap {
            map: BTreeMap::new(),
            next_key: None,
        })
    }

    fn serialize_struct( self, _name: &'static str, length: usize ) -> Result< Self::SerializeStruct, Self::Error > {
        self.serialize_map( Some( length ) )
    }

    fn serialize_struct_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, _length: usize ) -> Result< Self::SerializeStructVariant, Self::Error > {
        Ok( SerializeStructVariant {
            name: String::from( variant ),
            map: BTreeMap::new(),
        })
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[inline]
pub fn to_value< T: Serialize >( value: T ) -> Result< Value, ConversionError > {
    let mut serializer = Serializer {};
    value.serialize( &mut serializer )
}

#[doc(hidden)]
#[inline]
pub fn from_value< 'de, T: Deserialize< 'de > >( value: Value ) -> Result< T, ConversionError > {
    Deserialize::deserialize( value )
}

#[doc(hidden)]
#[derive(Debug)]
pub struct SerializeVec {
    elements: Vec< Value >,
}

#[doc(hidden)]
#[derive(Debug)]
pub struct SerializeTupleVariant {
    name: String,
    elements: Vec< Value >,
}

#[doc(hidden)]
#[derive(Debug)]
pub struct SerializeMap {
    map: BTreeMap< String, Value >,
    next_key: Option< String >,
}

#[doc(hidden)]
#[derive(Debug)]
pub struct SerializeStructVariant {
    name: String,
    map: BTreeMap< String, Value >,
}

impl ser::SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = ConversionError;

    #[inline]
    fn serialize_element< T: ?Sized + Serialize >( &mut self, value: &T ) -> Result< (), Self::Error > {
        self.elements.push( to_value( &value )? );
        Ok(())
    }

    #[inline]
    fn end( self ) -> Result< Self::Ok, Self::Error > {
        Ok( self.elements.into() )
    }
}

impl ser::SerializeTuple for SerializeVec {
    type Ok = Value;
    type Error = ConversionError;

    #[inline]
    fn serialize_element< T: ?Sized + Serialize >( &mut self, value: &T ) -> Result< (), Self::Error > {
        ser::SerializeSeq::serialize_element( self, value )
    }

    #[inline]
    fn end( self ) -> Result< Self::Ok, Self::Error > {
        ser::SerializeSeq::end( self )
    }
}

impl ser::SerializeTupleStruct for SerializeVec {
    type Ok = Value;
    type Error = ConversionError;

    fn serialize_field< T: ?Sized + Serialize >( &mut self, value: &T ) -> Result< (), Self::Error > {
        ser::SerializeSeq::serialize_element( self, value )
    }

    fn end( self ) -> Result< Self::Ok, Self::Error > {
        ser::SerializeSeq::end( self )
    }
}

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = ConversionError;

    fn serialize_field< T: ?Sized + Serialize >( &mut self, value: &T ) -> Result< (), Self::Error > {
        self.elements.push( to_value( &value )? );
        Ok(())
    }

    fn end( self ) -> Result< Self::Ok, Self::Error > {
        let mut object: BTreeMap< String, Value > = BTreeMap::new();
        object.insert( self.name, self.elements.into() );
        Ok( object.into() )
    }
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = ConversionError;

    fn serialize_key< T: ?Sized + Serialize >( &mut self, key: &T ) -> Result< (), Self::Error > {
        match to_value( &key )? {
            Value::String( string ) => self.next_key = Some( string ),
            Value::Number( number ) => {
                if let Ok( value ) = number.try_into() {
                    let value: u64 = value;
                    self.next_key = Some( value.to_string() );
                } else if let Ok( value ) = number.try_into() {
                    let value: i64 = value;
                    self.next_key = Some( value.to_string() );
                } else {
                    return Err( ConversionError::invalid_key() )
                }
            },
            _ => return Err( ConversionError::invalid_key() )
        }

        Ok(())
    }

    fn serialize_value< T: ?Sized + Serialize >( &mut self, value: &T ) -> Result< (), Self::Error > {
        let key = self.next_key.take();
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = key.expect( "serialize_value called before serialize_key" );
        self.map.insert( key, to_value( &value )? );
        Ok(())
    }

    fn end( self ) -> Result< Self::Ok, Self::Error > {
        Ok( self.map.into() )
    }
}

impl ser::SerializeStruct for SerializeMap {
    type Ok = Value;
    type Error = ConversionError;

    fn serialize_field< T: ?Sized + Serialize >( &mut self, key: &'static str, value: &T ) -> Result< (), Self::Error > {
        ser::SerializeMap::serialize_key( self, key )?;
        ser::SerializeMap::serialize_value( self, value )
    }

    fn end( self ) -> Result< Self::Ok, Self::Error > {
        ser::SerializeMap::end( self )
    }
}

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = ConversionError;

    fn serialize_field< T: ?Sized + Serialize >( &mut self, key: &'static str, value: &T ) -> Result< (), Self::Error > {
        self.map.insert( String::from( key ), to_value( &value )? );
        Ok(())
    }

    fn end( self ) -> Result< Self::Ok, Self::Error > {
        let mut object: BTreeMap< String, Value > = BTreeMap::new();
        object.insert( self.name, self.map.into() );
        Ok( object.into() )
    }
}

macro_rules! number_deserializer {
    ($([$name:ident $visitor:ident $type:ty])+) => {
        $(
            #[inline]
            fn $name< V: Visitor< 'de > >( self, visitor: V ) -> Result< V::Value, Self::Error > {
                let value: $type = self.try_into()?;
                visitor.$visitor( value )
            }
        )+
    };
}

impl< 'de > de::Deserializer< 'de > for Number {
    type Error = ConversionError;

    #[inline]
    fn deserialize_any< V: Visitor< 'de > >( self, visitor: V ) -> Result< V::Value, Self::Error > {
        match *get_storage( &self ) {
            number::Storage::I32( value ) => visitor.visit_i32( value ),
            number::Storage::F64( value ) => visitor.visit_f64( value )
        }
    }

    number_deserializer! {
        [deserialize_i8 visit_i8 i8]
        [deserialize_i16 visit_i16 i16]
        [deserialize_i32 visit_i32 i32]
        [deserialize_i64 visit_i64 i64]
        [deserialize_u8 visit_u8 u8]
        [deserialize_u16 visit_u16 u16]
        [deserialize_u32 visit_u32 u32]
        [deserialize_u64 visit_u64 u64]
        [deserialize_f64 visit_f64 f64]
    }

    forward_to_deserialize_any! {
        bool f32 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

// TODO: Impl for `&'a Number` and `&'a mut Number`.

impl Value {
    fn unexpected( &self ) -> de::Unexpected {
        match *self {
            Value::Undefined => de::Unexpected::Other( "undefined" ),
            Value::Null => de::Unexpected::Other( "null" ),
            Value::Bool( value ) => de::Unexpected::Bool( value ),
            Value::Number( ref value ) => {
                match *get_storage( value ) {
                    number::Storage::I32( value ) => de::Unexpected::Signed( value as i64 ),
                    number::Storage::F64( value ) => de::Unexpected::Float( value )
                }
            },
            Value::Symbol( _ ) => de::Unexpected::Other( "Symbol" ),
            Value::String( ref value ) => de::Unexpected::Str( value ),
            Value::Reference( _ ) => de::Unexpected::Other( "reference to a JavaScript value" )
        }
    }
}

macro_rules! value_proxy_number_deserializer {
    ($($name:ident)+) => {
        $(
            #[inline]
            fn $name< V: Visitor< 'de > >( self, visitor: V ) -> Result< V::Value, Self::Error > {
                match self {
                    Value::Number( value ) => value.$name( visitor ),
                    value => value.deserialize_any( visitor )
                }
            }
        )+
    };
}

impl< 'de > de::Deserializer< 'de > for Value {
    type Error = ConversionError;

    #[inline]
    fn deserialize_any< V: Visitor< 'de > >( self, visitor: V ) -> Result< V::Value, Self::Error > {
        match self {
            Value::Undefined => visitor.visit_unit(),
            Value::Null => visitor.visit_unit(),
            Value::Bool( value ) => visitor.visit_bool( value ),
            Value::Number( value ) => de::Deserializer::deserialize_any( value, visitor ),
            Value::Symbol( _ ) => unimplemented!( "Deserialization of symbols is unimplemented!" ),
            Value::String( value ) => visitor.visit_string( value ),
            Value::Reference( reference ) => {
                if Array::instance_of( &reference ) {
                    let value: Array = reference.try_into().unwrap();
                    let value: Vec< _ > = value.into();
                    let length = value.len();
                    let mut deserializer = SeqDeserializer::new( value );
                    let seq = visitor.visit_seq( &mut deserializer )?;
                    let remaining = deserializer.iter.len();
                    if remaining == 0 {
                        Ok( seq )
                    } else {
                        Err( de::Error::invalid_length( length, &"fewer elements in the array" ) )
                    }
                } else if Object::instance_of( &reference ) {
                    let value: Object = reference.try_into().unwrap();
                    let value: BTreeMap< _, _ > = value.into();
                    let length = value.len();
                    let mut deserializer = MapDeserializer::new( value );
                    let map = visitor.visit_map( &mut deserializer )?;
                    let remaining = deserializer.iter.len();
                    if remaining == 0 {
                        Ok( map )
                    } else {
                        Err( de::Error::invalid_length( length, &"fewer elements in the object" ) )
                    }
                } else {
                    unimplemented!(); // TODO: ?
                }
            }
        }
    }

    value_proxy_number_deserializer! {
        deserialize_i8
        deserialize_i16
        deserialize_i32
        deserialize_i64
        deserialize_u8
        deserialize_u16
        deserialize_u32
        deserialize_u64
        deserialize_f32
        deserialize_f64
    }

    #[inline]
    fn deserialize_option< V: Visitor< 'de > >( self, visitor: V ) -> Result< V::Value, Self::Error > {
        match self {
            Value::Undefined => visitor.visit_none(),
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some( self )
        }
    }

    #[inline]
    fn deserialize_enum< V: Visitor< 'de > >( self, _name: &str, _variants: &'static [&'static str], visitor: V ) -> Result< V::Value, Self::Error > {
        let (variant, value) = match self {
            Value::Reference( reference ) => {
                let value: Object = match reference.try_into() {
                    Ok( object ) => object,
                    Err( _ ) => return Err( de::Error::invalid_value( de::Unexpected::Map, &"map with a single key" ) )
                };

                let value: BTreeMap< _, _ > = value.into();
                let mut iter = value.into_iter();
                let (variant, value) = match iter.next() {
                    Some( value ) => value,
                    None => {
                        return Err( de::Error::invalid_value( de::Unexpected::Map, &"map with a single key" ) );
                    }
                };

                // Enums are encoded as objects with a single key:value pair.
                if iter.next().is_some() {
                    return Err( de::Error::invalid_value( de::Unexpected::Map, &"map with a single key" ) );
                }

                (variant, Some( value ))
            },
            Value::String( variant ) => (variant, None),
            other => {
                return Err( de::Error::invalid_type( other.unexpected(), &"string or map" ) );
            }
        };

        visitor.visit_enum( EnumDeserializer {
            variant: variant,
            value: value,
        })
    }

    #[inline]
    fn deserialize_newtype_struct< V: Visitor< 'de > >( self, _name: &'static str, visitor: V ) -> Result< V::Value, Self::Error > {
        visitor.visit_newtype_struct( self )
    }

    forward_to_deserialize_any! {
        bool char str string unit seq
        bytes byte_buf map unit_struct tuple_struct struct
        identifier tuple ignored_any
    }
}

struct EnumDeserializer {
    variant: String,
    value: Option< Value >,
}

impl< 'de > de::EnumAccess< 'de > for EnumDeserializer {
    type Error = ConversionError;
    type Variant = VariantDeserializer;

    fn variant_seed< V: de::DeserializeSeed< 'de > >( self, seed: V ) -> Result< (V::Value, VariantDeserializer), Self::Error > {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer { value: self.value };
        seed.deserialize( variant ).map( |v| (v, visitor) )
    }
}

struct VariantDeserializer {
    value: Option< Value >,
}

impl< 'de > de::VariantAccess< 'de > for VariantDeserializer {
    type Error = ConversionError;

    fn unit_variant( self ) -> Result< (), Self::Error > {
        match self.value {
            Some( value ) => de::Deserialize::deserialize( value ),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed< T: de::DeserializeSeed< 'de > >( self, seed: T ) -> Result< T::Value, Self::Error > {
        match self.value {
            Some( value ) => seed.deserialize( value ),
            None => Err( de::Error::invalid_type( de::Unexpected::UnitVariant, &"newtype variant" ) ),
        }
    }

    fn tuple_variant< V: Visitor< 'de > >( self, _length: usize, visitor: V ) -> Result< V::Value, Self::Error > {
        match self.value {
            Some( Value::Reference( reference ) ) => {
                let array: Array = match reference.try_into() {
                    Ok( array ) => array,
                    Err( _ ) => return Err( de::Error::invalid_type( de::Unexpected::UnitVariant, &"tuple variant" ) )
                };
                de::Deserializer::deserialize_any( SeqDeserializer::new( array.into() ), visitor )
            },
            Some( other ) => Err( de::Error::invalid_type( other.unexpected(), &"tuple variant" ) ),
            None => Err( de::Error::invalid_type( de::Unexpected::UnitVariant, &"tuple variant" ) )
        }
    }

    fn struct_variant< V: Visitor< 'de > >( self, _fields: &'static [&'static str], visitor: V ) -> Result< V::Value, Self::Error > {
        match self.value {
            Some( Value::Reference( reference ) ) => {
                let object: Object = match reference.try_into() {
                    Ok( object ) => object,
                    Err( _ ) => return Err( de::Error::invalid_type( de::Unexpected::UnitVariant, &"struct variant" ) )
                };
                de::Deserializer::deserialize_any( MapDeserializer::new( object.into() ), visitor )
            },
            Some( other ) => Err( de::Error::invalid_type( other.unexpected(), &"struct variant" ) ),
            _ => Err( de::Error::invalid_type( de::Unexpected::UnitVariant, &"struct variant" ) )
        }
    }
}

struct SeqDeserializer {
    iter: vec::IntoIter< Value >,
}

impl SeqDeserializer {
    fn new( vec: Vec< Value >) -> Self {
        SeqDeserializer {
            iter: vec.into_iter(),
        }
    }
}

impl< 'de > de::Deserializer< 'de > for SeqDeserializer {
    type Error = ConversionError;

    #[inline]
    fn deserialize_any< V: Visitor< 'de > >( mut self, visitor: V ) -> Result< V::Value, Self::Error > {
        let length = self.iter.len();
        if length == 0 {
            visitor.visit_unit()
        } else {
            let ret = visitor.visit_seq( &mut self )?;
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok( ret )
            } else {
                Err( de::Error::invalid_length( length, &"fewer elements in array" ) )
            }
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

impl< 'de > de::SeqAccess< 'de > for SeqDeserializer {
    type Error = ConversionError;

    fn next_element_seed< T: de::DeserializeSeed< 'de > >( &mut self, seed: T ) -> Result< Option< T::Value >, Self::Error > {
        match self.iter.next() {
            Some( value ) => seed.deserialize( value ).map( Some ),
            None => Ok( None ),
        }
    }

    fn size_hint( &self ) -> Option< usize > {
        match self.iter.size_hint() {
            (lower, Some( upper )) if lower == upper => Some( upper ),
            _ => None
        }
    }
}

struct MapDeserializer {
    iter: <BTreeMap< String, Value > as IntoIterator>::IntoIter,
    value: Option< Value >,
}

impl MapDeserializer {
    fn new( map: BTreeMap< String, Value > ) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}

impl< 'de > de::MapAccess< 'de > for MapDeserializer {
    type Error = ConversionError;

    fn next_key_seed< T: de::DeserializeSeed< 'de > >( &mut self, seed: T ) -> Result< Option< T::Value >, Self::Error> {
        match self.iter.next() {
            Some( (key, value) ) => {
                self.value = Some( value );
                seed.deserialize( key.into_deserializer() ).map( Some )
            }
            None => Ok( None )
        }
    }

    fn next_value_seed< T: de::DeserializeSeed< 'de > >( &mut self, seed: T ) -> Result< T::Value, Self::Error > {
        match self.value.take() {
            Some( value ) => seed.deserialize( value ),
            None => Err( de::Error::custom( "value is missing" ) ),
        }
    }

    fn size_hint( &self ) -> Option< usize > {
        match self.iter.size_hint() {
            (lower, Some( upper )) if lower == upper => Some( upper ),
            _ => None
        }
    }
}

impl< 'de > de::Deserializer< 'de > for MapDeserializer {
    type Error = ConversionError;

    #[inline]
    fn deserialize_any< V: Visitor< 'de > >( self, visitor: V ) -> Result< V::Value, Self::Error > {
        visitor.visit_map( self )
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __js_serializable_serde_boilerplate {
    (($($impl_arg:tt)*) ($($kind_arg:tt)*) ($($bounds:tt)*)) => {
        __js_serializable_boilerplate!( ($($impl_arg)*) ($($kind_arg)*) ($($bounds)*) );

        impl< $($impl_arg),* > $crate::private::JsSerialize for $($kind_arg)* where $($bounds)* {
            #[inline]
            fn _into_js< 'x >( &'x self ) -> $crate::private::SerializedValue< 'x > {
                let value = $crate::private::to_value( self ).unwrap();
                $crate::private::serialize_value( value )
            }
        }

        impl< $($impl_arg),* > $crate::unstable::TryFrom< $($kind_arg)* > for $crate::Value where $($bounds)* {
            type Error = $crate::serde::ConversionError;
            #[inline]
            fn try_from( value: $($kind_arg)* ) -> Result< Self, Self::Error > {
                $crate::private::to_value( value )
            }
        }

        impl< '_a, $($impl_arg),* > $crate::unstable::TryFrom< &'_a $($kind_arg)* > for $crate::Value where $($bounds)* {
            type Error = $crate::serde::ConversionError;
            #[inline]
            fn try_from( value: &'_a $($kind_arg)* ) -> Result< Self, Self::Error > {
                $crate::private::to_value( value )
            }
        }

        impl< '_a, $($impl_arg),* > $crate::unstable::TryFrom< &'_a mut $($kind_arg)* > for $crate::Value where $($bounds)* {
            type Error = $crate::serde::ConversionError;
            #[inline]
            fn try_from( value: &'_a mut $($kind_arg)* ) -> Result< Self, Self::Error > {
                $crate::private::to_value( value )
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __js_deserializable_serde_boilerplate {
    (($($impl_arg:tt)*) ($($kind_arg:tt)*) ($($bounds:tt)*)) => {
        impl< $($impl_arg),* > $crate::unstable::TryFrom< $crate::Value > for $($kind_arg)* where $($bounds)* {
            type Error = $crate::serde::ConversionError;
            #[inline]
            fn try_from( value: $crate::Value ) -> Result< Self, Self::Error > {
                $crate::private::from_value( value )
            }
        }
    }
}

/// A macro which makes it possible to pass an instance of a given type
/// implementing Serde's `Serialize` into the [js!](macro.js.html) macro.
///
/// For types defined outside of your crate you can also use the [Serde](serde/struct.Serde.html)
/// newtype to make them serializable indirectly.
///
/// # Examples
///
/// ```
/// #[derive(Serialize, Debug)]
/// struct Person {
///     name: String,
///     age: i32
/// }
///
/// js_serializable!( Person );
///
/// let person = Person {
///    name: "Bob".to_owned(),
///    age: 33
/// };
///
/// js! {
///     var person = @{person};
///     console.log( person.name + " is " + person.age + " years old." );
/// };
/// ```
///
/// This macro also accepts generics:
///
/// ```
/// trait Foobar {}
///
/// #[derive(Serialize)]
/// struct Wrapper< 'a, T: Serialize + 'a >( &'a T );
///
/// js_serializable!( impl< 'a, T > for Wrapper< 'a, T > where T: Serialize + Foobar );
/// ```
#[macro_export]
macro_rules! js_serializable {
    ($kind:tt) => {
        $crate::__js_serializable_serde_boilerplate!( () ($kind) () );
    };

    (impl< $($impl_arg:tt),* > for $kind:ty where $($bounds:tt)*) => {
        $crate::__js_serializable_serde_boilerplate!( ($($impl_arg),*) ($kind) ($($bounds)*) );
    };

    (impl< $($impl_arg:tt),* > for $kind:ty) => {
        $crate::__js_serializable_serde_boilerplate!( ($($impl_arg),*) ($kind) () );
    };
}

/// A macro which makes it possible to convert an instance of a given type
/// implementing Serde's `Deserialize` into a [Value](enum.Value.html) using
/// [TryInto](unstable/trait.TryInto.html).
///
/// For types defined outside of your crate you can also use the [Serde](serde/struct.Serde.html)
/// newtype to make them deserializable indirectly.
///
/// # Examples
///
/// ```
/// #[derive(Deserialize, Debug)]
/// struct Person {
///     name: String,
///     age: i32
/// }
///
/// js_deserializable!( Person );
///
/// let value = js! {
///     return {
///         name: "Bob",
///         age: 33
///     };
/// };
///
/// let structure: Person = value.try_into().unwrap();
/// assert_eq!( structure.name, "Bob" );
/// assert_eq!( structure.age, 33 );
/// ```
///
/// This macro also accepts generics just as the [js_serializable!](macro.js_serializable.html) does.
#[macro_export]
macro_rules! js_deserializable {
    ($kind:tt) => {
        $crate::__js_deserializable_serde_boilerplate!( () ($kind) () );
    };

    (impl< $($impl_arg:tt),* > for $kind:ty where $($bounds:tt)*) => {
        $crate::__js_deserializable_serde_boilerplate!( ($($impl_arg),*) ($kind) ($($bounds)*) );
    };

    (impl< $($impl_arg:tt),* > for $kind:ty) => {
        $crate::__js_deserializable_serde_boilerplate!( ($($impl_arg),*) ($kind) () );
    };
}

/// A newtype which makes it possible to pass a value which implements
/// Serde's `Serializable` into the [js!](macro.js.html) macro.
///
/// For types defined in your crate you can also use the [js_serializable!](macro.js_serializable.html)
/// macro to make them serializable directly.
///
/// # Examples
///
/// ```
/// #[derive(Serialize, Debug)]
/// struct Person {
///     name: String,
///     age: i32
/// }
///
/// let person = Person {
///    name: "Bob".to_owned(),
///    age: 33
/// };
///
/// js! {
///     var person = @{Serde( person )};
///     console.log( person.name + " is " + person.age + " years old." );
/// };
/// ```
pub struct Serde< T >( pub T );

impl< T: fmt::Debug > fmt::Debug for Serde< T > {
    #[inline]
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        self.0.fmt( formatter )
    }
}

impl< T: Serialize > JsSerialize for Serde< T > {
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        let value = to_value( &self.0 ).unwrap();
        global_arena::serialize_value( value )
    }
}

impl< T: Serialize > TryFrom< Serde< T > > for Value {
    type Error = ConversionError;
    #[inline]
    fn try_from( value: Serde< T > ) -> Result< Self, Self::Error > {
        to_value( &value.0 )
    }
}

impl< 'a, T: Serialize > TryFrom< &'a Serde< T > > for Value {
    type Error = ConversionError;
    #[inline]
    fn try_from( value: &'a Serde< T > ) -> Result< Self, Self::Error > {
        to_value( &value.0 )
    }
}

impl< 'a, T: Serialize > TryFrom< &'a mut Serde< T > > for Value {
    type Error = ConversionError;
    #[inline]
    fn try_from( value: &'a mut Serde< T > ) -> Result< Self, Self::Error > {
        to_value( &value.0 )
    }
}

impl< 'de, T: Deserialize< 'de > > TryFrom< Value > for Serde< T > {
    type Error = ConversionError;
    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        Ok( Serde( from_value( value )? ) )
    }
}

impl< 'de, T: Deserialize< 'de > > TryFrom< Value > for Option< Serde< T > > {
    type Error = ConversionError;
    #[inline]
    fn try_from( value: Value ) -> Result< Self, Self::Error > {
        match value {
            Value::Undefined | Value::Null => Ok( None ),
            value => value.try_into().map( Some )
        }
    }
}

__js_serializable_boilerplate!( impl< T > for Serde< T > where T: Serialize );

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_undefined() {
        // This is technically incorrect as `undefined` is not serializable into JSON,
        // but serde is generic so it serialized it anyway.
        assert_eq!( serde_json::to_string( &Undefined ).unwrap(), "null" );
    }

    #[test]
    fn serialize_null() {
        assert_eq!( serde_json::to_string( &Null ).unwrap(), "null" );
    }

    #[test]
    fn serialize_number_negative() {
        let value: Number = (-123_i32).into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "-123" );
    }

    #[test]
    fn serialize_number_positive() {
        let value: Number = 123_i32.into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "123" );
    }

    #[test]
    fn serialize_number_float() {
        let value: Number = 3.33_f64.into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "3.33" );
    }

    #[test]
    fn serialize_value_undefined() {
        let value: Value = Undefined.into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "null" );
    }

    #[test]
    fn serialize_value_null() {
        let value: Value = Null.into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "null" );
    }

    #[test]
    fn serialize_value_bool_true() {
        let value: Value = true.into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "true" );
    }

    #[test]
    fn serialize_value_bool_false() {
        let value: Value = false.into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "false" );
    }

    #[test]
    fn serialize_value_number() {
        let value: Value = (123_i32).into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "123" );
    }

    #[test]
    fn serialize_value_string() {
        let value: Value = "死神はりんごしか食べない".into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "\"死神はりんごしか食べない\"" );
    }

    #[test]
    fn serialize_value_array() {
        let value: Value = (&[true, false][..]).into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "[true,false]" );
    }

    #[test]
    fn serialize_value_object() {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        map.insert( "1", "one" );
        map.insert( "2", "two" );

        let value: Value = map.into();
        assert_eq!( serde_json::to_string( &value ).unwrap(), "{\"1\":\"one\",\"2\":\"two\"}" );
    }

    #[test]
    fn deserialize_value_null() {
        let value: Value = serde_json::from_str( "null" ).unwrap();
        assert_eq!( value, Value::Null );
    }

    #[test]
    fn deserialize_value_bool_false() {
        let value: Value = serde_json::from_str( "false" ).unwrap();
        assert_eq!( value, Value::Bool( false ) );
    }

    #[test]
    fn deserialize_value_bool_true() {
        let value: Value = serde_json::from_str( "true" ).unwrap();
        assert_eq!( value, Value::Bool( true ) );
    }

    #[test]
    fn deserialize_value_number_integer() {
        let value: Value = serde_json::from_str( "33" ).unwrap();
        assert_eq!( value, Value::Number( 33.into() ) );
    }

    #[test]
    fn deserialize_value_number_float() {
        let value: Value = serde_json::from_str( "33.33" ).unwrap();
        assert_eq!( value, Value::Number( 33.33.into() ) );
    }

    #[test]
    fn deserialize_value_string() {
        let value: Value = serde_json::from_str( "\"Bob\"" ).unwrap();
        assert_eq!( value, Value::String( "Bob".to_owned() ) );
    }

    #[test]
    fn deserialize_value_array() {
        let value: Value = serde_json::from_str( "[true, false]" ).unwrap();
        assert_eq!( value.is_array(), true );

        let value: Vec< Value > = value.try_into().unwrap();
        assert_eq!( value, vec![ Value::Bool( true ), Value::Bool( false ) ] );
    }

    #[test]
    fn deserialize_value_object() {
        let value: Value = serde_json::from_str( "{\"1\":\"one\",\"2\":\"two\"}" ).unwrap();
        let mut map: BTreeMap< String, Value > = BTreeMap::new();
        map.insert( "1".to_owned(), Value::String( "one".to_owned() ) );
        map.insert( "2".to_owned(), Value::String( "two".to_owned() ) );

        let value: BTreeMap< _, _ > = value.into_object().unwrap().into();
        assert_eq!( value, map );
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Structure {
        number: i32,
        string: String
    }

    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    struct StructureSerializable {
        number: i32,
        string: String
    }

    js_serializable!( StructureSerializable );
    js_deserializable!( StructureSerializable );

    #[test]
    fn serialization_into_value_through_macro() {
        let structure = StructureSerializable {
            number: 123,
            string: "Hello!".to_owned()
        };

        let value: Value = structure.try_into().unwrap();
        let mut map = BTreeMap::new();
        map.insert( "number".to_owned(), Value::Number( 123.into() ) );
        map.insert( "string".to_owned(), Value::String( "Hello!".to_owned() ) );

        let value: BTreeMap< _, _ > = value.into_object().unwrap().into();
        assert_eq!( value, map );
    }

    #[test]
    fn serialization_into_javascript_through_macro() {
        let structure = StructureSerializable {
            number: 123,
            string: "Hello!".to_owned()
        };

        let result = js! {
            var object = @{structure};
            return object.number === 123 && object.string === "Hello!" && Object.keys( object ).length == 2;
        };

        assert_eq!( result, true );
    }

    #[test]
    fn serialization_into_value_through_newtype() {
        let structure = Structure {
            number: 123,
            string: "Hello!".to_owned()
        };

        let value: Value = Serde( structure ).try_into().unwrap();
        let mut map = BTreeMap::new();
        map.insert( "number".to_owned(), Value::Number( 123.into() ) );
        map.insert( "string".to_owned(), Value::String( "Hello!".to_owned() ) );

        let value: BTreeMap< _, _ > = value.into_object().unwrap().into();
        assert_eq!( value, map );
    }

    #[test]
    fn serialization_into_javascript_through_newtype() {
        let structure = Structure {
            number: 123,
            string: "Hello!".to_owned()
        };

        let result = js! {
            var object = @{Serde( structure )};
            return object.number === 123 && object.string === "Hello!" && Object.keys( object ).length == 2;
        };

        assert_eq!( result, true );
    }

    #[test]
    fn deserialization_into_value_through_macro() {
        let value = js! {
            return {
                number: 123,
                string: "Hello!"
            };
        };

        let structure: StructureSerializable = value.try_into().unwrap();
        assert_eq!( structure.number, 123 );
        assert_eq!( structure.string, "Hello!" );
    }

    #[test]
    fn deserialization_into_value_through_newtype() {
        let value = js! {
            return {
                number: 123,
                string: "Hello!"
            };
        };

        let structure: Serde< Structure > = value.try_into().unwrap();
        assert_eq!( structure.0.number, 123 );
        assert_eq!( structure.0.string, "Hello!" );
    }

    #[test]
    fn deserialization_into_option_through_newtype() {
        let value = js! {
            return {
                number: 123,
                string: "Hello!"
            };
        };

        let structure: Option< Serde< Structure > > = value.try_into().unwrap();
        let structure = structure.unwrap();
        assert_eq!( structure.0.number, 123 );
        assert_eq!( structure.0.string, "Hello!" );

        let structure: Option< Serde< Structure > > = Value::Null.try_into().unwrap();
        assert!( structure.is_none() );
    }

    #[test]
    fn serialization_and_deserialization_of_btreemap_with_serializable_values() {
        let mut original = BTreeMap::new();
        original.insert( "key", StructureSerializable {
            number: 123,
            string: "Hello!".to_owned()
        });

        let value: Value = (&original).into();
        let deserialized: BTreeMap< String, StructureSerializable > = value.try_into().unwrap();

        assert_eq!( original.len(), deserialized.len() );
        assert_eq!( original.get( "key" ).unwrap(), deserialized.get( "key" ).unwrap() );
    }

    #[test]
    fn serialization_and_deserialization_of_array_with_serializable_elements() {
        let original = vec![ StructureSerializable {
            number: 123,
            string: "Hello!".to_owned()
        }];

        let value: Value = (&original).into();
        let deserialized: Vec< StructureSerializable > = value.try_into().unwrap();

        assert_eq!( original.len(), deserialized.len() );
        assert_eq!( original[ 0 ], deserialized[ 0 ] );
    }

    #[test]
    fn deserialization_of_a_big_number() {
        #[derive(Deserialize, Debug)]
        struct Struct {
            number: u64
        }

        let structure: Serde< Struct > = js!( return { number: 1535164942454 }; ).try_into().unwrap();
        assert_eq!( structure.0.number, 1535164942454 );
    }

    #[test]
    fn deserialization_of_a_very_big_number() {
        #[derive(Deserialize, Debug)]
        struct Struct {
            number: u64
        }

        let structure: Serde< Struct > = js!( return { number: 9223372049167088120 }; ).try_into().unwrap();
        assert_eq!( structure.0.number, 9223372049167087616 );
    }
}
