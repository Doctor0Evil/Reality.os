use reality_core::types::{Identity, IdentityFlags, IdentityState, IdentityStatus, IdentityType};
use chrono::Utc;
use uuid::Uuid;

pub fn new_augmented_citizen() -> (Identity, IdentityStatus) {
    let flags = IdentityFlags {
        neuro_linked: true,
        data_sensitivity_high: true,
        under_attack_risk: true,
        cognitive_safety_required: true,
        protected: true,
    };

    let now = Utc::now();
    let identity = Identity {
        id: Uuid::new_v4(),
        identity_type: IdentityType::AugmentedCitizen,
        flags,
        eco_impact_score: 0.0,
        current_karma: 0.8,
        security_trust_score: 1.0,
        contribution_score: 0.0,
        created_at: now,
        updated_at: now,
    };

    let status = IdentityStatus {
        identity_id: identity.id,
        state: IdentityState::Normal,
        last_transition: now,
    };

    (identity, status)
}
