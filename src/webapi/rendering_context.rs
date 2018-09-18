use webcore::value::{Reference, ConversionError};
use webcore::try_from::{TryFrom, TryInto};
use webcore::value::{Undefined, Value};
use webapi::html_elements::{CanvasElement, ImageElement};
use webapi::html_element::IHtmlElement;
use webapi::dom_exception::{SyntaxError, IndexSizeError, InvalidStateError, SecurityError, NotSupportedError};
use webapi::error::TypeError;

/// Trait implemented by rendering contexts which can be obtained from a canvas.
pub trait RenderingContext {
    /// Type of error which can occur whilst creating this context
    type Error;
    /// Name which identifies this kind of rendering context.
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, Self::Error> where Self: Sized;
}

/// Used for drawing rectangles, text, images and other objects onto the canvas element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
// https://html.spec.whatwg.org/#canvasrenderingcontext2d
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "CanvasRenderingContext2D")]
pub struct CanvasRenderingContext2d(Reference);

/// The CanvasGradient struct represents an opaque object describing a gradient.
/// It is returned by the methods CanvasRenderingContext2D.createLinearGradient() or
/// CanvasRenderingContext2D.createRadialGradient().
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient)
// https://html.spec.whatwg.org/#canvasgradient
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "CanvasGradient")]
pub struct CanvasGradient(Reference);

/// The CanvasPattern struct represents an opaque object describing a pattern, based on an image,
/// a canvas or a video, created by the CanvasRenderingContext2D.createPattern() method.
/// Intentionally blank, no non-experimental properties or methods.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern)
// https://html.spec.whatwg.org/#canvaspattern
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "CanvasPattern")]
pub struct CanvasPattern(Reference);

/// The ImageData struct represents the underlying pixel data of an area of a `<canvas>` element.
/// You can create a new instance by calling [`CanvasRenderingContext2d::create_image_data`](struct.CanvasRenderingContext2d.html#method.create_image_data)
/// or [`CanvasRenderingContext2d::create_image_data_size_of`](struct.CanvasRenderingContext2d.html#method.create_image_data_size_of).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)
// https://html.spec.whatwg.org/#imagedata
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ImageData")]
pub struct ImageData(Reference);

/// The TextMetrics struct represents the dimension of a text in the canvas, as created by the CanvasRenderingContext2D.measureText() method.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics)
// https://html.spec.whatwg.org/#textmetrics
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "TextMetrics")]
pub struct TextMetrics(Reference);

/// The type of compositing operation to apply when drawing new shapes
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CompositeOperation {
    /// Draws new shapes over existing canvas content
    SourceOver,
    /// Draws new shapes over existing canvas content, but only where existing canvas content overlaps. Anything not in that area becomes transparent.
    SourceIn,
    /// New shapes are drawn where there is no existing canvas content. Everything else (including old canvas content) is made transparent.
    SourceOut,
    /// Draws new shapes only where there is existing canvas content, over existing canvas content
    SourceAtop,
    /// Draws new shapes behind existing canvas content
    DestinationOver,
    /// Keeps existing canvas content where it overlaps with the new shape. Everything outside the overlap is made transparent.
    DestinationIn,
    /// The existing content is kept where it doesn't overlap with the new shape. Everything else, including the new shape area, is made transparent.
    DestinationOut,
    /// Existing content is kept only where it overlaps with the new shape. The new shape is drawn behind the existing content.
    DestinationAtop,
    /// Where both shapes overlap, the new color is determined by adding color values
    Lighter,
    /// Only the new shape is shown
    Copy,
    /// Where both shapes overlap, make it transparent
    Xor,
    /// The pixels of the new and old layer are multiplied. (Pixel values are in the range of [0,1], so this makes a darker picture)
    Multiply,
    /// Pixels from both new and old are inverted, multiplied together, then inverted again. (Pixel values are in the range of [0,1], so this makes a lighter picture)
    Screen,
    /// Applies Multiply to dark colors in the existing content, and Screen to bright colors in the existing content
    Overlay,
    /// Retains the darkest pixels
    Darken,
    /// Retains the lighest pixels
    Lighten,
    /// Divides the bottom layer by the inverted top layer.
    ColorDodge,
    /// Divides the inverted bottom layer by the top layer, and then inverts the result.
    ColorBurn,
    /// A combination of multiply and screen like overlay, but with top and bottom layer swapped.
    HardLight,
    /// A softer version of hard-light. Pure black or white does not result in pure black or white.
    SoftLight,
    /// Subtracts the bottom layer from the top layer or the other way round to always get a positive value.
    Difference,
    /// Like difference, but with lower contrast.
    Exclusion,
    /// Preserves the luma and chroma of the bottom layer, while adopting the hue of the top layer.
    Hue,
    /// Preserves the luma and hue of the bottom layer, while adopting the chroma of the top layer.
    Saturation,
    /// Preserves the luma of the bottom layer, while adopting the hue and chroma of the top layer.
    Color,
    /// Preserves the hue and chroma of the bottom layer, while adopting the luma of the top layer.
    Luminosity
}

/// The algorithm by which to determine if a point is inside a path or outside a path.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
// https://html.spec.whatwg.org/#canvasfillrule
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FillRule {
    /// [Non-zero winding rule](https://en.wikipedia.org/wiki/Nonzero-rule)
    NonZero,
    /// [Even-odd winding rule](https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule)
    EvenOdd
}

/// Certain style functions can return multiple types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanvasStyle {
    /// String representing the style
    String(String),
    /// CanvasGradient representing the style
    CanvasGradient(CanvasGradient),
    /// CanvasPattern representing the style
    CanvasPattern(CanvasPattern),
}

