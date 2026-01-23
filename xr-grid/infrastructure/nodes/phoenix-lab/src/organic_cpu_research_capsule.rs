use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EligibilityMetrics {
    pub sleep_token_s: f32,   // S in [0,1]
    pub psych_risk_r: f32,    // R in [0,1]
    pub enstasis_es: f32,     // Es in [0,1]
    pub eligibility_e: f32,   // E = S * (1 - R) * Es
}

impl EligibilityMetrics {
    pub fn compute(&mut self) {
        let s = self.sleep_token_s.clamp(0.0, 1.0);
        let r = self.psych_risk_r.clamp(0.0, 1.0);
        let es = self.enstasis_es.clamp(0.0, 1.0);
        self.eligibility_e = (s * (1.0 - r) * es).clamp(0.0, 1.0);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganicCpuState {
    pub ofc_capacity: f32,
    pub nram_capacity: f32,
    pub enfr_capacity: f32,
    pub cso_cpu: f32,
    pub stability_score_neuro: f32,
    pub organic_cpu_stability_score: f32,
    pub quantum_ratio_scaling_qs: f32,
    pub organic_cpu_research_index_ori: f32,
}

impl OrganicCpuState {
    pub fn compute_ori(
        &mut self,
        elig: &EligibilityMetrics,
        c_global: f32,
    ) {
        let e = elig.eligibility_e.clamp(0.0, 1.0);
        let s_neuro = self.stability_score_neuro.clamp(0.0, 1.0);
        let c = c_global.clamp(0.0, 1.0);
        let ori = 0.4 * e + 0.3 * s_neuro + 0.3 * c;
        self.organic_cpu_research_index_ori = ori.clamp(0.0, 1.0);
    }

    pub fn allow_high_intensity(&self, elig: &EligibilityMetrics) -> bool {
        elig.eligibility_e >= 0.7
            && self.organic_cpu_stability_score >= 0.75
            && self.organic_cpu_research_index_ori >= 0.5
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsCompliance {
    pub mental_privacy: bool,
    pub cognitive_liberty: bool,
    pub mental_integrity: bool,
    pub non_commercial_neural_data: bool,
    pub no_punitive_xr: bool,
    pub no_person_scoring: bool,
    pub soul_non_addressable: bool,
    pub geneva_ok: bool,
    pub brussels_ok: bool,
    pub santiago_ok: bool,
    pub la_jolla_ok: bool,
    pub phoenix_ok: bool,
}

impl NeurorightsCompliance {
    pub fn c_global(&self) -> f32 {
        let rights_ok = self.mental_privacy
            && self.cognitive_liberty
            && self.mental_integrity
            && self.non_commercial_neural_data
            && self.no_punitive_xr
            && self.no_person_scoring
            && self.soul_non_addressable;
        let juris_ok = self.geneva_ok
            && self.brussels_ok
            && self.santiago_ok
            && self.la_jolla_ok
            && self.phoenix_ok;
        if rights_ok && juris_ok { 1.0 } else { 0.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentSafetyState {
    pub csi: f32,
    pub cats: f32,
    pub explicit_consent_token: bool,
}

impl ConsentSafetyState {
    pub fn consent_valid(&self) -> bool {
        self.explicit_consent_token && self.cats >= 0.65 && self.csi > 0.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganicCpuResearchSnapshot {
    pub session_id: String,
    pub epoch_index: u32,
    pub eligibility: EligibilityMetrics,
    pub ocpu_state: OrganicCpuState,
    pub neurorights: NeurorightsCompliance,
    pub consent: ConsentSafetyState,
    pub c_global: f32,
    pub allow_high_intensity_play: bool,
    pub high_quality_ocpu_session: bool,
}

impl OrganicCpuResearchSnapshot {
    pub fn evaluate(&mut self) {
        self.eligibility.compute();
        self.c_global = self.neurorights.c_global();
        self.ocpu_state
            .compute_ori(&self.eligibility, self.c_global);
        let allow_intensity = self.ocpu_state.allow_high_intensity(&self.eligibility);
        let consent_ok = self.consent.consent_valid();
        self.allow_high_intensity_play = allow_intensity && consent_ok;
        self.high_quality_ocpu_session =
            self.ocpu_state.organic_cpu_research_index_ori >= 0.5
                && (self.c_global - 1.0).abs() < f32::EPSILON;
    }
}
