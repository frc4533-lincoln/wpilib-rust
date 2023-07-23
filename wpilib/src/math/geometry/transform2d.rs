use super::{Pose2d, Rotation2d, Translation2d};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform2d {
    pub translation: Translation2d,
    pub rotation: Rotation2d,
}

impl Transform2d {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translation: Translation2d::new(),
            rotation: Rotation2d::new(),
        }
    }

    #[must_use]
    pub fn new_pose_pose(initial: Pose2d, last: Pose2d) -> Self {
        let translation = last
            .translation
            .minus(&initial.translation)
            .rotate_by(&initial.rotation.unary_minus());
        let rotation = last.rotation.minus(&initial.rotation);
        Self {
            translation,
            rotation,
        }
    }

    #[must_use]
    pub const fn new_trans_rot(translation: Translation2d, rotation: Rotation2d) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new_trans_rot(self.translation.times(scalar), self.rotation.times(scalar))
    }

    #[must_use]
    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        Self::new_pose_pose(
            Pose2d::new(),
            Pose2d::new().transform_by(*self).transform_by(*other),
        )
    }

    #[must_use]
    pub fn inverse(&self) -> Self {
        Self::new_trans_rot(
            self.translation
                .unary_minus()
                .rotate_by(&self.rotation.unary_minus()),
            self.rotation.unary_minus(),
        )
    }
}

impl Default for Transform2d {
    fn default() -> Self {
        Self::new()
    }
}