/// How the end points of every line are drawn.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
// https://html.spec.whatwg.org/#canvaslinecap
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineCap {
    /// The ends of lines are squared off at the endpoints
    Butt,
    /// The ends of lines are rounded
    Round,
    /// The ends of lines are squared off by adding a box with an equal width and half the height of the line's thickness.
    Square
}

/// determines how two connecting segments (of lines, arcs or curves) with non-zero lengths in a shape are
/// joined together (degenerate segments with zero lengths, whose specified endpoints and control points are
/// exactly at the same position, are skipped).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)
// https://html.spec.whatwg.org/#canvaslinejoin
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineJoin {
    /// Fills an additional triangular area
    Bevel,
    /// Rounds off the corners of a shape
    Round,
    /// Connected segments are joined by extending their outside edges to connect at a single point
    Miter
}

/// An enum indicating how to repeat the image.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Repetition {
    /// Repeat in both directions
    Repeat,
    /// Repeat horizontally
    RepeatX,
    /// Repeat vertically
    RepeatY,
    /// Don't repeat
    NoRepeat
}

/// Specifies text alignment
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)
// https://html.spec.whatwg.org/#canvastextalign
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextAlign {
    /// Text is left-aligned
    Left,
    /// Text is right-aligned
    Right,
    /// Text is centered
    Center,
    /// Text is aligned at the normal start of the line for the current locale
    Start,
    /// Text is aligned at the normal end of the line for the current locale
    End
}

/// Text baseline being used when drawing text
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)
// https://html.spec.whatwg.org/#canvastextbaseline
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextBaseline {
    /// Text baseline is top of the em square
    Top,
    /// Text baseline is the hanging baseline.
    Hanging,
    /// Text baseline is the middle of the em square
    Middle,
    /// Text baseline is the normal alphabetic baseline. (default)
    Alphabetic,
    /// Text baseline is the ideographic baseline
    Ideographic,
    /// Text baseline is the bottom of the bounding box.
    Bottom
}

error_enum_boilerplate! {
    /// A enum of the exceptions that CanvasGradient.add_color_stop() may throw
    // https://html.spec.whatwg.org/multipage/canvas.html#dom-canvasgradient-addcolorstop
    AddColorStopError,
    /// A SyntaxError if the color could not be parsed as a valid CSS color
    SyntaxError,
    /// An IndexSizeError if the offset was not between 0 and 1, inclusive
    IndexSizeError
}

error_enum_boilerplate! {
    /// A enum of the exceptions that CanvasRenderingContext2D.draw_image() and similar may throw
    DrawImageError,
    /// An IndexSizeError if the source or destination rectangle has an width or height of 0
    IndexSizeError,
    /// An InvalidStateError if the image has no image data
    InvalidStateError,
    /// A NotSupportedError
    NotSupportedError,
    /// A TypeError if the specified source element isn't supported
    TypeError
}

error_enum_boilerplate! {
    /// A enum of the exceptions that CanvasRenderingContext2D.get_image_data() may throw
    GetImageDataError,
    /// An IndexSizeError if thw width or height is 0
    IndexSizeError,
    /// A SecurityError
    SecurityError
}

impl TryFrom<Value> for CanvasStyle {
    type Error = ConversionError;

    /// Performs the conversion.
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Ok(v) = String::try_from(value.clone()) {
            return Ok(CanvasStyle::String(v));
        }
        if let Ok(v) = CanvasGradient::try_from(value.clone()) {
            return Ok(CanvasStyle::CanvasGradient(v));
        }
        if let Ok(v) = CanvasPattern::try_from(value.clone()) {
            return Ok(CanvasStyle::CanvasPattern(v));
        }
        Err(::webcore::value::ConversionError::type_mismatch( &value, "String, CanvasGradient or CanvasPattern".into() ))
    }
}

impl Default for FillRule {
    fn default() -> FillRule { FillRule::NonZero }
}

impl Default for Repetition {
    fn default() -> Repetition { Repetition::Repeat }
}

impl RenderingContext for CanvasRenderingContext2d {
    type Error = ConversionError;
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
        js!(
            return @{canvas}.getContext("2d");
        ).try_into()
    }
}

impl CanvasGradient {

    /// Adds a new stop, defined by an offset and a color, to the gradient. If the offset is
    /// not between 0 and 1, an INDEX_SIZE_ERR is returned, if the color can't be parsed as a
    /// CSS <color>, a SYNTAX_ERR is returned.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient/addColorStop)
    // https://html.spec.whatwg.org/#2dcontext:dom-canvasgradient-addcolorstop
    pub fn add_color_stop(&self, offset: f64, color: &str) -> Result<(), AddColorStopError> {
        js_try! ( @(no_return)
            @{&self.0}.addColorStop(@{offset}, @{color});
        ).unwrap()
    }
}

impl ImageData {

    /*
    /// Returns a Uint8ClampedArray representing a one-dimensional array containing the data in the RGBA order,
    /// with integer values between 0 and 255 (included).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data)
    // https://html.spec.whatwg.org/#2dcontext:dom-imagedata-data
    // TODO: Return Uint8ClampedArray reference PR 96: https://github.com/koute/stdweb/pull/96

    pub fn get_data(&self) -> TypedArray<u8> {
        js! (
            return @{&self.0}.data;
        ).try_into().unwrap()
    }*/

    /// Returns the number of rows in the image data object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/height)
    // https://html.spec.whatwg.org/#2dcontext:dom-imagedata-height
    pub fn get_height(&self) -> u32 {
        js! (
            return @{&self.0}.height;
        ).try_into().unwrap()
    }

    /// Returns the number of pixels per row in the image data object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/width)
    // https://html.spec.whatwg.org/#2dcontext:dom-imagedata-width
    pub fn get_width(&self) -> u32 {
        js! (
            return @{&self.0}.width;
        ).try_into().unwrap()
    }
}

impl CanvasRenderingContext2d {

