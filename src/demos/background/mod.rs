use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    asset::AssetServer,
    ecs::system::{Commands, Res},
};
use bevy_ecs_ldtk::prelude::*;

pub struct BackgroundPlugin;
#[derive(Component)]
struct Background;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    let scale = 0.23;
    let window_width = 1080.0;
    let window_height = 720.0;
    let denominator = 2. / scale;
    let translation_x = window_width / denominator;
    let translation_y = window_height / denominator;
    camera.projection.scale = scale;
    camera.transform.translation.x += translation_x;
    camera.transform.translation.y += translation_y;
    commands.spawn(camera);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("bevy-basics.ldtk"),
        ..Default::default()
    });
}
