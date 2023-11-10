use super::components::{Direction, Movable, Velocity};
use crate::grid::components::GridPosition;
use bevy::prelude::*;

const FRICTION: f32 = 0.96;

pub fn calculate_velocity(mut movables: Query<(&mut Velocity, &Movable, &Direction)>) {
    for (mut velocity, movable, direction) in &mut movables {
        if direction.x != 0 {
            velocity.x = direction.x as f32 * movable.speed;
        }
        if direction.y != 0 {
            velocity.y = direction.y as f32 * movable.speed;
        }
    }
}

pub fn move_movable(mut movables: Query<(&mut GridPosition, &mut Velocity)>, time: Res<Time>) {
    for (mut gpos, mut velocity) in &mut movables {
        if velocity.x != 0. && velocity.y != 0. {
            velocity.x *= 1. / std::f32::consts::SQRT_2;
            velocity.y *= 1. / std::f32::consts::SQRT_2;
        }

        velocity.x *= FRICTION;
        velocity.y *= FRICTION;

        gpos.ratio.x += velocity.x * time.delta_seconds();
        gpos.ratio.y += velocity.y * time.delta_seconds();

        if gpos.ratio.x > 1.0 {
            gpos.ratio.x -= 1.0;
            gpos.coordinates.x += 1;
        } else if gpos.ratio.x < 0.0 {
            gpos.ratio.x += 1.0;
            gpos.coordinates.x -= 1;
        }

        if gpos.ratio.y > 1.0 {
            gpos.ratio.y -= 1.0;
            gpos.coordinates.y += 1;
        } else if gpos.ratio.y < 0.0 {
            gpos.ratio.y += 1.0;
            gpos.coordinates.y -= 1;
        }
    }
}

pub fn update_position(mut movables: Query<(&mut Transform, &GridPosition)>) {
    for (mut transform, gpos) in &mut movables {
        transform.translation.x = gpos.unit_x();
        transform.translation.y = gpos.unit_y();
    }
}
