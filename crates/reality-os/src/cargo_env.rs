use std::time::Duration;

use bioscale_upgrade_store::{EvidenceBundle, HostBudget};
use nanoswarm_host_maths::DEFAULTBIOPHYSEVIDENCE;

/// Which Rust toolchains and targets are allowed on this host.
#[derive(Clone, Debug)]
pub struct RustToolchainEnvelope {
    pub channel: String,                  // e.g. "stable", "beta"
    pub min_version_semver: String,       // e.g. "1.76.0"
    pub max_version_semver: Option<String>,
    pub allowed_targets: Vec<String>,     // e.g. ["x86_64-unknown-linux-gnu", "thumbv7em-none-eabihf"]
    pub forbid_unsafe_code: bool,
    pub allow_proc_macros: bool,
    pub allow_build_rs: bool,
}

/// Constraints on dependency graph and supply chain.
#[derive(Clone, Debug)]
pub struct CargoSupplyChainPolicy {
    pub allowed_orgs: Vec<String>,        // e.g. ["Doctor0Evil", "rust-lang"]
    pub allowed_registries: Vec<String>,  // e.g. ["crates.io-index"]
    pub forbid_git_deps: bool,
    pub forbid_path_deps_outside_lab: bool,
    pub max_transitive_deps: u32,
}

/// OTA / CI-CD routing constraints for upgrades.
#[derive(Clone, Debug)]
pub struct OtaConstraint {
    pub github_org: String,
    pub allowed_repos: Vec<String>,
    pub allowed_branches: Vec<String>,
    pub max_ota_runtime: Duration,
}

/// Bioscale + corridor envelopes that must be honored by upgrades.
#[derive(Clone, Debug)]
pub struct BioscaleEnvEnvelope {
    pub host_budget: HostBudget,
    pub default_evidence: EvidenceBundle,
    pub max_corridor_score: f64, // S_bio^C max
}

/// Top-level environment descriptor exported by Reality.os.
#[derive(Clone, Debug)]
pub struct CargoEnvDescriptor {
    pub rust: RustToolchainEnvelope,
    pub supply_chain: CargoSupplyChainPolicy,
    pub ota: OtaConstraint,
    pub bioscale: BioscaleEnvEnvelope,
}

impl CargoEnvDescriptor {
    /// Hard-coded host descriptor for this Reality.os shard.
    /// In your lab this can be loaded from TOML / on-chain config.
    pub fn current() -> Self {
        let host_budget = HostBudget {
            daily_energy_joules: 8.0e6,      // ~1900 kcal/day
            remaining_energy_joules: 4.0e6,  // conservative half-day envelope
            daily_protein_grams: 120.0,
            remaining_protein_grams: 60.0,
        };

        CargoEnvDescriptor {
            rust: RustToolchainEnvelope {
                channel: "stable".into(),
                min_version_semver: "1.76.0".into(),
                max_version_semver: None,
                allowed_targets: vec![
                    "x86_64-unknown-linux-gnu".into(),
                    "aarch64-unknown-linux-gnu".into(),
                    "thumbv7em-none-eabihf".into(),
                ],
                forbid_unsafe_code: true,
                allow_proc_macros: true,
                allow_build_rs: false,
            },
            supply_chain: CargoSupplyChainPolicy {
                allowed_orgs: vec![
                    "Doctor0Evil".into(),
                    "rust-lang".into(),
                ],
                allowed_registries: vec!["crates.io-index".into()],
                forbid_git_deps: true,
                forbid_path_deps_outside_lab: true,
                max_transitive_deps: 128,
            },
            ota: OtaConstraint {
                github_org: "Doctor0Evil".into(),
                allowed_repos: vec![
                    "Cyberswarm".into(),
                    "Cybercore-Brain".into(),
                    "CyberNano".into(),
                    "Bioscale-tech.Inc".into(),
                ],
                allowed_branches: vec!["main".into(), "lab".into()],
                max_ota_runtime: Duration::from_secs(15 * 60),
            },
            bioscale: BioscaleEnvEnvelope {
                host_budget,
                default_evidence: DEFAULTBIOPHYSEVIDENCE.clone(),
                max_corridor_score: 0.7,
            },
        }
    }

    /// Minimal query: is this GitHub repo allowed for OTA on this host?
    pub fn is_ota_repo_allowed(&self, org: &str, repo: &str, branch: &str) -> bool {
        if org != self.ota.github_org {
            return false;
        }
        if !self.ota.allowed_repos.iter().any(|r| r == repo) {
            return false;
        }
        if !self.ota.allowed_branches.iter().any(|b| b == branch) {
            return false;
        }
        true
    }

    /// Minimal query: is this Rust target allowed?
    pub fn is_target_allowed(&self, target_triple: &str) -> bool {
        self.rust
            .allowed_targets
            .iter()
            .any(|t| t == target_triple)
    }
}

/// Convenience entry-point expected by other crates.
pub fn describe_cargo_env() -> CargoEnvDescriptor {
    CargoEnvDescriptor::current()
}
