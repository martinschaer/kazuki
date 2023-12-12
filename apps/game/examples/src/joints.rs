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
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_wheel, update_upright));
    }
}

#[derive(Component)]
struct WheelJoint;

#[derive(Component)]
struct UprightJoint;

#[derive(Component)]
struct Upright;

fn update_wheel(config: Res<Configuration>, mut q: Query<&mut MultibodyJoint, With<WheelJoint>>) {
    for mut joint in q.iter_mut() {
        joint
            .data
            .set_motor_velocity(JointAxis::AngX, config.wheel_vel, 1.)
            .set_local_anchor2(Vec3::new(0., config.wheel_offset, 0.))
            .set_local_anchor1(Vec3::new(config.wheel_offset, 0., 0.));
    }
}

fn make_front_upright_wheel_joint(offset: f32) -> GenericJoint {
    let mut builder = GenericJointBuilder::new(
        JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
    )
    .local_axis2(-Vec3::Y)
    .local_axis1(Vec3::X)
    .local_anchor2(Vec3::new(0., offset, 0.))
    .local_anchor1(Vec3::new(offset, 0., 0.));
    let unlocked_axis = JointAxis::AngX;
    builder = builder.set_motor(unlocked_axis, 0., 5., 0., 0.);
    builder.build()
}

// Working
fn update_upright(config: Res<Configuration>, mut q: Query<&mut Transform, With<Upright>>) {
    for mut transform in q.iter_mut() {
        transform.rotation = Quat::from_rotation_y(config.steering_angle / 180. * PI);
    }
}
// TODO: why doesn't this work? is there another way to apply a rotational force?
// fn update_upright(config: Res<Configuration>, mut q: Query<&mut ImpulseJoint, With<UprightJoint>>) {
//     for mut joint in q.iter_mut() {
//         joint.data.set_motor(JointAxis::AngX, config.steering_angle / 180. * PI, 5., 0., 0.);
//     }
// }

fn make_front_upright_chasis_joint(offset: f32) -> GenericJoint {
    let builder = GenericJointBuilder::new(
        // JointAxesMask::X // this is the vertical axis
        JointAxesMask::Y
            | JointAxesMask::Z
            // | JointAxesMask::ANG_X // this is the rotation axis
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
    )
    .local_axis2(Vec3::Y)
    .local_axis1(Vec3::Y)
    .local_anchor2(Vec3::new(0., 0., 0.))
    .local_anchor1(Vec3::new(offset, 0., 0.))
    // .limits(JointAxis::AngX, [PI * -0.5, PI * 0.5])
    .limits(JointAxis::X, [-1., 0.]);
    builder.build()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut config: ResMut<Configuration>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1. }));
    let upright_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.4 }));
    let wheel_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.5,
        height: 1.,
        ..default()
    }));

    config.wheel_offset = 0.6;
    config.wheel_vel = 5.;
    config.upright_offset = 0.6;
    config.steering_angle = 0.; //15.;

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
        .insert(Upright)
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
    let wheel_joint = make_front_upright_wheel_joint(config.wheel_offset);

    commands
        .entity(wheel)
        .insert((MultibodyJoint::new(upright, wheel_joint), WheelJoint));

    // Upright - Body Joint
    let upright_joint = make_front_upright_chasis_joint(1.2);

    commands
        .entity(upright)
        .insert((ImpulseJoint::new(body, upright_joint), UprightJoint));
}
