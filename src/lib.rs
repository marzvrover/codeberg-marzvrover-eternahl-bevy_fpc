#![doc = include_str!("../README.md")]

use bevy::prelude::*;

mod angular;
mod config;
mod incarnation;
mod linear;
mod shape;

// API
pub use angular::AngularState;
pub use bevy_fpc_common::Player;
pub use config::{
    FpcConfiguration, KeyboardLinearInputs, LINEAR_AZERTY_LAYOUT, LINEAR_QWERTY_LAYOUT,
};
pub use shape::FpcBundle;

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
            .add_state::<angular::AngularState>()
            .add_systems(
                (
                    angular::handle_request.in_set(OnUpdate(angular::AngularState::Enabled)),
                    angular::apply_motion,
                )
                    .chain(),
            )
            .add_system(angular::lock_cursor.in_schedule(OnEnter(angular::AngularState::Enabled)))
            .add_system(angular::free_cursor.in_schedule(OnExit(angular::AngularState::Enabled)))
            .add_systems((linear::handle_movements, incarnation::embody));

        #[cfg(feature = "bevy_fpc_sprint")]
        app.init_resource::<bevy_fpc_sprint::FpcSprintConfiguration>()
            .add_system(bevy_fpc_sprint::handle_sprint);
    }
}

/// Fpc marker
#[derive(Component, Default)]
struct Fpc;
