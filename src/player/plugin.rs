use super::components::Player;
use crate::movable::components::{Movable, Velocity};
use crate::GridPosition;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, process_input);
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("scribbles/characters/purple_character.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            texture: texture.clone(),
            ..default()
        },
        GridPosition {
            x: 0,
            y: 0,
            xr: 0.0,
            yr: 0.0,
        },
        Velocity { x: 0.0, y: 0.0 },
        Movable { speed: 3.0 },
        Player,
        Name::new("Player"),
    ));
}

fn process_input(
    mut player: Query<(&mut Velocity, &Movable, With<Player>)>,
    input: Res<Input<KeyCode>>,
) {
    for (mut velocity, movable, _) in &mut player {
        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            velocity.y = movable.speed;
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            velocity.y = -movable.speed;
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            velocity.x = movable.speed;
        }
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            velocity.x = -movable.speed;
        }
    }
}
