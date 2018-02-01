use webcore::value::{Reference, ConversionError};
use webcore::try_from::TryInto;
use webapi::html_elements::CanvasElement;

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
pub struct CanvasRenderingContext2d(Reference);

reference_boilerplate! {
    CanvasRenderingContext2d,
    instanceof CanvasRenderingContext2D
}

impl RenderingContext for CanvasRenderingContext2d {
    type Error = ConversionError;
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
        js!(
            return @{canvas}.getContext("2d");
        ).try_into()
    }
}

impl CanvasRenderingContext2d {
    /// Draws a filled rectangle whose starting point is at the coordinates (x, y) with the
    /// specified width and height and whose style is determined by the fillStyle attribute.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)
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
}
