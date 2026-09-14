use crate::config::EngineConfig;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EngineState {
    Running,
    Stalled,
}

pub struct Engine {
    pub rpm: f32,
    pub state: EngineState,
}

impl Engine {
    pub fn new(cfg: &EngineConfig) -> Self {
        Self {
            rpm: cfg.idle_rpm,
            state: EngineState::Stalled,
        }
    }

    /// Torque available at the crank at current RPM and throttle [0..1]
    pub fn torque_at(&self, cfg: &EngineConfig, throttle: f32) -> f32 {
        let curve = &cfg.torque_curve;
        if curve.is_empty() {
            return 0.0;
        }
        let t = self.rpm.clamp(curve[0].0, curve.last().unwrap().0);

        // linear interpolation on the curve
        let base = curve
            .windows(2)
            .find(|w| t >= w[0].0 && t <= w[1].0)
            .map(|w| {
                let f = (t - w[0].0) / (w[1].0 - w[0].0);
                w[0].1 + f * (w[1].1 - w[0].1)
            })
            .unwrap_or(curve.last().unwrap().1);

        let engine_brake = cfg.engine_brake_torque * (1.0 - throttle);
        base * throttle - engine_brake
    }

    /// Integrate engine RPM when clutch is slipping/disengaged (freewheeling)
    /// Returns nothing; caller applies to state.
    pub fn integrate_freewheel(&mut self, cfg: &EngineConfig, net_torque: f32, dt: f32) {
        self.rpm += (net_torque / cfg.inertia) * dt * 60.0 / (2.0 * std::f32::consts::PI);
        self.clamp_rpm(cfg);
    }

    pub fn clamp_rpm(&mut self, cfg: &EngineConfig) {
        // hard rev limiter: cut torque by clamping
        self.rpm = self.rpm.min(cfg.max_rpm);
        // stall check must run BEFORE the idle governor, otherwise the governor
        // raises rpm back to idle first and the engine can never stall.
        if self.rpm < cfg.idle_rpm * 0.5 {
            self.state = EngineState::Stalled;
        }
        if self.state == EngineState::Running {
            self.rpm = self.rpm.max(cfg.idle_rpm); // idle governor (very simplified)
        }
    }

    pub fn start(&mut self, cfg: &EngineConfig) {
        self.rpm = self.rpm.max(cfg.idle_rpm);
        self.state = EngineState::Running;
    }
}
