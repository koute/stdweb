use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use webcore::try_from::{TryFrom, TryInto};
use webcore::value::{Reference, Value, ConversionError};
use webcore::serialization::{JsSerialize, deserialize_object, deserialize_object_to_iter};

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

    /// Retrieves an iterator over this object's keys and values.
    ///
    /// When called, this method will pull all the objecy's keys and values
    /// from JavaScript, then return an iterator which accesses them in pairs.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
    /// # use std::collections::HashMap;
    /// use stdweb::{ js, unstable::TryInto, Object };
    ///
    /// let obj: Object = js!( return { [1]: 2 } ).try_into()?;
    ///
    /// let map: HashMap< i32, i32 > = obj
    ///     .to_iter()
    ///     .map( |(k, v)| Ok( ( k.parse()?, v.try_into()? ) ) )
    ///     .collect::< Result< _, Box< dyn std::error::Error > > >()?;
    ///
    /// assert_eq!( map[ &1 ], 2 );
    /// panic!("ahhh");
    /// # Ok( () )
    /// # }
    /// ```
    pub fn to_iter( &self ) -> impl ExactSizeIterator < Item = ( String, Value ) > {
        deserialize_object_to_iter( self.as_ref() )
    }

    /// Returns `true` if object has no elements.
    pub fn is_empty( &self ) -> bool {
        self.len() == 0
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

impl< S: std::hash::BuildHasher + Default > From< Object > for HashMap< String, Value, S > {
    fn from( object: Object ) -> Self {
        deserialize_object( &object.0, |iter| iter.collect() )
    }
}

impl< 'a, S: std::hash::BuildHasher + Default > From< &'a Object > for HashMap< String, Value, S > {
    fn from( object: &'a Object ) -> Self {
        deserialize_object( &object.0, |iter| iter.collect() )
    }
}

impl< 'a, S: std::hash::BuildHasher + Default > From< &'a mut Object > for HashMap< String, Value, S > {
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

impl< K, V, S: std::hash::BuildHasher > From< HashMap< K, V, S > > for Object where K: AsRef< str > + Hash + Eq, V: JsSerialize {
    #[inline]
    fn from( value: HashMap< K, V, S > ) -> Self {
        (&value).into()
    }
}

impl< 'a, K, V, S: std::hash::BuildHasher > From< &'a HashMap< K, V, S > > for Object where K: AsRef< str > + Hash + Eq, V: JsSerialize {
    #[inline]
    fn from( value: &'a HashMap< K, V, S > ) -> Self {
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

impl< 'a, K: Hash + Eq, V, S: std::hash::BuildHasher > From< &'a mut HashMap< K, V, S > > for Object where K: AsRef< str >, V: JsSerialize {
    #[inline]
    fn from( value: &'a mut HashMap< K, V, S > ) -> Self {
        let value: &HashMap< K, V, S > = value;
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

impl< E: Into< ConversionError >, V: TryFrom< Value, Error = E >, S: std::hash::BuildHasher + Default > TryFrom< Object > for HashMap< String, V, S > {
    type Error = ConversionError;

    fn try_from( object: Object ) -> Result< Self, Self::Error > {
        deserialize_object( object.as_ref(), |deserializer| -> Result< HashMap< String, V, S >, E > {
            let mut output = HashMap::with_capacity_and_hasher( deserializer.len(), Default::default() );
            for (key, value) in deserializer {
                output.insert( key, value.try_into()? );
            }

            Ok( output )
        }).map_err( |err| err.into() )
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use stdweb::{ js, unstable::TryInto, Object };

    /// This duplicates and tests the example in `Object::to_iter` documentation.
    #[test]
    fn test_object_to_iter() {
        let obj: Object = js!( return { [1]: 2 } ).try_into().unwrap();

        let map: HashMap< i32, i32 > = obj
            .to_iter()
            .map( |(k, v)| Ok( ( k.parse()?, v.try_into()? ) ) )
            .collect::< Result< _, Box< dyn std::error::Error > > >().unwrap();

        assert_eq!( map[ &1 ], 2 );
    }
}
