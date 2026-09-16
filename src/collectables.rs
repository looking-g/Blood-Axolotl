//! Mod for the collectable items system(s)

// Interfacing with the rest of the game \\
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{
    aabb::Aabb,
    physics::{PhsObj, Pin, Depth},
    player::Player,
};
use rand::random_range;

pub fn collectable_plugin(app: &mut App) {
    app
        .insert_resource(Collected (0))
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
pub struct CollectableLocation {
    in_use: bool
}

impl std::default::Default for CollectableLocation{
    fn default() -> Self {
        Self {
            in_use: false,
        }
    }
}

/// Number of collectables spawned into the world with [`place_collectables`].
const NUM_COLLECTABLES: u32 = 3;

/// The max dist the player can be from a collectable to collect it
const COLLECTABLE_DIST: f32 = 300.0;

// Systems \\

/// Places collectables in the game world
pub fn place_collectables(
    mut commands: Commands,
    mut places_to_spawn: Query<(&Transform, &Aabb, &mut CollectableLocation), With<CollectableLocation>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut len = places_to_spawn.iter().len();
    if len == 0 { return; }

    // placing collectables
    for _ in 0..NUM_COLLECTABLES {
        // getting random positions
        let mut rand_sots_iter = places_to_spawn
            .iter_mut()
            .filter(|(_, _, collect_loc)| !collect_loc.in_use);

        if let Some((place_transform, place_aabb, mut collectable_location)) = rand_sots_iter.nth(random_range( 0_usize..(len) )) {
            PhsObj::new_to_world(
                &mut commands,
                place_transform.translation.xy() + Vec2::new(0.0, place_aabb.top() + 15.0),
                Vec2::new(15.0, 15.0),
                &mut meshes,
                &mut materials,
                Vec3::new(0.0, 1.0, 1.0),
                Some((Collectable, Pin, depth!(10))),
            );
            collectable_location.in_use = true;
            len -= 1;
        }

    }

}

/// Make collectables react to being collected
/// Assumes a camera scale of 1.0!
fn collectables_reaction(
    camera_transform: Single<&Transform, With<Camera2d>>,
    player_transform: Single<&Transform, With<Player>>,
    collectables: Query<(&Transform, &Aabb, Entity), With<Collectable>>,

    mut collect_count: ResMut<Collected>,
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    // getting curser pos in the world //

    // getting curser pos on screen
    let Some(mut cur_pos) = window.cursor_position() else {return;};
    // centering
    cur_pos -= window.physical_size().as_vec2() * 0.5;
    // flip y
    cur_pos.y *= -1.0;
    // accounting for camera pos
    cur_pos += camera_transform.translation.xy();

    let cur_size = Vec2::new(3.0, 3.0);
    let cur_aabb = Aabb::new(
        cur_pos + (cur_size * 0.5),
        cur_pos - (cur_size * 0.5),
    );

    // check mouse collision with each collectable //
    for (c_transform, c_aabb, c_entity) in collectables {
        if player_transform.translation.xy().distance_squared(c_transform.translation.xy()) <= COLLECTABLE_DIST*COLLECTABLE_DIST { // if close enough
            if c_aabb.translate(c_transform.translation.xy()).collide(&cur_aabb) { // if mouse over
                if buttons.any_just_pressed([MouseButton::Left, MouseButton::Middle, MouseButton::Right]) { // if mouse click
                    commands.entity(c_entity).despawn();
                    collect_count.0 += 1;
                }
            }
        }
    }
}




