use bevy::prelude::*;

pub mod aabb;
pub mod physics;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Blood Axolotl".into(),
                    name: Some("blood-axolotl.app".into()),
                    resolution: (1200, 700).into(),
                    ..default()
                }),
                ..default()
            }),
        ))

        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
    ));


    commands.spawn((
        Mesh2d( meshes.add(Rectangle::new(100.0, 100.0)) ),
        MeshMaterial2d(materials.add( Color::srgba(0.0, 0.0, 0.0, 1.0) )),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));


}
