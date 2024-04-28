use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Res},
    sprite::SpriteBundle,
};

pub struct SpriteDemoPlugin;

impl Plugin for SpriteDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 1.;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("SpriteDemo.png"),
        ..default()
    });
}
