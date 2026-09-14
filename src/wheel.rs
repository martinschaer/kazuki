use crate::config::TireConfig;

#[derive(Debug, Clone, Default)]
pub struct Wheel {
    /// Angular velocity (rad/s). Positive = rolling forward.
    pub angular_velocity: f32,
    /// Vertical (suspension) force this frame, N
    pub load_n: f32,
    /// Contact patch velocities in wheel frame (m/s), set by Vehicle each step
    pub v_long: f32,
    pub v_lat: f32,
    /// Slip outputs for debug
    pub slip_ratio: f32,
    pub slip_angle: f32,
    /// Forces produced this frame (wheel frame), N
    pub force_long: f32,
    pub force_lat: f32,
    pub on_ground: bool,
}

impl Wheel {
    pub fn slip_ratio(&self, tire: &TireConfig) -> f32 {
        let denom = self.v_long.abs().max(3.0); // low-speed clamp avoids blowups
        (self.angular_velocity * tire.radius_m - self.v_long) / denom
    }

    /// Longitudinal wheel dynamics: drive torque − brake − reaction from traction.
    pub fn integrate(&mut self, tire: &TireConfig, drive_torque: f32, brake_torque: f32, dt: f32) {
        // Reaction torque from traction force at contact patch
        let traction_torque = -self.force_long * tire.radius_m;
        let net = drive_torque + traction_torque;

        self.angular_velocity += net / tire.wheel_inertia * dt;

        // Brakes: can't reverse wheel direction through zero (static-ish clamp)
        if brake_torque > 0.0 {
            let delta_from_brake = brake_torque / tire.wheel_inertia * dt;
            if self.angular_velocity.abs() <= delta_from_brake {
                self.angular_velocity = 0.0;
            } else {
                self.angular_velocity -= delta_from_brake * self.angular_velocity.signum();
            }
        }
    }
}
