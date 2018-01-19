use webcore::value::{Reference, Value, ConversionError};
use webapi::node_list::NodeList;
use webcore::try_from::{TryFrom, TryInto};
use webapi::node::{INode, Node};


/// `MutationObserver` provides developers with a way to react to changes in a DOM.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)
pub struct MutationObserver( Reference );

reference_boilerplate! {
    MutationObserver,
    instanceof MutationObserver
}


pub struct MutationObserverInit<'a> {
    child_list: bool,
    attribute: bool,
    character_data: bool,
    subtree: bool,
    attribute_old_value: bool,
    character_data_old_value: bool,
    attribute_filter: &'a [ &'a str ],
}


impl MutationObserver {
    // TODO implement second argument for callback
    fn new< F >( callback: F ) -> Self
        where F: FnMut( Vec< MutationRecord > ) + 'static {
        js! (
            return new MutationObserver( @{callback} );
        ).try_into().unwrap()
    }

    fn observe< T: INode >( &self, target: &T, options: MutationObserverInit ) {
        js! { @(no_return)
            @{self.as_ref()}.observe( @{target.as_ref()}, {
                childList: @{options.child_list},
                attributes: @{options.attribute},
                characterData: @{options.character_data},
                subtree: @{options.subtree},
                attributeOldValue: @{options.attribute_old_value},
                characterDataOldValue: @{options.character_data_old_value},
                attributeFilter: @{options.attribute_filter.from()}
            } );
        }
    }

    fn disconnect( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.disconnect();
        }
    }

    fn take_records( &self ) -> Vec< MutationRecord > {
        js!(
            return @{self.as_ref()}.takeRecords();
        ).try_into().unwrap()
    }
}


#[derive(Debug)]
pub enum MutationRecord {
    Attribute {
        target: Node,
        attribute_name: Option< String >,
        attribute_namespace: Option< String >,
        old_value: Option< String >,
    },

    CharacterData {
        target: Node,
        old_data: Option< String >,
    },

    ChildList {
        target: Node,
        added_nodes: NodeList,
        removed_nodes: NodeList,
        previous_sibling: Option< Node >,
        next_sibling: Option< Node >,
    },
}


impl TryFrom< Value > for MutationRecord {
    type Error = ConversionError;

    fn try_from( v: Value ) -> Result< Self, Self::Error > {
        match v {
            Value::Reference( ref r ) => {
                // TODO propagate errors with ?
                let _type: String = js!( @{r}.type ).try_into().unwrap();

                // TODO propagate errors with ?
                let target: Node = js!( @{r}.target ).try_into().unwrap();

                match _type.as_str() {
                    "attributes" => Ok( MutationRecord::Attribute {
                        target: target,
                        attribute_name: js!( @{r}.attributeName ).try_into()?,
                        attribute_namespace: js!( @{r}.attributeNamespace ).try_into()?,
                        old_value: js!( @{r}.oldValue ).try_into()?,
                    } ),

                    "characterData" => Ok( MutationRecord::CharacterData {
                        target: target,
                        old_data: js!( @{r}.oldValue ).try_into()?,
                    } ),

                    "childList" => Ok( MutationRecord::ChildList {
                        target: target,

                        // TODO propagate errors with ?
                        added_nodes: js!( @{r}.addedNodes ).try_into().unwrap(),

                        // TODO propagate errors with ?
                        removed_nodes: js!( @{r}.removedNodes ).try_into().unwrap(),

                        previous_sibling: js!( @{r}.previousSibling ).try_into()?,

                        next_sibling: js!( @{r}.nextSibling ).try_into()?,
                    } ),

                    other => Err( ConversionError::Custom( format!( "Unknown MutationRecord type: {:?}", other ) ) )
                }
            },
            other => Err( ConversionError::Custom( format!( "Expected MutationRecord but got: {:?}", other ) ) )
        }
    }
}
