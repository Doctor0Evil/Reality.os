use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitParams {
    pub baseline_da: f64,
    pub nicotine_sensitivity: f64,
    pub gaba_inhibition: f64,
    pub pfc_control_gain: f64,
    pub learning_rate: f64,
    pub temperature: f64, // for softmax choice
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitState {
    pub da_level: f64,      // normalized 0..1
    pub q_smoke: f64,
    pub q_abstain: f64,
    pub q_alt: f64,
}

impl CircuitState {
    pub fn effective_da(&self, nic_brain: f64, p: &CircuitParams) -> f64 {
        let nic_effect = p.nicotine_sensitivity * nic_brain;
        let gaba_term = p.gaba_inhibition;
        let raw = p.baseline_da + nic_effect - gaba_term;
        raw.clamp(0.0, 1.0)
    }

    pub fn choose_action(&self, p: &CircuitParams) -> usize {
        let beta = 1.0 / p.temperature.max(1e-3);
        let v = [
            self.q_smoke,
            self.q_abstain,
            self.q_alt,
        ];
        let mut exp_v = [0.0; 3];
        let mut sum = 0.0;
        for (i, val) in v.iter().enumerate() {
            let e = (beta * val).exp();
            exp_v[i] = e;
            sum += e;
        }
        let probs = [exp_v[0]/sum, exp_v[1]/sum, exp_v[2]/sum];
        // simple argmax; for stochastic behavior, sample from probs
        if probs[0] >= probs[1] && probs[0] >= probs[2] { 0 }
        else if probs[1] >= probs[2] { 1 } else { 2 }
    }

    pub fn update_q(&mut self, action: usize, reward: f64, eff_da: f64, p: &CircuitParams) {
        let prediction = match action {
            0 => self.q_smoke,
            1 => self.q_abstain,
            _ => self.q_alt,
        };
        let delta = reward + eff_da - prediction;
        let alpha = p.learning_rate;
        match action {
            0 => self.q_smoke += alpha * delta,
            1 => self.q_abstain += alpha * delta,
            _ => self.q_alt += alpha * delta,
        }
        self.da_level = eff_da;
    }
}
