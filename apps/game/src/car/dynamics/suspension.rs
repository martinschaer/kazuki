use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    GenericJoint, GenericJointBuilder, JointAxesMask, JointAxis, MultibodyJoint,
};
use std::f32::consts::PI;

use crate::car::{dynamics::WheelJoint, Configuration, Upright};
use crate::plugins::controls::ControlsState;

pub fn system_update_wheel(
    config: Res<Configuration>,
    mut q: Query<&mut MultibodyJoint, With<WheelJoint>>,
) {
    for mut joint in q.iter_mut() {
        joint
            .data
            .set_motor_velocity(JointAxis::AngX, config.wheel_vel, 1.)
            .set_local_anchor2(Vec3::new(0., config.wheel_offset, 0.))
            .set_local_anchor1(Vec3::new(config.wheel_offset, 0., 0.));
    }
}

pub fn make_front_upright_wheel_joint(offset: f32) -> GenericJoint {
    let mut builder = GenericJointBuilder::new(
        JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
    )
    .local_axis2(-Vec3::Y)
    .local_axis1(Vec3::X)
    .local_anchor2(Vec3::new(0., offset, 0.))
    .local_anchor1(Vec3::new(offset, 0., 0.));
    let unlocked_axis = JointAxis::AngX;
    builder = builder.set_motor(unlocked_axis, 0., 5., 0., 0.);
    builder.build()
}

pub fn system_steering(
    controls: Res<ControlsState>,
    // mut query: Query<(&Upright, &mut ImpulseJoint)>,
    mut q: Query<&mut Transform, With<Upright>>,
) {
    let turning_degrees = 90.;
    let steering_wheel_degrees_range = 900.;
    let angle = (turning_degrees / 360.) * 2. * PI * controls.steering_wheel_degrees
        / steering_wheel_degrees_range;
    println!("angle: {}", angle);
    // for (_, mut joint) in query.iter_mut() {
    //     joint.data.set_motor_position(JointAxis::Y, angle, 1e9, 1e3);
    // }
    for mut transform in q.iter_mut() {
        transform.rotation = Quat::from_rotation_y(angle / 180. * PI);
    }
}

// Working
pub fn system_update_upright(
    config: Res<Configuration>,
    mut q: Query<&mut Transform, With<Upright>>,
) {
    for mut transform in q.iter_mut() {
        transform.rotation = Quat::from_rotation_y(config.steering_angle / 180. * PI);
    }
}
// TODO: why doesn't this work? is there another way to apply a rotational force?
// fn update_upright(config: Res<Configuration>, mut q: Query<&mut ImpulseJoint, With<UprightJoint>>) {
//     for mut joint in q.iter_mut() {
//         joint.data.set_motor(JointAxis::AngX, config.steering_angle / 180. * PI, 5., 0., 0.);
//     }
// }

pub fn make_front_upright_chasis_joint(offset: f32) -> GenericJoint {
    let builder = GenericJointBuilder::new(
        // JointAxesMask::X // this is the vertical axis
        JointAxesMask::Y
            | JointAxesMask::Z
            // | JointAxesMask::ANG_X // this is the rotation axis
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z,
    )
    .local_axis2(Vec3::Y)
    .local_axis1(Vec3::Y)
    .local_anchor2(Vec3::new(0., 0., 0.))
    .local_anchor1(Vec3::new(offset, 0., 0.))
    // .limits(JointAxis::AngX, [PI * -0.5, PI * 0.5])
    .limits(JointAxis::X, [-1., 0.]);
    builder.build()
}