// File: src/core/environment.rs
use crate::config::Config;

/// Returns the daily spawn factor (0..1) based on tick count.
pub fn spawn_factor(tick_count: u64, config: &Config) -> f64 {
    let phase = (tick_count % config.day_length) as f64 / config.day_length as f64;
    (0.4 + 0.6 * (phase * 2.0 * std::f64::consts::PI).sin()) as f64
}

/// Checks whether an Ice Age is currently active.
pub fn is_ice_age(tick_count: u64, config: &Config) -> bool {
    (tick_count % config.ice_age_period) < config.ice_age_duration
}