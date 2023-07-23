use crate::math::units::{angle::Radian, distance::Meter};

use nalgebra::ComplexField;
use num::clamp;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rotation2d {
    pub value: Radian,
    pub sin: f64,
    pub cos: f64,
}
impl Rotation2d {
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: 0.0.into(),
            sin: 0.0,
            cos: 1.0,
        }
    }
    pub fn new_angle(angle: impl Into<Radian>) -> Self {
        let value: Radian = angle.into();
        Self {
            value,
            sin: value.sin().into(),
            cos: value.cos().into(),
        }
    }
    pub fn new_xy(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        let x = x.into();
        let y = y.into();
        let magnitude = x.hypot(y);
        let sin;
        let cos;
        if magnitude > 1e-6 {
            sin = f64::from(y) / magnitude;
            cos = f64::from(x) / magnitude;
        } else {
            sin = 0.0;
            cos = 1.0;
        }
        let value = sin.atan2(cos);
        Self {
            value: value.into(),
            sin,
            cos,
        }
    }

    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        self.rotate_by(other)
    }
    #[must_use]
    pub fn minus(&self, other: &Self) -> Self {
        self.rotate_by(&other.unary_minus())
    }
    #[must_use]
    pub fn unary_minus(&self) -> Self {
        Self::new_angle(-self.value)
    }
    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new_angle(f64::from(self.value) * scalar)
    }
    #[must_use]
    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }
    #[must_use]
    pub fn rotate_by(&self, other: &Self) -> Self {
        Self::new_xy(
            self.cos.mul_add(other.cos, -self.sin * other.sin),
            self.cos.mul_add(other.sin, -self.sin * other.cos),
        )
    }

    #[must_use]
    pub fn get_tan(&self) -> f64 {
        self.sin / self.cos
    }

    #[must_use]
    pub fn interpolate(&self, end_value: &Self, t: f64) -> Self {
        self.plus(&end_value.minus(self).times(clamp(t, 0.0, 1.0)))
    }
}

impl Default for Rotation2d {
    fn default() -> Self {
        Self::new()
    }
}
