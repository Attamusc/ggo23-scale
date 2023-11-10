use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::Enemy>()
            .add_systems(Startup, systems::setup_enemies);
    }
}
