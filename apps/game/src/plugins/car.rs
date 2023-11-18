use crate::plugins::controls::ControlsState;
use std::f32::consts::{FRAC_PI_2, PI};

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
struct Upright;

#[derive(Component)]
struct FrontWheel;

#[derive(Component)]
struct RearWheel;

#[derive(Resource)]
struct CarSpecs {
    height: f32,
    width: f32,
    length: f32,
    wheel_half_height: f32,
    wheel_diameter: f32,
    // mass: f32,
}

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CarSpecs {
            height: 0.95,
            length: 5.5,
            width: 2.,
            wheel_half_height: 0.4,
            wheel_diameter: 0.72,
            // mass: 796.,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, steering);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    car_specs: Res<CarSpecs>,
) {
    let car_transform = Transform::from_xyz(1., 3., -1.).with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        0.,
        FRAC_PI_2,
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
            car_specs.width * -0.5,
            -car_specs.height,
            car_specs.length * -0.3,
        ),
        Vec3::new(
            car_specs.width * 0.5,
            -car_specs.height,
            car_specs.length * -0.3,
        ),
        Vec3::new(
            car_specs.width * -0.5,
            -car_specs.height,
            car_specs.length * 0.4,
        ),
        Vec3::new(
            car_specs.width * 0.5,
            -car_specs.height,
            car_specs.length * 0.4,
        ),
    ];
    for (i, anchor) in wheels_anchors.iter().enumerate() {
        // TODO: give wheel material a name and don't pass is to spawn_wheel
        let material = materials.add(Color::hsl(90. * i as f32, 1.0, 0.5).into());
        spawn_wheel(
            material,
            &mut commands,
            &mut meshes,
            &car_specs,
            body_entity,
            // car_transform,
            *anchor,
            i,
        );
    }
}

fn spawn_wheel(
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

    let axel_joint = GenericJointBuilder::new(JointAxesMask::LOCKED_REVOLUTE_AXES)
        .local_axis1(Vec3::X)
        .local_axis2(if wheel_num % 2 == 0 {
            Vec3::Y
        } else {
            -Vec3::Y
        })
        // .local_basis1(Quat::from_axis_angle(Vec3::Y, 0.)) // hackfix, prevents jumping on collider edges
        .local_anchor1(Vec3::new(
            if wheel_num % 2 == 0 {
                -car_specs.wheel_half_height
            } else {
                car_specs.wheel_half_height
            },
            0.,
            0.,
        ))
        .local_anchor2(Vec3::new(0., 0., 0.))
        // .set_motor(JointAxis::Y, 0., 0., 1e6, 1e3)
        .build();

    // let joint = FixedJointBuilder::new().local_anchor1(anchor);
    let upright_joint = GenericJointBuilder::new(JointAxesMask::LOCKED_REVOLUTE_AXES)
        .local_axis2(Vec3::Y)
        .local_axis1(Vec3::Y)
        .local_anchor2(Vec3::new(0., 0., 0.))
        .local_anchor1(anchor)
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
            // transform: Transform::from_translation(
            //     car_transform.translation + car_transform.rotation.mul_vec3(anchor),
            // ),
            // transform: Transform::from_translation(anchor),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(AdditionalMassProperties::Mass(0.2))
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
            // translation: Vec3::new(if wheel_num % 2 == 0 { -0.4 } else { 0.4 }, 0., 0.),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(wheel_collider)
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

fn steering(controls: Res<ControlsState>, mut query: Query<(&Upright, &mut Transform)>) {
    let _ = &dbg!(controls.steering_wheel_degrees);
    let turning_degrees = 90.;
    let steering_wheel_degrees_range = 900.;
    for (_, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            0.,
            (turning_degrees / 360.) * 2. * PI * controls.steering_wheel_degrees
                / steering_wheel_degrees_range,
            0.,
        );
    }
}
