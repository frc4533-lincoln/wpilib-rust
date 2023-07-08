use nalgebra::Translation2;

use crate::math::units::distance::Meter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation2d {
    pub x: Meter,
    pub y: Meter,
}

impl Translation2d {
    pub fn new() -> Self{
        Self {
            x: 0.0.into(),
            y: 0.0.into(),
        }
    }
    pub fn new_xy(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
    // pub fn new_dist_angle(distance: f64, )
    // #[must_use] pub fn x(&self) -> Meter {
    //     self.inner.x
    // }
    // #[must_use] pub fn y(&self) -> Meter {
    //     self.inner.y
    // }
    // #[must_use] pub fn strength(&self) -> Meter {
    //     self.inner.vector.norm().into()
    // }
    // pub fn normalize(&mut self) {
    //     self.inner.vector.normalize_mut();
    // }
}
