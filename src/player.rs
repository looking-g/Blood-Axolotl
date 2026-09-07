/// Mod that holds all the player systems
use bevy::prelude::*;
use crate::physics::{Velocity, Depth};

pub fn player_plugin(app: &mut App) {
    app
        .add_systems(Update, (movement_system, depth_system, move_camera))
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

fn depth_system(
    player: Single<&mut Depth, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {

    let key_w = input.just_pressed(KeyCode::KeyW);
    let key_s = input.just_pressed(KeyCode::KeyS);

    if let Depth::Single(ref mut d) = *player.into_inner() {
        if !(key_w ^ key_s) {
            return;
        } else if key_s {
            *d -= 1;
        } else if key_w {
            *d += 1;
        }
    }
}

fn move_camera(
    player: Single<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let transfrom_diff = player.translation - camera.translation;
    camera.translation += transfrom_diff * 0.15;
}
