use webcore::value::{Reference, ConversionError};
use webcore::try_from::TryInto;
use webapi::html_elements::CanvasElement;
use webapi::html_elements::{CanvasElement, ImageElement};

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

#[derive(Debug)]
pub enum FillRule {
    NonZero,
    EvenOdd
}

#[derive(Debug)]
pub enum Repitition {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat
}

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
    
    /// Adds an arc to the path which is centered at (x, y) position with radius r starting 
    /// at startAngle and ending at endAngle going in the given direction by anticlockwise 
    /// (defaulting to clockwise).
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)
    pub fn arc(&self, x: f64, y: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) {
        js! { @(no_return)
            @{&self.0}.arc(@{x}, @{y}, @{start_angle}, @{end_angle}, @{anticlockwise});
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
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        js! { @(no_return)
            @{&self.0}.arcTo(@{x1}, @{y1}, @{x2}, @{y2}, @{radius});
        }
    }

    /// Starts a new path by emptying the list of sub-paths. Call this method when you want to create a new path.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)
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
    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.bezierCurveTo(@{cp1x}, @{cp1y}, @{cp2x}, @{cp2y}, @{x}, @{y});
        }
    }

    /// Sets all pixels in the rectangle defined by starting point (x, y) and size (width, height) 
    /// to transparent black, erasing any previously drawn content.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)
    pub fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        js! { @(no_return)
            @{&self.0}.clearRect(@{x}, @{y}, @{width}, @{width}, @{height});
        }
    }

    /// Turns the path currently being built into the current clipping path.
    /// ctx.clip(path, fillRule) is not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)
    pub fn clip(&self, fill_rule: Option<FillRule>) {
        if let Some(fill_rule) = fill_rule {
            let fill_rule_str;
            match fill_rule {
                FillRule::NonZero => {
                    fill_rule_str = "nonzero";
                }

                FillRule::EvenOdd => {
                    fill_rule_str = "evenodd";
                }
            }
            js! { @(no_return)
                @{&self.0}.clip(@{fill_rule_str});
            }    
        }
        else {
            js! { @(no_return)
                @{&self.0}.clip();
            }
        }
    }

    /// Causes the point of the pen to move back to the start of the current sub-path. It tries 
    /// to add a straight line (but does not actually draw it) from the current point to the start. 
    /// If the shape has already been closed or has only one point, this function does nothing.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)
    pub fn close_path(&self) {
        js! { @(no_return)
            @{&self.0}.closePath();
        }
    }

    /// Creates a gradient along the line given by the coordinates represented by the parameters.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createLinearGradient)
    pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) {
        //TODO: returns linear gradient
        js! { @(no_return)
            @{&self.0}.createLinearGradient(@{x0}, @{y0}, @{x1}, @{y1});
        }
    }

    /// Creates a pattern using the specified image (a CanvasImageSource). It repeats the source in 
    /// the directions specified by the repetition argument. This method returns a CanvasPattern.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
    pub fn create_pattern_image(&self, image: ImageElement, repitition: Option<Repitition>) {
        let repitition_string;
        if let Some(repitition) = repitition {
            match repitition {
                Repitition::Repeat => {
                    repitition_string = "repeat";
                }

                Repitition::RepeatX => {
                    repitition_string = "repeat-x";
                }

                Repitition::RepeatY => {
                    repitition_string = "repeat-y";
                }

                Repitition::NoRepeat => {
                    repitition_string = "no-repeat";  
                }
            }
        }
        else {
            repitition_string = "repeat";
        }

        // TODO: returns CanvasPattern
        js! { @(no_return)
            @{&self.0}.createPattern(@{image}, @{repitition_string});
        }
    }

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
