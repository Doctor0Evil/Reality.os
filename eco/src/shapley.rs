use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Minimal Shapley calculator for eco contributions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: Uuid,
    pub marginal_contributions: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapleyAllocation {
    pub participant_id: Uuid,
    pub value: f64,
}

/// Simple average marginal contribution approximation.
pub fn compute_shapley(participants: &[Participant]) -> Vec<ShapleyAllocation> {
    participants
        .iter()
        .map(|p| {
            let v = if p.marginal_contributions.is_empty() {
                0.0
            } else {
                p.marginal_contributions.iter().sum::<f64>()
                    / p.marginal_contributions.len() as f64
            };
            ShapleyAllocation {
                participant_id: p.id,
                value: v,
            }
        })
        .collect()
}
