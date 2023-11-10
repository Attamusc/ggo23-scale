use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    LoadAssets,
    MainMenu,
    Game,
}
