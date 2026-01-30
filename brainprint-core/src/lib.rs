use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NeurorightsMode {
    Conservative,
    Copilot,
    Autoevolve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyLayer {
    pub lifeforce_index: f32,
    pub pain_envelope_proximity: f32,
    pub neurorights_mode: NeurorightsMode,
    pub evolve_mode_flag: bool,
    pub sovereignty_violation_risk: f32,

    // Sovereign transhuman-evolution channels (numeric, non-identifying)
    pub transhuman_evolution_rights: u8, // 0 = not declared, 1 = declared+enforced
    pub self_evolution_active:       u8, // 0 = off, 1 = on
    pub continuous_sovereign_ops:    u8, // 0 = degraded, 1 = nominal
    pub self_identity_guard_level:   u8, // 0..=3
    pub sovereign_channel_state:     u8, // 0 = closed, 1 = read-only, 2 = read/write
}
