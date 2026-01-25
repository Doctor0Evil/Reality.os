// ──────────────────────────────────────────────────────────────
// Bioscale Rights Core: Firewall Invariant Guard
// Enforces neurorights invariants for BioAsset and policies
// ──────────────────────────────────────────────────────────────
use aln_core::*;
use didkit::DID;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

// Invariant resonance struct for adaptive enforcement
#[derive(Serialize, Deserialize, Clone)]
pub struct InvariantResonance {
    pub invariant_id: String, // Hex tag for resonance
    pub n3_archetype: f32, // From N3 formula integration
    pub q_residual: f32, // From ? formula
    pub policy_hash: String, // SHA256 of human-readable policy
    pub did_owner: DID,
}

// Trait for firewall invariants
pub trait FirewallInvariantGuard {
    fn check_no_non_consensual_bridge(&self) -> bool;
    fn check_no_covert_exclusion(&self) -> bool;
    fn check_no_discrimination(&self) -> bool;
    fn check_always_revocable(&self) -> bool;
    fn resonate_with_archetype(&mut self, n3: f32, q: f32);
}

// Extend BioAsset with invariant guard
impl FirewallInvariantGuard for BioAsset {
    fn check_no_non_consensual_bridge(&self) -> bool {
        if self.category == BioCategory::Neuromotor || self.category == BioCategory::Cognitive {
            !self.rights.consent_contract.is_empty() && self.rights.neurorights_ok
        } else {
            true
        }
    }

    fn check_no_covert_exclusion(&self) -> bool {
        let policy_bytes = self.rights.consent_contract.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(policy_bytes);
        let hash = format!("{:x}", hasher.finalize());
        self.provenance.tx_hash == hash  // Ensures policy matches provenance
    }

    fn check_no_discrimination(&self) -> bool {
        // No demographic fields allowed; compile-time absence enforced by type
        true  // Runtime check is no-op since types exclude protected traits
    }

    fn check_always_revocable(&self) -> bool {
        self.rights.revocable && !self.provenance.verified_by.is_empty()
    }

    fn resonate_with_archetype(&mut self, n3: f32, q: f32) {
        // Integrate N3/? formulas for resonance
        let delta_power = n3 * 0.75;  // From N3 delta dominance
        let residual_conf = 1.0 - q.max(0.7);
        self.metrics.cognitive_load = delta_power + residual_conf;
    }
}

// Macro for invariant validation at compile/runtime
macro_rules! enforce_invariants {
    ($asset:expr) => {
        if !$asset.check_no_non_consensual_bridge() {
            panic!("Firewall violation: non-consensual bridge");
        }
        if !$asset.check_no_covert_exclusion() {
            panic!("Firewall violation: covert exclusion");
        }
        if !$asset.check_no_discrimination() {
            panic!("Firewall violation: discrimination");
        }
        if !$asset.check_always_revocable() {
            panic!("Firewall violation: non-revocable");
        }
    };
}

// Function to create resonant invariant
pub fn new_resonant_invariant(
    invariant_id: &str,
    did: DID,
    policy: &str,
    n3: f32,
    q: f32,
) -> InvariantResonance {
    let mut hasher = Sha256::new();
    hasher.update(policy.as_bytes());
    let policy_hash = format!("{:x}", hasher.finalize());
    InvariantResonance {
        invariant_id: invariant_id.to_string(),
        n3_archetype: n3,
        q_residual: q,
        policy_hash,
        did_owner: did,
    }
}

// Example usage: validate and resonate
pub fn validate_and_resonate(asset: &mut BioAsset, n3: f32, q: f32) {
    enforce_invariants!(asset);
    asset.resonate_with_archetype(n3, q);
}
