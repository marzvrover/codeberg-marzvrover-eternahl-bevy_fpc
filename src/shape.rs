use crate::{angular::VisionMotionTarget, linear::WalkSpeed, Fpc};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Bundle containing all neccessary components for the base of a first person controller entity.
#[derive(Bundle)]
pub struct FpcBundle {
    fpc: Fpc,
    body: RigidBody,
    collider: Collider,
    controller: KinematicCharacterController,
    vmt: VisionMotionTarget,
    walk_speed: WalkSpeed,
    #[bundle]
    spatial: SpatialBundle,
    #[cfg(feature = "bevy_fpc_sprint")]
    sprint_rate: bevy_fpc_sprint::SprintRate,
}
impl Default for FpcBundle {
    fn default() -> Self {
        Self {
            fpc: Fpc,
            body: RigidBody::KinematicPositionBased,
            collider: Collider::capsule_y(0.5, 0.5),
            controller: KinematicCharacterController::default(),
            spatial: SpatialBundle::default(),
            vmt: VisionMotionTarget::default(),
            walk_speed: WalkSpeed::default(),
            #[cfg(feature = "bevy_fpc_sprint")]
            sprint_rate: bevy_fpc_sprint::SprintRate::default(),
        }
    }
}
