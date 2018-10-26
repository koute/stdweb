use webcore::value::{Value, Reference};
use webcore::try_from::TryInto;
use webapi::cross_origin_setting::CrossOriginSetting;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};

/// The HTML image element is used to manipulate the layout and presentation of
/// `<img>` elements.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement)
// https://html.spec.whatwg.org/#htmlimageelement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLImageElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct ImageElement( Reference );

impl IEventTarget for ImageElement {}
impl INode for ImageElement {}
impl IElement for ImageElement {}
impl IHtmlElement for ImageElement {}

impl ImageElement {
    /// Constructs a new ImageElement.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/Image)
    // https://html.spec.whatwg.org/#the-img-element:dom-image
    pub fn new() -> ImageElement {
        js! (
            return new Image();
        ).try_into().unwrap()
    }

    /// Constructs a new ImageElement with the given width and height.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/Image)
    // https://html.spec.whatwg.org/#the-img-element:dom-image
    pub fn with_size(width: u32, height: u32) -> ImageElement {
        js! (
            return new Image(@{width}, @{height});
        ).try_into().unwrap()
    }

    /// Returns the HTML `alt` attribute, representing the fallback context for the image.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-alt
    #[inline]
    pub fn alt( &self ) -> String {
        js! (
            return @{self}.alt;
        ).try_into().unwrap()
    }

    /// Sets the HTML `alt` attribute, representing the fallback context for the image.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-alt
    pub fn set_alt( &self, value: &str ) {
        js! { @(no_return)
            @{self}.alt = @{value};
        }
    }

    /// Returns true if the browser has finished fetching the image, whether
    /// successful or not. It also return true if the image has no src value.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/complete)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-complete
    pub fn complete( &self ) -> bool {
        js! (
            return @{self}.complete;
        ).try_into().unwrap()
    }

    /// Returns the Cross-Origin Resource Sharing (CORS) setting for the image.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-crossorigin
    pub fn cross_origin( &self ) -> CrossOriginSetting {
        match js!(
            return @{self}.crossOrigin;
        ) {
            Value::Null => CrossOriginSetting::None,
            Value::String( ref s ) if *s == "anonymous" => CrossOriginSetting::Anonymous,
            Value::String( ref s ) if *s == "use-credentials" => CrossOriginSetting::UseCredentials,
            _ => unreachable!("Unexpected crossOrigin value")
        }
    }

    /// Sets the Cross-Origin Resource Sharing (CORS) setting for the image.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-crossorigin
    pub fn set_cross_origin( &self, value: CrossOriginSetting ) {
        js! { @(no_return)
            @{self}.crossOrigin = @{
                match value {
                    CrossOriginSetting::None => None,
                    CrossOriginSetting::Anonymous => Some("anonymous"),
                    CrossOriginSetting::UseCredentials => Some("use-credentials")
                }
            }
        }
    }

    /// Returns the the rendered height of the image in CSS pixels.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-height
    pub fn height( &self ) -> u32 {
        js! (
            return @{self}.height;
        ).try_into().unwrap()
    }

    /// Sets the the rendered height of the image in CSS pixels.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-height
    pub fn set_height( &self, value: u32 ) {
        js! { @(no_return)
            @{self}.height = @{value};
        }
    }

    /// Indicates whether the image is part of a server-side image map.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-ismap
    pub fn is_map( &self ) -> bool {
        js!(
            return @{self}.isMap;
        ).try_into().unwrap()
    }

    /// Sets whether the image is part of a server-side image map.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-ismap
    pub fn set_is_map( &self, value: bool ) {
        js! { @(no_return)
            @{self}.isMap = @{value};
        }
    }

    /// Returns the intrinsic height of the image in CSS pixels, if it is available.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalHeight)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-naturalheight
    pub fn natural_height( &self ) -> Option< u32 > {
        match js!(
            return @{self}.naturalHeight;
        ).try_into().unwrap() {
            0 => None,
            value => Some( value )
        }
    }

    /// Returns the intrinsic width of the image in CSS pixels, if it is available.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalWidth)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-naturalwidth
    pub fn natural_width( &self ) -> Option< u32 > {
        match js!(
            return @{self}.naturalWidth;
        ).try_into().unwrap() {
            0 => None,
            value => Some( value )
        }
    }

    /// Returns the full URL of the image, including the base URI.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-src
    pub fn src( &self ) -> String {
        js! (
            return @{self}.src;
        ).try_into().unwrap()
    }

    /// Sets the full URL of the image, including the base URI.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-src
    pub fn set_src( &self, value: &str ) {
        js! { @(no_return)
            @{self}.src = @{value};
        }
    }

    /// Returns the `usemap` HTML attribute, containing a partial URL of a map element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-usemap
    pub fn use_map( &self ) -> String {
        js!(
            return @{self}.useMap;
        ).try_into().unwrap()
    }

    /// Sets the `usemap` HTML attribute, containing a partial URL of a map element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-usemap
    pub fn set_use_map( &self, value: &str ) {
        js! { @(no_return)
             @{self}.useMap = @{value};
        }
    }

    /// Returns the rendered width of the image in CSS pixels.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-width
    pub fn width( &self ) -> u32 {
        js! (
            return @{self}.width;
        ).try_into().unwrap()
    }

    /// Sets the rendered width of the image in CSS pixels.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)
    // https://html.spec.whatwg.org/#the-img-element:dom-img-width
    pub fn set_width( &self, value: u32 ) {
        js! { @(no_return)
            @{self}.width = @{value};
        }
    }
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let image = ImageElement::new();
        assert_eq!(image.alt(), "");
    }

    #[test]
    fn test_with_size() {
        let image = ImageElement::with_size(333, 444);
        assert_eq!(image.width(), 333);
        assert_eq!(image.height(), 444);
    }

    #[test]
    fn test_alt() {
        let image = ImageElement::new();
        assert_eq!(image.alt(), "");
        image.set_alt("test");
        assert_eq!(image.alt(), "test");
    }

    #[test]
    fn test_complete() {
        let image = ImageElement::new();
        assert_eq!(image.complete(), true);
    }

    #[test]
    fn test_width_height() {
        let image = ImageElement::new();
        assert_eq!(image.width(), 0);
        assert_eq!(image.height(), 0);
        image.set_width(4);
        image.set_height(5);
        assert_eq!(image.width(), 4);
        assert_eq!(image.height(), 5);
    }

    #[test]
    fn test_src() {
        let image = ImageElement::new();
        assert_eq!(image.src(), "");
        image.set_src("http://example.com/image.gif");
        assert_eq!(image.src(), "http://example.com/image.gif");
    }

    #[test]
    fn test_use_map() {
        let image = ImageElement::new();
        assert_eq!(image.use_map(), "");
        image.set_use_map("test");
        assert_eq!(image.use_map(), "test");
    }

    #[test]
    fn test_natural_width_height() {
        let image = ImageElement::new();
        assert_eq!(image.natural_width(), None);
        assert_eq!(image.natural_height(), None);
    }

    #[test]
    fn test_cross_origin() {
        let image = ImageElement::new();
        assert_eq!(image.cross_origin(), CrossOriginSetting::None);
        image.set_cross_origin(CrossOriginSetting::Anonymous);
        assert_eq!(image.cross_origin(), CrossOriginSetting::Anonymous);
        image.set_cross_origin(CrossOriginSetting::UseCredentials);
        assert_eq!(image.cross_origin(), CrossOriginSetting::UseCredentials);
        image.set_cross_origin(CrossOriginSetting::None);
        assert_eq!(image.cross_origin(), CrossOriginSetting::None);
    }
}
