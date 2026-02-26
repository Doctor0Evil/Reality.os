#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// How strictly to follow existing Phoenix/Reality.os semantics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DescriptorCompatibility {
    /// Must remain byte/field compatible with current CargoEnvDescriptor.
    PreserveExisting,
    /// May extend with new fields under versioned `extensions`.
    ExtendWithVersionedFields,
}

/// Where compile-time macro gates must hold.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MacroScope {
    /// Only guarantee local host build + runtime safety.
    LocalHostOnly,
    /// Also guarantee distributed ALN corridor constraints cross-node.
    HostAndDistributedCorridors,
}

/// How ingestion treats env vs descriptor.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnvAuthority {
    /// CargoEnvDescriptor is sole source of truth; env vars cannot relax it.
    DescriptorOverridesEnv,
    /// Limited, ALN-approved overrides allowed via explicit particles.
    DescriptorWithAlnOverrides,
}

/// Q&A policy shard that future agents and CI can read.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoEnvResearchPolicy {
    pub compatibility: DescriptorCompatibility,
    pub macro_scope: MacroScope,
    pub env_authority: EnvAuthority,
    /// ALN object id that encodes this policy (e.g. ?cargoenv-research-policy.v1).
    pub aln_policy_id: String,
}

/// Concrete policy reflecting your answers:
/// - Prefer compatibility, but allow versioned extensions.
/// - Macros must enforce local AND distributed corridor contracts.
/// - Descriptor is the authority; env overrides only via ALN.
impl CargoEnvResearchPolicy {
    pub fn current() -> Self {
        Self {
            compatibility: DescriptorCompatibility::ExtendWithVersionedFields,
            macro_scope: MacroScope::HostAndDistributedCorridors,
            env_authority: EnvAuthority::DescriptorWithAlnOverrides,
            aln_policy_id: "?cargoenv-research-policy.v1".to_string(),
        }
    }

    /// Helper: may macros assume distributed ALN corridors are in scope?
    pub fn require_distributed_corridors(&self) -> bool {
        matches!(self.macro_scope, MacroScope::HostAndDistributedCorridors)
    }

    /// Helper: may env vars ever override descriptor?
    pub fn env_override_requires_aln(&self) -> bool {
        matches!(self.env_authority, EnvAuthority::DescriptorWithAlnOverrides)
    }
}

// ---------------------- neuro.print! research surface ------------------------

/// Structured neuro-aware debug event; never raw println! noise.
/// This is where `neuro.print!` will land.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroPrintEvent {
    /// Stable identifier of the host / DID (Bostrom ALN).
    pub did: String,
    /// High-level corridor or upgrade id (e.g. "motor-assist:v2").
    pub corridor_id: String,
    /// Optional local node id (BCI node, nanoswarm cluster, dev-tunnel).
    pub node_id: Option<String>,
    /// Short hex key identifying the evidence bundle driving this print.
    /// e.g. "a1f3c9b2" .. "8f09d5ee".
    pub evidence_hex: &'static str,
    /// Log class: Trace/Debug/Info/Warn/Error at neuro-layer.
    pub level: NeuroPrintLevel,
    /// Human-readable, neurorights-aware message (no raw PII / raw EEG).
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NeuroPrintLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Trait for sinks that consume `neuro.print!` events (stdout, ALN, Prometheus).
pub trait NeuroPrintSink: Send + Sync {
    fn record(&self, event: NeuroPrintEvent);
}

/// Global, swappable neuro-print backend (lab: stdout; prod: ALN ledger).
static mut NEURO_PRINT_SINK: Option<&'static dyn NeuroPrintSink> = None;

/// Set once at startup (e.g. Reality.os main, test harness).
pub fn install_neuro_print_sink(sink: &'static dyn NeuroPrintSink) {
    unsafe {
        NEURO_PRINT_SINK = Some(sink);
    }
}

/// Core helper used by the neuro.print! macro expansion.
pub fn neuro_print_emit(event: NeuroPrintEvent) {
    unsafe {
        if let Some(sink) = NEURO_PRINT_SINK {
            sink.record(event);
        }
    }
}
