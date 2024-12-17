pub mod dta;
pub mod mng;
pub mod s16;
pub mod sfc;

use bevy::prelude::*;
use dta::{DTAAssetLoader, DTAFile};
use mng::{MNGAssetLoader, MNGFile};
use s16::{S16AssetLoader, S16Image};

use crate::camera::main_camera::MainCamera;
use crate::state::GameState;

pub struct GameFormatsPlugin;

#[derive(Resource, Default)]
struct PreloadedAssets(Vec<Handle<S16Image>>);

#[derive(Resource, Default)]
struct PreloadedSounds(Vec<Handle<MNGFile>>);

impl Plugin for GameFormatsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<S16AssetLoader>();
        app.init_asset::<S16Image>();
        app.init_asset_loader::<MNGAssetLoader>();
        app.init_asset::<MNGFile>();
        app.init_asset_loader::<DTAAssetLoader>();
        app.init_asset::<DTAFile>();

        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                check_sprite_loading.run_if(in_state(GameState::Loading)),
                keyboard_scrolling.run_if(in_state(GameState::Running)),
            ),
        );

        app.insert_resource(ScrollSpeed { speed: 500.0 });
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PreloadedAssets(vec![
        asset_server.load("Images/syst.s16".to_string()),
        asset_server.load("Images/Back.s16".to_string()),
        asset_server.load("Images/bubb.s16".to_string()),
    ]));
    commands.insert_resource(PreloadedSounds(vec![
        asset_server.load("Sounds/Music.mng".to_string())
    ]));
}

fn check_sprite_loading(
    mut next_state: ResMut<NextState<GameState>>,
    sprite_folder: Res<PreloadedAssets>,
    sound_folder: Res<PreloadedSounds>,
    mut events_sprites: EventReader<AssetEvent<S16Image>>,
    mut events_sounds: EventReader<AssetEvent<MNGFile>>,
    mut sprites_loaded: Local<usize>,
    mut sounds_loaded: Local<usize>,
) {
    let sprite_asset_ids = sprite_folder.0.iter().map(|h| h.id()).collect::<Vec<_>>();
    let sound_asset_ids = sound_folder.0.iter().map(|h| h.id()).collect::<Vec<_>>();

    for event in events_sprites.read() {
        for asset_id in sprite_asset_ids.clone() {
            if event.is_loaded_with_dependencies(asset_id) {
                println!("sprite loaded: {:?}", asset_id);
                *sprites_loaded += 1;
            }
        }
    }

    for event in events_sounds.read() {
        for asset_id in sound_asset_ids.clone() {
            if event.is_loaded_with_dependencies(asset_id) {
                println!("sound loaded: {:?}", asset_id);
                *sounds_loaded += 1;
            }
        }
    }

    if *sprites_loaded >= sprite_folder.0.len() - 1 && *sounds_loaded >= sound_folder.0.len() - 1 {
        println!("sprites and sounds loaded");

        next_state.set(GameState::Running);
    } else {
        println!(
            "sprites and sounds not loaded: {} {} {} {}",
            *sprites_loaded,
            *sounds_loaded,
            sprite_folder.0.len(),
            sound_folder.0.len()
        );
    }
}

#[derive(Resource, Default, Reflect)]
pub struct ScrollSpeed {
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
