use webcore::value::{Reference, ConversionError};
use webcore::try_from::TryInto;
use webcore::value::Undefined;
use webapi::html_elements::{CanvasElement, ImageElement};
use webapi::html_element::HtmlElement;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FillRule {
    NonZero,
    EvenOdd
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Repetition {
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
        let repetition_string = match repetition {
            Some(Repetition::Repeat) | None => {
                "repeat"
            }

            Some(Repetition::RepeatX) => {
                "repeat-x"
            }

            Some(Repetition::RepeatY) => {
                "repeat-y"
            }

            Some(Repetition::NoRepeat) => {
                "no-repeat"
            }
        };

        // TODO: returns CanvasPattern
        js! { @(no_return)
            @{&self.0}.createPattern(@{image}, @{repitition_string});
        }
    }

    /// Creates a radial gradient given by the coordinates of the two circles represented by the parameters. 
    /// This method returns a CanvasGradient.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createRadialGradient)
    pub fn create_radial_gradient(&self, x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) {
        //TODO: returns radial gradient
        js! { @(no_return)
            @{&self.0}.createRadialGradient(@{x0}, @{y0}, @{r0}, @{x1}, @{y1}, @{r1});
        }
    }

    /// Draws a focus ring around the current path or given path, If a given element is focused.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawFocusIfNeeded)
    pub fn draw_focus_if_needed(&self, element: HtmlElement) {
        js! { @(no_return)
            @{&self.0}.drawFocusIfNeeded(@{element});
        }
    }

    //draw_image will go here but waiting to figure out how to do CanvasImageSource

    /// Fills the current or given path with the current fill style using the non-zero or even-odd winding rule.
    /// 
    /// ctx.fill(path, fillRule) is not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
    pub fn fill(&self, fill_rule: Option<FillRule>) {
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
                @{&self.0}.fill(@{fill_rule_str});
            }    
        }
        else {
            js! { @(no_return)
                @{&self.0}.fill();
            }
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

    /// Returns an ImageData object representing the underlying pixel data for the area of the 
    /// canvas denoted by the rectangle which starts at (sx, sy) and has an sw width and sh height. 
    /// This method is not affected by the canvas transformation matrix.
    /// Pixels outside of the canvas area are present as transparent black values in the returned ImageData.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)
    pub fn get_image_data(&self, sx: f64, sy: f64, sw: f64, sh: f64) {
        //TODO: return ImageData
        js! { @(no_return)
            @{&self.0}.getImageData(@{sx}, @{sy}, @{sw}, @{sh});
        }
    }

    /// Gets the current line dash pattern.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)
    pub fn get_line_dash(&self) {
        //TODO: return Array
        js! { @(no_return)
            @{&self.0}.getLineDash();
        }
    }

    /// Reports whether or not the specified point is contained in the current path.
    /// 
    /// ctx.isPointInPath(path, x, y) and ctx.isPointInPath(path, x, y, fillRule) 
    /// are not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental 
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)
    pub fn is_point_in_path(&self, x: f64, y: f64, fill_rule: Option<FillRule>) {
        //TODO: return Boolean
        //TODO: change this fill_rule stuff into a function
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
                @{&self.0}.isPointInPath(@{x}, @{y}, @{fill_rule_str});
            }    
        }
        else {
            js! { @(no_return)
                @{&self.0}.isPointInPath(@{x}, @{y});
            }
        }
    }

    /// Reports whether or not the specified point is inside the area contained by the stroking of a path.
    /// 
    /// ctx.isPointInStroke(path, x, y) is not supported because [(Path2D)](https://developer.mozilla.org/en-US/docs/Web/API/Path2D) is still experimental 
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)
    pub fn is_point_in_stroke(&self, x: f64, y: f64) {
        //TODO: return Boolean
        js! { @(no_return)
            @{&self.0}.isPointInStroke(@{x}, @{y});
        }
    }

    /// Connects the last point in the sub-path to the x, y coordinates with a straight line (but does not actually draw it).
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)
    pub fn line_to(&self, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.lineTo(@{x}, @{y});
        }
    }

    /// Returns a TextMetrics object that contains information about the measured text (such as its width for example).
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText)
    pub fn measure_text(&self, text: &str) {
        //TODO: return TextMetrics
        js! { @(no_return)
            @{&self.0}.measureText(@{text});
        }
    }

    /// Moves the starting point of a new sub-path to the (x, y) coordinates.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)
    pub fn move_to(&self, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.moveTo(@{x}, @{y});
        }
    }

    /// Adds a quadratic Bézier curve to the path. It requires two points. 
    /// The first point is a control point and the second one is the end point. 
    /// The starting point is the last point in the current path, which can be changed using 
    /// moveTo() before creating the quadratic Bézier curve.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)
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
    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        js! { @(no_return)
            @{&self.0}.rect(@{x}, @{y}, @{width}, @{height});
        }
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. 
    /// If there is no saved state, this method does nothing.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)
    pub fn restore(&self) {
        js! { @(no_return)
            @{&self.0}.restore();
        }
    }

    /// Adds a rotation to the transformation matrix. The angle argument represents a clockwise rotation angle and is expressed in radians.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)
    pub fn rotate(&self, angle: f64) {
        js! { @(no_return)
            @{&self.0}.rotate(@{angle});
        }
    }

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)
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
    pub fn scale(&self, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.scale(@{x}, @{y});
        }
    }

    /// Sets the line dash pattern used when stroking lines, using an array of values which specify alternating lengths of lines and gaps which describe the pattern.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)
    pub fn set_line_dash(&self, segments: Vec<f64>) {
        js! { @(no_return)
            @{&self.0}.setLineDash(@{segments});
        }
    }

    /// Resets (overrides) the current transformation to the identity matrix and then invokes a transformation described by the arguments of this method.
    /// See also the transform() method, which does not override the current transform matrix and multiplies it with a given one.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)
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
    pub fn stroke(&self) {
        js! { @(no_return)
            @{&self.0}.stroke();
        }
    }

    /// Paints a rectangle which has a starting point at (x, y) and has a w width and an h height onto the canvas, using the current stroke style.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)
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
    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        js! { @(no_return)
            @{&self.0}.transform(@{a}, @{b}, @{c}, @{d}, @{e}, @{f});
        }
    }

    /// Adds a translation transformation by moving the canvas and its origin x horizontally and y vertically on the grid.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)
    pub fn translate(&self, x: f64, y: f64) {
        js! { @(no_return)
            @{&self.0}.translate(@{x}, @{y});
        }
    }
}
