use bevy::{
    app::{App, Plugin},
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;

use super::MainScenePlugin;
use crate::car::Body;
use crate::plugins::{CameraType, GROUP_BODY, GROUP_SURFACE, GROUP_WHEEL};

#[derive(Component)]
struct DebugText;

impl Plugin for MainScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Startup, setup_3d)
            .add_systems(Update, text_update_system);
        match self.camera_type {
            CameraType::Follow => {
                app.add_systems(Startup, setup_camera)
                    .add_systems(Update, system_cam_follow);
            }
            CameraType::Fly => {
                app.add_plugins(NoCameraPlayerPlugin)
                    .add_systems(Startup, setup_fly_camera);
            }
        };
    }
}

fn system_cam_follow(
    mut q_c: Query<&mut Transform, (With<Camera3d>, Without<Body>)>,
    q_b: Query<&Transform, With<Body>>,
) {
    if let Ok(mut cam_transform) = q_c.get_single_mut() {
        if let Ok(body_transform) = q_b.get_single() {
            cam_transform.look_at(body_transform.translation, Vec3::Y);
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Hack-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::hsl(120., 0.5, 0.1),
    };

    // 2d text
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Kazuki", text_style.clone()).clone(),
            ..default()
        })
        .insert(Name::new("Kazuki Title"));

    // UI text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Debug: ",
                TextStyle {
                    font,
                    font_size: 10.0,
                    color: Color::hsl(60., 0.5, 0.1),
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Hack-Regular.ttf"),
                font_size: 10.0,
                color: Color::hsl(120., 0.5, 0.1),
            }),
        ]),
        DebugText,
        Name::new("DebugText"),
    ));
}

fn setup_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(128.0).into()),
            material: materials.add(Color::hsla(180.0, 0.5, 0.95, 0.1).into()),
            ..default()
        })
        .insert(Name::new("Floor"))
        .with_children(|children| {
            children
                .spawn(RigidBody::Fixed)
                .insert(Collider::cuboid(64., 0.1, 64.))
                .insert(CollisionGroups::new(
                    bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_SURFACE),
                    bevy_rapier3d::geometry::Group::from_bits_truncate(GROUP_WHEEL | GROUP_BODY),
                ))
                .insert(Friction::new(1.))
                .insert(TransformBundle::from(Transform::from_xyz(0., -0.05, 0.)));
        });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 5.0,
    });

    // directional light
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         // shadows_enabled: true,
    //         illuminance: 100000.,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_xyzw(-PI / 8., 0., -PI / 8., 1.),
    //         ..default()
    //     },
    //     // The default cascade config is designed to handle large scenes.
    //     // As this example has a much smaller world, we can tighten the shadow
    //     // bounds for better visual quality.
    //     cascade_shadow_config: CascadeShadowConfigBuilder {
    //         first_cascade_far_bound: 4.0,
    //         maximum_distance: 10.0,
    //         ..default()
    //     }
    //     .into(),
    //     ..default()
    // });
}

fn setup_camera(
    mut commands: Commands,
    // windows: Query<&Window>,
    // mut images: ResMut<Assets<Image>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
) {
    // Postprocessing
    /*
    let window = windows.single();
    let size = bevy::render::render_resource::Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    // texture that will be rendered to
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let image_handle = images.add(image);
    */

    // Camera
    commands.spawn((
        Camera3dBundle {
            // projection: OrthographicProjection {
            //     scale: 5.0,
            //     scaling_mode: ScalingMode::FixedVertical(2.0),
            //     ..default()
            // }
            // .into(),
            transform: Transform::from_xyz(-16.0, 16.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::BLACK,
                ),
                ..default()
            },
            camera: Camera {
                // Postprocessing
                // target: RenderTarget::Image(image_handle.clone()),
                order: 1,
                ..default()
            },
            ..default()
        },
        // UiCameraConfig { show_ui: false },
    ));

    // Without postprocessing
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            // Postprocessing
            // target: RenderTarget::Image(image_handle.clone()),
            order: 2,
            ..default()
        },
        ..default()
    });

    // Postprocessing
    /*
    let resolution = Vec2::new(size.width as f32, size.height as f32);
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(resolution)));
    let material_handle = post_processing_materials.add(PostProcessingMaterial {
        source_image: image_handle,
        time: 0.,
        intensity: 0.005,
    });
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            // material: toon_material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
    ));

    // The post-processing pass camera.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                order: 1,
                ..default()
            },
            ..Camera2dBundle::default()
        },
        post_processing_pass_layer,
    ));
    */
}

fn setup_fly_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 4.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::BLACK,
                ),
                ..default()
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        FlyCam,
        // UiCameraConfig { show_ui: false },
    ));
}

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<DebugText>>,
) {
    let mut fps = 0.0;
    for mut text in &mut query {
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }
        text.sections[1].value = format!("{fps:.2}");
    }
}

/*
fn material_animation_system(
    time: Res<Time>,
    post_material: Query<&Handle<PostProcessingMaterial>>,
    mut materials: ResMut<Assets<PostProcessingMaterial>>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    let mut mouse_delta = 0.;
    for mouse_event in mouse_events.iter() {
        mouse_delta = mouse_event.delta.length();
    }
    for mat_handle in &post_material {
        if let Some(m) = materials.get_mut(mat_handle) {
            m.time = time.elapsed_seconds();
            m.intensity = mouse_delta * 0.01;
        }
    }
}
*/

/*
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "759c427a-98fa-4545-966d-7a8da94ba40a"]
struct PostProcessingMaterial {
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
    #[uniform(2)]
    time: f32,
    #[uniform(3)]
    intensity: f32,
}

impl Material2d for PostProcessingMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom.vert".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/custom.frag".into()
    }
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &Hashed<InnerMeshVertexBufferLayout, FixedState>,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
*/
