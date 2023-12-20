use bevy::prelude::*;

pub mod suspension;

#[derive(Component)]
pub struct WheelJoint {
    pub is_left: bool,
    pub is_front: bool,
}

#[derive(Component)]
pub struct UprightJoint {
    pub is_left: bool,
    pub is_front: bool,
}

