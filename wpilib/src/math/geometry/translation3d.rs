use nalgebra::Quaternion;
use nalgebra::ComplexField;

use crate::math::geometry::Rotation3d;
use crate::math::util::MathUtil;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation3d {
    pub x: Meter,
    pub y: Meter,
    pub z: Meter,
}

impl Translation3d {
    pub fn new() -> Self {
        Self::new_xyz(0.0, 0.0, 0.0)
    }

    pub fn new_xyz(x: impl Into<Meter>, y: impl Into<Meter>, z: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn new_dist_angle(dist: impl Into<Meter>, angle: impl Into<Radian>) -> Self {
        let rectangle = Self::Translation3d(distance, 0.0, 0.0).rotate_by(angle);
        Self{
            x: rectangle.x,
            y: rectangle.y,
            z: rectangle.z,
        }
    }

    pub fn get_distance(&self, other: &Self) -> Meter {
        ComplexField::sqrt(
            (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)
        )
    }

    pub fn get_norm(&self) -> Meter {
        ComplexField::sqrt(self.x.pow(2) + self.y.pow(2) + self.z.pow(2))
    }

    pub fn rotate_by(&self, other: &Rotation3d) -> Self {
        let p = Quaternion::new(0.0, self.x, self.y, self.z);
        let qprime = other.q.times(p).times(other.q.inverse());
        Self::new_xyz(qprime.i, qprime.j, qprime.k)
    }

    pub fn plus(&self, other: &Self) -> Self {
        Self::new_xyz(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn minus(&self, other: &Self) -> Self {
        Self::new_xyz(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn unary_minus(&self) -> Self {
        Self::new_xyz(-self.x, -self.y, -self.z)
    }

    pub fn times(&self, scalar: f64) -> Self {
        Self::new_xyz(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    pub fn interpolate (end_value: Translation3d, t: f64) -> Self {
        Self::new_xyz(
            MathUtil::interpolate(self.x, end_value.x, t),
            MathUtil::interpolate(self.y, end_value.y, t),
            MathUtil::interpolate(self.z, end_value.z, t),
        )
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