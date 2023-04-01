#![doc = include_str!("../README.md")]

use bevy::prelude::*;
use bevy_fpc_core::{
    angular::{apply_motion, free_cursor, handle_request, lock_cursor, VisionMotionTarget},
    incarnation::embody,
    linear::handle_movements,
};
use bevy_rapier3d::prelude::*;

// API
pub use bevy_fpc_core::{
    angular::AngularState,
    config::{FpcConfiguration, KeyboardLinearInputs, LINEAR_AZERTY_LAYOUT, LINEAR_QWERTY_LAYOUT},
    linear::WalkSpeed,
    Player,
};
#[cfg(feature = "bevy_fpc_sprint")]
pub use bevy_fpc_sprint::{FpcSprintConfiguration, SprintRate};

/// The `bevy_fpc` structure implementing the bevy `Plugin` trait.
/// This plugin require the `bevy_rapier3d` plugin.
/// ```
/// App::new()
/// .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
/// .add_plugin(FpcPlugin::default())
/// ```
pub struct FpcPlugin;
impl Plugin for FpcPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<FpcConfiguration>()
            .add_state::<AngularState>()
            .add_systems(
                (
                    handle_request.in_set(OnUpdate(AngularState::Enabled)),
                    apply_motion,
                )
                    .chain(),
            )
            .add_system(lock_cursor.in_schedule(OnEnter(AngularState::Enabled)))
            .add_system(free_cursor.in_schedule(OnExit(AngularState::Enabled)))
            .add_systems((handle_movements, embody));

        #[cfg(feature = "bevy_fpc_sprint")]
        app.init_resource::<bevy_fpc_sprint::FpcSprintConfiguration>()
            .add_system(bevy_fpc_sprint::handle_sprint);
    }
}

/// Fpc marker
#[derive(Component, Default)]
struct Fpc;

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
