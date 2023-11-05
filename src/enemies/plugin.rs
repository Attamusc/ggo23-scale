use super::components::Enemy;
use crate::movable::components::{Movable, Velocity};
use crate::GridPosition;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .add_systems(Startup, setup_enemies);
    }
}

fn setup_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("scribbles/characters/red_character.png");

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
            x: 3,
            y: 3,
            xr: 0.0,
            yr: 0.0,
        },
        Velocity { x: 1.0, y: 0.0 },
        Movable { speed: 3.0 },
        Enemy,
        Name::new("Enemy"),
    ));

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
            x: -3,
            y: -3,
            xr: 0.0,
            yr: 0.0,
        },
        Velocity { x: 1.0, y: 0.0 },
        Movable { speed: 3.0 },
        Enemy,
        Name::new("Enemy"),
    ));
}
