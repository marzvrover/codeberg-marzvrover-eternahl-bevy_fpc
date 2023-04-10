use bevy::{prelude::*, DefaultPlugins};
use bevy_fpc::{AngularState, FpcBundle};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(bevy_fpc::FpcPlugin)
        .add_startup_system(init)
        .add_system(angular_state_switching)
        .run()
}

fn init(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn(SpatialBundle::default())
        .insert(Collider::cuboid(100., 0., 100.))
        .with_children(|builder| {
            builder.spawn(SceneBundle {
                scene: assets_server.load("temple_ruins-deyama.glb#Scene0"),
                transform: Transform::from_xyz(0., 0., -5.),
                ..Default::default()
            });
        });
    commands.insert_resource(AmbientLight {
        brightness: 0.8,
        ..Default::default()
    });
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 100000.0 / 2.,
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(1., 2., 0.),
            rotation: Quat::from_rotation_x(-PI / 2.),
            ..Default::default()
        });

    // Awaiting for issue #1325
    // commands.spawn(TextBundle::from_section(
    //     "Press `any key` to switch angular state",
    //     TextStyle::default(),
    // ));

    commands
        .spawn(FpcBundle::default())
        .insert(bevy_fpc::Player)
        .insert(TransformBundle::from(Transform::from_xyz(0., 1., 0.)));
}

/// Handle `AngularState` switching by input
fn angular_state_switching(
    inputs: Res<Input<KeyCode>>,
    state: Res<State<AngularState>>,
    mut next: ResMut<NextState<AngularState>>,
) {
    if inputs.just_pressed(KeyCode::Tab) {
        if state.0 == AngularState::Enabled {
            next.set(AngularState::Disabled);
        } else {
            next.set(AngularState::Enabled);
        }
    }
}
