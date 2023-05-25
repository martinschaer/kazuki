use bevy::{
    app::{App, Plugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use super::CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::hsla(60.0, 0.0, 0.5, 0.8).into()),
            transform: Transform::from_xyz(0., 4., 2.)
                .with_rotation(Quat::from_rotation_y((45_f32).to_radians())),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.25, 0.25, 0.25))
        .insert(Restitution::coefficient(0.5));
}
