use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceProfile {
    pub name: String,
    pub bbb_penetration_prob: f64, // 0..1 (from chemoinformatic models)
    pub cns_toxicity_index: f64,   // 0..1 relative
    pub lung_toxicity_index: f64,  // 0..1
    pub cardio_toxicity_index: f64,// 0..1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureState {
    pub cumulative_cns_toxicity: f64,
    pub cumulative_lung_toxicity: f64,
    pub cumulative_cardio_toxicity: f64,
}

impl ExposureState {
    pub fn apply_dose(&mut self, dose_mg: f64, profile: &SubstanceProfile) {
        let scaled = dose_mg / 10.0;
        self.cumulative_cns_toxicity += scaled * profile.bbb_penetration_prob * profile.cns_toxicity_index;
        self.cumulative_lung_toxicity += scaled * profile.lung_toxicity_index;
        self.cumulative_cardio_toxicity += scaled * profile.cardio_toxicity_index;
    }
}
