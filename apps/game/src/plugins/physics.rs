use bevy::{
    app::{App, Plugin},
    ecs::system::Commands,
    math::Vec3,
    prelude::*,
};

use super::PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update);
    }
}

#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct CenterOfMass(Vec3);

#[derive(Component)]
struct BoxBounds {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct Velocity(Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vec3::new(0., 0., 0.))
    }
}

#[derive(Component)]
struct Drag(f32);

#[derive(Component)]
struct Friction(f32);

fn setup(mut commands: Commands) {
    commands.spawn((
        Position(Vec3::default()),
        CenterOfMass(Vec3::default()),
        BoxBounds {
            width: 1.,
            height: 1.,
        },
        Velocity(Vec3::new(1., 0., 0.)),
        Drag(0.1),
        Friction(0.5),
    ));
}

fn update(time: Res<Time>, mut query: Query<(&mut Position, &mut Velocity, &Drag, &Friction)>) {
    let delta = time.delta().as_secs_f32();
    println!("Physics update, delta {}", delta);
    for (mut position, mut velocity, drag, friction) in &mut query {
        let new_vel = velocity.0 - (velocity.0 * ((drag.0 + friction.0) * delta));
        let new_pos = position.0 + (new_vel * delta);
        position.0 = new_pos;
        velocity.0 = new_vel;
        println!("Position: {}, Velocity: {}", position.0, velocity.0);
    }
}
