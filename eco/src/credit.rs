use crate::shapley::{compute_shapley, Participant, ShapleyAllocation};
use reality_core::events::{KarmaChangeReason, LedgerEvent};
use reality_core::karma::KarmaEngine;
use reality_core::types::Identity;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NodeContributionEvidence {
    pub identity_id: Uuid,
    pub marginal_eco_impact: Vec<f64>,
    pub evidence_ref: String,
}

/// Allocate eco_impact_score and karma via Shapley values.
pub fn allocate_node_impact(
    identities: &mut [Identity],
    evidences: &[NodeContributionEvidence],
) -> (Vec<ShapleyAllocation>, Vec<LedgerEvent>) {
    let participants: Vec<Participant> = evidences
        .iter()
        .map(|e| Participant {
            id: e.identity_id,
            marginal_contributions: e.marginal_eco_impact.clone(),
        })
        .collect();

    let shapley = compute_shapley(&participants);
    let mut events = Vec::new();

    for alloc in &shapley {
        if let Some(id_ref) = identities.iter_mut().find(|id| id.id == alloc.participant_id) {
            id_ref.eco_impact_score = (id_ref.eco_impact_score + alloc.value).clamp(0.0, 1.0);
            let (_change, ev) = KarmaEngine::apply_delta(
                id_ref,
                &mut reality_core::types::IdentityStatus {
                    identity_id: id_ref.id,
                    state: reality_core::types::IdentityState::Normal,
                    last_transition: chrono::Utc::now(),
                },
                alloc.value * 0.05,
                KarmaChangeReason::ShapleyAllocation,
                "CEIM node impact allocation",
            );
            events.push(ev);
        }
    }

    (shapley, events)
}
