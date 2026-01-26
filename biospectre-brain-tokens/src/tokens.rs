use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct BloodTokenState {
    pub balance_ml_equiv: f32,
    pub max_drain_per_night: f32,
}

#[derive(Debug, Clone)]
pub struct BrainTokenState {
    pub balance: f32,
    pub epoch_mint_factors: (f32, f32, u64), // (sn3, s_unknown, eco_flops)
    pub nightly_soft_cap: f32,
}

#[derive(Debug, Clone)]
pub struct ProteinTokenState {
    pub balance_g_equiv: f32,
    pub synaptic_plasticity_weight: f32,
    pub catabolic_penalty: f32,
}

#[derive(Debug, Clone)]
pub struct DraculaWaveState {
    pub balance_quanta: f32,
    pub dutycycle_max_pct: f32,
    pub cooldown_sec: u64,
    pub last_burst_utc: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct EcoTokenState {
    pub balance_nj_equiv: f32,
    pub target_reduction_vs_baseline_pct: f32,
}

#[derive(Debug, Clone)]
pub struct BiophysicalTokenBundle {
    pub blood: BloodTokenState,
    pub brain: BrainTokenState,
    pub protein: ProteinTokenState,
    pub dracula: DraculaWaveState,
    pub eco: EcoTokenState,
}

impl BiophysicalTokenBundle {
    pub fn can_schedule_workload(
        &self,
        required_brain_tokens: f32,
        required_dracula_quanta: f32,
        required_eco_nj: f32,
    ) -> bool {
        self.brain.balance >= required_brain_tokens
            && self.dracula.balance_quanta >= required_dracula_quanta
            && self.eco.balance_nj_equiv + required_eco_nj
                <= self.eco.balance_nj_equiv
                    * (1.0 + self.eco.target_reduction_vs_baseline_pct / 100.0)
    }
}
