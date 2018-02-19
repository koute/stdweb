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
#![cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    feature(proc_macro)
)]
#![cfg_attr(feature = "nightly", feature(core_intrinsics))]
#![cfg_attr(feature = "nightly", feature(never_type))]
#![recursion_limit="1500"]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde as serde_crate;

#[cfg(any(test, feature = "serde_json"))]
extern crate serde_json;

#[cfg(all(test, feature = "serde"))]
#[macro_use]
extern crate serde_derive;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
extern crate stdweb_internal_macros;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use stdweb_internal_macros::js_export;

#[cfg(feature = "futures")]
extern crate futures;

#[macro_use]
extern crate stdweb_derive;

#[macro_use]
mod webcore;
mod webapi;
mod ecosystem;

// This is here so that our procedural macros
// can work within the crate.
pub(crate) mod stdweb {
    pub use super::*;
}

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
pub use webcore::symbol::Symbol;

pub use webcore::unsafe_typed_array::UnsafeTypedArray;
pub use webcore::once::Once;
pub use webcore::instance_of::InstanceOf;
pub use webcore::reference_type::ReferenceType;
pub use webcore::serialization::JsSerialize;

pub use webcore::promise::Promise;

#[cfg(feature = "futures")]
pub use webcore::promise_future::PromiseFuture;

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
    pub use webapi::parent_node::IParentNode;
    pub use webapi::non_element_parent_node::INonElementParentNode;
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
    pub use webapi::rendering_context::{RenderingContext, CanvasRenderingContext2d, CanvasGradient, CanvasPattern, ImageData, TextMetrics};
    pub use webapi::mutation_observer::{MutationObserver, MutationObserverHandle, MutationObserverInit, MutationRecord};
    pub use webapi::xml_http_request::{XmlHttpRequest, XhrReadyState};
    pub use webapi::blob::{IBlob, Blob};

    /// A module containing error types.
    pub mod error {
        pub use webapi::dom_exception::{
            IDomException,
            DomException,
            HierarchyRequestError,
            IndexSizeError,
            InvalidAccessError,
            NotFoundError,
            NotSupportedError,
            SecurityError,
            SyntaxError,
            TypeError
        };
        pub use webapi::error::{IError, Error};
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
            IEvent,
            IUiEvent,
            ConcreteEvent,

            EventPhase
        };

        pub use webapi::events::mouse::{
            IMouseEvent,
            ClickEvent,
            DoubleClickEvent,
            MouseDownEvent,
            MouseUpEvent,
            MouseMoveEvent,

            MouseButton
        };

        pub use webapi::events::keyboard::{
            IKeyboardEvent,
            KeyPressEvent,
            KeyDownEvent,
            KeyUpEvent,

            KeyboardLocation,
            ModifierKey
        };

        pub use webapi::events::progress::{
            IProgressEvent,
            ProgressEvent,
            LoadStartEvent,
            LoadEndEvent,
            ProgressLoadEvent,
            ProgressAbortEvent,
            ProgressErrorEvent
        };

        pub use webapi::events::socket::{
            IMessageEvent,
            SocketCloseEvent,
            SocketErrorEvent,
            SocketOpenEvent,
            SocketMessageEvent
        };

        pub use webapi::events::history::{
            HashChangeEvent,
            PopStateEvent
        };

        pub use webapi::events::dom::{
            ChangeEvent,
            ResourceLoadEvent,
            ResourceAbortEvent,
            ResourceErrorEvent,
            ResizeEvent,
            InputEvent,
            ReadyStateChangeEvent
        };

        pub use webapi::events::focus::{
            IFocusEvent,
            FocusEvent,
            BlurEvent
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

/// A module containing reexports of all of our interface traits.
///
/// You should **only** import its contents through a wildcard, e.g.: `use stdweb::traits::*`.
pub mod traits {
    pub use super::web::{
        // Real interfaces.
        IEventTarget,
        INode,
        IElement,
        IHtmlElement,
        IBlob,

        // Mixins.
        IWindowOrWorker,
        IParentNode,
        INonElementParentNode
    };

    pub use super::web::error::{
        IDomException,
        IError
    };

    pub use super::web::event::{
        IEvent,
        IUiEvent,
        IMouseEvent,
        IKeyboardEvent,
        IProgressEvent,
        IMessageEvent,
        IFocusEvent
    };
}

#[doc(hidden)]
pub mod private {
    pub use webcore::ffi::exports::*;
    pub use webcore::serialization::{
        JsSerialize,
        JsSerializeOwned,
        PreallocatedArena,
        SerializedValue
    };

    pub use webcore::newtype::{
        IntoNewtype,
        Newtype
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

    // TODO: Remove this.
    #[derive(Debug)]
    pub struct TODO;

    impl ::std::fmt::Display for TODO {
        fn fmt( &self, _: &mut ::std::fmt::Formatter ) -> Result< (), ::std::fmt::Error > {
            unreachable!();
        }
    }

    impl ::std::error::Error for TODO {
        fn description( &self ) -> &str {
            unreachable!();
        }
    }

    pub use webcore::value::ConversionError;
}
