/// Holds all the buildings in the game
use bevy::prelude::*;
use crate::physics::*;
use super::stairs::*;

pub fn buildings_plugin(app: &mut App) {

    app
        .add_systems(Startup, (ground, building_1))
    ;
}


fn ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(0.0, -300.0),
        Vec2::new(1_000.0, 50.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 0.0),
        Some(Pin),
    );
}

fn building_1(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    StairsBundle::new_to_world(
        &mut commands,
        Vec2::new(0.0, -250.0),
        50.0,
        50.0,
        &mut meshes,
        &mut materials,
        Vec3::splat(1.0) * 0.8,
        Some(Depth(1)),
    );

    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-200.0, -225.0),
        Vec2::new(200.0, 25.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 0.0) * 0.8,
        Some((
            Depth(1),
            Pin,
        )),
    );


    StairsBundle::new_to_world(
        &mut commands,
        Vec2::new(-400.0, -250.0),
        -50.0,
        50.0,
        &mut meshes,
        &mut materials,
        Vec3::splat(1.0) * 0.8,
        Some(Depth(1)),
    );

}
