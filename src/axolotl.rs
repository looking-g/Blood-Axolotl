/// Modual for the Blood Axolotl. (beware!)

use bevy::prelude::*;
use crate::physics::*;
use crate::Depth;
use crate::player::Player;

pub fn axolotl_plugin(app: &mut App) {
    app
        // core systems
        .add_systems(Startup, make_axolotl)
        .add_systems(Update, go_to_player)

        // other systems
        .add_systems(Update, set_to_front)
    ;
}

#[derive(Component)]
struct BloodAxolotl;

/// Speed at which the Blood Axolotl moves at (in world units)
const BA_SPEED: f32 = 1.0;

/// Summons the Blood Axolotl
fn make_axolotl(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-800.0, 60.0),
        Vec2::new(25.0, 70.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 0.0, 0.0),
        Some( (BloodAxolotl, depth!(0)) ),
    );
}

/// The Blood Axolotl is in pursuit of the player!
/// Makes the Blood Axolotl move towrod the player.
fn go_to_player(
    mut ba_transfrom: Single<&mut Transform, (With<BloodAxolotl>, Without<Player>)>,
    player_transfrom: Single<&Transform, With<Player>>,
) {
    let player_direction = player_transfrom.translation.xy() - ba_transfrom.translation.xy();

    let movement = (player_direction.x/player_direction.x.abs()) * BA_SPEED.min(player_direction.x.abs());
    
    if !movement.is_nan() {
        ba_transfrom.translation.x += movement; 
    }
}


/// Sets the Blood Axolotl to the front
fn set_to_front(
    mut ba_transfrom: Single<&mut Transform, With<BloodAxolotl>>,
    ba_depth: Single<&mut Depth, With<BloodAxolotl>>,
) {
    if let Depth::Single(d) = **ba_depth{
        ba_transfrom.translation.z = 100.0 - d as f32;
    }
}
