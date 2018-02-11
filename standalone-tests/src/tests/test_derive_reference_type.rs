use utils::*;

#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "Error")]
pub struct Error( ::stdweb::Reference );

pub fn run() {
    test( "custom_reference_type", || {
        use stdweb::unstable::TryInto;
        let value = js! { return new ReferenceError(); };
        let _: Error = value.try_into().unwrap();
    });
}
