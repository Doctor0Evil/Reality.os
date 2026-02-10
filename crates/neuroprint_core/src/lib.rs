use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use std::fmt;

// ============ CORE INVARIENTS ============
// RoH ≤ 0.3 is hard-bound; all outputs must reflect this ceiling
// No actuation, no state mutation, no external API calls
// All inputs are validated snapshots; no raw biometrics exposed
// All outputs are advisory-only; no policy enforcement

// ============ BIOPHYSICAL ENVELOPE SNAPSHOT ============
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophysicalEnvelopeSnapshot {
    pub heart_rate: f32,           // BPM, raw
    pub heart_rate_variability: f32, // ms, raw
    pub eeg_alpha: f32,            // µV², normalized [0.0, 1.0]
    pub eeg_beta: f32,             // µV², normalized [0.0, 1.0]
    pub eeg_gamma: f32,            // µV², normalized [0.0, 1.0]
    pub eeg_alpha_cve: f32,        // Alpha-CVE composite [0.0, 1.0]
    pub eda: f32,                  // µS, normalized [0.0, 1.0]
    pub motion_accel: f32,         // G-force, normalized [0.0, 1.0]
    pub roh: f32,                  // Risk-of-Harm [0.0, 0.3]
    pub capability_tier: CapabilityTier,
    pub evolve_index: u64,         // Event count since session start
    pub session_epoch: u64,        // Seconds since session start
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CapabilityTier {
    CapModelOnly,
    CapLabBench,
    CapGeneralUse,
    CapAugmentedCitizen,
    CapSovereignKernel,
}

impl CapabilityTier {
    pub fn to_scalar(&self) -> f32 {
        match self {
            CapabilityTier::CapModelOnly => 0.0,
            CapabilityTier::CapLabBench => 0.25,
            CapabilityTier::CapGeneralUse => 0.5,
            CapabilityTier::CapAugmentedCitizen => 0.75,
            CapabilityTier::CapSovereignKernel => 1.0,
        }
    }
}

// ============ TREE OF LIFE ASSETS (0.0–1.0) ============
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeOfLifeView {
    pub blood: f32,      // 1.0 - normalized_heart_rate (strain)
    pub oxygen: f32,     // normalized_heart_rate_variability
    pub wave: f32,       // avg(eeg_alpha, eeg_beta, eeg_gamma, eeg_alpha_cve)
    pub h2o: f32,        // fixed neutral (0.5) until hydration axis defined
    pub time: f32,       // normalized session_epoch / max_session_window
    pub decay: f32,      // roh / 0.3 (clamped)
    pub lifeforce: f32,  // 1.0 - decay
    pub nano: f32,       // normalize(evolve_index / 1000.0) — granularity proxy
    pub brain: f32,      // capability_tier scalar
    pub smart: f32,      // (brain + normalize(evolve_index/1000.0)) / 2.0
    pub evolve: f32,     // normalize(evolve_index / 1000.0)
    pub power: f32,      // weighted sum of WARN/RISK fractions (EDA, HR, Motion)
    pub tech: f32,       // brain + active_axis_count / 6.0
    pub fear: f32,       // weighted(eda, heart_rate) — sympathetic arousal
    pub pain: f32,       // fear + weighted(motion_accel) — physical distress
}

impl TreeOfLifeView {
    pub fn from_snapshot(snapshot: &BiophysicalEnvelopeSnapshot) -> Self {
        // Normalize inputs
        let normalized_hr = (snapshot.heart_rate - 60.0) / 60.0; // 60–120 BPM range
        let normalized_hrv = snapshot.heart_rate_variability / 80.0; // 0–80 ms
        let normalized_eeg_alpha = snapshot.eeg_alpha;
        let normalized_eeg_beta = snapshot.eeg_beta;
        let normalized_eeg_gamma = snapshot.eeg_gamma;
        let normalized_eeg_alpha_cve = snapshot.eeg_alpha_cve;
        let normalized_eda = snapshot.eda;
        let normalized_motion = snapshot.motion_accel;
        let normalized_epoch = (snapshot.session_epoch as f32) / 3600.0; // 1hr max session
        let normalized_evolve = (snapshot.evolve_index as f32) / 1000.0; // 0–1000 events

        // Compute TREE assets (pure functions, no side effects)
        let blood = 1.0 - normalized_hr.max(0.0).min(1.0); // Inverse strain
        let oxygen = normalized_hrv.max(0.0).min(1.0); // Direct reserve
        let wave = (normalized_eeg_alpha + normalized_eeg_beta + normalized_eeg_gamma + normalized_eeg_alpha_cve) / 4.0;
        let h2o = 0.5; // Neutral placeholder
        let time = normalized_epoch.max(0.0).min(1.0);
        let decay = (snapshot.roh / 0.3).min(1.0); // Clamped ceiling
        let lifeforce = 1.0 - decay;
        let nano = normalized_evolve.min(1.0);
        let brain = snapshot.capability_tier.to_scalar();
        let smart = (brain + nano) / 2.0;
        let evolve = normalized_evolve.min(1.0);
        let power = ((normalized_eda + normalized_hr + normalized_motion) / 3.0).min(1.0);
        let tech = (brain + 0.5) / 2.0; // 0.5 = 3 active axes (HR, EDA, Motion) out of 6
        let fear = (normalized_eda + normalized_hr) / 2.0;
        let pain = (fear + normalized_motion) / 2.0;

        // Clamp all to [0.0, 1.0]
        Self {
            blood: blood.max(0.0).min(1.0),
            oxygen: oxygen.max(0.0).min(1.0),
            wave: wave.max(0.0).min(1.0),
            h2o: h2o.max(0.0).min(1.0),
            time: time.max(0.0).min(1.0),
            decay: decay.max(0.0).min(1.0),
            lifeforce: lifeforce.max(0.0).min(1.0),
            nano: nano.max(0.0).min(1.0),
            brain: brain.max(0.0).min(1.0),
            smart: smart.max(0.0).min(1.0),
            evolve: evolve.max(0.0).min(1.0),
            power: power.max(0.0).min(1.0),
            tech: tech.max(0.0).min(1.0),
            fear: fear.max(0.0).min(1.0),
            pain: pain.max(0.0).min(1.0),
        }
    }
}

// ============ NATURE TOKENS (ADVISORY PREDICATES) ============
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatureToken {
    pub label: String,
    pub confidence: f32,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroprintView {
    pub timestamp_ms: u64,
    pub subject_id: String,
    pub tree_of_life: TreeOfLifeView,
    pub nature_tokens: Vec<NatureToken>,
    pub diagnostics: NeuroprintDiagnostics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroprintDiagnostics {
    pub labels: Vec<String>,
    pub cooldown_advised: bool,
    pub fairness_imbalance: bool,
    pub sovereignty_integrity: bool,
}

impl NeuroprintView {
    pub fn new(snapshot: &BiophysicalEnvelopeSnapshot, subject_id: &str) -> Self {
        let tree_of_life = TreeOfLifeView::from_snapshot(snapshot);

        let nature_tokens = Self::infer_nature_tokens(&tree_of_life);

        let diagnostics = Self::generate_diagnostics(&tree_of_life, &nature_tokens);

        Self {
            timestamp_ms: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            subject_id: subject_id.to_string(),
            tree_of_life,
            nature_tokens,
            diagnostics,
        }
    }

    fn infer_nature_tokens(tree: &TreeOfLifeView) -> Vec<NatureToken> {
        let mut tokens = Vec::new();

        // CALM_STABLE: high lifeforce, high oxygen, low decay, low fear/pain
        if tree.lifeforce > 0.7 && tree.oxygen > 0.7 && tree.decay < 0.3 && tree.fear < 0.3 && tree.pain < 0.3 {
            tokens.push(NatureToken {
                label: "CALM_STABLE".to_string(),
                confidence: 0.9,
                evidence: vec![
                    "lifeforce > 0.7".to_string(),
                    "oxygen > 0.7".to_string(),
                    "decay < 0.3".to_string(),
                    "fear < 0.3".to_string(),
                    "pain < 0.3".to_string(),
                ],
            });
        }

        // OVERLOADED: decay > 0.6, power > 0.7, lifeforce < 0.3
        if tree.decay > 0.6 && tree.power > 0.7 && tree.lifeforce < 0.3 {
            tokens.push(NatureToken {
                label: "OVERLOADED".to_string(),
                confidence: 0.85,
                evidence: vec![
                    "decay > 0.6".to_string(),
                    "power > 0.7".to_string(),
                    "lifeforce < 0.3".to_string(),
                ],
            });
        }

        // RECOVERY: decay down 0.2+, lifeforce up 0.2+, fear/pain down
        // (In practice, this requires historical context — here we simulate one-step delta)
        // For single snapshot, we use heuristic: decay > 0.4 but < 0.6 and lifeforce > 0.4
        if tree.decay > 0.4 && tree.decay < 0.6 && tree.lifeforce > 0.4 && tree.fear < 0.4 {
            tokens.push(NatureToken {
                label: "RECOVERY".to_string(),
                confidence: 0.7,
                evidence: vec![
                    "decay in [0.4, 0.6]".to_string(),
                    "lifeforce > 0.4".to_string(),
                    "fear < 0.4".to_string(),
                ],
            });
        }

        // UNFAIR_DRAIN: low lifeforce + high power + high tech + low brain
        // (simulated: low lifeforce, high power, high tech, low brain)
        if tree.lifeforce < 0.3 && tree.power > 0.7 && tree.tech > 0.7 && tree.brain < 0.4 {
            tokens.push(NatureToken {
                label: "UNFAIR_DRAIN".to_string(),
                confidence: 0.8,
                evidence: vec![
                    "lifeforce < 0.3".to_string(),
                    "power > 0.7".to_string(),
                    "tech > 0.7".to_string(),
                    "brain < 0.4".to_string(),
                ],
            });
        }

        // Default
        if tokens.is_empty() {
            tokens.push(NatureToken {
                label: "BALANCED".to_string(),
                confidence: 0.6,
                evidence: vec!["no strong pattern detected".to_string()],
            });
        }

        tokens
    }

    fn generate_diagnostics(tree: &TreeOfLifeView, tokens: &[NatureToken]) -> NeuroprintDiagnostics {
        let mut labels = Vec::new();
        for token in tokens {
            labels.push(token.label.clone());
        }

        let cooldown_advised = tokens.iter().any(|t| t.label == "OVERLOADED");
        let fairness_imbalance = tokens.iter().any(|t| t.label == "UNFAIR_DRAIN");
        let sovereignty_integrity = tree.decay <= 0.3 && tree.lifeforce >= 0.0;

        NeuroprintDiagnostics {
            labels,
            cooldown_advised,
            fairness_imbalance,
            sovereignty_integrity,
        }
    }
}

// ============ EXPORT: PURE FUNCTION FOR .evolve.jsonl ============
// This function is the ONLY public interface that writes to the blockchain stream
pub fn neuroprint_from_snapshot(snapshot: &BiophysicalEnvelopeSnapshot, subject_id: &str) -> NeuroprintView {
    // No external calls. No state mutation. No I/O. Pure.
    NeuroprintView::new(snapshot, subject_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_of_life_computation() {
        let snapshot = BiophysicalEnvelopeSnapshot {
            heart_rate: 90.0,
            heart_rate_variability: 60.0,
            eeg_alpha: 0.7,
            eeg_beta: 0.5,
            eeg_gamma: 0.4,
            eeg_alpha_cve: 0.6,
            eda: 0.3,
            motion_accel: 0.2,
            roh: 0.18,
            capability_tier: CapabilityTier::CapAugmentedCitizen,
            evolve_index: 150,
            session_epoch: 1800,
        };

        let tree = TreeOfLifeView::from_snapshot(&snapshot);

        assert!(tree.blood >= 0.0 && tree.blood <= 1.0);
        assert!(tree.oxygen >= 0.0 && tree.oxygen <= 1.0);
        assert!(tree.decay >= 0.0 && tree.decay <= 1.0);
        assert!((tree.lifeforce - (1.0 - tree.decay)).abs() < 0.001);
        assert_eq!(tree.h2o, 0.5);
    }

    #[test]
    fn test_nature_tokens() {
        let snapshot = BiophysicalEnvelopeSnapshot {
            heart_rate: 110.0,
            heart_rate_variability: 20.0,
            eeg_alpha: 0.3,
            eeg_beta: 0.8,
            eeg_gamma: 0.7,
            eeg_alpha_cve: 0.2,
            eda: 0.8,
            motion_accel: 0.6,
            roh: 0.28,
            capability_tier: CapabilityTier::CapModelOnly,
            evolve_index: 950,
            session_epoch: 3500,
        };

        let tree = TreeOfLifeView::from_snapshot(&snapshot);
        let tokens = NeuroprintView::infer_nature_tokens(&tree);

        assert!(tokens.iter().any(|t| t.label == "OVERLOADED"));
        assert!(tokens.iter().any(|t| t.label == "BALANCED"));
        assert!(tokens.len() >= 1);
    }

    #[test]
    fn test_neuroprint_view_integrity() {
        let snapshot = BiophysicalEnvelopeSnapshot {
            heart_rate: 70.0,
            heart_rate_variability: 70.0,
            eeg_alpha: 0.6,
            eeg_beta: 0.4,
            eeg_gamma: 0.3,
            eeg_alpha_cve: 0.5,
            eda: 0.2,
            motion_accel: 0.1,
            roh: 0.10,
            capability_tier: CapabilityTier::CapAugmentedCitizen,
            evolve_index: 50,
            session_epoch: 1200,
        };

        let nv = NeuroprintView::new(&snapshot, "self-bostrom");

        assert_eq!(nv.subject_id, "self-bostrom");
        assert!(nv.tree_of_life.decay <= 0.334);
        assert!(nv.tree_of_life.lifeforce >= 0.666);
        assert!(nv.diagnostics.sovereignty_integrity);
        assert_eq!(nv.nature_tokens.len(), 1);
        assert_eq!(nv.nature_tokens[0].label, "CALM_STABLE");
    }
}
