use bevy::{
    core::TaskPoolThreadAssignmentPolicy, prelude::*, tasks::available_parallelism,
    window::PresentMode,
};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, window::WindowResized};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::plugins::{CameraType, CarPlugin, ControlsPlugin, CubesPlugin, MainScenePlugin};

pub fn run() {
    let mut app = App::new();
    app.add_plugins(
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
            .set(AssetPlugin::default())
            .set(TaskPoolPlugin {
                task_pool_options: TaskPoolOptions {
                    compute: TaskPoolThreadAssignmentPolicy {
                        // set the minimum # of compute threads
                        // to the total number of available threads
                        min_threads: available_parallelism(),
                        max_threads: std::usize::MAX, // unlimited max threads
                        percent: 1.0,                 // this value is irrelevant in this case
                    },
                    // keep the defaults for everything else
                    ..default()
                },
            }),
    )
    .add_plugins(MainScenePlugin {
        camera_type: CameraType::Follow,
    })
    .add_plugins(CarPlugin)
    .add_plugins(ControlsPlugin)
    .add_plugins(CubesPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(FrameTimeDiagnosticsPlugin)
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_systems(Update, on_resize_system);

    #[cfg(feature = "inspector")]
    {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.read() {
        println!("{:.1} x {:.1}", e.width, e.height);
    }
}
