use crate::math::units::{angle::Radian, distance::Meter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Twist2d {
    pub dx: Meter,
    pub dy: Meter,
    pub dtheta: Radian,
}

impl Twist2d {
    #[must_use]
    pub fn new(dx: impl Into<Meter>, dy: impl Into<Meter>, dtheta: impl Into<Radian>) -> Self {
        Self {
            dx: dx.into(),
            dy: dy.into(),
            dtheta: dtheta.into(),
        }
    }
}

impl Default for Twist2d {
    fn default() -> Self {
        Self {
            dx: 0.0.into(),
            dy: 0.0.into(),
            dtheta: 0.0.into(),
        }
    }
}
