use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    GenericJoint, GenericJointBuilder, ImpulseJoint, JointAxesMask, JointAxis, MultibodyJoint,
};
use std::f32::consts::PI;

use crate::car::{
    dynamics::{UprightJoint, WheelJoint},
    Configuration, Upright,
};
use crate::plugins::controls::ControlsState;

pub fn system_update_wheel(
    config: Res<Configuration>,
    mut q: Query<(&mut MultibodyJoint, &WheelJoint)>,
) {
    if config.enable_physics {
        for (mut joint, wheel_joint) in q.iter_mut() {
            let vel = if wheel_joint.is_left {
                -config.wheel_vel
            } else {
                config.wheel_vel
            };
            joint.data.set_motor_velocity(JointAxis::AngX, vel, 1.);

            let offset = if wheel_joint.is_left {
                -config.wheel_offset
            } else {
                config.wheel_offset
            };
            joint
                .data
                .set_local_anchor1(Vec3::new(offset * 0.5, 0., 0.));
            joint
                .data
                .set_local_anchor2(Vec3::new(0., config.wheel_offset * -0.5, 0.));
        }
    }
}

pub fn make_front_upright_wheel_joint(abs_offset: f32, is_left: bool) -> GenericJoint {
    let offset = is_left.then(|| -abs_offset).unwrap_or(abs_offset);
    let builder = GenericJointBuilder::new(
        JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
    )
    .local_axis1(if is_left { -Vec3::X } else { Vec3::X })
    .local_axis2(-Vec3::Y)
    .local_basis1(Quat::from_axis_angle(
        Vec3::Y,
        if is_left { 0. } else { PI },
    )) // hackfix
    .local_anchor1(Vec3::new(offset * 0.5, 0., 0.))
    .local_anchor2(Vec3::new(0., abs_offset * -0.5, 0.));
    // builder = builder.set_motor(JointAxis::AngX, 0., 5., 0., 0.);
    let mut joint = builder.build();
    joint.set_contacts_enabled(false);
    joint
}

pub fn system_update_upright_steering(controls: Res<ControlsState>, mut q: Query<(&mut Transform, &Upright)>) {
    let turning_degrees = 90.;
    let steering_wheel_degrees_range = 900.;
    let angle = turning_degrees * controls.steering_wheel_degrees / steering_wheel_degrees_range
        - turning_degrees * 0.5;
    for (mut transform, upright) in q.iter_mut() {
        if upright.is_front {
            transform.rotation = Quat::from_rotation_y(-angle.to_radians());
        }
    }
}

pub fn system_update_upright_config(config: Res<Configuration>, mut q: Query<(&mut Transform, &Upright)>) {
    if config.enable_physics {
        for (mut transform, upright) in q.iter_mut() {
            if upright.is_front {
                transform.rotation = Quat::from_rotation_y(config.steering_angle.to_radians());
            }
        }
    }
}

pub fn system_update_upright_joint(
    config: Res<Configuration>,
    mut q: Query<(&mut ImpulseJoint, &UprightJoint)>,
) {
    if config.enable_physics {
        for (mut joint, upright_joint) in q.iter_mut() {
            // TODO: why doesn't this work? is there another way to apply a rotational force?
            // joint.data.set_motor(
            //     JointAxis::AngX,
            //     config.steering_angle / 180. * PI,
            //     5.,
            //     0.,
            //     0.,
            // );
            // Another old try:
            //     joint.data.set_motor_position(JointAxis::Y, angle, 1e9, 1e3);
            joint.data.set_local_anchor2(Vec3::new(
                if upright_joint.is_left {
                    config.upright_offset
                } else {
                    -config.upright_offset
                },
                0.,
                0.,
            ));
        }
    }
}

pub fn make_front_upright_chasis_joint(
    anchor: Vec3,
    offset: f32,
    suspension_limits: [f32; 2],
    lock_direction: bool,
) -> GenericJoint {
    // X is the vertical axis
    // ANG_X is the rotation axis
    let locked_axes = if lock_direction {
        JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_X
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z
    } else {
        JointAxesMask::Y | JointAxesMask::Z | JointAxesMask::ANG_Y | JointAxesMask::ANG_Z
    };
    let mut builder = GenericJointBuilder::new(locked_axes)
        .local_axis2(Vec3::Y)
        .local_axis1(Vec3::Y)
        .local_anchor2(Vec3::new(-offset, 0., 0.))
        .local_anchor1(anchor)
        .limits(JointAxis::X, suspension_limits);
    if lock_direction {
    } else {
        // builder = builder.limits(JointAxis::X, suspension_limits);
        // builder = builder.limits(JointAxis::AngX, [-90_f32.to_radians(), 90_f32.to_radians()]);
        builder = builder.limits(JointAxis::AngX, [-45_f32.to_radians(), 45_f32.to_radians()]);
    }
    builder.build()
}
