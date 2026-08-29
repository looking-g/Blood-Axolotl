/// Mod that holds all the player systems
use bevy::prelude::*;
use crate::physics::Velocity;

pub fn player_plugin(app: &mut App) {
    app
        .add_systems(Update, movement_system)
    ;
}

/// Marker for the player
#[derive(Component)]
pub struct Player;

/// The player's speed
const PLAYER_SPEED: f32 = 100.0;

fn movement_system(
    mut player: Single<&mut Velocity, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    let key_d = input.pressed(KeyCode::KeyD);
    let key_a = input.pressed(KeyCode::KeyA);

    if !(key_a ^ key_d) {
        player.0.x = 0.0;
    } else if key_a {
        player.0.x = -PLAYER_SPEED * dt;
    } else if key_d {
        player.0.x = PLAYER_SPEED * dt;
    }
}

