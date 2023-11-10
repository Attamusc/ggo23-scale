use crate::globals::{WORLD_HEIGHT, WORLD_WIDTH};
use bevy::prelude::*;

mod camera;
mod debug;
mod enemies;
mod globals;
mod grid;
mod level;
mod movable;
mod player;
mod states;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "ggo23-scale".into(),
                    resolution: (WORLD_WIDTH, WORLD_HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),))
        .add_state::<states::GameState>()
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(level::LevelPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(grid::GridWorldPlugin)
        .add_plugins(movable::MovablePlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemies::EnemyPlugin)
        .run();
}
