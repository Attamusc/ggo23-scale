use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::Velocity>()
            .register_type::<components::Movable>()
            .register_type::<components::Direction>()
            .add_systems(
                Update,
                (systems::calculate_velocity, systems::move_movable).chain(),
            )
            .add_systems(FixedUpdate, systems::update_position);
    }
}
