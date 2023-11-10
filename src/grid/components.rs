use crate::globals::GRID_CELL_SIZE;
use bevy::prelude::*;

#[derive(Component)]
pub struct GridWorld;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GridPosition {
    pub coordinates: Coordinates,
    pub ratio: Ratio,
}

#[derive(Default, Reflect)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

#[derive(Default, Reflect)]
pub struct Ratio {
    pub x: f32,
    pub y: f32,
}

impl GridPosition {
    pub fn at_coordinates(x: i16, y: i16) -> Self {
        GridPosition {
            coordinates: Coordinates { x, y },
            ratio: Ratio { x: 0.5, y: 0.5 },
        }
    }

    pub fn unit_x(&self) -> f32 {
        (self.coordinates.x as f32 + self.ratio.x) * GRID_CELL_SIZE
    }

    pub fn unit_y(&self) -> f32 {
        (self.coordinates.y as f32 + self.ratio.y) * GRID_CELL_SIZE
    }
}
