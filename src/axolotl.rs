/// Modual for the Blood Axolotl. (beware!)

use bevy::prelude::*;
use crate::physics::*;
use crate::Depth;

pub fn axolotl_plugin(app: &mut App) {
    app
        .add_systems(Startup, make_axolotl)
        .add_systems(Update, set_to_front)
    ;
}

#[derive(Component)]
struct BloodAxolotl;

/// Summons the Blood Axolotl
fn make_axolotl(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    PhsObj::new_to_world(
        &mut commands,
        Vec2::new(-250.0, 60.0),
        Vec2::new(25.0, 70.0),
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 0.0, 0.0),
        Some( (BloodAxolotl, depth!(0)) ),
    );
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
