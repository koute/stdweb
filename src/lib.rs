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
//! Closures are also supported:
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
//! let button = document().query_selector( "#hide-button" ).unwrap().unwrap();
//! button.add_event_listener( move |_: ClickEvent| {
//!     for anchor in document().query_selector_all( "#main a" ) {
//!         js!( @{anchor}.style = "display: none;"; );
//!     }
//! });
//! ```
//!
//! Exposing Rust functions to JavaScript is supported too:
//!
//! ```rust
//! #[js_export]
//! fn hash( string: String ) -> String {
//!     let mut hasher = Sha1::new();
//!     hasher.update( string.as_bytes() );
//!     hasher.digest().to_string()
//! }
//! ```
//!
//! Then you can do this from Node.js:
//!
//! ```js
//! var hasher = require( "hasher.js" ); // Where `hasher.js` is generated from Rust code.
//! console.log( hasher.hash( "Hello world!" ) );
//! ```
//!
//! Or you can take the same `.js` file and use it in a web browser:
//!
//! ```html
//! <script src="hasher.js"></script>
//! <script>
//!     Rust.hasher.then( function( hasher ) {
//!         console.log( hasher.hash( "Hello world!" ) );
//!     });
//! </script>
//! ```
//!
//! If you're using [Parcel] you can also use our [experimental Parcel plugin];
//! first do this in your existing Parcel project:
//!
//!     $ npm install --save parcel-plugin-cargo-web
//!
//! And then simply:
//!
//! ```js
//! import hasher from "./hasher/Cargo.toml";
//! console.log( hasher.hash( "Hello world!" ) );
//! ```
//!
//! [Parcel]: https://parceljs.org/
//! [experimental Parcel plugin]: https://github.com/koute/parcel-plugin-cargo-web

#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_numeric_casts,
    unused_import_braces
)]
#![cfg_attr(
    all(test, rust_nightly),
    feature(linkage) // Needed for async tests.
)]
#![cfg_attr(rust_nightly, feature(core_intrinsics))]
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

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))]
extern crate wasm_bindgen;

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web), test))]
#[macro_use]
extern crate wasm_bindgen_test;

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web), test))]
#[macro_use]
extern crate stdweb_internal_test_macro;

extern crate stdweb_internal_macros;

#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown"
))]
pub use stdweb_internal_macros::js_export;

pub use stdweb_internal_macros::async_test;

#[cfg(feature = "futures-support")]
extern crate futures_core;

#[cfg(feature = "futures-support")]
extern crate futures_util;

#[cfg(feature = "futures-support")]
extern crate futures_channel;

#[cfg(feature = "futures-support")]
extern crate futures_executor;

#[macro_use]
extern crate stdweb_derive;
#[macro_use]
extern crate stdweb_internal_runtime;

extern crate discard;

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web), test))]
wasm_bindgen_test_configure!( run_in_browser );

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
pub use webcore::mutfn::Mut;
pub use webcore::once::Once;
pub use webcore::instance_of::InstanceOf;
pub use webcore::reference_type::ReferenceType;
pub use webcore::serialization::JsSerialize;

pub use webcore::discard::DiscardOnDrop;

#[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
pub use webcore::promise::{TypedPromise, Promise, DoneHandle};

