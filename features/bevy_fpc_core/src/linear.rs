//! Linear movement related module

use crate::{
    config::{FpcConfiguration, KeyboardLinearInputs},
    Player,
};
use bevy::prelude::*;
use bevy_rapier3d::{prelude::*};

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

#[derive(Component)]
pub struct Gravity(pub f32);

/// Enable linear movements from player inputs.
///
/// Keyboard inputs are configurable with `FpcConfiguration.keyboard_linear_inputs`.
/// Internally using _Rapier_ `KinematicCharacterController`.
pub fn handle_movements(
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &mut RigidBody,
            &Transform,
            &WalkSpeed,
            &mut Gravity,
            &KinematicCharacterControllerOutput,
        ),
        With<Player>,
    >,
    fpc_conf: Res<FpcConfiguration>,
    inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(
        |(mut controller, _ridgidbody, transform, walk_speed, mut gravity, controller_output)| {
            /* if !*ridgidbody.is_ccd_enabled(){
                *ridgidbody.enable_ccd(true);
            } */

            let KeyboardLinearInputs {
                forward,
                right,
                back,
                left,
            } = fpc_conf.keyboard_linear_inputs;
            let mut linear_velocity = Vec3::ZERO;
            if inputs.pressed(forward) {
                linear_velocity += transform.forward().normalize();
            }
            if inputs.pressed(back) {
                linear_velocity += transform.back().normalize();
            }
            if inputs.pressed(right) {
                linear_velocity += transform.right().normalize();
            }
            if inputs.pressed(left) {
                linear_velocity += transform.left().normalize();
            }

            if linear_velocity.length() > 0. {
                linear_velocity = linear_velocity.normalize();
                linear_velocity *= time.delta().as_secs_f32();
                linear_velocity *= walk_speed.0;
            }

            if controller_output.grounded {
                *gravity = Gravity(0.0);
            }

            if inputs.just_pressed(KeyCode::Space) {
                *gravity = Gravity(-10.0 * time.delta().as_secs_f32());
            }

            gravity.0 += (4.9_f32 * time.delta().as_secs_f32()).powf(2.0).clamp(0.0, 50.0);

            linear_velocity -= Vec3::new(0.0, gravity.0, 0.0);

            controller.translation = Some(linear_velocity);
        },
    );
}
