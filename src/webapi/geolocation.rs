//! Implementation of Geolocation API as defined at:
//! https://developer.mozilla.org/en-US/docs/Web/API/Geolocation
//! https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/Using_geolocation
//!
//! Note that this implementation currently doesn't handle the failure to get position callbacks
//! nor does it handle geo_options.

use webcore::once::Once;
use webcore::try_from::TryInto;
use webcore::value::{Reference, Value};

/// Representation of positional coordinate information.
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "Object")]
pub struct Coordinates(Reference);

impl Coordinates {
    /// Return the latitude for the coordinate.
    pub fn latitude(&self) -> f64 {
        js! (
            return @{self}.latitude;
        ).try_into().unwrap()
    }

    /// Return the longitude for the coordinate.
    pub fn longitude(&self) -> f64 {
        js! (
            return @{self}.longitude;
        ).try_into().unwrap()
    }

    /// Return the latitude of the coordinate.
    pub fn altitude(&self) -> Option<f64> {
        js! (
            return @{self}.altitude;
        ).try_into().ok()
    }

    /// Return the accuracy of the coordinate reading.
    pub fn accuracy(&self) -> f64 {
        js! (
            return @{self}.accuracy;
        ).try_into().unwrap()
    }

    /// Return the accuracy of the coordinate reading.
    pub fn altitude_accuracy(&self) -> Option<f64> {
        js! (
            return @{self}.altitudeAccuracy;
        ).try_into().ok()
    }

    /// Return the heading of the coordinate.
    pub fn heading(&self) -> Option<f64> {
        js! (
            return @{self}.heading;
        ).try_into().ok()
    }

    /// Return the speed of the coordinate.
    pub fn speed(&self) -> Option<f64> {
        js! (
            return @{self}.speed;
        ).try_into().ok()
    }
}

/// Representation of position information which is coordinates at a given time.
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "Object")]
pub struct Position(Reference);

impl Position {
    /// Return the coordinate details for this position.
    pub fn coords(&self) -> Coordinates {
        Coordinates(
            js! (
                return @{self}.coords;
            ).try_into().unwrap(),
        )
    }

    /// Return the timestamp for this position recording.
    pub fn timestamp(&self) -> f64 {
        js! (
            return @{self}.timestamp;
        ).try_into().unwrap()
    }
}

/// Attempt to get current position and invoke callback on success.
pub fn get_current_position<F: FnOnce(Position) + 'static>(callback: F) {
    js! (
        navigator.geolocation.getCurrentPosition(@{Once(callback)})
    );
}

/// Handle to position watch.
#[derive(Debug)]
pub struct GeoWatchHandle(Value);

/// Watch for position changes and call function with updates.
pub fn watch_position<F: FnMut(Position) + 'static>(callback: F) -> GeoWatchHandle {
    GeoWatchHandle(js! (
            navigator.geolocation.watchPosition(@{callback})
            ))
}

impl GeoWatchHandle {
    /// Clear watch disabling callback on position updates.
    pub fn clear_watch(self) {
        js! (
            navigator.geolocation.clearWatch(@{&self.0});
            );
    }
}
