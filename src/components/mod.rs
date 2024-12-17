use bevy::prelude::*;
pub use debug::{add_debug_text_bg, CreaturesGizmos, DebugText};

pub mod debug;
pub mod utils;

pub mod room;

use crate::components::room::RoomPlugin;

pub struct GameComponentsPlugin;

impl Plugin for GameComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_debug_text_bg);

        app.add_plugins((RoomPlugin,));
    }
}
