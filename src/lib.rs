mod config;
mod i18n;
mod core;
mod agent;

use wasm_bindgen::prelude::*;
use js_sys::Object;
use core::Simulation;
use config::Config;

#[wasm_bindgen]
pub struct WasmSimulation {
    sim: Simulation,
    agents_cache: Vec<f32>,
    grid_ptr: *const u8,
    grid_len: usize,
    agents_ptr: *const f32,
    agents_len: usize,
}

impl WasmSimulation {
    fn update_caches(&mut self) {
        self.grid_ptr = self.sim.grid.as_ptr();
        self.grid_len = self.sim.grid.len();

        self.agents_cache.clear();
        let count = self.sim.agents.len();
        self.agents_cache.reserve_exact(count * 10);
        for agent in &self.sim.agents {
            self.agents_cache.push(agent.x as f32);
            self.agents_cache.push(agent.y as f32);
            let (r, g, b) = agent.genome.phenotype_color(self.sim.config.max_vision);
            self.agents_cache.push(r as f32 / 255.0);
            self.agents_cache.push(g as f32 / 255.0);
            self.agents_cache.push(b as f32 / 255.0);
            self.agents_cache.push(agent.energy);
            self.agents_cache.push(agent.age as f32);
            self.agents_cache.push(agent.genome.speed() as f32);
            self.agents_cache.push(agent.genome.vision_range(self.sim.config.max_vision) as f32);
            self.agents_cache.push(agent.genome.reproduction_threshold());
        }
        self.agents_ptr = self.agents_cache.as_ptr();
        self.agents_len = self.agents_cache.len();
    }
}

#[wasm_bindgen]
impl WasmSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let config = Config::default();
        let sim = Simulation::new(config, 40);
        let mut wasm = WasmSimulation {
            sim,
            agents_cache: Vec::new(),
            grid_ptr: std::ptr::null(),
            grid_len: 0,
            agents_ptr: std::ptr::null(),
            agents_len: 0,
        };
        wasm.update_caches();
        wasm
    }

    pub fn tick(&mut self) {
        self.sim.tick();
        self.update_caches();
    }

    pub fn tick_multiple(&mut self, count: u32) {
        for _ in 0..count {
            if self.sim.agents.is_empty() {
                break;
            }
            self.sim.tick();
        }
        self.update_caches();
    }

    pub fn toggle_pause(&mut self) { /* no‑op */ }

    pub fn spawn_chaos_food(&mut self) {
        self.sim.spawn_chaos_food();
        self.update_caches();
    }

    pub fn trigger_ice_age(&mut self) {
        self.sim.force_ice_age();
        self.update_caches();
    }

    pub fn grid_ptr(&self) -> *const u8 { self.grid_ptr }
    pub fn grid_len(&self) -> usize { self.grid_len }

    pub fn agents_ptr(&self) -> *const f32 { self.agents_ptr }
    pub fn agents_len(&self) -> usize { self.agents_len }

    pub fn grid_width(&self) -> usize { self.sim.config.width }
    pub fn grid_height(&self) -> usize { self.sim.config.height }

    pub fn get_stats(&self) -> Object {
        let agent_count = self.sim.agents.len();
        let avg_speed = if agent_count > 0 {
            self.sim.agents.iter()
                .map(|a| a.genome.speed() as f32)
                .sum::<f32>() / agent_count as f32
        } else { 0.0 };
        let avg_vision = if agent_count > 0 {
            self.sim.agents.iter()
                .map(|a| a.genome.vision_range(self.sim.config.max_vision) as f32)
                .sum::<f32>() / agent_count as f32
        } else { 0.0 };
        let avg_repro = if agent_count > 0 {
            self.sim.agents.iter()
                .map(|a| a.genome.reproduction_threshold())
                .sum::<f32>() / agent_count as f32
        } else { 0.0 };
        let total_food: u32 = self.sim.grid.iter().map(|&c| c as u32).sum();
        let phase = (self.sim.tick_count % self.sim.config.day_length) as f64
            / self.sim.config.day_length as f64;

        let obj = Object::new();
        let _ = js_sys::Reflect::set(&obj, &"tick".into(), &(self.sim.tick_count as f64).into());
        let _ = js_sys::Reflect::set(&obj, &"agents".into(), &(agent_count as f64).into());
        let _ = js_sys::Reflect::set(&obj, &"phase".into(), &phase.into());
        let _ = js_sys::Reflect::set(&obj, &"avgSpeed".into(), &(avg_speed as f64).into());
        let _ = js_sys::Reflect::set(&obj, &"avgVision".into(), &(avg_vision as f64).into());
        let _ = js_sys::Reflect::set(&obj, &"avgRepro".into(), &(avg_repro as f64).into());
        let _ = js_sys::Reflect::set(&obj, &"food".into(), &(total_food as f64).into());
        obj
    }

    pub fn get_log(&self) -> String {
        self.sim.history_log.join("\n")
    }
}