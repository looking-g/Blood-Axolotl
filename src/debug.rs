/// File for debuging the app
use bevy::prelude::*;
use crate::physics::Depth;
use crate::player::Player;

pub fn debug_plugin(app: &mut App) {
    app
        .add_systems(Update, display_info_top)
    ;
}

fn display_info_top(
    mut text_gizmos: Gizmos,
    player: Single<&Depth, With<Player>>,
) {
    text_gizmos.text_2d(
        Isometry2d::from_xy(0.0, 300.0),
        &format!("player depth: {}", player.0),
        30.0,
        Vec2::ZERO,
        Color::WHITE,
    );
}
