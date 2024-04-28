use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::Commands,
};
use leafwing_input_manager::prelude::*;

pub struct MovableSpriteDemoPlugin;
#[derive(Component)]
struct Player;

impl Plugin for MovableSpriteDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ArpgAction>::default())
            // The InputMap and ActionState components will be added to any entity with the Player component
            .add_systems(Startup, spawn_player)
            // .add_systems(Update, sprite_controller)
            .add_event::<PlayerWalk>()
            .add_systems(Update, player_walks);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum ArpgAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
}

impl ArpgAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        ArpgAction::Up,
        ArpgAction::Down,
        ArpgAction::Left,
        ArpgAction::Right,
    ];

    fn direction(self) -> Option<Direction2d> {
        match self {
            ArpgAction::Up => Some(Direction2d::Y),
            ArpgAction::Down => Some(Direction2d::NEG_Y),
            ArpgAction::Left => Some(Direction2d::NEG_X),
            ArpgAction::Right => Some(Direction2d::X),
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    input_manager: InputManagerBundle<ArpgAction>,
    sprite: SpriteBundle,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<ArpgAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use ArpgAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Up, KeyCode::ArrowUp);
        input_map.insert(Up, GamepadButtonType::DPadUp);

        input_map.insert(Down, KeyCode::ArrowDown);
        input_map.insert(Down, GamepadButtonType::DPadDown);

        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Left, GamepadButtonType::DPadLeft);

        input_map.insert(Right, KeyCode::ArrowRight);
        input_map.insert(Right, GamepadButtonType::DPadRight);

        input_map
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        player: Player,
        input_manager: InputManagerBundle::with_map(PlayerBundle::default_input_map()),
        sprite: SpriteBundle {
            texture: asset_server.load("SpriteDemo.png"),
            ..default()
        },
    });
}

#[derive(Event)]
pub struct PlayerWalk {
    pub direction: Direction2d,
}

fn player_walks(
    state_query: Query<&ActionState<ArpgAction>, With<Player>>,
    mut event_writer: EventWriter<PlayerWalk>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let action_state = state_query.single();
    let mut player = player_query.single_mut();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in ArpgAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += *direction;
            }
        }
    }

    // Then reconvert at the end, normalizing the magnitude
    let net_direction = Direction2d::new(direction_vector);

    if let Ok(direction) = net_direction {
        event_writer.send(PlayerWalk { direction });
    }

    if action_state.pressed(&ArpgAction::Up) {
        player.translation.y += 1.0;
    }
    if action_state.pressed(&ArpgAction::Down) {
        player.translation.y -= 1.0;
    }
    if action_state.pressed(&ArpgAction::Left) {
        player.translation.x -= 1.0;
    }
    if action_state.pressed(&ArpgAction::Right) {
        player.translation.x += 1.0;
    }
}
