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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Distance {
    Meters(Meters),
    Feet(Feet),
    Inches(Inches),
}
impl Distance {
    pub fn to_meter(&self) -> Meters {
        match self {
            Distance::Meters(m) => *m,
            Distance::Feet(f) => feet_to_meters(*f),
            Distance::Inches(i) => feet_to_meters(*i / 12.0),
        }
    }

    pub fn to_feet(&self) -> Feet {
        match self {
            Distance::Meters(m) => meters_to_feet(*m),
            Distance::Feet(f) => *f,
            Distance::Inches(i) => *i / 12.0,
        }
    }

    pub fn to_inch(&self) -> Inches {
        match self {
            Distance::Meters(m) => meters_to_feet(*m) * 12.0,
            Distance::Feet(f) => *f * 12.0,
            Distance::Inches(i) => *i,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Angle {
    Radians(Radians),
    Degrees(Degrees),
    Rotations(f64),
}
impl Angle {
    pub fn to_radians(&self) -> Radians {
        match self {
            Angle::Radians(r) => *r,
            Angle::Degrees(d) => degrees_to_radians(*d),
            Angle::Rotations(r) => r * 2.0 * PI,
        }
    }

    pub fn to_degrees(&self) -> Degrees {
        match self {
            Angle::Radians(r) => radians_to_degrees(*r),
            Angle::Degrees(d) => *d,
            Angle::Rotations(r) => r * 360.0,
        }
    }

    pub fn to_rotations(&self) -> f64 {
        match self {
            Angle::Radians(r) => r / (2.0 * PI),
            Angle::Degrees(d) => d / 360.0,
            Angle::Rotations(r) => *r,
        }
    }
}