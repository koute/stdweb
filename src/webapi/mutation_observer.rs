use webcore::value::{Reference, Value, ConversionError};
use webapi::node_list::NodeList;
use webcore::try_from::{TryFrom, TryInto};
use webapi::node::{INode, Node};


/// `MutationObserver` provides a way to receive notifications about changes in the DOM.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)
pub struct MutationObserver( Reference );

reference_boilerplate! {
    MutationObserver,
    instanceof MutationObserver
}


/// Specifies which changes should be observed for the target.
///
/// Only used with the [`MutationObserver::observe`](struct.MutationObserver.html#method.observe) method.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver#MutationObserverInit)
#[ derive( Debug, Clone ) ]
pub struct MutationObserverInit< 'a > {
    /// If `true` it will observe all inserts and removals of the target's children (including text nodes).
    ///
    /// This is **not** recursive, it will only observe immediate children
    /// (unless [`subtree`](struct.MutationObserverInit.html#structfield.subtree) is `true` in which case it will
    /// observe **all** children and sub-children recursively).
    pub child_list: bool,

    /// If `true` it will observe all changes to the target's attributes.
    pub attributes: bool,

    /// If `true` it will observe all changes to the `CharacterData`'s data.
    pub character_data: bool,

    /// If `true` it will observe all changes to the target, the target's children, and the target's sub-children.
    ///
    /// This is recursive, so it causes **all** children and sub-children to be observed.
    pub subtree: bool,

    /// If `true` it will store the target's old attribute value in [`old_value`](enum.MutationRecord.html#variant.Attribute).
    pub attribute_old_value: bool,

    /// If `true` it will store the `CharacterData`'s old data in [`old_data`](enum.MutationRecord.html#variant.CharacterData).
    pub character_data_old_value: bool,

    /// If `Some` it will only observe the specified attributes. The attributes should be specified without a namespace.
    ///
    /// If `None` it will observe all attributes.
    pub attribute_filter: Option< &'a [ &'a str ] >,
}


impl MutationObserver {
    /// Returns a new `MutationObserver` with the given callback.
    ///
    /// The callback will be called when the observed DOM nodes change.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver#Constructor)
    pub fn new< F >( callback: F ) -> Self
        where F: FnMut( Vec< MutationRecord > ) + 'static {
        js! (
            // TODO implement second argument for callback
            return new MutationObserver( @{callback} );
        ).try_into().unwrap()
    }

    /// Starts observing the `target`.
    ///
    /// The `MutationObserver`'s callback will be called with a list of
    /// [`MutationRecord`](enum.MutationRecord.html) when the `target` changes.
    ///
    /// The `options` specifies which changes should be observed.
    ///
    /// Multiple different targets can be observed simultaneously (with the same or different `options`).
    ///
    /// If you call `observe` on the same `target` multiple times, it will replace the old `options`
    /// with the new `options`. It will **not** call the callback multiple times for the same `target`.
    ///
    /// # Panics
    ///
    /// * At least one of
    /// [`child_list`](struct.MutationObserverInit.html#structfield.child_list),
    /// [`attributes`](struct.MutationObserverInit.html#structfield.attributes), or
    /// [`character_data`](struct.MutationObserverInit.html#structfield.character_data) must be `true`.
    ///
    /// * If [`attribute_old_value`](struct.MutationObserverInit.html#structfield.attribute_old_value) is `true`, then
    /// [`attributes`](struct.MutationObserverInit.html#structfield.attributes) must be `true`.
    ///
    /// * If [`attribute_filter`](struct.MutationObserverInit.html#structfield.attribute_filter) is `Some`, then
    /// [`attributes`](struct.MutationObserverInit.html#structfield.attributes) must be `true`.
    ///
    /// * If [`character_data_old_value`](struct.MutationObserverInit.html#structfield.character_data_old_value) is `true`, then
    /// [`character_data`](struct.MutationObserverInit.html#structfield.character_data) must be `true`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver#observe())
    pub fn observe< T: INode >( &self, target: &T, options: &MutationObserverInit ) {
        let attribute_filter: Value = match options.attribute_filter {
            Some( a ) => a.into(),
            // This must compile to JavaScript `undefined`, NOT `null`
            None => Value::Undefined,
        };

        js! { @(no_return)
            @{self.as_ref()}.observe( @{target.as_ref()}, {
                childList: @{options.child_list},
                attributes: @{options.attributes},
                characterData: @{options.character_data},
                subtree: @{options.subtree},
                attributeOldValue: @{options.attribute_old_value},
                characterDataOldValue: @{options.character_data_old_value},
                attributeFilter: @{attribute_filter}
            } );
        }
    }

    /// Stops observing all targets.
    ///
    /// Until the [`observe`](struct.MutationObserver.html#method.observe) method is called again,
    /// the `MutationObserver`'s callback will not be called.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver#disconnect())
    pub fn disconnect( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.disconnect();
        }
    }

    /// Empties the `MutationObserver`'s record queue and returns what was in there.
    ///
    /// This method is generally not needed, instead use the `MutationObserver` callback to respond to changes.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver#takeRecords())
    pub fn take_records( &self ) -> Vec< MutationRecord > {
        js!(
            return @{self.as_ref()}.takeRecords();
        ).try_into().unwrap()
    }
}


