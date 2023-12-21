use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    GenericJoint, GenericJointBuilder, ImpulseJoint, JointAxesMask, JointAxis,
};
use std::f32::consts::PI;

use crate::car::{
    dynamics::{UprightJoint, WheelJoint},
    RearWheel,
};
use crate::plugins::controls::ControlsState;

fn steering_to_angle(steering_wheel_degrees: f32) -> f32 {
    let turning_degrees = 90.;
    let steering_wheel_degrees_range = 900.;
    turning_degrees * (steering_wheel_degrees + steering_wheel_degrees_range * 0.5)
        / steering_wheel_degrees_range
        - turning_degrees * 0.5
}

pub fn system_rear_axle_motor(
    controls: Res<ControlsState>,
    mut q: Query<(&mut ImpulseJoint, &WheelJoint), With<RearWheel>>,
) {
    for (mut joint, wheel_joint) in q.iter_mut() {
        let vel = if wheel_joint.is_left {
            controls.accelerator * -1000.
        } else {
            controls.accelerator * 1000.
        };
        joint.data.set_motor_velocity(JointAxis::AngX, vel, 1.);
    }
}

// pub fn system_update_upright_steering(
//     controls: Res<ControlsState>,
//     mut q: Query<(&mut Transform, &Upright)>,
// ) {
//     let angle = steering_to_angle(controls.steering_wheel_degrees);
//     for (mut transform, upright) in q.iter_mut() {
//         if upright.is_front {
//             transform.rotation = Quat::from_rotation_y(-angle.to_radians());
//         }
//     }
// }
pub fn system_update_upright_steering(
    controls: Res<ControlsState>,
    mut q: Query<(&mut ImpulseJoint, &UprightJoint)>,
) {
    let angle = steering_to_angle(controls.steering_wheel_degrees);
    for (mut joint, upright_joint) in q.iter_mut() {
        if upright_joint.is_front {
            joint
                .data
                .set_motor_position(JointAxis::AngX, angle.to_radians(), 1e6, 1e5);
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
        builder = builder.limits(JointAxis::AngX, [-45_f32.to_radians(), 45_f32.to_radians()]);
        builder = builder.set_motor(JointAxis::AngX, 0., 0., 1e6, 1e5);
    }
    let mut joint = builder.build();
    joint.set_contacts_enabled(false);
    joint
}

pub fn make_upright_wheel_joint(abs_offset: f32, is_left: bool) -> GenericJoint {
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
    let mut joint = builder.build();
    joint.set_contacts_enabled(false);
    joint
}
