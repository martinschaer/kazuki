use bevy::app::{App, Plugin};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::ScalingMode,
    window::WindowResized,
};
use std::f32::consts::PI;

use super::MainScenePlugin;

#[derive(Component)]
struct DebugText;

#[derive(Component)]
struct Player {
    index: u8,
}

impl Plugin for MainScenePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
            .add_startup_system(setup)
            .add_startup_system(setup_3d)
            .add_startup_system(setup_camera)
            .add_system(text_update_system)
            .add_system(on_resize_system)
            // .add_system(material_animation_system)
            .add_system(cube_animation_system);
    }
}

fn cube_animation_system(time: Res<Time>, mut players: Query<(&mut Transform, &Player)>) {
    for (mut transform, player) in &mut players {
        transform.translation = Vec3::new(
            player.index as f32 - 2.5,
            0.25,
            (time.elapsed_seconds() + player.index as f32 * PI / 6.0).cos() + 1.25,
        );
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
    commands.spawn(Text2dBundle {
        text: Text::from_section("Kazuki", text_style.clone()).clone(),
        ..default()
    });

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
    ));
}

fn setup_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(16.0).into()),
        material: materials.add(Color::hsl(180.0, 0.5, 0.95).into()),
        ..default()
    });

    // cube
    for x in 0..6 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::hsl(60.0 * x as f32, 1.0, 0.5).into()),
                transform: Transform::from_xyz(-2.5 + 1.0 * x as f32, 0.25, 0.0),
                ..default()
            },
            Player { index: x },
        ));
    }

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 5.0,
    });

    // directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 100000.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_xyzw(-PI / 8., 0., -PI / 8., 1.),
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
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(-4.0, 4.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::BLACK,
                ),
                ..default()
            },
            camera: Camera {
                // Postprocessing
                // target: RenderTarget::Image(image_handle.clone()),
                order: 0,
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
            ..default()
        },
        camera: Camera {
            // Postprocessing
            // target: RenderTarget::Image(image_handle.clone()),
            order: 1,
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

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<DebugText>>) {
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

fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.iter() {
        println!("{:.1} x {:.1}", e.width, e.height);
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
