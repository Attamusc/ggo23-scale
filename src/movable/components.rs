use bevy::prelude::*;

#[derive(Bundle)]
pub struct MovableBundle {
    pub movable: Movable,
    pub direction: Direction,
    pub velocity: Velocity,
    pub collidor: Collidor,
}

impl Default for MovableBundle {
    fn default() -> Self {
        MovableBundle {
            movable: Movable { speed: 0. },
            direction: Direction { x: 0, y: 0 },
            velocity: Velocity { x: 0., y: 0. },
            collidor: Collidor { radius: 0. },
        }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Direction {
    pub x: i8,
    pub y: i8,
}

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

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Collidor {
    pub radius: f32,
}
