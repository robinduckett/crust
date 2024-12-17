use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    window::{
        CursorOptions, EnabledButtons, PresentMode, PrimaryWindow, WindowResolution, WindowTheme,
    },
};

use crate::state::GameState;

pub struct GameWindowPlugin;

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_plugins(game_window_plugins());
        app.add_systems(OnEnter(GameState::Running), make_visible);
        app.add_systems(PreUpdate, show_cursor.run_if(in_state(GameState::Running)));
    }
}

#[derive(Component)]
pub struct Cursor {
    pub image: Handle<Image>,
}

fn show_cursor(
    window: Single<&mut Window, With<PrimaryWindow>>,
    mut cursor: Query<(&Cursor, &mut Node, &mut ImageNode, &mut Visibility), With<Cursor>>,
) {
    let (cursor_entity, mut node, mut image, mut visibility) = cursor.single_mut();

    if let Some(mouse_pos) = window.cursor_position() {
        node.left = Val::Px(mouse_pos.x);
        node.top = Val::Px(mouse_pos.y);
    }

    if cursor_entity.image != image.image {
        image.image = cursor_entity.image.clone();
    }

    if !window.cursor_options.visible && *visibility == Visibility::Hidden {
        *visibility = Visibility::Visible;
    } else if window.cursor_options.visible && *visibility == Visibility::Visible {
        *visibility = Visibility::Hidden;
    }
}

fn make_visible(
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let cursor_image = asset_server.load("Images/syst.s16#0".to_string());

    let cursor = (
        Cursor {
            image: cursor_image.clone(),
        },
        Node {
            left: Val::Px(210.0),
            top: Val::Px(10.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ImageNode {
            image: cursor_image.clone(),
            ..default()
        },
        Visibility::Visible,
        InheritedVisibility::default(),
        Name::new("syst"),
    );

    commands.spawn(cursor);

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
                resolution: WindowResolution::new(1280.0, 720.0),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: EnabledButtons {
                    maximize: true,
                    ..Default::default()
                },
                cursor_options: CursorOptions {
                    visible: true,
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
