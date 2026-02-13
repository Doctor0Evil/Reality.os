use std::time::{Duration, SystemTime};
use bioscale_upgrade_store::{HostBudget, ReversalConditions, UpgradeDecision, UpgradeDescriptor};
use cybernetic_evolution_points::{DowngradeContractBinding, DowngradeContractClient};
use phoenixlab_cyberswarm_neurostack_router::GatewayStateSnapshot;

#[derive(Clone, Debug)]
pub struct NanoswarmComplianceFieldV1 {
    pub il6_level: f32,
    pub hrv_ms: f32,
    pub eeg_load: f32,
    pub kernel_distance: f32,
    pub duty_cycle: f32,
    pub thresholds: ComplianceThresholds,
    pub observed_at: SystemTime,
}

#[derive(Clone, Debug)]
pub struct ComplianceThresholds {
    pub max_il6: f32,
    pub min_hrv_ms: f32,
    pub max_eeg_load: f32,
    pub min_kernel_distance: f32,
    pub max_duty_cycle: f32,
}

impl ComplianceThresholds {
    pub fn from_reversal(rev: ReversalConditions) -> Self {
        let max_il6 = (rev.max_inflammation_score * 5.0).clamp(0.0, 10.0);
        let min_hrv_ms = 30.0;
        let max_eeg_load = 0.6;
        let min_kernel_distance = 0.10;
        let max_duty_cycle = 0.4;
        Self {
            max_il6,
            min_hrv_ms,
            max_eeg_load,
            min_kernel_distance,
            max_duty_cycle,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ComplianceDecision {
    Safe,
    Brake { reason: &'static str },
    RollbackRequired { reason: &'static str },
}

impl NanoswarmComplianceFieldV1 {
    pub fn evaluate(&self) -> ComplianceDecision {
        if self.kernel_distance < self.thresholds.min_kernel_distance {
            return ComplianceDecision::Brake {
                reason: "kernel distance below Lyapunov margin",
            };
        }
        if self.duty_cycle > self.thresholds.max_duty_cycle {
            return ComplianceDecision::Brake {
                reason: "duty cycle exceeds envelope",
            };
        }
        if self.il6_level > self.thresholds.max_il6 {
            return ComplianceDecision::RollbackRequired {
                reason: "IL-6 above inflammatory limit",
            };
        }
        if self.hrv_ms < self.thresholds.min_hrv_ms {
            return ComplianceDecision::RollbackRequired {
                reason: "HRV below stress floor",
            };
        }
        if self.eeg_load > self.thresholds.max_eeg_load {
            return ComplianceDecision::RollbackRequired {
                reason: "EEG neuromorphic load above cap",
            };
        }
        ComplianceDecision::Safe
    }
}

pub fn enforce_nanoswarm_compliance<C: DowngradeContractClient>(
    host: &mut HostBudget,
    upgrade: &UpgradeDescriptor,
    binding: &DowngradeContractBinding,
    compliance: &NanoswarmComplianceFieldV1,
    client: &C,
    now: SystemTime,
) -> UpgradeDecision {
    match compliance.evaluate() {
        ComplianceDecision::Safe => UpgradeDecision::Approved {
            scheduled_at: now,
            expected_completion: now,
        },
        ComplianceDecision::Brake { reason } => UpgradeDecision::Denied { reason },
        ComplianceDecision::RollbackRequired { reason: _ } => {
            let decision = client.may_downgrade(host.clone(), upgrade.clone(), binding.clone());
            if !decision.allowed {
                return UpgradeDecision::Denied {
                    reason: "rollback required but contract denied",
                };
            }
            UpgradeDecision::Denied {
                reason: "rollback authorized; evolution blocked until downgrade completes",
            }
        }
    }
}
