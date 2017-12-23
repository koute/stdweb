#![cfg(feature = "serde")]
use serde_crate::Serialize;
use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::serialization::JsSerializable;

/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
pub struct History(Reference);

reference_boilerplate! {
    History,
    instanceof History
}

impl History {
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
    pub fn push_state<T: JsSerializable>(&self, state: T, title: Option<&str>, url: String) {
        js!{
            @{self}.pushState(@{state}, @{title}, @{url});
        };
    }

    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
    pub fn replace_state<T: JsSerializable>(&self, state: T, title: Option<&str>, url: String) {
        js!{
            @{self}.replaceState(@{state}, @{title}, @{url});
        };
    }

    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
    pub fn go(&self, offset: i32) {
        js!(@{self}.go(@{offset}));
    }

    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
    pub fn back(&self) {
        js!(@{self}.back());
    }

    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
    pub fn forward(&self) {
        js!(@{self}.forward());
    }

    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
    pub fn length(&self) -> i32 {
        js!(@{self}.length).try_into().unwrap()
    }
}
