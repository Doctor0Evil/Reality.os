use serde::{Deserialize, Serialize};
use crate::did::DidUri;
use crate::eco_metrics::EcoImpactScore;
use chrono::{DateTime, Utc};

/// Organic CPU Validator: lab rigs, clinical gateways, eco-robotic hubs
/// with measured energy envelopes and audited cooling/placement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganicCpuValidator {
    /// Validator DID (e.g., did:aln:bostrom:organic_cpu:...)
    pub validator_did: DidUri,
    /// Human-readable name for the validator node
    pub name: String,
    /// Validator type (lab_rig, clinical_gateway, eco_robotic_hub, edge_node)
    pub validator_type: ValidatorType,
    /// Geographic location (jurisdiction)
    pub jurisdiction: String,
    /// Energy envelope (max joules per day)
    pub energy_envelope_joules_per_day: u64,
    /// Current energy usage (joules today)
    pub current_energy_usage_joules: u64,
    /// Eco-impact score (must be ≥ 0.86 for health validation)
    pub eco_impact_score: EcoImpactScore,
    /// Risk-of-harm rating (must be ≤ 0.3 for biophysical ops)
    pub risk_of_harm: f32,
    /// K/E/R scoreboard reference
    pub ker_scoreboard_id: Option<String>,
    /// Uptime percentage (last 30 days)
    pub uptime_percentage: f32,
    /// Missed precommits (last 30 days)
    pub missed_precommits: u32,
    /// RoH compliance score (0-1)
    pub roh_compliance_score: f32,
    /// Hardware attestation hash (low-toxicity, audited cooling)
    pub hardware_attestation_hash: String,
    /// ROW anchor height when this validator was admitted
    pub row_admission_height: u64,
    /// Last heartbeat timestamp
    pub last_heartbeat: DateTime<Utc>,
    /// Forward-only: true if currently active
    pub is_active: bool,
    /// Forward-only: true if currently validating
    pub is_validating: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidatorType {
    LabRig,
    ClinicalGateway,
    EcoRoboticHub,
    EdgeNode,
}

impl OrganicCpuValidator {
    /// Check if validator meets health validation requirements
    pub fn meets_health_validation_requirements(&self) -> bool {
        self.eco_impact_score.value >= 0.86
            && self.risk_of_harm <= 0.3
            && self.uptime_percentage >= 95.0
            && self.roh_compliance_score >= 0.95
    }

    /// Check if validator meets general validation requirements
    pub fn meets_general_validation_requirements(&self) -> bool {
        self.eco_impact_score.value >= 0.70
            && self.risk_of_harm <= 0.5
            && self.uptime_percentage >= 90.0
            && self.roh_compliance_score >= 0.90
    }

    /// Check if validator is currently healthy (heartbeat within 5 minutes)
    pub fn is_healthy(&self) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.last_heartbeat);
        elapsed.num_seconds() < 300 // 5 minutes
    }

    /// Calculate current energy usage percentage
    pub fn energy_usage_percentage(&self) -> f32 {
        if self.energy_envelope_joules_per_day == 0 {
            return 0.0;
        }
        (self.current_energy_usage_joules as f32
            / self.energy_envelope_joules_per_day as f32)
            * 100.0
    }

    /// Check if energy envelope is exceeded
    pub fn is_energy_envelope_exceeded(&self) -> bool {
        self.current_energy_usage_joules >= self.energy_envelope_joules_per_day
    }
}

/// Validator set shard (append-only list of all validators)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidatorSetShard {
    pub validators: Vec<OrganicCpuValidator>,
    pub total_validators: u32,
    pub active_validators: u32,
    pub last_updated_height: u64,
}

impl ValidatorSetShard {
    /// Get all active, healthy validators
    pub fn get_active_healthy_validators(&self) -> Vec<&OrganicCpuValidator> {
        self.validators
            .iter()
            .filter(|v| v.is_active && v.is_healthy())
            .collect()
    }

    /// Get validators suitable for health validation
    pub fn get_health_validators(&self) -> Vec<&OrganicCpuValidator> {
        self.validators
            .iter()
            .filter(|v| {
                v.is_active
                    && v.is_healthy()
                    && v.meets_health_validation_requirements()
            })
            .collect()
    }

    /// Append a new validator entry (forward-only)
    pub fn append_validator(&mut self, validator: OrganicCpuValidator, height: u64) {
        // Mark previous entry from same validator as not active
        for v in self.validators.iter_mut() {
            if v.validator_did == validator.validator_did {
                v.is_active = false;
            }
        }
        self.validators.push(validator);
        self.total_validators = self.validators.len() as u32;
        self.active_validators = self
            .validators
            .iter()
            .filter(|v| v.is_active)
            .count() as u32;
        self.last_updated_height = height;
    }

    /// Update validator heartbeat (forward-only, creates new entry)
    pub fn update_heartbeat(&mut self, validator_did: &DidUri, height: u64) {
        for v in self.validators.iter_mut() {
            if v.validator_did == validator_did && v.is_active {
                v.last_heartbeat = Utc::now();
            }
        }
        self.last_updated_height = height;
    }
}
