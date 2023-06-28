

wpilib_macros::unit!(Meter, f64);
wpilib_macros::unit!(Feet, f64);


wpilib_macros::unit_conversion!(Meter f64, Feet f64, meter_to_feet);

pub fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_meter_to_feet() {
        let meter = Meter::new(1.0);
        let feet = Feet::new(3.28084);
        assert_eq!(feet, meter);
        assert_eq!(meter, feet);
        let double = feet + meter;
        assert!(double == Feet::new(6.56168));
        assert!(double == Meter::new(2.0));
        assert!(double > feet);
    }
}