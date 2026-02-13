#![forbid(unsafe_code)]

use std::time::SystemTime;
use bioscale_upgrade_store::{
    HostBudget, ThermodynamicEnvelope, EvidenceBundle, UpgradeDescriptor, UpgradeDecision,
    BioscaleUpgradeStore,
};
use reality_os_cargoenv::CargoEnvDescriptor;
use cyberswarm_router::CyberSwarmNeurostackRouter;
use phoenix_lab_aln::ALNComplianceParticle;

/// A single, routed Reality.os biostream over nanobyte-territory.
#[derive(Clone, Debug)]
pub struct RealityBiostream {
    pub env: CargoEnvDescriptor,
    pub host_budget: HostBudget,
    pub thermo: ThermodynamicEnvelope,
    pub evidence: EvidenceBundle,
    pub neurorights: ALNComplianceParticle,
}

impl RealityBiostream {
    /// Construct a new biostream snapshot for the current host + lab zone.
    pub fn current(env: CargoEnvDescriptor,
                   host_budget: HostBudget,
                   thermo: ThermodynamicEnvelope,
                   evidence: EvidenceBundle,
                   neurorights: ALNComplianceParticle) -> Self {
        RealityBiostream {
            env,
            host_budget,
            thermo,
            evidence,
            neurorights,
        }
    }

    /// Evaluate an upgrade as a unit of cybernetic-liquidity over nanobyte-territory.
    /// This enforces:
    /// - env gate (toolchain, OTA, DNS posture),
    /// - HostBudget + thermo corridors,
    /// - neurorights clauses on the ALN particle.
    pub fn route_liquidity<S: BioscaleUpgradeStore>(
        &mut self,
        store: &S,
        router: &CyberSwarmNeurostackRouter,
        upgrade: UpgradeDescriptor,
    ) -> UpgradeDecision {
        // 1. Environment gate: no evolution outside Reality.os descriptor.
        if !self.env.is_target_allowed(upgrade.target_triple.as_str()) {
            return UpgradeDecision::Denied {
                reason: "target triple not allowed by RealityBiostream.env".into(),
            };
        }
        if !self.env.is_ota_repo_allowed(
            upgrade.github_org.as_str(),
            upgrade.github_repo_slug.as_str(),
            upgrade.github_branch.as_str(),
        ) {
            return UpgradeDecision::Denied {
                reason: "OTA repo not allowed by RealityBiostream.env".into(),
            };
        }

        // 2. Neurorights gate: require rollback + non-covert modulation.
        if !self.neurorights.is_compliant_for(&upgrade, &self.evidence) {
            return UpgradeDecision::Denied {
                reason: "neurorights particle not compliant for this upgrade".into(),
            };
        }

        // 3. Bioscale evaluation under current HostBudget + ThermodynamicEnvelope.
        let now = SystemTime::now();
        let decision = store.evaluate_with_env_and_thermo(
            self.host_budget.clone(),
            self.thermo.clone(),
            upgrade.clone(),
            now,
        );

        // 4. If approved, route through the neurostack router; otherwise, propagate denial.
        match decision {
            UpgradeDecision::Approved { scheduled_at, expected_completion } => {
                router.route_with_bioscale(
                    self.host_budget.clone(),
                    upgrade,
                    self.env.clone(),
                    scheduled_at,
                    expected_completion,
                );
                UpgradeDecision::Approved { scheduled_at, expected_completion }
            }
            denied @ UpgradeDecision::Denied { .. } => denied,
        }
    }
}
