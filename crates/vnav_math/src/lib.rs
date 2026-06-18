//! Core robotics math utilities for VNAV Rust.
//!
//! This crate starts with SE(3)-style rigid transform utilities used in Lab 2.

use nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3};

/// A rigid transform in 3D: rotation + translation.
pub type Transform3 = Isometry3<f64>;

/// Construct a transform from xyz translation and roll-pitch-yaw rotation.
pub fn from_xyz_rpy(xyz: [f64; 3], rpy: [f64; 3]) -> Transform3 {
    let translation = Translation3::from(Vector3::new(xyz[0], xyz[1], xyz[2]));
    let rotation = UnitQuaternion::from_euler_angles(rpy[0], rpy[1], rpy[2]);
    Isometry3::from_parts(translation, rotation)
}

/// Compute the transform from frame `a` to frame `b`, given world-frame poses.
pub fn relative_transform(world_from_a: &Transform3, world_from_b: &Transform3) -> Transform3 {
    world_from_a.inverse() * world_from_b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_relative_transform() {
        let pose = from_xyz_rpy([1.0, 2.0, 3.0], [0.1, 0.2, 0.3]);
        let rel = relative_transform(&pose, &pose);
        assert!(rel.translation.vector.norm() < 1e-9);
        assert!(rel.rotation.angle() < 1e-9);
    }
}
