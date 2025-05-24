//! Module related to player control of first person controller

use crate::Player;
use bevy::prelude::*;

/// Give vision to a player from the first person controller point of view.
pub fn embody(mut commands: Commands, query: Query<Entity, Added<Player>>) {
    query.iter().for_each(|entity| {
        if let Ok(mut ec) = commands.get_entity(entity) {
            ec.with_children(|builder| {
                builder.spawn(Camera3d::default());
            });
        }
    });
}
