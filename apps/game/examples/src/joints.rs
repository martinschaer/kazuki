use std::f32::consts::PI;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    Collider, ColliderMassProperties, ImpulseJoint, MultibodyJoint, RigidBody,
};

use super::JointsPlugin;
use crate::car::{
    dynamics::{
        suspension::{
            make_front_upright_chasis_joint, make_front_upright_wheel_joint, update_upright,
            update_wheel,
        },
        UprightJoint, WheelJoint,
    },
    Upright,
};
use crate::Configuration;

impl Plugin for JointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_wheel, update_upright));
    }
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
