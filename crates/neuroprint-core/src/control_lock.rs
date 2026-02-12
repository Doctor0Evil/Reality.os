use serde::{Serialize, Deserialize};
use super::repair_elasticity::ElasticityResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockInputs {
    pub cp: f64,
    pub delta_bioload: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlLockResult {
    pub is_lock: bool,
    pub reason: String,
}

pub fn evaluate_lock(
    elasticity: &ElasticityResult,
    inputs: &LockInputs
) -> ControlLockResult {

    let lock = elasticity.rec_bounded < 0.0
        && inputs.cp > 0.7
        && inputs.delta_bioload <= 0.0;

    let reason = if lock {
        "High constraint pressure suppressing repair; no bioload improvement".into()
    } else {
        "No control-lock detected".into()
    };

    ControlLockResult {
        is_lock: lock,
        reason,
    }
}
