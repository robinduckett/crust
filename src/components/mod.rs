use bevy::prelude::*;
pub use debug::{add_debug_text_bg, update_debug_text, CreaturesGizmos, DebugText};

pub mod debug;
pub mod utils;

pub mod room;

use crate::components::room::RoomPlugin;

pub struct GameComponentsPlugin;

impl Plugin for GameComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_debug_text, add_debug_text_bg).chain());

        app.add_plugins((RoomPlugin,));
    }
}
