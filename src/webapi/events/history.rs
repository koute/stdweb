use webcore::value::{Reference, Value};
use webcore::try_from::TryInto;
use webapi::event::{IEvent, Event, ConcreteEvent};

/// The `HashChangeEvent` is fired when the fragment
/// identifier of the URL has changed (the part of the URL
/// that follows the # symbol, including the # symbol).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/hashchange)
pub struct HashChangeEvent( Reference );

impl IEvent for HashChangeEvent {}
impl ConcreteEvent for HashChangeEvent {
    const EVENT_TYPE: &'static str = "hashchange";
}

reference_boilerplate! {
    HashChangeEvent,
    instanceof HashChangeEvent
    convertible to Event
}

impl HashChangeEvent {
    /// The previous URL from which the window was navigated.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent)
    #[inline]
    pub fn old_url( &self ) -> String {
        js!(
            return @{self.as_ref()}.oldURL;
        ).try_into().unwrap()
    }

    /// The new URL to which the window was navigated.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent)
    #[inline]
    pub fn new_url( &self ) -> String {
        js!(
            return @{self.as_ref()}.newURL;
        ).try_into().unwrap()
    }
}

/// A popstate event is dispatched to the window every time the active history entry changes
/// between two history entries for the same document. If the history entry being activated was
/// created by a call to history.push_state() or was affected by a call to history.replace_state(),
/// the popstate event's state property contains a copy of the history entry's state object.
///
/// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent)
pub struct PopStateEvent(Reference);

reference_boilerplate! {
    PopStateEvent,
    instanceof Event
    convertible to Event
}

impl PopStateEvent {
    /// The state object associated to the new history entry, if that entry was created with
    /// push_state or affected by replace_state.
    ///
    /// Example usage:
    ///
    /// ```rust,ignore
    /// let state: Option<MyStruct> = event.state().try_into().ok();
    /// ```
    #[inline]
    pub fn state(&self) -> Value {
        js!(return @{self}.state;)
    }
}

impl IEvent for PopStateEvent {}

impl ConcreteEvent for PopStateEvent {
    const EVENT_TYPE: &'static str = "popstate";
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_hash_change_event() {
        let event: HashChangeEvent = js!(
            return new HashChangeEvent(
                @{HashChangeEvent::EVENT_TYPE},
                {
                    oldURL: "http://test.com#foo",
                    newURL: "http://test.com#bar"
                }
            );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), HashChangeEvent::EVENT_TYPE );
        assert_eq!( event.old_url(), "http://test.com#foo" );
        assert_eq!( event.new_url(), "http://test.com#bar" );
    }

    #[test]
    fn test_pop_state_event() {
        let event: PopStateEvent = js!(
            return new PopStateEvent(
                @{PopStateEvent::EVENT_TYPE},
                {
                    state: {
                        color: "tomato"
                    }
                }
            );
        ).try_into().unwrap();

        assert_eq!(event.event_type(), PopStateEvent::EVENT_TYPE);

        let state_value: Value = event.state();
        let state: ::std::collections::BTreeMap<String, Value> = state_value
            .as_object()
            .unwrap()
            .into();
        let mut expected = ::std::collections::BTreeMap::new();
        expected.insert("color".to_string(), "tomato".into());

        assert_eq!(state, expected);
    }
}
