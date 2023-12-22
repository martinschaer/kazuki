use bevy::{
    app::{App, Plugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::car::{
    dynamics::suspension::{system_rear_axle_motor, system_update_upright_steering},
    objects::wheels::spawn_wheel,
    Body, CarMatMeshColliderHandles, CarSpecs,
};
use crate::plugins::{CarPlugin, GROUP_BODY, GROUP_SURFACE};

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CarSpecs>()
            .init_resource::<CarMatMeshColliderHandles>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (system_update_upright_steering, system_rear_axle_motor),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut car_specs: ResMut<CarSpecs>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut car_handles: ResMut<CarMatMeshColliderHandles>,
) {
    let car_transform = Transform::from_xyz(-1., 2., -3.);
    let body_mesh = meshes.add(Mesh::from(shape::Box {
        min_x: car_specs.width / -2.,
        max_x: car_specs.width / 2.,
        min_y: car_specs.height / -2.,
        max_y: car_specs.height / 2.,
        min_z: car_specs.length / -2.,
        max_z: car_specs.length / 2.,
    }));
    let body_collider = Collider::cuboid(
        car_specs.width / 2.,
        car_specs.height / 2.,
        car_specs.length / 2.,
    );

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
    car_handles.wheel = wheel_mesh;
    car_handles.wheel_collider = wheel_collider;

    let upright_mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -0.1,
        max_x: 0.1,
        min_y: car_specs.wheel_half_height * -0.5,
        max_y: car_specs.wheel_half_height * 0.5,
        min_z: -0.1,
        max_z: 0.1,
    }));
    let upright_collider = Collider::cuboid(0.1, car_specs.wheel_half_height * 0.5, 0.1);
    car_handles.upright = upright_mesh;
    car_handles.upright_collider = upright_collider;

    // material
    car_handles.material = materials.add(Color::hsla(60.0, 0.0, 0.5, 0.5).into());

    // calculate car mass
    car_specs.mass -= 4. * (car_specs.wheel_mass + car_specs.upright_mass);

    // body
    let body_entity = commands
        .spawn(PbrBundle {
            mesh: body_mesh,
            material: car_handles.material.clone(),
            transform: car_transform,
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(body_collider)
        // TODO: check if this is the total mass and not added to the guessed one from the collider
        // .insert(ColliderMassProperties::Mass(car_specs.mass))
        .insert(CollisionGroups::new(
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_BODY),
            bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_BODY | GROUP_SURFACE),
        ))
        .insert(Name::new("Body"))
        .insert(Body)
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
        spawn_wheel(
            &car_transform,
            &car_handles,
            &mut commands,
            &car_specs,
            body_entity,
            *anchor,
            i,
        );
    }
}