#[cfg(all(
    feature = "futures-support",
    feature = "experimental_features_which_may_break_on_minor_version_bumps"
))]
pub use webcore::promise_future::{PromiseFuture, spawn_local, print_error_panic, unwrap_future};

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
    #[cfg(feature = "futures-support")]
    pub use webapi::timer_future::{
        Wait,
        wait,
        IntervalBuffered,
        interval_buffered
    };

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
        alert,
        confirm
    };
    pub use webapi::cross_origin_setting::CrossOriginSetting;
    pub use webapi::date::Date;
    pub use webapi::event_target::{IEventTarget, EventTarget, EventListenerHandle};
    pub use webapi::window::RequestAnimationFrameHandle;
    pub use webapi::node::{INode, Node, CloneKind, NodeType};
    pub use webapi::element::{IElement, Element};
    pub use webapi::document_fragment::DocumentFragment;
    pub use webapi::text_node::TextNode;
    pub use webapi::html_element::{IHtmlElement, HtmlElement, Rect};
    pub use webapi::window_or_worker::IWindowOrWorker;
    pub use webapi::parent_node::IParentNode;
    pub use webapi::slotable::ISlotable;
    pub use webapi::non_element_parent_node::INonElementParentNode;
    pub use webapi::token_list::TokenList;
    pub use webapi::node_list::NodeList;
    pub use webapi::string_map::StringMap;
    pub use webapi::storage::Storage;
    pub use webapi::location::Location;
    pub use webapi::array_buffer::ArrayBuffer;
    pub use webapi::typed_array::TypedArray;
    pub use webapi::file::File;
    pub use webapi::file_reader::{FileReader, FileReaderResult, FileReaderReadyState};
    pub use webapi::file_list::FileList;
    pub use webapi::history::History;
    pub use webapi::web_socket::{WebSocket, SocketCloseCode, SocketBinaryType, SocketReadyState};
    pub use webapi::rendering_context::{RenderingContext, CanvasRenderingContext2d, CanvasGradient, CanvasPattern, CanvasStyle, CompositeOperation, FillRule, ImageData, LineCap, LineJoin, Repetition, TextAlign, TextBaseline, TextMetrics};
    pub use webapi::mutation_observer::{MutationObserver, MutationObserverHandle, MutationObserverInit, MutationRecord};
    pub use webapi::xml_http_request::{XmlHttpRequest, XhrReadyState, XhrResponseType};
    pub use webapi::blob::{IBlob, Blob};
    pub use webapi::html_collection::HtmlCollection;
    pub use webapi::child_node::IChildNode;
    pub use webapi::gamepad::{Gamepad, GamepadButton, GamepadMappingType};
    pub use webapi::touch::{Touch, TouchType};
    pub use webapi::selection::Selection;
    pub use webapi::shadow_root::{ShadowRootMode, ShadowRoot};
    pub use webapi::html_elements::SlotContentKind;
    pub use webapi::form_data::{FormData, FormDataEntry};
    pub use webapi::window_or_worker::TimeoutHandle;

    /// A module containing error types.
    pub mod error {
        pub use webapi::dom_exception::{
            IDomException,
            DomException,
            HierarchyRequestError,
            IndexSizeError,
            InvalidAccessError,
            InvalidStateError,
            NotFoundError,
            NotSupportedError,
            SecurityError,
            SyntaxError,
            InvalidCharacterError,
            AbortError
        };

        pub use webapi::error::{
            IError,
            Error,
            TypeError
        };

        pub use webapi::rendering_context::{AddColorStopError, DrawImageError, GetImageDataError};
        pub use webapi::html_elements::UnknownValueError;
        pub use webapi::xml_http_request::XhrSetResponseTypeError;
    }

    /// A module containing HTML DOM elements.
    pub mod html_element {
        pub use webapi::html_elements::ImageElement;
        pub use webapi::html_elements::InputElement;
        pub use webapi::html_elements::TextAreaElement;
        pub use webapi::html_elements::CanvasElement;
        pub use webapi::html_elements::SelectElement;
        pub use webapi::html_elements::OptionElement;
        pub use webapi::html_elements::TemplateElement;
        pub use webapi::html_elements::SlotElement;
    }

    /// A module containing JavaScript DOM events.
    pub mod event {
        pub use webapi::event::{
            IEvent,
            IUiEvent,
            ConcreteEvent,

            UnloadEvent,
            BeforeUnloadEvent,

            FullscreenChangeEvent,

            EventPhase
        };

        pub use webapi::events::mouse::{
            IMouseEvent,
            ClickEvent,
            AuxClickEvent,
            ContextMenuEvent,
            DoubleClickEvent,
            MouseDownEvent,
            MouseUpEvent,
            MouseMoveEvent,
            MouseOverEvent,
            MouseOutEvent,
            MouseEnterEvent,
            MouseLeaveEvent,
            MouseWheelEvent,
            MouseWheelDeltaMode,
            MouseButton,
            MouseButtonsState
        };

        pub use webapi::events::touch::{
            ITouchEvent,
            TouchEvent,
            TouchMove,
            TouchLeave,
            TouchEnter,
            TouchEnd,
            TouchCancel,
            TouchStart,
        };

        pub use webapi::events::pointer::{
            IPointerEvent,
            PointerOverEvent,
            PointerEnterEvent,
            PointerDownEvent,
            PointerMoveEvent,
            PointerUpEvent,
            PointerCancelEvent,
            PointerOutEvent,
            PointerLeaveEvent,
            GotPointerCaptureEvent,
            LostPointerCaptureEvent,
            PointerLockChangeEvent,
            PointerLockErrorEvent
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
            SocketMessageEvent,
            SocketMessageData
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
            ScrollEvent,
            InputEvent,
            ReadyStateChangeEvent,
            SubmitEvent,
            SelectionChangeEvent
        };

        pub use webapi::events::focus::{
            IFocusEvent,
            FocusEvent,
            BlurEvent
        };

        pub use webapi::events::gamepad::{
            IGamepadEvent,
            GamepadConnectedEvent,
            GamepadDisconnectedEvent,
        };

        pub use webapi::events::drag::{
            IDragEvent,
            DragRelatedEvent,
            DragEvent,
            DragStartEvent,
            DragEndEvent,
            DragEnterEvent,
            DragLeaveEvent,
            DragOverEvent,
            DragExitEvent,
            DragDropEvent,
            DataTransfer,
            EffectAllowed,
            DropEffect,
            DataTransferItemList,
            DataTransferItem,
            DataTransferItemKind,
        };

        pub use webapi::events::slot::SlotChangeEvent;
    }

    #[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
    /// APIs related to MIDI.
    pub mod midi {
        pub use webapi::midi::{
            MidiOptions,
            MidiAccess,
            MidiPort,
            MidiInput,
            MidiOutput,
            IMidiPort
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
    #[doc(hidden)]
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
        INonElementParentNode,
        IChildNode,
        ISlotable,
    };

    #[doc(hidden)]
    pub use super::web::error::{
        IDomException,
        IError
    };

    #[doc(hidden)]
    pub use super::web::event::{
        IEvent,
        IUiEvent,
        IMouseEvent,
        IPointerEvent,
        IKeyboardEvent,
        IProgressEvent,
        IMessageEvent,
        IFocusEvent,
        IDragEvent,
        ITouchEvent,
    };

    #[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
    #[doc(hidden)]
    pub use super::web::midi::IMidiPort;
}

