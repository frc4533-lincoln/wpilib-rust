use std::f64::consts::PI;


//metric
pub type Meters = f64;
pub type MetersPerSecond = f64;
pub type MetersPerSecondSquared = f64;

pub type Radians = f64;
pub type RadiansPerSecond = f64;
pub type RadiansPerSecondSquared = f64;


//imperial
pub type Feet = f64;
pub type FeetPerSecond = f64;
pub type FeetPerSecondSquared = f64;

pub type Degrees = f64;
pub type DegreesPerSecond = f64;
pub type DegreesPerSecondSquared = f64;

pub type Inches = f64;
pub type InchesPerSecond = f64;
pub type InchesPerSecondSquared = f64;

//time
pub type Hours = f64;
pub type Minutes = f64;
pub type Seconds = f64;
pub type Milliseconds = f64;
pub type Microseconds = f64;


pub fn meters_to_feet(meters: Meters) -> Feet {
    meters * 3.28084
}

pub fn feet_to_meters(feet: Feet) -> Meters {
    feet / 3.28084
}

pub fn radians_to_degrees(radians: Radians) -> Degrees {
    radians * PI / 180.0
}

pub fn degrees_to_radians(degrees: Degrees) -> Radians {
    degrees * 180.0 / PI
}

pub fn seconds_to_hours(seconds: Seconds) -> Hours {
    seconds / 3600.0
}

