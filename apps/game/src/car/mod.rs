use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub mod dynamics;
pub mod objects;

#[derive(Component)]
pub struct Upright {
    pub is_left: bool,
    pub is_front: bool,
}

// TODO: refactor to Wheel { is_front, is_left } unless its more performant as different Components
// for querying
#[derive(Component)]
pub struct FrontWheel;

#[derive(Component)]
pub struct RearWheel;

#[derive(Component)]
pub struct Body;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    pub wheel_vel: f32,
    pub wheel_offset: f32,
    pub upright_offset: f32,
    pub steering_angle: f32,
    pub enable_physics: bool,
}

#[derive(Resource)]
pub struct CarSpecs {
    pub height: f32,
    pub width: f32,
    pub length: f32,
    pub wheel_half_height: f32,
    pub wheel_diameter: f32,
    pub wheel_offset: f32,
    pub mass: f32,
    pub wheel_mass: f32,
    pub upright_mass: f32,
}

impl Default for CarSpecs {
    fn default() -> Self {
        CarSpecs {
            height: 0.95,
            length: 5.5,
            width: 2.,
            wheel_half_height: 0.4,
            wheel_diameter: 0.72,
            wheel_offset: 0.2,
            mass: 796.,
            wheel_mass: 2.5,
            upright_mass: 2.5,
        }
    }
}

#[derive(Resource, Default)]
pub struct CarMatMeshColliderHandles {
    pub material: Handle<StandardMaterial>,
    pub wheel: Handle<Mesh>,
    pub upright: Handle<Mesh>,
    pub wheel_collider: Collider,
    pub upright_collider: Collider,
}
