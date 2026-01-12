use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::Duration;

use bioscale_upgrade_store::{EvidenceBundle, HostBudget}; // existing ABI.[file:12]

/// Rust toolchain + Cargo profile info for this host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostToolchainPredicates {
    /// Full `rustc -Vv` output string.
    pub rustc_version_verbose: String,
    /// Channel: stable, beta, nightly.
    pub channel: String,
    /// Supported target triples.
    pub target_triples: Vec<String>,
    /// Available Cargo profiles: dev, release, lab-sim, etc.
    pub cargo_profiles: Vec<String>,
    /// Feature flags known to be compiled in for this workspace.
    pub enabled_features: Vec<String>,
    /// Max CPU seconds allowed per dev-tunnel build command.
    pub max_cpu_time_per_cmd: Duration,
    /// Max memory in bytes allowed per dev-tunnel build command.
    pub max_memory_bytes_per_cmd: u64,
    /// Max disk space in bytes allowed for build artifacts.
    pub max_disk_bytes_per_cmd: u64,
}

/// Neurorights, bioscale, and reversibility predicates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyNeurorightsPredicates {
    /// True if any hardware that can actuate the host is currently reachable.
    pub bci_hw_present: bool,
    /// If true, runtime will refuse any command that could actuate hardware.
    pub hardware_actuation_disabled: bool,
    /// If false, this environment cannot produce OTA artifacts.
    pub ota_updates_allowed: bool,
    /// True if bioscale ABI is linkable and usable in this workspace.
    pub bioscale_abi_present: bool,
    /// Default host budget envelope this environment promises to respect.
    pub default_host_budget: HostBudget,
    /// Canonical evidence bundle used to validate upgrades.
    pub default_evidence_bundle: EvidenceBundle,
    /// True if ReversalConditions and downgrade paths are compiled in.
    pub reversible_upgrades_enabled: bool,
}

/// Dev-tunnel specific predicates for AI-chat build agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevTunnelTransportPredicates {
    /// Positive allowlist of permitted `cargo` subcommands.
    /// Example: ["check", "test --no-run", "doc"].
    pub allowed_cargo_commands: HashSet<String>,
    /// True if OTA-related workflows are disabled for this tunnel.
    pub ota_disabled_for_tunnel: bool,
    /// Network latency class: "low", "medium", "high".
    pub latency_class: String,
    /// Network bandwidth class: "low", "medium", "high".
    pub bandwidth_class: String,
    /// True if every command is logged with args, git SHA, EvidenceBundle,
    /// and bound to an authenticated Bostrom/ALN identity.[file:1]
    pub audit_logging_enabled: bool,
}

/// Top-level descriptor object that AI-chat dev-tunnels must expose.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoEnvDescriptor {
    pub host_toolchain: HostToolchainPredicates,
    pub safety_neurorights: SafetyNeurorightsPredicates,
    pub dev_tunnel: DevTunnelTransportPredicates,
}

impl CargoEnvDescriptor {
    /// Hard safety check: does this environment qualify to build for a
    /// cybernetic host with active BCI hardware?
    pub fn is_bci_safety_qualified(&self) -> bool {
        // Must have bioscale ABI and reversible upgrades.
        if !self.safety_neurorights.bioscale_abi_present {
            return false;
        }
        if !self.safety_neurorights.reversible_upgrades_enabled {
            return false;
        }

        // If BCI hardware is present, require hardware actuation to be disabled
        // and OTA disabled for this tunnel by default.[file:1]
        if self.safety_neurorights.bci_hw_present {
            if !self.safety_neurorights.hardware_actuation_disabled {
                return false;
            }
            if !self.dev_tunnel.ota_disabled_for_tunnel {
                return false;
            }
        }

        // Require strict audit logging for all dev-tunnel usage.[file:1]
        if !self.dev_tunnel.audit_logging_enabled {
            return false;
        }

        true
    }

    /// Check whether a requested `cargo` command string is allowed.
    pub fn allows_cargo_command<S: AsRef<str>>(&self, cmd: S) -> bool {
        self.dev_tunnel
            .allowed_cargo_commands
            .contains(cmd.as_ref())
    }
}

/// Minimal query API: Reality.os host service.
#[derive(Debug)]
pub enum EnvQueryError {
    IntrospectionFailed(&'static str),
}

/// This function should be exposed as a gRPC/HTTP endpoint in Reality.os,
/// returning the current CargoEnvDescriptor for the host.
/// AI-chat dev-tunnels must call this before issuing any build commands.[file:1]
pub fn describe_cargo_env() -> Result<CargoEnvDescriptor, EnvQueryError> {
    // For now, use placeholders; in production this should:
    //   - Spawn `rustc -Vv`, `rustc --print target-list`.
    //   - Read host limits via OS APIs (rlimits, cgroups, etc.).
    //   - Populate HostBudget and EvidenceBundle from bioscale defaults.[file:1][file:12]
    let host_toolchain = HostToolchainPredicates {
        rustc_version_verbose: "rustc 1.76.0 (example)".to_string(),
        channel: "stable".to_string(),
        target_triples: vec!["x86_64-unknown-linux-gnu".to_string()],
        cargo_profiles: vec!["dev".to_string(), "release".to_string(), "lab-sim".to_string()],
        enabled_features: vec!["bioscale".to_string(), "neurorights-strict".to_string()],
        max_cpu_time_per_cmd: Duration::from_secs(60),
        max_memory_bytes_per_cmd: 2 * 1024 * 1024 * 1024, // 2 GiB
        max_disk_bytes_per_cmd: 4 * 1024 * 1024 * 1024,    // 4 GiB
    };

    let default_host_budget = HostBudget {
        daily_energy_joules: 8_000_000.0,
        remaining_energy_joules: 4_000_000.0,
        daily_protein_grams: 100.0,
        remaining_protein_grams: 60.0,
    };

    let default_evidence_bundle = bioscale_upgrade_store::defaults::DEFAULT_BIOPHYS_EVIDENCE;

    let safety_neurorights = SafetyNeurorightsPredicates {
        bci_hw_present: true,
        hardware_actuation_disabled: true,
        ota_updates_allowed: false,
        bioscale_abi_present: true,
        default_host_budget,
        default_evidence_bundle,
        reversible_upgrades_enabled: true,
    };

    let mut allowed = HashSet::new();
    allowed.insert("check".to_string());
    allowed.insert("test --no-run".to_string());
    allowed.insert("doc".to_string());

    let dev_tunnel = DevTunnelTransportPredicates {
        allowed_cargo_commands: allowed,
        ota_disabled_for_tunnel: true,
        latency_class: "medium".to_string(),
        bandwidth_class: "medium".to_string(),
        audit_logging_enabled: true,
    };

    Ok(CargoEnvDescriptor {
        host_toolchain,
        safety_neurorights,
        dev_tunnel,
    })
}
