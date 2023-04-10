//! Linear movement related module

use crate::{
    config::{FpcConfiguration, KeyboardLinearInputs},
    Player,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Default value for the `WalkSpeed` component
const DEFAULT_WALK_SPEED: f32 = 2.;

/// Component used to mutate speed of controller horizontal translations when walking.
/// Contain one value by which the linear velocity will be multiplied.
///
/// Default value is `DEFAULT_WALK_SPEED`.
#[derive(Component)]
pub struct WalkSpeed(pub f32);
impl Default for WalkSpeed {
    fn default() -> Self {
        Self(DEFAULT_WALK_SPEED)
    }
}

/// Enable linear movements from player inputs.
///
/// Keyboard inputs are configurable with `FpcConfiguration.keyboard_linear_inputs`.
/// Internally using _Rapier_ `KinematicCharacterController`.
pub fn handle_movements(
    mut query: Query<(&mut KinematicCharacterController, &Transform, &WalkSpeed), With<Player>>,
    fpc_conf: Res<FpcConfiguration>,
    inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    query.for_each_mut(|(mut controller, transform, walk_speed)| {
        let KeyboardLinearInputs {
            forward,
            right,
            back,
            left,
        } = fpc_conf.keyboard_linear_inputs;
        let mut linear_velocity = Vec3::ZERO;
        if inputs.pressed(forward) {
            linear_velocity += transform.forward();
        }
        if inputs.pressed(back) {
            linear_velocity += transform.back();
        }
        if inputs.pressed(right) {
            linear_velocity += transform.right();
        }
        if inputs.pressed(left) {
            linear_velocity += transform.left();
        }

        if linear_velocity.length() > 0. {
            linear_velocity = linear_velocity.normalize();
            linear_velocity *= time.delta_seconds();
            linear_velocity *= walk_speed.0;

            controller.translation = Some(linear_velocity);
        }
    });
}
