#[derive(Debug, Clone)]
pub struct VehicleConfig {
    pub mass_kg: f32,
    pub wheelbase_m: f32,
    pub track_m: f32, // left-right wheel separation
    pub front_weight_dist: f32, // 0.5 = balanced

    pub engine: EngineConfig,
    pub transmission: TransmissionConfig,

    pub tire: TireConfig,
    pub suspension: SuspensionConfig,
    pub brake_torque_max: f32,   // Nm at each wheel, front/rear handled later
    pub handbrake_torque: f32,
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub idle_rpm: f32,
    pub max_rpm: f32,
    pub inertia: f32,            // kg·m²
    /// (rpm, torque_Nm) pairs, sorted by rpm, linear interpolation between
    pub torque_curve: Vec<(f32, f32)>,
    pub engine_brake_torque: f32, // Nm at full off-throttle
}

#[derive(Debug, Clone)]
pub struct TransmissionConfig {
    /// index 0 = reverse, index 1 = neutral (0.0), index 2.. = forward gears
    pub gear_ratios: Vec<f32>,
    pub final_drive: f32,
    pub shift_time_s: f32,
    pub efficiency: f32, // ~0.9
}

#[derive(Debug, Clone)]
pub struct TireConfig {
    pub radius_m: f32,
    pub wheel_inertia: f32,      // kg·m²
    // Simplified Pacejka: peak force factor B, shape C, peak D, stiffness curve E
    pub lat_b: f32, pub lat_c: f32, pub lat_d: f32, pub lat_e: f32,
    pub long_b: f32, pub long_c: f32, pub long_d: f32, pub long_e: f32,
    pub load_sensitivity: f32,   // grip coefficient at load L scales as (L / nominal)^(-this)
    pub nominal_load_n: f32,
}

#[derive(Debug, Clone)]
pub struct SuspensionConfig {
    pub rest_length_m: f32,
    pub spring_stiffness_n_m: f32,
    pub damper_n_s_m: f32,
    pub max_travel_m: f32,
    pub anti_roll_stiffness_n_m: f32, // 0 = none
}

/// A sensible default (something like a light sports car)
impl Default for VehicleConfig {
    fn default() -> Self {
        Self {
            mass_kg: 1200.0,
            wheelbase_m: 2.5,
            track_m: 1.6,
            front_weight_dist: 0.5,
            engine: EngineConfig {
                idle_rpm: 900.0,
                max_rpm: 7500.0,
                inertia: 0.25,
                engine_brake_torque: 40.0,
                torque_curve: vec![
                    (1000.0, 180.0),
                    (2500.0, 260.0),
                    (4000.0, 310.0),
                    (5500.0, 320.0),
                    (6800.0, 290.0),
                    (7500.0, 240.0),
                ],
            },
            transmission: TransmissionConfig {
                gear_ratios: vec![-3.2, 0.0, 3.6, 2.2, 1.6, 1.25, 1.0, 0.85],
                final_drive: 3.7,
                shift_time_s: 0.25,
                efficiency: 0.9,
            },
            tire: TireConfig {
                radius_m: 0.33,
                wheel_inertia: 1.2,
                lat_b: 10.0, lat_c: 1.9, lat_d: 1.0, lat_e: 0.97,
                long_b: 12.0, long_c: 1.65, long_d: 1.0, long_e: 0.97,
                load_sensitivity: 0.06,
                nominal_load_n: 4000.0,
            },
            suspension: SuspensionConfig {
                rest_length_m: 0.35,
                spring_stiffness_n_m: 45000.0,
                damper_n_s_m: 4200.0,
                max_travel_m: 0.25,
                anti_roll_stiffness_n_m: 8000.0,
            },
            brake_torque_max: 1800.0,
            handbrake_torque: 2200.0,
        }
    }
}
