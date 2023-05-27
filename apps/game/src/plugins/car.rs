use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin},
    prelude::*,
};
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxesMask};

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
    let car_half_size = 0.75;
    let car_transform = Transform::from_xyz(1., 3., -1.).with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        0.,
        PI / 2.,
        0.,
    ));

    // body
    let body_entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: car_half_size * 2.,
            })),
            material: materials.add(Color::hsla(60.0, 0.0, 0.5, 0.5).into()),
            transform: car_transform,
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            car_half_size,
            car_half_size,
            car_half_size,
        ))
        .id();

    // wheels
    let wheels_anchors = [
        Vec3::new(-1.2, -car_half_size, -car_half_size),
        Vec3::new(1.2, -car_half_size, -car_half_size),
        Vec3::new(-1.2, -car_half_size, car_half_size),
        Vec3::new(1.2, -car_half_size, car_half_size),
    ];
    for (i, anchor) in wheels_anchors.iter().enumerate() {
        let material = materials.add(Color::hsl(90. * i as f32, 1.0, 0.5).into());
        spawn_wheel(
            material,
            &mut commands,
            &mut meshes,
            body_entity,
            car_transform,
            *anchor,
            i % 2 == 0,
        );
    }
}

fn spawn_wheel(
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    body_entity: Entity,
    car_transform: Transform,
    anchor: Vec3,
    is_left: bool,
) {
    let wheel_half_height = 0.2;
    let wheel_radius = 0.4;
    let wheel_border_radius = 0.05;
    let wheel_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: wheel_radius,
        height: wheel_half_height,
        ..default()
    }));

    let wheel_collider = Collider::round_cylinder(
        wheel_half_height - (wheel_border_radius * 2.),
        wheel_radius - wheel_border_radius,
        wheel_border_radius,
    );

    let joint = GenericJointBuilder::new(JointAxesMask::LOCKED_REVOLUTE_AXES)
        .local_axis1(Vec3::X)
        .local_axis2(match is_left {
            true => Vec3::Y,
            false => -Vec3::Y,
        })
        // .local_basis1(Quat::from_axis_angle(Vec3::Y, 0.)) // hackfix, prevents jumping on collider edges
        .local_anchor1(anchor)
        .local_anchor2(Vec3::new(
            0.,
            wheel_half_height + wheel_border_radius + 0.1,
            0.,
        ))
        // .set_motor(JointAxis::Y, 0., 0., 1e6, 1e3)
        .build();

    commands
        .spawn(PbrBundle {
            mesh: wheel_mesh,
            material,
            ..default()
        })
        .insert(
            Transform::from_translation(
                car_transform.translation + car_transform.rotation.mul_vec3(anchor),
            ), // .with_rotation(
               //     car_transform.rotation
               //         * Quat::from_euler(EulerRot::XYZ, 0., 0., (90_f32).to_radians()),
               // ),
        )
        .insert(RigidBody::Dynamic)
        .insert(wheel_collider)
        .insert(Restitution::coefficient(0.5))
        .insert(ImpulseJoint::new(body_entity, joint));
}
