use std::fmt;

use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::reference_type::ReferenceType;
use webcore::mutfn::Mut;
use webapi::event::{ConcreteEvent, IEvent};
use private::TODO;

/// A handle to a particular event listener.
pub struct EventListenerHandle {
    event_type: &'static str,
    reference: Reference,
    listener_reference: Reference
}

impl fmt::Debug for EventListenerHandle {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "EventListenerHandle {{ event_type: {}, reference: {:?} }}", self.event_type, self.reference )
    }
}

impl EventListenerHandle {
    /// Removes the listener from the [IEventTarget](trait.IEventTarget.html) on
    /// which it was previously registered.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)
    // https://dom.spec.whatwg.org/#ref-for-dom-eventtarget-removeeventlistener%E2%91%A0
    pub fn remove( self ) {
        js! { @(no_return)
            var listener = @{&self.listener_reference};
            @{&self.reference}.removeEventListener( @{self.event_type}, listener );
            listener.drop();
        }
    }
}

/// `IEventTarget` is an interface implemented by objects that
/// can receive events and may have listeners for them.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)
// https://dom.spec.whatwg.org/#eventtarget
pub trait IEventTarget: ReferenceType {
    /// Adds given event handler to the list of event listeners for
    /// the specified `EventTarget` on which it's called.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)
    // https://dom.spec.whatwg.org/#ref-for-dom-eventtarget-addeventlistener%E2%91%A0
    fn add_event_listener< T, F >( &self, listener: F ) -> EventListenerHandle
        where T: ConcreteEvent, F: FnMut( T ) + 'static
    {
        let reference = self.as_ref();

        let listener_reference = js! {
            var listener = @{Mut(listener)};
            @{reference}.addEventListener( @{T::EVENT_TYPE}, listener );
            return listener;
        }.try_into().unwrap();

        EventListenerHandle {
            event_type: T::EVENT_TYPE,
            reference: reference.clone(),
            listener_reference: listener_reference
        }
    }

    /// Dispatches an `Event` at this `EventTarget`, invoking the affected event listeners in the
    /// appropriate order.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent)
    fn dispatch_event< T: IEvent >( &self, event: &T ) -> Result< bool, TODO > {
        Ok( js! (
            return @{self.as_ref()}.dispatchEvent( @{event.as_ref()} );
        ).try_into().unwrap() )
    }
}

/// A reference to a JavaScript object which implements the [IEventTarget](trait.IEventTarget.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)
// https://dom.spec.whatwg.org/#eventtarget
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "EventTarget")]
pub struct EventTarget( Reference );

impl IEventTarget for EventTarget {}
