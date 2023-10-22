# Longitude [![crates.io](https://img.shields.io/crates/v/longitude.svg?logo=rust)](https://crates.io/crates/longitude)
A coordinate library in Rust. Dealing with coordinate math is tricky because you are trying to move a point on a spherical object (the earth). Manipulating the longitude is pretty straight forward as it is just the length from the North Pole to the South Pole. Latitude is a bit more complicated because the circumference of the earth varies based on how close you are to the equator. I use this library whenever I need to deal with interactive maps, triangulation, etc. I've used this in my app sardines and another app called Batch Watch.

## Dealing with Units of Distance

Create a distance which is a value associated with a unit.
```rust
use longitude::{Distance, DistanceUnit};
let distance_a = Distance::from_kilometers(10.);
let distance_b = Distance::from_miles(6.213712);
```
You can now perform operations on these measurements including conversion, addition, subtraction and multiplying by a scalar.

```rust
println!("{:?}", distance_a.convert_to(DistanceUnit::Kilometers));

```

## Performing operators on coordinates
```rust
// Add a distance to a coordinate point:
let location_a = Location::from(40.7885447, -111.7656248);
let distance_a = Distance::from_kilometers(8.2);

let location_b = location_a.add(&distance_a, Direction::North);
let location_result = Location::from(40.8622065532978, -111.7656248);

assert!(location_b == location_result)
```

```rust
// Measure the distance between 2 coordinate points:
let location_a = Location::from(40.7885447, -111.7656248);
let location_b = Location::from(40.7945846, -111.6950349);
let distance_a = location_a.distance(&location_b);
let distance_b = Distance::from_kilometers(5.9868);

assert!(distance_a == distance_b);

```

## How does it work?
First it uses the Distance struct for all measurements. This makes conversion easy and ensures you never get confused about units. The location struct stores longitude and latitude. This is how the distance of 2 points is calculated:
```rust
impl Location {
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
}

```
This function uses the Haversine Formula.
(Read more here: https://en.wikipedia.org/wiki/Haversine_formula)

You can also add some distance to coordinate point.
```rust
pub fn add(&self, distance: &Distance, direction: Direction) -> Self
```
The function add takes a distance and direction. The direction is North, South, East of West. If it
East or West we can use this formula:
```rust
let d = distance.kilometers() / RADIUS_OF_EARTH.kilometers();
let c = 180. / PI;
let offset = d * c / (self.latitude * PI / 180.).cos();
let scalar = if direction == Direction::East { 1. } else { -1. };

Self {
    latitude: self.latitude,
    longitude: self.longitude + (offset * scalar),
}
```
We need the trig because the latitude will vary based on distance from the equator.

Adding to the North or South is a bit more straight forward:
```rust
let d = distance.kilometers() / RADIUS_OF_EARTH.kilometers();
let c = 180. / PI;
let offset = d * c;
let scalar = if direction == Direction::North { 1. } else { -1. };

Self {
    latitude: self.latitude + (offset * scalar),
    longitude: self.longitude,
}

```
