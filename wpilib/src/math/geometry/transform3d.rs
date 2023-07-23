use super::{Pose3d, Rotation3d, Transform2d, Translation3d};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform3d {
    pub translation: Translation3d,
    pub rotation: Rotation3d,
}

impl Transform3d {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translation: Translation3d::new(),
            rotation: Rotation3d::new(),
        }
    }

    #[must_use]
    pub fn new_pose_pose(initial: Pose3d, last: Pose3d) -> Self {
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
    pub const fn new_trans_rot(translation: Translation3d, rotation: Rotation3d) -> Self {
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
            Pose3d::new(),
            Pose3d::new().transform_by(*self).transform_by(*other),
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

impl Default for Transform3d {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Transform2d> for Transform3d {
    fn from(transform: Transform2d) -> Self {
        Self::new_trans_rot(transform.translation.into(), transform.rotation.into())
    }
}

impl From<Transform3d> for Transform2d {
    fn from(transform: Transform3d) -> Self {
        Self::new_trans_rot(transform.translation.into(), transform.rotation.into())
    }
}
