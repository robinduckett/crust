pub mod tileset;

use crate::state::GameState;
use bevy::prelude::*;
use tileset::BackgroundTiles;

pub struct GameDisplayPlugin;

impl Plugin for GameDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            tileset::render_background_wrapped.run_if(in_state(GameState::Running)),
        );
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("BackgroundTiles"),
        BackgroundTiles,
        Transform::default(),
        Visibility::default(),
    ));
}

#[derive(Component)]
pub struct CreaturesViewport {
    pub rect: Rect,
}

pub fn get_viewport_rect(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    ortho: &OrthographicProjection,
) -> Option<Rect> {
    // Get the viewport size
    let viewport_size = match camera.logical_viewport_size() {
        Some(size) => size,
        None => return None,
    };

    // let position = camera_transform.translation().xy();

    // let position = position - Vec2::new(0.0, position.y * 2.0);

    // let orthoganal_zoom = ortho.scale;

    // let viewport_size = viewport_size * orthoganal_zoom;
    let half_viewport = viewport_size / 2.0;

    let viewport_position = camera
        .viewport_to_world_2d(camera_transform, half_viewport)
        .unwrap();

    let left = (viewport_position.x - half_viewport.x).floor();
    let top = (viewport_position.y + half_viewport.y).floor();
    let right = (viewport_position.x + half_viewport.x).ceil();
    let bottom = (viewport_position.y - half_viewport.y).ceil();

    Some(Rect::new(left, top, right, bottom))
}
