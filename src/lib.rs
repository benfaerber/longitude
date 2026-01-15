#![cfg_attr(not(feature = "std"), no_std)]

mod location;
mod measurement;

pub use location::{find_center_point, Direction, Location};
pub use measurement::{Distance, DistanceUnit};

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Distance Unit Tests ====================

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
    fn conversion_to_same_unit() {
        let distance = Distance::from_kilometers(5.0);
        let converted = distance.convert_to(DistanceUnit::Kilometers);
        assert!(distance == converted);
    }

    #[test]
    fn conversion_all_units() {
        let distance_km = Distance::from_kilometers(1.0);

        // Test metric conversions
        let in_meters = distance_km.in_unit(DistanceUnit::Meters);
        assert!((in_meters - 1000.0).abs() < 0.001);

        let in_cm = distance_km.in_unit(DistanceUnit::Centimeters);
        assert!((in_cm - 100000.0).abs() < 0.1);

        // Test imperial conversions
        let in_miles = distance_km.in_unit(DistanceUnit::Miles);
        assert!((in_miles - 0.621371).abs() < 0.001);

        let in_feet = distance_km.in_unit(DistanceUnit::Feet);
        assert!((in_feet - 3280.84).abs() < 0.1);

        let in_yards = distance_km.in_unit(DistanceUnit::Yards);
        assert!((in_yards - 1093.61).abs() < 0.1);

        let in_inches = distance_km.in_unit(DistanceUnit::Inches);
        assert!((in_inches - 39370.1).abs() < 1.0);
    }

    #[test]
    fn distance_arithmetic_add() {
        let d1 = Distance::from_kilometers(5.0);
        let d2 = Distance::from_kilometers(3.0);
        let result = d1 + d2;
        assert!(result == Distance::from_kilometers(8.0));
    }

    #[test]
    fn distance_arithmetic_add_different_units() {
        let d1 = Distance::from_kilometers(1.0);
        let d2 = Distance::from_meters(500.0);
        let result = d1 + d2;
        assert!(result == Distance::from_kilometers(1.5));
    }

    #[test]
    fn distance_arithmetic_sub() {
        let d1 = Distance::from_kilometers(10.0);
        let d2 = Distance::from_kilometers(3.0);
        let result = d1 - d2;
        assert!(result == Distance::from_kilometers(7.0));
    }

    #[test]
    fn distance_arithmetic_mul() {
        let d = Distance::from_kilometers(5.0);
        let result = d * 3.0;
        assert!(result == Distance::from_kilometers(15.0));
    }

    #[test]
    fn distance_arithmetic_div() {
        let d = Distance::from_kilometers(15.0);
        let result = d / 3.0;
        assert!(result == Distance::from_kilometers(5.0));
    }

    #[test]
    fn distance_comparison() {
        let d1 = Distance::from_kilometers(5.0);
        let d2 = Distance::from_kilometers(10.0);
        let d3 = Distance::from_miles(3.10686); // ~5km

        assert!(d1 < d2);
        assert!(d2 > d1);
        assert!(d1 == d3);
    }

    #[test]
    fn distance_default() {
        let d = Distance::default();
        assert!(d == Distance::from_meters(0.0));
    }

    #[test]
    fn zero_distance() {
        let d = Distance::from_kilometers(0.0);
        assert!(d.kilometers() == 0.0);
        assert!(d.meters() == 0.0);
        assert!(d.miles() == 0.0);
    }

    // ==================== Location Distance Tests ====================

    #[test]
    fn measure_distance() {
        let location_a = Location::from(40.7885447, -111.7656248);
        let location_b = Location::from(40.7945846, -111.6950349);
        let distance_a = location_a.distance(&location_b);
        let distance_b = Distance::from_kilometers(5.9868);

        assert!(distance_a == distance_b);
    }

    #[test]
    fn distance_same_location() {
        let location = Location::from(40.7885447, -111.7656248);
        let distance = location.distance(&location);
        assert!(distance == Distance::from_kilometers(0.0));
    }

    #[test]
    fn distance_is_symmetric() {
        let location_a = Location::from(40.7885447, -111.7656248);
        let location_b = Location::from(40.7945846, -111.6950349);

        let d1 = location_a.distance(&location_b);
        let d2 = location_b.distance(&location_a);

        // Compare kilometers directly with small tolerance
        assert!((d1.kilometers() - d2.kilometers()).abs() < 0.001);
    }

    #[test]
    fn distance_across_equator() {
        let north = Location::from(10.0, 0.0);
        let south = Location::from(-10.0, 0.0);
        let distance = north.distance(&south);

        // ~2224 km for 20 degrees latitude
        assert!(distance.kilometers() > 2200.0);
        assert!(distance.kilometers() < 2250.0);
    }

    #[test]
    fn distance_across_prime_meridian() {
        let west = Location::from(0.0, -10.0);
        let east = Location::from(0.0, 10.0);
        let distance = west.distance(&east);

        // At equator, ~2224 km for 20 degrees longitude
        assert!(distance.kilometers() > 2200.0);
        assert!(distance.kilometers() < 2250.0);
    }

    #[test]
    fn distance_across_international_date_line() {
        let west = Location::from(0.0, 170.0);
        let east = Location::from(0.0, -170.0);
        let distance = west.distance(&east);

        // 20 degrees at equator
        assert!(distance.kilometers() > 2200.0);
        assert!(distance.kilometers() < 2250.0);
    }

    // ==================== Location Add Direction Tests ====================

    #[test]
    fn add_distance_to_point() {
        let location_a = Location::from(40.7885447, -111.7656248);
        let distance_a = Distance::from_kilometers(8.2);

        let location_b = location_a.add(&distance_a, Direction::North);
        let location_result = Location::from(40.8622065532978, -111.7656248);

        assert!(location_b == location_result)
    }

    #[test]
    fn add_distance_south() {
        let location = Location::from(40.7885447, -111.7656248);
        let distance = Distance::from_kilometers(8.2);

        let result = location.add(&distance, Direction::South);

        // Latitude should decrease, longitude unchanged
        assert!(result.latitude < location.latitude);
        assert!((result.longitude - location.longitude).abs() < 0.0001);

        // Should be symmetric with North
        let back = result.add(&distance, Direction::North);
        assert!((back.latitude - location.latitude).abs() < 0.0001);
    }

    #[test]
    fn add_distance_east() {
        let location = Location::from(40.7885447, -111.7656248);
        let distance = Distance::from_kilometers(8.2);

        let result = location.add(&distance, Direction::East);

        // Longitude should increase (less negative), latitude unchanged
        assert!(result.longitude > location.longitude);
        assert!((result.latitude - location.latitude).abs() < 0.0001);
    }

    #[test]
    fn add_distance_west() {
        let location = Location::from(40.7885447, -111.7656248);
        let distance = Distance::from_kilometers(8.2);

        let result = location.add(&distance, Direction::West);

        // Longitude should decrease (more negative), latitude unchanged
        assert!(result.longitude < location.longitude);
        assert!((result.latitude - location.latitude).abs() < 0.0001);

        // Should be symmetric with East
        let back = result.add(&distance, Direction::East);
        assert!((back.longitude - location.longitude).abs() < 0.0001);
    }

    #[test]
    fn add_zero_distance() {
        let location = Location::from(40.7885447, -111.7656248);
        let zero = Distance::from_kilometers(0.0);

        let north = location.add(&zero, Direction::North);
        let south = location.add(&zero, Direction::South);
        let east = location.add(&zero, Direction::East);
        let west = location.add(&zero, Direction::West);

        assert!(location == north);
        assert!(location == south);
        assert!(location == east);
        assert!(location == west);
    }

    #[test]
    fn add_distance_round_trip_north_south() {
        let location = Location::from(40.0, -111.0);
        let distance = Distance::from_kilometers(100.0);

        let moved = location.add(&distance, Direction::North);
        let returned = moved.add(&distance, Direction::South);

        assert!((returned.latitude - location.latitude).abs() < 0.0001);
        assert!((returned.longitude - location.longitude).abs() < 0.0001);
    }

    #[test]
    fn add_distance_round_trip_east_west() {
        let location = Location::from(40.0, -111.0);
        let distance = Distance::from_kilometers(100.0);

        let moved = location.add(&distance, Direction::East);
        let returned = moved.add(&distance, Direction::West);

        assert!((returned.latitude - location.latitude).abs() < 0.0001);
        assert!((returned.longitude - location.longitude).abs() < 0.0001);
    }

    // ==================== Estimate Distance Tests ====================

    #[test]
    fn estimate_distance_same_location() {
        let location = Location::from(40.0, -111.0);
        let estimate = location.estimate_distance(&location);
        assert!(estimate == 0.0);
    }

    #[test]
    fn estimate_distance_different_locations() {
        let location_a = Location::from(40.0, -111.0);
        let location_b = Location::from(41.0, -110.0);
        let estimate = location_a.estimate_distance(&location_b);

        // Should be sum of absolute differences: |40-41| + |-111-(-110)| = 1 + 1 = 2
        assert!((estimate - 2.0).abs() < 0.0001);
    }

    #[test]
    fn estimate_distance_is_symmetric() {
        let location_a = Location::from(40.0, -111.0);
        let location_b = Location::from(45.0, -100.0);

        let e1 = location_a.estimate_distance(&location_b);
        let e2 = location_b.estimate_distance(&location_a);

        assert!((e1 - e2).abs() < 0.0001);
    }

    #[test]
    fn estimate_distance_latitude_only() {
        let location_a = Location::from(40.0, -111.0);
        let location_b = Location::from(50.0, -111.0);
        let estimate = location_a.estimate_distance(&location_b);

        assert!((estimate - 10.0).abs() < 0.0001);
    }

    #[test]
    fn estimate_distance_longitude_only() {
        let location_a = Location::from(40.0, -111.0);
        let location_b = Location::from(40.0, -101.0);
        let estimate = location_a.estimate_distance(&location_b);

        assert!((estimate - 10.0).abs() < 0.0001);
    }

    // ==================== Find Center Point Tests ====================

    #[test]
    fn center_point_single_location() {
        let locations = [Location::from(40.0, -111.0)];
        let center = find_center_point(&locations);

        assert!((center.latitude - 40.0).abs() < 0.0001);
        assert!((center.longitude - (-111.0)).abs() < 0.0001);
    }

    #[test]
    fn center_point_two_locations() {
        let locations = [Location::from(40.0, -111.0), Location::from(50.0, -101.0)];
        let center = find_center_point(&locations);

        assert!((center.latitude - 45.0).abs() < 0.0001);
        assert!((center.longitude - (-106.0)).abs() < 0.0001);
    }

    #[test]
    fn center_point_four_corners() {
        let locations = [
            Location::from(10.0, 10.0),
            Location::from(10.0, 20.0),
            Location::from(20.0, 10.0),
            Location::from(20.0, 20.0),
        ];
        let center = find_center_point(&locations);

        assert!((center.latitude - 15.0).abs() < 0.0001);
        assert!((center.longitude - 15.0).abs() < 0.0001);
    }

    #[test]
    fn center_point_symmetric_around_origin() {
        let locations = [Location::from(10.0, 10.0), Location::from(-10.0, -10.0)];
        let center = find_center_point(&locations);

        assert!((center.latitude - 0.0).abs() < 0.0001);
        assert!((center.longitude - 0.0).abs() < 0.0001);
    }

    // ==================== Location Default & Clone Tests ====================

    #[test]
    fn location_default() {
        let loc = Location::default();
        assert!(loc.latitude == 0.0);
        assert!(loc.longitude == 0.0);
    }

    #[test]
    fn location_clone() {
        let loc = Location::from(40.0, -111.0);
        let cloned = loc.clone();
        assert!(loc == cloned);
    }

    // ==================== Direction Tests ====================

    #[test]
    fn direction_clone_and_copy() {
        let dir = Direction::North;
        let cloned = dir.clone();
        let copied = dir;
        assert!(dir == cloned);
        assert!(dir == copied);
    }

    #[test]
    fn direction_equality() {
        assert!(Direction::North == Direction::North);
        assert!(Direction::South == Direction::South);
        assert!(Direction::East == Direction::East);
        assert!(Direction::West == Direction::West);
        assert!(Direction::North != Direction::South);
        assert!(Direction::East != Direction::West);
    }

    // ==================== Coordinate Normalization Tests ====================

    #[test]
    fn longitude_wraps_past_180() {
        let location = Location::from(0.0, 170.0);
        let distance = Distance::from_kilometers(2000.0); // ~18 degrees at equator

        let result = location.add(&distance, Direction::East);

        // Should wrap around to negative longitude
        assert!(result.longitude >= -180.0);
        assert!(result.longitude <= 180.0);
    }

    #[test]
    fn longitude_wraps_past_negative_180() {
        let location = Location::from(0.0, -170.0);
        let distance = Distance::from_kilometers(2000.0);

        let result = location.add(&distance, Direction::West);

        // Should wrap around to positive longitude
        assert!(result.longitude >= -180.0);
        assert!(result.longitude <= 180.0);
    }

    #[test]
    fn latitude_wraps_past_north_pole() {
        let location = Location::from(80.0, 0.0);
        let distance = Distance::from_kilometers(2000.0); // ~18 degrees

        let result = location.add(&distance, Direction::North);

        // Should bounce back from the pole and stay in valid range
        assert!(result.latitude >= -90.0);
        assert!(result.latitude <= 90.0);
        // Latitude should be less than 90 (bounced back)
        assert!(result.latitude < 90.0);
    }

    #[test]
    fn latitude_wraps_past_south_pole() {
        let location = Location::from(-80.0, 0.0);
        let distance = Distance::from_kilometers(2000.0);

        let result = location.add(&distance, Direction::South);

        // Should bounce back from the pole and stay in valid range
        assert!(result.latitude >= -90.0);
        assert!(result.latitude <= 90.0);
        // Latitude should be greater than -90 (bounced back)
        assert!(result.latitude > -90.0);
    }

    #[test]
    fn full_circumference_east_returns_near_origin() {
        let location = Location::from(0.0, 0.0);
        // Earth's circumference at equator is ~40,075 km
        let distance = Distance::from_kilometers(40075.0);

        let result = location.add(&distance, Direction::East);

        // Should end up near the starting point
        assert!(result.latitude >= -90.0 && result.latitude <= 90.0);
        assert!(result.longitude >= -180.0 && result.longitude <= 180.0);
        assert!((result.longitude - 0.0).abs() < 1.0); // Allow small error
    }

    #[test]
    fn full_circumference_north_south() {
        let location = Location::from(0.0, 0.0);
        // Half circumference through the poles is ~20,000 km
        let distance = Distance::from_kilometers(20000.0);

        let result = location.add(&distance, Direction::North);

        // Should end up on the other side of the Earth
        assert!(result.latitude >= -90.0 && result.latitude <= 90.0);
        assert!(result.longitude >= -180.0 && result.longitude <= 180.0);
    }

    #[test]
    fn multiple_circumferences_stays_valid() {
        let location = Location::from(45.0, 90.0);
        // 3x Earth's circumference
        let distance = Distance::from_kilometers(120000.0);

        let result_east = location.add(&distance, Direction::East);
        let result_north = location.add(&distance, Direction::North);

        // All results should be in valid coordinate ranges
        assert!(result_east.latitude >= -90.0 && result_east.latitude <= 90.0);
        assert!(result_east.longitude >= -180.0 && result_east.longitude <= 180.0);
        assert!(result_north.latitude >= -90.0 && result_north.latitude <= 90.0);
        assert!(result_north.longitude >= -180.0 && result_north.longitude <= 180.0);
    }

    #[test]
    fn pole_crossing_flips_longitude() {
        let location = Location::from(85.0, 0.0);
        // Go 10 degrees north, crossing the pole
        let distance = Distance::from_kilometers(1112.0); // ~10 degrees

        let result = location.add(&distance, Direction::North);

        // After crossing the pole, longitude should flip by 180 degrees
        // and latitude should bounce back
        assert!(result.latitude >= -90.0 && result.latitude <= 90.0);
        assert!(result.longitude >= -180.0 && result.longitude <= 180.0);
        // Should be on the opposite side of the globe (longitude near 180 or -180)
        assert!((result.longitude).abs() > 170.0);
    }

    #[test]
    fn from_normalized_constructor() {
        // Test the normalized constructor directly
        let loc1 = Location::from_normalized(0.0, 270.0);
        assert!((loc1.longitude - (-90.0)).abs() < 0.001);

        let loc2 = Location::from_normalized(0.0, -270.0);
        assert!((loc2.longitude - 90.0).abs() < 0.001);

        let loc3 = Location::from_normalized(100.0, 0.0);
        assert!(loc3.latitude >= -90.0 && loc3.latitude <= 90.0);
    }
}
