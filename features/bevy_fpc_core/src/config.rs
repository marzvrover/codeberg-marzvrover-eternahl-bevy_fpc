//! Configuration related module

use bevy::prelude::{KeyCode, Resource};
use std::f32::consts::PI;

/// Linear movement inputs for a `qwerty` keyboard layout (WASD)
pub const LINEAR_QWERTY_LAYOUT: KeyboardLinearInputs = KeyboardLinearInputs {
    forward: KeyCode::KeyW,
    back: KeyCode::KeyS,
    right: KeyCode::KeyD,
    left: KeyCode::KeyA,
};

/// Linear movement inputs for a `azerty` keyboard layout (ZQSD)
pub const LINEAR_AZERTY_LAYOUT: KeyboardLinearInputs = KeyboardLinearInputs {
    forward: KeyCode::KeyZ,
    back: KeyCode::KeyS,
    right: KeyCode::KeyD,
    left: KeyCode::KeyQ,
};

/// Structure for the keyboard linear inputs configuration
#[derive(Clone, Copy, Debug)]
pub struct KeyboardLinearInputs {
    pub forward: KeyCode,
    pub back: KeyCode,
    pub right: KeyCode,
    pub left: KeyCode,
}

#[derive(Resource, Copy, Clone, Debug)]
/// A resource for specifying configuration information about first person controllers.
pub struct FpcConfiguration {
    /// Keyboard inputs for fpc linear movements (forward, back, right, left).
    ///
    /// Current default is `WSDA`
    pub keyboard_linear_inputs: KeyboardLinearInputs,
    /// Clamping angle value for angular movements.
    ///
    /// Default is `PI/2`, this correspond to a range of 180°.
    pub angular_clamping: f32,
    /// Angular movements sensitivity.
    ///
    /// Default is `1.`.
    pub angular_sensitivity: f32,
    /// Angular movements smoothing.
    ///
    /// Default is `1.`.
    pub angular_smoothing: f32,
}
impl Default for FpcConfiguration {
    fn default() -> Self {
        Self {
            keyboard_linear_inputs: LINEAR_QWERTY_LAYOUT,
            angular_clamping: PI / 2.,
            angular_sensitivity: 1.,
            angular_smoothing: 1.,
        }
    }
}
