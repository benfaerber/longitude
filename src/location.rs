use core::f64::consts::PI;
#[cfg(feature = "std")]
use core::fmt;
#[cfg(feature = "std")]
use lazy_static::lazy_static;
use libm::{atan2, cos, fmod, pow, sin, sqrt};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::measurement::Distance;

#[cfg(feature = "std")]
lazy_static! {
    pub static ref RADIUS_OF_EARTH: Distance = Distance::from_kilometers(6378.137);
}

#[cfg(not(feature = "std"))]
pub static RADIUS_OF_EARTH: Distance = Distance::from_kilometers(6378.137);

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

/// Normalizes longitude to the range [-180, 180].
fn normalize_longitude(lng: f64) -> f64 {
    let mut result = fmod(lng, 360.0);
    if result > 180.0 {
        result -= 360.0;
    } else if result < -180.0 {
        result += 360.0;
    }
    result
}

/// Normalizes latitude and longitude, handling pole crossings.
/// When latitude exceeds 90 deg, it "bounces back" and longitude flips by 180 deg.
fn normalize_coordinates(lat: f64, lng: f64) -> (f64, f64) {
    // First, normalize latitude to [-180, 180] range by wrapping
    let mut lat = fmod(lat, 360.0);
    if lat > 180.0 {
        lat -= 360.0;
    } else if lat < -180.0 {
        lat += 360.0;
    }

    let mut result_lat = lat;
    let mut result_lng = lng;

    // Handle pole crossings
    if lat > 90.0 {
        // Crossed north pole: bounce back and flip longitude
        result_lat = 180.0 - lat;
        result_lng += 180.0;
    } else if lat < -90.0 {
        // Crossed south pole: bounce back and flip longitude
        result_lat = -180.0 - lat;
        result_lng += 180.0;
    }

    (result_lat, normalize_longitude(result_lng))
}

impl Location {
    pub fn from(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Creates a new Location with normalized coordinates.
    /// Latitude is clamped/wrapped to [-90, 90] with proper pole crossing handling.
    /// Longitude is wrapped to [-180, 180].
    pub fn from_normalized(latitude: f64, longitude: f64) -> Self {
        let (lat, lng) = normalize_coordinates(latitude, longitude);
        Self {
            latitude: lat,
            longitude: lng,
        }
    }

    pub fn distance(&self, other: &Location) -> Distance {
        let (lat1, lng1) = (self.latitude, self.longitude);
        let (lat2, lng2) = (other.latitude, other.longitude);

        let pi_180 = |x: f64| (x * PI) / 180.;
        let d_lat = pi_180(lat2) - pi_180(lat1);
        let d_lng = pi_180(lng2) - pi_180(lng1);

        let a = pow(sin(d_lat / 2.), 2.) + pow(cos(pi_180(lat2)), 2.) * pow(sin(d_lng / 2.), 2.);

        let c = 2. * atan2(sqrt(a), sqrt(1. - a));

        RADIUS_OF_EARTH.clone() * c
    }

    pub fn add(&self, distance: &Distance, direction: Direction) -> Self {
        let d = distance.kilometers() / RADIUS_OF_EARTH.kilometers();
        let c = 180. / PI;

        match direction {
            Direction::East | Direction::West => {
                let offset = d * c / cos(self.latitude * PI / 180.);
                let scalar = if direction == Direction::East {
                    1.
                } else {
                    -1.
                };

                let new_lng = self.longitude + (offset * scalar);
                Self::from_normalized(self.latitude, new_lng)
            }

            Direction::North | Direction::South => {
                let offset = d * c;
                let scalar = if direction == Direction::North {
                    1.
                } else {
                    -1.
                };

                let new_lat = self.latitude + (offset * scalar);
                Self::from_normalized(new_lat, self.longitude)
            }
        }
    }

    pub fn estimate_distance(&self, other: &Location) -> f64 {
        let lat_dif = (self.latitude - other.latitude).abs();
        let lng_dif = (self.longitude - other.longitude).abs();
        lat_dif + lng_dif
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.latitude, self.longitude)
    }
}

pub fn find_center_point(locations: &[Location]) -> Location {
    let (total_lat, total_lng) = locations.iter().fold(
        (0.0, 0.0),
        |(alat, alng),
         Location {
             latitude,
             longitude,
         }| { (alat + latitude, alng + longitude) },
    );

    let f_lat = total_lat / locations.len() as f64;
    let f_lng = total_lng / locations.len() as f64;

    Location::from(f_lat, f_lng)
}
