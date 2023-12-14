mod car;
pub mod controls;
mod cubes;
mod main_scene;

pub struct CarPlugin;
pub struct ControlsPlugin;
pub struct CubesPlugin;
pub struct MainScenePlugin;

pub const GROUP_SURFACE: u32 = 0b01;
pub const GROUP_BODY: u32 = 0b10;
pub const GROUP_WHEEL: u32 = 0b100;
