use bevy::{prelude::*, DefaultPlugins};
use bevy_fpc::{AngularState, FpcBundle};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(bevy_fpc::FpcPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, angular_state_switching)
        .run();
}

fn init(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // sky color
    commands.insert_resource(ClearColor(Color::hsl(203., 0.51, 0.51)));

    // ground
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0))),
            material: materials.add(Color::srgb(0.8, 0.655, 0.317)),
            ..Default::default()
        })
        .insert(Collider::cuboid(100., 0., 100.));

    // map model
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(0., 0., -8.),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(SceneBundle {
                scene: assets_server.load("temple_ruins-deyama.glb#Scene0"),
                ..Default::default()
            });

            builder
                .spawn(Collider::compound(vec![
                    // floor colliders
                    (Vec3::ZERO, Quat::default(), Collider::cuboid(2., 0.3, 2.)),
                    (Vec3::ZERO, Quat::default(), Collider::cuboid(1.7, 0.6, 1.7)),
                    // stair collider
                    (
                        Vec3::new(0., 0., 2.0),
                        Quat::default(),
                        Collider::cuboid(0.5, 0.15, 0.15),
                    ),
                    (
                        Vec3::new(0., 0.3, 1.7),
                        Quat::default(),
                        Collider::cuboid(0.5, 0.15, 0.15),
                    ),
                ]))
                .insert(TransformBundle::default());
        });

    // lights
    commands.insert_resource(AmbientLight {
        brightness: 0.8,
        ..Default::default()
    });
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 100000.0 / 2.,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(1., 2., 0.),
            rotation: Quat::from_rotation_x(-PI / 2.),
            ..Default::default()
        });

    commands.spawn(TextBundle::from_section(
        "Press `tab` key to switch angular state",
        TextStyle::default(),
    ));

    commands
        .spawn(FpcBundle::default())
        .insert(bevy_fpc::Player)
        .insert(TransformBundle::from(Transform::from_xyz(0., 0.75, 0.)));
}

/// Handle `AngularState` switching by input
fn angular_state_switching(
    inputs: Res<ButtonInput<KeyCode>>,
    state: Res<State<AngularState>>,
    mut next: ResMut<NextState<AngularState>>,
) {
    if inputs.just_pressed(KeyCode::Tab) {
        if state.eq(&AngularState::Enabled) {
            next.set(AngularState::Disabled);
        } else {
            next.set(AngularState::Enabled);
        }
    }
}
