use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealityUpgradeProposal {
    pub proposalid: String,
    pub subjectid: String,
    pub module: String,
    pub kind: String,
    pub rohbefore: f32,
    pub rohafter: f32,
    pub tsafemode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RealityUpgradeDecision {
    Allowed,
    RejectedRoH,
    RejectedMode,
    RejectedTsafe,
}

pub struct RealityUpgradeKernel {
    pub roh_ceiling: f32,
}

impl RealityUpgradeKernel {
    pub fn new(roh_ceiling: f32) -> Self {
        Self { roh_ceiling }
    }

    pub fn evaluate(&self, p: &RealityUpgradeProposal) -> RealityUpgradeDecision {
        if p.rohafter > self.roh_ceiling + 1e-6 || p.rohafter > p.rohbefore + 1e-6 {
            return RealityUpgradeDecision::RejectedRoH;
        }
        if p.module != "RealityOS" {
            return RealityUpgradeDecision::RejectedMode;
        }
        if p.tsafemode != "COPILOT" {
            return RealityUpgradeDecision::RejectedTsafe;
        }
        RealityUpgradeDecision::Allowed
    }
}
