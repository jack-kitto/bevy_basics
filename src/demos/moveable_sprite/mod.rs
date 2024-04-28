use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    asset::AssetServer,
    ecs::system::{Commands, Res},
    sprite::SpriteBundle,
};

pub struct MovableSpriteDemoPlugin;
#[derive(Component)]
struct Player;

impl Plugin for MovableSpriteDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sprite_demo)
            .add_systems(Update, sprite_controller);
    }
}

fn sprite_demo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("SpriteDemo.png"),
            ..default()
        },
    ));
}

fn sprite_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player = query.single_mut();
    let velocity = 5.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        player.translation.x -= velocity
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        player.translation.x += velocity;
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        player.translation.y += velocity;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        player.translation.y -= velocity;
    }
}
