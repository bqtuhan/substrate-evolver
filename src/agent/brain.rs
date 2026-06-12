// File: src/agent/brain.rs
use crate::agent::genetics::{W_IH_OFFSET, B_H_OFFSET, W_HO_OFFSET, B_O_OFFSET, INPUT_SIZE, HIDDEN_SIZE, OUTPUT_SIZE};

/// Feedforward evaluation using gene‑encoded weights and biases.
/// Input size is 10, hidden size 6, output 2.
pub fn feedforward(genes: &[f32], inputs: &[f32; INPUT_SIZE]) -> [f32; OUTPUT_SIZE] {
    let w_ih = &genes[W_IH_OFFSET..B_H_OFFSET];
    let b_h = &genes[B_H_OFFSET..W_HO_OFFSET];
    let w_ho = &genes[W_HO_OFFSET..B_O_OFFSET];
    let b_o = &genes[B_O_OFFSET..B_O_OFFSET + OUTPUT_SIZE];

    let mut hidden = [0.0f32; HIDDEN_SIZE];
    for i in 0..HIDDEN_SIZE {
        let mut sum = b_h[i];
        for j in 0..INPUT_SIZE {
            sum += inputs[j] * w_ih[j * HIDDEN_SIZE + i];
        }
        hidden[i] = sum.tanh();
    }

    let mut output = [0.0f32; OUTPUT_SIZE];
    for i in 0..OUTPUT_SIZE {
        let mut sum = b_o[i];
        for j in 0..HIDDEN_SIZE {
            sum += hidden[j] * w_ho[j * OUTPUT_SIZE + i];
        }
        output[i] = sum.tanh();
    }
    output
}