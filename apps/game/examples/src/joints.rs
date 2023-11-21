use std::f32::consts::PI;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    Collider, GenericJointBuilder, ImpulseJoint, JointAxesMask, RigidBody, JointAxis,
};

use super::JointsPlugin;

impl Plugin for JointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1. }));
    let wheel_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.5,
        height: 1.,
        ..default()
    }));
    let cube = commands
        .spawn((PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        },))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .id();

    let wheel = commands
        .spawn((PbrBundle {
            mesh: wheel_mesh.clone(),
            transform: Transform {
                translation: Vec3::new(1.2, 1., 0.),
                rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 2.),
                ..default()
            },
            ..default()
        },))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(0.5, 0.5))
        .id();

    // Joint
    let joint = GenericJointBuilder::new(
        JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
    )
    .local_axis2(-Vec3::Y)
    .local_axis1(Vec3::X)
    .local_anchor2(Vec3::new(0., 0.6, 0.))
    .local_anchor1(Vec3::new(0.6, 0., 0.))
    .set_motor(JointAxis::AngX, 0., 1e2, 1e6, 1e3)
    .build();

    commands
        .entity(wheel)
        .insert(ImpulseJoint::new(cube, joint));
}
