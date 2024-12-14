use bevy::{
    app::PluginGroupBuilder,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{EnabledButtons, PresentMode, PrimaryWindow, WindowResolution, WindowTheme},
};

use crate::state::GameState;

pub struct GameWindowPlugin;

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_plugins((game_window_plugins(), FrameTimeDiagnosticsPlugin));
        app.add_systems(OnEnter(GameState::Running), make_visible);
    }
}

fn make_visible(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in windows.iter_mut() {
        window.visible = true;
    }
}

fn game_window_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "crust".into(),
                name: Some("crust.app".into()),
                resolution: WindowResolution::new(1920.0, 1080.0),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: EnabledButtons {
                    maximize: true,
                    ..Default::default()
                },
                resizable: false,
                visible: true,
                ..default()
            }),
            close_when_requested: true,
            ..default()
        })
        .set(ImagePlugin::default_linear())
}
