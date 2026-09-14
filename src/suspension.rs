use crate::config::SuspensionConfig;

/// Spring + damper force given compression (m) and its rate of change (m/s,
/// positive while compressing). Returns force >= 0 — wheels can't pull the car down.
pub fn force(cfg: &SuspensionConfig, compression: f32, compression_rate: f32) -> f32 {
    let spring = cfg.spring_stiffness_n_m * compression;
    let damper = cfg.damper_n_s_m * compression_rate;
    (spring + damper).max(0.0)
}

/// Anti-roll bar: a pure couple across one axle. Signed — positive (up) on the
/// more compressed side and exactly the opposite on the other, so the bar
/// transfers load between the wheels instead of adding net lift to the car.
pub fn anti_roll_force(cfg: &SuspensionConfig, own: f32, opposite: f32) -> f32 {
    cfg.anti_roll_stiffness_n_m * (own - opposite)
}
