use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, render::camera::Exposure};
use bevy_rapier3d::geometry::{Collider, Sensor};
use kazuki::ground::build_ground;
use noise::permutationtable::PermutationTable;
use std::f32::consts::PI;

use crate::data::{Cursor, Ground, GroundParams, Parameters};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    parameters: Res<Parameters>,
) {
    // permutation table
    let permutation_table = PermutationTable::new(0);
    let ground_params = GroundParams { permutation_table };
    commands.insert_resource(ground_params);

    // ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(build_ground(20, 20, 200, 200)),
            // material: materials.add(Color::srgb_u8(255, 0, 0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb_u8(20, 211, 88),
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        // Wireframe,
        Ground,
    ));

    // ambient light
    // commands.insert_resource(AmbientLight {
    //     color: ORANGE_RED.into(),
    //     brightness: 0.02,
    // });

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // spot light
    // commands.spawn(SpotLightBundle {
    //     transform: Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
    //     spot_light: SpotLight {
    //         intensity: 100_000.0,
    //         color: Color::srgb_u8(255, 255, 255),
    //         shadows_enabled: false,
    //         inner_angle: 0.6,
    //         outer_angle: 0.8,
    //         ..default()
    //     },
    //     ..default()
    // });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 2.5, 0.).looking_at(Vec3::ZERO, Vec3::Y),
        exposure: Exposure::from_physical_camera(**parameters),
        ..default()
    });

    // sphere
    commands.spawn((
        // PbrBundle {
        //     mesh: meshes.add(Mesh::from(Sphere::new(1.).mesh().ico(1).unwrap())),
        //     material: materials.add(Color::srgba_u8(0, 0, 0, 0)),
        //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
        //     ..default()
        // },
        Transform::from_xyz(0.0, 0.5, 0.0),
        Collider::ball(1.),
        Sensor,
        // Wireframe,
        Cursor,
    ));
}
