use bevy::prelude::*;

pub mod aabb;
pub mod physics;
pub mod player;
pub mod buildings;

use physics::*;
use player::{Player, player_plugin};
use buildings::stairs::{stair_plugin, StairsBundle};

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
        .add_plugins((physics_plugin, stair_plugin, player_plugin))

        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
    ));

    
    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-200.0, 60.0),
        Vec2::new(25.0, 50.0),
        Some((
            Mesh2d( meshes.add(Rectangle::new(50.0, 100.0)) ),
            MeshMaterial2d(materials.add( Color::srgba(0.0, 0.0, 0.0, 1.0) )),
            Player,
        )),
    );

    
    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(0.0, -300.0),
        Vec2::new(500.0, 50.0),
        Some((
            Mesh2d( meshes.add(Rectangle::new(1_000.0, 100.0)) ),
            MeshMaterial2d(materials.add( Color::srgba(1.0, 1.0, 0.0, 1.0) )),
            Pin,
        )),
    );


    StairsBundle::new_to_world(
        &mut commands,
        Vec2::new(0.0, -250.0),
        -50.0,
        50.0,
        Some((
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(-50.0, 0.0),
                Vec2::new(0.0, 50.0),
            ))),
            MeshMaterial2d(materials.add( Color::srgba(1.0, 1.0, 1.0, 1.0) )),
        )),
    );


    StairsBundle::new_to_world(
        &mut commands,
        Vec2::new(300.0, -250.0),
        -50.0,
        150.0,
        Some((
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(-50.0, 0.0),
                Vec2::new(0.0, 150.0),
            ))),
            MeshMaterial2d(materials.add( Color::srgba(1.0, 1.0, 1.0, 1.0) )),
        )),
    );


    StairsBundle::new_to_world(
        &mut commands,
        Vec2::new(-400.0, -250.0),
        150.0,
        50.0,
        Some((
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(150.0, 0.0),
                Vec2::new(0.0, 50.0),
            ))),
            MeshMaterial2d(materials.add( Color::srgba(1.0, 1.0, 1.0, 1.0) )),
        )),
    );

}
