// ──────────────────────────────────────────────────────────────
// Bioscale Fairness Audit & OrganicCpu Router Thresholds
// SovereignValidatorResonance for DID-native, biophysical domains
// ──────────────────────────────────────────────────────────────

use aln_core::*;
use didkit::DID;
use serde::{Serialize, Deserialize};

/// Biophysical token kinds, aligned with non-monetary rights budgets.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BioTokenKind {
    BloodToken,      // neuromotor workload / recovery
    BrainToken,      // computational exposure budget
    ProteinToken,    // learning / plasticity budget
    EcoScoreToken,   // eco-positive behavior / device-hours
    InclusionToken,  // access guarantees, non-scarce for basics
}

/// OrganicCpu router thresholds for a single, localized host.
/// These are non-deployed metrics: configuration and analysis only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganicCpuRouterThresholds {
    pub max_brain_tokens_per_hour: i64,   // exposure budget per hour
    pub max_blood_tokens_per_day: i64,    // neuromotor work per day
    pub max_concurrent_domains: u8,       // parallel domain engagements
    pub dracula_wave_cap: f32,            // 0.0–0.5: max fraction for high-intensity modes
}

/// Token audit entry for non-exclusion metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAuditEntry {
    pub token_kind: BioTokenKind,
    pub balance_variance: f32, // variance across DIDs (offshore/local sample)
    pub min_guaranteed: i64,   // minimum InclusionToken per domain or station
    pub exclusion_flag: bool,  // true if structural disparity detected
}

/// Sovereign validator state for a single biophysical domain.
/// This is data for organic_cpu validation, not a deployed node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignValidatorResonance {
    pub domain_id: String,                // e.g. "neuromotor_net", "cognitive_token_net"
    pub validator_did: DID,              // sovereign DID for this organic_cpu scope
    pub local_router: OrganicCpuRouterThresholds,
    pub n3_archetype_quorum: f32,        // N3-derived quorum for consensus (0–1)
    pub q_residual_proof: f32,           // ?-residual metric for proof-of-resonance (0–1)
    pub token_distributions: Vec<TokenAuditEntry>,
    pub bostrom_anchor_hex: String,      // hex tag for jurisdiction/policy profile
}

/// Trait defining sovereign validator workflows for organic_cpu scopes.
pub trait SovereignValidatorWorkflow {
    /// N3/?-grounded consensus: returns true when local metrics satisfy quorum.
    fn validate_consensus(&self, quorum: f32) -> bool;

    /// Audit non-exclusion using token distribution metrics.
    fn audit_non_exclusion(&mut self, distributions: Vec<TokenAuditEntry>);

    /// Apply a high-intensity "dracula_wave" spend, respecting dracula_wave_cap.
    fn apply_dracula_wave_cap(&mut self, spend_fraction: f32) -> Result<(), &'static str>;
}

impl SovereignValidatorWorkflow for SovereignValidatorResonance {
    fn validate_consensus(&self, quorum: f32) -> bool {
        // Grounded in N3 and ? formulas from Biospectre Doctrine.
        // N3: delta-dominant deep sleep -> stronger archetype quorum
        // ? : residual uncertainty -> careful weighting for proof-of-resonance

        let delta_component = self.n3_archetype_quorum * 0.75;
        let residual_component = if self.q_residual_proof < 0.7 {
            // high residual (S? < 0.7) contributes positively
            (0.7 - self.q_residual_proof) * 0.5
        } else {
            0.0
        };

        delta_component + residual_component >= quorum
    }

    fn audit_non_exclusion(&mut self, distributions: Vec<TokenAuditEntry>) {
        // Enforce: variance must remain low, minimum InclusionToken above floor.
        // No demographic fields are ever used here; only token metrics.
        for mut entry in distributions {
            if entry.token_kind == BioTokenKind::InclusionToken {
                if entry.balance_variance > 0.10 || entry.min_guaranteed < 100 {
                    entry.exclusion_flag = true;
                } else {
                    entry.exclusion_flag = false;
                }
            }
            self.token_distributions.push(entry);
        }
    }

    fn apply_dracula_wave_cap(&mut self, spend_fraction: f32) -> Result<(), &'static str> {
        // spend_fraction is 0.0–1.0 relative to full organic_cpu high-intensity budget.
        // Enforce cap: never exceed configured dracula_wave_cap (e.g. 0.5 = 50%).
        if spend_fraction < 0.0 || spend_fraction > 1.0 {
            return Err("Invalid spend fraction");
        }
        if spend_fraction > self.local_router.dracula_wave_cap {
            return Err("Dracula_wave spend exceeds configured cap");
        }
        Ok(())
    }
}

/// Create a sovereign validator profile for a given domain and host.
/// All parameters are local, non-deployed metrics for this organic_cpu.
pub fn create_sovereign_validator(
    domain_id: &str,
    validator_did: DID,
    router_thresholds: OrganicCpuRouterThresholds,
    n3_archetype_quorum: f32,
    q_residual_proof: f32,
    bostrom_anchor_hex: &str,
) -> SovereignValidatorResonance {
    SovereignValidatorResonance {
        domain_id: domain_id.to_string(),
        validator_did,
        local_router: router_thresholds,
        n3_archetype_quorum,
        q_residual_proof,
        token_distributions: Vec::new(),
        bostrom_anchor_hex: bostrom_anchor_hex.to_string(),
    }
}

/// Run a fairness audit across multiple sovereign validator scopes.
/// This function uses only local/offshore metrics; no on-chain node routing.
pub fn run_fairness_audit(
    validators: &mut [SovereignValidatorResonance],
    distributions: Vec<TokenAuditEntry>,
    quorum: f32,
) -> bool {
    let mut all_valid = true;

    for validator in validators.iter_mut() {
        if !validator.validate_consensus(quorum) {
            all_valid = false;
            continue;
        }

        validator.audit_non_exclusion(distributions.clone());

        // Example high-intensity request at 40% of allowed budget.
        if let Err(_) = validator.apply_dracula_wave_cap(0.4) {
            all_valid = false;
        }
    }

    all_valid
}

/// Example: configure a neuromotor domain profile for a localized organic_cpu.
pub fn neuromotor_domain_profile(did: DID, bostrom_anchor_hex: &str) -> SovereignValidatorResonance {
    let router = OrganicCpuRouterThresholds {
        max_brain_tokens_per_hour: 120,
        max_blood_tokens_per_day: 1_000,
        max_concurrent_domains: 3,
        dracula_wave_cap: 0.5,
    };

    create_sovereign_validator(
        "neuromotor_net",
        did,
        router,
        0.8, // strong N3 archetype quorum requirement
        0.6, // moderate ?-residual confidence
        bostrom_anchor_hex,
    )
}
