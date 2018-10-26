use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::once::Once;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};
use webapi::blob::Blob;
use webapi::rendering_context::RenderingContext;
use private::TODO;

/// The HTML `<canvas>` element provides an empty graphic zone on which specific JavaScript APIs
/// can draw (such as Canvas 2D or WebGL).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement)
// https://html.spec.whatwg.org/#htmlcanvaselement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLCanvasElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct CanvasElement( Reference );

impl IEventTarget for CanvasElement {}
impl INode for CanvasElement {}
impl IElement for CanvasElement {}
impl IHtmlElement for CanvasElement {}

impl CanvasElement {
    /// Returns a positive integer reflecting the height HTML attribute of the <canvas> element
    /// interpreted in CSS pixels. When the attribute is not specified, or if it is set to an
    /// invalid value, like a negative, the default value of 150 is used.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)
    // https://html.spec.whatwg.org/#the-canvas-element:dom-canvas-height
    pub fn height( &self ) -> u32 {
        js! (
            return @{self}.height;
        ).try_into().unwrap()
    }

    /// Sets a positive integer reflecting the height HTML attribute of the <canvas> element
    /// interpreted in CSS pixels. When the attribute is not specified, or if it is set to an
    /// invalid value, like a negative, the default value of 150 is used.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)
    // https://html.spec.whatwg.org/#the-canvas-element:dom-canvas-height
    pub fn set_height( &self, value: u32 ) {
        js! { @(no_return)
            @{self}.height = @{value};
        }
    }

    /// Returns a positive integer reflecting the width HTML attribute of the <canvas> element
    /// interpreted in CSS pixels. When the attribute is not specified, or if it is set to an
    /// invalid value, like a negative, the default value of 300 is used.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)
    // https://html.spec.whatwg.org/#the-canvas-element:dom-canvas-width
    pub fn width( &self ) -> u32 {
        js! (
            return @{self}.width;
        ).try_into().unwrap()
    }

    /// Sets a positive integer reflecting the width HTML attribute of the <canvas> element
    /// interpreted in CSS pixels. When the attribute is not specified, or if it is set to an
    /// invalid value, like a negative, the default value of 300 is used.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)
    // https://html.spec.whatwg.org/#the-canvas-element:dom-canvas-width
    pub fn set_width( &self, value: u32 ) {
        js! { @(no_return)
            @{self}.width = @{value};
        }
    }

    /// Returns a drawing context on the canvas, or None if the context identifier is not supported.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)
    // https://html.spec.whatwg.org/#the-canvas-element:dom-canvas-getcontext
    pub fn get_context<T: RenderingContext>( &self ) -> Result<T, T::Error> {
        T::from_canvas(self)
    }

    /// Returns a data URI containing a representation of the image in the format specified by the
    /// type parameter (defaults to PNG). The returned image is in a resolution of 96 dpi.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataUrl)
    // https://html.spec.whatwg.org/#the-canvas-element:dom-canvas-todataurl
    pub fn to_data_url( &self, mime_type: Option<&str>, quality: Option<f64> ) -> Result< String, TODO > {
        Ok( js! (
            return @{self}.toDataURL(@{mime_type}, @{quality});
        ).try_into().unwrap() )
    }

    /// Creates a Blob object representing the image contained in the canvas; this file may be
    /// cached on the disk or stored in memory at the discretion of the user agent.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)
    // https://html.spec.whatwg.org/#the-canvas-element:dom-canvas-toblob
    pub fn to_blob<F: FnOnce(Blob) + 'static>( &self, f: F, mime_type: Option<&str>, quality: Option<f64> ) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.toBlob(@{Once(f)}, @{mime_type}, @{quality});
        }

        Ok(())
    }
}
