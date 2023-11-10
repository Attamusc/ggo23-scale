use crate::globals::GRID_CELL_SIZE;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // camera.projection.scale = 4.;
    camera.transform.translation = Vec3::new(
        10. * GRID_CELL_SIZE,
        6. * GRID_CELL_SIZE,
        camera.transform.translation.z,
    );

    commands.spawn(camera);
}
