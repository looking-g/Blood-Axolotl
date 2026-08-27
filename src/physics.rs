//! This mod handles all physics related work like collision handleing and gravity

use bevy::prelude::*;
use crate::aabb::*;

#[derive(Component, Default)]
/// Holds the velocity of an object.
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
/// Marker for physics objects
pub struct Phs;

#[derive(Bundle, Default)]
/// An object that is effected by physics
pub struct PhsObj{
    aabb: Aabb,
    transform: Transform,
    velocity: Velocity,
    phs: Phs,
}
impl PhsObj {
    pub fn new(pos: Vec2, half_size: Vec2) -> Self {
        Self {
            aabb: Aabb::new(
                half_size * Vec2::new(1.0, -1.0),
                half_size * Vec2::new(-1.0, 1.0),
            ),
            transform: Transform::from_translation(pos.extend(1.0)),
            ..default()
        }
    }

    /// Creates a new [`PhsObj`] and puts it into the World.
    pub fn new_to_world(
        commands: &mut Commands,
        pos: Vec2, 
        half_size: Vec2,
        extra_components: Option<impl Bundle>,
    ) {
        let id = commands.spawn(
            Self::new(pos, half_size)
        ).id();

        if let Some(bundle) = extra_components {
            commands.entity(id).insert(bundle);
        }

    }
}


pub fn physics_plugin(app: &mut App) {
    app
        .add_systems(Update, (gravity_system, apply_vel_system))
    ;
}

// TODO: change to a normal value later
const GRAVITY_CONST: Vec2 = Vec2::new(0.0, -0.01);

/// Applys gravity to an object's velocity.
pub fn gravity_system(
    mut phs_objs: Query<&mut Velocity, With<Phs>>,
) {
    for mut vel in phs_objs.iter_mut() {
        vel.0 += GRAVITY_CONST;
    }
}

/// Applys an object's velocity to it's position.
pub fn apply_vel_system(
    mut phs_objs: Query<(&Velocity, &mut Transform), With<Phs>>,
) {
    for (vel, mut transform) in phs_objs.iter_mut() {
        transform.translation += vel.0.extend(0.0);
    }

}






