use bevy::prelude::*;

pub mod components;

pub struct GridWorldPlugin;

impl Plugin for GridWorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::GridPosition>();
    }
}
