use nalgebra::{ComplexField, Quaternion};

use crate::math::units::distance::Meter;
use crate::math::util::math_util::MathUtil;

use super::{Rotation3d, Translation2d};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation3d {
    pub x: Meter,
    pub y: Meter,
    pub z: Meter,
}

impl Translation3d {
    #[must_use]
    pub fn new() -> Self {
        Self::new_xyz(0.0, 0.0, 0.0)
    }

    #[must_use]
    pub fn new_xyz(x: impl Into<Meter>, y: impl Into<Meter>, z: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    #[must_use]
    pub fn new_dist_angle(dist: impl Into<Meter>, angle: Rotation3d) -> Self {
        let rectangle = Self::new_xyz(dist, 0.0, 0.0).rotate_by(&angle);
        Self {
            x: rectangle.x,
            y: rectangle.y,
            z: rectangle.z,
        }
    }

    #[must_use]
    fn get_distance(&self, other: &Self) -> Meter {
        ComplexField::sqrt(
            (other.x - self.x).square() + (other.y - self.y).square() + (other.z - self.z).square(),
        )
    }

    #[must_use]
    pub fn get_norm(&self) -> Meter {
        ComplexField::sqrt(self.x.square() + self.y.square() + self.z.square())
    }

    #[must_use]
    pub fn rotate_by(&self, other: &Rotation3d) -> Self {
        let p = Quaternion::new(0.0, self.x.into(), self.y.into(), self.z.into());
        let mut qprime: Quaternion<f64> = other.q.quaternion() * p;
        //TODO: inversion quaternion meanie
        other.q.try_inverse().map_or_else(
            || panic!("ROTATED BY ZERO QUATERNION 😪"),
            |invert| {
                qprime *= invert;
            },
        );

        Self::new_xyz(qprime.i, qprime.j, qprime.k)
    }

    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        Self::new_xyz(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[must_use]
    pub fn minus(&self, other: &Self) -> Self {
        Self::new_xyz(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    #[must_use]
    pub fn unary_minus(&self) -> Self {
        Self::new_xyz(-self.x, -self.y, -self.z)
    }

    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new_xyz(
            f64::from(self.x) * scalar,
            f64::from(self.y) * scalar,
            f64::from(self.z) * scalar,
        )
    }

    #[must_use]
    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    #[must_use]
    pub fn interpolate(&self, end_value: Self, t: f64) -> Self {
        Self::new_xyz(
            MathUtil::interpolate(self.x.into(), end_value.x.into(), t),
            MathUtil::interpolate(self.y.into(), end_value.y.into(), t),
            MathUtil::interpolate(self.z.into(), end_value.z.into(), t),
        )
    }
}

impl Default for Translation3d {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Translation2d> for Translation3d {
    fn from(translation: Translation2d) -> Self {
        Self::new_xyz(translation.x, translation.y, 0.0)
    }
}

impl From<Translation3d> for Translation2d {
    fn from(translation: Translation3d) -> Self {
        Self::new_xy(translation.x, translation.y)
    }
}
