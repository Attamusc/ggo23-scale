use super::components::Enemy;
use crate::grid::components::GridPosition;
use crate::movable::components::{Movable, MovableBundle};
use bevy::prelude::*;

pub fn setup_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("scribbles/characters/red_character.png");

    commands.spawn((
        Name::new("Enemy"),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            texture: texture.clone(),
            ..default()
        },
        GridPosition::at_coordinates(3, 3),
        MovableBundle {
            movable: Movable { speed: 3. },
            ..default()
        },
        Enemy,
    ));

    commands.spawn((
        Name::new("Enemy"),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            texture: texture.clone(),
            ..default()
        },
        GridPosition::at_coordinates(5, 3),
        MovableBundle {
            movable: Movable { speed: 3. },
            ..default()
        },
        Enemy,
    ));
}
