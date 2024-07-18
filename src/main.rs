use std::f32::consts::PI;

use bevy::{color::palettes::css::ORANGE_RED, pbr::CascadeShadowConfigBuilder, prelude::*};

use ground::build_ground;

mod ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground plane
    commands.spawn(PbrBundle {
        // mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(5., 5.))),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(build_ground()),
        material: materials.add(Color::srgb_u8(0, 125, 125)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 0.02,
    });

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
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

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
