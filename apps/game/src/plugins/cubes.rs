use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, CollisionGroups, RigidBody};
use std::f32::consts::PI;

use super::CubesPlugin;
use crate::plugins::{GROUP_BODY, GROUP_SURFACE, GROUP_WHEEL};

#[derive(Component)]
struct CubeObstacle {
    index: u8,
}

impl Plugin for CubesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, cube_animation_system);
    }
}

fn cube_animation_system(time: Res<Time>, mut players: Query<(&mut Transform, &CubeObstacle)>) {
    for (mut transform, player) in &mut players {
        transform.translation = Vec3::new(
            player.index as f32 - 2.5,
            0.25,
            (time.elapsed_seconds() + player.index as f32 * PI / 6.0).cos() + 1.25,
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    for x in 0..6 {
        commands
            .spawn((PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::hsl(60.0 * x as f32, 1.0, 0.5).into()),
                transform: Transform::from_xyz(-2.5 + 1.0 * x as f32, 0.25, 0.0),
                ..default()
            },))
            .insert(CubeObstacle { index: x })
            .insert(Name::new("Cube"))
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::cuboid(0.25, 0.25, 0.25))
            .insert(CollisionGroups::new(
                bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_SURFACE),
                bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_BODY | GROUP_WHEEL),
            ));
    }
}
