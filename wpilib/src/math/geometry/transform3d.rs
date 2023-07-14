use super::{Pose3d, Rotation3d, Translation3d};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform3d {
    pub translation: Translation3d,
    pub rotation: Rotation3d,
}

impl Transform3d {
    pub fn new() -> Self {
        Self {
            translation: Translation3d::new(),
            rotation: Rotation3d::new(),
        }
    }

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

    pub fn new_trans_rot(translation: Translation3d, rotation: Rotation3d) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn times(&self, scalar: f64) -> Self {
        Transform3d::new_trans_rot(self.translation.times(scalar), self.rotation.times(scalar))
    }

    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    pub fn plus(&self, other: &Self) -> Self {
        Self::new_pose_pose(
            Pose3d::new(),
            Pose3d::new().transform_by(*self).transform_by(*other),
        )
    }

    pub fn inverse(&self) -> Self {
        Self::new_trans_rot(
            self.translation
                .unary_minus()
                .rotate_by(&self.rotation.unary_minus()),
            self.rotation.unary_minus(),
        )
    }
}
