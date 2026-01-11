use crate::events::{KarmaChange, KarmaChangeReason, LedgerEvent, LedgerEventKind};
use crate::types::{Identity, IdentityFlags, IdentityState, IdentityStatus};
use chrono::Utc;
use uuid::Uuid;

pub const MIN_KARMA: f64 = 0.0;
pub const MAX_KARMA: f64 = 1.0;

/// High‑karma lower bound for protected identities.
/// For AugmentedCitizen, enforced ≥ 0.8 unless evidence of environmental harm.
pub const AUGMENTED_MIN_KARMA: f64 = 0.8;

/// Clamp utility.
fn clamp_karma(v: f64) -> f64 {
    v.clamp(MIN_KARMA, MAX_KARMA)
}

pub struct KarmaEngine;

impl KarmaEngine {
    pub fn apply_delta(
        identity: &mut Identity,
        status: &mut IdentityStatus,
        delta: f64,
        reason: KarmaChangeReason,
        evidence_ref: impl Into<String>,
    ) -> (KarmaChange, LedgerEvent) {
        let mut new_karma = identity.current_karma + delta;

        if matches!(identity.identity_type, crate::types::IdentityType::AugmentedCitizen) {
            // For Augmented citizens, never drop below 0.8 unless already below.
            if identity.current_karma >= AUGMENTED_MIN_KARMA && new_karma < AUGMENTED_MIN_KARMA {
                new_karma = AUGMENTED_MIN_KARMA;
            }
        }

        new_karma = clamp_karma(new_karma);
        identity.current_karma = new_karma;
        identity.updated_at = Utc::now();

        let change = KarmaChange {
            id: Uuid::new_v4(),
            identity_id: identity.id,
            delta,
            new_value: new_karma,
            reason,
            evidence_ref: evidence_ref.into(),
            timestamp: Utc::now(),
        };

        let event = LedgerEvent {
            id: Uuid::new_v4(),
            kind: LedgerEventKind::KarmaChange(change.clone()),
            created_at: Utc::now(),
        };

        (change, event)
    }

    pub fn can_apply_negative_trust_update(flags: &IdentityFlags, under_attack: bool) -> bool {
        if flags.neuro_linked || flags.cognitive_safety_required {
            if under_attack {
                return false;
            }
        }
        true
    }

    pub fn should_escalate_review(delta: f64, identity: &Identity) -> bool {
        let magnitude = delta.abs();
        magnitude >= 0.05 || identity.current_karma >= 0.8
    }

    pub fn transition_state(
        status: &mut IdentityStatus,
        to: IdentityState,
        reason: impl Into<String>,
    ) -> LedgerEvent {
        let from = status.state.clone();
        status.state = to;
        status.last_transition = Utc::now();

        LedgerEvent {
            id: Uuid::new_v4(),
            kind: LedgerEventKind::IdentityStateChange {
                identity_id: status.identity_id,
                from,
                to: status.state.clone(),
                reason: reason.into(),
            },
            created_at: Utc::now(),
        }
    }
}
