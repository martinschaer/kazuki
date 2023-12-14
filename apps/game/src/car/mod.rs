use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

pub mod dynamics;

#[derive(Component)]
pub struct Upright;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    pub wheel_vel: f32,
    pub wheel_offset: f32,
    pub upright_offset: f32,
    pub steering_angle: f32,
}
