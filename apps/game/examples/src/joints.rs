use std::f32::consts::PI;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    Collider, GenericJoint, GenericJointBuilder, ImpulseJoint, JointAxesMask, JointAxis, RigidBody,
};

use super::JointsPlugin;

impl Plugin for JointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn make_wheel_joint(
    locked_axes: JointAxesMask,
    parent_axis: Vec3,
    child_axis: Vec3,
    parent_anchor: Vec3,
    child_anchor: Vec3,
    // (pos, vel, stiffness, damping)
    motor: (f32, f32, f32, f32),
) -> GenericJoint {
    let unlocked_axis = if (JointAxesMask::all() - locked_axes).contains(JointAxesMask::ANG_X) {
        JointAxis::AngX
    } else if (JointAxesMask::all() - locked_axes).contains(JointAxesMask::ANG_Y) {
        JointAxis::AngY
    } else {
        JointAxis::AngZ
    };
    GenericJointBuilder::new(locked_axes)
        .local_axis2(parent_axis)
        .local_axis1(child_axis)
        .local_anchor2(parent_anchor)
        .local_anchor1(child_anchor)
        .set_motor(unlocked_axis, motor.0, motor.1, motor.2, motor.3)
        .build()
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
    let wheel_joint = make_wheel_joint(
        JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
        -Vec3::Y,
        Vec3::X,
        Vec3::new(0., 0.6, 0.),
        Vec3::new(0.6, 0., 0.),
        (0., 1e2, 1e6, 1e3),
    );

    commands
        .entity(wheel)
        .insert(ImpulseJoint::new(cube, wheel_joint));
}
