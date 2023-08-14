mod location;
mod measurement;

pub use location::{Location, Direction, find_center_point};
pub use measurement::{Distance, DistanceUnit};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_equality() {
        let distance_a = Distance::from_kilometers(10.); 
        let distance_b = Distance::from_miles(6.213712);
        let distance_c = Distance::from_kilometers(1.25);
            
        assert!(distance_a == distance_b);
        assert!(distance_a != distance_c);
    }

    #[test]
    fn unit_conversion() {
        let distance_a = Distance::from_miles(5.2);
        let distance_b = Distance::from_kilometers(8.368589);

        assert!(distance_a.convert_to(DistanceUnit::Kilometers) == distance_b);
    }

    #[test]
    fn measure_distance() {
        let location_a = Location::from(40.7885447, -111.7656248);
        let location_b = Location::from(40.7945846, -111.6950349);
        let distance_a = location_a.distance(&location_b);
        let distance_b = Distance::from_kilometers(5.9868);
        
        assert!(distance_a == distance_b);
    }

    #[test]
    fn add_distance_to_point() {
        let location_a = Location::from(40.7885447, -111.7656248);
        let distance_a = Distance::from_kilometers(8.2);

        let location_b = location_a.add(&distance_a, Direction::North);
        let location_result = Location::from(40.8622065532978, -111.7656248);
    
        assert!(location_b == location_result)
    }
}
