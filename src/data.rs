use bevy::{prelude::*, render::camera::PhysicalCameraParameters};
use noise::permutationtable::PermutationTable;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Cursor;

#[derive(Resource)]
pub struct GroundParams {
    pub permutation_table: PermutationTable,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Parameters(pub PhysicalCameraParameters);
