use bevy::prelude::*;

mod demos;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(demos::sprite::SpriteDemoPlugin)
        .add_plugins(demos::moveable_sprite::MovableSpriteDemoPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    println!("Hello, Bevy!");
}
