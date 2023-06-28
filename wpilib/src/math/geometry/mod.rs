use super::units::Meter;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation2d {
    x: Meter,
    y: Meter,
}
impl Translation2d {
    pub fn new(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}