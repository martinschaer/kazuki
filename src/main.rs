use bevy::{
    input::common_conditions::input_toggle_active,
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        CascadeShadowConfigBuilder,
    },
    prelude::*,
    render::{
        camera::{Exposure, PhysicalCameraParameters},
        mesh::VertexAttributeValues,
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::geometry::{Collider, Sensor};
use noise::permutationtable::PermutationTable;
use std::f32::consts::PI;

use ground::{build_ground, update_ground};

mod ground;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Cursor;

#[derive(Resource)]
struct GroundParams {
    permutation_table: PermutationTable,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin))
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
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::WHITE.into(),
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                system_update_ground,
                system_update_cursor,
                system_update_cursor_collisions,
            ),
        )
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
            mesh: meshes.add(build_ground(20, 20, 200, 200)),
            // material: materials.add(Color::srgb_u8(0, 125, 125)),
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

fn system_update_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();
    let mut cursor = cursor_query.single_mut();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    // gizmos.circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);

    // Update the cursor's position.
    cursor.translation = point;
}

fn system_update_cursor_collisions(
    mut meshes: ResMut<Assets<Mesh>>,
    cursor_query: Query<(&Collider, &Transform), With<Cursor>>,
    ground_query: Query<&Handle<Mesh>, With<Ground>>,
) {
    let (cursor_collider, transform) = cursor_query.single();

    let ground_handle = ground_query.single();
    let ground_mesh = meshes.get_mut(ground_handle).unwrap();
    let ground_vertices = ground_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

    let mut vertices_intersected = Vec::new();
    let mut vertices_not_intersected = Vec::new();

    if let VertexAttributeValues::Float32x3(v) = ground_vertices {
        for (i, vertex) in v.iter().enumerate() {
            if cursor_collider.contains_point(
                transform.translation,
                transform.rotation,
                Vec3::from(vertex.clone()),
            ) {
                vertices_intersected.push(i);
            } else {
                vertices_not_intersected.push(i);
            }
        }
    }

    let ground_vertices_color = ground_mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR).unwrap();
    if let VertexAttributeValues::Float32x4(v) = ground_vertices_color {
        for i in vertices_intersected {
            v[i] = [0., 1., 0.8, 0.];
        }
        for i in vertices_not_intersected {
            v[i] = [1., 0.2, 0., 0.];
        }
    }
}
