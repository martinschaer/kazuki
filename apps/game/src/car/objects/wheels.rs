use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxesMask};

use crate::car::{CarSpecs, FrontWheel, RearWheel, Upright};
use crate::plugins::{GROUP_SURFACE, GROUP_WHEEL};

pub fn spawn_wheel(
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    car_specs: &CarSpecs,
    body_entity: Entity,
    // car_transform: Transform,
    anchor: Vec3,
    wheel_num: usize,
) {
    let wheel_border_radius = 0.1;
    let wheel_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: car_specs.wheel_diameter / 2.,
        height: car_specs.wheel_half_height,
        ..default()
    }));

    let wheel_collider = Collider::round_cylinder(
        car_specs.wheel_half_height - (wheel_border_radius * 2.),
        car_specs.wheel_diameter / 2. - wheel_border_radius,
        wheel_border_radius,
    );

    let axel_joint = GenericJointBuilder::new(
        JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z
            | JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z,
    )
    .local_axis1(Vec3::X)
    // it may be not necessary to flip the axis
    .local_axis2(if wheel_num % 2 == 0 {
        Vec3::Y
    } else {
        -Vec3::Y
    })
    .local_anchor1(Vec3::new(
        if wheel_num % 2 == 0 {
            -car_specs.wheel_half_height
        } else {
            car_specs.wheel_half_height
        },
        0.,
        0.,
    ))
    .local_anchor2(Vec3::ZERO)
    .build();

    let upright_joint = GenericJointBuilder::new(
        JointAxesMask::ANG_X
            | JointAxesMask::ANG_Z
            | JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z,
    )
    .local_axis1(Vec3::Y)
    .local_axis2(Vec3::Y)
    .local_anchor1(anchor)
    .local_anchor2(Vec3::ZERO)
    .build();

    // upright
    let upright_entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -0.1,
                max_x: 0.1,
                min_y: car_specs.height / -2.,
                max_y: car_specs.height / 2.,
                min_z: -0.1,
                max_z: 0.1,
            })),
            // material: suspension_mat_handle.clone(),
            transform: Transform {
                // translation: car_transform.translation + car_transform.rotation.mul_vec3(anchor),
                rotation: Quat::from_euler(
                    EulerRot::XYZ,
                    0.,
                    if wheel_num % 2 == 0 {
                        PI * 2.5
                    } else {
                        PI * 0.5
                    },
                    0.,
                ),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Name::new(format!("upright_{}", wheel_num)))
        .insert(AdditionalMassProperties::Mass(car_specs.upright_mass))
        .insert(Upright)
        .insert(ImpulseJoint::new(body_entity, upright_joint))
        .id();

    // wheel
    let wheel_entity = commands
        .spawn(PbrBundle {
            mesh: wheel_mesh,
            material,
            ..default()
        })
        .insert(Transform {
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                0.,
                0.,
                (if wheel_num % 2 == 0 { 90_f32 } else { 270_f32 }).to_radians(),
            ),
            // translation: car_transform.translation + car_transform.rotation.mul_vec3(anchor),
            ..default()
        })
        .insert(Name::new(format!("wheel_{}", wheel_num)))
        .insert(RigidBody::Dynamic)
        .insert(wheel_collider)
        .insert(Ccd::enabled())
        .insert(ColliderMassProperties::Mass(car_specs.wheel_mass))
        .insert(CollisionGroups::new(
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_WHEEL),
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_SURFACE),
        ))
        // .insert(Restitution::coefficient(0.5))
        .insert(ImpulseJoint::new(upright_entity, axel_joint))
        .id();

    if wheel_num / 2 == 0 {
        commands.entity(wheel_entity).insert(FrontWheel);
    } else {
        commands.entity(wheel_entity).insert(RearWheel);
    }
}
