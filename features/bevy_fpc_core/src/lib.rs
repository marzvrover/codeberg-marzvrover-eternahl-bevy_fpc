//! Core features for `bevy_fpc`

use bevy::prelude::*;

pub mod angular;
pub mod config;
pub mod incarnation;
pub mod linear;

/// Marker used for embodying the associated first person controller.
#[derive(Component, Default)]
pub struct Player;
