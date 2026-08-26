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
}


const GRAVITY_CONST: Vec2 = Vec2::new(0.0, -1.0);

pub fn gravity_system(
    mut phs_objs: Query<&mut Velocity, With<Phs>>,
) {
    for mut vel in phs_objs.iter_mut() {
        vel.0 += GRAVITY_CONST;
    }
}





