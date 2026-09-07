use bevy::prelude::*;

pub mod aabb;
#[macro_use]
pub mod physics;
pub mod player;
pub mod buildings;
pub mod debug;
pub mod collectables;

use physics::*;
use player::{Player, player_plugin};
use buildings::stairs::stair_plugin;
use buildings::buildings::buildings_plugin;
use collectables::collectable_plugin;
use debug::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Blood Axolotl".into(),
                    name: Some("blood-axolotl.app".into()),
                    resolution: (1200, 700).into(),
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, setup)
        .add_plugins((physics_plugin, stair_plugin, player_plugin, debug_plugin, buildings_plugin, collectable_plugin))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(-200.0, 0., 0.)),
    ));
 
    // player
    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-200.0, 60.0),
        Vec2::new(25.0, 50.0),
        &mut meshes,
        &mut materials,
        Vec3::splat(0.0),
        Some((
            Player,
            depth!(0),
        )),
    ); 

}
