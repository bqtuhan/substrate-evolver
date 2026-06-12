pub mod environment;
pub mod substrate;

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::f32::consts::PI;
use crate::config::Config;
use crate::agent::Agent;
use crate::agent::genetics::AgentGenome;

pub struct Simulation {
    pub config: Config,
    pub grid: Vec<u8>,
    pub agents: Vec<Agent>,
    pub tick_count: u64,
    pub history_log: Vec<String>,
    rng: StdRng,

    // reusable buffers
    mate_candidates: Vec<(usize, usize, usize)>,
    cell_agents: Vec<Vec<usize>>,
    mutation_scale: f32,
    density_grid: Vec<u8>,
}

impl Simulation {
    pub fn new(config: Config, initial_agents: usize) -> Self {
        let rng = if let Some(seed) = config.seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_rng(rand::thread_rng()).unwrap()
        };

        let mut sim = Self {
            grid: vec![0; config.width * config.height],
            agents: Vec::with_capacity(initial_agents),
            tick_count: 0,
            history_log: Vec::new(),
            rng,
            mate_candidates: Vec::new(),
            cell_agents: vec![Vec::new(); config.width * config.height],
            mutation_scale: 1.0,
            density_grid: vec![0; config.width * config.height],
            config,
        };

        // initial food
        for _ in 0..(sim.config.width * sim.config.height / 6) {
            let x = sim.rng.gen_range(0..sim.config.width);
            let y = sim.rng.gen_range(0..sim.config.height);
            sim.grid[y * sim.config.width + x] += 1;
        }

