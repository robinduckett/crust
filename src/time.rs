use bevy::{
    input::common_conditions::input_just_pressed, prelude::*,
    time::common_conditions::on_real_timer, window::PrimaryWindow,
};
use std::time::Duration;

use crate::state::GameState;

pub struct GameTimePlugin;

#[allow(dead_code)]
#[derive(Component)]
pub struct DebugGameTime;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_time);
        app.add_systems(
            Update,
            (
                update_time.run_if(on_real_timer(Duration::from_secs(1))),
                toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
            ),
        );
    }
}

fn setup_time(mut commands: Commands, mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed(1.0); // startup - set time to normal speed

    let command_line_args = std::env::args();

    if command_line_args.into_iter().any(|arg| arg == "--debug") {
        commands.spawn(DebugGameTime);
    }
}

fn update_time(
    real_time: Res<Time<Real>>,
    virtual_time: Res<Time<Virtual>>,
    debug: Query<(), With<DebugGameTime>>,
) {
    if !debug.is_empty() {
        println!(
            "time: {}, paused: {}, virtual_time: {}",
            real_time.elapsed_secs(),
            virtual_time.is_paused(),
            virtual_time.elapsed_secs()
        );
    }
}

fn toggle_pause(
    mut time: ResMut<Time<Virtual>>,
    mut state: ResMut<NextState<GameState>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if time.is_paused() {
        time.unpause();
        state.set(GameState::Running);
        window.title = "CL".into();
    } else {
        time.pause();
        state.set(GameState::Paused);
        window.title = "CL - Paused".into();
    }
}
