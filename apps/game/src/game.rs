use bevy::window::WindowResized;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::plugins::{CameraType, CarPlugin, ControlsPlugin, CubesPlugin, MainScenePlugin};

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Kazuki!".into(),
                        resolution: (640., 480.).into(),
                        present_mode: PresentMode::AutoVsync,
                        canvas: Some("main canvas".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin::default()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(MainScenePlugin {
            camera_type: CameraType::Orthographic,
        })
        .add_plugins(CarPlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(CubesPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, on_resize_system)
        .run();
}

fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.read() {
        println!("{:.1} x {:.1}", e.width, e.height);
    }
}
