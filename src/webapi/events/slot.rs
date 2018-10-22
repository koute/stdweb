use webcore::value::Reference;
use webapi::event::{IEvent, Event};

/// The `slotchange` event is fired on an HTMLSlotElement instance
/// (`<slot>` element) when the node(s) contained in that slot change.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/slotchange)
// https://dom.spec.whatwg.org/#mutation-observers
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(event = "slotchange")]
#[reference(subclass_of(Event))]
pub struct SlotChangeEvent( Reference );

impl IEvent for SlotChangeEvent {}
