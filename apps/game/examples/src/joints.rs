use std::f32::consts::PI;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    Collider, ColliderMassProperties, GenericJoint, GenericJointBuilder, ImpulseJoint,
    JointAxesMask, JointAxis, MultibodyJoint, RigidBody,
};

use crate::Configuration;

use super::JointsPlugin;

impl Plugin for JointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

#[derive(Component)]
struct WheelJoint;

#[derive(Component)]
struct UprightJoint;

fn update(config: Res<Configuration>, mut q: Query<&mut MultibodyJoint, With<WheelJoint>>) {
    for mut joint in q.iter_mut() {
        joint
            .data
            .set_motor_velocity(JointAxis::AngX, config.wheel_vel, 1.);
    }
}

fn make_joint(
    locked_axes: JointAxesMask,
    parent_axis: Vec3,
    child_axis: Vec3,
    parent_anchor: Vec3,
    child_anchor: Vec3,
    // (pos, vel, stiffness, damping)
    motor: (f32, f32, f32, f32),
) -> GenericJoint {
    let unlocked_axis = if (JointAxesMask::all() - locked_axes).contains(JointAxesMask::ANG_X) {
        Some(JointAxis::AngX)
    } else if (JointAxesMask::all() - locked_axes).contains(JointAxesMask::ANG_Y) {
        Some(JointAxis::AngY)
    } else if (JointAxesMask::all() - locked_axes).contains(JointAxesMask::ANG_Z) {
        Some(JointAxis::AngZ)
    } else {
        None
    };
    let mut builder = GenericJointBuilder::new(locked_axes)
        .local_axis2(parent_axis)
        .local_axis1(child_axis)
        .local_anchor2(parent_anchor)
        .local_anchor1(child_anchor);
    if let Some(u_a) = unlocked_axis {
        builder = builder.set_motor(u_a, motor.0, motor.1, motor.2, motor.3);
    }
    builder.build()
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1. }));
    let upright_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.4 }));
    let wheel_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.5,
        height: 1.,
        ..default()
    }));

    let body = commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_xyz(-1.2, 1.6, 0.),
            ..default()
        })
        .insert(Name::new("Body"))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .id();

    let upright = commands
        .spawn(PbrBundle {
            mesh: upright_mesh.clone(),
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.2, 0.2, 0.2))
        .insert(ColliderMassProperties::Density(4.))
        .id();

    let wheel = commands
        .spawn(PbrBundle {
            mesh: wheel_mesh.clone(),
            transform: Transform {
                translation: Vec3::new(1.2, 1., 0.),
                rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 2.),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(0.5, 0.5))
        .id();

    // Wheel - Upright Joint
    let wheel_joint = make_joint(
        JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
        -Vec3::Y,
        Vec3::X,
        Vec3::new(0., 0.6, 0.),
        Vec3::new(0.6, 0., 0.),
        (0., 5., 0., 0.),
    );

    commands
        .entity(wheel)
        // with this the motor doesn't work
        .insert((MultibodyJoint::new(upright, wheel_joint), WheelJoint));
    // with this the motor works
    // .insert(ImpulseJoint::new(upright, wheel_joint));

    // Upright - Body Joint
    let upright_joint = make_joint(
        JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
        Vec3::Y,
        Vec3::Y,
        Vec3::new(0., 0., 0.),
        Vec3::new(1.2, 0., 0.),
        (0., 0., 0., 0.),
    );

    commands
        .entity(upright)
        .insert((ImpulseJoint::new(body, upright_joint), UprightJoint));
}
