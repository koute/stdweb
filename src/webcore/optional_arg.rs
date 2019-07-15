use webcore::try_from::{TryFrom, TryInto};

#[derive(Debug)]
pub enum OptionalArg< T > {
    Some( T ),
    None
}

impl< T > From< Option< T > > for OptionalArg< T > {
    fn from( value: Option< T > ) -> Self {
        match value {
            Some( value ) => OptionalArg::Some( value ),
            None => OptionalArg::None
        }
    }
}

impl< T > OptionalArg< T > {
    pub fn as_ref( &self ) -> OptionalArg< &T > {
        match *self {
            OptionalArg::Some( ref value ) => OptionalArg::Some( value ),
            OptionalArg::None => OptionalArg::None
        }
    }
}

impl< T, U > TryFrom< Option< T > > for OptionalArg< U >
    where T: TryInto< U >
{
    type Error = T::Error;
    fn try_from( value: Option< T > ) -> Result< OptionalArg< U >, Self::Error > {
        let result = match value {
            Some( value ) => OptionalArg::Some( value.try_into()? ),
            None => OptionalArg::None
        };

        Ok( result )
    }
}

#[test]
fn test_try_into_optional_arg() {
    use webcore::number::Number;

    let value: Option< u64 > = Some( 123 );
    let value: OptionalArg< Number > = value.try_into().unwrap();
    assert_eq!( js!( return @{value}; ), 123 );
}
