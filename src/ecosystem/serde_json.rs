use std::collections::BTreeMap;
use serde_json::value::Value as JsonValue;
use webcore::value::Value;
use webcore::try_from::{TryFrom, TryInto};
use webcore::number::ConversionError;

impl TryFrom< JsonValue > for Value {
    type Error = ConversionError;

    #[inline]
    fn try_from( value: JsonValue ) -> Result< Self, Self::Error > {
        let result = match value {
            JsonValue::Null => Value::Null,
            JsonValue::Bool( value ) => Value::Bool( value ),
            JsonValue::Number( value ) => {
                if let Some( value ) = value.as_u64() {
                    Value::Number( value.try_into()? )
                } else if let Some( value ) = value.as_i64() {
                    Value::Number( value.try_into()? )
                } else {
                    Value::Number( value.as_f64().unwrap().into() )
                }
            },
            JsonValue::String( value ) => Value::String( value ),
            JsonValue::Array( value ) => {
                let mut vector: Vec< Value > = Vec::new();

                vector.reserve( value.len() );
                for element in value.into_iter() {
                    vector.push( element.try_into()? );
                }

                vector.into()
            },
            JsonValue::Object( value ) => {
                let mut map: BTreeMap< String, Value > = BTreeMap::new();
                for (key, value) in value.into_iter() {
                    map.insert( key.into(), value.try_into()? );
                }

                map.into()
            }
        };

        Ok( result )
    }
}
