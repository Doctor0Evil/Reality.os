use std::collections::HashMap;
use crate::did_verification::{DIDSignature, DIDVerificationResult, verify_did_signature, did_to_public_key};

/// Categories of tokenized assets under protection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetKind {
    BrainToken,
    EcoToken,
    EvolutionPoint,
    UpgradeEntitlement,
}

/// Integration types for cybernetic enhancements (software-only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegrationType {
    CognitiveLoadReduction,
    FocusStabilization,
    DreamExcavationDepth,
    GovernanceAssistant,
    EcoOptimizer,
}

/// Minimal per-asset record with provenance and eco-impact.
#[derive(Debug, Clone)]
pub struct TokenizedAsset {
    pub id: String,
    pub kind: AssetKind,
    pub units: u64,
    pub provenance_hash: String, // e.g., SHA3-256 hex
    pub eco_energynj: f32,       // energy spent or saved (positive = saved)
}

/// Nightly eco-profile summary, aligned with N-stage and cluster shards.
#[derive(Debug, Clone)]
pub struct NightlyEcoProfile {
    pub avg_flops_per_epoch: f32,
    pub avg_energynj_per_epoch: f32,
    pub energy_reduction_vs_baseline_pct: f32,
    pub device_hours_saved: f32,
    pub cognitive_clarity_score_01: f32,
    pub emotional_salience_score_01: f32,
}

/// Governance policy for upgrades and assistance.
#[derive(Debug, Clone)]
pub struct UpgradePolicy {
    /// Minimum eco tokens required to unlock evolution upgrades.
    pub min_eco_tokens_for_upgrade: u64,
    /// Minimum nightly energy reduction percentage for eligibility.
    pub min_energy_reduction_pct: f32,
    /// Minimum cognitive clarity to allow high-intensity modules.
    pub min_clarity_for_heavy: f32,
    /// Maximum allowed stress proxy (1 - clarity * eco alignment).
    pub max_stress_score: f32,
}

/// Aggregate wallet and state for one host.
#[derive(Debug, Default)]
pub struct AssetWallet {
    pub assets: HashMap<AssetKind, u64>,
}

impl AssetWallet {
    pub fn credit(&mut self, kind: AssetKind, amount: u64) {
        *self.assets.entry(kind).or_insert(0) += amount;
    }

    pub fn balance(&self, kind: AssetKind) -> u64 {
        self.assets.get(&kind).copied().unwrap_or(0)
    }

    pub fn try_debit(&mut self, kind: AssetKind, amount: u64) -> bool {
        let bal = self.balance(kind);
        if bal < amount {
            return false;
        }
        self.assets.insert(kind, bal - amount);
        true
    }
}

/// Decision outcome for requested integration upgrade.
#[derive(Debug, Clone)]
pub struct UpgradeDecision {
    pub allowed: bool,
    pub reason: String,
    pub did_signature: Option<DIDSignature>,
}

/// Core eco-governor for tokenized assets and upgrades with DID verification.
pub struct AssetEcoGovernor {
    policy: UpgradePolicy,
    stakeholder_did: String,
}

impl AssetEcoGovernor {
    pub fn new(policy: UpgradePolicy, stakeholder_did: &str) -> Self {
        Self {
            policy,
            stakeholder_did: stakeholder_did.to_string(),
        }
    }

    /// Verify DID signature before processing upgrade
    pub fn verify_did_signature(
        &self,
        signature: &DIDSignature,
        message: &str,
    ) -> Result<DIDVerificationResult, anyhow::Error> {
        let public_key = did_to_public_key(&self.stakeholder_did)?;
        verify_did_signature(signature, &public_key)
    }

    /// Compute a simple eco-alignment score from nightly profile.
    fn eco_alignment_score(&self, profile: &NightlyEcoProfile) -> f32 {
        // Normalize energy reduction to 0–1 and combine with device-hours saved.
        let reduction_norm = (profile.energy_reduction_vs_baseline_pct / 100.0).clamp(0.0, 1.0);
        let hours_norm = (profile.device_hours_saved / 3.0).clamp(0.0, 1.0);
        0.6 * reduction_norm + 0.4 * hours_norm
    }

    /// Estimate a stress proxy from clarity and eco-alignment.
    fn stress_score(&self, profile: &NightlyEcoProfile) -> f32 {
        let eco_score = self.eco_alignment_score(profile);
        (1.0 - profile.cognitive_clarity_score_01 * eco_score).clamp(0.0, 1.0)
    }

