use crate::config::TireConfig;

/// Simplified Pacejka "magic formula": F = D * sin(C * atan(B*slip - E*(B*slip - atan(B*slip))))
fn pacejka(slip: f32, b: f32, c: f32, d: f32, e: f32) -> f32 {
    let bx = b * slip;
    d * (c * (bx - e * (bx - bx.atan())).atan()).sin()
}

/// Normalized slip curves peak at 1.0; caller scales by grip and load.
pub fn lateral_force(cfg: &TireConfig, slip_angle_rad: f32) -> f32 {
    pacejka(slip_angle_rad, cfg.lat_b, cfg.lat_c, cfg.lat_d, cfg.lat_e)
}

pub fn longitudinal_force(cfg: &TireConfig, slip_ratio: f32) -> f32 {
    pacejka(slip_ratio, cfg.long_b, cfg.long_c, cfg.long_d, cfg.long_e)
}

/// Friction coefficient multiplier at a given load. Grip *coefficient* falls as
/// load rises, so total force (multiplier * load) stays sublinear in load.
pub fn load_multiplier(cfg: &TireConfig, load_n: f32) -> f32 {
    // floor the ratio: powf(0, negative) is +inf and poisons the force with NaN
    let l = (load_n / cfg.nominal_load_n).max(1e-3);
    l.powf(-cfg.load_sensitivity)
}

/// Combined slip: scale lat/long forces so their vector magnitude
/// respects the friction circle. Inputs are normalized forces (peak 1),
/// output is the pair to multiply by mu * load.
pub fn combine(lat_norm: f32, long_norm: f32) -> (f32, f32) {
    let mag = (lat_norm * lat_norm + long_norm * long_norm).sqrt();
    if mag > 1.0 {
        (lat_norm / mag, long_norm / mag)
    } else {
        (lat_norm, long_norm)
    }
}
