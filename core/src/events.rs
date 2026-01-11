use crate::types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KarmaChangeReason {
    EcoImpactIncrease,
    EcoImpactDecrease,
    SecurityIncident,
    FalsePositiveCorrection,
    RightsViolationCompensation,
    PlatformFairnessCorrection,
    ShapleyAllocation,
    SandboxExit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarmaChange {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub delta: f64,
    pub new_value: f64,
    pub reason: KarmaChangeReason,
    pub evidence_ref: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedgerEventKind {
    KarmaChange(KarmaChange),
    IdentityStateChange {
        identity_id: Uuid,
        from: IdentityState,
        to: IdentityState,
        reason: String,
    },
    RightsViolation {
        identity_id: Uuid,
        article: String,
        description: String,
    },
    PlatformFairnessAudit {
        platform_id: Uuid,
        previous_trust: f64,
        new_trust: f64,
        details: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEvent {
    pub id: Uuid,
    pub kind: LedgerEventKind,
    pub created_at: DateTime<Utc>,
}
