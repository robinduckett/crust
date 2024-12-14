mod camera;
mod components;
mod constants;
mod display;
mod formats;
mod state;
mod time;
mod window;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::GameCameraPlugin;
use components::GameComponentsPlugin;
use display::GameDisplayPlugin;
use formats::GameFormatsPlugin;
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
            GameFormatsPlugin,
            GameCameraPlugin,
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::KeyI)),
        )
        .run();
}
