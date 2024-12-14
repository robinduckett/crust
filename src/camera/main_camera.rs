use bevy::prelude::*;
#[derive(Component)]
pub struct MainCamera;

pub fn mouse_pos_to_world(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window>,
) -> Vec2 {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = window_query.single().cursor_position() else {
        return Vec2::ZERO;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return Vec2::ZERO;
    };

    world_position
}