#[doc(hidden)]
pub mod private {
    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))]
    pub extern crate wasm_bindgen;

    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))]
    pub use webcore::ffi::get_module;

    pub use webcore::ffi::exports::*;
    pub use webcore::serialization::{
        JsSerialize,
        JsSerializeOwned,
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

    pub use webcore::global_arena::ArenaRestorePoint;
    pub use webcore::global_arena::serialize_value;

    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", cargo_web))]
    pub use stdweb_internal_macros::wasm32_unknown_unknown_js_attr as js_attr;
    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", cargo_web))]
    pub use stdweb_internal_macros::wasm32_unknown_unknown_js_no_return_attr as js_no_return_attr;
    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", cargo_web))]
    pub use stdweb_internal_macros::wasm32_unknown_unknown_js_raw_attr as js_raw_attr;

    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))]
    pub use stdweb_internal_macros::wasm_bindgen_js_attr as js_attr;
    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))]
    pub use stdweb_internal_macros::wasm_bindgen_js_no_return_attr as js_no_return_attr;
    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))]
    pub use stdweb_internal_macros::wasm_bindgen_js_raw_attr as js_raw_attr;

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub use stdweb_internal_macros::emscripten_js_attr as js_attr;
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub use stdweb_internal_macros::emscripten_js_no_return_attr as js_no_return_attr;
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub use stdweb_internal_macros::emscripten_js_raw_attr as js_raw_attr;

    // This is to prevent an unused_mut warnings in macros, because an `allow` doesn't work apparently?
    #[allow(dead_code)]
    #[inline(always)]
    pub fn noop< T >( _: &mut T ) {}

    // TODO: Remove this.
    #[derive(Debug)]
    pub struct TODO;

    impl std::fmt::Display for TODO {
        fn fmt( &self, _: &mut std::fmt::Formatter ) -> Result< (), std::fmt::Error > {
            unreachable!();
        }
    }

    impl std::error::Error for TODO {
        fn description( &self ) -> &str {
            unreachable!();
        }
    }

    pub use webcore::value::ConversionError;
}
