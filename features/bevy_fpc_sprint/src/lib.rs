//! Sprint feature crate for `bevy_fpc`
//!
//! Give the ability to move faster on forward movement

use bevy::prelude::*;
use bevy_fpc_core::Player;
use bevy_rapier3d::prelude::KinematicCharacterController;

/// Default value for the `SprintRate` component
const DEFAULT_SPRINT_RATE: f32 = 2.;

#[derive(Resource, Copy, Clone, Debug)]
/// A resource for specifying configuration information about `bevy_fpc` sprinting feature.
pub struct FpcSprintConfiguration {
    /// Keyboard input for toggling sprint.
    ///
    /// Current default is `LeftShift`
    pub keyboard_sprint_input: KeyCode,
}
impl Default for FpcSprintConfiguration {
    fn default() -> Self {
        Self {
            keyboard_sprint_input: KeyCode::LShift,
        }
    }
}

/// Component used to mutate speed of controller horizontal translations when sprinting.
/// Contain one value by which the `WalkSpeed` will be multiplied.
///
/// Default value is `DEFAULT_SPRINT_RATE`.
#[derive(Component)]
pub struct SprintRate(pub f32);
impl Default for SprintRate {
    fn default() -> Self {
        Self(DEFAULT_SPRINT_RATE)
    }
}

/// Add sprinting value to entity forward translation when the associated input is pressed
///
/// Keyboard input is configurable with `FpcSprintConfiguration.keyboard_sprint_input`.
/// Internally using _Rapier_ `KinematicCharacterController`.
pub fn handle_sprint(
    mut query: Query<(&mut KinematicCharacterController, &Transform, &SprintRate), With<Player>>,
    inputs: Res<Input<KeyCode>>,
    fpc_conf: Res<FpcSprintConfiguration>,
) {
    query.for_each_mut(|(mut controller, transform, SprintRate(rate))| {
        if let Some(translation) = controller.translation {
            if inputs.pressed(fpc_conf.keyboard_sprint_input) {
                let dot_forward = Vec2::new(transform.local_z().x, transform.local_z().z)
                    .dot(Vec2::new(translation.x, translation.z));

                // detect forward movement
                if dot_forward < -0.001 && dot_forward > -0.1 {
                    controller.translation = Some(translation * *rate);
                }
            }
        }
    });
}