/// Contains information about an individual change to the DOM.
///
/// It is passed to [`MutationObserver`](struct.MutationObserver.html)'s callback.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord)
#[ derive( Debug, Clone ) ]
pub enum MutationRecord {
    /// One of the target's attributes was changed.
    Attribute {
        /// The [`Node`](struct.Node.html) whose attribute changed.
        target: Node,

        /// The name of the changed attribute.
        name: String,

        /// The namespace of the changed attribute.
        namespace: Option< String >,

        /// The value of the changed attribute before the change.
        old_value: Option< String >,
    },

    /// The target's data was changed.
    CharacterData {
        /// The `CharacterData` node whose data changed.
        target: Node,

        /// The data of the target before the change.
        old_data: Option< String >,
    },

    /// The children of the target were changed.
    ChildList {
        /// The [`Node`](struct.Node.html) whose children changed.
        target: Node,

        /// The nodes which were inserted. Will be an empty [`NodeList`](struct.NodeList.html) if no nodes were inserted.
        inserted_nodes: NodeList,

        /// The nodes which were removed. Will be an empty [`NodeList`](struct.NodeList.html) if no nodes were removed.
        removed_nodes: NodeList,

        /// The previous sibling of the inserted or removed nodes, or `None`.
        previous_sibling: Option< Node >,

        /// The next sibling of the inserted or removed nodes, or `None`.
        next_sibling: Option< Node >,
    },
}


impl TryFrom< Value > for MutationRecord {
    type Error = ConversionError;

    fn try_from( v: Value ) -> Result< Self, Self::Error > {
        match v {
            Value::Reference( ref r ) => {
                let _type: String = js!( @{r}.type ).try_into()?;
                let target: Node = js!( @{r}.target ).try_into()?;

                match _type.as_str() {
                    "attributes" => Ok( MutationRecord::Attribute {
                        target: target,
                        name: js!( @{r}.attributeName ).try_into()?,
                        namespace: js!( @{r}.attributeNamespace ).try_into()?,
                        old_value: js!( @{r}.oldValue ).try_into()?,
                    } ),

                    "characterData" => Ok( MutationRecord::CharacterData {
                        target: target,
                        old_data: js!( @{r}.oldValue ).try_into()?,
                    } ),

                    "childList" => Ok( MutationRecord::ChildList {
                        target: target,
                        inserted_nodes: js!( @{r}.addedNodes ).try_into()?,
                        removed_nodes: js!( @{r}.removedNodes ).try_into()?,
                        previous_sibling: js!( @{r}.previousSibling ).try_into()?,
                        next_sibling: js!( @{r}.nextSibling ).try_into()?,
                    } ),

                    other => Err( ConversionError::Custom( format!( "Unknown MutationRecord type: {:?}", other ) ) ),
                }
            },
            other => Err( ConversionError::Custom( format!( "Expected MutationRecord but got: {:?}", other ) ) ),
        }
    }
}


#[ cfg( all( test, feature = "web_test" ) ) ]
mod tests {
    use super::*;
    use webapi::document::document;

    #[ test ]
    fn test_observe() {
        let observer = MutationObserver::new( |_| {} );

        // TODO replace with document.body
        observer.observe( &document(),  &MutationObserverInit {
            child_list: true,
            attributes: false,
            character_data: true,
            subtree: true,
            attribute_old_value: false,
            character_data_old_value: false,
            attribute_filter: Some( &[ "foo", "bar", "qux" ] ),
        } );
    }
}