    /// The CanvasRenderingContext2D.canvas property is a read-only reference to the HTMLCanvasElement
    /// object that is associated with the context. It might be null if there is no association with an <canvas> element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/canvas)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-canvas
    pub fn get_canvas(&self) -> CanvasElement {
        js! (
            return @{&self.0}.canvas;
        ).try_into().unwrap()
    }

    /// The CanvasRenderingContext2D.fillStyle property of the Canvas 2D API specifies the color or style to use inside shapes.
    /// The default is #000 (black).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-fillstyle
    pub fn get_fill_style(&self) -> CanvasStyle {
        js! (
            return @{&self.0}.fillStyle
        ).try_into().unwrap()
    }

    /// The CanvasRenderingContext2D.fillStyle property of the Canvas 2D API specifies the color or style to use inside shapes.
    /// The default is #000 (black).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-fillstyle
    pub fn set_fill_style_color(&self, color: &str){
        js! { @(no_return)
            @{&self.0}.fillStyle = @{color};
        }
    }

    /// The CanvasRenderingContext2D.fillStyle property of the Canvas 2D API specifies the color or style to use inside shapes.
    /// The default is #000 (black).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)
    // https://html.spec.whatwg.org/#dom-context-2d-fillstyle
    pub fn set_fill_style_gradient(&self, gradient: &CanvasGradient){
        js! { @(no_return)
            @{&self.0}.fillStyle = @{gradient};
        }
    }

    /// The CanvasRenderingContext2D.fillStyle property of the Canvas 2D API specifies the color or style to use inside shapes.
    /// The default is #000 (black).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)
    // https://html.spec.whatwg.org/#dom-context-2d-fillstyle
    pub fn set_fill_style_pattern(&self, pattern: &CanvasPattern){
        js! { @(no_return)
            @{&self.0}.fillStyle = @{pattern};
        }
    }

    /// The CanvasRenderingContext2D.font property of the Canvas 2D API specifies the current
    /// text style being used when drawing text. This string uses the same syntax as the CSS
    /// font specifier. The default font is 10px sans-serif.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-font
    pub fn get_font(&self) -> String {
        js! (
            return @{&self.0}.font
        ).try_into().unwrap()
    }

    /// The CanvasRenderingContext2D.font property of the Canvas 2D API specifies the current
    /// text style being used when drawing text. This string uses the same syntax as the CSS
    /// font specifier. The default font is 10px sans-serif.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-font
    pub fn set_font(&self, font: &str) {
        js! { @(no_return)
            @{&self.0}.font = @{font};
        }
    }

    /// The CanvasRenderingContext2D.globalAlpha property of the Canvas 2D API specifies the alpha
    /// value that is applied to shapes and images before they are drawn onto the canvas.
    /// The value is in the range from 0.0 (fully transparent) to 1.0 (fully opaque).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-globalalpha
    pub fn get_global_alpha(&self) -> f64 {
        js! (
            return @{&self.0}.globalAlpha
        ).try_into().unwrap()
    }

    /// The CanvasRenderingContext2D.globalAlpha property of the Canvas 2D API specifies the alpha
    /// value that is applied to shapes and images before they are drawn onto the canvas.
    /// The value is in the range from 0.0 (fully transparent) to 1.0 (fully opaque).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-globalalpha
    pub fn set_global_alpha(&self, global_alpha: f64) {
        js! { @(no_return)
            @{&self.0}.globalAlpha = @{global_alpha};
        }
    }

    /// The CanvasRenderingContext2D.globalCompositeOperation property of the Canvas 2D API sets the
    /// type of compositing operation to apply when drawing new shapes, where type is a string identifying
    /// which of the compositing or blending mode operations to use.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-globalcompositeoperation
    pub fn get_global_composite_operation(&self) -> CompositeOperation {
        let composite_operation_str: String = js! (
            return @{&self.0}.globalCompositeOperation
        ).try_into().unwrap();
        match composite_operation_str.as_ref() {
            "source-over" => CompositeOperation::SourceOver,
            "source-in" => CompositeOperation::SourceIn,
            "source-out" => CompositeOperation::SourceOut,
            "source-atop" => CompositeOperation::SourceAtop,
            "destination-over" => CompositeOperation::DestinationOver,
            "destination-in" => CompositeOperation::DestinationIn,
            "destination-out" => CompositeOperation::DestinationOut,
            "destination-atop" => CompositeOperation::DestinationAtop,
            "lighter" => CompositeOperation::Lighter,
            "copy" => CompositeOperation::Copy,
            "xor" => CompositeOperation::Xor,
            "multiply" => CompositeOperation::Multiply,
            "screen" => CompositeOperation::Screen,
            "overlay" => CompositeOperation::Overlay,
            "darken" => CompositeOperation::Darken,
            "lighten" => CompositeOperation::Lighten,
            "color-dodge" => CompositeOperation::ColorDodge,
            "color-burn" => CompositeOperation::ColorBurn,
            "hard-light" => CompositeOperation::HardLight,
            "soft-light" => CompositeOperation::SoftLight,
            "difference" => CompositeOperation::Difference,
            "exclusion" => CompositeOperation::Exclusion,
            "hue" => CompositeOperation::Hue,
            "saturation" => CompositeOperation::Saturation,
            "color" => CompositeOperation::Color,
            "luminosity" => CompositeOperation::Luminosity,
            _ => panic!("Unexpected globalCompositeOperation value: {:?}", composite_operation_str),
        }
    }

