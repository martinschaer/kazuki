use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

use crate::car::dynamics::suspension::{make_front_upright_chasis_joint, make_upright_wheel_joint};
use crate::car::dynamics::{UprightJoint, WheelJoint};
use crate::car::{CarSpecs, FrontWheel, RearWheel, Upright};
use crate::plugins::{GROUP_SURFACE, GROUP_WHEEL};

pub fn get_suspension_geometry(
    is_left: bool,
    upright_offset_relative: f32,
    wheel_offset_abs: f32,
    body_pos: Vec3,
    anchor: Vec3,
) -> ((Vec3, Quat), (Vec3, Quat)) {
    let upright_translation = Vec3::new(
        upright_offset_relative + body_pos.x + anchor.x,
        body_pos.y + anchor.y,
        body_pos.z + anchor.z,
    );
    let upright_rotation = Quat::IDENTITY;

    let (wheel_pos_x, wheel_rot_z) = if is_left {
        (upright_translation.x - wheel_offset_abs, 0.5 * PI)
    } else {
        (upright_translation.x + wheel_offset_abs, PI * -0.5)
    };
    let wheel_translation = Vec3::new(wheel_pos_x, upright_translation.y, upright_translation.z);
    let wheel_rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., wheel_rot_z);

    (
        (upright_translation, upright_rotation),
        (wheel_translation, wheel_rotation),
    )
}

pub fn spawn_wheel(
    car_transform: &Transform,
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    car_specs: &CarSpecs,
    body_entity: Entity,
    anchor: Vec3,
    wheel_num: usize,
) {
    let is_front = wheel_num / 2 == 0;
    let is_left = wheel_num % 2 == 0;
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

    let upright_mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -0.1,
        max_x: 0.1,
        min_y: car_specs.wheel_half_height * -0.5,
        max_y: car_specs.wheel_half_height * 0.5,
        min_z: -0.1,
        max_z: 0.1,
    }));
    let upright_collider = Collider::cuboid(0.1, car_specs.wheel_half_height * 0.5, 0.1);

    // Geometry
    let ((upright_translation, upright_rotation), (wheel_translation, wheel_rotation)) =
        get_suspension_geometry(is_left, 0., 0., car_transform.translation, anchor);

    // upright
    let upright_entity = commands
        .spawn(PbrBundle {
            mesh: upright_mesh,
            // material: suspension_mat_handle.clone(),
            material: material.clone(),
            transform: Transform {
                translation: upright_translation,
                rotation: upright_rotation,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Name::new(format!("upright_{}", wheel_num)))
        .insert(upright_collider)
        .insert(ColliderMassProperties::Mass(car_specs.upright_mass))
        // .insert(AdditionalMassProperties::Mass(car_specs.upright_mass))
        .insert(Upright { is_left, is_front })
        .id();

    // wheel
    let wheel_entity = commands
        .spawn(PbrBundle {
            mesh: wheel_mesh,
            material,
            transform: Transform {
                translation: wheel_translation,
                rotation: wheel_rotation,
                ..default()
            },
            ..default()
        })
        .insert(Name::new(format!("wheel_{}", wheel_num)))
        .insert(RigidBody::Dynamic)
        .insert(wheel_collider)
        // .insert(Ccd::enabled())
        .insert(ColliderMassProperties::Mass(car_specs.wheel_mass))
        .insert(CollisionGroups::new(
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_WHEEL),
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_SURFACE),
        ))
        // .insert(Restitution::coefficient(0.5))
        .id();

    if is_front {
        commands.entity(wheel_entity).insert(FrontWheel);
    } else {
        commands.entity(wheel_entity).insert(RearWheel);
    }

    // Wheel - Upright Joint
    let wheel_joint = make_upright_wheel_joint(car_specs.wheel_offset, is_left);

    commands.entity(wheel_entity).insert((
        ImpulseJoint::new(upright_entity, wheel_joint),
        WheelJoint { is_left, is_front },
    ));

    // Upright - Body Joint
    let upright_joint = make_front_upright_chasis_joint(anchor, 0., [-1., 0.], !is_front);

    commands.entity(upright_entity).insert((
        ImpulseJoint::new(body_entity, upright_joint),
        UprightJoint { is_left, is_front },
    ));
}
