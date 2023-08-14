use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DistanceUnit {
    Centimeters,
    Meters,
    Kilometers,

    Inches,
    Feet,
    Yards,
    Miles,
}

impl DistanceUnit {
    pub fn in_meters(&self) -> f64 {
        match self {
            DistanceUnit::Centimeters => 0.01,
            DistanceUnit::Meters => 1.,
            DistanceUnit::Kilometers => 1000.,

            DistanceUnit::Inches => 0.0254,
            DistanceUnit::Feet => 0.3048,
            DistanceUnit::Yards => 0.9144,
            DistanceUnit::Miles => 1609.344,
        }
    }

    fn abbreviation(&self) -> String {
        match self {
            DistanceUnit::Centimeters => "cm",
            DistanceUnit::Meters => "m",
            DistanceUnit::Kilometers => "km",

            DistanceUnit::Inches => "in",
            DistanceUnit::Feet => "ft",
            DistanceUnit::Yards => "yd",
            DistanceUnit::Miles => "mi",
        }.into()
    }

    #[allow(dead_code)]
    fn name(&self) -> String {
        match self {
            DistanceUnit::Centimeters => "centimeters",
            DistanceUnit::Meters => "meters",
            DistanceUnit::Kilometers => "kilometers",

            DistanceUnit::Inches => "inches",
            DistanceUnit::Feet => "feet",
            DistanceUnit::Yards => "yards",
            DistanceUnit::Miles => "miles",
        }.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance {
    unit: DistanceUnit,
    value: f64,
}

impl Distance {
    pub fn from(value: f64, unit: DistanceUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_kilometers(value: f64) -> Self {
        Self::from(value, DistanceUnit::Kilometers)
    }

    pub fn from_meters(value: f64) -> Self {
        Self::from(value, DistanceUnit::Meters)
    }

    #[allow(dead_code)]
    pub fn from_miles(value: f64) -> Self {
        Self::from(value, DistanceUnit::Miles)
    }

    pub fn convert_to(&self, unit: DistanceUnit) -> Self {
        if self.unit == unit {
            self.clone()
        } else {
            let ratio = self.unit.in_meters() / unit.in_meters();
            let new_value = self.value * ratio;
            Self::from(new_value, unit)
        }
    }

    pub fn in_unit(&self, unit: DistanceUnit) -> f64 {
        self.convert_to(unit).value
    }

    pub fn meters(&self) -> f64 {
        self.in_unit(DistanceUnit::Meters)
    }

    pub fn kilometers(&self) -> f64 {
        self.in_unit(DistanceUnit::Kilometers)
    }

    #[allow(dead_code)]
    pub fn miles(&self) -> f64 {
        self.in_unit(DistanceUnit::Miles)
    }

    pub fn to_string(&self) -> String {
        format!("{:.1}{}", self.value, self.unit.abbreviation())
    }
}

const APPROX_EQUAL_PLACES: u8 = 3;
fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
    let factor = 10.0f64.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}

impl PartialEq for Distance {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let a = self.in_unit(self.unit);
        let b = other.in_unit(self.unit);
        approx_equal(a, b, APPROX_EQUAL_PLACES)
    }
}

impl PartialOrd for Distance {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.in_unit(self.unit).partial_cmp(&other.in_unit(self.unit))
    }
}

impl Add for Distance {
    type Output = Self;

    fn add(self, other: Distance) -> Self {
        Self::from(
            self.value + other.in_unit(self.unit),
            self.unit
        )
    }
}

impl Sub for Distance {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(
            self.value - other.in_unit(self.unit),
            self.unit
        )
    }
}

impl Mul<f64> for Distance {
    type Output = Self;

    fn mul(self, multiplier: f64) -> Self {
        Self::from(
            self.value * multiplier,
            self.unit
        )
    }
}

impl Default for Distance {
    fn default() -> Self {
        Self::from_meters(0.)
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
