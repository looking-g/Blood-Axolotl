/// File for debuging the app
use bevy::prelude::*;
use crate::physics::Depth;
use crate::player::Player;
use crate::collectables::Collected;

pub fn debug_plugin(app: &mut App) {
    app
        .add_systems(Update, display_info_top)
    ;
}

fn display_info_top(
    mut text_gizmos: Gizmos,
    player: Single<&Depth, With<Player>>,
    collect_count: Res<Collected>,
) {
    if let Depth::Single(d) = player.into_inner() {
        text_gizmos.text_2d(
            Isometry2d::from_xy(0.0, 300.0),
            &format!("player depth: {}", d),
            30.0,
            Vec2::ZERO,
            Color::WHITE,
        );
    }

    text_gizmos.text_2d(
        Isometry2d::from_xy(0.0, 340.0),
        &format!("collected collectables: {}", collect_count.0),
        30.0,
        Vec2::ZERO,
        Color::WHITE,
    );
}
