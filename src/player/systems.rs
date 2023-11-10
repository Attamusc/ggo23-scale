use super::components::Player;
use crate::globals::GRID_CELL_SIZE;
use crate::grid::components::GridPosition;
use crate::movable::components::{Direction, Movable, MovableBundle};
use bevy::prelude::*;

const PLAYER_Z: f32 = 10.0;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("scribbles/characters/purple_character.png");

    commands.spawn((
        Name::new("Player"),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRID_CELL_SIZE, GRID_CELL_SIZE)),
                ..default()
            },
            texture: texture.clone(),
            transform: Transform::from_xyz(0.0, 0.0, PLAYER_Z),
            ..default()
        },
        GridPosition::at_coordinates(10, 6),
        MovableBundle {
            movable: Movable { speed: 3.0 },
            ..default()
        },
        Player,
    ));
}

pub fn process_input(
    mut player: Query<(&mut Direction, With<Player>)>,
    input: Res<Input<KeyCode>>,
) {
    for (mut direction, _) in &mut player {
        direction.x = 0;
        direction.y = 0;

        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            direction.y = 1;
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            direction.y = -1;
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            direction.x = 1;
        }
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            direction.x = -1;
        }
    }
}
