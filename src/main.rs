use bevy::{
    color::palettes::css::ORANGE_RED,
    input::common_conditions::input_toggle_active,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use noise::permutationtable::PermutationTable;
use std::f32::consts::PI;

use ground::{build_ground, update_ground};

mod ground;

#[derive(Component)]
struct Ground;

#[derive(Resource)]
struct GroundParams {
    permutation_table: PermutationTable,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        // .add_plugins((DefaultPlugins, DevPlugins))
        .insert_resource(Parameters(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 125.0,
            sensitivity_iso: 100.0,
            sensor_height: 0.01866,
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, system_update_ground)
        .run();
}

fn setup(
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
            mesh: meshes.add(build_ground(10, 10, 10, 10)),
            // material: materials.add(Color::srgb_u8(0, 125, 125)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb_u8(0, 125, 125),
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Ground,
    ));

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
        transform: Transform::from_xyz(5.0, 2.5, 0.).looking_at(Vec3::ZERO, Vec3::Y),
        exposure: Exposure::from_physical_camera(**parameters),
        ..default()
    });
}

fn system_update_ground(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_query: Query<&Handle<Mesh>, With<Ground>>,
    res_ground_params: Res<GroundParams>,
) {
    let mesh_handle = mesh_query.get_single().expect("Query not successful");
    let mesh = meshes.get_mut(mesh_handle).unwrap();
    update_ground(
        mesh,
        time.elapsed_seconds(),
        &res_ground_params.permutation_table,
    );
}
