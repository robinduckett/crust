use super::main_camera::{mouse_pos_to_world, MainCamera};
use bevy::prelude::*;

#[derive(Component)]
pub struct CameraTarget {
    pub target: Vec2,
}

#[derive(Component)]
pub struct CreatureEyeView;

pub fn toggle_creatures_eye_view(mut camera_target_query: Query<&mut Node, With<CreatureEyeView>>) {
    for node in camera_target_query.iter_mut() {
        let mut node = node;
        node.display = if node.display == Display::None {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn select_new_eye_view(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut target_query: Query<&mut CameraTarget, With<CreatureEyeView>>,
    window_query: Query<&Window>,
) {
    let value = mouse_pos_to_world(camera_query, window_query);

    for mut camera_target in target_query.iter_mut() {
        camera_target.target = value;
    }
}

pub fn move_creature_eye_view(
    mut camera_target_query: Query<(&mut Transform, &CameraTarget), With<CreatureEyeView>>,
) {
    for (mut transform, camera_target) in camera_target_query.iter_mut() {
        let new_translation = transform.translation.lerp(
            Vec3::new(camera_target.target.x, camera_target.target.y, 0.0),
            0.1,
        );

        transform.translation = new_translation;
    }
}
