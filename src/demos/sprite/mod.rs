use bevy::app::{App, Plugin, Startup};

pub struct SpriteDemoPlugin;

impl Plugin for SpriteDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sprite_demo);
    }
}

fn sprite_demo() {
    println!("Hello, Bevy! This is the sprite demo.");
}
