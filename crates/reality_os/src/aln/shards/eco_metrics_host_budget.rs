use serde::{Deserialize, Serialize};
use crate::did::DidUri;
use chrono::{DateTime, Utc};

/// Eco-Impact Score (0-1 scale, higher is better)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoImpactScore {
    pub value: f32,
    pub gco2_per_joule: f32,
    pub renewable_energy_percentage: f32,
    pub hardware_toxicity_score: f32, // 0 = non-toxic, 1 = highly toxic
    pub cooling_efficiency: f32,
    pub last_calculated: DateTime<Utc>,
}

impl EcoImpactScore {
    /// Calculate eco-impact score from component metrics
    pub fn calculate(
        gco2_per_joule: f32,
        renewable_percentage: f32,
        toxicity_score: f32,
        cooling_efficiency: f32,
    ) -> Self {
        // Weighted formula: higher renewable, lower toxicity, better cooling = higher score
        let renewable_component = renewable_percentage / 100.0;
        let toxicity_component = 1.0 - toxicity_score;
        let cooling_component = cooling_efficiency;
        let carbon_component = 1.0 - (gco2_per_joule / 10.0).min(1.0);

        let value = (renewable_component * 0.3
            + toxicity_component * 0.25
            + cooling_component * 0.25
            + carbon_component * 0.2)
            .clamp(0.0, 1.0);

        Self {
            value,
            gco2_per_joule,
            renewable_energy_percentage: renewable_percentage,
            hardware_toxicity_score: toxicity_score,
            cooling_efficiency,
            last_calculated: Utc::now(),
        }
    }

    /// Check if score meets minimum floor
    pub fn meets_floor(&self, floor: f32) -> bool {
        self.value >= floor
    }
}

/// Host Budget: energy, compute, and bandwidth limits per citizen device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostBudget {
    /// Citizen DID
    pub citizen_did: DidUri,
    /// Host device DID (phone, gateway, implant, etc.)
    pub host_device_did: DidUri,
    /// Daily energy budget (millijoules)
    pub daily_energy_budget_mj: u64,
    /// Current energy usage today (millijoules)
    pub current_energy_usage_mj: u64,
    /// Daily compute budget (gas units)
    pub daily_compute_budget: u64,
    /// Current compute usage today (gas units)
    pub current_compute_usage: u64,
    /// Daily bandwidth budget (bytes)
    pub daily_bandwidth_budget_bytes: u64,
    /// Current bandwidth usage today (bytes)
    pub current_bandwidth_usage_bytes: u64,
    /// Budget reset time (UTC midnight)
    pub budget_reset_time: DateTime<Utc>,
    /// ROW anchor height when budget was set
    pub row_anchor_height: u64,
    /// Forward-only: true if this is the current budget
    pub is_current: bool,
}

impl HostBudget {
    /// Check if energy budget is exceeded
    pub fn is_energy_budget_exceeded(&self) -> bool {
        self.current_energy_usage_mj >= self.daily_energy_budget_mj
    }

    /// Check if compute budget is exceeded
    pub fn is_compute_budget_exceeded(&self) -> bool {
        self.current_compute_usage >= self.daily_compute_budget
    }

    /// Check if bandwidth budget is exceeded
    pub fn is_bandwidth_budget_exceeded(&self) -> bool {
        self.current_bandwidth_usage_bytes >= self.daily_bandwidth_budget_bytes
    }

    /// Get energy usage percentage
    pub fn energy_usage_percentage(&self) -> f32 {
        if self.daily_energy_budget_mj == 0 {
            return 0.0;
        }
        (self.current_energy_usage_mj as f32 / self.daily_energy_budget_mj as f32) * 100.0
    }

    /// Check if any budget is exceeded
    pub fn is_any_budget_exceeded(&self) -> bool {
        self.is_energy_budget_exceeded()
            || self.is_compute_budget_exceeded()
            || self.is_bandwidth_budget_exceeded()
    }

    /// Create a new budget entry (forward-only, never mutates existing)
    pub fn create_new_budget(
        base: &HostBudget,
        new_energy_mj: u64,
        new_compute: u64,
        new_bandwidth: u64,
        height: u64,
    ) -> HostBudget {
        HostBudget {
            citizen_did: base.citizen_did.clone(),
            host_device_did: base.host_device_did.clone(),
            daily_energy_budget_mj: new_energy_mj,
            current_energy_usage_mj: 0,
            daily_compute_budget: new_compute,
            current_compute_usage: 0,
            daily_bandwidth_budget_bytes: new_bandwidth,
            current_bandwidth_usage_bytes: 0,
            budget_reset_time: Utc::now() + chrono::Duration::days(1),
            row_anchor_height: height,
            is_current: true,
        }
    }
}

/// Eco Attestation: third-party verification of eco-metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoAttestation {
    /// Attestation ID
    pub attestation_id: String,
    /// Validator DID being attested
    pub validator_did: DidUri,
    /// Attester DID (third-party auditor)
    pub attester_did: DidUri,
    /// Eco-impact score at time of attestation
    pub eco_impact_score: EcoImpactScore,
    /// Attestation timestamp
    pub attestation_time: DateTime<Utc>,
    /// Expiry time (attestations expire after 90 days)
    pub expiry_time: DateTime<Utc>,
    /// ROW anchor height
    pub row_anchor_height: u64,
    /// Forward-only: true if currently valid
    pub is_valid: bool,
}

impl EcoAttestation {
    /// Check if attestation is still valid (not expired)
    pub fn is_currently_valid(&self) -> bool {
        self.is_valid && Utc::now() < self.expiry_time
    }
}
