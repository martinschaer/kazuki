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
    mass: f32,
    wheel_mass: f32,
    upright_mass: f32,
}

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        let mut car_specs = CarSpecs {
            height: 0.95,
            length: 5.5,
            width: 2.,
            wheel_half_height: 0.4,
            wheel_diameter: 0.72,
            mass: 796.,
            wheel_mass: 2.5,
            upright_mass: 2.5,
        };
        car_specs.mass -= 4. * (car_specs.wheel_mass + car_specs.upright_mass);
        app.insert_resource(car_specs)
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
    let car_transform = Transform::from_xyz(1., 2., -1.).with_rotation(Quat::from_euler(
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
        // TODO: check if this is the total mass and not added to the guessed one from the collider
        .insert(ColliderMassProperties::Mass(car_specs.mass))
        .insert(CollisionGroups::new(
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_BODY),
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_BODY | GROUP_SURFACE),
        ))
        .insert(Name::new("car_body"))
        .id();

    // wheels
    let wheels_anchors = [
        Vec3::new(
            car_specs.width * -0.5,
            car_specs.height * -0.5,
            car_specs.length * -0.3,
        ),
        Vec3::new(
            car_specs.width * 0.5,
            car_specs.height * -0.5,
            car_specs.length * -0.3,
        ),
        Vec3::new(
            car_specs.width * -0.5,
            car_specs.height * -0.5,
            car_specs.length * 0.4,
        ),
        Vec3::new(
            car_specs.width * 0.5,
            car_specs.height * -0.5,
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

fn steering(controls: Res<ControlsState>, mut query: Query<(&Upright, &mut ImpulseJoint)>) {
    let turning_degrees = 90.;
    let steering_wheel_degrees_range = 900.;
    let angle = (turning_degrees / 360.) * 2. * PI * controls.steering_wheel_degrees
        / steering_wheel_degrees_range;
    println!("angle: {}", angle);
    for (_, mut joint) in query.iter_mut() {
        joint.data.set_motor_position(JointAxis::Y, angle, 1e9, 1e3);
    }
}
