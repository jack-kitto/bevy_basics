use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy::{
    app::{App, Plugin, Startup},
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

pub struct BackgroundPlugin;
#[derive(Component)]
struct Background;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (on_resize_system));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window>,
    resize_event: Res<Events<WindowResized>>,
) {
    let window = windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();
    commands.spawn((
        Background,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            texture: asset_server.load("background.png"),
            ..default()
        },
    ));
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<&mut Transform, With<Background>>,
) {
    for e in resize_reader.read() {
        // When resolution is being changed
        // text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
        let width = e.width;
        let height = e.height;
        let mut background = query.single_mut();
    }
}
