use bevy::{
    app::{App, Plugin},
    input::InputSystem,
    prelude::*,
    window::PrimaryWindow,
};
use leafwing_input_manager::{
    axislike::DualAxisData, plugin::InputManagerSystem, prelude::*, systems::run_if_enabled,
};

use super::ControlsPlugin;

#[derive(Default, Resource)]
pub struct ControlsState {
    // Steering wheel is in the range [-450, 450]
    pub steering_wheel_degrees: f32,
    // Accelerator and brake are both in the range [0, 1]
    pub accelerator: f32,
    // brake: f32,
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Reflect)]
enum BoxMovement {
    MousePosition,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    TowToPits,
}

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<ControlsState>()
            .add_plugins(InputManagerPlugin::<BoxMovement>::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                update_cursor_state_from_window
                    .run_if(run_if_enabled::<BoxMovement>)
                    .in_set(InputManagerSystem::ManualControl)
                    .before(InputManagerSystem::ReleaseOnDisable)
                    .after(InputManagerSystem::Tick)
                    .after(InputManagerSystem::Update)
                    .after(InputSystem),
            )
            .add_systems(Update, turn_steering_wheel);
    }
}

fn setup(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 0,
            ..default()
        },
        ..default()
    });

    let entity = commands
        .spawn(SpriteBundle {
            transform: Transform::from_scale(Vec3::new(100., 100., 1.)),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(InputManagerBundle::<BoxMovement>::default())
        // Note: another entity will be driving this input
        .id();

    commands.entity(window.single()).insert(ActionStateDriver {
        action: BoxMovement::MousePosition,
        targets: entity.into(),
    });
}

fn update_cursor_state_from_window(
    window_query: Query<(&Window, &ActionStateDriver<BoxMovement>)>,
    mut action_state_query: Query<&mut ActionState<BoxMovement>>,
) {
    // Update each actionstate with the mouse position from the window
    // by using the referenced entities in ActionStateDriver and the stored action as
    // a key into the action data
    for (window, driver) in window_query.iter() {
        for entity in driver.targets.iter() {
            let mut action_state = action_state_query
                .get_mut(*entity)
                .expect("Entity does not exist, or does not have an `ActionState` component");

            if let Some(val) = window.cursor_position() {
                action_state.action_data_mut(driver.action).axis_pair =
                    Some(DualAxisData::from_xy(val));
            }
        }
    }
}

fn turn_steering_wheel(
    window_query: Query<&Window>,
    mut query: Query<&ActionState<BoxMovement>>,
    mut controls: ResMut<ControlsState>,
) {
    let win = window_query.single();
    let win_w = win.width();
    let win_h = win.height();
    let action_state = query.single_mut();
    if let Some(x) = action_state.axis_pair(BoxMovement::MousePosition) {
        // TODO: make magic numbers configurable
        controls.steering_wheel_degrees = (x.x() / win_w) * 900. - 450.;
        controls.accelerator = 1. - x.y() / win_h;
    }
}
