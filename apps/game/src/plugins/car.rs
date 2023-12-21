use bevy::{
    app::{App, Plugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::car::{
    dynamics::suspension::{system_rear_axle_motor, system_update_upright_steering},
    objects::wheels::spawn_wheel,
    CarSpecs,
};
use crate::plugins::{CarPlugin, GROUP_BODY, GROUP_SURFACE};

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CarSpecs>()
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut car_specs: ResMut<CarSpecs>,
) {
    let car_transform = Transform::from_xyz(-1., 2., -3.);
    let body_mat = materials.add(Color::hsla(60.0, 0.0, 0.5, 0.5).into());
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

    // calculate car mass
    car_specs.mass -= 4. * (car_specs.wheel_mass + car_specs.upright_mass);

    // body
    let body_entity = commands
        .spawn(PbrBundle {
            mesh: body_mesh,
            material: body_mat,
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
        // TODO: give wheel material a name and don't pass it to spawn_wheel
        let material = materials.add(Color::hsl(90. * i as f32, 1.0, 0.5).into());
        spawn_wheel(
            material,
            &mut commands,
            &mut meshes,
            &car_specs,
            body_entity,
            *anchor,
            i,
        );
    }
}
