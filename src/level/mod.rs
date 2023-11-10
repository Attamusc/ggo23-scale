use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::Uid(0))
            .add_systems(Startup, spawn_level);
    }
}

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server.load("levels/test.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..default()
    });
}
