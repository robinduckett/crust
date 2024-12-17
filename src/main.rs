mod camera;
mod components;
mod constants;
mod display;
mod formats;
mod inspector;
mod music;
mod state;
mod time;
mod window;

use bevy::prelude::*;

use camera::GameCameraPlugin;
use components::GameComponentsPlugin;
use display::GameDisplayPlugin;
use formats::GameFormatsPlugin;
use inspector::GameInspectorPlugin;
use music::GameMusicPlugin;
use state::GameStatePlugin;
use time::GameTimePlugin;
use window::GameWindowPlugin;

fn main() {
    App::new()
        .add_plugins((
            GameTimePlugin,
            GameDisplayPlugin,
            GameWindowPlugin,
            GameStatePlugin,
            GameComponentsPlugin,
            GameInspectorPlugin,
            GameMusicPlugin,
            GameFormatsPlugin,
            GameCameraPlugin,
        ))
        .run();
}
