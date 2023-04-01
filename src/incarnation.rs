//! Module related to player control of first person controller

use bevy::prelude::*;
use bevy_fpc_common::Player;

/// Give vision to a player from the first person controller point of view.
pub(crate) fn embody(mut commands: Commands, query: Query<Entity, Added<Player>>) {
    query.for_each(|entity| {
        if let Some(mut ec) = commands.get_entity(entity) {
            ec.with_children(|builder| {
                builder.spawn(Camera3dBundle::default());
            });
        }
    });
}
