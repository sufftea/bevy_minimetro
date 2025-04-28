use bevy::{prelude::*, reflect::hash_error};

mod game;
mod style;
mod ui;
mod utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    Game,
    Results,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(AppState = AppState::Game)]
enum GameState {
    #[default]
    Running,
    Paused,
}

// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
// struct GamePausedState;
//
// impl ComputedStates for GamePausedState {
//     type SourceStates = AppState;
//
//     fn compute(sources: Self::SourceStates) -> Option<Self> {
//         match sources {
//             AppState::Game { paused: true } => Some(Self),
//             _ => None,
//         }
//     }
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_sub_state::<GameState>()
        .add_plugins(ui::plugin)
        .add_plugins(game::plugin)
        .run();
}
