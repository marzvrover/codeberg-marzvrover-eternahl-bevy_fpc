//! Angular movement related module

use crate::FpcConfiguration;
use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_fpc_common::Player;
use bevy_rapier3d::na::clamp;

/// State used to eithers contains the mouse cursor while allowing angular movements
/// or let the mouse cursor free while preventing angular movements.
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AngularState {
    #[default]
    Enabled,
    Disabled,
}

/// Internal component used to achieve angular interpolation.
#[derive(Component, Default, Debug)]
pub(crate) struct VisionMotionTarget {
    /// Horizontal rotation target
    pub horizontal: f32,
    /// Vertical rotation target
    pub vertical: f32,
}

/// Process angular inputs and mutate rotation targets for next transformation.
pub(crate) fn handle_request(
    mut query: Query<(&Children, &mut VisionMotionTarget, &Transform), With<Player>>,
    mut children_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut mouse_evr: EventReader<MouseMotion>,
    fpc_conf: Res<FpcConfiguration>,
) {
    // update vision motion target
    for ev in mouse_evr.iter() {
        query.for_each_mut(|(children, mut vmt, transform)| {
            let cursor_ratio = 0.001 * fpc_conf.angular_sensitivity;

            // calculate horizontal angle target
            let mut horizontal_transform = transform.clone();
            horizontal_transform.rotation = Quat::from_rotation_y(vmt.horizontal);
            horizontal_transform.rotate_y(-ev.delta.x * cursor_ratio);

            // write horizontal angle target
            let Vec3 { y, .. } = horizontal_transform.rotation.to_scaled_axis();
            vmt.horizontal = y;

            // calculate vertical angle target
            for &child in children.iter() {
                // apply delta
                let mut vertical_transform = children_query.get_mut(child).unwrap().clone();
                vertical_transform.rotation = Quat::from_rotation_x(vmt.vertical);
                vertical_transform.rotate_x(-ev.delta.y * cursor_ratio);

                // clamping
                let Vec3 { x, .. } = vertical_transform.rotation.to_scaled_axis();
                let clamped_angle = clamp(x, -fpc_conf.angular_clamping, fpc_conf.angular_clamping);
                vertical_transform.rotation = Quat::from_rotation_x(clamped_angle);

                // write vertical angle target
                let Vec3 { x, .. } = vertical_transform.rotation.to_scaled_axis();
                vmt.vertical = x;
            }
        });
    }
}

/// Apply processed angular motion target.
pub(crate) fn apply_motion(
    mut query: Query<(&Children, &mut Transform, &VisionMotionTarget), With<Player>>,
    mut children_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    time: Res<Time>,
    fpc_conf: Res<FpcConfiguration>,
) {
    // interpolation speed
    let interpolation = time.delta_seconds() * 20. / fpc_conf.angular_smoothing;
    query.for_each_mut(
        |(
            children,
            mut transform,
            VisionMotionTarget {
                horizontal,
                vertical,
            },
        )| {
            // apply entity horizontal rotation
            transform.rotation = transform
                .rotation
                .slerp(Quat::from_rotation_y(*horizontal), interpolation);

            for &child in children.iter() {
                // apply camera vertical rotation
                let mut camera_transform_target = children_query.get_mut(child).unwrap();
                camera_transform_target.rotation = camera_transform_target
                    .rotation
                    .slerp(Quat::from_rotation_x(*vertical), interpolation);
            }
        },
    );
}

/// Configure mouse cursor grab mode to `Locked` and hide the cursor.
pub(crate) fn lock_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

/// Configure mouse cursor grab mode to `None` ans show the cursor.
pub(crate) fn free_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
