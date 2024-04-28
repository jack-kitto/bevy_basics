use bevy::{prelude::*, window::PresentMode};
use bevy_ecs_ldtk::prelude::*;

mod demos;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Basics".into(),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(demos::sprite::SpriteDemoPlugin)
        .add_plugins(demos::background::BackgroundPlugin)
        .add_plugins(demos::moveable_sprite::MovableSpriteDemoPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    println!("Hello, Bevy!");
    commands.spawn(Camera2dBundle::default());
}
