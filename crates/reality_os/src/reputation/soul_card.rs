use serde::{Deserialize, Serialize};

/// Soul-bound reputation snapshot for an augmented citizen.
/// No monetary fields, forward-only evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulReputationCard {
    /// Bostrom / ALN DID of the citizen.
    pub did: String,
    /// Total validated hours spent in missions (clinical, eco, training).
    pub validated_hours: f64,
    /// Count of successfully completed missions.
    pub missions_completed: u32,
    /// Safety score in [0,1], penalized by incident density.
    pub safety_score: f32,
    /// Learning contribution score in [0,1] (annotations, code, reviews).
    pub learning_score: f32,
    /// Care and assistance contribution score in [0,1].
    pub care_score: f32,
    /// Composite CAC index in [0,1], computed from the three axes.
    pub cac_index: f32,
    /// Count of filed safety incidents involving this DID (as reporter or subject).
    pub incidents_reported: u32,
    /// Last immutable ledger height where this snapshot was anchored.
    pub last_anchored_height: u64,
}

impl SoulReputationCard {
    /// Recompute the composite CAC index from axis scores.
    /// Forward-only: callers are expected to only persist strictly newer snapshots.
    pub fn recompute_cac_index(&mut self) {
        // Simple weighted average; weights can be made configurable on-chain.
        let w_safety: f32 = 0.4;
        let w_learning: f32 = 0.35;
        let w_care: f32 = 0.25;

        let raw = w_safety * self.safety_score
            + w_learning * self.learning_score
            + w_care * self.care_score;

        // Clamp to [0,1] to avoid drift from numerical error.
        self.cac_index = raw.clamp(0.0, 1.0);
    }

    /// Returns a human-readable label for UI badges.
    pub fn grade_label(&self) -> &'static str {
        match self.cac_index {
            x if x >= 0.90 => "Exemplar",
            x if x >= 0.75 => "Trusted",
            x if x >= 0.50 => "Contributor",
            x if x >  0.20 => "Learner",
            _              => "New / Under Review",
        }
    }
}
