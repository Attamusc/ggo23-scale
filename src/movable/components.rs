use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Movable {
    pub speed: f32,
}
