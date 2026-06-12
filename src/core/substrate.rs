// File: src/core/substrate.rs
use rand::Rng;
use crate::config::Config;

/// Wraps a coordinate toroidally.
pub fn wrap_coord(coord: i32, bound: usize) -> usize {
    let b = bound as i32;
    (((coord % b) + b) % b) as usize
}

/// Spawns food on the grid, respecting zone multipliers and environmental factors.
pub fn spawn_food(
    grid: &mut [u8],
    config: &Config,
    rng: &mut impl Rng,
    spawn_factor: f64,
    ice_spawn_factor: f64,
) {
    for y in 0..config.height {
        for x in 0..config.width {
            let zone_mult = if x < config.width / 2 { 1.5 } else { 0.3 };
            let prob = config.base_spawn_prob * spawn_factor * zone_mult * ice_spawn_factor;
            if rng.gen_bool(prob) {
                let idx = y * config.width + x;
                if grid[idx] < 5 {
                    grid[idx] += 1;
                }
            }
        }
    }

    // occasional clusters
    if rng.gen_bool(config.cluster_prob * ice_spawn_factor) {
        let cx = rng.gen_range(0..config.width);
        let cy = rng.gen_range(0..config.height);
        let radius = rng.gen_range(1..=3);
        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                if rng.gen_bool(0.6) {
                    let wx = wrap_coord(cx as i32 + dx, config.width);
                    let wy = wrap_coord(cy as i32 + dy, config.height);
                    let idx = wy * config.width + wx;
                    if grid[idx] < 5 {
                        grid[idx] += 1;
                    }
                }
            }
        }
    }
}

/// Shortest signed toroidal difference from a to b.
pub fn toroidal_diff(a: usize, b: usize, bound: usize) -> i32 {
    let d = b as i32 - a as i32;
    let half = bound as i32 / 2;
    if d > half {
        d - bound as i32
    } else if d < -half {
        d + bound as i32
    } else {
        d
    }
}