use webapi::element::Element;
use webcore::try_from::TryInto;
use webcore::value::Reference;

/// The Touch interface represents a single contact point on a touch-sensitive
/// device. The contact point is commonly a finger or stylus and the device may
/// be a touchscreen or trackpad.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch)
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "Touch")]
pub struct Touch( Reference );

impl Touch {

    /// Returns a unique identifier for this Touch object. A given touch point
    /// (say, by a finger) will have the same identifier for the duration of
    /// its movement around the surface. This lets you ensure that you're
    /// tracking the same touch all the time.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/identifier)
    #[inline]
    pub fn identifier(&self) -> f64 {
        js!(
            return @{self.as_ref()}.identifier;
        ).try_into().unwrap()
    }

    /// Returns the X coordinate of the touch point relative to the left edge of the screen.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenX)
    #[inline]
    pub fn screen_x(&self) -> f64 {
        js!(
            return @{self.as_ref()}.screenX;
        ).try_into().unwrap()
    }

    /// Returns the Y coordinate of the touch point relative to the left edge of the screen.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenY)
    #[inline]
    pub fn screen_y(&self) -> f64 {
        js!(
            return @{self.as_ref()}.screenY;
        ).try_into().unwrap()
    }

    /// Returns the X coordinate of the touch point relative to the left edge of the browser viewport, not including any scroll offset.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientX)
    #[inline]
    pub fn client_x(&self) -> f64 {
        js!(
            return @{self.as_ref()}.clientX;
        ).try_into().unwrap()
    }

    /// Returns the Y coordinate of the touch point relative to the left edge of the browser viewport, not including any scroll offset.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientY)
    #[inline]
    pub fn client_y(&self) -> f64 {
        js!(
            return @{self.as_ref()}.clientY;
        ).try_into().unwrap()
    }

    /// Returns the X coordinate of the touch point relative to the left edge of the document. Unlike clientX, this value includes the horizontal scroll offset, if any.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageX)
    #[inline]
    pub fn page_x(&self) -> f64 {
        js!(
            return @{self.as_ref()}.pageX;
        ).try_into().unwrap()
    }

    /// Returns the Y coordinate of the touch point relative to the left edge of the document. Unlike clientX, this value includes the horizontal scroll offset, if any.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageY)
    #[inline]
    pub fn page_y(&self) -> f64 {
        js!(
            return @{self.as_ref()}.pageY;
        ).try_into().unwrap()
    }

    /// Returns the Element on which the touch point started when it was first placed on the surface, even if the touch point has since moved outside the interactive area of that element or even been removed from the document.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/target)
    #[inline]
    pub fn target(&self) -> Element {
        js!(
            return @{self.as_ref()}.target;
        ).try_into().unwrap()
    }

    /// Returns the X radius of the ellipse that most closely circumscribes the area of contact with the screen. The value is in pixels of the same scale as screenX.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusX)
    #[inline]
    pub fn radius_x(&self) -> f64 {
        js!(
            return @{self.as_ref()}.radiusX;
        ).try_into().unwrap()
    }

    /// Returns the Y radius of the ellipse that most closely circumscribes the area of contact with the screen. The value is in pixels of the same scale as screenY.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusY)
    #[inline]
    pub fn radius_y(&self) -> f64 {
        js!(
            return @{self.as_ref()}.radiusY;
        ).try_into().unwrap()
    }

    /// Returns the angle (in degrees) that the ellipse described by radiusX and radiusY must be rotated, clockwise, to most accurately cover the area of contact between the user and the surface.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/rotationAngle)
    #[inline]
    pub fn rotation_angle(&self) -> f64 {
        js!(
            return @{self.as_ref()}.rotationAngle;
        ).try_into().unwrap()
    }

    /// Returns the amount of pressure being applied to the surface by the user, as a float between 0.0 (no pressure) and 1.0 (maximum pressure).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Touch/force)
    #[inline]
    pub fn force(&self) -> f64 {
        js!(
            return @{self.as_ref()}.force;
        ).try_into().unwrap()
    }
}