    /// The CanvasRenderingContext2D.globalCompositeOperation property of the Canvas 2D API sets the
    /// type of compositing operation to apply when drawing new shapes, where type is a string identifying
    /// which of the compositing or blending mode operations to use.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-globalcompositeoperation
    pub fn set_global_composite_operation(&self, composite_operation: CompositeOperation) {
        let composite_string = match composite_operation {
            CompositeOperation::SourceOver => "source-over",
            CompositeOperation::SourceIn => "source-in",
            CompositeOperation::SourceOut => "source-out",
            CompositeOperation::SourceAtop => "source-atop",
            CompositeOperation::DestinationOver => "destination-over",
            CompositeOperation::DestinationIn => "destination-in",
            CompositeOperation::DestinationOut => "destination-out",
            CompositeOperation::DestinationAtop => "destination-atop",
            CompositeOperation::Lighter => "lighter",
            CompositeOperation::Copy => "copy",
            CompositeOperation::Xor => "xor",
            CompositeOperation::Multiply => "multiply",
            CompositeOperation::Screen => "screen",
            CompositeOperation::Overlay => "overlay",
            CompositeOperation::Darken => "darken",
            CompositeOperation::Lighten => "lighten",
            CompositeOperation::ColorDodge => "color-dodge",
            CompositeOperation::ColorBurn => "color-burn",
            CompositeOperation::HardLight => "hard-light",
            CompositeOperation::SoftLight => "soft-light",
            CompositeOperation::Difference => "difference",
            CompositeOperation::Exclusion => "exclusion",
            CompositeOperation::Hue => "hue",
            CompositeOperation::Saturation => "saturation",
            CompositeOperation::Color => "color",
            CompositeOperation::Luminosity => "luminosity"
        };
        js! {@(no_return)
            @{&self.0}.globalCompositeOperation = @{composite_string};
        }
    }

    /// Determines how the end points of every line are drawn.
    /// There are three possible values for this property and those are: butt, round and square.
    /// By default this property is set to butt.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-linecap
    pub fn get_line_cap(&self) -> LineCap {
        let line_cap_str: String = js! (
            return @{&self.0}.lineCap
        ).try_into().unwrap();

        match line_cap_str.as_ref() {
            "butt" => LineCap::Butt,
            "round" => LineCap::Round,
            "square" => LineCap::Square,
            _ => panic!("Unexpected lineCap value: {:?}", line_cap_str),
        }
    }

    /// Determines how the end points of every line are drawn.
    /// There are three possible values for this property and those are: butt, round and square.
    /// By default this property is set to butt.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-linecap
    pub fn set_line_cap(&self, line_cap: LineCap) {
        let line_cap_string = match line_cap {
            LineCap::Butt => "butt",
            LineCap::Round => "round",
            LineCap::Square => "square",
        };
        js! { @(no_return)
            @{&self.0}.lineCap = @{line_cap_string};
        }
    }

    /// Sets the line dash pattern offset or "phase" to achieve a "marching ants" effect, for example.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-linedashoffset
    pub fn get_line_dash_offset(&self) -> f64 {
        js! (
            return @{&self.0}.lineDashOffset;
        ).try_into().unwrap()
    }

    /// Sets the line dash pattern offset or "phase" to achieve a "marching ants" effect, for example.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-linedashoffset
    pub fn set_line_dash_offset(&self, line_dash_offset: f64) {
        js! { @(no_return)
            @{&self.0}.lineDashOffset = @{line_dash_offset};
        }
    }

    /// Determines how two connecting segments (of lines, arcs or curves) with non-zero lengths in a shape are
    /// joined together (degenerate segments with zero lengths, whose specified endpoints and control points are
    /// exactly at the same position, are skipped).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-linejoin
    pub fn get_line_join(&self) -> LineJoin {
        let line_join_str: String = js! (
            return @{&self.0}.lineJoin;
        ).try_into().unwrap();
        match line_join_str.as_ref() {
            "bevel" => LineJoin::Bevel,
            "round" => LineJoin::Round,
            "miter" => LineJoin::Miter,
            _ => panic!("Unexpected lineJoin value: {:?}", line_join_str),
        }
    }

    /// Determines how two connecting segments (of lines, arcs or curves) with non-zero lengths in a shape are
    /// joined together (degenerate segments with zero lengths, whose specified endpoints and control points are
    /// exactly at the same position, are skipped).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-linejoin
    pub fn set_line_join(&self, line_join: LineJoin) {
        let line_join_str = match line_join {
            LineJoin::Bevel => "bevel",
            LineJoin::Round => "round",
            LineJoin::Miter => "miter",
        };
        js! { @(no_return)
            @{&self.0}.lineJoin = @{line_join_str};
        }
    }

    /// Sets the thickness of lines in space units. When getting, it returns the current value (1.0 by default).
    /// When setting, zero, negative, Infinity and NaN values are ignored; otherwise the current value is set to the new value.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-linewidth
    pub fn get_line_width(&self) -> f64 {
        js! (
            return @{&self.0}.lineWidth;
        ).try_into().unwrap()
    }

    /// Sets the thickness of lines in space units. When getting, it returns the current value (1.0 by default).
    /// When setting, zero, negative, Infinity and NaN values are ignored; otherwise the current value is set to the new value.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-linewidth
    pub fn set_line_width(&self, line_width: f64) {
        js! { @(no_return)
            @{&self.0}.lineWidth = @{line_width};
        }
    }

    /// sets the miter limit ratio in space units. When getting, it returns the current value (10.0 by default).
    /// When setting, zero, negative, Infinity and NaN values are ignored; otherwise the current value is set to the new value.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-miterlimit
    pub fn get_miter_limit(&self) -> f64 {
        js! (
            return @{&self.0}.miterLimit;
        ).try_into().unwrap()
    }

    /// sets the miter limit ratio in space units. When getting, it returns the current value (10.0 by default).
    /// When setting, zero, negative, Infinity and NaN values are ignored; otherwise the current value is set to the new value.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-miterlimit
    pub fn set_miter_limit(&self, miter_limit: f64) {
        js! { @(no_return)
            @{&self.0}.miterLimit = @{miter_limit};
        }
    }

