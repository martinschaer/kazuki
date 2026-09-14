use crate::config::VehicleConfig;
use crate::engine::{Engine, EngineState};
use crate::suspension;
use crate::tire;
use crate::transmission::Transmission;
use crate::wheel::Wheel;
use glam::{Quat, Vec3};

/// Wheel order is FL, FR, RL, RR. `i ^ 1` is the other wheel on the same axle.
const DRIVEN: [usize; 2] = [2, 3]; // RWD for now

#[derive(Debug, Clone)]
pub struct ChassisState {
    pub position: Vec3,
    pub orientation: Quat,
    pub velocity: Vec3,         // world frame
    pub angular_velocity: Vec3, // world frame
}

/// Per-wheel inputs each step
#[derive(Debug, Clone, Copy, Default)]
pub struct Controls {
    pub throttle: f32, // 0..1
    pub brake: f32,    // 0..1
    pub steer: f32,    // -1..1
    pub handbrake: bool,
    pub clutch: f32, // 0 = engaged, 1 = fully disengaged
    pub shift_up: bool,
    pub shift_down: bool,
}

/// Suspension raycast result supplied by the Godot layer each step
#[derive(Debug, Clone, Copy)]
pub struct WheelHit {
    pub hit: bool,
    /// distance from attachment to ground (m)
    pub distance: f32,
}

pub struct Vehicle {
    pub cfg: VehicleConfig,
    pub chassis: ChassisState,
    pub engine: Engine,
    pub transmission: Transmission,
    pub wheels: [Wheel; 4],
    /// previous compression per wheel (for damper rate)
    prev_compression: [f32; 4],
    /// (shift_up, shift_down) last step — shifts are edge triggered
    prev_shift: (bool, bool),
    /// Steering angle currently applied (rad) — smoothed toward target
    pub steer_angle: f32,
}

impl Vehicle {
    pub fn new(cfg: VehicleConfig, start: ChassisState) -> Self {
        let engine = Engine::new(&cfg.engine);
        Self {
            cfg,
            chassis: start,
            engine,
            transmission: Transmission::new(),
            wheels: Default::default(),
            prev_compression: [0.0; 4],
            prev_shift: (false, false),
            steer_angle: 0.0,
        }
    }

    // ---------- geometry helpers ----------

    /// Local suspension attachment point of each wheel (FL, FR, RL, RR).
    /// Forward is -Z, so the front axle sits at negative Z.
    pub fn attachment_local(&self, i: usize) -> Vec3 {
        let hw = self.cfg.wheelbase_m * 0.5;
        let ht = self.cfg.track_m * 0.5;
        let (x, z) = match i {
            0 => (-ht, -hw), // FL
            1 => (ht, -hw),  // FR
            2 => (-ht, hw),  // RL
            _ => (ht, hw),   // RR
        };
        Vec3::new(x, 0.0, z)
    }

    // ---------- main step ----------

