use bevy::prelude::*;
use demos::sprite::SpriteDemoPlugin;

mod demos;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpriteDemoPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    println!("Hello, Bevy!");
}