    /// Specifies the level of the blurring effect; this value doesn't correspond to a number of pixels and is not
    /// affected by the current transformation matrix. The default value is 0.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-shadowblur
    pub fn get_shadow_blur(&self) -> f64 {
        js! (
            return @{&self.0}.shadowBlur;
        ).try_into().unwrap()
    }

    /// Specifies the level of the blurring effect; this value doesn't correspond to a number of pixels and is not
    /// affected by the current transformation matrix. The default value is 0.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-shadowblur
    pub fn set_shadow_blur(&self, shadow_blur: f64) {
        js! { @(no_return)
            @{&self.0}.shadowBlur = @{shadow_blur};
        }
    }

    /// Specifies the color of the shadow.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-shadowcolor
    pub fn get_shadow_color(&self) -> String {
        js! (
            return @{&self.0}.shadowColor;
        ).try_into().unwrap()
    }

    /// Specifies the color of the shadow.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-shadowcolor
    pub fn set_shadow_color(&self, shadow_color: &str) {
        js! { @(no_return)
            @{&self.0}.shadowColor = @{shadow_color};
        }
    }

    /// Specifies the distance that the shadow will be offset in horizontal distance.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-shadowoffsetx
    pub fn get_shadow_offset_x(&self) -> f64 {
        js! (
            return @{&self.0}.shadowOffsetX;
        ).try_into().unwrap()
    }

    /// Specifies the distance that the shadow will be offset in horizontal distance.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-shadowoffsetx
    pub fn set_shadow_offset_x(&self, shadow_offset_x: f64) {
        js! { @(no_return)
            @{&self.0}.shadowOffsetX = @{shadow_offset_x};
        }
    }

    /// Specifies the distance that the shadow will be offset in vertical distance.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-shadowoffsetx
    pub fn get_shadow_offset_y(&self) -> f64 {
        js! (
            return @{&self.0}.shadowOffsetY;
        ).try_into().unwrap()
    }

    /// Specifies the distance that the shadow will be offset in vertical distance.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-shadowoffsetx
    pub fn set_shadow_offset_y(&self, shadow_offset_y: f64) {
        js! { @(no_return)
            @{&self.0}.shadowOffsetY = @{shadow_offset_y};
        }
    }

    /// Specifies the color or style to use for the lines around shapes. The default is #000 (black).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-strokestyle
    pub fn get_stroke_style(&self) -> CanvasStyle {
        js! (
            return @{&self.0}.strokeStyle;
        ).try_into().unwrap()
    }

    /// Specifies the color or style to use for the lines around shapes. The default is #000 (black).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-strokestyle
    pub fn set_stroke_style_color(&self, color: &str){
        js! { @(no_return)
            @{&self.0}.strokeStyle = @{color};
        }
    }

    /// Specifies the color or style to use for the lines around shapes. The default is #000 (black).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-strokestyle
    pub fn set_stroke_style_gradient(&self, gradient: &CanvasGradient){
        js! { @(no_return)
            @{&self.0}.strokeStyle = @{gradient};
        }
    }

    /// Specifies the color or style to use for the lines around shapes. The default is #000 (black).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-strokestyle
    pub fn set_stroke_style_pattern(&self, pattern: &CanvasPattern){
        js! { @(no_return)
            @{&self.0}.strokeStyle = @{pattern};
        }
    }

    /// specifies the current text alignment being used when drawing text.
    /// Beware that the alignment is based on the x value of the fillText() method.
    /// So if textAlign is "center", then the text would be drawn at x - (width / 2).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-textalign
    pub fn get_text_align(&self) -> TextAlign {
        let text_align_str: String = js! (
            return @{&self.0}.textAlign;
        ).try_into().unwrap();
        match text_align_str.as_ref() {
            "center" => TextAlign::Center,
            "end" => TextAlign::End,
            "left" => TextAlign::Left,
            "right" => TextAlign::Right,
            "start" => TextAlign::Start,
            _ => panic!("Unexpected textAlign value: {:?}", text_align_str),
        }
    }

    /// specifies the current text alignment being used when drawing text.
    /// Beware that the alignment is based on the x value of the fillText() method.
    /// So if textAlign is "center", then the text would be drawn at x - (width / 2).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-textalign
    pub fn set_text_align(&self, text_align: TextAlign) {
        let text_align_str = match text_align {
            TextAlign::Center => "center",
            TextAlign::End => "end",
            TextAlign::Left => "left",
            TextAlign::Right => "right",
            TextAlign::Start => "start",
        };
        js! { @(no_return)
            @{&self.0}.textAlign = @{text_align_str};
        }
    }

    /// Specifies the current text baseline being used when drawing text.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-textbaseline
    pub fn get_text_baseline(&self) -> TextBaseline {
        let text_baseline_str: String = js! (
            return @{&self.0}.textBaseline;
        ).try_into().unwrap();
        match text_baseline_str.as_ref() {
            "alphabetic" => TextBaseline::Alphabetic,
            "bottom" => TextBaseline::Bottom,
            "hanging" => TextBaseline::Hanging,
            "ideographic" => TextBaseline::Ideographic,
            "middle" => TextBaseline::Middle,
            "top" => TextBaseline::Top,
            _ => panic!("Unexpected textBaseLine value: {:?}", text_baseline_str)
        }
    }

    /// Specifies the current text baseline being used when drawing text.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-textbaseline
    pub fn set_text_baseline(&self, text_baseline: TextBaseline) {
        let text_baseline_str = match text_baseline {
            TextBaseline::Alphabetic => "alphabetic",
            TextBaseline::Bottom => "bottom",
            TextBaseline::Hanging => "hanging",
            TextBaseline::Ideographic => "ideographic",
            TextBaseline::Middle => "middle",
            TextBaseline::Top => "top"
        };
        js! { @(no_return)
            @{&self.0}.textBaseline = @{text_baseline_str};
        }
    }

