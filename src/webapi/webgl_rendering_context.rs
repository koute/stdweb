use webcore::value::{Reference, ConversionError};
use webcore::try_from::TryInto;
use webapi::rendering_context::RenderingContext;
use webapi::html_elements::CanvasElement;

/// Used for drawing Webgl content onto the canvas element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)
// https://html.spec.whatwg.org/#webglrenderingcontext
pub struct WebGLRenderingContext(Reference);

reference_boilerplate! {
	WebGLRenderingContext,
	instanceof WebGLRenderingContext
}

impl RenderingContext for WebGLRenderingContext {
	type Error = ConversionError;
	fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
		js!(
			return @{canvas}.getContext("webgl");
		).try_into()
	}
}
	
impl WebGLRenderingContext {
	/// This specifies what color values to use when calling the clear() method.
	/// The values are clamped between 0 and 1.
	///
	/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearColor)
	pub fn clear_color(&self, red: f64, green: f64, blue: f64, alpha: f64) {
		js! { @(no_return)
			@{&self.0}.clearColor(@{red}, @{green}, @{blue}, @{alpha});
		}
	}

	/// The WebGLRenderingContext.clear() method of the WebGL API clears buffers to preset values.
	///
	/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clear)
	pub fn clear(&self) {
		js! { @(no_return)
			@{&self.0}.clear(@{&self.0}.COLOR_BUFFER_BIT);
		}
	}
}
