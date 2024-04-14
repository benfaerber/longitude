#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use std::f64::consts::PI;
use libm::atan2;
use std::fmt;
use lazy_static::lazy_static;

use crate::measurement::Distance;

lazy_static! {
    pub static ref RADIUS_OF_EARTH: Distance = Distance::from_kilometers(6378.137);
}

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

impl Location {
    pub fn from(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    pub fn distance(&self, other: &Location) -> Distance {
        let (lat1, lng1) = (self.latitude, self.longitude);
        let (lat2, lng2) = (other.latitude, other.longitude);

        let pi_180 = |x: f64| (x * PI) / 180.;
        let d_lat = pi_180(lat2) - pi_180(lat1);
        let d_lng = pi_180(lng2) - pi_180(lng1);

        let a = (d_lat / 2.).sin().powf(2.) +
            pi_180(lat2).cos().powf(2.) *
            (d_lng / 2.).sin().powf(2.);

        let c = 2. * atan2(a.sqrt(), (1. - a).sqrt());

        RADIUS_OF_EARTH.clone() * c
    }

    pub fn add(&self, distance: &Distance, direction: Direction) -> Self {
        let d = distance.kilometers() / RADIUS_OF_EARTH.kilometers();
        let c = 180. / PI;

        match direction {
            Direction::East | Direction::West => {
                let offset = d * c / (self.latitude * PI / 180.).cos();
                let scalar = if direction == Direction::East { 1. } else { -1. };

                Self {
                    latitude: self.latitude,
                    longitude: self.longitude + (offset * scalar),
                }
            },

            Direction::North | Direction::South => {
                let offset = d * c;
                let scalar = if direction == Direction::North { 1. } else { -1. };

                Self {
                    latitude: self.latitude + (offset * scalar),
                    longitude: self.longitude,
                }
            }
        }
    }

    pub fn estimate_distance(&self, other: &Location) -> f64 {
        let lat_dif = (self.latitude - other.latitude).abs();
        let lng_dif = (self.longitude - other.longitude).abs();
        lat_dif + lng_dif
    }

    pub fn to_string(&self) -> String {
        format!("{},{}", self.latitude, self.longitude)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn find_center_point(locations: Vec<&Location>) -> Location {
    let (total_lat, total_lng) = locations.iter()
        .fold((0.0, 0.0), |(alat, alng), Location {latitude, longitude}| {
            (alat + latitude, alng + longitude)
        });

    let f_lat = total_lat / locations.len() as f64;
    let f_lng = total_lng / locations.len() as f64;

    Location::from(f_lat, f_lng)
}
