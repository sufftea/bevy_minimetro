#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]


use bevy::{prelude::*, reflect::hash_error};
use bevy_tweening::TweeningPlugin;

mod game;
mod main_menu;
mod style;
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
        .add_plugins(TweeningPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .init_state::<AppState>()
        .add_sub_state::<GameState>()
        .add_plugins(main_menu::plugin)
        .add_plugins(game::plugin)
        .run();
}
