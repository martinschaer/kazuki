use crate::plugins::CarPlugin;
use crate::plugins::MainScenePlugin;
use bevy::window::WindowResized;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

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
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MainScenePlugin)
        .add_plugin(CarPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,
            ..default()
        })
        .add_system(on_resize_system)
        .run();
}

fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.iter() {
        println!("{:.1} x {:.1}", e.width, e.height);
    }
}