    /// dt must be the fixed physics step.
    /// `hits`: raycast results from the Godot layer.
    /// Returns accumulated force & torque (world frame) to apply to the RigidBody3D.
    pub fn step(&mut self, controls: &Controls, hits: &[WheelHit; 4], dt: f32) -> (Vec3, Vec3) {
        let up = self.chassis.orientation * Vec3::Y;
        let forward = self.chassis.orientation * Vec3::NEG_Z; // Godot forward
        let right = self.chassis.orientation * Vec3::X;

        // --- steering smoothing (one scalar, front axle only) ---
        let target = controls.steer * 35f32.to_radians();
        let rate = 8.0 * dt;
        self.steer_angle += (target - self.steer_angle).clamp(-rate, rate);

        // --- suspension pass 1: all compressions, then spring/damper load ---
        // The anti-roll bar needs every compression, so it can't run in this pass.
        let mut compressions = [0.0f32; 4];
        let mut spring_load = [0.0f32; 4];
        for i in 0..4 {
            let comp = if hits[i].hit {
                (self.cfg.suspension.rest_length_m - hits[i].distance)
                    .clamp(0.0, self.cfg.suspension.max_travel_m)
            } else {
                0.0
            };
            // positive while compressing, so the damper opposes the motion
            let comp_rate = (comp - self.prev_compression[i]) / dt;
            self.prev_compression[i] = comp;
            compressions[i] = comp;
            spring_load[i] = suspension::force(&self.cfg.suspension, comp, comp_rate);
        }

        // --- drivetrain: engine -> transmission -> wheels ---
        self.transmission.update(dt);
        let top_gear = Transmission::top_gear(&self.cfg.transmission);
        if controls.shift_up && !self.prev_shift.0 {
            self.transmission.request_shift(
                &self.cfg.transmission,
                (self.transmission.gear + 1).min(top_gear),
            );
        }
        if controls.shift_down && !self.prev_shift.1 {
            self.transmission
                .request_shift(&self.cfg.transmission, (self.transmission.gear - 1).max(-1));
        }
        self.prev_shift = (controls.shift_up, controls.shift_down);

        let gear_ratio = self.transmission.overall_ratio(&self.cfg.transmission);
        let clutch_engaged = 1.0 - controls.clutch;

        // engine rpm implied by the driven wheels through the current gear
        let avg_wheel_w: f32 = DRIVEN
            .iter()
            .map(|&i| self.wheels[i].angular_velocity)
            .sum::<f32>()
            / DRIVEN.len() as f32;
        let wheel_rpm = avg_wheel_w * gear_ratio * 60.0 / (2.0 * std::f32::consts::PI);

        if clutch_engaged > 0.5 && gear_ratio.abs() > 1e-6 {
            // ponytail: rigid coupling with an idle floor standing in for clutch
            // slip. Add a real slipping-clutch model if launches/stalling matter.
            self.engine.rpm = wheel_rpm.abs().max(self.cfg.engine.idle_rpm);
            self.engine.clamp_rpm(&self.cfg.engine);
        } else {
            // freewheel: throttle revs it, engine braking slows it
            let t = self.engine.torque_at(&self.cfg.engine, controls.throttle);
            self.engine.integrate_freewheel(&self.cfg.engine, t, dt);
        }

        if self.engine.state == EngineState::Stalled {
            self.engine.start(&self.cfg.engine); // auto-restart for simplicity
        }

        // torque sent to driven wheels
        let crank_torque = if self.transmission.is_shifting() {
            0.0
        } else {
            self.engine.torque_at(&self.cfg.engine, controls.throttle)
        };
        let wheel_torque = crank_torque * gear_ratio * self.cfg.transmission.efficiency
            * clutch_engaged
            / DRIVEN.len() as f32;

        // --- pass 2: anti-roll, suspension + tire forces, wheel spin ---
        let mut total_force = Vec3::ZERO;
        let mut total_torque = Vec3::ZERO;
        let mu = 1.0; // surface friction; make configurable later
        let (s, c) = self.steer_angle.sin_cos();
        let steer_fwd = forward * c - right * s;
        let steer_right = right * c + forward * s;

        for i in 0..4 {
            // computed before borrowing the wheel mutably (methods take &self)
            let attach =
                self.chassis.position + self.chassis.orientation * self.attachment_local(i);
            // tire forces act at the contact patch, not at the top mount —
            // this is what gives lateral force a roll moment arm
            let contact = attach - up * (self.cfg.suspension.rest_length_m - compressions[i]);

            // anti-roll bar across the axle: signed, so it transfers load
            // between the two sides instead of adding net lift.
            let arb =
                suspension::anti_roll_force(&self.cfg.suspension, compressions[i], compressions[i ^ 1]);
            let load = (spring_load[i] + arb).max(0.0);

            let drive = if DRIVEN.contains(&i) { wheel_torque } else { 0.0 };
            let brake = controls.brake * self.cfg.brake_torque_max
                + if controls.handbrake && i >= 2 {
                    self.cfg.handbrake_torque
                } else {
                    0.0
                };

            let w = &mut self.wheels[i];
            w.load_n = load;
            w.on_ground = hits[i].hit && load > 0.0;

            if !w.on_ground {
                // free spin: only drive/brake torques act
                w.force_long = 0.0;
                w.force_lat = 0.0;
                w.slip_ratio = 0.0;
                w.slip_angle = 0.0;
                w.integrate(&self.cfg.tire, drive, brake, dt);
                continue;
            }

            // suspension force applied at the attachment point (world up)
            let f_susp = up * load;
            total_force += f_susp;
            total_torque += (attach - self.chassis.position).cross(f_susp);

            let contact_v = self.chassis.velocity
                + self
                    .chassis
                    .angular_velocity
                    .cross(contact - self.chassis.position);

            // velocity in the wheel frame — must use the same basis the forces
            // are projected back onto below, or slip has the wrong sign.
            let (e_fwd, e_right) = if i < 2 {
                (steer_fwd, steer_right)
            } else {
                (forward, right)
            };
            w.v_long = contact_v.dot(e_fwd);
            w.v_lat = contact_v.dot(e_right);

            // slip quantities. SAE sign convention: slip angle is negated so a
            // positive lateral force opposes the slide.
            let slip_ratio = w.slip_ratio(&self.cfg.tire);
            let slip_angle = (-w.v_lat).atan2(w.v_long.abs().max(1.0));
            w.slip_ratio = slip_ratio;
            w.slip_angle = slip_angle;

            // normalized forces (peak 1), combined via friction circle
            let lat_n = tire::lateral_force(&self.cfg.tire, slip_angle);
            let long_n = tire::longitudinal_force(&self.cfg.tire, slip_ratio);
            let (lat_n, long_n) = tire::combine(lat_n, long_n);

            let grip = mu * tire::load_multiplier(&self.cfg.tire, load);
            w.force_lat = lat_n * grip * load;
            w.force_long = long_n * grip * load;

            let f_world = e_fwd * w.force_long + e_right * w.force_lat;
            total_force += f_world;
            total_torque += (contact - self.chassis.position).cross(f_world);

            w.integrate(&self.cfg.tire, drive, brake, dt);
        }

        // --- aerodynamics (gravity is applied by Godot's RigidBody, not here) ---
        let speed = self.chassis.velocity.length();
        let drag_coef = 0.5 * 1.225 * 2.2 * 0.3; // 0.5 * rho * A * Cd
        total_force -= self.chassis.velocity.normalize_or_zero() * (drag_coef * speed * speed);
        let downforce_coef = 2.0; // tune me
        total_force -= up * (downforce_coef * speed * speed);

        (total_force, total_torque)
    }
}
