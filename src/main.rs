use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::movable::plugin::MovablePlugin;
use crate::player::plugin::PlayerPlugin;

mod movable;
mod player;

const GRID_CELL_SIZE: f32 = 64.0;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GridWorld;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
    pub xr: f32,
    pub yr: f32,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ggo23-scale".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .register_type::<GridWorld>()
        .register_type::<GridPosition>()
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Grave)),
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(MovablePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
