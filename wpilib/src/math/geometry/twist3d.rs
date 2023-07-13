use crate::math::units::{angle::Radian, distance::Meter};

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
    pub fn new() -> Self {
        Self {
            dx: 0.0.into(),
            dy: 0.0.into(),
            dz: 0.0.into(),
            rx: 0.0.into(),
            ry: 0.0.into(),
            rz: 0.0.into(),
        }
    }

    pub fn new_dv(
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
