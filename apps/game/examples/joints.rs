#[path = "../src/car/mod.rs"]
mod car;
#[path = "../src/plugins/mod.rs"]
mod plugins;
mod src;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use plugins::MainScenePlugin;
use src::JointsPlugin;
use car::Configuration;

pub fn main() {
    App::new()
        .init_resource::<Configuration>()
        .register_type::<Configuration>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Joints".into(),
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
        .add_plugins(MainScenePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        .add_plugins(JointsPlugin)
        .run();
}
