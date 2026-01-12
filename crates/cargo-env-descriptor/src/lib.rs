use std::time::Duration;
use bioscale_upgrade_store::{EvidenceBundle, HostBudget};
use nanoswarm_host_math::MlDutyEnvelope;

/// Bostrom/ALN-bound identity and audit hooks for this environment.
#[derive(Clone, Debug)]
pub struct IdentityBinding {
    pub bostrom_address: String,
    pub aln_did: String,
    pub kyc_attested: bool,
    pub audit_log_uri: String,
}

/// Toolchain and target metadata relevant to bioscale / BCI crates.
#[derive(Clone, Debug)]
pub struct ToolchainProfile {
    pub rustc_version: String,
    pub target_triple: String,
    pub no_std: bool,
    pub has_sse_avx: bool,
    pub has_neon: bool,
}

/// Neurorights and bioscale capabilities exposed by this env.
#[derive(Clone, Debug)]
pub struct NeurorightsCaps {
    pub bioscale_abi_present: bool,
    pub default_evidence: EvidenceBundle,
    pub host_budget_profile: HostBudget,
    pub ml_duty_envelope: MlDutyEnvelope,
    pub reversal_supported: bool,
}

/// Dev-tunnel and sandbox predicates for safe Cargo usage.
#[derive(Clone, Debug)]
pub struct DevTunnelCaps {
    pub allowlist_commands: Vec<String>,   // e.g. ["cargo check", "cargo test --no-run"]
    pub cpu_millis_quota: u64,
    pub memory_bytes_quota: u64,
    pub ota_updates_enabled: bool,
    pub max_tunnel_duration: Duration,
    pub transport_quic: bool,
    pub transport_http3: bool,
    pub transport_cbor: bool,
}

/// Top-level descriptor: what a Phoenix-compatible Cargo env *must* declare.
#[derive(Clone, Debug)]
pub struct CargoEnvDescriptor {
    pub identity: IdentityBinding,
    pub toolchain: ToolchainProfile,
    pub neurorights: NeurorightsCaps,
    pub dev_tunnel: DevTunnelCaps,
}
