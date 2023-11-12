use crate::plugins::controls::ControlsState;
use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin},
    prelude::*,
};
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxesMask};

use super::{
    main_scene::{GROUP_BODY, GROUP_SURFACE, GROUP_WHEEL},
    CarPlugin,
};

#[derive(Component)]
struct FrontWheel;

#[derive(Component)]
struct RearWheel;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, steering);
    }
}

struct CarSpecs {
    height: f32,
    width: f32,
    length: f32,
    wheel_half_height: f32,
    wheel_diameter: f32,
    // mass: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: make this a resource
    let car_specs = CarSpecs {
        height: 0.95,
        length: 5.5,
        width: 2.,
        wheel_half_height: 0.4,
        wheel_diameter: 0.72,
        // mass: 796.,
    };
    let car_transform = Transform::from_xyz(1., 3., -1.).with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        0.,
        PI / 2.,
        0.,
    ));

    // body
    let body_entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: car_specs.width / -2.,
                max_x: car_specs.width / 2.,
                min_y: car_specs.height / -2.,
                max_y: car_specs.height / 2.,
                min_z: car_specs.length / -2.,
                max_z: car_specs.length / 2.,
            })),
            material: materials.add(Color::hsla(60.0, 0.0, 0.5, 0.5).into()),
            transform: car_transform,
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            car_specs.width / 2.,
            car_specs.height / 2.,
            car_specs.length / 2.,
        ))
        .insert(CollisionGroups::new(
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_BODY),
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_BODY | GROUP_SURFACE),
        ))
        .id();

    // wheels
    let wheels_anchors = [
        Vec3::new(
            car_specs.width / -2. + car_specs.wheel_half_height * 2.,
            -car_specs.height + car_specs.wheel_diameter * 0.5,
            car_specs.length * -0.3,
        ),
        Vec3::new(
            car_specs.width / 2. - car_specs.wheel_half_height * 2.,
            -car_specs.height + car_specs.wheel_diameter * 0.5,
            car_specs.length * -0.3,
        ),
        Vec3::new(
            car_specs.width / -2. + car_specs.wheel_half_height * 2.,
            -car_specs.height + car_specs.wheel_diameter * 0.5,
            car_specs.length * 0.4,
        ),
        Vec3::new(
            car_specs.width / 2. - car_specs.wheel_half_height * 2.,
            -car_specs.height + car_specs.wheel_diameter * 0.5,
            car_specs.length * 0.4,
        ),
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
            i,
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
    wheel_num: usize,
) {
    // TODO: get from CarSpecs resource
    let wheel_half_height = 0.4;
    let wheel_diameter = 0.72;
    let wheel_border_radius = 0.1;
    let wheel_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: wheel_diameter / 2.,
        height: wheel_half_height,
        ..default()
    }));

    let wheel_collider = Collider::round_cylinder(
        wheel_half_height - (wheel_border_radius * 2.),
        wheel_diameter / 2. - wheel_border_radius,
        wheel_border_radius,
    );

    let joint = GenericJointBuilder::new(JointAxesMask::LOCKED_REVOLUTE_AXES)
        .local_axis1(Vec3::X)
        .local_axis2(if wheel_num % 2 == 0 {
            Vec3::Y
        } else {
            -Vec3::Y
        })
        // .local_basis1(Quat::from_axis_angle(Vec3::Y, 0.)) // hackfix, prevents jumping on collider edges
        .local_anchor1(anchor)
        .local_anchor2(Vec3::new(
            0.,
            wheel_half_height + wheel_border_radius + 0.,
            0.,
        ))
        // .set_motor(JointAxis::Y, 0., 0., 1e6, 1e3)
        .build();

    let wheel_id = commands
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
        .insert(CollisionGroups::new(
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_WHEEL),
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_SURFACE),
        ))
        // .insert(Restitution::coefficient(0.5))
        .insert(ImpulseJoint::new(body_entity, joint))
        .id();

    if wheel_num / 2 == 0 {
        commands.entity(wheel_id).insert(FrontWheel);
    } else {
        commands.entity(wheel_id).insert(RearWheel);
    }
}

fn steering(
    controls: Res<ControlsState>, /*, mut query: Query<(&FrontWheel, &mut ImpulseJoint)> */
) {
    let _ = &dbg!(controls.steering_wheel_degrees);
    // for (_, _) in query.iter_mut() {
    //     joint.data.set_local_axis1(Vec3::X * PI * controls.steering_wheel_degrees / 900.);
    // }
}
