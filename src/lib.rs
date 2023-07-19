//! First-person-controller plugin for the [Bevy](https://bevyengine.org) game-engine.
//!
//! The controller benefits from the features offered by the [`rapier character controller`](https://rapier.rs/docs/user_guides/bevy_plugin/character_controller).
//!
//! Plugins initialization:
//!
//! ```rust
//! // Require the `bevy_rapier3d` crate
//! # use bevy_fpc::FpcPlugin;
//! # use bevy::prelude::*;
//! # use bevy_rapier3d::prelude::*;
//! App::new()
//! 	.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
//! 	.add_plugin(FpcPlugin);
//! ```
//!
//! Spawn and embody an `fpc` entity:
//!
//! ```rust
//! # use bevy_fpc::{FpcBundle, Player};
//! # use bevy::prelude::{Commands};
//! # fn init(mut commands: Commands) {
//! commands.spawn(FpcBundle::default()).insert(Player);
//! # }
//! ```
//!
//! Custom configuration:
//!
//! ```rust
//! # use bevy_fpc::{FpcConfiguration, LINEAR_AZERTY_LAYOUT};
//! # use bevy::prelude::{App, Commands};
//! # let mut app = App::new();
//! app.insert_resource(FpcConfiguration{
//!   keyboard_linear_inputs: LINEAR_AZERTY_LAYOUT,
//!   ..Default::default()
//! });
//! ```
//!
//! # Features
//!
//! - [`bevy_fpc`](https://docs.rs/bevy_fpc) main crate / API
//! - [`bevy_fpc_core`](https://docs.rs/bevy_fpc_core) add minimal features to the controller (walking and looking)
//! - [`bevy_fpc_sprint`](https://docs.rs/bevy_fpc_sprint) (default) add sprinting capability to the controller

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
/// # use bevy_fpc::FpcPlugin;
/// # use bevy::prelude::*;
/// # use bevy_rapier3d::prelude::*;
/// App::new()
/// .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
/// .add_plugin(FpcPlugin);
/// ```
pub struct FpcPlugin;
impl Plugin for FpcPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<FpcConfiguration>()
            .add_state::<AngularState>()
            .add_systems(
                Update,
                (
                    handle_request.run_if(in_state(AngularState::Enabled)),
                    apply_motion,
                )
                    .chain(),
            )
            .add_systems(OnEnter(AngularState::Enabled), lock_cursor)
            .add_systems(OnExit(AngularState::Enabled), free_cursor)
            .add_systems(Update, (handle_movements, embody));

        #[cfg(feature = "bevy_fpc_sprint")]
        app.init_resource::<bevy_fpc_sprint::FpcSprintConfiguration>()
            .add_systems(Update, bevy_fpc_sprint::handle_sprint);
    }
}

/// Fpc marker
#[derive(Component, Default)]
pub struct Fpc;

/// Bundle containing all neccessary components for the base of a first person controller entity.
#[derive(Bundle)]
pub struct FpcBundle {
    pub fpc: Fpc,
    pub body: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub vmt: VisionMotionTarget,
    pub walk_speed: WalkSpeed,
    pub spatial: SpatialBundle,
    #[cfg(feature = "bevy_fpc_sprint")]
    pub sprint_rate: bevy_fpc_sprint::SprintRate,
}
impl Default for FpcBundle {
    fn default() -> Self {
        Self {
            fpc: Fpc,
            body: RigidBody::KinematicPositionBased,
            collider: Collider::capsule_y(0.5, 0.25),
            controller: KinematicCharacterController::default(),
            spatial: SpatialBundle::default(),
            vmt: VisionMotionTarget::default(),
            walk_speed: WalkSpeed::default(),
            #[cfg(feature = "bevy_fpc_sprint")]
            sprint_rate: bevy_fpc_sprint::SprintRate::default(),
        }
    }
}