    /// Decide if a requested integration-type upgrade is allowed.
    pub fn decide_upgrade(
        &self,
        wallet: &mut AssetWallet,
        nightly: &NightlyEcoProfile,
        integration: IntegrationType,
        eco_cost: u64,
        evolution_cost: u64,
        signature: Option<DIDSignature>,
    ) -> UpgradeDecision {
        // Verify DID signature is present and valid
        if let Some(sig) = signature {
            let message = format!(
                "upgrade:{}:eco_cost:{}:evolution_cost:{}",
                integration as i32,
                eco_cost,
                evolution_cost
            );
            
            match self.verify_did_signature(&sig, &message) {
                Ok(DIDVerificationResult::Valid) => {}
                Ok(DIDVerificationResult::Invalid) => {
                    return UpgradeDecision {
                        allowed: false,
                        reason: "Invalid DID signature for upgrade request".to_string(),
                        did_signature: Some(sig),
                    };
                }
                Ok(DIDVerificationResult::MissingSignature) => {
                    return UpgradeDecision {
                        allowed: false,
                        reason: "Missing DID signature for upgrade request".to_string(),
                        did_signature: None,
                    };
                }
                Ok(DIDVerificationResult::InvalidFormat) => {
                    return UpgradeDecision {
                        allowed: false,
                        reason: "Invalid DID signature format".to_string(),
                        did_signature: Some(sig),
                    };
                }
                Err(e) => {
                    return UpgradeDecision {
                        allowed: false,
                        reason: format!("DID verification error: {}", e),
                        did_signature: Some(sig),
                    };
                }
            }
        } else {
            return UpgradeDecision {
                allowed: false,
                reason: "DID signature required for upgrade request".to_string(),
                did_signature: None,
            };
        }

        let eco_balance = wallet.balance(AssetKind::EcoToken);
        let evo_balance = wallet.balance(AssetKind::EvolutionPoint);

        if eco_balance < self.policy.min_eco_tokens_for_upgrade {
            return UpgradeDecision {
                allowed: false,
                reason: format!(
                    "Insufficient eco tokens: have {}, need at least {}.",
                    eco_balance, self.policy.min_eco_tokens_for_upgrade
                ),
                did_signature: signature,
            };
        }

        if nightly.energy_reduction_vs_baseline_pct < self.policy.min_energy_reduction_pct {
            return UpgradeDecision {
                allowed: false,
                reason: format!(
                    "Energy reduction {:.1}% below required {:.1}%.",
                    nightly.energy_reduction_vs_baseline_pct,
                    self.policy.min_energy_reduction_pct
                ),
                did_signature: signature,
            };
        }

        let stress = self.stress_score(nightly);
        let clarity = nightly.cognitive_clarity_score_01;

        // High-intensity integration-types require stricter clarity/stress limits.
        let heavy = matches!(
            integration,
            IntegrationType::DreamExcavationDepth | IntegrationType::FocusStabilization
        );

        if heavy {
            if clarity < self.policy.min_clarity_for_heavy {
                return UpgradeDecision {
                    allowed: false,
                    reason: format!(
                        "Clarity {:.2} below required {:.2} for heavy integration.",
                        clarity, self.policy.min_clarity_for_heavy
                    ),
                    did_signature: signature,
                };
            }
            if stress > self.policy.max_stress_score {
                return UpgradeDecision {
                    allowed: false,
                    reason: format!(
                        "Stress score {:.2} exceeds maximum {:.2} for safe heavy upgrade.",
                        stress, self.policy.max_stress_score
                    ),
                    did_signature: signature,
                };
            }
        }

        if eco_balance < eco_cost || evo_balance < evolution_cost {
            return UpgradeDecision {
                allowed: false,
                reason: format!(
                    "Insufficient balances: eco {} / need {}, evolution {} / need {}.",
                    eco_balance, eco_cost, evo_balance, evolution_cost
                ),
                did_signature: signature,
            };
        }

        // All checks passed: debit and approve.
        if !wallet.try_debit(AssetKind::EcoToken, eco_cost)
            || !wallet.try_debit(AssetKind::EvolutionPoint, evolution_cost)
        {
            return UpgradeDecision {
                allowed: false,
                reason: "Concurrent debit conflict; retry upgrade decision.".to_string(),
                did_signature: signature,
            };
        }

        UpgradeDecision {
            allowed: true,
            reason: format!(
                "Upgrade {:?} approved with eco_cost={} and evolution_cost={}.",
                integration, eco_cost, evolution_cost
            ),
            did_signature: signature,
        }
    }
}
