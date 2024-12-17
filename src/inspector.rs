use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::common_conditions::{input_just_pressed, input_toggle_active};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::{
    bevy_inspector::hierarchy::SelectedEntities, DefaultInspectorConfigPlugin,
};

use crate::formats::ScrollSpeed;
use crate::music::MusicManager;
pub struct GameInspectorPlugin;

impl Plugin for GameInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(DefaultInspectorConfigPlugin);
        app.add_plugins(EguiPlugin);
        app.add_systems(
            Update,
            (
                stats_ui.run_if(input_toggle_active(true, KeyCode::Escape)),
                toggle_cursor.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        );
    }
}

pub fn toggle_cursor(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.cursor_options = CursorOptions {
        visible: !window.cursor_options.visible,
        grab_mode: CursorGrabMode::None,
        hit_test: true,
    };
}

pub fn stats_ui(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::SidePanel::left("hierarchy")
        .default_width(200.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Frame Info");

                let mut diagnostics_string = String::new();

                if let Some(diagnostics) = world.get_resource::<DiagnosticsStore>() {
                    for diagnostic in diagnostics.iter() {
                        diagnostics_string.push_str(&format!(
                            "{path}: {value:>.2}{suffix:}\n",
                            path = diagnostic.path(),
                            value = diagnostic.smoothed().unwrap_or(0.0),
                            suffix = diagnostic.suffix
                        ));
                    }
                }

                ui.label(diagnostics_string);

                ui.heading("Hierarchy");

                bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                    world,
                    ui,
                    &mut selected_entities,
                );

                ui.label("\n");

                ui.heading("Crust");

                egui::CollapsingHeader::new("MusicManager").show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_resource::<MusicManager>(world, ui);
                });

                egui::CollapsingHeader::new("ScrollSpeed").show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_resource::<ScrollSpeed>(world, ui);
                });

                ui.label("\nPress escape to toggle UI\n");
            });
        });

    egui::SidePanel::right("inspector")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Inspector");

                match selected_entities.as_slice() {
                    &[entity] => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                    }
                    entities => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                            world, entities, ui,
                        );
                    }
                }

                ui.allocate_space(ui.available_size());
            });
        });
}
