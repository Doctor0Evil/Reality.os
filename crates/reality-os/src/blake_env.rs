use std::time::Duration;
use crate::cargoenv::{CargoEnvDescriptor, BioscaleEnvEnvelope};
use bioscale_upgrade_store::{HostBudget, EvidenceBundle};
use bioscale_upgrade_store::defaults::DEFAULTBIOPHYSEVIDENCE;

/// Blake-family crypto posture for this host.
#[derive(Clone, Debug)]
pub struct BlakeCryptoPosture {
    /// If false, any Blake3-related compile, run, or OTA is forbidden.
    pub blake3_allowed: bool,
    /// Optional evidence tag that must appear in an upgrade's EvidenceBundle.
    pub required_evidence_tag: Option<&'static str>,
    /// Version of the static denylist in force.
    pub pattern_set_version: &'static str,
}

/// Extend the core environment descriptor with Blake posture.
#[derive(Clone, Debug)]
pub struct BlakeAwareEnv {
    pub env: CargoEnvDescriptor,
    pub blake: BlakeCryptoPosture,
}

impl BlakeAwareEnv {
    /// Hard-coded host descriptor; in-lab you can load from TOML or chain.
    pub fn current() -> Self {
        let bioscale = BioscaleEnvEnvelope {
            hostbudget: HostBudget {
                dailyenergyjoules: 8.0e6,
                remainingenergyjoules: 4.0e6,
                dailyproteingrams: 120.0,
                remainingproteingrams: 60.0,
            },
            defaultevidence: DEFAULTBIOPHYSEVIDENCE.clone(),
            maxcorridorscore: 0.7,
        };

        let env = CargoEnvDescriptor::new_default(bioscale);
        let blake = BlakeCryptoPosture {
            blake3_allowed: false,
            required_evidence_tag: None,
            pattern_set_version: "blake-patterns-v1",
        };

        Self { env, blake }
    }

    /// Does this env permit Blake3 given the upgrade's evidence tags?
    pub fn permits_blake3(&self, tags: &[&str]) -> bool {
        if !self.blake.blake3_allowed {
            return false;
        }
        if let Some(required) = self.blake.required_evidence_tag {
            tags.iter().any(|t| *t == required)
        } else {
            true
        }
    }
}
