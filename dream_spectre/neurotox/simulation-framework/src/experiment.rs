use serde::{Serialize, Deserialize};

use crate::pkpd::{PKParams, PKState};
use crate::brain::{CircuitParams, CircuitState};
use crate::toxicology::{SubstanceProfile, ExposureState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub pk_params: PKParams,
    pub circuit_params: CircuitParams,
    pub nicotine_profile: SubstanceProfile,
    pub toxins: Vec<SubstanceProfile>,
    pub dt: f64,
    pub total_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationSnapshot {
    pub t: f64,
    pub conc_brain_nic: f64,
    pub da_level: f64,
    pub q_smoke: f64,
    pub q_abstain: f64,
    pub q_alt: f64,
    pub cumulative_cns_tox: f64,
}

pub fn run_simulation(cfg: &SimulationConfig) -> Vec<SimulationSnapshot> {
    let mut pk = PKState { t: 0.0, dose_lung: 10.0, conc_plasma: 0.0, conc_brain: 0.0 };
    let mut circuit = CircuitState { da_level: 0.0, q_smoke: 0.0, q_abstain: 0.0, q_alt: 0.0 };
    let mut exposure = ExposureState {
        cumulative_cns_toxicity: 0.0,
        cumulative_lung_toxicity: 0.0,
        cumulative_cardio_toxicity: 0.0,
    };
    let mut out = Vec::new();
    while pk.t <= cfg.total_time {
        pk.step(cfg.dt, &cfg.pk_params);
        exposure.apply_dose(
            cfg.dt * pk.conc_plasma, 
            &cfg.nicotine_profile
        );
        let eff_da = circuit.effective_da(pk.conc_brain, &cfg.circuit_params);
        let action = circuit.choose_action(&cfg.circuit_params);
        let reward = if action == 0 { eff_da } else { 0.1 };
        circuit.update_q(action, reward, eff_da, &cfg.circuit_params);
        out.push(SimulationSnapshot {
            t: pk.t,
            conc_brain_nic: pk.conc_brain,
            da_level: circuit.da_level,
            q_smoke: circuit.q_smoke,
            q_abstain: circuit.q_abstain,
            q_alt: circuit.q_alt,
            cumulative_cns_tox: exposure.cumulative_cns_toxicity,
        });
    }
    out
}
