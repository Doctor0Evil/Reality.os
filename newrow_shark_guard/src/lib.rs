use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Domain {
    AlienGameRule,
    SceneVariant,
    NpcDialogTemplate,
    LabCapLabBenchConfig,
    CapState,
    ConsentState,
    QpuSafety,
    Wallet,
    HudLiveOverlay,
    Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewRowOp {
    pub source: String,      // "NEWROW-SHARK"
    pub domain: Domain,
    pub schema: String,      // concrete table/schema name
    pub is_runtime: bool,    // true if hitting live runtime shards
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewRowGuardResult {
    pub allowed: bool,
    pub reason: String,
}

fn is_whitelisted_design_domain(d: &Domain) -> bool {
    matches!(
        d,
        Domain::AlienGameRule
            | Domain::SceneVariant
            | Domain::NpcDialogTemplate
            | Domain::LabCapLabBenchConfig
    )
}

fn is_blacklisted_sensitive_domain(d: &Domain) -> bool {
    matches!(
        d,
        Domain::CapState
            | Domain::ConsentState
            | Domain::QpuSafety
            | Domain::Wallet
            | Domain::HudLiveOverlay
    )
}

/// Core invariant: NewRow-Shark is design-only, infra-only, and non-actuating.
pub fn eval_newrow_invariant(op: &NewRowOp) -> NewRowGuardResult {
    if op.source != "NEWROW-SHARK" {
        return NewRowGuardResult {
            allowed: false,
            reason: "op.source != NEWROW-SHARK".to_string(),
        };
    }

    // Must never target sensitive domains
    if is_blacklisted_sensitive_domain(&op.domain) {
        return NewRowGuardResult {
            allowed: false,
            reason: "Forbidden domain for NewRow-Shark".to_string(),
        };
    }

    // Must only operate inside design domains
    if !is_whitelisted_design_domain(&op.domain) {
        return NewRowGuardResult {
            allowed: false,
            reason: "Non-whitelisted domain for NewRow-Shark".to_string(),
        };
    }

    // Must never touch live runtime state
    if op.is_runtime {
        return NewRowGuardResult {
            allowed: false,
            reason: "NewRow-Shark cannot operate on live runtime state".to_string(),
        };
    }

    NewRowGuardResult {
        allowed: true,
        reason: "NewRow-Shark invariant satisfied".to_string(),
    }
}