    /// Adds an arc to the path which is centered at (x, y) position with radius r starting
    /// at startAngle and ending at endAngle going in the given direction by anticlockwise
    /// (defaulting to clockwise).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-arc
    pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) {
        js! { @(no_return)
            @{&self.0}.arc(@{x}, @{y}, @{radius}, @{start_angle}, @{end_angle}, @{anticlockwise});
        }
    }

    /// Adds an arc to the path with the given control points and radius.
    /// The arc drawn will be a part of a circle, never elliptical.
    /// Typical use could be making a rounded corner.
    /// One way to think about the arc drawn is to imagine two straight segments, from the
    /// starting point (latest point in current path) to the first control point, and then
    /// from the first control point to the second control point. These two segments form
    /// a sharp corner with the first control point being in the corner. Using arcTo, the
    /// corner will instead be an arc with the given radius.
    /// The arc is tangential to both segments, which can sometimes produce surprising results,
    /// e.g. if the radius given is larger than the distance between the starting point and the first control point.
    /// If the radius specified doesn't make the arc meet the starting point (latest point in the current path),
    /// the starting point is connected to the arc with a straight line segment.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-arcto
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Result<(), IndexSizeError> {
        js_try! ( @(no_return)
            @{&self.0}.arcTo(@{x1}, @{y1}, @{x2}, @{y2}, @{radius});
        ).unwrap()
    }

    /// Starts a new path by emptying the list of sub-paths. Call this method when you want to create a new path.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-beginpath
    pub fn begin_path(&self) {
        js! { @(no_return)
            @{&self.0}.beginPath();
        }
    }

    /// Adds a cubic Bézier curve to the path. It requires three points. The first two points
    /// are control points and the third one is the end point. The starting point is the last
    /// point in the current path, which can be changed using moveTo() before creating the Bézier curve.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/bezierCurveTo)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-beziercurveto
    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.bezierCurveTo(@{cp1x}, @{cp1y}, @{cp2x}, @{cp2y}, @{x}, @{y});
        }
    }

    /// Sets all pixels in the rectangle defined by starting point (x, y) and size (width, height)
    /// to transparent black, erasing any previously drawn content.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-clearrect
    pub fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        js! { @(no_return)
            @{&self.0}.clearRect(@{x}, @{y}, @{width}, @{height});
        }
    }

    /// Turns the path currently being built into the current clipping path.
    /// ctx.clip(path, fillRule) is not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-clip
    pub fn clip(&self, fill_rule: FillRule) {
        let fill_rule_str = fill_rule_to_str(fill_rule);
        js! { @(no_return)
            @{&self.0}.clip(@{fill_rule_str});
        }
    }

    /// Causes the point of the pen to move back to the start of the current sub-path. It tries
    /// to add a straight line (but does not actually draw it) from the current point to the start.
    /// If the shape has already been closed or has only one point, this function does nothing.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-closepath
    pub fn close_path(&self) {
        js! { @(no_return)
            @{&self.0}.closePath();
        }
    }

    /// Creates a gradient along the line given by the coordinates represented by the parameters.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createLinearGradient)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-createlineargradient
    pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient {
        js! (
            return @{&self.0}.createLinearGradient(@{x0}, @{y0}, @{x1}, @{y1});
        ).try_into().unwrap()
    }

    /// Creates a new, blank ImageData object with the specified dimensions.
    /// All of the pixels in the new object are transparent black.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-createimagedata
    pub fn create_image_data(&self, width: f64, height: f64) -> Result<ImageData, IndexSizeError> {
        js_try! (
            return @{&self.0}.createImageData(@{width}, @{height});
        ).unwrap()
    }

    /// Creates a new, blank ImageData object with the specified dimensions.
    /// All of the pixels in the new object are transparent black.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-createimagedata
    pub fn create_image_data_size_of(&self, image_data: ImageData) -> ImageData {
        js! (
            return @{&self.0}.createImageData(@{image_data});
        ).try_into().unwrap()
    }

    /// Creates a pattern using the specified image (a CanvasImageSource). It repeats the source in
    /// the directions specified by the repetition argument. This method returns a CanvasPattern.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-createpattern
    pub fn create_pattern_image(&self, image: ImageElement, repetition: Repetition) -> CanvasPattern {
        let repetition_string = match repetition {
            Repetition::Repeat => {
                "repeat"
            }

            Repetition::RepeatX => {
                "repeat-x"
            }

            Repetition::RepeatY => {
                "repeat-y"
            }

            Repetition::NoRepeat => {
                "no-repeat"
            }
        };

        js! (
            return @{&self.0}.createPattern(@{image}, @{repetition_string});
        ).try_into().unwrap()
    }

    /// Creates a radial gradient given by the coordinates of the two circles represented by the parameters.
    /// This method returns a CanvasGradient.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createRadialGradient)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-createradialgradient
    pub fn create_radial_gradient(&self, x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> Result<CanvasGradient, IndexSizeError> {
        js_try! (
            return @{&self.0}.createRadialGradient(@{x0}, @{y0}, @{r0}, @{x1}, @{y1}, @{r1});
        ).unwrap()
    }

    /// Draws a focus ring around the current path or given path, If a given element is focused.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawFocusIfNeeded)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-drawfocusifneeded
    pub fn draw_focus_if_needed< T: IHtmlElement >(&self, element: &T) {
        js! { @(no_return)
            @{&self.0}.drawFocusIfNeeded(@{element.as_ref()});
        }
    }

    /// Provides different ways to draw an image onto the canvas.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-drawimage
    pub fn draw_image(&self, image: ImageElement, dx: f64, dy: f64) -> Result<(), DrawImageError> {
        js_try! (@(no_return)
            @{&self.0}.drawImage(@{image}, @{dx}, @{dy});
        ).unwrap()
    }

    /// Provides different ways to draw an image onto the canvas.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-drawimage
    pub fn draw_image_d(&self, image: ImageElement, dx: f64, dy: f64, d_width: f64, d_height: f64) -> Result<(), DrawImageError> {
        js_try! (@(no_return)
            @{&self.0}.drawImage(@{image}, @{dx}, @{dy}, @{d_width}, @{d_height});
        ).unwrap()
    }

    /// Provides different ways to draw an image onto the canvas.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-drawimage
    pub fn draw_image_s(&self, image: ImageElement,
                        sx: f64, sy: f64, s_width: f64, s_height: f64,
                        dx: f64, dy: f64, d_width: f64, d_height: f64
                    ) -> Result<(), DrawImageError> {
        js_try!(@(no_return)
            @{&self.0}.drawImage(@{image}, @{sx}, @{sy}, @{s_width}, @{s_height}, @{dx}, @{dy}, @{d_width}, @{d_height});
        ).unwrap()
    }

    /// Fills the current or given path with the current fill style using the non-zero or even-odd winding rule.
    ///
    /// ctx.fill(path, fillRule) is not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-fill
    pub fn fill(&self, fill_rule: FillRule) {
        let fill_rule_str = fill_rule_to_str(fill_rule);
        js! { @(no_return)
            @{&self.0}.fill(@{fill_rule_str});
        }
    }

    /// Draws a filled rectangle whose starting point is at the coordinates (x, y) with the
    /// specified width and height and whose style is determined by the fillStyle attribute.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-fillrect
    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        js! { @(no_return)
            @{&self.0}.fillRect(@{x}, @{y}, @{width}, @{height});
        }
    }

    /// Draws a text string at the specified coordinates, filling the string's characters
    /// with the current foreground color. An optional parameter allows specifying a maximum
    /// width for the rendered text, which the user agent will achieve by condensing the
    /// text or by using a lower font size.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-filltext
    pub fn fill_text(&self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        if let Some(max_width) = max_width {
            js! { @(no_return)
                @{&self.0}.fillText(@{text}, @{x}, @{y}, @{max_width});
            }
        }
        else {
            js! { @(no_return)
                @{&self.0}.fillText(@{text}, @{x}, @{y});
            }
        }
    }

    /// Returns an ImageData object representing the underlying pixel data for the area of the
    /// canvas denoted by the rectangle which starts at (sx, sy) and has an sw width and sh height.
    /// This method is not affected by the canvas transformation matrix.
    /// Pixels outside of the canvas area are present as transparent black values in the returned ImageData.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-getimagedata
    pub fn get_image_data(&self, sx: f64, sy: f64, sw: f64, sh: f64) -> Result<ImageData, GetImageDataError> {
        js_try! (
            return @{&self.0}.getImageData(@{sx}, @{sy}, @{sw}, @{sh});
        ).unwrap()
    }

    /// Gets the current line dash pattern.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-getlinedash
    pub fn get_line_dash(&self) -> Vec<f64> {
        js! (
            return @{&self.0}.getLineDash();
        ).try_into().unwrap()
    }

    /// Reports whether or not the specified point is contained in the current path.
    ///
    /// ctx.isPointInPath(path, x, y) and ctx.isPointInPath(path, x, y, fillRule)
    /// are not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-ispointinpath
    pub fn is_point_in_path(&self, x: f64, y: f64, fill_rule: FillRule) -> bool {
        let fill_rule_str = fill_rule_to_str(fill_rule);
        js! (
            return @{&self.0}.isPointInPath(@{x}, @{y}, @{fill_rule_str});
        ).try_into().unwrap()
    }

    /// Reports whether or not the specified point is inside the area contained by the stroking of a path.
    ///
    /// ctx.isPointInStroke(path, x, y) is not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-ispointinstroke
    pub fn is_point_in_stroke(&self, x: f64, y: f64) -> bool {
        js! (
            return @{&self.0}.isPointInStroke(@{x}, @{y});
        ).try_into().unwrap()
    }

    /// Connects the last point in the sub-path to the x, y coordinates with a straight line (but does not actually draw it).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-lineto
    pub fn line_to(&self, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.lineTo(@{x}, @{y});
        }
    }

    /// Returns a TextMetrics object that contains information about the measured text (such as its width for example).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-measuretext
    pub fn measure_text(&self, text: &str) -> Result<TextMetrics, SecurityError> {
        js_try! (
            return @{&self.0}.measureText(@{text});
        ).unwrap()
    }

    /// Moves the starting point of a new sub-path to the (x, y) coordinates.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-moveto
    pub fn move_to(&self, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.moveTo(@{x}, @{y});
        }
    }

    /// Paints data from the given ImageData object onto the bitmap. If a dirty rectangle is provided, only the pixels
    /// from that rectangle are painted. This method is not affected by the canvas transformation matrix.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-putimagedata
    pub fn put_image_data(&self,
                            image_data: ImageData,
                            dx: f32, dy: f32
                        ) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{&self.0}.putImageData(@{image_data}, @{dx}, @{dy});
        ).unwrap()
    }

    /// Paints data from the given ImageData object onto the bitmap. If a dirty rectangle is provided, only the pixels
    /// from that rectangle are painted. This method is not affected by the canvas transformation matrix.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-putimagedata
    pub fn put_image_data_dirty(&self,
                            image_data: ImageData,
                            dx: f32, dy: f32,
                            dirty_x: f32, dirty_y: f32,
                            dirty_width: f32, dirty_height: f32
                        ) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{&self.0}.putImageData(@{image_data}, @{dx}, @{dy}, @{dirty_x}, @{dirty_y}, @{dirty_width}, @{dirty_height});
        ).unwrap()
    }

    /// Adds a quadratic Bézier curve to the path. It requires two points.
    /// The first point is a control point and the second one is the end point.
    /// The starting point is the last point in the current path, which can be changed using
    /// moveTo() before creating the quadratic Bézier curve.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-quadraticcurveto
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x:f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.quadraticCurveTo(@{cpx}, @{cpy}, @{x}, @{y});
        }
    }

    /// Creates a path for a rectangle at position (x, y) with a size that is determined by width and height.
    /// Those four points are connected by straight lines and the sub-path is marked as closed,
    /// so that you can fill or stroke this rectangle.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rect)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-rect
    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        js! { @(no_return)
            @{&self.0}.rect(@{x}, @{y}, @{width}, @{height});
        }
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack.
    /// If there is no saved state, this method does nothing.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-restore
    pub fn restore(&self) {
        js! { @(no_return)
            @{&self.0}.restore();
        }
    }

    /// Adds a rotation to the transformation matrix. The angle argument represents a clockwise rotation angle and is expressed in radians.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-rotate
    pub fn rotate(&self, angle: f64) {
        js! { @(no_return)
            @{&self.0}.rotate(@{angle});
        }
    }

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-save
    pub fn save(&self) {
        js! { @(no_return)
            @{&self.0}.save();
        }
    }

    /// adds a scaling transformation to the canvas units by x horizontally and by y vertically.
    /// By default, one unit on the canvas is exactly one pixel. If we apply, for instance, a scaling factor of 0.5,
    /// the resulting unit would become 0.5 pixels and so shapes would be drawn at half size.
    /// In a similar way setting the scaling factor to 2.0 would increase the unit size and one unit now becomes two pixels.
    /// This results in shapes being drawn twice as large.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/scale)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-scale
    pub fn scale(&self, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.scale(@{x}, @{y});
        }
    }

    /// Sets the line dash pattern used when stroking lines, using an array of values which specify alternating lengths of lines and gaps which describe the pattern.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-setlinedash
    pub fn set_line_dash(&self, segments: Vec<f64>) {
        js! { @(no_return)
            @{&self.0}.setLineDash(@{segments});
        }
    }

    /// Resets (overrides) the current transformation to the identity matrix and then invokes a transformation described by the arguments of this method.
    /// See also the transform() method, which does not override the current transform matrix and multiplies it with a given one.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-settransform
    pub fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        js! { @(no_return)
            @{&self.0}.setTransform(@{a}, @{b}, @{c}, @{d}, @{e}, @{f});
        }
    }

    /// Strokes the current or given path with the current stroke style using the non-zero winding rule.
    ///
    /// ctx.stroke(path) is not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-stroke
    pub fn stroke(&self) {
        js! { @(no_return)
            @{&self.0}.stroke();
        }
    }

    /// Paints a rectangle which has a starting point at (x, y) and has a w width and an h height onto the canvas, using the current stroke style.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-strokerect
    pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        js! { @(no_return)
            @{&self.0}.strokeRect(@{x}, @{y}, @{width}, @{height});
        }
    }

    /// Strokes — that is, draws the outlines of — the characters of a specified text string at the given (x, y) position.
    /// If the optional fourth parameter for a maximum width is provided, the text is scaled to fit that width.
    /// See the CanvasRenderingContext2D.fillText() method to draw the text with the characters filled with color rather than having just their outlines drawn.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-stroketext
    pub fn stroke_text(&self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        if let Some(max_width) = max_width {
            js! { @(no_return)
                @{&self.0}.strokeText(@{text}, @{x}, @{y}, @{max_width});
            }
        }
        else {
            js! { @(no_return)
                @{&self.0}.strokeText(@{text}, @{x}, @{y}, @{Undefined});
            }
        }
    }

    /// Multiplies the current transformation with the matrix described by the arguments of this method.
    /// You are able to scale, rotate, move and skew the context.
    /// See also the setTransform() method which resets the current transform to the identity matrix and then invokes transform().
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/transform)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-transform
    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        js! { @(no_return)
            @{&self.0}.transform(@{a}, @{b}, @{c}, @{d}, @{e}, @{f});
        }
    }

    /// Adds a translation transformation by moving the canvas and its origin x horizontally and y vertically on the grid.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)
    // https://html.spec.whatwg.org/#2dcontext:dom-context-2d-translate
    pub fn translate(&self, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.translate(@{x}, @{y});
        }
    }
}

