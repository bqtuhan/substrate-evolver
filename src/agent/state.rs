// File: src/agent/state.rs
use crate::agent::genetics::AgentGenome;

#[derive(Clone, Debug)]
pub struct Agent {
    pub genome: AgentGenome,
    pub x: usize,
    pub y: usize,
    pub energy: f32,
    pub age: usize,
    pub has_reproduced_this_tick: bool,
}