//! Mod for the collectable items system(s)

// Interfacing with the rest of the game \\
use bevy::prelude::*;
use crate::{
    aabb::Aabb,
    physics::{PhsObj, Pin},
};
use rand::random_range;

pub fn collectable_plugin(app: &mut App) {
    app
        .add_systems(Update, collectables_reaction)
    ;
}

// Components / Parts \\

/// Tottle number of collectables collected
#[derive(Resource)]
pub struct Collected(pub u32);

/// Tags an entity **as a** collectable.
#[derive(Component)]
struct Collectable;

/// Tags an entity **as a location** for collectables to spawn at (need a Transform and a
/// Aabb). 
/// Do NOT use [`Collectable`] and [`CollectableLocation`] in the same Entity!
#[derive(Component)]
pub struct CollectableLocation;

/// Number of collectables spawned into the world with [`place_collectables`].
const NUM_COLLECTABLES: u32 = 3;

// Systems \\

/// Places collectables in the game world
pub fn place_collectables(
    mut commands: Commands,
    places_to_spawn: Query<(&Transform, &Aabb), With<CollectableLocation>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let len = places_to_spawn.iter().len();
    if len == 0 { return; }

    // placing collectables
    for _ in 0..NUM_COLLECTABLES {
        // getting random positions
        if let Some((place_transform, place_aabb)) = places_to_spawn.iter().nth(random_range( 0_usize..(len) )) {
            PhsObj::new_to_world(
                &mut commands,
                place_transform.translation.xy() + Vec2::new(0.0, place_aabb.top() + 15.0),
                Vec2::new(15.0, 15.0),
                &mut meshes,
                &mut materials,
                Vec3::new(0.0, 1.0, 1.0),
                Some((Collectable, Pin)),
            );
        } 

    }

}

/// Make collectables react to being collected
fn collectables_reaction(

) {

}




