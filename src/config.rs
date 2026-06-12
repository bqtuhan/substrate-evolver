// File: src/config.rs
/// Strongly‑typed system configuration.
/// All tunable parameters are gathered here to support runtime customisation.
#[derive(Clone, Debug)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub max_vision: usize,
    pub max_age: usize,
    pub initial_energy: f32,
    pub child_energy: f32,
    pub reproduction_cost: f32,
    pub food_energy: f32,
    pub base_spawn_prob: f64,
    pub cluster_prob: f64,
    pub day_length: u64,
    pub step_cost: f32,
    pub metabolic_cost_per_vision: f32,
    pub ice_age_period: u64,
    pub ice_age_duration: u64,
    pub oasis_vision_cost_mult: f32,
    pub tundra_food_energy_mult: f32,
    pub base_mutation_strength: f32,
    pub seed: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 80,
            height: 24,
            max_vision: 10,
            max_age: 300,
            initial_energy: 100.0,
            child_energy: 50.0,
            reproduction_cost: 30.0,
            food_energy: 30.0,
            base_spawn_prob: 0.04,
            cluster_prob: 0.015,
            day_length: 400,
            step_cost: 0.2,
            metabolic_cost_per_vision: 0.008,
            ice_age_period: 1200,
            ice_age_duration: 200,
            oasis_vision_cost_mult: 1.3,
            tundra_food_energy_mult: 1.5,
            base_mutation_strength: 0.2,
            seed: None,
        }
    }
}