//! The goal of this crate is to provide Rust bindings to the Web APIs and to allow
//! a high degree of interoperability between Rust and JavaScript.
//!
//! ## Examples
//!
//! You can directly embed JavaScript code into Rust:
//!
//! ```rust
//! let message = "Hello, 世界!";
//! let result = js! {
//!     alert( @{message} );
//!     return 2 + 2 * 2;
//! };
//!
//! println!( "2 + 2 * 2 = {:?}", result );
//! ```
//!
//! Even closures are supported:
//!
//! ```rust
//! let print_hello = |name: String| {
//!     println!( "Hello, {}!", name );
//! };
//!
//! js! {
//!     var print_hello = @{print_hello};
//!     print_hello( "Bob" );
//!     print_hello.drop(); // Necessary to clean up the closure on Rust's side.
//! }
//! ```
//!
//! You can also pass arbitrary structures thanks to [serde]:
//!
//! ```rust
//! #[derive(Serialize)]
//! struct Person {
//!     name: String,
//!     age: i32
//! }
//!
//! js_serializable!( Person );
//!
//! js! {
//!     var person = @{person};
//!     console.log( person.name + " is " + person.age + " years old." );
//! };
//! ```
//!
//! [serde]: https://serde.rs/
//!
//! This crate also exposes a number of Web APIs, for example:
//!
//! ```rust
//! let button = document().query_selector( "#hide-button" ).unwrap();
//! button.add_event_listener( move |_: ClickEvent| {
//!     for anchor in document().query_selector_all( "#main a" ) {
//!         js!( @{anchor}.style = "display: none;"; );
//!     }
//! });
//! ```

#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(not(feature = "nightly"), deny(unstable_features))]
#![cfg_attr(feature = "nightly", feature(core_intrinsics))]
#![recursion_limit="1500"]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde as serde_crate;

#[cfg(any(test, feature = "serde_json"))]
extern crate serde_json;

#[cfg(all(test, feature = "serde"))]
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod webcore;
mod webapi;
mod ecosystem;

pub use webcore::initialization::{
    initialize,
    event_loop
};
pub use webcore::value::{
    Undefined,
    Null,
    Value,
    Reference
};
pub use webcore::number::Number;
pub use webcore::object::Object;
pub use webcore::array::Array;

pub use webcore::unsafe_typed_array::UnsafeTypedArray;
pub use webcore::once::Once;

#[cfg(feature = "serde")]
/// A module with serde-related APIs.
pub mod serde {
    pub use ecosystem::serde::{
        ConversionError,
        Serde
    };
}

/// A module with bindings to the Web APIs.
pub mod web {
    pub use webapi::window::{
        Window,
        window
    };
    pub use webapi::document::{
        Document,
        document
    };
    pub use webapi::global::{
        set_timeout,
        alert
    };
    pub use webapi::cross_origin_setting::CrossOriginSetting;
    pub use webapi::date::Date;
    pub use webapi::event_target::{IEventTarget, EventTarget, EventListenerHandle};
    pub use webapi::window::RequestAnimationFrameHandle;
    pub use webapi::node::{INode, Node, CloneKind};
    pub use webapi::element::{IElement, Element};
    pub use webapi::text_node::TextNode;
    pub use webapi::html_element::{IHtmlElement, HtmlElement};
    pub use webapi::window_or_worker::IWindowOrWorker;
    pub use webapi::token_list::TokenList;
    pub use webapi::node_list::NodeList;
    pub use webapi::string_map::StringMap;
    pub use webapi::storage::Storage;
    pub use webapi::location::Location;
    pub use webapi::array_buffer::ArrayBuffer;
    pub use webapi::typed_array::TypedArray;
    pub use webapi::file_reader::{FileReader, FileReaderResult};
    pub use webapi::history::History;
    pub use webapi::web_socket::{WebSocket, SocketCloseCode};
    pub use webapi::rendering_context::{RenderingContext, CanvasRenderingContext2d};
    pub use webapi::mutation_observer::{MutationObserver, MutationObserverHandle, MutationObserverInit, MutationRecord};


    /// A module containing XmlHttpRequest and its XhrReadyState
    pub mod xml_http_request {
        pub use webapi::xml_http_request::{ XmlHttpRequest, XhrReadyState };
    }
    /// A module containing error types.
    pub mod error {
        pub use webapi::node::NotFoundError;
    }

    /// A module containing HTML DOM elements.
    pub mod html_element {
        pub use webapi::html_elements::ImageElement;
        pub use webapi::html_elements::InputElement;
        pub use webapi::html_elements::TextAreaElement;
        pub use webapi::html_elements::CanvasElement;
    }

    /// A module containing JavaScript DOM events.
    pub mod event {
        pub use webapi::event::{
            ConcreteEvent,

            IEvent,
            IKeyboardEvent,
            IUiEvent,
            IMouseEvent,
            IFocusEvent,
            IProgressEvent,
            IMessageEvent,

            EventPhase,
            KeyboardLocation,
            ModifierKey,
            MouseButton,

            ChangeEvent,
            KeypressEvent,
            KeydownEvent,
            KeyupEvent,
            ClickEvent,
            DoubleClickEvent,
            MouseDownEvent,
            MouseUpEvent,
            MouseMoveEvent,
            FocusEvent,
            BlurEvent,
            HashChangeEvent,
            ResourceLoadEvent,
            ResourceAbortEvent,
            ResourceErrorEvent,
            ResizeEvent,
            ProgressEvent,
            LoadStartEvent,
            LoadEndEvent,
            ProgressLoadEvent,
            ProgressAbortEvent,
            ProgressErrorEvent,
            InputEvent,
            ReadyStateChange,
            PopStateEvent,

            SocketCloseEvent,
            SocketErrorEvent,
            SocketOpenEvent,
            SocketMessageEvent,
        };
    }
}

/// A module containing stable counterparts to currently
/// unstable Rust features.
pub mod unstable {
    pub use webcore::try_from::{
        TryFrom,
        TryInto
    };

    pub use webcore::void::Void;
}

#[doc(hidden)]
pub mod private {
    pub use webcore::ffi::exports::*;
    pub use webcore::serialization::{
        JsSerializable,
        JsSerializableOwned,
        PreallocatedArena,
        SerializedValue
    };

    pub use webcore::newtype::{
        IntoNewtype,
        Newtype
    };

    pub use webcore::value::{
        FromReference,
        FromReferenceUnchecked
    };

    #[cfg(feature = "serde")]
    pub use ecosystem::serde::{
        to_value,
        from_value
    };

    // This is to prevent an unused_mut warnings in macros, because an `allow` doesn't work apparently?
    #[allow(dead_code)]
    #[inline(always)]
    pub fn noop< T >( _: &mut T ) {}
}
