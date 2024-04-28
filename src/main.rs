use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
mod demos;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Basics".to_string(),
                        resizable: false,
                        resolution: WindowResolution::new(1080., 720.),
                        ..default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LdtkPlugin)
        // .add_plugins(demos::sprite::SpriteDemoPlugin)
        // .add_plugins(demos::moveable_sprite::MovableSpriteDemoPlugin)
        .add_plugins(demos::background::BackgroundPlugin)
        .insert_resource(LevelSelection::index(0))
        .run();
}
