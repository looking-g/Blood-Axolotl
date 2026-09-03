use bevy::prelude::*;
use crate::physics::{Phs, Pin, SolveCollision};
use crate::aabb::Aabb;

pub fn stair_plugin(app: &mut App) {
    app
        .add_systems(Update, stair_system)
    ;
}

/// Updates the stars (makes them work)
fn stair_system(
    phs_objs: Query<(&Aabb, &Transform, Entity), (With<Phs>, Without<Pin>)>,
    stairs: Query<(&Transform, &StairsSize), With<Stairs>>,
    mut writer: MessageWriter<SolveCollision>,
) {
    for (stair_transform, stair_size) in stairs.iter() {
        let stairs_aabb = Aabb::new(
            stair_transform.translation.xy(),
            Vec2::new(
                stair_size.length,
                stair_size.height,
            ) + stair_transform.translation.xy(),
        );

        for (ob_aabb, ob_transform, ob_entity) in phs_objs.iter() { 
            // basic collision check
            // translated ob_aabb
            let t_ob_aabb = ob_aabb.translate(ob_transform.translation.xy());
            if !stairs_aabb.collide(&t_ob_aabb) { continue; }

            // the vertical collision of the other object down onto the stars
            // (negetive = inside stairs)
            let collision_height = {
                // closest ob_aabb point to the triangle
                let cpoint = Vec2::new(
                    if stair_size.length < 0.0 {
                        t_ob_aabb.right()
                    } else {
                        t_ob_aabb.left()
                    },
                    t_ob_aabb.bottom(),
                );

                // given the x val of cpoint, we solve for what the y
                // would be if the point was right on the line
                //
                // y − y_1 = m(x − x_1)
                //   + y_1              + y_1
                // y = m(x - x_1) + y_1

                let top_y_pos = 
                    (-stair_size.height / stair_size.length)
                    * (
                        cpoint.x.clamp(stairs_aabb.left(), stairs_aabb.right())
                        - stair_transform.translation.x
                    )
                    + stairs_aabb.top()
                ;

                // getting the difference from top_x_pos to current object x
                
                t_ob_aabb.bottom() - top_y_pos
            };

            let corrected_collision = -collision_height;

            if collision_height < 0.0 || true { // remove || true of snaping 
                                                // becomes an issue
                writer.write(SolveCollision{
                    entity: ob_entity,
                    x_overlap: 1.0 / 0.0,
                    y_overlap: corrected_collision,
                });
            }

        }
    }
}


/// Stairs are unique; where normal AABB haves standared collision proporties and functions, stairs
/// only displaces __Physics Objects__ upword.
/// Stairs also have a traianglular "collision aria," specifiacly, a right triangle with the
/// opposite above the adjacent.
/// Stairs and _technically_ always pinned.
#[derive(Component, Default)]
pub struct Stairs;

/// Holds the size of the staris.
#[derive(Component, Default)]
struct StairsSize {
    /// The lenght of the adjacent/bace. Positive is right, negitive is left.
    length: f32,
    /// The height of the opposite. Should always be positive. A negitive height dosn't make sence
    /// in the way of stairs (try moveing the pos down?).
    height: f32,
}

#[derive(Bundle, Default)]
pub struct StairsBundle{
    /// The pos of the stairs are located where the two legs meet, where the 90° is at.
    transform: Transform,
    /// See [`StairSize`] for more info.
    size: StairsSize,
    stairs: Stairs,
}

impl StairsBundle {
    pub fn new(pos: Vec2, length: f32, height: f32) -> Self {
        if height < 0.0 {panic!("Stairs can't have a negitive height!")}

        Self {
            transform: Transform::from_translation(pos.extend(0.0)),
            size: StairsSize {
                length,
                height,
            },
            ..default()
        }
    }

    /// Creates a new [`PhsObj`] and puts it into the World.
    pub fn new_to_world(
        commands: &mut Commands,
        pos: Vec2, 
        length: f32, 
        height: f32,
        extra_components: Option<impl Bundle>,
    ) {
        let id = commands.spawn(
            Self::new(pos, length, height)
        ).id();

        if let Some(bundle) = extra_components {
            commands.entity(id).insert(bundle);
        }

    }
}

