pub mod s16;
pub mod sfc;

use bevy::{asset::LoadedFolder, prelude::*};
use s16::{S16AssetLoader, S16Image};

use crate::camera::main_camera::MainCamera;
use crate::state::GameState;

pub struct GameFormatsPlugin;

#[derive(Resource, Default)]
struct SpriteFolder(Handle<LoadedFolder>);

impl Plugin for GameFormatsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<S16AssetLoader>();
        app.init_asset::<S16Image>();

        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                check_sprite_loading.run_if(in_state(GameState::Loading)),
                keyboard_scrolling.run_if(in_state(GameState::Running)),
            ),
        );

        app.insert_resource(ScrollSpeed { speed: 200.0 });
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SpriteFolder(asset_server.load_folder("sprites")));
}

fn check_sprite_loading(
    mut next_state: ResMut<NextState<GameState>>,
    sprite_folder: Res<SpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sprite_folder.0) {
            next_state.set(GameState::Running);
        }
    }
}

#[derive(Resource, Default)]
struct ScrollSpeed {
    speed: f32,
}

fn keyboard_scrolling(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection, &Camera), With<MainCamera>>,
    scroll_speed: Res<ScrollSpeed>,
) {
    for (mut transform, mut ortho, _) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale = ortho.scale.lerp(ortho.scale + 2.0, time.delta_secs());
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale = ortho.scale.lerp(ortho.scale - 2.0, time.delta_secs());
        }

        // ortho.scale = ortho.scale.clamp(0.80, 2.0);

        direction *= ortho.scale;

        let z = transform.translation.z;
        transform.translation += time.delta_secs() * direction * scroll_speed.speed;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;

        transform.translation = Vec3 {
            x: transform.translation.x.round(),
            y: transform.translation.y.round(),
            z: transform.translation.z.round(),
        };
    }
}
