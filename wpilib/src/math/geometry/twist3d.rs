use crate::math::units::{angle::Radian, distance::Meter};

use super::Twist2d;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Twist3d {
    pub dx: Meter,
    pub dy: Meter,
    pub dz: Meter,
    pub rx: Radian,
    pub ry: Radian,
    pub rz: Radian,
}

impl Twist3d {
    pub fn new(
        dx: impl Into<Meter>,
        dy: impl Into<Meter>,
        dz: impl Into<Meter>,
        rx: impl Into<Radian>,
        ry: impl Into<Radian>,
        rz: impl Into<Radian>,
    ) -> Self {
        Self {
            dx: dx.into(),
            dy: dy.into(),
            dz: dz.into(),
            rx: rx.into(),
            ry: ry.into(),
            rz: rz.into(),
        }
    }
}

impl Default for Twist3d {
    fn default() -> Self {
        Self {
            dx: 0.0.into(),
            dy: 0.0.into(),
            dz: 0.0.into(),
            rx: 0.0.into(),
            ry: 0.0.into(),
            rz: 0.0.into(),
        }
    }
}

impl From<Twist2d> for Twist3d {
    fn from(twist: Twist2d) -> Self {
        Self::new(twist.dx, twist.dy, 0.0, 0.0, 0.0, twist.dtheta)
    }
}

impl From<Twist3d> for Twist2d {
    fn from(twist: Twist3d) -> Self {
        Self::new(twist.dx, twist.dy, twist.rz)
    }
}
