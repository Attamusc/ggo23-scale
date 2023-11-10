use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::Player>()
            .add_systems(Startup, systems::setup_player)
            .add_systems(Update, systems::process_input);
    }
}
