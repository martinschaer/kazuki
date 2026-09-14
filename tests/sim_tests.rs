use vehicle_sim::config::VehicleConfig;
use vehicle_sim::vehicle::{ChassisState, Controls, Vehicle, WheelHit};
use vehicle_sim::{suspension, tire};

const DT: f32 = 1.0 / 60.0;

/// Car sitting on flat ground, 0.10 m of suspension compression per corner.
fn on_flat_ground(velocity: glam::Vec3) -> Vehicle {
    let mut v = Vehicle::new(
        VehicleConfig::default(),
        ChassisState {
            position: glam::Vec3::ZERO,
            orientation: glam::Quat::IDENTITY,
            velocity,
            angular_velocity: glam::Vec3::ZERO,
        },
    );
    let hits = [WheelHit { hit: true, distance: 0.25 }; 4];
    // two steps so the first-frame damper transient settles out
    v.step(&Controls::default(), &hits, DT);
    v.step(&Controls::default(), &hits, DT);
    v
}

#[test]
fn torque_curve_interpolates() {
    let cfg = VehicleConfig::default();
    let mut e = vehicle_sim::engine::Engine::new(&cfg.engine);
    e.start(&cfg.engine);

    e.rpm = 4000.0;
    let t = e.torque_at(&cfg.engine, 1.0);
    assert!((t - 310.0).abs() < 1.0, "got {}", t);

    // zero throttle => only engine braking
    let tb = e.torque_at(&cfg.engine, 0.0);
    assert!(tb < 0.0);
}

#[test]
fn pacejka_peaks_near_expected_slip() {
    let cfg = VehicleConfig::default().tire;
    // lateral peaks around ~8-10 deg
    let mut peak = 0.0;
    let mut peak_angle = 0.0;
    let mut a = 0.0;
    while a < 1.0 {
        let f = tire::lateral_force(&cfg, a);
        if f > peak {
            peak = f;
            peak_angle = a;
        }
        a += 0.01;
    }
    assert!(peak > 0.95 && peak <= 1.0);
    assert!(
        peak_angle > 0.1 && peak_angle < 0.25,
        "peak at {} rad",
        peak_angle
    );
}

#[test]
fn friction_circle_limits_combined_force() {
    let (l, g) = vehicle_sim::tire::combine(1.0, 1.0);
    assert!(l * l + g * g <= 1.0 + 1e-4);
}

#[test]
fn gear_ratio_math() {
    let cfg = VehicleConfig::default();
    let mut t = vehicle_sim::transmission::Transmission::new();
    t.gear = 3; // 3rd gear = index 4 in ratios vec => 1.6
    let overall = t.overall_ratio(&cfg.transmission);
    assert!((overall - 1.6 * 3.7).abs() < 1e-5);
}

#[test]
fn lateral_tire_force_opposes_the_slide() {
    // sliding to the right (+X) must produce a force to the left (-X)
    let mut v = on_flat_ground(glam::Vec3::new(5.0, 0.0, 0.0));
    let hits = [WheelHit { hit: true, distance: 0.25 }; 4];
    let (force, _) = v.step(&Controls::default(), &hits, DT);
    assert!(force.x < -1000.0, "lateral force reinforced the slide: {force:?}");
}

#[test]
fn suspension_holds_the_car_up_and_damping_is_positive() {
    let v = on_flat_ground(glam::Vec3::ZERO);
    let total_load: f32 = v.wheels.iter().map(|w| w.load_n).sum();
    // 4 corners * 45000 N/m * 0.10 m, damper contributes nothing at rest
    assert!((total_load - 18_000.0).abs() < 1.0, "got {total_load}");

    // compressing further must ADD force (positive damping), not subtract
    let cfg = VehicleConfig::default().suspension;
    assert!(suspension::force(&cfg, 0.1, 1.0) > suspension::force(&cfg, 0.1, 0.0));
}

#[test]
fn anti_roll_bar_is_a_pure_couple() {
    let cfg = VehicleConfig::default().suspension;
    let a = suspension::anti_roll_force(&cfg, 0.15, 0.05);
    let b = suspension::anti_roll_force(&cfg, 0.05, 0.15);
    assert!(a > 0.0 && (a + b).abs() < 1e-3, "arb adds net lift: {a} {b}");
}

#[test]
fn grip_is_sublinear_in_load() {
    let cfg = VehicleConfig::default().tire;
    let f = |n: f32| tire::load_multiplier(&cfg, n) * n;
    assert!(f(8000.0) < 2.0 * f(4000.0), "grip grows superlinearly with load");
    assert!(tire::load_multiplier(&cfg, 0.0).is_finite());
}

#[test]
fn front_wheels_are_at_the_front() {
    let v = Vehicle::new(
        VehicleConfig::default(),
        ChassisState {
            position: glam::Vec3::ZERO,
            orientation: glam::Quat::IDENTITY,
            velocity: glam::Vec3::ZERO,
            angular_velocity: glam::Vec3::ZERO,
        },
    );
    // forward is -Z, so the steered wheels (0, 1) must have the smaller Z
    assert!(v.attachment_local(0).z < v.attachment_local(2).z);
}

#[test]
fn out_of_range_gear_does_not_panic() {
    let cfg = VehicleConfig::default();
    let mut t = vehicle_sim::transmission::Transmission::new();
    t.gear = 99;
    assert_eq!(t.ratio(&cfg.transmission), 0.0);
    t.gear = -99;
    assert_eq!(t.ratio(&cfg.transmission), 0.0);
}
