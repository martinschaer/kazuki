use bevy::{
    input::common_conditions::input_toggle_active,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::camera::PhysicalCameraParameters,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use data::Parameters;
use scenes::terrain::{
    system_update_cursor, system_update_cursor_collisions, system_update_ground,
};
use setup::setup;

mod data;
mod ground;
mod scenes;
mod setup;

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
