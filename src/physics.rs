//! This mod handles all physics related work like collision handleing and gravity

use bevy::prelude::*;
use crate::aabb::*;

#[derive(Component, Default)]
/// Holds the velocity of an object.
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
/// Marker for physics objects
pub struct Phs;

#[derive(Component, Default)]
/// Marker that states that a [`PhsObj`] will not be effected by gravity.
pub struct Pin;

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
                half_size * Vec2::new(-1.0, 1.0),
                half_size * Vec2::new(1.0, -1.0),
            ),
            transform: Transform::from_translation(pos.extend(0.0)),
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
        .add_message::<SolveCollision>()
        .add_systems(Update, (gravity_system, apply_vel_system, collision_reaction_system, collision_reaction_reader).chain())
    ;
}

// TODO: change to a normal value later
const GRAVITY_CONST: Vec2 = Vec2::new(0.0, -25.0);

/// Applys gravity to an object's velocity.
pub fn gravity_system(
    mut phs_objs: Query<&mut Velocity, (With<Phs>, Without<Pin>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for mut vel in phs_objs.iter_mut() {
        vel.0 += GRAVITY_CONST * dt;
    }
}

/// Applys an object's velocity to it's position.
pub fn apply_vel_system(
    mut phs_objs: Query<(&Velocity, &mut Transform), (With<Phs>, Without<Pin>)>,
) {
    for (vel, mut transform) in phs_objs.iter_mut() {
        transform.translation += vel.0.extend(0.0);
    }

}


/// Finds collisions for [`PhsObj`]s, reactions are caried out by [`collision_reaction_reader`]
pub fn collision_reaction_system(
    phs_objs: Query<(&Aabb, &Transform, Has<Pin>, Entity), With<Phs>>,
    mut writer: MessageWriter<SolveCollision>,
) {
    for (aabb, transform, has_pin, entity) in phs_objs.iter() {
        for (other_aabb, other_transform, _ , other_entity) in phs_objs.iter() {
            if !has_pin && entity != other_entity{
                let self_world_aabb = aabb.translate(transform.translation.xy());
                let other_world_aabb = other_aabb.translate(other_transform.translation.xy());

                if Aabb::collide(&self_world_aabb, &other_world_aabb) {

                    // getting overlap

                    let overlap = self_world_aabb.collideing_side(&other_world_aabb);
 
                    writer.write(SolveCollision{
                            entity,
                            x_overlap: overlap.x,
                            y_overlap: overlap.y,
                    });
                }
            }
        }
    }
}

#[derive(Message, Debug)]
pub struct SolveCollision {
    entity: Entity,
    x_overlap: f32,
    y_overlap: f32,
}

/// Enacts collision reactions found by [`collision_reaction_system`]
fn collision_reaction_reader(
    mut phs_objs: Query<(&mut Velocity, &mut Transform), With<Phs>>,
    mut reader: MessageReader<SolveCollision>,
) {
    for SolveCollision{entity, x_overlap, y_overlap} in reader.read() {
        if let Ok((mut vel, mut transform)) = phs_objs.get_mut(*entity) {
            if y_overlap.abs() + x_overlap.abs() < 0.0001 {
                continue; 
            } else if y_overlap.abs() <= x_overlap.abs() {
                vel.0.y = 0.0;
                transform.translation.y += y_overlap;
            } else if x_overlap.abs() < y_overlap.abs() {
                vel.0.x = 0.0;
                transform.translation.x += x_overlap;
            }

        }
    }
}









