use super::components::{Movable, Velocity};
use crate::{GridPosition, GRID_CELL_SIZE};
use bevy::prelude::*;

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Velocity>()
            .register_type::<Movable>()
            .add_systems(Update, calculate_movement)
            .add_systems(FixedUpdate, update_position);
    }
}

fn calculate_movement(mut movables: Query<(&mut GridPosition, &mut Velocity)>, time: Res<Time>) {
    for (mut gpos, mut velocity) in &mut movables {
        velocity.x *= 0.96;
        velocity.y *= 0.96;

        gpos.xr += velocity.x * time.delta_seconds();
        gpos.yr += velocity.y * time.delta_seconds();

        if gpos.xr > 1.0 {
            gpos.xr -= 1.0;
            gpos.x += 1;
        } else if gpos.xr < 0.0 {
            gpos.xr += 1.0;
            gpos.x -= 1;
        }

        if gpos.yr > 1.0 {
            gpos.yr -= 1.0;
            gpos.y += 1;
        } else if gpos.yr < 0.0 {
            gpos.yr += 1.0;
            gpos.y -= 1;
        }
    }
}

fn update_position(mut movables: Query<(&mut Transform, &GridPosition)>) {
    for (mut transform, gpos) in &mut movables {
        transform.translation.x = (f32::from(gpos.x) + gpos.xr) * GRID_CELL_SIZE;
        transform.translation.y = (f32::from(gpos.y) + gpos.yr) * GRID_CELL_SIZE;
    }
}
