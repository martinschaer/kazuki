use bevy::{
    app::{App, Plugin},
    prelude::*,
};
use bevy_rapier3d::{
    geometry::ColliderMassProperties,
    prelude::{Collider, ImpulseJoint, MultibodyJoint, RapierConfiguration, RigidBody},
};

use super::JointsPlugin;
use crate::car::{
    dynamics::{
        suspension::{
            make_front_upright_chasis_joint, make_front_upright_wheel_joint, system_update_upright,
            system_update_upright_joint, system_update_wheel,
        },
        UprightJoint, WheelJoint,
    },
    objects::wheels::get_suspension_geometry,
    Upright,
};
use crate::Configuration;

impl Plugin for JointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                system_update_wheel,
                system_update_upright,
                system_update_upright_joint,
                update_physics_active,
            ),
        );
    }
}

fn update_physics_active(
    config: Res<Configuration>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.physics_pipeline_active = config.enable_physics;
}

struct SuspensionParams {
    body_w: f32,
    upright_w: f32,
    is_left: bool,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut config: ResMut<Configuration>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body_w = 1.;
    let upright_w = 0.4;
    let wheel_w = 1.;

    let mesh = meshes.add(Mesh::from(shape::Cube { size: body_w }));
    let upright_mesh = meshes.add(Mesh::from(shape::Cube { size: upright_w }));
    let wheel_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: wheel_w * 0.5,
        height: wheel_w,
        ..default()
    }));
    let material = materials.add(Color::hsla(60.0, 0.0, 0.5, 0.5).into());

    config.wheel_offset = 0.7;
    config.wheel_vel = 5.;
    config.upright_offset = 0.1;
    config.steering_angle = 0.;
    config.enable_physics = false;

    spawn_wheel(
        &mut commands,
        &config,
        mesh.clone(),
        upright_mesh.clone(),
        wheel_mesh.clone(),
        material.clone(),
        SuspensionParams {
            body_w,
            upright_w,
            is_left: true,
        },
    );

    spawn_wheel(
        &mut commands,
        &config,
        mesh,
        upright_mesh,
        wheel_mesh,
        material,
        SuspensionParams {
            body_w,
            upright_w,
            is_left: false,
        },
    );
}

fn spawn_wheel(
    commands: &mut Commands,
    config: &Configuration,
    mesh: Handle<Mesh>,
    upright_mesh: Handle<Mesh>,
    wheel_mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    params: SuspensionParams,
) {
    let body_pos_x = if params.is_left {
        params.body_w * -0.5
    } else {
        params.body_w * 0.5
    };
    let body_pos_y = 1.6;
    let mut anchor = params.body_w * 0.5 + params.upright_w * 0.5;
    if params.is_left {
        anchor *= -1.;
    }

    let upright_offset_relative = if params.is_left {
        -config.upright_offset
    } else {
        config.upright_offset
    };
    let ((upright_translation, upright_rotation), (wheel_translation, wheel_rotation)) =
        get_suspension_geometry(
            params.is_left,
            upright_offset_relative,
            config.wheel_offset,
            body_pos_x,
            body_pos_y,
            params.body_w,
            anchor,
        );

    let body = commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(body_pos_x, body_pos_y, 0.),
            ..default()
        })
        .insert(Name::new("Body"))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .id();

    let upright = commands
        .spawn(PbrBundle {
            mesh: upright_mesh.clone(),
            material: material.clone(),
            transform: Transform {
                translation: upright_translation,
                rotation: upright_rotation,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.2, 0.2, 0.2))
        .insert(ColliderMassProperties::Density(4.))
        .insert(Upright {
            is_front: true,
            is_left: params.is_left,
        })
        .id();

    let wheel = commands
        .spawn(PbrBundle {
            mesh: wheel_mesh.clone(),
            material,
            transform: Transform {
                translation: wheel_translation,
                rotation: wheel_rotation,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(0.5, 0.5))
        .id();

    // Wheel - Upright Joint
    let wheel_joint = make_front_upright_wheel_joint(config.wheel_offset, params.is_left);

    commands.entity(wheel).insert((
        MultibodyJoint::new(upright, wheel_joint),
        WheelJoint {
            is_front: true,
            is_left: params.is_left,
        },
    ));

    // Upright - Body Joint
    let upright_joint = make_front_upright_chasis_joint(
        Vec3::new(anchor, 0., 0.),
        upright_offset_relative,
        [-1., 0.],
        false,
    );

    commands.entity(upright).insert((
        ImpulseJoint::new(body, upright_joint),
        UprightJoint {
            is_left: params.is_left,
            is_front: true,
        },
    ));
}