        // initial agents
        for _ in 0..initial_agents {
            let genome = AgentGenome::random(&mut sim.rng);
            let agent = Agent {
                genome,
                x: sim.rng.gen_range(0..sim.config.width),
                y: sim.rng.gen_range(0..sim.config.height),
                energy: sim.config.initial_energy,
                age: 0,
                has_reproduced_this_tick: false,
            };
            sim.agents.push(agent);
        }
        sim
    }

    /// Public helper to scatter extra food (used by WASM interface).
    pub fn spawn_chaos_food(&mut self) {
        for _ in 0..(self.config.width * self.config.height / 4) {
            let idx = self.rng.gen_range(0..self.grid.len());
            if self.grid[idx] < 5 {
                self.grid[idx] += 1;
            }
        }
    }

    /// Public helper to force an Ice Age immediately.
    pub fn force_ice_age(&mut self) {
        self.tick_count = (self.tick_count / self.config.ice_age_period) * self.config.ice_age_period;
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        let pop = self.agents.len();
        self.mutation_scale = if pop < 15 {
            2.5
        } else if pop > 120 {
            0.4
        } else {
            1.0
        };

        let spawn_f = environment::spawn_factor(self.tick_count, &self.config);
        let ice_age_active = environment::is_ice_age(self.tick_count, &self.config);
        let ice_spawn_factor: f64 = if ice_age_active { 0.1 } else { 1.0 };

        // logging
        if ice_age_active && (self.tick_count % self.config.ice_age_period == 0) {
            self.history_log.push(format!(
                "{}: Ice Age started (tick {})",
                self.tick_count, self.tick_count
            ));
        } else if !ice_age_active && (self.tick_count % self.config.ice_age_period == self.config.ice_age_duration) {
            self.history_log.push(format!(
                "{}: Ice Age ended (tick {})",
                self.tick_count, self.tick_count
            ));
        }
        if self.history_log.len() > 20 {
            self.history_log.remove(0);
        }

        // food spawning
        substrate::spawn_food(
            &mut self.grid,
            &self.config,
            &mut self.rng,
            spawn_f,
            ice_spawn_factor,
        );

        // reset reproduction flags
        for agent in &mut self.agents {
            agent.has_reproduced_this_tick = false;
        }

        // mate candidates
        self.mate_candidates.clear();
        self.mate_candidates.extend(
            self.agents.iter().enumerate()
                .filter(|(_, a)| a.energy >= a.genome.reproduction_threshold())
                .map(|(i, a)| (i, a.x, a.y)),
        );

        // pre‑compute cell densities for neural input #10
        self.density_grid.fill(0);
        for agent in &self.agents {
            let idx = agent.y * self.config.width + agent.x;
            self.density_grid[idx] = self.density_grid[idx].saturating_add(1);
        }

        // shuffle agent order
        let num_agents = self.agents.len();
        let mut order: Vec<usize> = (0..num_agents).collect();
        for i in 0..order.len() {
            let j = self.rng.gen_range(i..order.len());
            order.swap(i, j);
        }

        let day_phase = (self.tick_count % self.config.day_length) as f64
            / self.config.day_length as f64;
        let map_phase_signal = (2.0 * PI as f64 * day_phase).sin() as f32;

        // perception and movement
        for &i in &order {
            let agent = &self.agents[i];
            let vision = agent.genome.vision_range(self.config.max_vision);
            let v = vision as i32;

            // nearest food (toroidal)
            let (food_rel_x, food_rel_y, is_food_visible) = {
                let mut best_sq = i32::MAX;
                let mut best_tdx = 0;
                let mut best_tdy = 0;
                for tdy in -v..=v {
                    for tdx in -v..=v {
                        let wx = substrate::wrap_coord(agent.x as i32 + tdx, self.config.width);
                        let wy = substrate::wrap_coord(agent.y as i32 + tdy, self.config.height);
                        if self.grid[wy * self.config.width + wx] > 0 {
                            let sq = tdx * tdx + tdy * tdy;
                            if sq < best_sq {
                                best_sq = sq;
                                best_tdx = tdx;
                                best_tdy = tdy;
                            }
                        }
                    }
                }
                if best_sq < i32::MAX {
                    (best_tdx as f32 / vision as f32, best_tdy as f32 / vision as f32, 1.0)
                } else {
                    (0.0, 0.0, -1.0)
                }
            };

            // nearest mate
            let (mate_rel_x, mate_rel_y, is_mate_visible) = {
                let mut best_sq = i32::MAX;
                let mut best_dx = 0;
                let mut best_dy = 0;
                for &(j, mx, my) in &self.mate_candidates {
                    if j == i { continue; }
                    let dx = substrate::toroidal_diff(agent.x, mx, self.config.width);
                    let dy = substrate::toroidal_diff(agent.y, my, self.config.height);
                    if dx.abs() <= v && dy.abs() <= v {
                        let sq = dx * dx + dy * dy;
                        if sq < best_sq {
                            best_sq = sq;
                            best_dx = dx;
                            best_dy = dy;
                        }
                    }
                }
                if best_sq < i32::MAX {
                    (best_dx as f32 / vision as f32, best_dy as f32 / vision as f32, 1.0)
                } else {
                    (0.0, 0.0, -1.0)
                }
            };

            let energy_frac = (agent.energy / self.config.initial_energy).clamp(0.0, 1.0);
            let clock_signal = (agent.age as f32 / self.config.max_age as f32 * 2.0 * PI).sin();
            let density = self.density_grid[agent.y * self.config.width + agent.x] as f32 / 10.0;

            let inputs = [
                food_rel_x, food_rel_y,
                mate_rel_x, mate_rel_y,
                energy_frac,
                is_food_visible,
                is_mate_visible,
                map_phase_signal,
                clock_signal,
                density,
            ];
            let outputs = agent.genome.feedforward(&inputs);

            let speed = agent.genome.speed() as f32;
            let raw_dx = outputs[0];
            let raw_dy = outputs[1];
            let mag = (raw_dx * raw_dx + raw_dy * raw_dy).sqrt();
            let (desired_dx, desired_dy) = if mag > speed {
                (raw_dx * speed / mag, raw_dy * speed / mag)
            } else {
                (raw_dx, raw_dy)
            };

            let step_x = if desired_dx.abs() > self.rng.gen::<f32>() { desired_dx.signum() as i32 } else { 0 };
            let step_y = if desired_dy.abs() > self.rng.gen::<f32>() { desired_dy.signum() as i32 } else { 0 };
            let steps_taken = step_x.abs() + step_y.abs();

            let zone_vision_mult = if agent.x < self.config.width / 2 {
                self.config.oasis_vision_cost_mult
            } else {
                1.0
            };
            let metabolic = vision as f32 * self.config.metabolic_cost_per_vision * zone_vision_mult;
            let movement = steps_taken as f32 * speed * speed * self.config.step_cost;

            let mut age_factor = 1.0 + 0.5 * (agent.age as f32 / self.config.max_age as f32);
            if ice_age_active {
                age_factor *= 2.0;
            }
            let total_cost = (metabolic + movement) * age_factor;

            let agent = &mut self.agents[i];
            agent.energy -= total_cost;
            agent.age += 1;

            let (new_x, new_y) = (
                substrate::wrap_coord(agent.x as i32 + step_x, self.config.width),
                substrate::wrap_coord(agent.y as i32 + step_y, self.config.height),
            );
            agent.x = new_x;
            agent.y = new_y;

            let idx = new_y * self.config.width + new_x;
            if self.grid[idx] > 0 {
                self.grid[idx] -= 1;
                let food_gain = if new_x >= self.config.width / 2 {
                    self.config.food_energy * self.config.tundra_food_energy_mult
                } else {
                    self.config.food_energy
                };
                agent.energy += food_gain;
            }
        }

        // aggression (split_at_mut)
        for v in &mut self.cell_agents {
            v.clear();
        }
        for (i, agent) in self.agents.iter().enumerate() {
            self.cell_agents[agent.y * self.config.width + agent.x].push(i);
        }

        for cell_vec in &self.cell_agents {
            if cell_vec.len() < 2 {
                continue;
            }
            for a in 0..cell_vec.len() {
                let attacker = cell_vec[a];
                let (is_predator, _) = {
                    let ag = &self.agents[attacker];
                    (ag.genome.speed() == 3 && ag.energy < 30.0, ag.energy)
                };
                if !is_predator {
                    continue;
                }

                let offset = self.rng.gen_range(1..cell_vec.len());
                let victim = cell_vec[(a + offset) % cell_vec.len()];

                let split_idx = attacker.max(victim);
                let (left, right) = self.agents.split_at_mut(split_idx);
                let (attacker_ref, victim_ref) = if attacker < victim {
                    (&mut left[attacker], &mut right[0])
                } else {
                    (&mut right[0], &mut left[victim])
                };

                let stolen = victim_ref.energy.min(25.0);
                if stolen > 0.0 {
                    attacker_ref.energy += stolen;
                    victim_ref.energy -= stolen;
                }
            }
        }

        // ice age cold damage
        if ice_age_active {
            for agent in &mut self.agents {
                let age_risk = (agent.age as f32 / self.config.max_age as f32).clamp(0.0, 1.0);
                if self.rng.gen_bool((age_risk * 0.1) as f64) {
                    agent.energy -= 10.0;
                }
            }
        }

        // death
        let prev_pop = self.agents.len();
        let mut dead: Vec<usize> = self.agents.iter().enumerate()
            .filter(|(_, a)| a.energy <= 0.0 || a.age >= self.config.max_age)
            .map(|(i, _)| i)
            .collect();
        dead.sort_unstable();
        for &i in dead.iter().rev() {
            self.agents.swap_remove(i);
        }
        if self.agents.len() < prev_pop {
            self.history_log.push(format!(
                "{}: Population dropped from {} to {}",
                self.tick_count, prev_pop, self.agents.len()
            ));
            if self.history_log.len() > 20 {
                self.history_log.remove(0);
            }
        }

        // reproduction
        for v in &mut self.cell_agents {
            v.clear();
        }
        for (i, agent) in self.agents.iter().enumerate() {
            self.cell_agents[agent.y * self.config.width + agent.x].push(i);
        }

        let mut new_agents = Vec::with_capacity(self.agents.len() / 4);
        for cell_vec in &self.cell_agents {
            if cell_vec.len() < 2 {
                continue;
            }
            for a in 0..cell_vec.len() {
                for b in a + 1..cell_vec.len() {
                    let i = cell_vec[a];
                    let j = cell_vec[b];
                    if self.agents[i].has_reproduced_this_tick || self.agents[j].has_reproduced_this_tick {
                        continue;
                    }
                    let thr_i = self.agents[i].genome.reproduction_threshold();
                    let thr_j = self.agents[j].genome.reproduction_threshold();
                    if self.agents[i].energy >= thr_i && self.agents[j].energy >= thr_j {
                        self.agents[i].energy -= self.config.reproduction_cost;
                        self.agents[j].energy -= self.config.reproduction_cost;
                        self.agents[i].has_reproduced_this_tick = true;
                        self.agents[j].has_reproduced_this_tick = true;

                        let mut child_genes = AgentGenome::two_point_crossover(
                            &self.agents[i].genome.genes,
                            &self.agents[j].genome.genes,
                            &mut self.rng,
                        );

                        AgentGenome::mutate_genes(
                            &mut child_genes,
                            &mut self.rng,
                            self.mutation_scale,
                            ice_age_active,
                        );

                        let offsets: [(i32, i32); 8] = [
                            (-1, -1), (0, -1), (1, -1),
                            (-1,  0),          (1,  0),
                            (-1,  1), (0,  1), (1,  1),
                        ];
                        let (dx, dy) = offsets[self.rng.gen_range(0..8)];
                        let child_x = substrate::wrap_coord(self.agents[i].x as i32 + dx, self.config.width);
                        let child_y = substrate::wrap_coord(self.agents[i].y as i32 + dy, self.config.height);

                        new_agents.push(Agent {
                            genome: AgentGenome { genes: child_genes },
                            x: child_x,
                            y: child_y,
                            energy: self.config.child_energy,
                            age: 0,
                            has_reproduced_this_tick: false,
                        });
                    }
                }
            }
        }

        self.agents.extend(new_agents);
    }
}