fn fill_rule_to_str(fill_rule: FillRule) -> &'static str {
    match fill_rule {
        FillRule::NonZero => {
            "nonzero"
        }

        FillRule::EvenOdd => {
            "evenodd"
        }
    }
}

impl TextMetrics {

    /// Contains the text's advance width (the width of that inline box) in CSS pixels.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/width)
    // https://html.spec.whatwg.org/#2dcontext:dom-textmetrics-width
    pub fn get_width(&self) -> f64 {
        js! (
            return @{&self.0}.width;
        ).try_into().unwrap()
    }
}

#[cfg(all(test, feature = "web_test"))]
mod test {
    use super::*;
    use webapi::document::document;

    fn new_canvas() -> CanvasRenderingContext2d {
        let canvas: CanvasElement = document().create_element("canvas").unwrap().try_into().unwrap();
        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
        ctx
    }

    #[test]
    fn test_canvas_fill_color() {
        let canvas = new_canvas();

        canvas.set_fill_style_color("rgb(200,0,0)");
        let style = canvas.get_fill_style();
        match style {
            CanvasStyle::String(s) => assert_eq!(s, "#c80000"),
            _ => assert!(false, "Expected style to be a string \"#c80000\" was instead {:?}", style),
        }
    }

    #[test]
    fn test_browser_create_radial_gradient() {
        let canvas = new_canvas();
        canvas.fill_rect(10 as f64, 10 as f64, 55 as f64, 50 as f64);

        let res: Result<CanvasGradient, IndexSizeError> = canvas.create_radial_gradient(100 as f64, 100 as f64, -1 as f64, 100 as f64, 100 as f64, 0 as f64);
        assert!(res.is_err());
    }
}
