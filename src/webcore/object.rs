use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use webcore::try_from::{TryFrom, TryInto};
use webcore::value::{Reference, Value, ConversionError};
use webcore::serialization::{JsSerialize, deserialize_object};

/// A type representing a JavaScript object.
#[derive(Clone, PartialEq, Eq, Debug, ReferenceType)]
#[reference(instance_of = "Object")]
pub struct Object( Reference );

impl Object {
    /// Returns the number of elements in this particular object.
    pub fn len( &self ) -> usize {
        js!(
            return Object.keys( @{self} ).length;
        ).try_into().unwrap()
    }
}

impl From< Object > for BTreeMap< String, Value > {
    fn from( object: Object ) -> Self {
        deserialize_object( &object.0, |iter| iter.collect() )
    }
}

impl< 'a > From< &'a Object > for BTreeMap< String, Value > {
    fn from( object: &'a Object ) -> Self {
        deserialize_object( &object.0, |iter| iter.collect() )
    }
}

impl< 'a > From< &'a mut Object > for BTreeMap< String, Value > {
    fn from( object: &'a mut Object ) -> Self {
        deserialize_object( &object.0, |iter| iter.collect() )
    }
}

impl From< Object > for HashMap< String, Value > {
    fn from( object: Object ) -> Self {
        deserialize_object( &object.0, |iter| iter.collect() )
    }
}

impl< 'a > From< &'a Object > for HashMap< String, Value > {
    fn from( object: &'a Object ) -> Self {
        deserialize_object( &object.0, |iter| iter.collect() )
    }
}

impl< 'a > From< &'a mut Object > for HashMap< String, Value > {
    fn from( object: &'a mut Object ) -> Self {
        deserialize_object( &object.0, |iter| iter.collect() )
    }
}

impl< K: AsRef< str >, V: JsSerialize > From< BTreeMap< K, V > > for Object {
    #[inline]
    fn from( value: BTreeMap< K, V > ) -> Self {
        (&value).into()
    }
}

impl< 'a, K, V > From< &'a BTreeMap< K, V > > for Object where K: AsRef< str >, V: JsSerialize {
    #[inline]
    fn from( value: &'a BTreeMap< K, V > ) -> Self {
        // TODO: Do something more efficient here?
        let value = js! {
            return @{value};
        };

        match value {
            Value::Reference( reference ) => Object( reference ),
            _ => unreachable!()
        }
    }
}

impl< 'a, K, V > From< &'a mut BTreeMap< K, V > > for Object where K: AsRef< str >, V: JsSerialize {
    #[inline]
    fn from( value: &'a mut BTreeMap< K, V > ) -> Self {
        let value: &BTreeMap< K, V > = value;
        value.into()
    }
}

impl< K, V > From< HashMap< K, V > > for Object where K: AsRef< str > + Hash + Eq, V: JsSerialize {
    #[inline]
    fn from( value: HashMap< K, V > ) -> Self {
        (&value).into()
    }
}

impl< 'a, K, V > From< &'a HashMap< K, V > > for Object where K: AsRef< str > + Hash + Eq, V: JsSerialize {
    #[inline]
    fn from( value: &'a HashMap< K, V > ) -> Self {
        // TODO: Do something more efficient here?
        let value = js! {
            return @{value};
        };

        match value {
            Value::Reference( reference ) => Object( reference ),
            _ => unreachable!()
        }
    }
}

impl< 'a, K: Hash + Eq, V > From< &'a mut HashMap< K, V > > for Object where K: AsRef< str >, V: JsSerialize {
    #[inline]
    fn from( value: &'a mut HashMap< K, V > ) -> Self {
        let value: &HashMap< K, V > = value;
        value.into()
    }
}


impl< E: Into< ConversionError >, V: TryFrom< Value, Error = E > > TryFrom< Object > for BTreeMap< String, V > {
    type Error = ConversionError;

    fn try_from( object: Object ) -> Result< Self, Self::Error > {
        deserialize_object( object.as_ref(), |deserializer| -> Result< BTreeMap< String, V >, E > {
            let mut output = BTreeMap::new();
            for (key, value) in deserializer {
                output.insert( key, value.try_into()? );
            }
            Ok( output )
        }).map_err( |err| err.into() )
    }
}

impl< E: Into< ConversionError >, V: TryFrom< Value, Error = E > > TryFrom< Object > for HashMap< String, V > {
    type Error = ConversionError;

    fn try_from( object: Object ) -> Result< Self, Self::Error > {
        deserialize_object( object.as_ref(), |deserializer| -> Result< HashMap< String, V >, E > {
            let mut output = HashMap::with_capacity( deserializer.len() );
            for (key, value) in deserializer {
                output.insert( key, value.try_into()? );
            }

            Ok( output )
        }).map_err( |err| err.into() )
    }
}
