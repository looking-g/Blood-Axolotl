/// Holds all the buildings in the game
use bevy::prelude::*;
use crate::physics::*;
use super::stairs::*;
use crate::collectables::{CollectableLocation, place_collectables};


pub fn buildings_plugin(app: &mut App) {

    app
        .add_systems(Startup, (ground, buildings, place_collectables).chain())
    ;
}


fn ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(0.0, -50.0),
        Vec2::new(1_000.0, 50.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 0.0),
        Some(Pin),
    );
}

fn buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    building_maker(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::ZERO,
    );

    building_maker(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(1_000.0, 0.0),
    );
}

fn building_maker(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec2,
) {

    // base floor
    StairsBundle::new_to_world(
        &mut commands,
        Vec2::new(0.0, 0.0) + pos,
        50.0,
        50.0,
        &mut meshes,
        &mut materials,
        Vec3::splat(1.0) * 0.8,
        Some(depth!(1)),
    );

    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-400.0, 25.0) + pos,
        Vec2::new(400.0, 25.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 0.0) * 0.8,
        Some((
            depth!(1, 2),
            Pin,
        )),
    );

    StairsBundle::new_to_world(
        &mut commands,
        Vec2::new(-800.0, 0.0) + pos,
        -50.0,
        50.0,
        &mut meshes,
        &mut materials,
        Vec3::splat(1.0) * 0.8,
        Some(depth!(1)),
    );

    // stairs up
    StairsBundle::new_to_world(
        &mut commands,
        Vec2::new(-400.0, 50.0) + pos,
        -100.0,
        200.0,
        &mut meshes,
        &mut materials,
        Vec3::splat(1.0) * 0.6,
        Some(depth!(2)),
    );

    // top floor
    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-400.0, 225.0) + pos,
        Vec2::new(400.0, 25.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 0.0) * 0.4,
        Some((
            depth!(3),
            Pin,
        )),
    );

    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-25.0, 225.0) + pos,
        Vec2::new(25.0, 75.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 0.0),
        Some(Pin),
    );

    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-775.0, 225.0) + pos,
        Vec2::new(25.0, 75.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1.0, 0.0),
        Some(Pin),
    );

    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-200.0, 275.0) + pos,
        Vec2::new(25.0, 25.0),
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, 1.0) * 0.2,
        Some((
            Pin,
            depth!(10),
            CollectableLocation,
        )),
    );
}
