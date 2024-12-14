use bevy::prelude::*;

pub struct GameStatePlugin;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    Loading,
    Running,
    Paused,
    Finished,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();

        app.add_systems(OnEnter(GameState::Finished), on_finished);
    }
}

fn on_finished(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}
