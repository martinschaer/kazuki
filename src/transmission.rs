use crate::config::TransmissionConfig;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShiftState {
    Ready,
    Shifting { time_left: f32 },
}

pub struct Transmission {
    pub gear: i32, // -1 reverse, 0 neutral, 1..n forward (index into ratios: gear+1)
    pub shift: ShiftState,
}

impl Default for Transmission {
    fn default() -> Self {
        Self::new()
    }
}

impl Transmission {
    pub fn new() -> Self {
        Self {
            gear: 0,
            shift: ShiftState::Ready,
        }
    }

    /// Highest selectable forward gear for this config.
    pub fn top_gear(cfg: &TransmissionConfig) -> i32 {
        cfg.gear_ratios.len() as i32 - 2
    }

    pub fn ratio(&self, cfg: &TransmissionConfig) -> f32 {
        if self.gear == 0 {
            return 0.0;
        }
        // `gear` is a public field, so never index blind here.
        usize::try_from(self.gear + 1)
            .ok()
            .and_then(|i| cfg.gear_ratios.get(i))
            .copied()
            .unwrap_or(0.0)
    }

    /// Overall ratio including final drive (0 in neutral)
    pub fn overall_ratio(&self, cfg: &TransmissionConfig) -> f32 {
        self.ratio(cfg) * cfg.final_drive
    }

    pub fn request_shift(&mut self, cfg: &TransmissionConfig, target: i32) {
        if self.shift == ShiftState::Ready
            && target != self.gear
            && (target == 0 || target >= -1 && ((target + 1) as usize) < cfg.gear_ratios.len())
        {
            self.shift = ShiftState::Shifting {
                time_left: cfg.shift_time_s,
            };
            self.gear = target; // torque cut during shift via is_shifting()
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let ShiftState::Shifting { time_left } = &mut self.shift {
            *time_left -= dt;
            if *time_left <= 0.0 {
                self.shift = ShiftState::Ready;
            }
        }
    }

    pub fn is_shifting(&self) -> bool {
        matches!(self.shift, ShiftState::Shifting { .. })
    }
}
