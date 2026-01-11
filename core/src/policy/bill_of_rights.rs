use crate::events::{LedgerEvent, LedgerEventKind};
use crate::types::Identity;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum EvidenceMode {
    Public,
    HashOnly,
    Internal,
}

#[derive(Debug, Clone)]
pub struct ProposedChange {
    pub identity_id: Uuid,
    pub delta_eco: f64,
    pub delta_contrib: f64,
    pub evidence_mode: EvidenceMode,
    pub reason: String,
}

pub struct EcoBillOfRights;

impl EcoBillOfRights {
    /// Article 1: no penalty when data is hash‑only or internal.
    pub fn enforce_article_1(
        identity: &Identity,
        proposed: &ProposedChange,
    ) -> Result<(), LedgerEvent> {
        if proposed.delta_eco < 0.0 || proposed.delta_contrib < 0.0 {
            match proposed.evidence_mode {
                EvidenceMode::HashOnly | EvidenceMode::Internal => {
                    let ev = LedgerEvent {
                        id: Uuid::new_v4(),
                        kind: LedgerEventKind::RightsViolation {
                            identity_id: identity.id,
                            article: "Bill-of-Rights Article 1".into(),
                            description: "Penalty proposed without public data".into(),
                        },
                        created_at: Utc::now(),
                    };
                    return Err(ev);
                }
                EvidenceMode::Public => {}
            }
        }
        Ok(())
    }

    /// Article 2: high karma review.
    pub fn requires_high_karma_review(identity: &Identity, proposed_delta_karma: f64) -> bool {
        identity.current_karma >= 0.8 && proposed_delta_karma < 0.0
    }

    /// Article 3: explanation requirement.
    pub fn validate_explanation(reason: &str, delta_karma: f64) -> bool {
        if delta_karma.abs() < 0.05 {
            return true;
        }
        !reason.trim().is_empty() && reason.len() >= 8
    }
}
