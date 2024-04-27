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
        app.add_systems(Startup, sprite_demo);
    }
}

fn sprite_demo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("SpriteDemo.png"),
        ..default()
    });
}
