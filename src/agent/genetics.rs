use rand::Rng;
use rand_distr::StandardNormal;
use crate::agent::brain;

pub const INPUT_SIZE: usize = 10;
pub const HIDDEN_SIZE: usize = 6;
pub const OUTPUT_SIZE: usize = 2;

pub const NN_PARAM_COUNT: usize =
    INPUT_SIZE * HIDDEN_SIZE + HIDDEN_SIZE * OUTPUT_SIZE + HIDDEN_SIZE + OUTPUT_SIZE;

pub const GENOME_LEN: usize = 3 + NN_PARAM_COUNT;

pub const GENE_SPEED: usize = 0;
pub const GENE_VISION: usize = 1;
pub const GENE_REPRO_THRESH: usize = 2;

pub const W_IH_OFFSET: usize = 3;
pub const B_H_OFFSET: usize = W_IH_OFFSET + INPUT_SIZE * HIDDEN_SIZE;
pub const W_HO_OFFSET: usize = B_H_OFFSET + HIDDEN_SIZE;
pub const B_O_OFFSET: usize = W_HO_OFFSET + HIDDEN_SIZE * OUTPUT_SIZE;

#[derive(Clone, Debug)]
pub struct AgentGenome {
    pub genes: [f32; GENOME_LEN],
}

impl AgentGenome {
    pub fn random(rng: &mut impl Rng) -> Self {
        let mut genes = [0.0; GENOME_LEN];
        for g in genes.iter_mut() {
            *g = rng.gen_range(-1.0..1.0);
        }
        Self { genes }
    }

    pub fn speed(&self) -> u8 {
        let val = self.genes[GENE_SPEED];
        if val < -0.33 {
            1
        } else if val < 0.33 {
            2
        } else {
            3
        }
    }

    pub fn vision_range(&self, max_vision: usize) -> usize {
        let val = self.genes[GENE_VISION];
        let range = ((val + 1.0) * 0.5 * (max_vision as f32 - 1.0) + 1.0).round() as usize;
        range.clamp(1, max_vision)
    }

    pub fn reproduction_threshold(&self) -> f32 {
        (self.genes[GENE_REPRO_THRESH] + 1.0) * 50.0
    }

    pub fn feedforward(&self, inputs: &[f32; INPUT_SIZE]) -> [f32; OUTPUT_SIZE] {
        brain::feedforward(&self.genes, inputs)
    }

    pub fn phenotype_color(&self, max_vision: usize) -> (u8, u8, u8) {
        let speed = self.speed() as f32;
        let vision = self.vision_range(max_vision) as f32;
        let repro = self.reproduction_threshold();
        let r = ((speed - 1.0) / 2.0 * 255.0) as u8;
        let g = ((repro / 100.0) * 255.0).clamp(0.0, 255.0) as u8;
        let b = ((vision - 1.0) / (max_vision as f32 - 1.0) * 255.0) as u8;
        (r, g, b)
    }

    pub fn two_point_crossover(
        parent_a: &[f32; GENOME_LEN],
        parent_b: &[f32; GENOME_LEN],
        rng: &mut impl Rng,
    ) -> [f32; GENOME_LEN] {
        let mut child = [0.0; GENOME_LEN];
        let i = rng.gen_range(0..GENOME_LEN);
        let j = rng.gen_range(i..=GENOME_LEN);
        for k in 0..GENOME_LEN {
            if k < i || k >= j {
                child[k] = parent_a[k];
            } else {
                child[k] = parent_b[k];
            }
        }
        child
    }

    pub fn mutate_genes(
        genes: &mut [f32; GENOME_LEN],
        rng: &mut impl Rng,
        mutation_scale: f32,
        ice_age_active: bool,
    ) {
        let mutation_range = 0.2 * mutation_scale;
        for gene in genes.iter_mut() {
            if rng.gen_bool(0.02) {
                if ice_age_active {
                    *gene += rng.sample::<f32, _>(StandardNormal) * mutation_range;
                } else {
                    *gene += rng.gen_range(-mutation_range..mutation_range);
                }
                *gene = gene.clamp(-1.0, 1.0);
            }
        }
    }
}